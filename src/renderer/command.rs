// src/renderer/command.rs
use crate::renderer::color::Color;

#[derive(Clone, Debug)]
pub enum RenderCommand {
    Clear {
        color: Color,
    },
    DrawCircle {
        x: f32,
        y: f32,
        radius: f32,
        color: Color,
    },
    DrawRectangle {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
    },
    DrawArc {
        x: f32,
        y: f32,
        sides: u8,
        radius: f32,
        rotation: f32,
        thickness: f32,
        arc: f32,
        color: Color,
    },
    DrawText {
        text: String,
        x: f32,
        y: f32,
        font_size: f32,
        color: Color,
    },
    DrawMultilineText {
        text: String,
        x: f32,
        y: f32,
        font_size: f32,
        line_distance: Option<f32>,
        color: Color,
    },
    DrawTexture {
        texture_key: String,
        x: f32,
        y: f32,
        width: Option<f32>,
        height: Option<f32>,
        rotation: f32,
        color: Color,
    },
}
