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
