pub mod bootstrap;
pub mod engine;
pub mod error;
pub mod generator;
pub mod hooks;

pub mod plugins;

pub use engine::ScriptEngine;
pub use generator::generate_d_ts;

pub fn api_metadata() -> Vec<plugins::ModuleMeta> {
    plugins::registry::api_metadata()
}

pub fn generate_api_d_ts() -> String {
    generator::generate_d_ts(&api_metadata())
}
