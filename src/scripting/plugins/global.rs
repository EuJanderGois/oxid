//! Built-in globals exposed directly on the JavaScript global object.

use rquickjs::{Ctx, Function, Object, Result, function::Opt};

use super::{FunctionMeta, FunctionParam, GlobalMeta, GlobalPlugin, ScriptType};

pub struct ConsolePlugin;

const MESSAGE_PARAMS: &[FunctionParam] = &[FunctionParam {
    name: "message",
    ty: ScriptType::String,
    docs: "Message to print.",
    optional: true,
}];

const ASSERT_PARAMS: &[FunctionParam] = &[
    FunctionParam {
        name: "condition",
        ty: ScriptType::Boolean,
        docs: "Condition to test.",
        optional: false,
    },
    FunctionParam {
        name: "message",
        ty: ScriptType::String,
        docs: "Message to print when the condition is false.",
        optional: true,
    },
];

const NO_PARAMS: &[FunctionParam] = &[];

const FUNCTIONS: &[FunctionMeta] = &[
    FunctionMeta {
        module: "global",
        name: "console.log",
        docs: "Writes a message to the Oxid console.",
        returns: ScriptType::Void,
        params: MESSAGE_PARAMS,
    },
    FunctionMeta {
        module: "global",
        name: "console.info",
        docs: "Writes an informational message to the Oxid console.",
        returns: ScriptType::Void,
        params: MESSAGE_PARAMS,
    },
    FunctionMeta {
        module: "global",
        name: "console.warn",
        docs: "Writes a warning message to the Oxid console.",
        returns: ScriptType::Void,
        params: MESSAGE_PARAMS,
    },
    FunctionMeta {
        module: "global",
        name: "console.error",
        docs: "Writes an error message to the Oxid console.",
        returns: ScriptType::Void,
        params: MESSAGE_PARAMS,
    },
    FunctionMeta {
        module: "global",
        name: "console.debug",
        docs: "Writes a debug message to the Oxid console.",
        returns: ScriptType::Void,
        params: MESSAGE_PARAMS,
    },
    FunctionMeta {
        module: "global",
        name: "console.assert",
        docs: "Writes a message when the supplied condition is false.",
        returns: ScriptType::Void,
        params: ASSERT_PARAMS,
    },
    FunctionMeta {
        module: "global",
        name: "console.trace",
        docs: "Writes a trace message to the Oxid console.",
        returns: ScriptType::Void,
        params: MESSAGE_PARAMS,
    },
    FunctionMeta {
        module: "global",
        name: "console.clear",
        docs: "Clears the console output when supported by the host.",
        returns: ScriptType::Void,
        params: NO_PARAMS,
    },
];

const META: GlobalMeta = GlobalMeta {
    name: "console",
    docs: "Console utilities available globally.",
    constants: &[],
    functions: FUNCTIONS,
};

fn print_message(level: &str, message: Opt<String>) {
    let message = message.into_inner().unwrap_or_default();
    println!("[{level}] {message}");
}

fn console_log(message: Opt<String>) {
    print_message("log", message);
}

fn console_info(message: Opt<String>) {
    print_message("info", message);
}

fn console_warn(message: Opt<String>) {
    print_message("warn", message);
}

fn console_error(message: Opt<String>) {
    print_message("error", message);
}

fn console_debug(message: Opt<String>) {
    print_message("debug", message);
}

fn console_assert(condition: bool, message: Opt<String>) {
    if !condition {
        print_message("assert", message);
    }
}

fn console_trace(message: Opt<String>) {
    print_message("trace", message);
}

fn console_clear() {}

impl GlobalPlugin for ConsolePlugin {
    const NAME: &'static str = "console";

    fn metadata() -> GlobalMeta {
        META
    }

    fn register<'js>(ctx: &Ctx<'js>) -> Result<()> {
        let console = Object::new(ctx.clone())?;
        console.set("log", Function::new(ctx.clone(), console_log)?)?;
        console.set("info", Function::new(ctx.clone(), console_info)?)?;
        console.set("warn", Function::new(ctx.clone(), console_warn)?)?;
        console.set("error", Function::new(ctx.clone(), console_error)?)?;
        console.set("debug", Function::new(ctx.clone(), console_debug)?)?;
        console.set("assert", Function::new(ctx.clone(), console_assert)?)?;
        console.set("trace", Function::new(ctx.clone(), console_trace)?)?;
        console.set("clear", Function::new(ctx.clone(), console_clear)?)?;
        ctx.globals().set(Self::NAME, console)?;
        Ok(())
    }
}
