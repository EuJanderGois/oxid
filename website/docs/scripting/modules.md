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

## Importing your own files

Besides the built-in modules above, scripts can `import`/`export` from other
`.js` files inside the project:

```js title="entities/player.js"
import { Entity } from "oxid/core";

export class Player extends Entity {
  onUpdate(dt) {}
}
```

```js title="main.js"
import { Player } from "./entities/player.js";

export function main() {
  return new Player();
}
```

- Relative specifiers (`./`, `../`) resolve relative to the file doing the
  importing.
- Bare specifiers without a leading `.` (e.g. `"entities/player.js"`) resolve
  relative to the project root (the folder containing `package.json`).
- The `.js` extension can be omitted; `./entities/player` and
  `./entities/player.js` both resolve to `entities/player.js`. A directory
  import (`./entities`) resolves to `entities/index.js` if present.
- Imports can only reach files inside the project directory — a specifier that
  would resolve outside of it (e.g. via `../../..`) fails to load.
- Each file is evaluated once, no matter how many other files import it.

## API metadata

Each native module also describes its public types and functions through metadata. That metadata is used to generate `oxid.d.ts`, so the runtime API and editor declarations share the same source of truth.

When adding an API to Oxid itself, update the module metadata alongside the implementation instead of editing a generated declaration file.
