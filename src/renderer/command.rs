// src/renderer/command.rs
use crate::renderer::color::Color;

/// A plain 2D point used by render commands.
///
/// Render commands are a pure description of what to draw and must not
/// depend on how the scripting layer happens to represent vectors today
/// (see `scripting::plugins::math::Vector2D`, which is an `rquickjs`-bound
/// class). Scripting plugins convert their own vector type into this one at
/// the boundary, the same way they already convert their `Color` type via
/// `scripting::plugins::color::to_renderer_color`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

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
    DrawLine {
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        thickness: f32,
        color: Color,
    },
    DrawTriangleLines {
        v1: Vec2,
        v2: Vec2,
        v3: Vec2,
        thickness: f32,
        color: Color,
    },
    DrawPolygonLines {
        x: f32,
        y: f32,
        sides: u8,
        radius: f32,
        rotation: f32,
        thickness: f32,
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
