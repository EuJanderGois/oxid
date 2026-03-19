---
title: oxid/math
slug: /api/math
---

# `oxid/math`

## `Transform2D`

```javascript
import {Transform2D} from 'oxid/math';
```

Represents a mutable 2D position or size.

```ts
class Transform2D {
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
