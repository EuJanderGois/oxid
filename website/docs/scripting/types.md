---
title: Types
slug: /scripting/types
---

# Type definitions and editor support

Oxid ships `oxid.d.ts` in newly generated projects so editors can understand the runtime modules and their public APIs.

## What this gives you

- autocomplete for `oxid/*` imports
- signatures for classes and functions exposed by the runtime
- basic checking when `checkJs` is enabled

## What this does not mean

This does **not** mean Oxid has official TypeScript runtime support today.

The current scripting flow is:

- write JavaScript
- use `oxid.d.ts` for tooling
- let your editor use `tsconfig.json` for JS-aware feedback

If you personally transpile other languages to JavaScript, that is outside the official runtime flow documented here.
