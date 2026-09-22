---
title: Modules
slug: /scripting/modules
---

# Built-in modules

The runtime currently registers these modules:

- `oxid/core`
- `oxid/math`
- `oxid/color`
- `oxid/shapes`
- `oxid/input`
- `oxid/text`
- `oxid/texture`

## What each module is for

### `oxid/core`

Base `Entity` class used by the default scripting pattern.

### `oxid/math`

2D utility types such as `Vector2D`.

### `oxid/color`

Color values used by the drawing APIs.

### `oxid/shapes`

Immediate drawing helpers for arcs, circles and rectangles.

### `oxid/input`

Keyboard and mouse queries backed by the native runtime.

### `oxid/text`

2D text rendering and text measurement.

### `oxid/texture`

Texture loading and textured drawing helpers.

## API metadata

Each native module also describes its public types and functions through metadata. That metadata is used to generate `oxid.d.ts`, so the runtime API and editor declarations share the same source of truth.

When adding an API to Oxid itself, update the module metadata alongside the implementation instead of editing a generated declaration file.
