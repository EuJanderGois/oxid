//! Core scripting module and Entity metadata.

use rquickjs::{Ctx, Module, Result};

use super::{
    error::ScriptEngineError,
    plugin::{ModuleMeta, TypeConstructorMeta, TypeMeta},
};

pub const NAME: &str = "oxid/core";

const META: ModuleMeta = ModuleMeta {
    name: NAME,
    docs: "Core application features and the entity lifecycle.",
    types: &[TypeMeta {
        module: NAME,
        name: "Entity",
        docs: "Base class for Oxid script entities.",
        constructors: &[TypeConstructorMeta { params: &[] }],
        properties: &[],
    }],
    functions: &[],
};

pub fn metadata() -> ModuleMeta {
    META
}

pub fn register(ctx: &Ctx<'_>) -> Result<(), ScriptEngineError> {
    let source = include_str!("stdlib/Entity.js");

    let module = Module::declare(ctx.clone(), NAME, source)
        .map_err(|error| ScriptEngineError::StdlibRegister(error.to_string()))?;

    module
        .eval()
        .map_err(|error| ScriptEngineError::StdlibRegister(error.to_string()))?;

    Ok(())
}
