---
title: Creating native scripting modules
slug: /technical-information/native-modules
---

# Creating native scripting modules

Native scripting modules are Rust modules exposed to JavaScript through QuickJS. A module has two responsibilities that must stay aligned:

- **runtime registration** — makes the native API available to JavaScript;
- **API metadata** — describes the public API to tooling and the generated `oxid.d.ts`.

Oxid keeps those responsibilities connected through `NativePlugin` and the scripting module registry.

## Module structure

A native module normally lives in `src/scripting/` and contains:

1. the Rust implementation;
2. the `ModuleDef` binding;
3. a `NativePlugin` implementation;
4. metadata for exported types and functions.

A simplified module looks like this:

```rust
pub struct TimePlugin;

impl ModuleDef for TimePlugin {
    // QuickJS declarations and exports.
}

impl NativePlugin for TimePlugin {
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

## Step 1: implement the QuickJS module

Implement `ModuleDef` using the existing native modules as examples.

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

## Step 5: register the module

Native modules are collected by `src/scripting/registry.rs`.

Add the plugin to `native_modules()`:

```rust
TimePlugin::registration(),
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

The registry connects the layers, but the generator does not execute the QuickJS runtime.

```text
Native module
    │
    ├── ModuleDef ──────────→ QuickJS runtime
    │
    └── NativePlugin
             │
             └── metadata ──→ TypeScript generator
                                  │
                                  └── oxid.d.ts
```

This separation makes the generator testable without starting QuickJS and keeps runtime code independent from TypeScript formatting.

## Adding a module checklist

Before submitting a new native module, verify:

- [ ] `ModuleDef` exposes the intended runtime API.
- [ ] `NativePlugin::NAME` is correct.
- [ ] exported types have `TypeMeta`.
- [ ] exported functions have `FunctionMeta`.
- [ ] parameters and return values use the correct `ScriptType`.
- [ ] custom types include their module path.
- [ ] public metadata documentation is in English.
- [ ] the module is added to `native_modules()`.
- [ ] `oxid.d.ts` contains the expected declarations.
- [ ] tests cover non-trivial metadata or generator behavior.
- [ ] user-facing behavior is documented.

## Design rule

A native module should own the description of its public scripting API. The central registry should only compose modules; it should not duplicate their metadata.

That rule keeps new modules local, makes `oxid.d.ts` deterministic, and prevents the scripting engine from becoming a single point of coupling.
