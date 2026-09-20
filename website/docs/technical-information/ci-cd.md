# CI/CD

Oxid uses GitHub Actions to automate validation, documentation deployment, and software releases.

The automation is divided into three responsibilities:

```text
CI
│
├── Validate code
│
Documentation
│
├── Build documentation
└── Publish website
│
Release
│
├── Build binaries
└── Publish GitHub Release
```

## Continuous Integration

CI validates changes before they become part of the stable branch.

The Rust pipeline checks:

```text
cargo fmt
cargo check
cargo test
```

The documentation pipeline verifies that the Docusaurus website can be built successfully.

This prevents broken code or documentation from being merged into `main`.

## Documentation Deployment

Documentation is deployed automatically from `main`.

The workflow:

```text
main
 │
 ▼
Build Docusaurus
 │
 ▼
Generate static website
 │
 ▼
Deploy to gh-pages
```

The generated documentation is therefore tied to the stable state of the project.

The `gh-pages` branch is an output of this process rather than a development branch.

## Release Automation

Release automation is triggered by version tags:

```text
git push origin v0.1.1
```

This starts the release workflow.

The workflow validates the release and builds the Oxid binary for the supported platforms.

```text
             v0.1.1
                │
                ▼
        Validate release
                │
       ┌────────┼────────┐
       ▼        ▼        ▼
    Windows   Linux    macOS
       │        │        │
       └────────┼────────┘
                ▼
        GitHub Release
```

The generated binaries are uploaded automatically as release assets.

## Automation Principles

The CI/CD system follows a few important principles:

### `main` is stable

The stable branch should always represent a releasable state.

### Releases are explicit

A release is created by pushing a version tag rather than by every merge into `main`.

### Repetitive work is automated

Formatting checks, tests, documentation deployment, binary builds, and release publication are handled by GitHub Actions.

### Developers make release decisions

Automation performs validation and publishing, but the decision about when to create a new version remains explicit.

## Workflow Overview

The complete development and release lifecycle is:

```text
                    ┌───────────┐
                    │   next    │
                    └─────┬─────┘
                          │
                          │ Pull Request
                          ▼
                    ┌───────────┐
                    │   main    │
                    └─────┬─────┘
                          │
              ┌───────────┴───────────┐
              │                       │
              ▼                       ▼
       Documentation               Version Tag
              │                       │
              ▼                       ▼
          gh-pages              Release Workflow
                                      │
                              ┌───────┼───────┐
                              ▼       ▼       ▼
                           Windows  Linux   macOS
                                      │
                                      ▼
                              GitHub Release
```

This separation keeps development, validation, documentation, and distribution independent while allowing them to work together as one automated workflow.
