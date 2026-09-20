---
title: oxid/shapes
slug: /api/shapes
---

# `oxid/shapes`

```javascript
import {drawArc, drawCircle, drawRectangle} from 'oxid/shapes';
```

## `drawArc(position, sides, radius, rotation, thickness, arc, color)`

Draws an arc on screen.

- `position`: `Transform2D`
- `sides`: numeric curve resolution
- `radius`: arc radius
- `rotation`: starting rotation in degrees
- `thickness`: line thickness
- `arc`: opening in degrees
- `color`: `Color`

## `drawCircle(x, y, radius, color)`

Draws a circle.

## `drawRectangle(x, y, width, height, color)`

Draws a rectangle.

All functions render immediately into the current frame queue managed by the runtime.
