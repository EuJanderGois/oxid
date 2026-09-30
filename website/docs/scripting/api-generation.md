---
title: API metadata and generated documentation
slug: /scripting/api-generation
---

# API metadata and generated documentation

Oxid describes its public scripting API once, in Rust metadata placed alongside the native binding. That metadata now feeds two developer-facing outputs:

```text
                 ModuleMeta / TypeMeta / FunctionMeta
                              │
                    ┌─────────┴─────────┐
                    ▼                   ▼
              oxid.d.ts            web API pages
                    │                   │
              editor tooling       Docusaurus
```

## Why this exists

Without a shared description, the following can drift independently:

1. what the runtime actually exports;
2. what `oxid.d.ts` tells the editor exists;
3. what the website tells a developer exists.

Metadata does not eliminate every documentation error — semantic behavior still needs prose — but it makes the structural API contract single-source.

## What metadata describes

- module name and description
- exported classes
- constructors and constructor parameters
- properties and mutability
- functions and parameters
- optional parameters
- return types
- cross-module custom types

## Generate the reference

From the repository root:

```bash
oxid docs
```

This writes the generated Markdown pages to `website/docs/api/generated/`. The documentation workflow runs the same command before Docusaurus builds the site.

`oxid new` continues to generate `oxid.d.ts` for game projects.

## What should remain hand-written

Do not put every semantic detail into metadata. Generated reference is ideal for signatures and inventory; conceptual documentation is better for things such as:

- lifecycle restrictions;
- coordinate conventions;
- units such as degrees vs radians;
- caching and ownership behavior;
- architectural reasons;
- examples that combine several modules.

This is why the site has both a generated reference and hand-written module guides.

## Adding a native API

When adding a function:

1. implement the runtime binding;
2. add its `FunctionMeta`;
3. describe every parameter and its return type;
4. use `ScriptType::Custom(module, name)` for exported types from another module;
5. register the plugin;
6. run `cargo test`;
7. run `oxid docs`;
8. update the conceptual guide if the behavior needs explanation.

See [Creating scripting plugins](../technical-information/native-modules) for the complete implementation flow.
