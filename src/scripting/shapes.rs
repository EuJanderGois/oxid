use rquickjs::{
    class::OwnedBorrow,
    module::{Declarations, Exports, ModuleDef},
    Ctx,
    Function,
    Result,
};

use crate::{
    renderer::{
        context::with_active_queue,
        // color::Color as RendererColor
    },
    scripting::plugin::{FunctionMeta, FunctionParam, NativePlugin, ScriptType},
};

use crate::scripting::{
    color::{Color, to_renderer_color},
    math::Transform2D,
};

fn draw_arc<'js>(
    pos: OwnedBorrow<'js, Transform2D>,
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
/// usa o renderizador para desenhar um círculo.
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
        exports.export(
            "drawArc",
            Function::new(ctx.clone(), draw_arc)?)?; 
        
        exports.export(
            "drawCircle", 
            Function::new(ctx.clone(), draw_circle)?)?;

        exports.export(
            "drawRectangle", 
            Function::new(ctx.clone(), draw_rectangle)?)?;
        
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
                docs: "Desenha um arco na tela.",
                returns: ScriptType::Void,
                params: &[
                    FunctionParam { name: "position", ty: ScriptType::Custom("Transform2D"), docs: "Posição do arco.", optional: false },
                    FunctionParam { name: "sides", ty: ScriptType::Number, docs: "Resolução usada para aproximar a curva; valores maiores deixam o arco mais suave.", optional: false },
                    FunctionParam { name: "radius", ty: ScriptType::Number, docs: "Raio do arco.", optional: false },
                    FunctionParam { name: "rotation", ty: ScriptType::Number, docs: "Rotação inicial em graus.", optional: false },
                    FunctionParam { name: "thickness", ty: ScriptType::Number, docs: "Espessura do arco.", optional: false },
                    FunctionParam { name: "arc", ty: ScriptType::Number, docs: "Abertura do arco em graus.", optional: false },
                    FunctionParam { name: "color", ty: ScriptType::Custom("Color"), docs: "Cor usada no desenho.", optional: false },
                ],
            },
            FunctionMeta {
                module: "oxid/shapes",
                name: "drawCircle",
                docs: "Desenha um círculo na tela.",
                returns: ScriptType::Void,
                params: &[
                    FunctionParam { name: "x", ty: ScriptType::Number, docs: "Posição horizontal.", optional: false },
                    FunctionParam { name: "y", ty: ScriptType::Number, docs: "Posição vertical.", optional: false },
                    FunctionParam { name: "radius", ty: ScriptType::Number, docs: "Raio do círculo.", optional: false },
                    FunctionParam { name: "color", ty: ScriptType::Custom("Color"), docs: "Cor usada no desenho.", optional: false },
                ],
            },
            FunctionMeta {
                module: "oxid/shapes",
                name: "drawRectangle",
                docs: "Desenha um retângulo na tela.",
                returns: ScriptType::Void,
                params: &[
                    FunctionParam { name: "x", ty: ScriptType::Number, docs: "Posição horizontal.", optional: false },
                    FunctionParam { name: "y", ty: ScriptType::Number, docs: "Posição vertical.", optional: false },
                    FunctionParam { name: "width", ty: ScriptType::Number, docs: "Largura do retângulo.", optional: false },
                    FunctionParam { name: "height", ty: ScriptType::Number, docs: "Altura do retângulo.", optional: false },
                    FunctionParam { name: "color", ty: ScriptType::Custom("Color"), docs: "Cor usada no desenho.", optional: false },
                ],
            }
        ]
    }
}
