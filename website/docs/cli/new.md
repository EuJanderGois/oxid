---
title: oxid new
slug: /cli/new
---

# `oxid new`

Create a new project with:

```bash
oxid new my-game
```

## What it does

Today, `oxid new`:

- creates a directory named after the project argument
- writes `package.json`
- writes `main.js`
- writes `oxid.d.ts`
- writes `tsconfig.json`

It also prints follow-up instructions showing how to enter the directory and run the project.

## Generated runtime configuration

The generated `package.json` includes:

- `oxid.entry`
- `oxid.title`
- `oxid.width`
- `oxid.height`
- `oxid.locale`

## Failure behavior

If the directory already exists, the command fails instead of modifying it.

## Locale note

If you call `oxid new` with `--lang`, the generated `oxid.locale` follows that active CLI locale. Otherwise, Oxid falls back to the current locale resolution and its default locale.
