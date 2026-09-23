use rquickjs::{
    Ctx, Function, Result,
    class::OwnedBorrow,
    module::{Declarations, Exports, ModuleDef},
};

use crate::{
    renderer::{
        context::with_active_queue,
        // color::Color as RendererColor
    },
    scripting::plugins::{FunctionMeta, FunctionParam, NativePlugin, ScriptType},
};

use crate::scripting::plugins::{
    color::{Color, to_renderer_color},
    math::Vector2D,
};

fn draw_arc<'js>(
    pos: OwnedBorrow<'js, Vector2D>,
    sides: u8,
    radius: f32,
    rotation: f32,
    thickness: f32,
    arc: f32,
    color: OwnedBorrow<'js, Color>,
) {
    let _ = with_active_queue(|queue| {
        queue.draw_arc(
            pos.x,
            pos.y,
            sides,
            radius,
            rotation,
            thickness,
            arc,
            to_renderer_color(&color),
        );
    });
}

///
/// Uses the renderer to draw a circle.
///
fn draw_circle<'js>(x: f32, y: f32, r: f32, color: OwnedBorrow<'js, Color>) {
    let _ = with_active_queue(|queue| {
        queue.draw_circle(x, y, r, to_renderer_color(&color));
    });
}

///
/// adiciona o comando draw_rectangle a queue
///
fn draw_rectangle<'js>(x: f32, y: f32, width: f32, height: f32, color: OwnedBorrow<'js, Color>) {
    let _ = with_active_queue(|queue| {
        queue.draw_rectangle(x, y, width, height, to_renderer_color(&color));
    });
}

///
/// 2D shapes rendering.
///
pub struct ShapesPlugin;

impl ModuleDef for ShapesPlugin {
    fn declare<'js>(declare: &Declarations<'js>) -> Result<()> {
        declare.declare("drawArc")?;
        declare.declare("drawCircle")?;
        declare.declare("drawRectangle")?;
        Ok(())
    } // declara ao script

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        exports.export("drawArc", Function::new(ctx.clone(), draw_arc)?)?;

        exports.export("drawCircle", Function::new(ctx.clone(), draw_circle)?)?;

        exports.export("drawRectangle", Function::new(ctx.clone(), draw_rectangle)?)?;

        Ok(())
    } // exporta ao script
}

impl NativePlugin for ShapesPlugin {
    const NAME: &'static str = "oxid/shapes";

    fn functions() -> &'static [FunctionMeta] {
        &[
            FunctionMeta {
                module: "oxid/shapes",
                name: "drawArc",
                docs: "Draws an arc on the screen.",
                returns: ScriptType::Void,
                params: &[
                    FunctionParam {
                        name: "position",
                        ty: ScriptType::Custom("oxid/math", "Vector2D"),
                        docs: "Arc position.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "sides",
                        ty: ScriptType::Number,
                        docs: "Resolution used to approximate the curve; higher values produce a smoother arc.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "radius",
                        ty: ScriptType::Number,
                        docs: "Arc radius.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "rotation",
                        ty: ScriptType::Number,
                        docs: "Initial rotation in degrees.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "thickness",
                        ty: ScriptType::Number,
                        docs: "Espessura do arco.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "arc",
                        ty: ScriptType::Number,
                        docs: "Abertura do arco em graus.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "color",
                        ty: ScriptType::Custom("oxid/color", "Color"),
                        docs: "Color used for drawing.",
                        optional: false,
                    },
                ],
            },
            FunctionMeta {
                module: "oxid/shapes",
                name: "drawCircle",
                docs: "Draws a circle on the screen.",
                returns: ScriptType::Void,
                params: &[
                    FunctionParam {
                        name: "x",
                        ty: ScriptType::Number,
                        docs: "Horizontal position.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "y",
                        ty: ScriptType::Number,
                        docs: "Vertical position.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "radius",
                        ty: ScriptType::Number,
                        docs: "Circle radius.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "color",
                        ty: ScriptType::Custom("oxid/color", "Color"),
                        docs: "Color used for drawing.",
                        optional: false,
                    },
                ],
            },
            FunctionMeta {
                module: "oxid/shapes",
                name: "drawRectangle",
                docs: "Draws a rectangle on the screen.",
                returns: ScriptType::Void,
                params: &[
                    FunctionParam {
                        name: "x",
                        ty: ScriptType::Number,
                        docs: "Horizontal position.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "y",
                        ty: ScriptType::Number,
                        docs: "Vertical position.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "width",
                        ty: ScriptType::Number,
                        docs: "Rectangle width.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "height",
                        ty: ScriptType::Number,
                        docs: "Rectangle height.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "color",
                        ty: ScriptType::Custom("oxid/color", "Color"),
                        docs: "Color used for drawing.",
                        optional: false,
                    },
                ],
            },
        ]
    }
}
