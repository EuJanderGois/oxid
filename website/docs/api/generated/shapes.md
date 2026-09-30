---
title: oxid/shapes
slug: /api/generated/oxid-shapes
sidebar_label: Reference
---

# `oxid/shapes`

> This page is generated from the Rust scripting metadata. Do not edit it manually.

**Import:** `import { ... } from "oxid/shapes";`

## Functions

### `drawArc`

Draws an arc on the screen.

```ts
drawArc(position: Vector2D, sides: number, radius: number, rotation: number, thickness: number, arc: number, color: Color): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `position` | `Vector2D` | No | Arc position. |
| `sides` | `number` | No | Resolution used to approximate the curve; higher values produce a smoother arc. |
| `radius` | `number` | No | Arc radius. |
| `rotation` | `number` | No | Initial rotation in degrees. |
| `thickness` | `number` | No | Arc line thickness. |
| `arc` | `number` | No | Arc opening in degrees. |
| `color` | `Color` | No | Color used for drawing. |

**Returns:** `void`

### `drawCircle`

Draws a circle on the screen.

```ts
drawCircle(x: number, y: number, radius: number, color: Color): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `x` | `number` | No | Horizontal position. |
| `y` | `number` | No | Vertical position. |
| `radius` | `number` | No | Circle radius. |
| `color` | `Color` | No | Color used for drawing. |

**Returns:** `void`

### `drawRectangle`

Draws a rectangle on the screen.

```ts
drawRectangle(x: number, y: number, width: number, height: number, color: Color): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `x` | `number` | No | Horizontal position. |
| `y` | `number` | No | Vertical position. |
| `width` | `number` | No | Rectangle width. |
| `height` | `number` | No | Rectangle height. |
| `color` | `Color` | No | Color used for drawing. |

**Returns:** `void`

### `drawLine`

Draws a line between two points.

```ts
drawLine(start: Vector2D, end: Vector2D, thickness: number, color: Color): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `start` | `Vector2D` | No | Starting point of the line. |
| `end` | `Vector2D` | No | Ending point of the line. |
| `thickness` | `number` | No | Line thickness. |
| `color` | `Color` | No | Color used for drawing. |

**Returns:** `void`

### `drawTriangleLines`

Draws the outline of a triangle.

```ts
drawTriangleLines(v1: Vector2D, v2: Vector2D, v3: Vector2D, thickness: number, color: Color): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `v1` | `Vector2D` | No | First vertex. |
| `v2` | `Vector2D` | No | Second vertex. |
| `v3` | `Vector2D` | No | Third vertex. |
| `thickness` | `number` | No | Line thickness. |
| `color` | `Color` | No | Color used for drawing. |

**Returns:** `void`

### `drawPolygonLines`

Draws the outline of a regular polygon.

```ts
drawPolygonLines(position: Vector2D, sides: number, radius: number, rotation: number, thickness: number, color: Color): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `position` | `Vector2D` | No | Center position of the polygon. |
| `sides` | `number` | No | Number of polygon sides. |
| `radius` | `number` | No | Distance from the center to each vertex. |
| `rotation` | `number` | No | Polygon rotation in degrees. |
| `thickness` | `number` | No | Line thickness. |
| `color` | `Color` | No | Color used for drawing. |

**Returns:** `void`

