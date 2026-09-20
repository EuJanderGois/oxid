pub mod color;
pub mod command;
pub mod context;
pub mod mq_renderer;
pub mod queue;
pub mod texture;

use crate::renderer::queue::RenderQueue;
pub use mq_renderer::MqRenderer;

pub trait Renderer {
    fn begin_frame(&mut self);
    fn delta_time(&self) -> f32;
    fn render(&mut self, queue: &mut RenderQueue);
}
