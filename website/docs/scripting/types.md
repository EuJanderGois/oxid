---
title: Types and editor support
slug: /scripting/types
---

# Types and editor support

Oxid projects receive an `oxid.d.ts` file containing declarations for the scripting API exposed by that Oxid build.

## Generated declarations

The declarations are generated from the same metadata used to describe the native scripting modules. They cover:

- `Entity`
- `Vector2D`
- `Color`
- `TextMetrics`
- `Texture2D`
- native scripting functions and their parameters

This provides autocomplete and JavaScript-aware type checking when `checkJs` is enabled in the generated `tsconfig.json`.

## JavaScript remains the runtime language

`oxid.d.ts` does not add TypeScript execution to the engine. The runtime still evaluates JavaScript through QuickJS.

The normal workflow is:

```text
JavaScript source
      ↓
QuickJS runtime

oxid.d.ts
      ↓
editor / static checking
```

See [API metadata and generated typings](./api-generation) for the extension and generation model.
