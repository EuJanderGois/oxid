//! QuickJS adapter for [`crate::scripting::resolver::ModuleResolver`].
//!
//! Native `oxid/*` modules are declared and evaluated ahead of time (see
//! [`super::bootstrap::register_modules`]), which means QuickJS already has
//! them cached under their exact name before any user script runs. Because
//! of that, [`FsResolver`] only needs to recognize those names and echo them
//! back unchanged for QuickJS to find them in its module cache; [`FsLoader`]
//! never actually gets called for them.
//!
//! All of the actual resolution *policy* — recognizing `oxid/*` built-ins,
//! resolving relative/root-relative/bare specifiers, and keeping imports
//! inside the project directory — lives in
//! [`crate::scripting::resolver::ModuleResolver`], which has no dependency on
//! QuickJS at all. This module is only the thin glue that satisfies the
//! `rquickjs::loader::{Resolver, Loader}` traits QuickJS requires.

use std::{fs, path::PathBuf};

use rquickjs::{
    Ctx, Error as JsError, Module, Result as JsResult,
    loader::{Loader, Resolver},
    module::Declared,
};

use crate::scripting::{
    plugins::registry,
    resolver::{ModuleResolver, ResolvedModule},
};

/// Resolves `import`/`export ... from` specifiers via [`ModuleResolver`],
/// translating its result into the module key string QuickJS expects.
pub struct FsResolver {
    resolver: ModuleResolver,
}

impl FsResolver {
    /// `root` must already be canonicalized; every resolved file path is
    /// required to stay inside it.
    pub fn new(root: PathBuf) -> Self {
        let builtins = registry::plugins().iter().map(|plugin| plugin.name);

        Self {
            resolver: ModuleResolver::new(root, builtins),
        }
    }
}

impl Resolver for FsResolver {
    fn resolve<'js>(&mut self, _ctx: &Ctx<'js>, base: &str, name: &str) -> JsResult<String> {
        match self.resolver.resolve(base, name) {
            Ok(ResolvedModule::Builtin(name)) => Ok(name.to_string()),
            Ok(ResolvedModule::File(path)) => Ok(path.to_string_lossy().into_owned()),
            Err(message) => Err(JsError::new_resolving_message(base, name, message)),
        }
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
