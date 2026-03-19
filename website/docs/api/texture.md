---
title: oxid/texture
slug: /api/texture
---

# `oxid/texture`

```javascript
import {drawTexture, drawTextureScaled, loadTexture, Texture2D} from 'oxid/texture';
```

## `Texture2D`

```ts
class Texture2D {
  readonly path: string;
  readonly width: number;
  readonly height: number;
}
```

## `loadTexture(path)`

Loads a texture from disk and returns a reusable `Texture2D` object.

Relative paths are resolved from the current working directory.

## `drawTexture(texture, position)`

Draws a texture at its original size.

## `drawTextureScaled(texture, position, size, rotation?)`

Draws a texture using a destination size and optional rotation.

- `position`: `Transform2D`
- `size`: `Transform2D`
- `rotation`: radians

The runtime validates that size values are finite and greater than zero.
