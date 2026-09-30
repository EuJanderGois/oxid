---
title: oxid/input
slug: /api/generated/oxid-input
sidebar_label: Reference
---

# `oxid/input`

> This page is generated from the Rust scripting metadata. Do not edit it manually.

**Import:** `import { ... } from "oxid/input";`

## Functions

### `isKeyDown`

Returns true while the key is held down.

```ts
isKeyDown(key: string): boolean;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `key` | `string` | No | Key name. Case-insensitive; spaces, '_' and '-' are ignored. Examples: "A", "ArrowLeft", "Space", "Enter", "Escape", "LeftShift", "F1". |

**Returns:** `boolean`

### `isKeyPressed`

Returns true only during the frame when the key was pressed.

```ts
isKeyPressed(key: string): boolean;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `key` | `string` | No | Key name. Case-insensitive; spaces, '_' and '-' are ignored. Examples: "A", "ArrowLeft", "Space", "Enter", "Escape", "LeftShift", "F1". |

**Returns:** `boolean`

### `isKeyReleased`

Returns true only during the frame when the key was released.

```ts
isKeyReleased(key: string): boolean;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `key` | `string` | No | Key name. Case-insensitive; spaces, '_' and '-' are ignored. Examples: "A", "ArrowLeft", "Space", "Enter", "Escape", "LeftShift", "F1". |

**Returns:** `boolean`

### `mousePosition`

Returns the current mouse position in screen coordinates.

```ts
mousePosition(): Vector2D;
```

**Returns:** `Vector2D`

### `isMouseButtonDown`

Returns true while the mouse button is held down.

```ts
isMouseButtonDown(button: string): boolean;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `button` | `string` | No | Mouse button name. Accepted values: "left", "middle", and "right". |

**Returns:** `boolean`

### `isMouseButtonPressed`

Returns true only during the frame when the mouse button was pressed.

```ts
isMouseButtonPressed(button: string): boolean;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `button` | `string` | No | Mouse button name. Accepted values: "left", "middle", and "right". |

**Returns:** `boolean`

### `isMouseButtonReleased`

Returns true only during the frame when the mouse button was released.

```ts
isMouseButtonReleased(button: string): boolean;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `button` | `string` | No | Mouse button name. Accepted values: "left", "middle", and "right". |

**Returns:** `boolean`

