---
title: oxid/math
slug: /api/math
---

# `oxid/math`

## `Vector2D`

```javascript
import { Vector2D } from 'oxid/math';
```

`Vector2D` is the current mutable two-component value used by Oxid's 2D APIs. It represents a pair of numeric coordinates, not a transform hierarchy or a full 2D transform.

```ts
class Vector2D {
  constructor(x: number, y: number);
  x: number;
  y: number;
}
```

### Common uses

- screen positions
- texture sizes
- arc origins
- mouse position values returned by `oxid/input`

### Important distinction

`Vector2D` is intentionally a simple mathematical value. It does not currently contain rotation, scale, parent/child relationships, or transformation matrices. Those concepts should not be inferred from the name.

### Migration from older snapshots

Older Oxid snapshots exposed this same two-component value as `Transform2D`. That name was misleading because the type only stored `x` and `y` and did not represent a 2D transform.

The public API now calls it `Vector2D`. Existing code using the old name should be updated to import and construct `Vector2D` instead. A future `Transform2D` type should only be introduced when it has actual transform semantics such as position, rotation, and scale.
