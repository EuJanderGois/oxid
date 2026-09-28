---
title: oxid run
slug: /cli/run
---

# `oxid run`

Run a project with:

```bash
oxid run
oxid run <path>
```

## Arguments

- `path` (optional) — path to the project directory to run. When omitted, Oxid uses the **current working directory**.

```bash
# runs the project in the current directory
oxid run

# runs a project located elsewhere, without cd'ing into it first
oxid run ./games/my-game
oxid run /absolute/path/to/my-game
```

`path` also affects locale resolution (see below): when it is given, Oxid reads `oxid.locale` from *that* project's `package.json`, not from the current directory's.

## Requirements

- the target directory (current directory, or `path` if given) must contain a valid `package.json`
- the manifest must contain an `oxid` object
- the configured entry file must exist

## What it loads

Oxid reads:

```json
{
  "oxid": {
    "entry": "main.js",
    "title": "My Game",
    "width": 800,
    "height": 600
  }
}
```

The runtime then reads the entry file from `oxid.entry`, evaluates the JavaScript module, calls its exported `main()` function, and drives the lifecycle hooks of the returned object.

## Asset paths and `path`

Once the runtime starts, relative asset paths used by the game (for example, the `path` argument to [`loadTexture`](/api/texture)) resolve against the **project's own root directory** — the directory containing its `package.json` — not against whatever directory you happened to run `oxid run` from. This matters specifically when you use `oxid run <path>` from outside the project: asset paths in the game's code still work the same way as if you had `cd`'d into the project first.

## Legacy compatibility

Older projects that still use the top-level `main` field continue to work as a fallback, but `oxid.entry` is the current configuration key.
