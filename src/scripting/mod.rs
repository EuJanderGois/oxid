pub mod math;
pub mod graphics;
pub mod color;
pub mod plugin;

use rquickjs::{Context, Ctx, Function, Module, Object, Runtime};
use plugin::NativePlugin;
use math::MathPlugin;
use graphics::GraphicsPlugin;

use crate::{renderer::{
    context::{clear_active_queue, set_active_queue},
    queue::RenderQueue,
}, scripting::color::ColorPlugin};

const HOOK_ON_INIT: &str = "__hook_on_init";
const HOOK_ON_UPDATE: &str = "__hook_on_update";
const HOOK_ON_DRAW: &str = "__hook_on_draw";
const APP_INSTANCE: &str = "__app_instance";
const MAIN_NAMESPACE: &str = "__main";

macro_rules! register_plugins {
    ($ctx:expr, $($plugin:ty),+ $(,)?) => {
        $( <$plugin as NativePlugin>::register(&$ctx).unwrap(); )+
    };
}

pub struct ScriptEngine {
    _rt: Runtime,
    ctx: Context,
}

impl ScriptEngine {

    fn register_stdlib(ctx: &Ctx) {
        // biblioteca padrão
        // futuramente precisa vir de arquivo js
        let base_code = "export class GameObject { onInit(){} onUpdate(){} onDraw(){} }";
        let oxid_mod = Module::declare(ctx.clone(), "oxid/core", base_code).unwrap();
        oxid_mod.eval().unwrap();
    }

    fn bootstrap_entry_module<'a>(ctx: &Ctx<'a>, script_code: &str, globals: &Object<'a>) {
        // eval da entrada
            let module = Module::declare(ctx.clone(), "main.js", script_code).unwrap();
            let _promise = module.clone().eval().unwrap();
            let namespace = module.namespace().unwrap();
            
            // namespace temporário
            globals.set(MAIN_NAMESPACE, namespace).unwrap();

            // instantiate 
            ctx.eval::<(), _>(format!(
                "globalThis.{APP_INSTANCE} = {MAIN_NAMESPACE}.main();"
            )).unwrap();
    }

    fn compile_hooks<'a>(ctx: &Ctx<'a>, globals: &Object<'a>) {
        // funções pré compiladas que preservam this
        let hook_on_init: Function = ctx.eval(
            format!(
                "() => {{ if ({APP_INSTANCE}.onInit) {APP_INSTANCE}.onInit(); }}"
            )).unwrap();
        let hook_on_update: Function = ctx.eval(
            format!(
                "(dt) => {{ if ({APP_INSTANCE}.onUpdate) {APP_INSTANCE}.onUpdate(dt); }}"
            )).unwrap();
        let hook_on_draw: Function = ctx.eval(
            format!(
                "() => {{ if ({APP_INSTANCE}.onDraw) {APP_INSTANCE}.onDraw(); }}"
            )).unwrap();

        // shims
        globals.set(HOOK_ON_INIT, hook_on_init).unwrap();
        globals.set(HOOK_ON_UPDATE, hook_on_update).unwrap();
        globals.set(HOOK_ON_DRAW, hook_on_draw).unwrap();
    }

    fn call_void_hook(&self, hook_name: &str) {
        self.ctx.with(|ctx| {
            if let Ok(func) = ctx.globals().get::<_, Function>(hook_name) {
                let _ = func.call::<_, ()>(());
            }
        });
    }

    fn call_f32_hook(&self, hook_name: &str, value: f32) {
        self.ctx.with(|ctx| {
            if let Ok(func) = ctx.globals().get::<_, Function>(hook_name) {
                let _ = func.call::<_, ()>((value,));
            }
        });
    }

    pub fn new(script_code: &str) -> Self {
        let rt = Runtime::new().unwrap();
        let ctx = Context::full(&rt).unwrap();

        ctx.with(|ctx| {
            // registra os modulos
            register_plugins!(ctx, 
                MathPlugin, 
                GraphicsPlugin,
                ColorPlugin
            );
            
            Self::register_stdlib(&ctx);

            let globals = ctx.globals();

            Self::bootstrap_entry_module(&ctx, script_code, &globals);        
            Self::compile_hooks(&ctx, &globals);
            
        });

        Self { _rt: rt, ctx }
    }

    pub fn on_init(&self) {
        self.call_void_hook(HOOK_ON_INIT);
    }

    pub fn on_update(&self, dt: f32) {
        self.call_f32_hook(HOOK_ON_UPDATE, dt);
    }

    pub fn on_draw(&self, queue: &mut RenderQueue) {
        set_active_queue(queue);
        self.call_void_hook(HOOK_ON_DRAW);
        clear_active_queue();
    }

}