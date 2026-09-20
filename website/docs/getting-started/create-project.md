---
title: Create a Project
slug: /getting-started/create-project
---

# Create a project

Use the CLI to scaffold a new Oxid project:

```bash
oxid new my-game
```

This creates a new directory and writes four files:

- `package.json`
- `main.js`
- `oxid.d.ts`
- `tsconfig.json`

If the target directory already exists, `oxid new` stops with an error instead of overwriting it.

## What `package.json` contains

The generated manifest stores Oxid runtime settings under the `oxid` field:

```json
{
  "name": "my-game",
  "oxid": {
    "entry": "main.js",
    "title": "my-game - Oxid Engine",
    "width": 800,
    "height": 600,
    "locale": "pt-BR"
  }
}
```

The exact locale value depends on the CLI locale resolution in effect when the project is created.

## Why `oxid.d.ts` and `tsconfig.json` are included

Oxid scripts run as JavaScript, but the generated project also includes editor support:

- `oxid.d.ts` describes the modules currently exposed by the runtime
- `tsconfig.json` enables `checkJs` and points tooling at those types

That gives you autocomplete and basic checking in editors such as VS Code, while keeping JavaScript as the runtime language.
