---
title: Overview
slug: /cli/overview
---

# CLI overview

Oxid is **CLI-first**.

Instead of hiding engine setup behind a large editor or build pipeline, the project entrypoint is a native command that:

- generates the initial files you need
- reads project metadata from `package.json`
- starts the runtime directly from the current directory

## Commands available today

### `oxid new`

Creates a new project directory and writes the initial scaffold.

### `oxid run`

Loads `package.json`, resolves `oxid.entry`, starts the runtime and opens the game window.

## Global locale option

The CLI also accepts:

```bash
oxid --lang en-US run
```

That option changes Oxid's own output and runtime error messages. It does not automatically localize your game content.
