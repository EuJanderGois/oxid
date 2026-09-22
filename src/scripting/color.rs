use rquickjs::module::{Declarations, Exports, ModuleDef};
use rquickjs::{Class, Ctx, JsLifetime, Result, class::Trace};

use crate::scripting::plugin::{
    FunctionParam, NativePlugin, ScriptType, TypeConstructorMeta, TypeMeta, TypePropertyMeta,
};

///
/// Represents a two-dimensional vector with x and y components.
///
#[rquickjs::class]
#[derive(Clone, Trace, JsLifetime)]
pub struct Color {
    #[qjs(get, set)]
    pub r: f32,
    #[qjs(get, set)]
    pub g: f32,
    #[qjs(get, set)]
    pub b: f32,
    #[qjs(get, set)]
    pub a: f32,
}

#[rquickjs::methods]
impl Color {
    #[qjs(constructor)]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

use crate::renderer::color::Color as RendererColor;
pub fn to_renderer_color(color: &Color) -> RendererColor {
    RendererColor::new(color.r, color.g, color.b, color.a)
}

///
/// Provides color types and module bindings.
///
pub struct ColorPlugin;

impl ModuleDef for ColorPlugin {
    fn declare<'js>(declare: &Declarations<'js>) -> Result<()> {
        declare.declare("Color")?;
        Ok(())
    } // declara ao script

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        exports.export("Color", Class::<Color>::create_constructor(ctx)?)?;
        Ok(())
    } // exporta ao script
}

impl NativePlugin for ColorPlugin {
    const NAME: &'static str = "oxid/color";

    fn docs() -> &'static str {
        "Types for representing RGBA colors."
    }

    fn types() -> &'static [TypeMeta] {
        static TYPES: [TypeMeta; 1] = [TypeMeta {
            module: "oxid/color",
            name: "Color",
            docs: "Mutable RGBA color.",
            constructors: &[TypeConstructorMeta {
                params: &[
                    FunctionParam {
                        name: "r",
                        ty: ScriptType::Number,
                        docs: "Red component.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "g",
                        ty: ScriptType::Number,
                        docs: "Green component.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "b",
                        ty: ScriptType::Number,
                        docs: "Blue component.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "a",
                        ty: ScriptType::Number,
                        docs: "Alpha component.",
                        optional: false,
                    },
                ],
            }],
            properties: &[
                TypePropertyMeta {
                    name: "r",
                    ty: ScriptType::Number,
                    docs: "Red component.",
                    readonly: false,
                },
                TypePropertyMeta {
                    name: "g",
                    ty: ScriptType::Number,
                    docs: "Green component.",
                    readonly: false,
                },
                TypePropertyMeta {
                    name: "b",
                    ty: ScriptType::Number,
                    docs: "Blue component.",
                    readonly: false,
                },
                TypePropertyMeta {
                    name: "a",
                    ty: ScriptType::Number,
                    docs: "Alpha component.",
                    readonly: false,
                },
            ],
        }];
        &TYPES
    }
}
