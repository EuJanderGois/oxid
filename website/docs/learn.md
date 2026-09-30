---
title: Learn Oxid
slug: /learn
---

# Learn Oxid

The documentation is organized around the path you actually take while building a game. You should not have to know the engine architecture before you can make something work.

## The path from zero to a running game

```text
Install Oxid
    │
    ▼
oxid new my-game
    │
    ▼
Project manifest + JavaScript + generated typings
    │
    ▼
Write gameplay code
    │
    ├──────────────► import { ... } from "oxid/..."
    │                         │
    │                         ▼
    │                   Native scripting API
    │
    ▼
oxid run
    │
    ▼
Project loader → JavaScript runtime → Entity lifecycle
                                      │
                                      ▼
                                  onDraw()
                                      │
                                      ▼
                                Render queue
                                      │
                                      ▼
                                  Renderer
```

## Choose where to start

- **I have never used Oxid** → [Installation](./getting-started/installation) → [Create a project](./getting-started/create-project) → [Run a project](./getting-started/run-project).
- **I want to understand scripts** → [Scripting](./scripting) → [Lifecycle](./scripting/lifecycle) → [Modules](./scripting/modules) → [Entity](./scripting/entity).
- **I know what API I need** → [API Reference](./api) → choose a module → follow the links to the underlying concept.
- **I want to understand the engine** → [Architecture](./architecture) → [Runtime](./architecture/runtime) → [Scripting runtime](./architecture/scripting-runtime) → [Rendering](./architecture/rendering).
- **I want to extend Oxid** → [Native modules](./technical-information/native-modules) → [API metadata](./scripting/api-generation) → [Architecture decisions](./architecture/decisions).

## How to read the reference

The **API Reference** is the contract you call from JavaScript. The **generated reference** underneath it is produced directly from the same Rust metadata that generates `oxid.d.ts`. Conceptual pages remain hand-written because signatures alone cannot explain lifecycle rules, coordinate systems, resource ownership, or why an API behaves the way it does.

This separation is deliberate: generated pages answer **“what exists?”**; conceptual pages answer **“when should I use it, how does it behave, and how does it connect to the rest of the engine?”**.
