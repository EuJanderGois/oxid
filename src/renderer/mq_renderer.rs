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
