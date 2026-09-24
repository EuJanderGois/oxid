---
title: Creating scripting plugins
slug: /technical-information/native-modules
---

# Creating scripting plugins

Scripting plugins are the unit used to expose Oxid APIs to JavaScript through QuickJS. A plugin has two responsibilities that must stay aligned:

- **runtime registration** — makes the plugin API available to JavaScript;
- **API metadata** — describes the public API to tooling and the generated `oxid.d.ts`.

Oxid keeps those responsibilities connected through `ScriptPlugin` and the central plugin registry.

## Module structure

A scripting plugin normally lives in `src/scripting/plugins/` and contains:

1. the plugin implementation;
2. the runtime registration;
3. a `ScriptPlugin` implementation;
4. metadata for exported types and functions.

A simplified module looks like this:

```rust
pub struct TimePlugin;

impl ModuleDef for TimePlugin {
    // QuickJS declarations and exports.
}

impl ScriptPlugin for TimePlugin {
    const NAME: &'static str = "oxid/time";

    fn docs() -> &'static str {
        "Frame timing utilities."
    }

    fn functions() -> &'static [FunctionMeta] {
        static FUNCTIONS: [FunctionMeta; 1] = [FunctionMeta {
            module: "oxid/time",
            name: "deltaTime",
            docs: "Returns the delta time of the current frame.",
            returns: ScriptType::Number,
            params: &[],
        }];

        &FUNCTIONS
    }
}
```

The metadata is intentionally declared next to the implementation. This avoids maintaining a second API description elsewhere in the engine.

## Step 1: implement the plugin

A plugin owns its runtime registration through `ScriptPlugin::register`. Rust-backed plugins should use the shared `register_module_def` helper after implementing `ModuleDef`; script-backed plugins can implement their registration directly when they need custom QuickJS setup.

The `NAME` constant is the JavaScript module path:

```rust
const NAME: &'static str = "oxid/time";
```

That same name must be used by the metadata.

## Step 2: describe exported types

For a class, use `TypeMeta`:

```rust
fn types() -> &'static [TypeMeta] {
    static TYPES: [TypeMeta; 1] = [TypeMeta {
        module: "oxid/time",
        name: "Timer",
        docs: "Represents a frame timer.",
        constructors: &[TypeConstructorMeta {
            params: &[],
        }],
        properties: &[],
    }];

    &TYPES
}
```

Properties use `TypePropertyMeta`. Set `readonly` to `true` when scripts must not be able to modify the property.

## Step 3: describe functions

Each exported function gets a `FunctionMeta`.

```rust
FunctionMeta {
    module: "oxid/time",
    name: "deltaTime",
    docs: "Returns the delta time of the current frame.",
    returns: ScriptType::Number,
    params: &[],
}
```

Parameters are described with `FunctionParam`:

```rust
FunctionParam {
    name: "scale",
    ty: ScriptType::Number,
    docs: "Value used to scale the duration.",
    optional: false,
}
```

**Public metadata documentation must be written in English.** These strings are emitted into generated developer tooling, so they should be clear and concise.

## Step 4: reference custom types correctly

Use the module path and exported name:

```rust
ScriptType::Custom("oxid/math", "Vector2D")
```

Do not use only `"Vector2D"`.

The module information allows the generator to automatically produce the required TypeScript import:

```ts
import type { Vector2D } from "oxid/math";
```

## Step 5: register the plugin

Plugins are collected by `src/scripting/plugins/registry.rs`.

Add the plugin to `plugins()` in the central registry. Each entry supplies the same three pieces of information: name, metadata, and registration function.

```rust
PluginRegistration {
    name: TimePlugin::NAME,
    metadata: TimePlugin::metadata,
    register: TimePlugin::register,
},
```

The registry uses the same registration object for runtime binding and metadata discovery. This is important: adding a module should not require maintaining two unrelated lists.

## Step 6: verify generated declarations

Create a project with:

```bash
oxid new my-game
```

Then inspect:

```text
my-game/
├── main.js
├── oxid.d.ts
├── package.json
└── tsconfig.json
```

The new module should appear in `oxid.d.ts`.

If the API is missing, check:

- the module is present in the registry;
- its `NAME` matches its metadata module name;
- exported functions/types have metadata;
- custom types use `ScriptType::Custom(module, name)`.

## Runtime and metadata are separate layers

The registry connects the layers, but the generator does not execute the QuickJS runtime. Every plugin follows the same registration and metadata contract, regardless of whether its implementation is Rust or JavaScript.

```text
Scripting plugin
    │
    ├── register ───────────→ QuickJS runtime
    │
    └── metadata ───────────→ TypeScript generator
                                  │
                                  └── oxid.d.ts
```

This separation makes the generator testable without starting QuickJS and keeps runtime code independent from TypeScript formatting.

## Adding a plugin checklist

Before submitting a new scripting plugin, verify:

- [ ] `ModuleDef` exposes the intended runtime API.
- [ ] `ScriptPlugin::NAME` is correct.
- [ ] exported types have `TypeMeta`.
- [ ] exported functions have `FunctionMeta`.
- [ ] parameters and return values use the correct `ScriptType`.
- [ ] custom types include their module path.
- [ ] public metadata documentation is in English.
- [ ] the module is added to `plugins()`.
- [ ] `oxid.d.ts` contains the expected declarations.
- [ ] tests cover non-trivial metadata or generator behavior.
- [ ] user-facing behavior is documented.

## Design rule

A scripting plugin owns the description and registration of its public scripting API. The central registry should only compose plugins; it should not duplicate their metadata or implementation details.

That rule keeps new modules local, makes `oxid.d.ts` deterministic, and prevents the scripting engine from becoming a single point of coupling.
