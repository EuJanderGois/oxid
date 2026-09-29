use macroquad::prelude::{
    screen_height as mq_screen_height, 
    screen_width as mq_screen_width,
    request_new_screen_size as mq_set_window_size
};
use rquickjs::{
    Ctx, Function, Result,
    module::{Declarations, Exports, ModuleDef},
};

use crate::scripting::plugins::{
    FunctionMeta, FunctionParam, ScriptPlugin, ScriptType, math::Vector2D, register_module_def,
};

// classes

// functions
fn get_window_width<'js>() -> Result<f32> {
    Ok(mq_screen_width())
}

fn get_window_height<'js>() -> Result<f32> {
    Ok(mq_screen_height())
}

fn set_window_width<'js>(width: f32) -> Result<()> {
    Ok(mq_set_window_size(width, mq_screen_height()))
}

fn set_window_height<'js>(height: f32) -> Result<()> {
    Ok(mq_set_window_size(mq_screen_width(), height))
}

fn get_window_size<'js>() -> Result<Vector2D> {
    Ok(Vector2D {
        x: mq_screen_width(),
        y: mq_screen_height(),
    })
}

fn set_window_size<'js>(size: Vector2D) -> Result<()> {
    Ok(mq_set_window_size(size.x, size.y))
}

pub struct WindowPlugin;

impl ModuleDef for WindowPlugin {
    fn declare<'js>(declare: &Declarations<'js>) -> Result<()> {
        declare.declare("getWindowWidth")?;
        declare.declare("getWindowHeight")?;
        declare.declare("getWindowSize")?;
        declare.declare("setWindowWidth")?;
        declare.declare("setWindowHeight")?;
        declare.declare("setWindowSize")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        exports.export(
            "getWindowWidth",
            Function::new(ctx.clone(), get_window_width)?,
        )?;
        exports.export(
            "getWindowHeight",
            Function::new(ctx.clone(), get_window_height)?,
        )?;
        exports.export(
            "getWindowSize",
            Function::new(ctx.clone(), get_window_size)?,
        )?;
        exports.export(
            "setWindowWidth", 
            Function::new(ctx.clone(), set_window_width)?,
        )?;
        exports.export(
            "setWindowHeight", 
            Function::new(ctx.clone(), set_window_height)?
        )?;
        exports.export(
            "setWindowSize", 
            Function::new(ctx.clone(), set_window_size)?
        )?;

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
                name: "getWindowWidth",
                docs: "Gets window width.",
                returns: ScriptType::Number,
                params: &[],
            },
            FunctionMeta {
                module: "oxid/window",
                name: "getWindowHeight",
                docs: "Gets window height.",
                returns: ScriptType::Number,
                params: &[],
            },
            FunctionMeta {
                module: "oxid/window",
                name: "setWindowWidth",
                docs: "Sets the window width.",
                returns: ScriptType::Void,
                params: &[FunctionParam {
                    name: "width",
                    ty: ScriptType::Number,
                    docs: "The new window width",
                    optional: false,
                }]
            },
            FunctionMeta {
                module: "oxid/window",
                name: "setWindowHeight",
                docs: "Sets the window height.",
                returns: ScriptType::Void,
                params: &[FunctionParam {
                    name: "height",
                    ty: ScriptType::Number,
                    docs: "The new window height",
                    optional: false,
                }]
            },
            FunctionMeta {
                module: "oxid/window",
                name: "getWindowSize",
                docs: "Gets the canvas size.",
                returns: ScriptType::Custom("oxid/math", "Vector2D"),
                params: &[],
            },
            FunctionMeta {
                module: "oxid/window",
                name: "setWindowSize",
                docs: "Sets the window size.",
                returns: ScriptType::Void,
                params: &[FunctionParam {
                    name: "size",
                    ty: ScriptType::Custom("oxid/math", "Vector2D"),
                    docs: "The new window size",
                    optional: false,
                }]
            },
        ]
    }
}
