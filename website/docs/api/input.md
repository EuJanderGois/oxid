---
title: oxid/input
slug: /api/input
---

# `oxid/input`

```javascript
import {
  isKeyDown,
  isKeyPressed,
  isKeyReleased,
  mousePosition,
  isMouseButtonDown,
  isMouseButtonPressed,
  isMouseButtonReleased,
} from 'oxid/input';
```

## Keyboard

```ts
isKeyDown(key: string): boolean
isKeyPressed(key: string): boolean
isKeyReleased(key: string): boolean
```

Key names are normalized before lookup:

- case is ignored
- spaces are ignored
- `_` is ignored
- `-` is ignored

Examples:

- `"A"`
- `"ArrowLeft"`
- `"Space"`
- `"Enter"`
- `"Escape"`
- `"LeftShift"`
- `"F1"`

Unknown key names raise a runtime error.

## Mouse

```ts
mousePosition(): Transform2D
isMouseButtonDown(button: string): boolean
isMouseButtonPressed(button: string): boolean
isMouseButtonReleased(button: string): boolean
```

Accepted mouse button names today are:

- `"left"`
- `"middle"`
- `"right"`

Aliases such as `"primary"` and `"secondary"` are also accepted internally.
