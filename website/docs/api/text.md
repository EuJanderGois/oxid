---
title: oxid/text
slug: /api/text
---

# `oxid/text`

```javascript
import {drawMultilineText, drawText, measureText, TextMetrics} from 'oxid/text';
```

## `TextMetrics`

```ts
class TextMetrics {
  readonly width: number;
  readonly height: number;
  readonly offset_y: number;
}
```

Returned by `measureText`.

## `drawText(text, position, fontSize, color)`

Draws a single line of text.

- `position` uses `Transform2D`
- `fontSize` must be greater than zero
- the `y` coordinate is treated as the text baseline

## `drawMultilineText(text, position, fontSize, color, lineDistance?)`

Draws multi-line text using `\n` as the separator.

- `position.y` is the baseline of the first line
- `lineDistance`, when provided, must be greater than zero

## `measureText(text, fontSize)`

Measures a single line of text and returns `TextMetrics`.
