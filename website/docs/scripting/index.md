---
title: Scripting
slug: /scripting
---

# Scripting

Oxid uses JavaScript for gameplay code and exposes native engine functionality through standard ES module imports. The goal is to make the scripting boundary feel like ordinary JavaScript while keeping the native engine responsible for platform, rendering and runtime work.

## The execution model

```text
main.js
  │
  ▼
project loader
  │
  ▼
QuickJS module graph
  │
  ├── project modules
  └── oxid/* native modules
          │
          ▼
      Entity lifecycle
      ├─ onInit()
      ├─ onUpdate(dt)
      └─ onDraw()
             │
             ▼
       render command queue
             │
             ▼
          renderer
```

## Learn in this order

1. [Lifecycle](./lifecycle) — when `onInit`, `onUpdate` and `onDraw` execute.
2. [Modules](./modules) — how project files and native `oxid/*` modules are resolved.
3. [Entity](./entity) — the object model used by the default script entrypoint.
4. [Types](./types) — the values shared by modules, such as `Vector2D` and `Color`.
5. [API metadata](./api-generation) — how the runtime API becomes `oxid.d.ts` and web reference pages.

## A useful distinction

There are three different things that are easy to confuse:

- **JavaScript source** — your game's behavior.
- **Native module binding** — Rust code that makes an API callable from JavaScript.
- **Metadata** — a static description of that public API used by tooling and documentation.

Keeping these concepts separate is important when extending Oxid. See [Creating scripting plugins](../technical-information/native-modules).
