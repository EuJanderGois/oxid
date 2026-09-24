use macroquad::{math::Vec2, shapes::{draw_line, draw_poly_lines, draw_triangle_lines}};

use crate::{
    i18n,
    renderer::{
        Renderer, color::Color, command::RenderCommand, queue::RenderQueue,
        texture::draw_cached_texture,
    },
};

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
        use macroquad::prelude::{
            clear_background, draw_arc, draw_circle, draw_multiline_text, draw_rectangle, draw_text,
        };

        for command in queue.drain() {
            match command {
                RenderCommand::DrawArc {
                    x,
                    y,
                    sides,
                    radius,
                    rotation,
                    thickness,
                    arc,
                    color,
                } => {
                    draw_arc(
                        x,
                        y,
                        sides,
                        radius,
                        rotation,
                        thickness,
                        arc,
                        Self::to_mq_color(color),
                    );
                }

                RenderCommand::Clear { color } => {
                    clear_background(Self::to_mq_color(color));
                } // clear

                RenderCommand::DrawCircle {
                    x,
                    y,
                    radius,
                    color,
                } => {
                    draw_circle(x, y, radius, Self::to_mq_color(color));
                } // draw_circle

                RenderCommand::DrawRectangle {
                    x,
                    y,
                    width,
                    height,
                    color,
                } => {
                    draw_rectangle(x, y, width, height, Self::to_mq_color(color));
                } // draw_rectangle

                RenderCommand::DrawLine { 
                    start_x, 
                    start_y, 
                    end_x, 
                    end_y, 
                    thickness, 
                    color 
                } => {
                    draw_line(start_x, start_y, end_x, end_y, thickness, Self::to_mq_color(color));
                }

                RenderCommand::DrawTriangleLines { 
                    v1, 
                    v2, 
                    v3, 
                    thickness, 
                    color 
                } => {
                    draw_triangle_lines(
                        Vec2::new(v1.x, v1.y), 
                        Vec2::new(v2.x, v2.y), 
                        Vec2::new(v3.x, v3.y), 
                        thickness, 
                        Self::to_mq_color(color)
                    );
                }

                RenderCommand::DrawPolygonLines { 
                    x, 
                    y,
                    sides, 
                    radius, 
                    rotation, 
                    thickness, 
                    color 
                } => {
                    draw_poly_lines(x, y, sides, radius, rotation, thickness, Self::to_mq_color(color));
                }

                RenderCommand::DrawText {
                    text,
                    x,
                    y,
                    font_size,
                    color,
                } => {
                    draw_text(&text, x, y, font_size, Self::to_mq_color(color));
                } // draw_text

                RenderCommand::DrawMultilineText {
                    text,
                    x,
                    y,
                    font_size,
                    line_distance,
                    color,
                } => {
                    draw_multiline_text(
                        &text,
                        x,
                        y,
                        font_size,
                        line_distance,
                        Self::to_mq_color(color),
                    );
                } // draw_multiline_text

                RenderCommand::DrawTexture {
                    texture_key,
                    x,
                    y,
                    width,
                    height,
                    rotation,
                    color,
                } => {
                    if let Err(err) =
                        draw_cached_texture(&texture_key, x, y, width, height, rotation, color)
                    {
                        eprintln!(
                            "{}",
                            i18n::prefixed_with(
                                "renderer",
                                "renderer.error.texture_draw",
                                &[("source", &err)],
                            )
                        );
                    }
                } // draw_texture
            }
        }
    }
}
