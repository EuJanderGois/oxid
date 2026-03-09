use oxid::renderer::{
    color::DARKGRAY,
    mq_renderer::MqRenderer,
    queue::RenderQueue,
    Renderer,
};
use oxid::scripting::ScriptEngine;

#[macroquad::main("Oxid GE")]
async fn main() {
    let script = r#"
        import { GameObject } from "oxid/core";
        import { Transform2D } from "oxid/math";
        import { drawCircle } from "oxid/graphics";

        export class MyApp extends GameObject {
            
            constructor() {
                super();
                this.jogador = new Transform2D(100.0, 100.0);
                this.velocidade = 24.0;
            }

            onUpdate(dt) {
                this.jogador.x += this.velocidade * dt;

                if (this.jogador.x > 800.0) {
                    this.jogador.x = 0;
                }
            }

            onDraw() {
                drawCircle(this.jogador.x, this.jogador.y, 25.0);
            }
        }

        export function main() {
            return new MyApp();
        }
    "#;

    let renderer: MqRenderer = MqRenderer;
    let engine = ScriptEngine::new(script);
    let mut queue = RenderQueue::new();
    
    engine.on_init();

    loop {
        renderer.begin_frame();
        queue.clear();

        let dt = renderer.delta_time();
        engine.on_update(dt);

        queue.clear_background(DARKGRAY);

        engine.on_draw();
        let mut queue = engine.take_render_queue();
        renderer.render(&mut queue);

        macroquad::prelude::next_frame().await;
    }
}