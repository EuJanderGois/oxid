---
title: Oxid
slug: /intro
---

# Oxid

**A small Rust game engine with JavaScript gameplay scripting.**

Oxid is built around a short feedback loop: create a project, write JavaScript, run it. The native side owns the window, runtime, rendering boundary and built-in modules; your project owns the game logic.

## The mental model

```text
Your game
  ├─ package.json       project configuration
  ├─ main.js            entry point
  ├─ src/...            your modules
  └─ assets/...         your resources
          │
          ▼
      Oxid runtime
  ┌───────────────────┐
  │ project loader    │
  │ JS runtime        │
  │ native modules    │
  │ frame lifecycle   │
  │ render queue      │
  │ renderer          │
  └───────────────────┘
```

The important boundary is that **game code talks to Oxid through the scripting API**. You do not need to know QuickJS or Macroquad to use the engine. You only need those internals when you are debugging or extending Oxid itself.

## Start here

[**Learn Oxid →**](./learn)

If this is your first project, follow [Installation](./getting-started/installation), then [Create a project](./getting-started/create-project).

## Documentation layers

| Layer | Question it answers |
| --- | --- |
| **Learn** | What do I do next? |
| **Scripting** | How does game code execute and communicate with the engine? |
| **API Reference** | Which functions, types and globals exist? |
| **Architecture** | What happens inside Oxid? |
| **Technical Information** | How do I contribute or add native functionality? |

Oxid is still evolving, so the documentation intentionally distinguishes the public behavior that exists today from architectural directions that are only planned.
