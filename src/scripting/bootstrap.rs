//! Script runtime bootstrap and entry-module initialization.

use std::path::Path;

use rquickjs::{Ctx, Module, Object};

use super::{error::ScriptEngineError, plugins::registry};

pub const APP_INSTANCE: &str = "__app_instance";
pub const MAIN_NAMESPACE: &str = "__main";

pub fn register_modules(ctx: &Ctx<'_>) -> Result<(), ScriptEngineError> {
    registry::register_plugins(ctx)
        .map_err(|(plugin, source)| ScriptEngineError::PluginRegister { plugin, source })
}

pub fn bootstrap_entry_module<'a>(
    ctx: &Ctx<'a>,
    script_code: &str,
    entry_path: &Path,
    globals: &Object<'a>,
) -> Result<(), ScriptEngineError> {
    // The entry module is declared under its real, absolute path (rather than
    // a fixed placeholder name) so that relative `import`s inside it resolve
    // against the project's actual directory structure. See `super::loader`.
    let entry_name = entry_path.to_string_lossy().into_owned();

    let module = Module::declare(ctx.clone(), entry_name, script_code)
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
