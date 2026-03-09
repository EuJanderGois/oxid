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
}