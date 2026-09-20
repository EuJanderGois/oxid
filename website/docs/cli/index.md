---
title: CLI
slug: /cli
---

# The Oxid CLI

The CLI is the main way to use Oxid today.

It is responsible for:

- scaffolding new projects
- selecting Oxid's active locale
- loading and running the current project

The command surface is intentionally small at the moment:

- `oxid new <project-name>`
- `oxid run`
- `--lang <locale>`

That small surface is deliberate. Oxid is still growing, and the current focus is the create-run-edit loop rather than a large command set.
