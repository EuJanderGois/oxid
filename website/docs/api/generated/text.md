---
title: oxid/text
slug: /api/generated/oxid-text
sidebar_label: Reference
---

# `oxid/text`

2D text rendering and measurement.

> This page is generated from the Rust scripting metadata. Do not edit it manually.

**Import:** `import { ... } from "oxid/text";`

## Types

### `TextMetrics`

Metrics calculated for a single line of text.

#### Constructor

`new TextMetrics(width: number, height: number, offsetY: number)`

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `width` | `number` | No | Text width. |
| `height` | `number` | No | Text height. |
| `offsetY` | `number` | No | Vertical metric offset. |

#### Properties

| Property | Type | Mutable | Description |
| --- | --- | --- | --- |
| `width` | `number` | No | Text width. |
| `height` | `number` | No | Text height. |
| `offset_y` | `number` | No | Vertical metric offset. |

## Functions

### `drawText`

Draws 2D text on the screen. The y coordinate represents the text baseline.

```ts
drawText(text: string, position: Vector2D, fontSize: number, color: Color): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `text` | `string` | No | Text content to draw. |
| `position` | `Vector2D` | No | Text position in screen coordinates. |
| `fontSize` | `number` | No | Font size in pixels. Must be greater than zero. |
| `color` | `Color` | No | Color used for the text. |

**Returns:** `void`

### `drawMultilineText`

Draws multiline text using '\n' as the separator. The y coordinate represents the first line baseline.

```ts
drawMultilineText(text: string, position: Vector2D, fontSize: number, color: Color, lineDistance?: number): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `text` | `string` | No | Text content to draw. |
| `position` | `Vector2D` | No | Initial text block position in screen coordinates. |
| `fontSize` | `number` | No | Font size in pixels. Must be greater than zero. |
| `color` | `Color` | No | Color used for the text. |
| `lineDistance` | `number` | Yes | Line spacing multiplier. Use 1.0 for default spacing. |

**Returns:** `void`

### `measureText`

Measures a single line of text using the default font and returns its width, height, and offset_y.

```ts
measureText(text: string, fontSize: number): TextMetrics;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `text` | `string` | No | Text content to measure. |
| `fontSize` | `number` | No | Font size in pixels. Must be greater than zero. |

**Returns:** `TextMetrics`

