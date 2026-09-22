use std::string::String as StdString;

use macroquad::prelude::measure_text as mq_measure_text;
use rquickjs::{
    Class, Ctx, Exception, Function, JsLifetime, Result,
    class::{OwnedBorrow, Trace},
    function::Opt,
    module::{Declarations, Exports, ModuleDef},
};

use crate::{
    i18n,
    renderer::context::with_active_queue,
    scripting::{
        color::{Color, to_renderer_color},
        math::Vector2D,
        plugin::{
            FunctionMeta, FunctionParam, NativePlugin, ScriptType, TypeConstructorMeta, TypeMeta,
            TypePropertyMeta,
        },
    },
};

///
/// Layout metrics for a single line of text.
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
        let message = i18n::text("scripting.api.invalid_font_size");
        return Err(Exception::throw_range(ctx, &message));
    }

    Ok(font_size)
}

fn validate_line_distance(ctx: &Ctx<'_>, line_distance: Option<f32>) -> Result<Option<f32>> {
    match line_distance {
        Some(distance) if !distance.is_finite() || distance <= 0.0 => {
            let message = i18n::text("scripting.api.invalid_line_distance");
            Err(Exception::throw_range(ctx, &message))
        }
        _ => Ok(line_distance),
    }
}

///
/// adiciona o comando draw_text a queue.
///
/// The y coordinate represents the text baseline, following Macroquad conventions.
///
fn draw_text<'js>(
    ctx: Ctx<'js>,
    text: StdString,
    position: OwnedBorrow<'js, Vector2D>,
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
/// adds a multiline text block to the render queue.
///
/// a coordenada y representa a baseline da primeira linha.
///
fn draw_multiline_text<'js>(
    ctx: Ctx<'js>,
    text: StdString,
    position: OwnedBorrow<'js, Vector2D>,
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
/// Measures a single line of text using the default font.
///
fn measure_text<'js>(
    ctx: Ctx<'js>,
    text: StdString,
    font_size: f32,
) -> Result<Class<'js, TextMetrics>> {
    let font_size = validate_font_size(&ctx, font_size)?;
    let metrics = mq_measure_text(&text, None, font_size as u16, 1.0);

    Class::instance(
        ctx,
        TextMetrics::from_macroquad(metrics.width, metrics.height, metrics.offset_y),
    )
}

///
/// 2D text.
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
        exports.export("drawText", Function::new(ctx.clone(), draw_text)?)?;
        exports.export(
            "drawMultilineText",
            Function::new(ctx.clone(), draw_multiline_text)?,
        )?;
        exports.export("measureText", Function::new(ctx.clone(), measure_text)?)?;
        Ok(())
    }
}

impl NativePlugin for TextPlugin {
    const NAME: &'static str = "oxid/text";

    fn docs() -> &'static str {
        "2D text rendering and measurement."
    }

    fn types() -> &'static [TypeMeta] {
        static TYPES: [TypeMeta; 1] = [TypeMeta {
            module: "oxid/text",
            name: "TextMetrics",
            docs: "Metrics calculated for a single line of text.",
            constructors: &[TypeConstructorMeta {
                params: &[
                    FunctionParam {
                        name: "width",
                        ty: ScriptType::Number,
                        docs: "Text width.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "height",
                        ty: ScriptType::Number,
                        docs: "Text height.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "offsetY",
                        ty: ScriptType::Number,
                        docs: "Vertical metric offset.",
                        optional: false,
                    },
                ],
            }],
            properties: &[
                TypePropertyMeta {
                    name: "width",
                    ty: ScriptType::Number,
                    docs: "Text width.",
                    readonly: true,
                },
                TypePropertyMeta {
                    name: "height",
                    ty: ScriptType::Number,
                    docs: "Text height.",
                    readonly: true,
                },
                TypePropertyMeta {
                    name: "offset_y",
                    ty: ScriptType::Number,
                    docs: "Vertical metric offset.",
                    readonly: true,
                },
            ],
        }];
        &TYPES
    }

    fn functions() -> &'static [FunctionMeta] {
        &[
            FunctionMeta {
                module: "oxid/text",
                name: "drawText",
                docs: "Draws 2D text on the screen. The y coordinate represents the text baseline.",
                returns: ScriptType::Void,
                params: &[
                    FunctionParam {
                        name: "text",
                        ty: ScriptType::String,
                        docs: "Text content to draw.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "position",
                        ty: ScriptType::Custom("oxid/math", "Vector2D"),
                        docs: "Text position in screen coordinates.",
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
                        ty: ScriptType::Custom("oxid/color", "Color"),
                        docs: "Color used for the text.",
                        optional: false,
                    },
                ],
            },
            FunctionMeta {
                module: "oxid/text",
                name: "drawMultilineText",
                docs: "Draws multiline text using '\\n' as the separator. The y coordinate represents the first line baseline.",
                returns: ScriptType::Void,
                params: &[
                    FunctionParam {
                        name: "text",
                        ty: ScriptType::String,
                        docs: "Text content to draw.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "position",
                        ty: ScriptType::Custom("oxid/math", "Vector2D"),
                        docs: "Initial text block position in screen coordinates.",
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
                        ty: ScriptType::Custom("oxid/color", "Color"),
                        docs: "Color used for the text.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "lineDistance",
                        ty: ScriptType::Number,
                        docs: "Line spacing multiplier. Use 1.0 for default spacing.",
                        optional: true,
                    },
                ],
            },
            FunctionMeta {
                module: "oxid/text",
                name: "measureText",
                docs: "Measures a single line of text using the default font and returns its width, height, and offset_y.",
                returns: ScriptType::Custom("oxid/text", "TextMetrics"),
                params: &[
                    FunctionParam {
                        name: "text",
                        ty: ScriptType::String,
                        docs: "Text content to measure.",
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
