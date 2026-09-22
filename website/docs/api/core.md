---
title: oxid/core
slug: /api/core
---

# `oxid/core`

## `Entity`

```javascript
import {Entity} from 'oxid/core';
```

`Entity` is the base class used by the default scripting model. It provides the lifecycle hook shape expected by the runtime.

### Hooks

```ts
class Entity {
  onInit?(): void;
  onUpdate?(dt: number): void;
  onDraw?(): void;
}
```

All hooks are optional. The runtime calls them only when they exist on the object returned by `main()`.
