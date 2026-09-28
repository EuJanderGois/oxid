---
title: Rendering
slug: /architecture/rendering
---

# Rendering: from a script call to a pixel

```text
JavaScript API call (e.g. drawCircle(x, y, r, color))
       ↓
RenderCommand                     ← a plain enum variant, no drawing happens here
       ↓
RenderQueue                       ← an ordered Vec<RenderCommand> for the current frame
       ↓
Renderer                          ← Oxid's own trait (begin_frame / delta_time / render)
       ↓
MqRenderer                        ← the only current implementation
       ↓
Macroquad                         ← the graphics/windowing crate actually drawing pixels
```

## Why a queue instead of drawing directly

A scripting plugin function such as `drawCircle` (`src/scripting/plugins/shapes.rs`) never calls a Macroquad drawing function. It does this instead:

```rust
fn draw_circle<'js>(x: f32, y: f32, r: f32, color: OwnedBorrow<'js, Color>) {
    let _ = with_active_queue(|queue| {
        queue.draw_circle(x, y, r, to_renderer_color(&color));
    });
}
```

`with_active_queue` (`src/renderer/context.rs`) reaches into a `thread_local` pointer that is only ever `Some` while `onDraw()` is executing — set at the start of `QuickJsRuntime::on_draw` and cleared right after. If a script calls a drawing function outside of `onDraw()` (for instance, from `onUpdate`), `with_active_queue` returns `None` and the call is a **silent no-op**, not an error. This is a real behavior, not an oversight to fix casually — changing it would need a decision about whether that should become a thrown error instead (see [Architecture decisions](/architecture/decisions#decision-renderqueue-as-the-boundary-between-scripting-and-rendering)).

This indirection is what makes `RenderCommand`/`RenderQueue` a genuine engine abstraction rather than a thin wrapper around Macroquad: plugin code only ever describes *what* to draw, in engine-native terms (`Vec2`, the renderer's own `Color`, plain floats) — never *how*. A second renderer backend would not require touching any plugin code at all.

## `RenderCommand` and `RenderQueue`

`RenderCommand` (`src/renderer/command.rs`) is a plain enum — one variant per drawing operation Oxid currently supports: `Clear`, `DrawCircle`, `DrawRectangle`, `DrawArc`, `DrawLine`, `DrawTriangleLines`, `DrawPolygonLines`, `DrawText`, `DrawMultilineText`, `DrawTexture`. Each variant carries only plain data (`f32`, `String`, the renderer's own `Color`/`Vec2` — never a scripting-layer or QuickJS type).

`RenderQueue` (`src/renderer/queue.rs`) is a `Vec<RenderCommand>` with a `push`-per-shape convenience method for each variant (`draw_circle`, `draw_rectangle`, ...) plus `clear`/`drain`. It is cleared at the start of every frame (`queue.clear()` in the game loop) and has exactly one producer (the active `onDraw()` call) and one consumer (the renderer, once per frame).

The background clear is always the first command pushed each frame — `queue.clear_background(DARKGRAY)` runs before `engine.on_draw(&mut queue)` in the game loop (see [Runtime and game loop](/architecture/runtime#the-frame-loop)), so a script's own draw calls always land on top of it.

## The `Renderer` trait and `MqRenderer`

```rust
pub trait Renderer {
    fn begin_frame(&mut self);
    fn delta_time(&self) -> f32;
    fn render(&mut self, queue: &mut RenderQueue);
}
```

This is deliberately small — just enough for the game loop to drive a frame without knowing which graphics backend is underneath. `MqRenderer` (`src/renderer/mq_renderer.rs`) is the only implementation: `delta_time` wraps `macroquad::get_frame_time()`, and `render` drains the queue and matches each `RenderCommand` variant to the corresponding Macroquad call (`draw_circle`, `draw_rectangle`, `draw_arc`, `draw_poly_lines`, `draw_texture_ex`, ...).

**Macroquad is the current implementation, not the architecture.** Nothing outside `renderer::mq_renderer` and `renderer::texture` (its texture-loading counterpart) references Macroquad types directly. `Renderer != Macroquad` in the same sense that `ScriptRuntime != QuickJS` (see [Scripting runtime](/architecture/scripting-runtime)) — both are stated design intentions of the project, reflected in real trait boundaries today, not just documentation aspirations.

## Textures

`renderer::texture` (`src/renderer/texture.rs`) is a small subsystem alongside the command/queue/renderer pipeline: `load_texture` decodes an image file and caches the resulting Macroquad texture in a `thread_local` `HashMap`, keyed by the canonicalized absolute path. `drawTexture`/`drawTextureScaled` (the scripting-facing functions) don't hold the texture itself — they push a `RenderCommand::DrawTexture` carrying the cache *key* (a `String`), and `MqRenderer` looks the texture back up from the cache at render time via `draw_cached_texture`. `set_project_root` (called once, from `runtime::game::launch`) is what lets relative texture paths resolve against the project directory rather than the process's working directory — see [`oxid/texture`](/api/texture).

## Adding a new `RenderCommand`

This exact workflow is already written down, in more tutorial form, in `src/renderer/RENDER_TASK.md` (Portuguese) — reproduced and generalized here. Adding a new drawing primitive touches four places, in this order:

1. **`RenderCommand`** (`src/renderer/command.rs`) — add a new variant carrying whatever plain data the draw needs.
2. **`RenderQueue`** (`src/renderer/queue.rs`) — add a `push`-wrapping method for it, following the existing `draw_*` naming.
3. **A scripting plugin** (`src/scripting/plugins/*.rs`) — the JS-facing function that calls `with_active_queue` and pushes the new command; add its `FunctionMeta` in the same file (see [How to add a native module](/technical-information/native-modules)).
4. **`MqRenderer`** (`src/renderer/mq_renderer.rs`) — match the new variant and call the corresponding Macroquad function.

If a second `Renderer` implementation ever exists, step 4 would need to be repeated there too — which is exactly the point of the trait: the engine can be certain steps 1–3 are enough to describe a new drawing operation, independent of how many renderer backends exist to fulfill it.
