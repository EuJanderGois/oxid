use oxid::{
    renderer::{MqRenderer, Renderer, queue::RenderQueue},
    scripting::ScriptEngine,
};

#[macroquad::main("Modular Scripting")]
async fn main() {
    let script = r#"
        import { GameObject } from "oxid/core";
        import { Transform2D } from "oxid/math";
        import { isKeyDown, isMouseButtonDown, mousePosition } from "oxid/input";
        import { drawCircle, drawRectangle, drawArc } from "oxid/shapes";
        import { drawText, drawMultilineText, measureText } from "oxid/text";
        import { loadTexture, drawTextureScaled } from "oxid/texture";
        import { Color } from "oxid/color";

        const RED = new Color(1.0, 0.0, 0.0, 1.0);
        const WHITE = new Color(1.0, 1.0, 1.0, 1.0);
        const LABEL = "PLAYER";

        export class MyApp extends GameObject {
            constructor() {
                super();
                this.jogador = new Transform2D(100.0, 100.0);
                this.arco = new Transform2D(80.0, 80.0);
                this.ui = new Transform2D(20.0, 30.0);
                this.mascotePos = new Transform2D(320.0, 120.0);
                this.mascoteSize = new Transform2D(160.0, 160.0);
                this.mascoteRotacao = 0.0;
                this.velocidade = 180.0;
            }

            onInit() {
                this.mascote = loadTexture("src/assets/oxil.png");
            }

            onUpdate(dt) {
                const step = this.velocidade * dt;

                if (isKeyDown("ArrowRight") || isKeyDown("D")) this.jogador.x += step;
                if (isKeyDown("ArrowLeft") || isKeyDown("A")) this.jogador.x -= step;
                if (isKeyDown("ArrowDown") || isKeyDown("S")) this.jogador.y += step;
                if (isKeyDown("ArrowUp") || isKeyDown("W")) this.jogador.y -= step;

                this.arco = mousePosition();
                this.mascoteRotacao += dt * 0.6;
            }

            onDraw() {
                const labelMetrics = measureText(LABEL, 24.0);
                const labelPosition = new Transform2D(
                    this.jogador.x - labelMetrics.width / 2.0,
                    this.jogador.y - 35.0,
                );

                drawMultilineText(
                    "WASD/setas movem.\nSegure o mouse esquerdo para desenhar o arco.\nO mascote usa oxid/texture.",
                    this.ui,
                    24.0,
                    WHITE,
                    1.2,
                );
                drawText(LABEL, labelPosition, 24.0, WHITE);
                drawCircle(this.jogador.x, this.jogador.y, 25.0, RED);
                drawRectangle(50.0, 50.0, 100.0, 20.0, RED);
                drawTextureScaled(
                    this.mascote,
                    this.mascotePos,
                    this.mascoteSize,
                    this.mascoteRotacao,
                );

                if (isMouseButtonDown("left")) {
                    drawArc(this.arco, 64, 40.0, 0.0, 8.0, 140.0, RED);
                }
            }
        }

        export function main() {
            return new MyApp();
        }
    "#;

    let mut renderer = MqRenderer;
    
    let engine = match ScriptEngine::new(script) {
        Ok(engine) => engine,
        Err(err) => {
            eprintln!("[scripting/bootstrap] {err}");
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
