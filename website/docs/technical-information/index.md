---
title: Technical Information
slug: /technical-information
---

# Technical Information

This section is for people who want to change Oxid itself rather than only build a game with it.

## Where to look

- [Creating scripting plugins](/technical-information/native-modules) — add a native JavaScript API and its metadata.
- [API metadata](/scripting/api-generation) — understand the single description used by typings and web reference generation.
- [Architecture](/architecture) — understand the boundaries between runtime, scripting and rendering.
- [Development workflow](/technical-information/development-workflow) — branch, validation and contribution flow.
- [CI/CD](/technical-information/ci-cd) — what automation validates and deploys.
- [Versioning and releases](/technical-information/versioning-and-releases) — how a change becomes a release.

## The extension loop

```text
Change engine code
      │
      ▼
Update scripting metadata (if public API changed)
      │
      ├──────────────► oxid.d.ts
      │
      └──────────────► web API reference
      │
      ▼
Tests + cargo check
      │
      ▼
Documentation build
```

The important rule is that public API changes should travel through the metadata layer instead of creating a second manually maintained API definition.
