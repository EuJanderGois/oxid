# Como adicionar tarefa de renderização

Tarefas de renderização são a maneira que os scripts se comunicam com o renderizador dizendo a ele o que, quando e como renderizar.

Para compreendermos este tutorial vamos adicionar juntos `DrawCircle` ao registro de renderização.

## Render Command

O primeiro passo é registrar o comando em `command.rs`.

```rust
pub enum RenderCommand {
    // ... código anterior
    DrawCircle {
        x: f32,
        y: f32,
        radius: f32,
        color: Color,
    },
}
```

## Render Queue

Depois de registrado o comando também devemos registrar na fila (Queue) em `queue.rs`.

``` rust
pub fn draw_circle(&mut self, x: f32, y: f32, radius: f32, color: Color) {
    self.push(RenderCommand::DrawCircle {
        x,
        y,
        radius,
        color,
    });
}
```

## Plugin

O proximo passo é definir como o plugin irá chamar o comando e adicionar a fila. No nosso exemplo ele é definido em `shapes.rs`.

``` rust
fn draw_circle(x: f32, y: f32, r: f32) {
    let _ = with_active_queue(|queue| {
        queue.draw_circle(x, y, r, Color::new(1.0, 0.5, 0.0, 1.0));
    });
}
```

## Renderer

O passo final é definir como o renderer irá renderizar a tarefa. Vamos usar no nosso exemplo o renderizador macroquad `mq_renderer.rs`.

``` rust
use macroquad::prelude::{draw_circle};

match command {
    crate::renderer::command::RenderCommand::DrawCircle {
        x, y, radius, color,
    } => {
        draw_circle(x, y, radius, Self::to_mq_color(color));
    }
}
```
