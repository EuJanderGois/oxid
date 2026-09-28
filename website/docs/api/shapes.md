---
title: oxid/shapes
slug: /api/shapes
---

# `oxid/shapes`

Immediate-mode 2D shape drawing. Every function in this module appends a draw command to the current frame's [render queue](/architecture/rendering) rather than drawing synchronously; commands are flushed by the renderer once `onDraw()` returns.

```javascript
import {
  drawArc,
  drawCircle,
  drawRectangle,
  drawLine,
  drawTriangleLines,
  drawPolygonLines,
} from 'oxid/shapes';
```

## `drawArc(position, sides, radius, rotation, thickness, arc, color)`

Draws an arc on screen.

- `position`: `Vector2D` — arc center
- `sides`: numeric curve resolution; higher values produce a smoother arc
- `radius`: arc radius
- `rotation`: starting rotation, **in degrees**
- `thickness`: line thickness
- `arc`: opening size, **in degrees**
- `color`: `Color`

## `drawCircle(x, y, radius, color)`

Draws a filled circle centered at `(x, y)`.

## `drawRectangle(x, y, width, height, color)`

Draws a filled rectangle with its top-left corner at `(x, y)`.

## `drawLine(start, end, thickness, color)`

Draws a straight line between two points.

- `start`, `end`: `Vector2D`
- `thickness`: line thickness

## `drawTriangleLines(v1, v2, v3, thickness, color)`

Draws the outline (not filled) of a triangle defined by three `Vector2D` vertices.

## `drawPolygonLines(position, sides, radius, rotation, thickness, color)`

Draws the outline of a regular polygon.

- `position`: `Vector2D` — polygon center
- `sides`: number of polygon sides
- `radius`: distance from the center to each vertex
- `rotation`: **in degrees**
- `thickness`: line thickness

## Rotation units: degrees here, radians in `oxid/texture`

`drawArc` and `drawPolygonLines` take `rotation` **in degrees**. This is not a typo or an inconsistency to work around — it is a real difference between the underlying native calls each function makes, and it is intentional: [`drawTextureScaled`](/api/texture#drawtexturescaledtexture-position-size-rotation) takes its `rotation` **in radians**. When mixing shape drawing and texture drawing in the same code, convert explicitly between the two (`degrees * Math.PI / 180`) rather than assuming a single global convention.

All functions render into the current frame's queue; they have no effect when called outside `onDraw()`.
