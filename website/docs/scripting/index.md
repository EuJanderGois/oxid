---
title: Scripting
slug: /scripting
---

# JavaScript scripting

Oxid runs game logic through a native JavaScript runtime embedded in the engine.

The current model is simple:

- your project exports a `main()` function
- `main()` returns the application object
- Oxid calls lifecycle hooks on that object when they exist
- native functionality is imported from `oxid/*` modules

Today, JavaScript is the official scripting language. Type definitions are included for better tooling, but the runtime model is still JavaScript-first.

## Current scripting model

The default application object is an [`Entity`](./scripting/entity). The built-in 2D value type is [`Vector2D`](./api/math), while native modules expose drawing, input, text, color, and texture APIs.

API declarations are generated from native module metadata; see [API metadata and generated typings](./scripting/api-generation).
