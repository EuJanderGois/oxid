// src/renderer/queue.rs
use crate::{renderer::{color::Color, command::RenderCommand}, scripting::plugins::math::Vector2D};

#[derive(Default, Debug)]
pub struct RenderQueue {
    commands: Vec<RenderCommand>,
}

impl RenderQueue {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }

    pub fn push(&mut self, command: RenderCommand) {
        self.commands.push(command);
    }

    pub fn commands(&self) -> &[RenderCommand] {
        &self.commands
    }

    pub fn drain(&mut self) -> impl Iterator<Item = RenderCommand> + '_ {
        self.commands.drain(..)
    }

    pub fn reserve(&mut self, additional: usize) {
        self.commands.reserve(additional);
    }

    pub fn clear_background(&mut self, color: Color) {
        self.push(RenderCommand::Clear { color });
    }

    pub fn draw_arc(
        &mut self,
        x: f32,
        y: f32,
        sides: u8,
        radius: f32,
        rotation: f32,
        thickness: f32,
        arc: f32,
        color: Color,
    ) {
        self.push(RenderCommand::DrawArc {
            x,
            y,
            sides,
            radius,
            rotation,
            thickness,
            arc,
            color,
        });
    }

    pub fn draw_circle(&mut self, x: f32, y: f32, radius: f32, color: Color) {
        self.push(RenderCommand::DrawCircle {
            x,
            y,
            radius,
            color,
        });
    }

    pub fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        self.push(RenderCommand::DrawRectangle {
            x,
            y,
            width,
            height,
            color,
        });
    }

    pub fn draw_text(&mut self, text: String, x: f32, y: f32, font_size: f32, color: Color) {
        self.push(RenderCommand::DrawText {
            text,
            x,
            y,
            font_size,
            color,
        });
    }

    pub fn draw_multiline_text(
        &mut self,
        text: String,
        x: f32,
        y: f32,
        font_size: f32,
        line_distance: Option<f32>,
        color: Color,
    ) {
        self.push(RenderCommand::DrawMultilineText {
            text,
            x,
            y,
            font_size,
            line_distance,
            color,
        });
    }

    pub fn draw_texture(
        &mut self,
        texture_key: String,
        x: f32,
        y: f32,
        width: Option<f32>,
        height: Option<f32>,
        rotation: f32,
        color: Color,
    ) {
        self.push(RenderCommand::DrawTexture {
            texture_key,
            x,
            y,
            width,
            height,
            rotation,
            color,
        });
    }

    pub fn draw_line(
        &mut self,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        thickness: f32,
        color: Color,
    ) {
        self.push(RenderCommand::DrawLine {
            start_x,
            start_y,
            end_x,
            end_y,
            thickness,
            color,
        });
    }

    pub fn draw_triangle_lines(
        &mut self,
        v1: Vector2D,
        v2: Vector2D,
        v3: Vector2D,
        thickness: f32,
        color: Color,
    ) {
        self.push(RenderCommand::DrawTriangleLines {
            v1,
            v2,
            v3,
            thickness,
            color,
        });
    }

    pub fn draw_polygon_lines(
        &mut self,
        x: f32,
        y: f32,
        sides: u8,
        radius: f32,
        rotation: f32,
        thickness: f32,
        color: Color,
    ) {
        self.push(RenderCommand::DrawPolygonLines {
            x,
            y,
            sides,
            radius,
            rotation,
            thickness,
            color,
        });
    }

}
