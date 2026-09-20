---
title: Lifecycle
slug: /scripting/lifecycle
---

# Lifecycle

Oxid loads the entry module defined in `oxid.entry` and expects that module to export a `main()` function.

## Boot sequence

At startup, the runtime:

1. evaluates the entry module
2. calls `main()`
3. stores the returned object
4. compiles lightweight wrappers around the supported hooks

The supported hooks today are:

- `onInit()`
- `onUpdate(dt)`
- `onDraw()`

All of them are optional.

## Hook order

The runtime executes hooks in this order:

1. `onInit()` once, before the game loop begins
2. `onUpdate(dt)` every frame
3. `onDraw()` every frame, after update

`dt` is the frame delta time provided by the renderer.

## Example

```javascript
import {GameObject} from 'oxid/core';

class MyApp extends GameObject {
  onInit() {
    console.log('ready');
  }

  onUpdate(dt) {
    this.time = (this.time ?? 0) + dt;
  }

  onDraw() {
    // draw here
  }
}

export function main() {
  return new MyApp();
}
```
