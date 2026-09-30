//! Central registry for scripting plugins.

use rquickjs::Ctx;

use crate::scripting::plugins::{
    GlobalMeta, GlobalPlugin, GlobalPluginRegistration, PluginRegistration, ScriptPlugin,
    color::ColorPlugin, core::CorePlugin, global::ConsolePlugin, input::InputPlugin,
    math::MathPlugin, shapes::ShapesPlugin, text::TextPlugin, texture::TexturePlugin,
    window::WindowPlugin,
};

pub fn plugins() -> &'static [PluginRegistration] {
    static PLUGINS: [PluginRegistration; 8] = [
        PluginRegistration {
            name: CorePlugin::NAME,
            metadata: CorePlugin::metadata,
            register: CorePlugin::register,
        },
        PluginRegistration {
            name: MathPlugin::NAME,
            metadata: MathPlugin::metadata,
            register: MathPlugin::register,
        },
        PluginRegistration {
            name: ColorPlugin::NAME,
            metadata: ColorPlugin::metadata,
            register: ColorPlugin::register,
        },
        PluginRegistration {
            name: ShapesPlugin::NAME,
            metadata: ShapesPlugin::metadata,
            register: ShapesPlugin::register,
        },
        PluginRegistration {
            name: InputPlugin::NAME,
            metadata: InputPlugin::metadata,
            register: InputPlugin::register,
        },
        PluginRegistration {
            name: TextPlugin::NAME,
            metadata: TextPlugin::metadata,
            register: TextPlugin::register,
        },
        PluginRegistration {
            name: TexturePlugin::NAME,
            metadata: TexturePlugin::metadata,
            register: TexturePlugin::register,
        },
        PluginRegistration {
            name: WindowPlugin::NAME,
            metadata: WindowPlugin::metadata,
            register: WindowPlugin::register,
        },
    ];

    &PLUGINS
}

pub fn api_metadata() -> Vec<crate::scripting::plugins::ModuleMeta> {
    plugins().iter().map(|plugin| (plugin.metadata)()).collect()
}

pub fn global_plugins() -> &'static [GlobalPluginRegistration] {
    static PLUGINS: [GlobalPluginRegistration; 2] = [
        GlobalPluginRegistration {
            name: ConsolePlugin::NAME,
            metadata: ConsolePlugin::metadata,
            register: ConsolePlugin::register,
        },
        GlobalPluginRegistration {
            name: <ColorPlugin as GlobalPlugin>::NAME,
            metadata: <ColorPlugin as GlobalPlugin>::metadata,
            register: <ColorPlugin as GlobalPlugin>::register,
        },
    ];

    &PLUGINS
}

pub fn global_metadata() -> Vec<GlobalMeta> {
    global_plugins()
        .iter()
        .map(|plugin| (plugin.metadata)())
        .collect()
}

pub fn register_globals(ctx: &Ctx<'_>) -> Result<(), (String, String)> {
    for plugin in global_plugins() {
        (plugin.register)(ctx).map_err(|error| (plugin.name.to_string(), error.to_string()))?;
    }

    Ok(())
}

pub fn register_plugins(ctx: &Ctx<'_>) -> Result<(), (String, String)> {
    for plugin in plugins() {
        (plugin.register)(ctx).map_err(|error| (plugin.name.to_string(), error.to_string()))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_registry_metadata_matches_plugin_names() {
        let registered = global_plugins();
        let names: Vec<_> = registered.iter().map(|plugin| plugin.name).collect();

        for plugin in registered {
            let metadata = (plugin.metadata)();

            assert_eq!(metadata.name, plugin.name);
            assert_eq!(
                names.iter().filter(|name| **name == plugin.name).count(),
                1,
                "global plugin '{}' is registered more than once",
                plugin.name
            );

            for function in metadata.functions {
                assert_eq!(function.module, "global");
            }
        }
    }

    #[test]
    fn global_plugins_are_available_without_imports() {
        fn add_one(value: i32) -> rquickjs::Result<i32> {
            Ok(value + 1)
        }

        let runtime = rquickjs::Runtime::new().unwrap();
        let context = rquickjs::Context::full(&runtime).unwrap();

        context.with(|ctx| {
            register_globals(&ctx).unwrap();
            crate::scripting::plugins::register_global_function(&ctx, "__oxid_test_add_one", add_one)
                .unwrap();
            crate::scripting::plugins::register_global_constant(&ctx, "__oxid_test_value", 41_i32)
                .unwrap();

            let result = ctx
                .eval::<i32, _>("__oxid_test_add_one(__oxid_test_value)")
                .unwrap();
            let console_log_type = ctx
                .eval::<String, _>("typeof console.log")
                .unwrap();
            let console_methods = ctx
                .eval::<bool, _>(
                    "typeof console.info === 'function' && typeof console.warn === 'function' && typeof console.error === 'function' && typeof console.debug === 'function' && typeof console.assert === 'function' && typeof console.trace === 'function' && typeof console.clear === 'function'",
                )
                .unwrap();
            let red_is_color = ctx
                .eval::<bool, _>("RED && RED.r !== undefined && RED.g !== undefined && RED.b !== undefined && RED.a !== undefined")
                .unwrap();

            assert_eq!(result, 42);
            assert_eq!(console_log_type, "function");
            assert!(console_methods);
            assert!(red_is_color);
        });
    }

    #[test]
    fn registry_metadata_matches_plugin_names() {
        let registered = plugins();
        let names: Vec<_> = registered.iter().map(|plugin| plugin.name).collect();

        for plugin in registered {
            let metadata = (plugin.metadata)();

            assert_eq!(metadata.name, plugin.name);
            assert!(
                names.iter().filter(|name| **name == plugin.name).count() == 1,
                "plugin '{}' is registered more than once",
                plugin.name
            );

            for ty in metadata.types {
                assert_eq!(ty.module, plugin.name);
            }

            for function in metadata.functions {
                assert_eq!(function.module, plugin.name);
            }
        }
    }
}
