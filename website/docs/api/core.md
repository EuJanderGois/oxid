---
title: oxid/core
slug: /api/core
---

# `oxid/core`

## `GameObject`

```javascript
import {GameObject} from 'oxid/core';
```

`GameObject` is the base class used by the current scripting model.

### Hooks

```ts
class GameObject {
  onInit?(): void;
  onUpdate?(dt: number): void;
  onDraw?(): void;
}
```

All hooks are optional. The runtime calls them only when they exist on the object returned by `main()`.
