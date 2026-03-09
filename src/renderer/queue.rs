// src/renderer/queue.rs
use crate::renderer::{
    color::Color,
    command::RenderCommand,
};

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

    pub fn draw_circle(&mut self, x: f32, y: f32, radius: f32, color: Color) {
        self.push(RenderCommand::DrawCircle {
            x,
            y,
            radius,
            color,
        });
    }
}