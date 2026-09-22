//! Script runtime bootstrap and entry-module initialization.

use rquickjs::{Ctx, Module, Object};

use super::{core, error::ScriptEngineError, registry};

pub const APP_INSTANCE: &str = "__app_instance";
pub const MAIN_NAMESPACE: &str = "__main";

pub fn register_modules(ctx: &Ctx<'_>) -> Result<(), ScriptEngineError> {
    registry::register_native_modules(ctx)
        .map_err(|(plugin, source)| ScriptEngineError::PluginRegister { plugin, source })?;
    core::register(ctx)
}

pub fn bootstrap_entry_module<'a>(
    ctx: &Ctx<'a>,
    script_code: &str,
    globals: &Object<'a>,
) -> Result<(), ScriptEngineError> {
    let module = Module::declare(ctx.clone(), "main.js", script_code)
        .map_err(|e| ScriptEngineError::EntryModuleDeclare(e.to_string()))?;

    module
        .clone()
        .eval()
        .map_err(|e| ScriptEngineError::EntryModuleEval(e.to_string()))?;

    let namespace = module
        .namespace()
        .map_err(|e| ScriptEngineError::MainNamespace(e.to_string()))?;

    globals
        .set(MAIN_NAMESPACE, namespace)
        .map_err(|e| ScriptEngineError::MainNamespace(e.to_string()))?;

    ctx.eval::<(), _>(format!(
        "globalThis.{APP_INSTANCE} = {MAIN_NAMESPACE}.main();"
    ))
    .map_err(|e| ScriptEngineError::AppInstance(e.to_string()))?;

    Ok(())
}
