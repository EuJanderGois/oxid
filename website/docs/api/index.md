---
title: API Reference
slug: /api
---

# API Reference

This is the reference for the JavaScript surface exposed by the current Oxid runtime.

## Two levels of reference

Use the **module guides** when you need to understand behavior. They contain examples, coordinate conventions, lifecycle restrictions and links into the engine architecture.

Use the **Generated reference** when you need an exact inventory of declarations. Those pages are generated from `ModuleMeta`, `FunctionMeta`, `TypeMeta` and related metadata in the Rust source. The same metadata also generates `oxid.d.ts`.

```text
Rust plugin implementation
        │
        ├──────────────► runtime registration
        │
        ▼
   API metadata
      │     │
      │     ├──────────► generated oxid.d.ts
      │     │
      │     └──────────► generated web API pages
      │
      ▼
hand-written conceptual guide
```

This gives us one structural source of truth without pretending that signatures can document every semantic rule.

## Module guides

- [`oxid/core`](./core) — `Entity` and the script lifecycle.
- [`oxid/math`](./math) — `Vector2D` and coordinate values.
- [`oxid/color`](./color) — `Color` and global color constants.
- [`oxid/shapes`](./shapes) — immediate-mode shape drawing and render commands.
- [`oxid/input`](./input) — keyboard and mouse state.
- [`oxid/text`](./text) — text drawing and measurement.
- [`oxid/texture`](./texture) — texture loading, caching and drawing.

## Generated reference

The generated pages include constructors, properties, functions, parameter types and metadata descriptions for every registered module, plus global APIs. They are regenerated with:

```bash
oxid docs
```

The documentation CI runs this command before building the site, so a metadata change cannot silently leave the reference behind.

## Shared conventions

- **Coordinates:** screen space, origin `(0, 0)` at the top-left, `x` right and `y` down.
- **Colors:** `Color` components normally use the `0.0`–`1.0` range.
- **Drawing:** shape, text and texture calls enqueue render commands during `onDraw()`. See [Rendering](../architecture/rendering).
- **Rotation:** check the individual API. Shapes currently use degrees for arc/polygon rotation, while `drawTextureScaled` uses radians.
- **Types:** `Vector2D` is a coordinate value, not a transform object.
