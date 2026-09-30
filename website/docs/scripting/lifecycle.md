---
title: Lifecycle
slug: /scripting/lifecycle
---

# Script lifecycle

The default Oxid scripting model is frame-oriented. A project exposes an `Entity` from `main()`, and the runtime invokes its optional lifecycle methods at the appropriate point in each frame.

```text
startup
  │
  ├─ load project
  ├─ resolve entry module
  ├─ evaluate module graph
  └─ call main()
          │
          ▼
      Entity instance
          │
          ├── onInit()       once
          │
          ▼
       ┌───────── frame ─────────┐
       │ onUpdate(dt)            │
       │      │                  │
       │      ▼                  │
       │ game state changes      │
       │                         │
       │ onDraw()               │
       │      │                  │
       │      ▼                  │
       │ render commands        │
       └─────────────────────────┘
```

## `onInit()`

Runs once after the entry entity has been created. Use it for initialization that should not happen every frame.

## `onUpdate(dt)`

Runs once per frame with the elapsed frame time in seconds. Use it for simulation, input handling, movement and other state changes.

## `onDraw()`

Runs once per frame after update. Drawing APIs enqueue commands into the active render queue. The renderer consumes those commands after the script draw phase.

That means drawing is intentionally separated from rendering: a call such as `drawCircle(...)` does not immediately invoke the backend renderer.

### Why this matters

Calling a draw API outside `onDraw()` currently has no active render queue and therefore has no effect. This is an architectural consequence of the scripting/rendering boundary; see [Rendering](../architecture/rendering).

## What `dt` is not

`dt` is elapsed frame time, not a fixed simulation tick. Code that requires deterministic simulation should not assume every frame has the same `dt`.
