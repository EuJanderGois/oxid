use rquickjs::{module::{Declarations, Exports, ModuleDef}, Ctx, Function, Result};
use macroquad::prelude as mq;
use crate::scripting::plugin::{NativePlugin, FunctionMeta, FunctionParam, ScriptType};

///
/// usa o renderizador para desenhar um círculo.
/// 
fn draw_circle(x: f32, y: f32, r: f32) {
    mq::draw_circle(x, y, r, mq::ORANGE);
}

/// 
/// gerencia os métodos e módulos de renderização
/// 
pub struct GraphicsPlugin;

impl ModuleDef for GraphicsPlugin {
    fn declare<'js>(declare: &Declarations<'js>) -> Result<()> {
        declare.declare("drawCircle")?;
        Ok(())
    } // declara ao script

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        exports.export(
            "drawCircle", 
            Function::new(ctx.clone(), draw_circle)?)?;
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