//! QuickJS implementation of [`ScriptRuntime`](crate::scripting::ScriptRuntime).
//!
//! This module (plus [`super::plugins`], see that module's documentation) is
//! the only part of Oxid allowed to depend on `rquickjs`. If Oxid ever grows
//! a second scripting backend (e.g. a pure-Rust JavaScript VM), it would live
//! in a sibling module implementing the same
//! [`ScriptRuntime`](crate::scripting::ScriptRuntime) trait, and this module
//! would not need to change.

mod bootstrap;
mod hooks;
mod loader;

use std::path::Path;

use rquickjs::{Context, Runtime};

use crate::{
    i18n,
    renderer::{
        context::{clear_active_queue, set_active_queue},
        queue::RenderQueue,
    },
};

use super::{error::ScriptError, runtime::ScriptRuntime};

/// QuickJS-backed [`ScriptRuntime`]. See the module documentation.
pub struct QuickJsRuntime {
    // Kept alive for as long as `context` needs it; never accessed directly
    // once construction finishes; see `ScriptRuntime::on_*` for the only
    // operations this runtime supports after startup.
    _runtime: Runtime,
    context: Context,
}

impl QuickJsRuntime {
    /// `project_root` is the directory containing the project's `package.json`;
    /// `entry_path` is the path to the entry script (e.g. `<project_root>/main.js`).
    /// Both are used to scope and resolve `import`s between the project's own
    /// script files (see [`loader`]).
    pub fn new(
        script_code: &str,
        project_root: &Path,
        entry_path: &Path,
    ) -> Result<Self, ScriptError> {
        let project_root = project_root
            .canonicalize()
            .map_err(|e| ScriptError::ModuleRootInit(e.to_string()))?;
        let entry_path = entry_path
            .canonicalize()
            .map_err(|e| ScriptError::ModuleRootInit(e.to_string()))?;

        let runtime = Runtime::new().map_err(|e| ScriptError::RuntimeInit(e.to_string()))?;

        runtime.set_loader(loader::FsResolver::new(project_root), loader::FsLoader);

        let context =
            Context::full(&runtime).map_err(|e| ScriptError::ContextInit(e.to_string()))?;

        context.with(|ctx| -> Result<(), ScriptError> {
            bootstrap::register_modules(&ctx)?;
            bootstrap::register_globals(&ctx)?;

            let globals = ctx.globals();

            bootstrap::bootstrap_entry_module(&ctx, script_code, &entry_path, &globals)?;
            hooks::compile_hooks(&ctx, &globals)?;

            Ok(())
        })?;

        Ok(Self {
            _runtime: runtime,
            context,
        })
    }
}

impl ScriptRuntime for QuickJsRuntime {
    fn on_init(&self) {
        if let Err(err) = hooks::call_void_hook(&self.context, hooks::HOOK_ON_INIT) {
            let source = err.to_string();
            eprintln!(
                "{}",
                i18n::prefixed_with(
                    "scripting",
                    "scripting.error.on_init",
                    &[("source", &source)]
                )
            );
        }
    }

    fn on_update(&self, delta_time: f32) {
        if let Err(err) = hooks::call_f32_hook(&self.context, hooks::HOOK_ON_UPDATE, delta_time) {
            let source = err.to_string();
            eprintln!(
                "{}",
                i18n::prefixed_with(
                    "scripting",
                    "scripting.error.on_update",
                    &[("source", &source)],
                )
            );
        }
    }

    fn on_draw(&self, queue: &mut RenderQueue) {
        set_active_queue(queue);

        if let Err(err) = hooks::call_void_hook(&self.context, hooks::HOOK_ON_DRAW) {
            let source = err.to_string();
            eprintln!(
                "{}",
                i18n::prefixed_with(
                    "scripting",
                    "scripting.error.on_draw",
                    &[("source", &source)]
                )
            );
        }

        clear_active_queue();
    }
}
