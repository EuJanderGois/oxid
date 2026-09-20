use std::fmt;

use crate::i18n;

#[derive(Debug)]
pub enum ScriptEngineError {
    RuntimeInit(String),
    ContextInit(String),
    PluginRegister {
        plugin: &'static str,
        source: String,
    },
    StdlibRegister(String),
    EntryModuleDeclare(String),
    EntryModuleEval(String),
    MainNamespace(String),
    AppInstance(String),
    HookCompile {
        hook: &'static str,
        source: String,
    },
    HookExecution {
        hook: &'static str,
        source: String,
    },
}

impl fmt::Display for ScriptEngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScriptEngineError::RuntimeInit(source) => write!(
                f,
                "{}",
                i18n::text_with("scripting.error.runtime_init", &[("source", source)])
            ),
            ScriptEngineError::ContextInit(source) => write!(
                f,
                "{}",
                i18n::text_with("scripting.error.context_init", &[("source", source)])
            ),
            ScriptEngineError::PluginRegister { plugin, source } => write!(
                f,
                "{}",
                i18n::text_with(
                    "scripting.error.plugin_register",
                    &[("plugin", plugin), ("source", source)],
                )
            ),
            ScriptEngineError::StdlibRegister(source) => write!(
                f,
                "{}",
                i18n::text_with("scripting.error.stdlib_register", &[("source", source)])
            ),
            ScriptEngineError::EntryModuleDeclare(source) => write!(
                f,
                "{}",
                i18n::text_with(
                    "scripting.error.entry_module_declare",
                    &[("source", source)]
                )
            ),
            ScriptEngineError::EntryModuleEval(source) => write!(
                f,
                "{}",
                i18n::text_with("scripting.error.entry_module_eval", &[("source", source)])
            ),
            ScriptEngineError::MainNamespace(source) => write!(
                f,
                "{}",
                i18n::text_with("scripting.error.main_namespace", &[("source", source)])
            ),
            ScriptEngineError::AppInstance(source) => write!(
                f,
                "{}",
                i18n::text_with("scripting.error.app_instance", &[("source", source)])
            ),
            ScriptEngineError::HookCompile { hook, source } => write!(
                f,
                "{}",
                i18n::text_with(
                    "scripting.error.hook_compile",
                    &[("hook", hook), ("source", source)],
                )
            ),
            ScriptEngineError::HookExecution { hook, source } => write!(
                f,
                "{}",
                i18n::text_with(
                    "scripting.error.hook_execution",
                    &[("hook", hook), ("source", source)],
                )
            ),
        }
    }
}

impl std::error::Error for ScriptEngineError {}
