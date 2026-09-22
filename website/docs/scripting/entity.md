---
title: Entity
slug: /scripting/entity
---

# `Entity`

`oxid/core` exports `Entity`, the base class used by the default scripting pattern.

```javascript
import { Entity } from 'oxid/core';

class MyApp extends Entity {
  onDraw() {
    // drawing code
  }
}

export function main() {
  return new MyApp();
}
```

## Lifecycle

An `Entity` can implement three optional hooks:

```ts
class Entity {
  onInit?(): void;
  onUpdate?(dt: number): void;
  onDraw?(): void;
}
```

- `onInit()` runs once before the game loop.
- `onUpdate(dt)` runs every frame with the frame delta time.
- `onDraw()` runs every frame after update.

The runtime dispatches these methods by their shape. Extending `Entity` is the documented convention, but the returned object from `main()` does not have to inherit from the class to receive compatible hooks.

## Why `Entity`?

The name describes the scripting abstraction without implying that every instance is a renderable object. This leaves room for future composition, transforms, components, and other entity-oriented features without changing the public base-class name again.
