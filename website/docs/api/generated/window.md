---
title: oxid/window
slug: /api/generated/oxid-window
sidebar_label: Reference
---

# `oxid/window`

Window and related helpers.

> This page is generated from the Rust scripting metadata. Do not edit it manually.

**Import:** `import { ... } from "oxid/window";`

## Functions

### `getWindowWidth`

Gets window width.

```ts
getWindowWidth(): number;
```

**Returns:** `number`

### `getWindowHeight`

Gets window height.

```ts
getWindowHeight(): number;
```

**Returns:** `number`

### `setWindowWidth`

Sets the window width.

```ts
setWindowWidth(width: number): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `width` | `number` | No | The new window width |

**Returns:** `void`

### `setWindowHeight`

Sets the window height.

```ts
setWindowHeight(height: number): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `height` | `number` | No | The new window height |

**Returns:** `void`

### `getWindowSize`

Gets the canvas size.

```ts
getWindowSize(): Vector2D;
```

**Returns:** `Vector2D`

### `setWindowSize`

Sets the window size.

```ts
setWindowSize(size: Vector2D): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `size` | `Vector2D` | No | The new window size |

**Returns:** `void`

