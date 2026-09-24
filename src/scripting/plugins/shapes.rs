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
    scripting::plugins::{
        FunctionMeta, FunctionParam, ScriptPlugin, ScriptType, register_module_def,
    },
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

fn draw_line<'js>(
    start: OwnedBorrow<'js, Vector2D>,
    end: OwnedBorrow<'js, Vector2D>,
    thickness: f32,
    color: OwnedBorrow<'js, Color>,
) {
    let _ = with_active_queue(|queue| {
        queue.draw_line(
            start.x,
            start.y,
            end.x,
            end.y,
            thickness,
            to_renderer_color(&color),
        );
    });
}

fn draw_triangle_lines<'js>(
    v1: Vector2D,
    v2: Vector2D,
    v3: Vector2D,
    thickness: f32,
    color: OwnedBorrow<'js, Color>,
) {
    let _ = with_active_queue(|queue| {
        queue.draw_triangle_lines(v1, v2, v3, thickness, to_renderer_color(&color));
    });
}

fn draw_polygon_lines<'js>(
    position: OwnedBorrow<'js, Vector2D>,
    sides: u8,
    radius: f32,
    rotation: f32,
    thickness: f32,
    color: OwnedBorrow<'js, Color>,
) {
    let _ = with_active_queue(|queue| {
        queue.draw_polygon_lines(
            position.x,
            position.y,
            sides,
            radius,
            rotation,
            thickness,
            to_renderer_color(&color),
        );
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
        declare.declare("drawLine")?;
        declare.declare("drawTriangleLines")?;
        declare.declare("drawPolygonLines")?;
        Ok(())
    } // declara ao script

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        exports.export("drawArc", Function::new(ctx.clone(), draw_arc)?)?;

        exports.export("drawCircle", Function::new(ctx.clone(), draw_circle)?)?;

        exports.export("drawRectangle", Function::new(ctx.clone(), draw_rectangle)?)?;

        exports.export("drawLine", Function::new(ctx.clone(), draw_line)?)?;

        exports.export(
            "drawTriangleLines",
            Function::new(ctx.clone(), draw_triangle_lines)?,
        )?;

        exports.export(
            "drawPolygonLines",
            Function::new(ctx.clone(), draw_polygon_lines)?,
        )?;

        Ok(())
    } // exporta ao script
}

impl ScriptPlugin for ShapesPlugin {
    fn register<'js>(ctx: &Ctx<'js>) -> Result<()> {
        register_module_def::<Self>(ctx, Self::NAME)
    }

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
            FunctionMeta {
                module: "oxid/shapes",
                name: "drawLine",
                docs: "Draws a line between two points.",
                returns: ScriptType::Void,
                params: &[
                    FunctionParam {
                        name: "start",
                        ty: ScriptType::Custom("oxid/math", "Vector2D"),
                        docs: "Starting point of the line.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "end",
                        ty: ScriptType::Custom("oxid/math", "Vector2D"),
                        docs: "Ending point of the line.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "thickness",
                        ty: ScriptType::Number,
                        docs: "Line thickness.",
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
                name: "drawTriangleLines",
                docs: "Draws the outline of a triangle.",
                returns: ScriptType::Void,
                params: &[
                    FunctionParam {
                        name: "v1",
                        ty: ScriptType::Custom("oxid/math", "Vector2D"),
                        docs: "First vertex.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "v2",
                        ty: ScriptType::Custom("oxid/math", "Vector2D"),
                        docs: "Second vertex.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "v3",
                        ty: ScriptType::Custom("oxid/math", "Vector2D"),
                        docs: "Third vertex.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "thickness",
                        ty: ScriptType::Number,
                        docs: "Line thickness.",
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
                name: "drawPolygonLines",
                docs: "Draws the outline of a regular polygon.",
                returns: ScriptType::Void,
                params: &[
                    FunctionParam {
                        name: "position",
                        ty: ScriptType::Custom("oxid/math", "Vector2D"),
                        docs: "Center position of the polygon.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "sides",
                        ty: ScriptType::Number,
                        docs: "Number of polygon sides.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "radius",
                        ty: ScriptType::Number,
                        docs: "Distance from the center to each vertex.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "rotation",
                        ty: ScriptType::Number,
                        docs: "Polygon rotation in degrees.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "thickness",
                        ty: ScriptType::Number,
                        docs: "Line thickness.",
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
