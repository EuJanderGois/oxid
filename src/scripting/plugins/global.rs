//! Built-in globals exposed directly on the JavaScript global object.

use rquickjs::{Ctx, Function, Object, Result};

use super::{
    FunctionMeta, FunctionParam, GlobalPlugin, GlobalMeta, ScriptType,
};

pub struct ConsolePlugin;

const LOG_PARAMS: &[FunctionParam] = &[FunctionParam {
    name: "message",
    ty: ScriptType::String,
    docs: "Message to print.",
    optional: false,
}];

const FUNCTIONS: &[FunctionMeta] = &[FunctionMeta {
    module: "global",
    name: "console.log",
    docs: "Writes a message to the Oxid console.",
    returns: ScriptType::Void,
    params: LOG_PARAMS,
}];

const META: GlobalMeta = GlobalMeta {
    name: "console",
    docs: "Console utilities available globally.",
    constants: &[],
    functions: FUNCTIONS,
};

fn console_log(message: String) -> Result<()> {
    println!("{message}");
    Ok(())
}

impl GlobalPlugin for ConsolePlugin {
    const NAME: &'static str = "console";

    fn metadata() -> GlobalMeta {
        META
    }

    fn register<'js>(ctx: &Ctx<'js>) -> Result<()> {
        let console = Object::new(ctx.clone())?;
        console.set("log", Function::new(ctx.clone(), console_log)?)?;
        ctx.globals().set(Self::NAME, console)?;
        Ok(())
    }
}
