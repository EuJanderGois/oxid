---
title: oxid/texture
slug: /api/generated/oxid-texture
sidebar_label: Reference
---

# `oxid/texture`

Loading and drawing of 2D textures.

> This page is generated from the Rust scripting metadata. Do not edit it manually.

**Import:** `import { ... } from "oxid/texture";`

## Types

### `Texture2D`

Texture loaded by the runtime with dimensions and source path.

#### Properties

| Property | Type | Mutable | Description |
| --- | --- | --- | --- |
| `path` | `string` | No | Path used to load the texture. |
| `width` | `number` | No | Texture width in pixels. |
| `height` | `number` | No | Texture height in pixels. |

## Functions

### `loadTexture`

Loads a texture from disk and returns a reusable Texture2D object.

```ts
loadTexture(path: string): Texture2D;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `path` | `string` | No | Texture file path. Relative paths are resolved from the project root. |

**Returns:** `Texture2D`

### `drawTexture`

Draws a texture using its original size.

```ts
drawTexture(texture: Texture2D, position: Vector2D): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `texture` | `Texture2D` | No | Texture returned by loadTexture. |
| `position` | `Vector2D` | No | Top-left position in screen coordinates. |

**Returns:** `void`

### `drawTextureScaled`

Draws a resized texture with optional rotation in radians.

```ts
drawTextureScaled(texture: Texture2D, position: Vector2D, size: Vector2D, rotation?: number): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `texture` | `Texture2D` | No | Texture returned by loadTexture. |
| `position` | `Vector2D` | No | Top-left position in screen coordinates. |
| `size` | `Vector2D` | No | Destination width and height for the texture. |
| `rotation` | `number` | Yes | Rotation in radians. Defaults to 0 when omitted. |

**Returns:** `void`

