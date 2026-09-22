---
title: API metadata and generated typings
slug: /scripting/api-generation
---

# API metadata and generated `oxid.d.ts`

The public scripting API is described by metadata declared alongside the Rust modules that expose it.

That metadata is used to generate the `oxid.d.ts` file created by `oxid new`.

## Source of truth

The intended flow is:

```text
Rust scripting module
       ↓
API metadata
       ↓
┌──────┴────────┐
│               │
runtime       oxid.d.ts
```

This means `oxid.d.ts` is a generated artifact, not a second hand-maintained API definition.

## What is described

Modules can describe:

- exported classes and their constructors
- public properties
- property mutability
- functions
- parameters and optional parameters
- return types
- documentation strings
- cross-module custom types

For example, `Vector2D` is declared by `oxid/math` and functions in other modules can reference it through metadata.

The metadata also prevents semantic names from drifting away from their runtime representation. For example, the old two-component `Transform2D` value was renamed to `Vector2D` because it only represented `x` and `y`; it is not kept as a misleading alias.

## Generating a project

The normal workflow is simply:

```bash
oxid new my-game
```

The command writes `oxid.d.ts` from the API metadata available in that Oxid build.

You normally should not edit this file manually. If the engine API changes, recreate or regenerate the project typings using the corresponding Oxid build.

## Adding a native API

When extending Oxid itself, keep the runtime implementation and metadata together:

1. expose the Rust function or type from the module
2. add its `FunctionMeta` or `TypeMeta`
3. use `ScriptType` for its parameter, property, and return types
4. reference custom types with both their module and exported name
5. register the module in the scripting API registry
6. update the documentation when the behavior is user-facing

The generator then includes the API in `oxid.d.ts` without a separate hand-written declaration.

## Runtime versus typings

`oxid.d.ts` does not implement the runtime. It gives JavaScript-aware editors and TypeScript's checker enough information to understand the native API while the actual implementation remains in Rust and QuickJS.
