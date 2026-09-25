use oxid::{
    i18n,
    renderer::{MqRenderer, Renderer, queue::RenderQueue},
    scripting::ScriptEngine,
};

use super::LoadedProject;

pub fn launch(project: LoadedProject) {
    oxid::renderer::texture::set_project_root(&project.root);

    let config = project.window_config();

    macroquad::Window::from_config(config, run_game(project));
}

async fn run_game(project: LoadedProject) {
    let mut renderer = MqRenderer;

    let engine = match ScriptEngine::new(&project.script, &project.root, &project.entry_path) {
        Ok(engine) => engine,
        Err(err) => {
            let source = err.to_string();

            eprintln!(
                "{}",
                i18n::prefixed_with(
                    "scripting",
                    "scripting.error.bootstrap",
                    &[("source", &source)],
                )
            );

            return;
        }
    };

    let mut queue = RenderQueue::new();

    engine.on_init();

    loop {
        renderer.begin_frame();
        queue.clear();

        let dt = renderer.delta_time();
        engine.on_update(dt);

        queue.clear_background(oxid::renderer::color::DARKGRAY);

        engine.on_draw(&mut queue);

        renderer.render(&mut queue);

        macroquad::prelude::next_frame().await;
    }
}
