pub mod math;
pub mod shapes;
pub mod color;
pub mod input;
pub mod text;
pub mod texture;
pub mod plugin;
pub mod error;

use rquickjs::{CaughtError, Context, Ctx, Function, Module, Object, Runtime};
use plugin::NativePlugin;
use math::MathPlugin;
use shapes::ShapesPlugin;
use input::InputPlugin;
use text::TextPlugin;
use texture::TexturePlugin;
use error::ScriptEngineError;

use crate::{renderer::{
    context::{clear_active_queue, set_active_queue},
    queue::RenderQueue,
}, scripting::color::ColorPlugin};

const HOOK_ON_INIT: &str = "__hook_on_init";
const HOOK_ON_UPDATE: &str = "__hook_on_update";
const HOOK_ON_DRAW: &str = "__hook_on_draw";
const APP_INSTANCE: &str = "__app_instance";
const MAIN_NAMESPACE: &str = "__main";

pub struct ScriptEngine {
    _rt: Runtime,
    ctx: Context,
}

impl ScriptEngine {
    fn hook_error(ctx: &Ctx<'_>, hook_name: &'static str, error: rquickjs::Error) -> ScriptEngineError {
        let source = CaughtError::from_error(ctx, error).to_string();

        ScriptEngineError::HookExecution {
            hook: hook_name,
            source,
        }
    }

    fn register_plugins(ctx: &Ctx<'_>) -> Result<(), ScriptEngineError> {
        MathPlugin::register(ctx).map_err(|e| ScriptEngineError::PluginRegister {
            plugin: MathPlugin::NAME,
            source: e.to_string(),
        })?;

        ShapesPlugin::register(ctx).map_err(|e| ScriptEngineError::PluginRegister {
            plugin: ShapesPlugin::NAME,
            source: e.to_string(),
        })?;

        ColorPlugin::register(ctx).map_err(|e| ScriptEngineError::PluginRegister {
            plugin: ColorPlugin::NAME,
            source: e.to_string(),
        })?;

        InputPlugin::register(ctx).map_err(|e| ScriptEngineError::PluginRegister {
            plugin: InputPlugin::NAME,
            source: e.to_string(),
        })?;

        TextPlugin::register(ctx).map_err(|e| ScriptEngineError::PluginRegister {
            plugin: TextPlugin::NAME,
            source: e.to_string(),
        })?;

        TexturePlugin::register(ctx).map_err(|e| ScriptEngineError::PluginRegister {
            plugin: TexturePlugin::NAME,
            source: e.to_string(),
        })?;

        Ok(())
    }

    fn register_stdlib(ctx: &Ctx<'_>) -> Result<(), ScriptEngineError> {
        let base_code = "export class GameObject { onInit(){} onUpdate(){} onDraw(){} }";

        let oxid_mod = Module::declare(ctx.clone(), "oxid/core", base_code)
            .map_err(|e| ScriptEngineError::StdlibRegister(e.to_string()))?;

        oxid_mod
            .eval()
            .map_err(|e| ScriptEngineError::StdlibRegister(e.to_string()))?;

        Ok(())
    }

