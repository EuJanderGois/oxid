use rquickjs::module::{Declarations, Exports, ModuleDef};
use rquickjs::{Class, Ctx, JsLifetime, Result, class::Trace};

use crate::scripting::plugin::NativePlugin;

///
/// vetor de x e y.
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
/// gerencia os métodos e módulos de cores.
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
}
