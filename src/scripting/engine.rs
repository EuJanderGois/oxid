//! QuickJS scripting engine lifecycle.

use rquickjs::{Context, Runtime};

use crate::{
    i18n,
    renderer::{
        context::{clear_active_queue, set_active_queue},
        queue::RenderQueue,
    },
};

use super::{bootstrap, error::ScriptEngineError, hooks};

pub struct ScriptEngine {
    _rt: Runtime,
    ctx: Context,
}

impl ScriptEngine {
    pub fn new(script_code: &str) -> Result<Self, ScriptEngineError> {
        let rt = Runtime::new().map_err(|e| ScriptEngineError::RuntimeInit(e.to_string()))?;

        let ctx = Context::full(&rt).map_err(|e| ScriptEngineError::ContextInit(e.to_string()))?;

        ctx.with(|ctx| -> Result<(), ScriptEngineError> {
            bootstrap::register_modules(&ctx)?;

            let globals = ctx.globals();

            bootstrap::bootstrap_entry_module(&ctx, script_code, &globals)?;
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