    fn bootstrap_entry_module<'a>(
        ctx: &Ctx<'a>,
        script_code: &str,
        globals: &Object<'a>,
    ) -> Result<(), ScriptEngineError> {
        let module = Module::declare(ctx.clone(), "main.js", script_code)
            .map_err(|e| ScriptEngineError::EntryModuleDeclare(e.to_string()))?;

        module
            .clone()
            .eval()
            .map_err(|e| ScriptEngineError::EntryModuleEval(e.to_string()))?;

        let namespace = module
            .namespace()
            .map_err(|e| ScriptEngineError::MainNamespace(e.to_string()))?;

        globals
            .set(MAIN_NAMESPACE, namespace)
            .map_err(|e| ScriptEngineError::MainNamespace(e.to_string()))?;

        ctx.eval::<(), _>(format!(
            "globalThis.{APP_INSTANCE} = {MAIN_NAMESPACE}.main();"
        ))
        .map_err(|e| ScriptEngineError::AppInstance(e.to_string()))?;

        Ok(())
    }

    fn compile_hooks<'a>(
        ctx: &Ctx<'a>,
        globals: &Object<'a>,
    ) -> Result<(), ScriptEngineError> {
        let hook_on_init: Function = ctx.eval(
            format!("() => {{ if ({APP_INSTANCE}.onInit) {APP_INSTANCE}.onInit(); }}")
        ).map_err(|e| ScriptEngineError::HookCompile {
            hook: "onInit",
            source: e.to_string(),
        })?;

        let hook_on_update: Function = ctx.eval(
            format!("(dt) => {{ if ({APP_INSTANCE}.onUpdate) {APP_INSTANCE}.onUpdate(dt); }}")
        ).map_err(|e| ScriptEngineError::HookCompile {
            hook: "onUpdate",
            source: e.to_string(),
        })?;

        let hook_on_draw: Function = ctx.eval(
            format!("() => {{ if ({APP_INSTANCE}.onDraw) {APP_INSTANCE}.onDraw(); }}")
        ).map_err(|e| ScriptEngineError::HookCompile {
            hook: "onDraw",
            source: e.to_string(),
        })?;

        globals.set(HOOK_ON_INIT, hook_on_init).map_err(|e| ScriptEngineError::HookCompile {
            hook: "onInit",
            source: e.to_string(),
        })?;

        globals.set(HOOK_ON_UPDATE, hook_on_update).map_err(|e| ScriptEngineError::HookCompile {
            hook: "onUpdate",
            source: e.to_string(),
        })?;

        globals.set(HOOK_ON_DRAW, hook_on_draw).map_err(|e| ScriptEngineError::HookCompile {
            hook: "onDraw",
            source: e.to_string(),
        })?;

        Ok(())
    }

    fn call_void_hook(&self, hook_name: &'static str) -> Result<(), ScriptEngineError> {
        self.ctx.with(|ctx| {
            let func: Function = ctx
                .globals()
                .get(hook_name)
                .map_err(|e| ScriptEngineError::HookExecution {
                    hook: hook_name,
                    source: format!("hook não encontrado: {e}"),
                })?;

            func.call::<_, ()>(())
                .map_err(|e| Self::hook_error(&ctx, hook_name, e))
        })
    }

    fn call_f32_hook(&self, hook_name: &'static str, value: f32) -> Result<(), ScriptEngineError> {
        self.ctx.with(|ctx| {
            let func: Function = ctx
                .globals()
                .get(hook_name)
                .map_err(|e| ScriptEngineError::HookExecution {
                    hook: hook_name,
                    source: format!("hook não encontrado: {e}"),
                })?;

            func.call::<_, ()>((value,))
                .map_err(|e| Self::hook_error(&ctx, hook_name, e))
        })
    }

    pub fn new(script_code: &str) -> Result<Self, ScriptEngineError> {
        let rt = Runtime::new()
            .map_err(|e| ScriptEngineError::RuntimeInit(e.to_string()))?;

        let ctx = Context::full(&rt)
            .map_err(|e| ScriptEngineError::ContextInit(e.to_string()))?;

        ctx.with(|ctx| -> Result<(), ScriptEngineError> {
            Self::register_plugins(&ctx)?;
            Self::register_stdlib(&ctx)?;

            let globals = ctx.globals();

            Self::bootstrap_entry_module(&ctx, script_code, &globals)?;
            Self::compile_hooks(&ctx, &globals)?;

            Ok(())
        })?;

        Ok(Self { _rt: rt, ctx })
    }

    pub fn on_init(&self) {
        if let Err(err) = self.call_void_hook(HOOK_ON_INIT) {
            eprintln!("[scripting/onInit] {err}");
        }
    }

    pub fn on_update(&self, dt: f32) {
        if let Err(err) = self.call_f32_hook(HOOK_ON_UPDATE, dt) {
            eprintln!("[scripting/onUpdate] {err}");
        }
    }

    pub fn on_draw(&self, queue: &mut RenderQueue) {
        set_active_queue(queue);

        if let Err(err) = self.call_void_hook(HOOK_ON_DRAW) {
            eprintln!("[scripting/onDraw] {err}");
        }

        clear_active_queue();
    }

}
