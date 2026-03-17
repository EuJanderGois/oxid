use std::string::String as StdString;

use macroquad::prelude::measure_text as mq_measure_text;
use rquickjs::{
    Class,
    Ctx,
    Exception,
    Function,
    JsLifetime,
    Result,
    class::{OwnedBorrow, Trace},
    function::Opt,
    module::{Declarations, Exports, ModuleDef},
};

use crate::{
    renderer::context::with_active_queue,
    scripting::{
        color::{Color, to_renderer_color},
        math::Transform2D,
        plugin::{FunctionMeta, FunctionParam, NativePlugin, ScriptType},
    },
};

///
/// métricas de layout para um texto de linha única.
///
#[rquickjs::class]
#[derive(Clone, Trace, JsLifetime)]
pub struct TextMetrics {
    #[qjs(get)]
    pub width: f32,
    #[qjs(get)]
    pub height: f32,
    #[qjs(get)]
    pub offset_y: f32,
}

#[rquickjs::methods]
impl TextMetrics {
    #[qjs(constructor)]
    pub fn new(width: f32, height: f32, offset_y: f32) -> Self {
        Self {
            width,
            height,
            offset_y,
        }
    }
}

impl TextMetrics {
    fn from_macroquad(width: f32, height: f32, offset_y: f32) -> Self {
        Self {
            width,
            height,
            offset_y,
        }
    }
}

fn validate_font_size(ctx: &Ctx<'_>, font_size: f32) -> Result<f32> {
    if !font_size.is_finite() || font_size <= 0.0 || font_size > u16::MAX as f32 {
        return Err(Exception::throw_range(
            ctx,
            "fontSize deve ser um número finito entre 0 e 65535.",
        ));
    }

    Ok(font_size)
}

fn validate_line_distance(ctx: &Ctx<'_>, line_distance: Option<f32>) -> Result<Option<f32>> {
    match line_distance {
        Some(distance) if !distance.is_finite() || distance <= 0.0 => Err(Exception::throw_range(
            ctx,
            "lineDistance deve ser um número finito maior que zero.",
        )),
        _ => Ok(line_distance),
    }
}

///
/// adiciona o comando draw_text a queue.
///
/// a coordenada y representa a baseline do texto, seguindo a convenção do macroquad.
///
fn draw_text<'js>(
    ctx: Ctx<'js>,
    text: StdString,
    position: OwnedBorrow<'js, Transform2D>,
    font_size: f32,
    color: OwnedBorrow<'js, Color>,
) -> Result<()> {
    let font_size = validate_font_size(&ctx, font_size)?;

    let _ = with_active_queue(|queue| {
        queue.draw_text(
            text,
            position.x,
            position.y,
            font_size,
            to_renderer_color(&color),
        );
    });

    Ok(())
}

///
/// adiciona um bloco de texto multilinha a queue.
///
/// a coordenada y representa a baseline da primeira linha.
///
fn draw_multiline_text<'js>(
    ctx: Ctx<'js>,
    text: StdString,
    position: OwnedBorrow<'js, Transform2D>,
    font_size: f32,
    color: OwnedBorrow<'js, Color>,
    line_distance: Opt<f32>,
) -> Result<()> {
    let font_size = validate_font_size(&ctx, font_size)?;
    let line_distance = validate_line_distance(&ctx, line_distance.into_inner())?;

    let _ = with_active_queue(|queue| {
        queue.draw_multiline_text(
            text,
            position.x,
            position.y,
            font_size,
            line_distance,
            to_renderer_color(&color),
        );
    });

    Ok(())
}

///
/// mede um texto de linha única usando a fonte padrão.
///
fn measure_text<'js>(ctx: Ctx<'js>, text: StdString, font_size: f32) -> Result<Class<'js, TextMetrics>> {
    let font_size = validate_font_size(&ctx, font_size)?;
    let metrics = mq_measure_text(&text, None, font_size as u16, 1.0);

    Class::instance(
        ctx,
        TextMetrics::from_macroquad(metrics.width, metrics.height, metrics.offset_y),
    )
}

///
/// texto 2D.
///
pub struct TextPlugin;

impl ModuleDef for TextPlugin {
    fn declare<'js>(declare: &Declarations<'js>) -> Result<()> {
        declare.declare("TextMetrics")?;
        declare.declare("drawText")?;
        declare.declare("drawMultilineText")?;
        declare.declare("measureText")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        exports.export(
            "TextMetrics",
            Class::<TextMetrics>::create_constructor(ctx)?,
        )?;
        exports.export(
            "drawText",
            Function::new(ctx.clone(), draw_text)?,
        )?;
        exports.export(
            "drawMultilineText",
            Function::new(ctx.clone(), draw_multiline_text)?,
        )?;
        exports.export(
            "measureText",
            Function::new(ctx.clone(), measure_text)?,
        )?;
        Ok(())
    }
}

impl NativePlugin for TextPlugin {
    const NAME: &'static str = "oxid/text";

    fn functions() -> &'static [FunctionMeta] {
        &[
            FunctionMeta {
                module: "oxid/text",
                name: "drawText",
                docs: "Desenha texto 2D na tela. A coordenada y representa a baseline do texto.",
                returns: ScriptType::Void,
                params: &[
                    FunctionParam {
                        name: "text",
                        ty: ScriptType::String,
                        docs: "Conteúdo textual a ser desenhado.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "position",
                        ty: ScriptType::Custom("Transform2D"),
                        docs: "Posição do texto em coordenadas de tela.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "fontSize",
                        ty: ScriptType::Number,
                        docs: "Tamanho da fonte em pixels. Deve ser maior que zero.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "color",
                        ty: ScriptType::Custom("Color"),
                        docs: "Cor usada no texto.",
                        optional: false,
                    },
                ],
            },
            FunctionMeta {
                module: "oxid/text",
                name: "drawMultilineText",
                docs: "Desenha texto multilinha usando '\\n' como separador. A coordenada y representa a baseline da primeira linha.",
                returns: ScriptType::Void,
                params: &[
                    FunctionParam {
                        name: "text",
                        ty: ScriptType::String,
                        docs: "Conteúdo textual a ser desenhado.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "position",
                        ty: ScriptType::Custom("Transform2D"),
                        docs: "Posição inicial do bloco de texto em coordenadas de tela.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "fontSize",
                        ty: ScriptType::Number,
                        docs: "Tamanho da fonte em pixels. Deve ser maior que zero.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "color",
                        ty: ScriptType::Custom("Color"),
                        docs: "Cor usada no texto.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "lineDistance",
                        ty: ScriptType::Number,
                        docs: "Multiplicador de distância entre linhas. Use 1.0 para o espaçamento padrão.",
                        optional: true,
                    },
                ],
            },
            FunctionMeta {
                module: "oxid/text",
                name: "measureText",
                docs: "Mede um texto de linha única usando a fonte padrão e retorna largura, altura e offset_y.",
                returns: ScriptType::Custom("TextMetrics"),
                params: &[
                    FunctionParam {
                        name: "text",
                        ty: ScriptType::String,
                        docs: "Conteúdo textual a ser medido.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "fontSize",
                        ty: ScriptType::Number,
                        docs: "Tamanho da fonte em pixels. Deve ser maior que zero.",
                        optional: false,
                    },
                ],
            },
        ]
    }
}
