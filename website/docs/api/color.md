---
title: oxid/color
slug: /api/color
---

# `oxid/color`

## `Color`

```javascript
import {Color} from 'oxid/color';
```

Represents an RGBA color used by the draw APIs.

```ts
class Color {
  constructor(r: number, g: number, b: number, a: number);
  r: number;
  g: number;
  b: number;
  a: number;
}
```

Values are floating-point numbers and are passed directly into the native renderer bindings used by Oxid.
