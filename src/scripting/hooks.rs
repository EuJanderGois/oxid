//! JavaScript lifecycle hook compilation and invocation.

use rquickjs::{CaughtError, Context, Ctx, Function, Object};

use crate::i18n;

use super::{bootstrap::APP_INSTANCE, error::ScriptEngineError};

pub const HOOK_ON_INIT: &str = "__hook_on_init";
pub const HOOK_ON_UPDATE: &str = "__hook_on_update";
pub const HOOK_ON_DRAW: &str = "__hook_on_draw";


pub fn compile_hooks<'a>(ctx: &Ctx<'a>, globals: &Object<'a>) -> Result<(), ScriptEngineError> {
    let hook_on_init: Function = ctx
        .eval(format!(
            "() => {{ if ({APP_INSTANCE}.onInit) {APP_INSTANCE}.onInit(); }}"
        ))
        .map_err(|e| ScriptEngineError::HookCompile {
            hook: "onInit",
            source: e.to_string(),
        })?;

    let hook_on_update: Function = ctx
        .eval(format!(
            "(dt) => {{ if ({APP_INSTANCE}.onUpdate) {APP_INSTANCE}.onUpdate(dt); }}"
        ))
        .map_err(|e| ScriptEngineError::HookCompile {
            hook: "onUpdate",
            source: e.to_string(),
        })?;

    let hook_on_draw: Function = ctx
        .eval(format!(
            "() => {{ if ({APP_INSTANCE}.onDraw) {APP_INSTANCE}.onDraw(); }}"
        ))
        .map_err(|e| ScriptEngineError::HookCompile {
            hook: "onDraw",
            source: e.to_string(),
        })?;

    globals
        .set(HOOK_ON_INIT, hook_on_init)
        .map_err(|e| ScriptEngineError::HookCompile {
            hook: "onInit",
            source: e.to_string(),
        })?;

    globals.set(HOOK_ON_UPDATE, hook_on_update).map_err(|e| {
        ScriptEngineError::HookCompile {
            hook: "onUpdate",
            source: e.to_string(),
        }
    })?;

    globals
        .set(HOOK_ON_DRAW, hook_on_draw)
        .map_err(|e| ScriptEngineError::HookCompile {
            hook: "onDraw",
            source: e.to_string(),
        })?;

    Ok(())
}


pub fn call_void_hook(context: &Context, hook_name: &'static str) -> Result<(), ScriptEngineError> {
    context.with(|ctx| {
        let func: Function = ctx.globals().get(hook_name).map_err(|e| {
            let source = e.to_string();
            ScriptEngineError::HookExecution {
                hook: hook_name,
                source: i18n::text_with("scripting.error.hook_missing", &[("source", &source)]),
            }
        })?;

        func.call::<_, ()>(())
            .map_err(|e| hook_error(&ctx, hook_name, e))
    })
}


pub fn call_f32_hook(context: &Context, hook_name: &'static str, value: f32) -> Result<(), ScriptEngineError> {
    context.with(|ctx| {
        let func: Function = ctx.globals().get(hook_name).map_err(|e| {
            let source = e.to_string();
            ScriptEngineError::HookExecution {
                hook: hook_name,
                source: i18n::text_with("scripting.error.hook_missing", &[("source", &source)]),
            }
        })?;

        func.call::<_, ()>((value,))
            .map_err(|e| hook_error(&ctx, hook_name, e))
    })
}

pub fn hook_error(
    ctx: &Ctx<'_>,
    hook_name: &'static str,
    error: rquickjs::Error,
) -> ScriptEngineError {
    let source = CaughtError::from_error(ctx, error).to_string();

    ScriptEngineError::HookExecution {
        hook: hook_name,
        source,
    }
}
