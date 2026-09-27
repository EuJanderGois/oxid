//! Oxid's own module-resolution policy, independent of any JavaScript engine.
//!
//! This is the part of "how ES Modules work in Oxid" that is a *rule of the
//! engine* rather than a detail of QuickJS: how `oxid/*` bare specifiers are
//! recognized as built-ins, how relative (`./`, `../`) and root-relative
//! (`/`) imports are resolved against the project's own files, and the rule
//! that a project may never import a file from outside its own directory.
//!
//! [`super::quickjs::loader`] adapts this to the
//! `rquickjs::loader::{Resolver, Loader}` traits QuickJS requires. A future
//! scripting backend would need a similarly thin adapter of its own, but it
//! would not need to reimplement any of the policy below, and this module
//! can be fully tested without starting a JavaScript runtime at all.

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

/// The outcome of resolving one `import`/`export ... from` specifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedModule {
    /// One of the built-in `oxid/*` native modules, identified by its bare
    /// name (e.g. `"oxid/math"`).
    Builtin(&'static str),
    /// A project script file, as a canonicalized absolute path.
    File(PathBuf),
}

/// Resolves `import` specifiers to a [`ResolvedModule`].
///
/// `root` must already be canonicalized; every resolved file is required to
/// stay inside it.
pub struct ModuleResolver {
    builtins: HashSet<&'static str>,
    root: PathBuf,
}

impl ModuleResolver {
    /// `root` must already be canonicalized. `builtins` are the bare module
    /// names (e.g. `"oxid/math"`) that should resolve to
    /// [`ResolvedModule::Builtin`] instead of being looked up on disk.
    pub fn new(root: PathBuf, builtins: impl IntoIterator<Item = &'static str>) -> Self {
        Self {
            builtins: builtins.into_iter().collect(),
            root,
        }
    }

    /// Candidate file paths to try, in order, for an extension-less or
    /// directory-style import specifier.
    fn candidates(path: &Path) -> [PathBuf; 3] {
        [
            path.to_path_buf(),
            path.with_extension("js"),
            path.join("index.js"),
        ]
    }

    fn find_existing_file(path: &Path) -> Option<PathBuf> {
        Self::candidates(path)
            .into_iter()
            .find(|candidate| candidate.is_file())
    }

    /// Resolves `name`, imported from `base`, to a [`ResolvedModule`].
    ///
    /// `base` is the module key of the importing file: the canonicalized,
    /// absolute path produced by an earlier call to `resolve` (or the
    /// project's entry point, for the project's first import).
    ///
    /// Returns a human-readable error message on failure; callers are
    /// expected to wrap it in whatever error type their backend uses.
    pub fn resolve(&self, base: &str, name: &str) -> Result<ResolvedModule, String> {
        // Built-in modules are already loaded under their bare name; nothing
        // to touch on disk for those. `get` (rather than `contains`) hands
        // back the `&'static str` stored in the set, since `name` itself is
        // only borrowed for the duration of this call.
        if let Some(builtin) = self.builtins.get(name).copied() {
            return Ok(ResolvedModule::Builtin(builtin));
        }

        let target = if let Some(from_root) = name.strip_prefix('/') {
            self.root.join(from_root)
        } else if name.starts_with('.') {
            Path::new(base).parent().unwrap_or(&self.root).join(name)
        } else {
            // Bare specifier: treat it as a path relative to the project root
            // (e.g. `import "entities/player.js"`).
            self.root.join(name)
        };

        let resolved = Self::find_existing_file(&target)
            .ok_or_else(|| format!("no such module file: '{}'", target.display()))?;

        let resolved = resolved.canonicalize().map_err(|err| err.to_string())?;

        if !resolved.starts_with(&self.root) {
            return Err(
                "modules can only be imported from inside the project directory".to_string(),
            );
        }

        Ok(ResolvedModule::File(resolved))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// A throwaway project directory under the OS temp dir, cleaned up on
    /// drop, so tests can exercise real filesystem resolution/canonicalization
    /// without touching the repository.
    struct TempProject {
        root: PathBuf,
    }

    impl TempProject {
        fn new(name: &str) -> Self {
            let root = std::env::temp_dir().join(format!(
                "oxid_resolver_test_{name}_{}_{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            fs::create_dir_all(&root).unwrap();
            Self { root }
        }

        fn write(&self, relative: &str, contents: &str) -> PathBuf {
            let path = self.root.join(relative);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(&path, contents).unwrap();
            path
        }

        fn canonical_root(&self) -> PathBuf {
            self.root.canonicalize().unwrap()
        }
    }

    impl Drop for TempProject {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    #[test]
    fn resolves_builtin_module_by_bare_name() {
        let project = TempProject::new("builtin");
        let resolver = ModuleResolver::new(project.canonical_root(), ["oxid/math"]);

        let entry = project.write("main.js", "");
        let resolved = resolver
            .resolve(&entry.to_string_lossy(), "oxid/math")
            .unwrap();

        assert_eq!(resolved, ResolvedModule::Builtin("oxid/math"));
    }

    #[test]
    fn resolves_relative_import_from_importing_file() {
        let project = TempProject::new("relative");
        let resolver = ModuleResolver::new(project.canonical_root(), []);

        let entry = project.write("main.js", "");
        let player = project.write("entities/player.js", "");

        let resolved = resolver
            .resolve(&entry.to_string_lossy(), "./entities/player.js")
            .unwrap();

        assert_eq!(resolved, ResolvedModule::File(player.canonicalize().unwrap()));
    }

    #[test]
    fn resolves_bare_specifier_relative_to_project_root() {
        let project = TempProject::new("bare");
        let resolver = ModuleResolver::new(project.canonical_root(), []);

        let entry = project.write("main.js", "");
        let utils = project.write("utils.js", "");

        let resolved = resolver
            .resolve(&entry.to_string_lossy(), "utils.js")
            .unwrap();

        assert_eq!(resolved, ResolvedModule::File(utils.canonicalize().unwrap()));
    }

    #[test]
    fn resolves_directory_import_to_index_js() {
        let project = TempProject::new("index");
        let resolver = ModuleResolver::new(project.canonical_root(), []);

        let entry = project.write("main.js", "");
        let index = project.write("entities/index.js", "");

        let resolved = resolver
            .resolve(&entry.to_string_lossy(), "./entities")
            .unwrap();

        assert_eq!(resolved, ResolvedModule::File(index.canonicalize().unwrap()));
    }

    #[test]
    fn rejects_import_outside_project_directory() {
        let project = TempProject::new("outside");
        let outsider = TempProject::new("outside_sibling");

        let resolver = ModuleResolver::new(project.canonical_root(), []);

        let entry = project.write("main.js", "");
        outsider.write("secret.js", "");

        let escaping_specifier = format!(
            "../{}/secret.js",
            outsider.root.file_name().unwrap().to_string_lossy()
        );

        let result = resolver.resolve(&entry.to_string_lossy(), &escaping_specifier);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_missing_module_file() {
        let project = TempProject::new("missing");
        let resolver = ModuleResolver::new(project.canonical_root(), []);

        let entry = project.write("main.js", "");
        let result = resolver.resolve(&entry.to_string_lossy(), "./does-not-exist.js");

        assert!(result.is_err());
    }
}
