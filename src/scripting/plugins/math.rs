use rquickjs::module::{Declarations, Exports, ModuleDef};
use rquickjs::{Class, Ctx, JsLifetime, Result, class::Trace};

use crate::scripting::plugins::{
    FunctionParam, ScriptPlugin, ScriptType, TypeConstructorMeta, TypeMeta, TypePropertyMeta,
    register_module_def,
};

///
/// Represents a two-dimensional vector with x and y components.
///
#[rquickjs::class]
#[derive(Clone, Trace, JsLifetime)]
pub struct Vector2D {
    #[qjs(get, set)]
    pub x: f32,
    #[qjs(get, set)]
    pub y: f32,
}

#[rquickjs::methods]
impl Vector2D {
    #[qjs(constructor)]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

///
/// Provides mathematical types and module bindings.
///
pub struct MathPlugin;

impl ModuleDef for MathPlugin {
    fn declare<'js>(declare: &Declarations<'js>) -> Result<()> {
        declare.declare("Vector2D")?;
        Ok(())
    } // declara ao script

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        exports.export("Vector2D", Class::<Vector2D>::create_constructor(ctx)?)?;
        Ok(())
    } // exporta ao script
}

impl ScriptPlugin for MathPlugin {
    fn register<'js>(ctx: &Ctx<'js>) -> Result<()> {
        register_module_def::<Self>(ctx, Self::NAME)
    }

    const NAME: &'static str = "oxid/math";

    fn docs() -> &'static str {
        "2D mathematical types and utilities."
    }

    fn types() -> &'static [TypeMeta] {
        static TYPES: [TypeMeta; 1] = [TypeMeta {
            module: "oxid/math",
            name: "Vector2D",
            docs: "Mutable 2D vector with x and y components.",
            constructors: &[TypeConstructorMeta {
                params: &[
                    FunctionParam {
                        name: "x",
                        ty: ScriptType::Number,
                        docs: "Horizontal component.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "y",
                        ty: ScriptType::Number,
                        docs: "Vertical component.",
                        optional: false,
                    },
                ],
            }],
            properties: &[
                TypePropertyMeta {
                    name: "x",
                    ty: ScriptType::Number,
                    docs: "Horizontal component.",
                    readonly: false,
                },
                TypePropertyMeta {
                    name: "y",
                    ty: ScriptType::Number,
                    docs: "Vertical component.",
                    readonly: false,
                },
            ],
        }];
        &TYPES
    }
}
