---
title: Architecture Overview
slug: /architecture
---

# Architecture overview

This section explains **how Oxid works internally**, so that changing or extending the engine does not require reading the whole source tree first. It complements, rather than repeats, the [User Documentation](/intro) (how to *use* Oxid) and the [API Reference](/api) (what the scripting surface *is*).

If you are looking for a specific subsystem, jump to:

- [Runtime and game loop](/architecture/runtime) — entrypoint, project loading, the frame loop, lifecycle hooks, error handling.
- [Scripting runtime](/architecture/scripting-runtime) — how JavaScript reaches Rust, the `ScriptRuntime`/QuickJS boundary, module resolution.
- [Rendering](/architecture/rendering) — how a drawing call in a script becomes pixels on screen.
- [Architecture decisions](/architecture/decisions) — why the engine is shaped this way, and what alternatives were considered.

## The end-to-end flow

```text
CLI (oxid run)
 ↓
Project configuration (package.json → oxid.* fields)
 ↓
Project loader (reads entry file from disk)
 ↓
Script runtime (QuickJS, via the ScriptRuntime trait)
 ↓
Native Oxid modules (oxid/core, oxid/math, oxid/color, ...)
 ↓
Game lifecycle (onInit / onUpdate / onDraw)
 ↓
Render queue (a plain description of what to draw)
 ↓
Renderer (Macroquad, via the Renderer trait)
 ↓
Platform backend (Windows / Linux / macOS window + GPU)
```

Each arrow above is a real boundary in the code, not just a conceptual grouping — every stage is a distinct module, and every stage except "Project configuration" is behind a Rust trait (`ScriptRuntime`, `Renderer`) or a data type with no behavior of its own (`RenderCommand`). That is deliberate: see [Architecture decisions](/architecture/decisions) for why.

## Repository structure

The `oxid` Cargo package builds **two targets that share one codebase**: a library (`src/lib.rs`) and a binary (`src/main.rs`). This split is not cosmetic — it is the actual module boundary that answers "where do I add X?":

```text
src/
├── lib.rs              # the `oxid` library crate root
├── main.rs             # the `oxid` binary crate root (calls cli::run())
├── old_main.rs          # dead code — not referenced by any `mod`, not compiled;
│                        # a leftover early prototype, ignore it
│
├── i18n/                # pub in lib.rs — CLI/runtime message catalogs (pt-BR default, en-US)
├── renderer/            # pub in lib.rs — RenderCommand, RenderQueue, Renderer trait, MqRenderer
├── scripting/           # pub in lib.rs — ScriptRuntime trait, QuickJsRuntime, plugins, generator
│
├── cli/                 # binary-only — argument parsing, `oxid new`, `oxid run`
└── runtime/             # binary-only — project loading (package.json) and the game loop
```

A consequence worth knowing as a contributor: `runtime::game` (the actual `async fn` game loop) and `cli` are **not** part of the `oxid` library — they only exist inside the binary. If you are working inside `src/scripting/` or `src/renderer/` and want to reference "the game loop" from a doc comment, it lives in a sibling crate target, not a sibling module; there is no `crate::runtime` reachable from library code.

## "Where is X?" — a map for contributors and agents

| Question | Answer |
|---|---|
| Where is the scripting runtime? | `src/scripting/runtime.rs` (the `ScriptRuntime` trait) and `src/scripting/quickjs/mod.rs` (`QuickJsRuntime`, its only implementation) |
| Where are native modules registered? | `src/scripting/plugins/registry.rs` (`plugins()` — the single list every plugin appears in) |
| Where is a specific native module implemented? | `src/scripting/plugins/<name>.rs`, e.g. `math.rs` for `oxid/math`, `shapes.rs` for `oxid/shapes` |
| Where is the renderer abstraction? | `src/renderer/mod.rs` (the `Renderer` trait) |
| Where is the Macroquad backend? | `src/renderer/mq_renderer.rs` (`MqRenderer`) |
| Where is texture loading/caching? | `src/renderer/texture.rs` |
| Where is metadata (for `oxid.d.ts`) generated? | `src/scripting/generator.rs` (`generate_d_ts`); the metadata itself is declared per-plugin (see [API metadata](/scripting/api-generation)) |
| Where is `oxid.d.ts` actually written to disk? | `src/cli/new.rs`, via `oxid::scripting::generate_api_d_ts()` |
| Where does the CLI create projects? | `src/cli/new.rs` (`create_project`), templates in `src/cli/templates.rs` |
| Where does the project loader work? | `src/runtime/project.rs` (`load`, `load_from_current_dir`) |
| Where is module resolution (`import` handling)? | `src/scripting/resolver.rs` (`ModuleResolver`, engine-agnostic) + `src/scripting/quickjs/loader.rs` (`FsResolver`/`FsLoader`, the QuickJS-specific adapter) |
| Where should I add a new native API? | See [How to add a native module](/technical-information/native-modules) |
| Where should I add a new drawing primitive? | See [Architecture: Rendering](/architecture/rendering#adding-a-new-rendercommand) |

## What this section intentionally leaves out

This is architecture documentation, not an exhaustive code walkthrough. It does not describe every internal struct — only the ones a contributor needs to reason about before making a change. Internals not covered here (for example, `i18n`'s catalog-loading and rich-message-styling code) are stable enough to read directly when you need to touch them, and are not part of the engine's public shape in the way `ScriptRuntime` or `Renderer` are.
