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
    scripting::{
        math::Transform2D,
        plugin::{FunctionMeta, FunctionParam, NativePlugin, ScriptType},
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

fn validate_size(ctx: &Ctx<'_>, size: &Transform2D) -> Result<()> {
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
    position: OwnedBorrow<'js, Transform2D>,
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
    position: OwnedBorrow<'js, Transform2D>,
    size: OwnedBorrow<'js, Transform2D>,
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

    fn functions() -> &'static [FunctionMeta] {
        &[
            FunctionMeta {
                module: "oxid/texture",
                name: "loadTexture",
                docs: "Carrega uma textura do disco e retorna um objeto Texture2D reutilizável.",
                returns: ScriptType::Custom("Texture2D"),
                params: &[FunctionParam {
                    name: "path",
                    ty: ScriptType::String,
                    docs: "Caminho do arquivo de textura. Caminhos relativos usam o diretório de trabalho atual.",
                    optional: false,
                }],
            },
            FunctionMeta {
                module: "oxid/texture",
                name: "drawTexture",
                docs: "Desenha uma textura usando seu tamanho original.",
                returns: ScriptType::Void,
                params: &[
                    FunctionParam {
                        name: "texture",
                        ty: ScriptType::Custom("Texture2D"),
                        docs: "Textura retornada por loadTexture.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "position",
                        ty: ScriptType::Custom("Transform2D"),
                        docs: "Posição do canto superior esquerdo em coordenadas de tela.",
                        optional: false,
                    },
                ],
            },
            FunctionMeta {
                module: "oxid/texture",
                name: "drawTextureScaled",
                docs: "Desenha uma textura redimensionada, com rotação opcional em radianos.",
                returns: ScriptType::Void,
                params: &[
                    FunctionParam {
                        name: "texture",
                        ty: ScriptType::Custom("Texture2D"),
                        docs: "Textura retornada por loadTexture.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "position",
                        ty: ScriptType::Custom("Transform2D"),
                        docs: "Posição do canto superior esquerdo em coordenadas de tela.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "size",
                        ty: ScriptType::Custom("Transform2D"),
                        docs: "Largura e altura de destino da textura.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "rotation",
                        ty: ScriptType::Number,
                        docs: "Rotação em radianos. Quando omitido, usa 0.",
                        optional: true,
                    },
                ],
            },
        ]
    }
}
