// src/renderer/queue.rs
use crate::renderer::{color::Color, command::RenderCommand};

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
}
