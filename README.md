<div align="center">

  <img src="https://raw.githubusercontent.com/EuJanderGois/oxid/refs/heads/main/website/static/img/oxid.svg" alt="Oxid Logo" width="250" />

  <h1>Oxid</h1>

  <p>
    <img src="https://img.shields.io/github/license/EuJanderGois/oxid?style=for-the-badge&logo=opensourceinitiative" alt="GitHub License" />
    <img src="https://img.shields.io/github/v/tag/EuJanderGois/oxid?style=for-the-badge&label=Version" alt="GitHub Tag" />
  </p>

</div>

> Zero-setup game engine with instant JavaScript scripting.

**Oxid** is a CLI-first game engine powered by a native Rust runtime, designed to let you start building games immediately with JavaScript.

The goal is simple: install Oxid, create a project, write JavaScript, and run the game without a complex engine setup.

## 🚀 Quick Start

Install the Oxid CLI using a prebuilt release or build it from source.

See the [Installation Guide](https://eujandergois.github.io/oxid/getting-started/installation) for installation instructions.

Once installed:

```bash
oxid new my-game
cd my-game
oxid run
```

That's it.

Oxid creates the project structure and provides the runtime and editor type definitions needed to start scripting immediately.

## ✨ Current Features

* **Native Runtime** — Core engine functionality is implemented in Rust.
* **JavaScript Scripting** — JavaScript is the official scripting language.
* **CLI-First Workflow** — Create and run projects directly from the command line.
* **Editor Support** — Projects include `oxid.d.ts` for JavaScript autocompletion and type information.
* **Project Templates** — New projects are generated from Oxid templates.
* **Modular API** — Engine functionality is exposed through dedicated JavaScript modules.
* **Cross-Platform Releases** — Official releases provide prebuilt binaries for Windows, Linux, and macOS.
* **Automated Releases** — Version tags are validated and released automatically through GitHub Actions.

## 💻 Hello Oxid

A minimal Oxid application can look like this:

```javascript
import { Entity } from "oxid/core";
import { Vector2D } from "oxid/math";
import { drawText } from "oxid/text";
import { Color } from "oxid/color";

export class MyApp extends Entity {
    constructor() {
        super();

        this.position = new Vector2D(300.0, 300.0);
        this.color = new Color(1.0, 1.0, 1.0, 1.0);
    }

    onDraw() {
        drawText(
            "Hello Oxid!",
            this.position,
            32.0,
            this.color
        );
    }
}

export function main() {
    return new MyApp();
}
```

The intention is to keep the scripting layer simple while the native runtime handles the underlying engine functionality.

## 📦 Releases

Official Oxid releases provide prebuilt binaries for supported platforms.

Each release contains platform-specific packages such as:

```text
oxid-vX.Y.Z-windows-x86_64.zip
oxid-vX.Y.Z-linux-x86_64.tar.gz
oxid-vX.Y.Z-macos-x86_64.tar.gz
```

See the [latest releases](https://github.com/EuJanderGois/oxid/releases) to download a prebuilt version.

## 📚 Documentation

The full documentation is available in the `website/` directory and is published through the Oxid documentation site.

Documentation is organized into:

* **Getting Started** — Installation and first steps.
* **CLI** — Command reference and usage.
* **Scripting** — JavaScript development with Oxid.
* **API Reference** — Engine modules and APIs.
* **Templates** — Project templates and project generation.
* **Technical Information** — Development workflow, versioning, CI/CD, and project architecture.
* **Roadmap** — Planned improvements and future direction.

Start with the [Getting Started](https://eujandergois.github.io/oxid/getting-started/installation) section.

## 🛠️ Development

Oxid uses a branch-based development workflow.

```text
next
  │
  │ Pull Request
  ▼
main
  │
  │ version tag
  ▼
vX.Y.Z
  │
  ▼
GitHub Release
```

The `next` branch is used for active development and integration.

The `main` branch represents the stable, releasable state of the project.

Changes are validated through CI before being merged into `main`.

For more information, see the [Technical Information](https://eujandergois.github.io/oxid/technical-information/development-workflow.md) documentation.

## 🤝 Contributing

Contributions are welcome.

Before contributing, please read the [Contributing Guidelines](CONTRIBUTING.md).

The contribution guide covers:

* Git workflow
* Branch usage
* Commit conventions
* Pull Requests
* Development practices

If you are planning a larger change, opening an issue or discussion before implementation can help align the work with the project's direction.

## 📐 Project Status

Oxid is actively evolving.

The engine, CLI, scripting API, project templates, and development workflow are still under development and may change as the project matures.

The `0.x` version range reflects this early development stage.

For planned features and longer-term goals, see the [Roadmap](https://eujandergois.github.io/oxid/roadmap).

## 📄 License

Oxid is released under the license included in this repository.
