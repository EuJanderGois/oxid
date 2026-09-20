---
title: Project Structure
slug: /getting-started/project-structure
---

# Generated project structure

The current scaffold is intentionally minimal:

```text
my-game/
  main.js
  oxid.d.ts
  package.json
  tsconfig.json
```

## File overview

### `main.js`

The default JavaScript entrypoint. New projects point to this file through `oxid.entry`.

### `package.json`

The project manifest. Oxid reads runtime settings from the `oxid` object, including:

- `entry`
- `title`
- `width`
- `height`
- `locale`

### `oxid.d.ts`

Type definitions for the current runtime modules. This is what makes module imports such as `oxid/text` and `oxid/input` understandable to editor tooling.

### `tsconfig.json`

Editor configuration for JavaScript projects using the generated type definitions.

## About `src/` and `assets/`

The current `oxid new` scaffold does **not** create `src/` or `assets/` directories automatically.

You can add your own folders as the project grows. For example, an `assets/` directory is a natural place to store textures, but that is a project convention, not a built-in scaffold requirement today.
