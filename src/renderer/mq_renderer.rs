// src/renderer/mq_renderer.rs
use macroquad::prelude::{
    clear_background,
    draw_circle,
    get_frame_time,
    Color as MqColor,
};

use crate::renderer::{
    color::Color,
    command::RenderCommand,
    queue::RenderQueue,
    Renderer,
};

pub struct MqRenderer;

impl MqRenderer {
    fn to_mq_color(color: Color) -> MqColor {
        MqColor::new(color.r, color.g, color.b, color.a)
    }
}

impl Renderer for MqRenderer {
    fn begin_frame(&mut self) {
        // espaço para stats, reset de estado, debug markers, etc.
    }

    fn delta_time(&self) -> f32 {
        get_frame_time()
    }

    fn clear_immediate(&mut self, color: Color) {
        clear_background(Self::to_mq_color(color));
    }

    fn render(&mut self, queue: &mut RenderQueue) {
        for command in queue.drain() {
            match command {
                RenderCommand::Clear { color } => {
                    clear_background(Self::to_mq_color(color));
                }
                RenderCommand::DrawCircle {
                    x,
                    y,
                    radius,
                    color,
                } => {
                    draw_circle(x, y, radius, Self::to_mq_color(color));
                }
            }
        }
    }
}