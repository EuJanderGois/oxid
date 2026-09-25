//! QuickJS scripting engine lifecycle.

use std::path::Path;

use rquickjs::{Context, Runtime};

use crate::{
    i18n,
    renderer::{
        context::{clear_active_queue, set_active_queue},
        queue::RenderQueue,
    },
};

use super::{
    bootstrap,
    error::ScriptEngineError,
    hooks,
    loader::{FsLoader, FsResolver},
};

pub struct ScriptEngine {
    _rt: Runtime,
    ctx: Context,
}

impl ScriptEngine {
    /// `project_root` is the directory containing the project's `package.json`;
    /// `entry_path` is the path to the entry script (e.g. `<project_root>/main.js`).
    /// Both are used to scope and resolve `import`s between the project's own
    /// script files (see `super::loader`).
    pub fn new(
        script_code: &str,
        project_root: &Path,
        entry_path: &Path,
    ) -> Result<Self, ScriptEngineError> {
        let project_root = project_root
            .canonicalize()
            .map_err(|e| ScriptEngineError::ModuleRootInit(e.to_string()))?;
        let entry_path = entry_path
            .canonicalize()
            .map_err(|e| ScriptEngineError::ModuleRootInit(e.to_string()))?;

        let rt = Runtime::new().map_err(|e| ScriptEngineError::RuntimeInit(e.to_string()))?;

        rt.set_loader(FsResolver::new(project_root), FsLoader);

        let ctx = Context::full(&rt).map_err(|e| ScriptEngineError::ContextInit(e.to_string()))?;

        ctx.with(|ctx| -> Result<(), ScriptEngineError> {
            bootstrap::register_modules(&ctx)?;

            let globals = ctx.globals();

            bootstrap::bootstrap_entry_module(&ctx, script_code, &entry_path, &globals)?;
            hooks::compile_hooks(&ctx, &globals)?;

            Ok(())
        })?;

        Ok(Self { _rt: rt, ctx })
    }

    pub fn on_init(&self) {
        if let Err(err) = hooks::call_void_hook(&self.ctx, hooks::HOOK_ON_INIT) {
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

    pub fn on_update(&self, dt: f32) {
        if let Err(err) = hooks::call_f32_hook(&self.ctx, hooks::HOOK_ON_UPDATE, dt) {
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

    pub fn on_draw(&self, queue: &mut RenderQueue) {
        set_active_queue(queue);

        if let Err(err) = hooks::call_void_hook(&self.ctx, hooks::HOOK_ON_DRAW) {
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
