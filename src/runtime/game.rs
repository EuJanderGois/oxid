use oxid::{
    i18n,
    renderer::{MqRenderer, Renderer, queue::RenderQueue},
    scripting::ScriptEngine,
};

pub async fn run_game(script: String) {
    let mut renderer = MqRenderer;

    let engine = match ScriptEngine::new(&script) {
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
