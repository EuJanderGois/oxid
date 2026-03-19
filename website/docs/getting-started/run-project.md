---
title: Run a Project
slug: /getting-started/run-project
---

# Run a project

After creating a project, move into its directory and run:

```bash
cd my-game
oxid run
```

`oxid run` expects to find a `package.json` in the **current working directory**.

## How the runtime resolves your project

When you run the command, Oxid:

1. reads `package.json`
2. loads the `oxid` configuration object
3. resolves the JavaScript entrypoint from `oxid.entry`
4. reads that file and boots the runtime
5. opens the native window using the configured title, width and height

Oxid still accepts the legacy top-level `main` field as a fallback for older projects, but new projects should use `oxid.entry`.

## Locale resolution

For `oxid run`, CLI messages are resolved in this order:

1. `--lang`
2. `OXID_LANG`
3. `oxid.locale` from `package.json`
4. the default locale

This affects Oxid's own messages, not your game text.
