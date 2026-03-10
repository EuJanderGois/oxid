use oxid::{
    renderer::{MqRenderer, Renderer, queue::RenderQueue},
    scripting::ScriptEngine,
};

#[macroquad::main("Modular Scripting")]
async fn main() {
    let script = r#"
        import { GameObject } from "oxid/core";
        import { Transform2D } from "oxid/math";
        import { drawCircle, drawRectangle } from "oxid/graphics";

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
                drawRectangle(50.0, 50.0, 100.0, 20.0);
            }
        }

        export function main() {
            return new MyApp();
        }
    "#;

    let mut renderer = MqRenderer;
    let engine = ScriptEngine::new(script);
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