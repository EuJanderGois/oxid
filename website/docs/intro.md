---
title: Overview
slug: /intro
---

# Overview

Oxid is a zero-setup game engine built around a small, direct workflow:

1. Create a project with the CLI.
2. Write game logic in JavaScript.
3. Run it through a native Rust runtime.

The current architecture is intentionally simple:

- The CLI is the main entrypoint.
- Projects are described through `package.json`.
- Runtime configuration lives under the `oxid` field.
- The JavaScript entry file is resolved from `oxid.entry`.
- Game scripts run against a set of built-in native modules.
- Editor types are provided through `oxid.d.ts`.

Oxid is **JavaScript-first** today. Type definitions exist to improve autocomplete and editor integration, but that is not the same thing as full official TypeScript runtime support.

The engine is also still evolving. The docs in this site focus on what is already present in the codebase today: `oxid new`, `oxid run`, the current lifecycle hooks, and the modules exposed by the runtime.
