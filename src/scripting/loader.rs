//! Filesystem-backed module resolver and loader.
//!
//! Native `oxid/*` modules are declared and evaluated ahead of time (see
//! [`super::bootstrap::register_modules`]), which means QuickJS already has them
//! cached under their exact name before any user script runs. Because of that,
//! [`FsResolver`] only needs to recognize those names and echo them back
//! unchanged for QuickJS to find them in its module cache; [`FsLoader`] never
//! actually gets called for them. Every other import specifier is treated as a
//! path to one of the project's own `.js` files, resolved relative to the
//! importing file (for `./`/`../` specifiers) or to the project root (for
//! bare/absolute specifiers), and constrained to stay inside the project
//! directory.

use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use rquickjs::{
    Ctx, Error as JsError, Module, Result as JsResult,
    loader::{Loader, Resolver},
    module::Declared,
};

use super::plugins::registry;

/// Resolves `import`/`export ... from` specifiers to a stable module key: the
/// untouched name of a built-in `oxid/*` module, or the canonicalized,
/// absolute path of a project script file.
pub struct FsResolver {
    builtins: HashSet<&'static str>,
    root: PathBuf,
}

impl FsResolver {
    /// `root` must already be canonicalized; every resolved file path is
    /// required to stay inside it.
    pub fn new(root: PathBuf) -> Self {
        Self {
            builtins: registry::plugins()
                .iter()
                .map(|plugin| plugin.name)
                .collect(),
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
}

impl Resolver for FsResolver {
    fn resolve<'js>(&mut self, _ctx: &Ctx<'js>, base: &str, name: &str) -> JsResult<String> {
        // Built-in modules are already loaded under their bare name; nothing
        // to touch on disk for those.
        if self.builtins.contains(name) {
            return Ok(name.to_string());
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

        let resolved = Self::find_existing_file(&target).ok_or_else(|| {
            JsError::new_resolving_message(
                base,
                name,
                format!("no such module file: '{}'", target.display()),
            )
        })?;

        let resolved = resolved
            .canonicalize()
            .map_err(|err| JsError::new_resolving_message(base, name, err.to_string()))?;

        if !resolved.starts_with(&self.root) {
            return Err(JsError::new_resolving_message(
                base,
                name,
                "modules can only be imported from inside the project directory".to_string(),
            ));
        }

        Ok(resolved.to_string_lossy().into_owned())
    }
}

/// Loads a module by the key produced by [`FsResolver`]. Built-in modules are
/// already registered before any user script evaluates, so QuickJS resolves
/// them straight from its module cache and this loader is only ever invoked
/// for project script files.
pub struct FsLoader;

impl Loader for FsLoader {
    fn load<'js>(&mut self, ctx: &Ctx<'js>, path: &str) -> JsResult<Module<'js, Declared>> {
        let source = fs::read_to_string(path)
            .map_err(|err| JsError::new_loading_message(path, err.to_string()))?;

        Module::declare(ctx.clone(), path, source)
    }
}
