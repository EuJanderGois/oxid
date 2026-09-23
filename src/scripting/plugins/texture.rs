use std::string::String as StdString;

use rquickjs::{
    Class, Ctx, Exception, Function, JsLifetime, Result,
    class::{OwnedBorrow, Trace},
    function::Opt,
    module::{Declarations, Exports, ModuleDef},
};

use crate::{
    i18n,
    renderer::{
        color::Color as RendererColor, context::with_active_queue,
        texture::load_texture as load_renderer_texture,
    },
    scripting::plugins::{
        FunctionMeta, FunctionParam, NativePlugin, ScriptType, TypeMeta, TypePropertyMeta,
        math::Vector2D,
    },
};

#[rquickjs::class(rename = "Texture2D")]
#[derive(Clone, Trace, JsLifetime)]
pub struct ScriptTexture2D {
    key: StdString,
    #[qjs(get)]
    path: StdString,
    #[qjs(get)]
    width: f32,
    #[qjs(get)]
    height: f32,
}

impl ScriptTexture2D {
    fn new(key: StdString, path: StdString, width: f32, height: f32) -> Self {
        Self {
            key,
            path,
            width,
            height,
        }
    }
}

fn white() -> RendererColor {
    RendererColor::new(1.0, 1.0, 1.0, 1.0)
}

fn validate_rotation(ctx: &Ctx<'_>, rotation: f32) -> Result<f32> {
    if !rotation.is_finite() {
        let message = i18n::text("scripting.api.invalid_rotation");
        return Err(Exception::throw_range(ctx, &message));
    }

    Ok(rotation)
}

fn validate_size(ctx: &Ctx<'_>, size: &Vector2D) -> Result<()> {
    if !size.x.is_finite() || size.x <= 0.0 {
        let message = i18n::text("scripting.api.invalid_size_x");
        return Err(Exception::throw_range(ctx, &message));
    }

    if !size.y.is_finite() || size.y <= 0.0 {
        let message = i18n::text("scripting.api.invalid_size_y");
        return Err(Exception::throw_range(ctx, &message));
    }

    Ok(())
}

fn load_texture<'js>(ctx: Ctx<'js>, path: StdString) -> Result<Class<'js, ScriptTexture2D>> {
    let texture =
        load_renderer_texture(&path).map_err(|err| Exception::throw_message(&ctx, &err))?;

    Class::instance(
        ctx,
        ScriptTexture2D::new(texture.key, path, texture.width, texture.height),
    )
}

fn draw_texture<'js>(
    texture: OwnedBorrow<'js, ScriptTexture2D>,
    position: OwnedBorrow<'js, Vector2D>,
) {
    let _ = with_active_queue(|queue| {
        queue.draw_texture(
            texture.key.clone(),
            position.x,
            position.y,
            None,
            None,
            0.0,
            white(),
        );
    });
}

fn draw_texture_scaled<'js>(
    ctx: Ctx<'js>,
    texture: OwnedBorrow<'js, ScriptTexture2D>,
    position: OwnedBorrow<'js, Vector2D>,
    size: OwnedBorrow<'js, Vector2D>,
    rotation: Opt<f32>,
) -> Result<()> {
    validate_size(&ctx, &size)?;
    let rotation = validate_rotation(&ctx, rotation.into_inner().unwrap_or(0.0))?;

    let _ = with_active_queue(|queue| {
        queue.draw_texture(
            texture.key.clone(),
            position.x,
            position.y,
            Some(size.x),
            Some(size.y),
            rotation,
            white(),
        );
    });

    Ok(())
}

pub struct TexturePlugin;

impl ModuleDef for TexturePlugin {
    fn declare<'js>(declare: &Declarations<'js>) -> Result<()> {
        declare.declare("loadTexture")?;
        declare.declare("drawTexture")?;
        declare.declare("drawTextureScaled")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        exports.export("loadTexture", Function::new(ctx.clone(), load_texture)?)?;
        exports.export("drawTexture", Function::new(ctx.clone(), draw_texture)?)?;
        exports.export(
            "drawTextureScaled",
            Function::new(ctx.clone(), draw_texture_scaled)?,
        )?;
        Ok(())
    }
}

impl NativePlugin for TexturePlugin {
    const NAME: &'static str = "oxid/texture";

    fn docs() -> &'static str {
        "Carregamento e desenho de texturas 2D."
    }

    fn types() -> &'static [TypeMeta] {
        static TYPES: [TypeMeta; 1] = [TypeMeta {
            module: "oxid/texture",
            name: "Texture2D",
            docs: "Texture loaded by the runtime with dimensions and source path.",
            constructors: &[],
            properties: &[
                TypePropertyMeta {
                    name: "path",
                    ty: ScriptType::String,
                    docs: "Path used to load the texture.",
                    readonly: true,
                },
                TypePropertyMeta {
                    name: "width",
                    ty: ScriptType::Number,
                    docs: "Texture width in pixels.",
                    readonly: true,
                },
                TypePropertyMeta {
                    name: "height",
                    ty: ScriptType::Number,
                    docs: "Texture height in pixels.",
                    readonly: true,
                },
            ],
        }];
        &TYPES
    }

    fn functions() -> &'static [FunctionMeta] {
        &[
            FunctionMeta {
                module: "oxid/texture",
                name: "loadTexture",
                docs: "Loads a texture from disk and returns a reusable Texture2D object.",
                returns: ScriptType::Custom("oxid/texture", "Texture2D"),
                params: &[FunctionParam {
                    name: "path",
                    ty: ScriptType::String,
                    docs: "Texture file path. Relative paths use the current working directory.",
                    optional: false,
                }],
            },
            FunctionMeta {
                module: "oxid/texture",
                name: "drawTexture",
                docs: "Draws a texture using its original size.",
                returns: ScriptType::Void,
                params: &[
                    FunctionParam {
                        name: "texture",
                        ty: ScriptType::Custom("oxid/texture", "Texture2D"),
                        docs: "Texture returned by loadTexture.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "position",
                        ty: ScriptType::Custom("oxid/math", "Vector2D"),
                        docs: "Top-left position in screen coordinates.",
                        optional: false,
                    },
                ],
            },
            FunctionMeta {
                module: "oxid/texture",
                name: "drawTextureScaled",
                docs: "Draws a resized texture with optional rotation in radians.",
                returns: ScriptType::Void,
                params: &[
                    FunctionParam {
                        name: "texture",
                        ty: ScriptType::Custom("oxid/texture", "Texture2D"),
                        docs: "Texture returned by loadTexture.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "position",
                        ty: ScriptType::Custom("oxid/math", "Vector2D"),
                        docs: "Top-left position in screen coordinates.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "size",
                        ty: ScriptType::Custom("oxid/math", "Vector2D"),
                        docs: "Destination width and height for the texture.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "rotation",
                        ty: ScriptType::Number,
                        docs: "Rotation in radians. Defaults to 0 when omitted.",
                        optional: true,
                    },
                ],
            },
        ]
    }
}
