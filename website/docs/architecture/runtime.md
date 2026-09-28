---
title: Runtime and Game Loop
slug: /architecture/runtime
---

# Runtime and game loop

This page follows what actually happens, in order, from typing `oxid run` to a frame appearing on screen — and what happens when something along the way fails.

## Entrypoint

`src/main.rs` is intentionally thin:

```rust
mod cli;
mod runtime;

fn main() {
    cli::run();
}
```

`cli::run()` (`src/cli/mod.rs`) parses arguments with `clap`, resolves which locale Oxid's own messages should use (see below), and dispatches to either `cli::new::create_project` or `cli::run::run_project`. Everything described on this page starts from the `Run` branch.

## Project loading

`cli::run::run_project` calls into `runtime::load_project` / `runtime::load_project_from_current_dir` (`src/runtime/project.rs`), which:

1. looks for `package.json` in the target directory (current directory, or the `path` argument to `oxid run` — see [`oxid run`](/cli/run));
2. parses it, expecting an `oxid` object with `title`, `width`, `height`, and optionally `entry`;
3. resolves the entry file from `oxid.entry`, falling back to the top-level `main` field for older projects;
4. reads that entry file's source into memory;
5. returns a `LoadedProject` carrying the script source, the canonical project root, the entry path, and the window title/dimensions.

Every failure in this sequence (missing `package.json`, invalid JSON, missing entry file, unreadable entry file) returns a `String` error built from the `runtime.*` i18n catalog, and is printed by `cli::run()`'s caller before the process exits with a non-zero code. None of this touches the script runtime yet — a project with a syntactically broken `main.js` still loads successfully at this stage; it only fails once the script runtime tries to evaluate it.

## Launching

`runtime::launch` (`src/runtime/game.rs`) does two things before starting the loop:

1. `oxid::renderer::texture::set_project_root(&project.root)` — this is what makes texture paths resolve relative to the *project*, not the process's current working directory (see [`oxid/texture`](/api/texture)).
2. `macroquad::Window::from_config(config, run_game(project))` — hands control to Macroquad, which opens the native window using the loaded title/width/height and then polls the given future (`run_game`) once per frame.

## The frame loop

`run_game` (an `async fn`) is the actual game loop:

```text
construct QuickJsRuntime  ──▶ on failure: print error, return (window closes)
       │
       ▼
engine.on_init()                          # once
       │
       ▼  ┌─────────────────────────────────────────────┐
       └─▶│ renderer.begin_frame()                       │
           │ queue.clear()                                │
           │ dt = renderer.delta_time()                    │
           │ engine.on_update(dt)                           │
           │ queue.clear_background(DARKGRAY)                │
           │ engine.on_draw(&mut queue)                        │
           │ renderer.render(&mut queue)                         │
           │ macroquad::next_frame().await                        │
           └─────────────────────────────────────────────┘ (repeat)
```

A few details worth calling out explicitly because they are easy to get wrong by guessing from the API alone:

- **The background is always cleared to a hardcoded dark gray** (`renderer::color::DARKGRAY`, `(0.31, 0.31, 0.31, 1.0)`) before every `onDraw()` call. There is currently no scripting API to change the clear color — a script can only draw over it. **Not yet implemented**: a scripting-facing way to set the background color.
- `onUpdate(dt)` always runs before `onDraw()` in the same frame, never interleaved with rendering.
- There is no `onShutdown` (or equivalent) hook today. **Not yet implemented.** The loop simply runs until the window is closed by the platform; Oxid does not currently run any script-defined cleanup logic on exit.
- `renderer.begin_frame()` on `MqRenderer` is a no-op today — it exists on the `Renderer` trait for backends that need per-frame setup, but the only current backend doesn't.

## Error handling and the Rust/JavaScript relationship

Two different failure modes are handled differently, on purpose:

- **Bootstrap failures** (the `QuickJsRuntime::new(...)` call itself fails — a syntax error in the entry module, a plugin that fails to register, `main()` missing or throwing) are fatal: the error is printed and `run_game` returns before entering the loop, which closes the window immediately. There is no partially-running game in this case.
- **Per-frame hook failures** (`onInit`, `onUpdate`, or `onDraw` throws, or was never defined) are non-fatal: `QuickJsRuntime::on_init/on_update/on_draw` catch the error from `hooks::call_void_hook`/`call_f32_hook`, print it via the `scripting.error.on_*` i18n messages, and the loop continues to the next frame. A script that throws inside `onUpdate` every frame will spam the console every frame, but the window stays open.

This asymmetry reflects a deliberate line: things that mean "this project cannot run at all" abort; things that mean "this frame's script logic misbehaved" degrade gracefully instead of crashing the native window.

The JavaScript side of the relationship is minimal by design: from Rust's point of view, a running game is just three callable functions (`__hook_on_init`, `__hook_on_update`, `__hook_on_draw`) stored as QuickJS globals — see [Scripting runtime](/architecture/scripting-runtime) for how those are compiled from the object `main()` returns.
