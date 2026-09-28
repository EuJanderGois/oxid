

use macroquad::prelude::{
    screen_width as mq_screen_width,
    screen_height as mq_screen_height
};
use rquickjs::{
    Result, Function, Ctx,
    module::{Declarations, Exports, ModuleDef}
};

use crate::scripting::plugins::{FunctionMeta, ScriptPlugin, ScriptType, math::Vector2D, register_module_def};


// classes

// functions
fn get_canvas_width<'js>() -> Result<f32> {
    Ok(mq_screen_width())
}

fn get_canvas_height<'js>() -> Result<f32> {
    Ok(mq_screen_height())
}

fn get_canvas_size<'js>() -> Result<Vector2D> {
    Ok(Vector2D { x: mq_screen_width(), y: mq_screen_height() })
}

pub struct WindowPlugin;

impl ModuleDef for WindowPlugin {
    fn declare<'js>(declare: &Declarations<'js>) -> Result<()> {
        declare.declare("getCanvasWidth")?;
        declare.declare("getCanvasHeight")?;
        declare.declare("getCanvasSize")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        exports.export("getCanvasWidth", Function::new(ctx.clone(), get_canvas_width)?)?;
        exports.export("getCanvasHeight", Function::new(ctx.clone(), get_canvas_height)?)?;
        exports.export("getCanvasSize", Function::new(ctx.clone(), get_canvas_size)?)?;
        Ok(())
    }
}

impl ScriptPlugin for WindowPlugin {
    fn register<'js>(ctx: &rquickjs::prelude::Ctx<'js>) -> rquickjs::Result<()> {
        register_module_def::<Self>(ctx, Self::NAME)
    }

    const NAME: &'static str = "oxid/window";

    fn docs() -> &'static str {
        "Window and related helpers."
    }

    fn functions() -> &'static [super::FunctionMeta] {
        &[
            FunctionMeta {
                module: "oxid/window",
                name: "getCanvasWidth",
                docs: "Gets canvas width.",
                returns: ScriptType::Number,
                params: &[],
            },
            FunctionMeta {
                module: "oxid/window",
                name: "getCanvasHeight",
                docs: "Gets canvas height.",
                returns: ScriptType::Number,
                params: &[],
            },
            FunctionMeta {
                module: "oxid/window",
                name: "getCanvasSize",
                docs: "Gets the canvas size.",
                returns: ScriptType::Custom("oxid/math", "Vector2D"),
                params: &[],
            },
        ]
    }
}