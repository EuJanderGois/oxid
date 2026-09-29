//! Oxid's scripting layer.
//!
//! The engine depends on [`ScriptRuntime`], not on any particular JavaScript
//! engine. [`quickjs::QuickJsRuntime`] is the current (and, for now, only)
//! implementation, built on top of `rquickjs`/QuickJS; see the module
//! documentation on [`runtime`] and [`quickjs`] for the reasoning behind the
//! split.
//!
//! [`plugins`] and [`generator`] (the `oxid.d.ts` generator) describe the
//! Oxid-native `oxid/*` API surface. That description (see
//! [`plugins::ModuleMeta`] and friends) has no dependency on QuickJS at all;
//! only the plugins' *bindings* (how each function/class is actually wired
//! into a running script context) are backend-specific, and today that means
//! `rquickjs`.

pub mod error;
pub mod generator;
pub mod plugins;
pub mod quickjs;
pub mod resolver;
pub mod runtime;

pub use error::ScriptError;
pub use generator::generate_d_ts;
pub use quickjs::QuickJsRuntime;
pub use runtime::ScriptRuntime;

pub fn api_metadata() -> Vec<plugins::ModuleMeta> {
    plugins::registry::api_metadata()
}

pub fn generate_api_d_ts() -> String {
    generator::generate_d_ts_with_globals(&api_metadata(), &plugins::registry::global_metadata())
}
