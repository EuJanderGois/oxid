---
title: oxid run
slug: /cli/run
---

# `oxid run`

Run the current project with:

```bash
oxid run
```

## Requirements

- you must be inside a project directory
- that directory must contain a valid `package.json`
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

## Legacy compatibility

Older projects that still use the top-level `main` field continue to work as a fallback, but `oxid.entry` is the current configuration key.
