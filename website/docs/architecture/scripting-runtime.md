---
title: Scripting Runtime
slug: /architecture/scripting-runtime
---

# Scripting runtime: from JavaScript to Rust

This page is about the *mechanism* that lets JavaScript call into Oxid — not the API surface itself (see [Scripting](/scripting) and the [API Reference](/api) for that). It draws the line between what is an **Oxid concept** and what is **QuickJS implementation detail**, because those are easy to conflate when reading the source for the first time.

## The layering

```text
Your JavaScript (main.js, entities/*.js, ...)
       ↓
ScriptRuntime                     ← Oxid's own abstraction (trait, no QuickJS types)
       ↓
QuickJsRuntime                    ← the only current implementation
       ↓
rquickjs                          ← Rust bindings crate
       ↓
QuickJS                           ← the actual C JavaScript engine
```

**`ScriptRuntime`** (`src/scripting/runtime.rs`) is a trait with exactly three methods: `on_init`, `on_update(delta_time)`, `on_draw(&mut RenderQueue)`. That is the entire contract the rest of the engine (`runtime::game`'s loop) depends on. Nothing outside `scripting::quickjs` needs to know that QuickJS exists at all — this mirrors how `runtime::game` only depends on the `Renderer` trait, not on Macroquad directly (see [Rendering](/architecture/rendering)).

**`QuickJsRuntime`** (`src/scripting/quickjs/mod.rs`) is the only implementation today, built on **`rquickjs`** (the Rust crate providing safe-ish bindings) wrapping **QuickJS** (Bellard's C JavaScript engine). Oxid does not hide the fact that QuickJS is what actually runs your JavaScript — there is no attempt to abstract "which JS engine" the way `ScriptRuntime` abstracts "is there even a JS engine involved at all." If Oxid ever grows a second scripting backend, it would be a sibling module implementing `ScriptRuntime`, and this module would not need to change.

Everything under `scripting::quickjs`, plus the *bindings* half of `scripting::plugins` (the `#[rquickjs::class]` structs and `ModuleDef` impls), is the only part of Oxid allowed to depend on `rquickjs` directly. The metadata half of `scripting::plugins` (`ModuleMeta`, `FunctionMeta`, `TypeMeta`, `ScriptType`) has no such dependency — see [API metadata](/scripting/api-generation).

## Bootstrap sequence

`QuickJsRuntime::new(script_code, project_root, entry_path)` runs, in order:

1. Canonicalize `project_root` and `entry_path` (resolves `..`, symlinks; also the point where a nonexistent path fails fast).
2. Create an `rquickjs::Runtime`, and install the module loader: `runtime.set_loader(FsResolver::new(project_root), FsLoader)` — see [Module resolution](#module-resolution) below.
3. Create a full `Context` (`Context::full`, which includes the standard JS built-ins QuickJS provides).
4. Inside that context:
   - **Register built-in modules** (`bootstrap::register_modules`): every plugin in the central registry (`scripting::plugins::registry::plugins()`) gets its `register` function called, which declares and evaluates its module (e.g. `oxid/math`) so it is cached and ready before any user script imports it.
   - **Bootstrap the entry module** (`bootstrap::bootstrap_entry_module`): the entry script is declared under its real, absolute path (not a placeholder name) so relative imports inside it resolve correctly, evaluated, and its namespace stored on `globalThis.__main`. Then `globalThis.__app_instance = __main.main();` runs — this is the moment a project's exported `main()` is called and its return value becomes "the game."
   - **Compile the lifecycle hooks** (`hooks::compile_hooks`): three tiny wrapper functions are `eval`'d and stored as globals (`__hook_on_init`, `__hook_on_update`, `__hook_on_draw`), each of the shape `() => { if (__app_instance.onInit) __app_instance.onInit(); }`. This is why every hook is optional on the JavaScript side — the check happens in the compiled wrapper, not in Rust.

From then on, `ScriptRuntime::on_init/on_update/on_draw` just call these three globals by name (`hooks::call_void_hook` / `call_f32_hook`).

## Module resolution

Two layers exist here for the same reason `ScriptRuntime` and `QuickJsRuntime` are separate: one is policy, one is a QuickJS-specific adapter.

**`ModuleResolver`** (`src/scripting/resolver.rs`) is Oxid's own resolution policy, and has zero QuickJS dependency — it is unit-tested by constructing a real temp-directory project and resolving specifiers against it, without starting a JS runtime at all. Given an importing file (`base`) and a specifier (`name`), it decides between:

- **Builtin** — `name` matches one of the registered `oxid/*` plugin names exactly; resolves to that name, no filesystem access.
- **Root-relative** — `name` starts with `/`; resolved against the project root.
- **Relative** — `name` starts with `.`; resolved against the *importing file's* directory.
- **Bare, non-builtin** — anything else (e.g. `"utils.js"`); resolved against the project root, same as a root-relative import.

For file-based specifiers, `.js` can be omitted (tries the literal path, then `path.js`, then `path/index.js`, in that order), and **every resolved file must canonicalize to somewhere inside the project root** — an import that would escape it (e.g. `../../../etc/passwd`, or a sibling project directory) is rejected with an error rather than resolved. This is enforced by path containment after canonicalization, not by string matching on `..`, so it holds even through symlinks.

**`FsResolver`/`FsLoader`** (`src/scripting/quickjs/loader.rs`) are the thin adapter satisfying `rquickjs::loader::{Resolver, Loader}`. `FsResolver` just calls `ModuleResolver::resolve` and translates the result into the string key QuickJS expects; `FsLoader` reads the resolved file from disk and declares it as a module. Because built-in modules are registered ahead of time (see above), `FsLoader` in practice is only ever invoked for project script files — QuickJS already finds built-ins in its own module cache.

## Error propagation

`ScriptError` (`src/scripting/error.rs`) is Oxid's own error type for this layer — every variant is a `String`-based description (`ModuleRootInit`, `RuntimeInit`, `ContextInit`, `PluginRegister`, `EntryModuleDeclare`, `EntryModuleEval`, `MainNamespace`, `AppInstance`, `HookCompile`, `HookExecution`); no `rquickjs` type appears in its definition. `scripting::quickjs` is responsible for converting `rquickjs::Error` (and, for thrown JS exceptions, `rquickjs::CaughtError`) into one of these variants at the point it's constructed, so the rest of the engine never has to know what an `rquickjs::Error` looks like. Every variant renders through the `i18n` catalog (`scripting.error.*` keys), so these messages are translatable the same way CLI messages are.

See [Runtime and game loop](/architecture/runtime#error-handling-and-the-rustjavascript-relationship) for which of these failures are fatal versus per-frame-recoverable.
