//! Oxid's own scripting-layer error type.
//!
//! Every variant here is a string-based, backend-agnostic description of
//! what went wrong (e.g. "the runtime failed to initialize", "a hook failed
//! to compile"); no `rquickjs` type appears anywhere in this file. Backends
//! (currently only [`super::quickjs`]) are responsible for converting their
//! own error types to a `String` at the point they construct one of these
//! variants, so this error can flow through the rest of the engine without
//! dragging QuickJS-specific types along with it.

use std::fmt;

use crate::i18n;

#[derive(Debug)]
pub enum ScriptError {
    ModuleRootInit(String),
    RuntimeInit(String),
    ContextInit(String),
    PluginRegister { plugin: String, source: String },
    EntryModuleDeclare(String),
    EntryModuleEval(String),
    MainNamespace(String),
    AppInstance(String),
    HookCompile { hook: &'static str, source: String },
    HookExecution { hook: &'static str, source: String },
}

impl fmt::Display for ScriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScriptError::ModuleRootInit(source) => write!(
                f,
                "{}",
                i18n::text_with("scripting.error.module_root_init", &[("source", source)])
            ),
            ScriptError::RuntimeInit(source) => write!(
                f,
                "{}",
                i18n::text_with("scripting.error.runtime_init", &[("source", source)])
            ),
            ScriptError::ContextInit(source) => write!(
                f,
                "{}",
                i18n::text_with("scripting.error.context_init", &[("source", source)])
            ),
            ScriptError::PluginRegister { plugin, source } => write!(
                f,
                "{}",
                i18n::text_with(
                    "scripting.error.plugin_register",
                    &[("plugin", plugin), ("source", source)],
                )
            ),
            ScriptError::EntryModuleDeclare(source) => write!(
                f,
                "{}",
                i18n::text_with(
                    "scripting.error.entry_module_declare",
                    &[("source", source)]
                )
            ),
            ScriptError::EntryModuleEval(source) => write!(
                f,
                "{}",
                i18n::text_with("scripting.error.entry_module_eval", &[("source", source)])
            ),
            ScriptError::MainNamespace(source) => write!(
                f,
                "{}",
                i18n::text_with("scripting.error.main_namespace", &[("source", source)])
            ),
            ScriptError::AppInstance(source) => write!(
                f,
                "{}",
                i18n::text_with("scripting.error.app_instance", &[("source", source)])
            ),
            ScriptError::HookCompile { hook, source } => write!(
                f,
                "{}",
                i18n::text_with(
                    "scripting.error.hook_compile",
                    &[("hook", hook), ("source", source)],
                )
            ),
            ScriptError::HookExecution { hook, source } => write!(
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

impl std::error::Error for ScriptError {}
