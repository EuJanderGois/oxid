use crate::renderer::{Renderer, color::Color, queue::RenderQueue};

pub struct MqRenderer;

impl MqRenderer {
    fn to_mq_color(color: Color) -> macroquad::prelude::Color {
        macroquad::prelude::Color::new(color.r, color.g, color.b, color.a)
    }
}

impl Renderer for MqRenderer {
    fn begin_frame(&mut self) {}

    fn delta_time(&self) -> f32 {
        macroquad::prelude::get_frame_time()
    }

    fn render(&mut self, queue: &mut RenderQueue) {
        use macroquad::prelude::{clear_background, draw_circle};

        for command in queue.drain() {
            match command {
                crate::renderer::command::RenderCommand::Clear { color } => {
                    clear_background(Self::to_mq_color(color));
                }
                crate::renderer::command::RenderCommand::DrawCircle {
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