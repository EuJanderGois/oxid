use rquickjs::module::{Declarations, Exports, ModuleDef};
use rquickjs::{Class, Ctx, JsLifetime, Result, class::Trace};

use crate::scripting::plugins::{
    FunctionParam, GlobalConstantMeta, GlobalMeta, GlobalPlugin, ScriptPlugin, ScriptType,
    TypeConstructorMeta, TypeMeta, TypePropertyMeta, register_module_def,
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

use crate::renderer::color::{self as renderer_color, Color as RendererColor};
pub fn to_renderer_color(color: &Color) -> RendererColor {
    RendererColor::new(color.r, color.g, color.b, color.a)
}

const GLOBAL_CONSTANTS: &[GlobalConstantMeta] = &[
    GlobalConstantMeta {
        name: "LIGHTGRAY",
        docs: "Light gray color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "GRAY",
        docs: "Gray color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "DARKGRAY",
        docs: "Dark gray color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "YELLOW",
        docs: "Yellow color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "GOLD",
        docs: "Gold color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "ORANGE",
        docs: "Orange color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "PINK",
        docs: "Pink color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "RED",
        docs: "Red color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "MAROON",
        docs: "Maroon color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "GREEN",
        docs: "Green color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "LIME",
        docs: "Lime color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "DARKGREEN",
        docs: "Dark green color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "SKYBLUE",
        docs: "Sky blue color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "BLUE",
        docs: "Blue color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "DARKBLUE",
        docs: "Dark blue color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "PURPLE",
        docs: "Purple color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "VIOLET",
        docs: "Violet color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "DARKPURPLE",
        docs: "Dark purple color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "BEIGE",
        docs: "Beige color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "BROWN",
        docs: "Brown color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "DARKBROWN",
        docs: "Dark brown color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "WHITE",
        docs: "White color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "BLACK",
        docs: "Black color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "BLANK",
        docs: "Fully transparent color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
    GlobalConstantMeta {
        name: "MAGENTA",
        docs: "Magenta color.",
        ty: ScriptType::Custom("oxid/color", "Color"),
    },
];

fn register_global_color<'js>(ctx: &Ctx<'js>, name: &str, color: RendererColor) -> Result<()> {
    ctx.globals().set(
        name,
        Class::instance(ctx.clone(), Color::new(color.r, color.g, color.b, color.a))?,
    )?;
    Ok(())
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

impl GlobalPlugin for ColorPlugin {
    const NAME: &'static str = "colors";

    fn metadata() -> GlobalMeta {
        GlobalMeta {
            name: <ColorPlugin as GlobalPlugin>::NAME,
            docs: "Standard colors available globally.",
            constants: GLOBAL_CONSTANTS,
            functions: &[],
        }
    }

    fn register<'js>(ctx: &Ctx<'js>) -> Result<()> {
        register_global_color(ctx, "LIGHTGRAY", renderer_color::LIGHTGRAY)?;
        register_global_color(ctx, "GRAY", renderer_color::GRAY)?;
        register_global_color(ctx, "DARKGRAY", renderer_color::DARKGRAY)?;
        register_global_color(ctx, "YELLOW", renderer_color::YELLOW)?;
        register_global_color(ctx, "GOLD", renderer_color::GOLD)?;
        register_global_color(ctx, "ORANGE", renderer_color::ORANGE)?;
        register_global_color(ctx, "PINK", renderer_color::PINK)?;
        register_global_color(ctx, "RED", renderer_color::RED)?;
        register_global_color(ctx, "MAROON", renderer_color::MAROON)?;
        register_global_color(ctx, "GREEN", renderer_color::GREEN)?;
        register_global_color(ctx, "LIME", renderer_color::LIME)?;
        register_global_color(ctx, "DARKGREEN", renderer_color::DARKGREEN)?;
        register_global_color(ctx, "SKYBLUE", renderer_color::SKYBLUE)?;
        register_global_color(ctx, "BLUE", renderer_color::BLUE)?;
        register_global_color(ctx, "DARKBLUE", renderer_color::DARKBLUE)?;
        register_global_color(ctx, "PURPLE", renderer_color::PURPLE)?;
        register_global_color(ctx, "VIOLET", renderer_color::VIOLET)?;
        register_global_color(ctx, "DARKPURPLE", renderer_color::DARKPURPLE)?;
        register_global_color(ctx, "BEIGE", renderer_color::BEIGE)?;
        register_global_color(ctx, "BROWN", renderer_color::BROWN)?;
        register_global_color(ctx, "DARKBROWN", renderer_color::DARKBROWN)?;
        register_global_color(ctx, "WHITE", renderer_color::WHITE)?;
        register_global_color(ctx, "BLACK", renderer_color::BLACK)?;
        register_global_color(ctx, "BLANK", renderer_color::BLANK)?;
        register_global_color(ctx, "MAGENTA", renderer_color::MAGENTA)?;
        Ok(())
    }
}

impl ScriptPlugin for ColorPlugin {
    fn register<'js>(ctx: &Ctx<'js>) -> Result<()> {
        register_module_def::<Self>(ctx, <ColorPlugin as ScriptPlugin>::NAME)
    }

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
