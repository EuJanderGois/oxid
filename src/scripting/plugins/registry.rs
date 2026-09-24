//! Central registry for scripting plugins.

use rquickjs::Ctx;

use crate::scripting::plugins::{
    PluginRegistration, ScriptPlugin, color::ColorPlugin, core::CorePlugin, input::InputPlugin,
    math::MathPlugin, shapes::ShapesPlugin, text::TextPlugin, texture::TexturePlugin,
};

pub fn plugins() -> &'static [PluginRegistration] {
    static PLUGINS: [PluginRegistration; 7] = [
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
    ];

    &PLUGINS
}

pub fn api_metadata() -> Vec<crate::scripting::plugins::ModuleMeta> {
    plugins().iter().map(|plugin| (plugin.metadata)()).collect()
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
