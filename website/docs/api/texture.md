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

Loads a texture from disk and returns a reusable `Texture2D` object. Textures are cached by their resolved absolute path, so calling `loadTexture` again with the same path returns the cached texture instead of decoding the file again.

Relative paths are resolved against the **project's root directory** (the directory containing that project's `package.json`) — not against the process's current working directory. In practice these are the same directory when you run `oxid run` from inside the project, but they can differ when the project is started with [`oxid run <path>`](/cli/run#asset-paths-and-path) from somewhere else. Either way, a relative path like `"assets/player.png"` always means the same file relative to the project itself.

Absolute paths are used as-is.

## `drawTexture(texture, position)`

Draws a texture at its original size, top-left corner at `position`.

## `drawTextureScaled(texture, position, size, rotation?)`

Draws a texture using a destination size and optional rotation.

- `position`: `Vector2D` — top-left corner
- `size`: `Vector2D` — destination width/height; both components must be finite and greater than zero
- `rotation`: optional, **in radians** (defaults to `0`). Note this is radians, not degrees — see the [rotation units note](/api/shapes#rotation-units-degrees-here-radians-in-oxidtexture) in `oxid/shapes`.
