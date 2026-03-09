// src/renderer/mod.rs

pub mod shapes;
pub mod color;
pub mod command;
pub mod queue;
pub mod mq_renderer;

use crate::renderer::{color::Color, queue::RenderQueue};

pub trait Renderer {
    fn begin_frame(&mut self);
    fn delta_time(&self) -> f32;
    fn render(&mut self, queue: &mut RenderQueue);
    fn clear_immediate(&mut self, color: Color);
}