# Development Workflow

Oxid uses a branch-based development workflow designed to keep the `main` branch stable while allowing active development to happen independently.

The workflow separates development, integration, and stable releases.

## Branches

### `next`

The `next` branch is the primary development and integration branch.

New features, improvements, refactors, fixes, and other changes should be developed here before becoming part of a stable release.

```text
next
 │
 ├── feature
 ├── fix
 ├── refactor
 └── documentation
```

Changes are continuously integrated into `next` and validated by CI.

### `main`

The `main` branch represents the stable state of the project.

Changes should enter `main` through a Pull Request from `next`.

`main` should always contain code that is considered suitable for release.

Stable version tags are created from commits contained in `main`.

### `gh-pages`

The `gh-pages` branch contains the generated documentation website.

It is maintained automatically by the documentation deployment workflow and should not be used for normal development.

## Pull Requests

The normal path for changes is:

```text
Development
     │
     ▼
   next
     │
     │ Pull Request
     ▼
   main
```

Pull Requests are used to validate the integration of the current development state before it becomes part of the stable branch.

CI must pass before changes are merged into `main`.

## Commit Convention

Oxid uses short, descriptive commit prefixes to make the project's history easier to understand.

### `feat`

Introduces a new feature or capability.

```text
feat: add template system
```

### `fix`

Fixes an existing bug or incorrect behavior.

```text
fix: resolve project initialization error
```

### `docs`

Changes documentation only.

```text
docs: document CLI commands
```

### `refactor`

Changes the internal implementation without intentionally changing its behavior.

```text
refactor: simplify project configuration
```

### `test`

Adds or modifies tests.

```text
test: add template validation tests
```

### `ci`

Changes CI/CD workflows or automation.

```text
ci: add multiplatform release builds
```

### `chore`

General maintenance that does not fit the categories above.

```text
chore: bump version to 0.1.1
```

## Commit Guidelines

Commits should:

* describe one logical change;
* use a clear and concise message;
* use the appropriate prefix;
* avoid mixing unrelated changes whenever possible.

The goal is not to enforce a complex commit specification, but to keep the project history readable and useful.

## Typical Development Cycle

A typical Oxid development cycle looks like this:

```text
1. Develop
      │
      ▼
2. Commit changes
      │
      ▼
3. Push to next
      │
      ▼
4. CI validation
      │
      ▼
5. Pull Request
      │
      ▼
6. Merge into main
```

Stable releases are handled separately through the versioning and release process.
