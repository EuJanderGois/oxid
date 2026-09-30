---
title: Globals
slug: /api/generated/globals
---

# Global API

> This page is generated from the Rust scripting metadata. Global APIs are available without an `import`.

## `console`

Console utilities available globally.

### `console.log`

Writes a message to the Oxid console.

```ts
log(message?: string): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `message` | `string` | Yes | Message to print. |

### `console.info`

Writes an informational message to the Oxid console.

```ts
info(message?: string): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `message` | `string` | Yes | Message to print. |

### `console.warn`

Writes a warning message to the Oxid console.

```ts
warn(message?: string): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `message` | `string` | Yes | Message to print. |

### `console.error`

Writes an error message to the Oxid console.

```ts
error(message?: string): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `message` | `string` | Yes | Message to print. |

### `console.debug`

Writes a debug message to the Oxid console.

```ts
debug(message?: string): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `message` | `string` | Yes | Message to print. |

### `console.assert`

Writes a message when the supplied condition is false.

```ts
assert(condition: boolean, message?: string): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `condition` | `boolean` | No | Condition to test. |
| `message` | `string` | Yes | Message to print when the condition is false. |

### `console.trace`

Writes a trace message to the Oxid console.

```ts
trace(message?: string): void;
```

| Parameter | Type | Optional | Description |
| --- | --- | --- | --- |
| `message` | `string` | Yes | Message to print. |

### `console.clear`

Clears the console output when supported by the host.

```ts
clear(): void;
```

## `colors`

Standard colors available globally.

### `LIGHTGRAY`

Light gray color.

**Type:** `Color`

### `GRAY`

Gray color.

**Type:** `Color`

### `DARKGRAY`

Dark gray color.

**Type:** `Color`

### `YELLOW`

Yellow color.

**Type:** `Color`

### `GOLD`

Gold color.

**Type:** `Color`

### `ORANGE`

Orange color.

**Type:** `Color`

### `PINK`

Pink color.

**Type:** `Color`

### `RED`

Red color.

**Type:** `Color`

### `MAROON`

Maroon color.

**Type:** `Color`

### `GREEN`

Green color.

**Type:** `Color`

### `LIME`

Lime color.

**Type:** `Color`

### `DARKGREEN`

Dark green color.

**Type:** `Color`

### `SKYBLUE`

Sky blue color.

**Type:** `Color`

### `BLUE`

Blue color.

**Type:** `Color`

### `DARKBLUE`

Dark blue color.

**Type:** `Color`

### `PURPLE`

Purple color.

**Type:** `Color`

### `VIOLET`

Violet color.

**Type:** `Color`

### `DARKPURPLE`

Dark purple color.

**Type:** `Color`

### `BEIGE`

Beige color.

**Type:** `Color`

### `BROWN`

Brown color.

**Type:** `Color`

### `DARKBROWN`

Dark brown color.

**Type:** `Color`

### `WHITE`

White color.

**Type:** `Color`

### `BLACK`

Black color.

**Type:** `Color`

### `BLANK`

Fully transparent color.

**Type:** `Color`

### `MAGENTA`

Magenta color.

**Type:** `Color`

