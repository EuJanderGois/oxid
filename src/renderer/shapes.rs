// src/renderer/shapes.rs

use crate::renderer::queue::RenderQueue;
use crate::renderer::Color;

pub trait ShapesAPI {
    fn draw_circle(&mut self, x: f32, y: f32, radius: f32, color: Color);
}

impl ShapesAPI for RenderQueue {
    fn draw_circle(&mut self, x: f32, y: f32, radius: f32, color: Color) {
        RenderQueue::draw_circle(self, x, y, radius, color);
    }
}