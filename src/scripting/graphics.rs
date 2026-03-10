use rquickjs::{module::{Declarations, Exports, ModuleDef}, Ctx, Function, Result};

use crate::{
    renderer::{
        context::with_active_queue,
        // color::Color as RendererColor
    },
    scripting::plugin::{FunctionMeta, FunctionParam, NativePlugin, ScriptType},
};

use crate::scripting::color::{Color, to_renderer_color};

///
/// usa o renderizador para desenhar um círculo.
/// 
fn draw_circle(x: f32, y: f32, r: f32, color: Color) {
    let _ = with_active_queue(|queue| {
        queue.draw_circle(x, y, r, to_renderer_color(color));
    });
}

///
/// adiciona o comando draw_rectangle a queue
/// 
fn draw_rectangle(x: f32, y: f32, width: f32, height: f32, color: Color) {
    let _ = with_active_queue(|queue| {
        queue.draw_rectangle(x, y, width, height, to_renderer_color(color));
    });
}

/// 
/// gerencia os métodos e módulos de renderização
/// 
pub struct GraphicsPlugin;

impl ModuleDef for GraphicsPlugin {
    fn declare<'js>(declare: &Declarations<'js>) -> Result<()> {
        declare.declare("drawCircle")?;
        declare.declare("drawRectangle")?;
        Ok(())
    } // declara ao script

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        exports.export(
            "drawCircle", 
            Function::new(ctx.clone(), draw_circle)?)?;

        exports.export(
            "drawRectangle", 
            Function::new(ctx.clone(), draw_rectangle)?)?;
        Ok(())
    } // exporta ao script
}

impl NativePlugin for GraphicsPlugin {
    const NAME: &'static str = "oxid/graphics";

    fn functions() -> &'static [FunctionMeta] {
        &[
            FunctionMeta {
                module: "oxid/graphics",
                name: "drawCircle",
                docs: "Desenha um círculo na tela.",
                returns: ScriptType::Void,
                params: &[
                    FunctionParam { name: "x", ty: ScriptType::Number, docs: "Posição horizontal.", optional: false },
                    FunctionParam { name: "y", ty: ScriptType::Number, docs: "Posição vertical.", optional: false },
                    FunctionParam { name: "radius", ty: ScriptType::Number, docs: "Raio do círculo.", optional: false },
                ],
            }
        ]
    }
}