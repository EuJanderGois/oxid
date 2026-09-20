---
title: GameObject
slug: /scripting/game-object
---

# `GameObject`

`oxid/core` exports a `GameObject` class:

```javascript
import {GameObject} from 'oxid/core';
```

The class mirrors the hook shape used by the runtime:

- `onInit?(): void`
- `onUpdate?(dt: number): void`
- `onDraw?(): void`

## Recommended usage

The default Oxid pattern is to extend `GameObject` and return that instance from `main()`:

```javascript
import {GameObject} from 'oxid/core';

class MyApp extends GameObject {
  onDraw() {
    // drawing code
  }
}

export function main() {
  return new MyApp();
}
```

## Important detail

The runtime itself only checks whether the returned app object has these hook methods. Extending `GameObject` is the intended and documented path, but the hook dispatch is shape-based rather than inheritance-enforced.
