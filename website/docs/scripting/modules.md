---
title: Modules
slug: /scripting/modules
---

# Modules

Oxid uses standard ES module syntax. There is no special `oxid.import(...)` API. Native engine modules use the `oxid/*` namespace and project code uses file paths.

## Native modules

```js
import { Vector2D } from "oxid/math";
import { drawCircle } from "oxid/shapes";
import { WHITE } from "oxid/color";
```

The built-in modules are registered by the scripting plugin registry and resolved by name. Their declarations are also emitted into `oxid.d.ts`.

## Project modules

Project code can use relative imports like ordinary JavaScript:

```js
// src/main.js
import { Ship } from "./entities/Ship.js";
import { CONFIG } from "./helpers/config.js";
```

The resolver anchors project paths to the project root and rejects imports that escape that root. This is an intentional sandbox boundary, not merely a path convenience.

## Resolution model

```text
import specifier
       │
       ├── oxid/... ───────► registered native module
       │
       └── relative/path ──► project root
                                  │
                                  ├─ .js module
                                  └─ reject outside root
```

## Native modules vs globals

Most engine APIs are explicit imports. A small set of APIs is intentionally global, such as `console` and the standard color constants. The distinction is visible in the generated [global API reference](../api/generated/globals).

## Type information

The generated `oxid.d.ts` describes native module exports, so editors can understand imports without changing the runtime language from JavaScript. See [API metadata](./api-generation) for the generation pipeline.
