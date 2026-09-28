---
title: Architecture Decisions
slug: /architecture/decisions
---

# Architecture decisions

This page records *why* Oxid is shaped the way it is, for the decisions that most affect how you'd extend it. It intentionally does not cover every choice in the codebase — only the ones that would otherwise have to be reverse-engineered from reading multiple files at once.

Each entry marked **Current** reflects a decision already implemented and enforced in the code referenced. The final entry is marked **Possible Future** and is not implemented; it is included because it explains the shape of some current decisions (see its own note).

## Decision: `ScriptRuntime` as an abstraction over QuickJS

**Current.**

- **Context**: the engine's game loop (`runtime::game`) needs to drive a running script (init/update/draw) without caring which JavaScript engine is executing it.
- **Decision**: `ScriptRuntime` (`src/scripting/runtime.rs`) is a trait exposing only the three lifecycle hooks; `QuickJsRuntime` is its only implementation.
- **Why**: constructing a runtime (loading scripts, wiring native modules, resolving imports) is inherently backend-specific — a hypothetical second JS engine might represent contexts and modules completely differently. Keeping construction out of the trait, and only standardizing the three calls the game loop actually needs, keeps the trait meaningful instead of becoming a leaky abstraction shaped entirely around QuickJS's API.
- **Consequences**: `runtime::game` has zero `rquickjs` imports. Tests and reasoning about the game loop don't need a real JS engine running.
- **Alternatives considered**: exposing QuickJS types directly through the loop was the simpler short-term option, and is what a "just make it work" first pass would look like; it was rejected in favor of the boundary because the project's stated direction is `ScriptRuntime != QuickJS` (see [Scripting runtime](/architecture/scripting-runtime)).

## Decision: `Renderer` as an abstraction over Macroquad

**Current.**

- **Context**: the same problem as above, for graphics: the engine needs to draw a frame without hardcoding "and it's Macroquad" into the game loop.
- **Decision**: `Renderer` (`src/renderer/mod.rs`) is a three-method trait (`begin_frame`, `delta_time`, `render`); `MqRenderer` is its only implementation.
- **Why**: mirrors `ScriptRuntime`/`QuickJsRuntime` for the same reason — `Renderer != Macroquad` is a stated goal, not only an implementation detail today.
- **Consequences**: swapping or adding a rendering backend touches `MqRenderer`-equivalent code and the `RenderCommand` match inside it, not the scripting plugins that produce commands.
- **Alternatives considered**: calling Macroquad functions directly from scripting plugins, which is simpler and was in fact the shape of the very first prototype (`src/old_main.rs`, now dead code) before the queue/trait split existed.

## Decision: `RenderQueue` as the boundary between scripting and rendering

**Current.**

- **Context**: given the two abstractions above, something has to connect "a script called `drawCircle`" to "the renderer draws a circle" without either side depending on the other's internals.
- **Decision**: scripting plugins push plain `RenderCommand` values onto a `RenderQueue` (via a thread-local "active queue" pointer, valid only during `onDraw()`); the renderer drains that same queue once per frame.
- **Why**: this keeps `RenderCommand` as pure data, makes the queue itself engine-agnostic (no QuickJS or Macroquad types appear in `renderer::command`/`renderer::queue`), and means a plugin function never needs a reference to "the renderer" at all — only to whatever queue happens to be active.
- **Consequences**: a draw call outside `onDraw()` (e.g. from `onUpdate`) is a silent no-op rather than an error, because there is no active queue to push onto at that point. This is documented behavior, not a bug, but it is a real trade-off: a typo'd draw call in the wrong hook currently fails silently instead of loudly.
- **Alternatives considered**: passing the queue explicitly through every native function call (rejected — would require every plugin function's signature to carry a queue parameter, cluttering the scripting API for something the script itself has no reason to manage).

## Decision: metadata-driven `oxid.d.ts` generation

**Current.**

- **Context**: `oxid.d.ts` needs to describe the exact same API the runtime exposes, without becoming a second, independently-maintained description that can silently drift from the real bindings.
- **Decision**: each plugin declares `ModuleMeta`/`FunctionMeta`/`TypeMeta` (`src/scripting/plugins/mod.rs`) alongside its QuickJS bindings, in the same file; `scripting::generator` turns that metadata into the `.d.ts` text, and `oxid new` writes the result to disk.
- **Why**: puts the description next to the implementation it describes, so adding an API and forgetting to describe it is a local, single-file oversight rather than a separately-tracked task.
- **Consequences**: this only guards *structural* drift (a function that exists but isn't described, or is described with the wrong shape) — it does **not** currently guard *content* drift. The metadata's `docs` strings are freeform, hand-written text with no link back to the implementation; nothing stops them from being wrong or, as happened before this documentation pass, written in the wrong language for public developer tooling (fixed in this pass; see the `generated_metadata_has_no_stray_diacritics` test added to `scripting::generator`).
- **Alternatives considered**: deriving metadata automatically from the `#[rquickjs::class]`/`ModuleDef` macros via a proc-macro or build script. Not done today — the current metadata types (`FunctionParam`, `TypeMeta`, ...) are hand-written `const`/`static` data, not generated from the binding macros.

## Decision: ES Modules as the scripting module system

**Current.**

- **Context**: project scripts need a way to import both Oxid's native modules and their own other files.
- **Decision**: standard ES Module syntax (`import`/`export`), resolved by Oxid's own `ModuleResolver` (`src/scripting/resolver.rs`) — built-ins by bare name, other specifiers as relative/root-relative/project-relative file paths, always sandboxed to the project root.
- **Why**: `import { X } from "oxid/y"` reads like importing any other package, which keeps the mental model close to ordinary JavaScript/TypeScript tooling (hence `oxid.d.ts`'s `declare module "oxid/y"` shape) rather than inventing an Oxid-specific include mechanism.
- **Consequences**: the sandboxing rule (imports cannot resolve outside the project root) is a real constraint, not a suggestion — enforced by canonicalized path containment and covered by a dedicated test (`rejects_import_outside_project_directory`). A project genuinely cannot import files from elsewhere on disk.
- **Alternatives considered**: none recorded in the codebase; this has been the module model since the resolver's introduction.

## Possible Future: a multi-thread execution model (main thread + worker pool)

**Possible Future — not implemented. No code exists for this today.**

The project's stated longer-term direction separates a main thread (game logic, scripting, rendering — what exists today) from a worker pool for CPU-bound work such as asset processing, procedural generation, and pathfinding:

```text
Main Thread                    Worker Pool
├── Game                       ├── Asset processing
├── Scripting                  ├── Procedural generation
└── Rendering                  ├── Pathfinding
                                └── other CPU-bound tasks
```

This is included here for context, not as a roadmap commitment: several current design choices would need to be revisited before anything like this could exist, and knowing that helps explain why those choices look the way they do today rather than already being generalized. Concretely: the "active render queue" (`renderer::context`) and the texture cache (`renderer::texture`) are both `thread_local`, which is a single-thread assumption baked into the current implementation, not an oversight — introducing a worker pool would require deciding how (or whether) scripting and rendering state should ever be touched from more than one thread. Treat this section as "why the current single-threaded design is intentional for now," not as a description of work in progress.
