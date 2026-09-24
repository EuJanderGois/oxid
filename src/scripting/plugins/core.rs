//! Core scripting module and Entity metadata.

use rquickjs::{Ctx, Module, Result};

use crate::scripting::plugins::{ModuleMeta, ScriptPlugin, TypeConstructorMeta, TypeMeta};

pub struct CorePlugin;

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

impl ScriptPlugin for CorePlugin {
    const NAME: &'static str = NAME;

    fn metadata() -> ModuleMeta {
        META
    }

    fn register<'js>(ctx: &Ctx<'js>) -> Result<()> {
        let source = include_str!("stdlib/Entity.js");
        let module = Module::declare(ctx.clone(), Self::NAME, source)?;
        module.eval()?;
        Ok(())
    }
}
