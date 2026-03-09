use rquickjs::{class::Trace, Class, Ctx, JsLifetime, Result};
use rquickjs::module::{Declarations, Exports, ModuleDef};

use crate::scripting::plugin::NativePlugin;


///
/// vetor de x e y.
/// 
#[rquickjs::class]
#[derive(Clone, Trace, JsLifetime)]
pub struct Transform2D {
    #[qjs(get, set)]
    pub x: f32,
    #[qjs(get, set)]
    pub y: f32,
}

#[rquickjs::methods]
impl Transform2D {
    #[qjs(constructor)]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}


/// 
/// gerencia os métodos e módulos de matemática.
/// 
pub struct MathPlugin;

impl ModuleDef for MathPlugin {
    fn declare<'js>(declare: &Declarations<'js>) -> Result<()> {
        declare.declare("Transform2D")?;
        Ok(())
    } // declara ao script

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        exports.export(
            "Transform2D",
            Class::<Transform2D>::create_constructor(ctx)?,
        )?;
        Ok(())
    } // exporta ao script
}

impl NativePlugin for MathPlugin {
    const NAME: &'static str = "oxid/math";
}