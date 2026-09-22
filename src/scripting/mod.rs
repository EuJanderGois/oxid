pub mod bootstrap;
pub mod color;
pub mod core;
pub mod engine;
pub mod error;
pub mod generator;
pub mod hooks;
pub mod input;
pub mod math;
pub mod plugin;
pub mod registry;
pub mod shapes;
pub mod text;
pub mod texture;

pub use engine::ScriptEngine;
pub use generator::generate_d_ts;

pub fn api_metadata() -> Vec<plugin::ModuleMeta> {
    registry::api_metadata()
}

pub fn generate_api_d_ts() -> String {
    generator::generate_d_ts(&api_metadata())
}
