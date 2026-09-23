//! Central registry for native scripting modules.

use rquickjs::Ctx;

use crate::scripting::plugins::{
    ModuleMeta, NativeModule, NativePlugin, color::ColorPlugin, input::InputPlugin,
    math::MathPlugin, shapes::ShapesPlugin, text::TextPlugin, texture::TexturePlugin,
};

pub fn native_modules() -> &'static [NativeModule] {
    static MODULES: [NativeModule; 6] = [
        NativeModule {
            name: MathPlugin::NAME,
            metadata: MathPlugin::metadata,
            register: MathPlugin::register,
        },
        NativeModule {
            name: ColorPlugin::NAME,
            metadata: ColorPlugin::metadata,
            register: ColorPlugin::register,
        },
        NativeModule {
            name: ShapesPlugin::NAME,
            metadata: ShapesPlugin::metadata,
            register: ShapesPlugin::register,
        },
        NativeModule {
            name: InputPlugin::NAME,
            metadata: InputPlugin::metadata,
            register: InputPlugin::register,
        },
        NativeModule {
            name: TextPlugin::NAME,
            metadata: TextPlugin::metadata,
            register: TextPlugin::register,
        },
        NativeModule {
            name: TexturePlugin::NAME,
            metadata: TexturePlugin::metadata,
            register: TexturePlugin::register,
        },
    ];

    &MODULES
}

pub fn api_metadata() -> Vec<ModuleMeta> {
    let mut modules = vec![super::core::metadata()];
    modules.extend(native_modules().iter().map(|module| (module.metadata)()));
    modules
}

pub fn register_native_modules(ctx: &Ctx<'_>) -> Result<(), (String, String)> {
    for module in native_modules() {
        (module.register)(ctx).map_err(|error| (module.name.to_string(), error.to_string()))?;
    }

    Ok(())
}
