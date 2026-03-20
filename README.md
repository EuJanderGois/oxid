<div align="center">
  <img src="https://raw.githubusercontent.com/EuJanderGois/oxid/refs/heads/main/website/static/img/oxid.svg" alt="Oxid Logo" width="250" />

  <h1>Oxid</h1>

  <p>
    <img src="https://img.shields.io/github/license/EuJanderGois/oxid?style=for-the-badge&logo=opensourceinitiative" alt="GitHub License" />
    <img src="https://img.shields.io/github/v/tag/EuJanderGois/oxid?style=for-the-badge&label=Version" alt="GitHub Tag" />
  </p>
</div>

> Zero-setup game engine with instant JavaScript scripting.

**Oxid** is a CLI-first game engine powered by a native Rust runtime, designed to get you building games immediately with JavaScript. 

The generated project is based on standard web tooling using `package.json`, sets `oxid.entry` as the JavaScript entrypoint, and ships editor types out-of-the-box through `oxid.d.ts`.

## 🚀 Quick Start

The core workflow is intentionally minimal and fast:

```bash
oxid new my-game
cd my-game
oxid run
```

## ✨ Current Scope

- **Native Core:** High-performance runtime written in Rust.
- **Scripting:** JavaScript as the official, instant scripting language.
- **Editor Support:** Built-in types via `oxid.d.ts` for seamless autocompletion.
- **CLI Tools:** Simple commands focused on `oxid new` and `oxid run`.
- **Modular API:** Runtime modules exposed natively as `oxid/core`, `oxid/math`, `oxid/color`, `oxid/shapes`, `oxid/input`, `oxid/text`, and `oxid/texture`.
- **Architecture:** Modular design that is continuously evolving.

## 💻 Hello Oxid

Here is a quick look at how simple it is to get something on the screen:

```javascript
import { GameObject } from "oxid/core";
import { Transform2D } from "oxid/math";
import { drawText } from "oxid/text";
import { Color } from "oxid/color";

export class MyApp extends GameObject {
    constructor() {
        super();
        this.position = new Transform2D(300.0, 300.0);
        this.color = new Color(1.0, 1.0, 1.0, 1.0);
    }

    onDraw() {
        drawText("Hello Oxid!", this.position, 32.0, this.color);
    }
}

export function main() {
    return new MyApp();
}
```

## 📚 Documentation

The Docusaurus documentation lives in the [`website/`](website) directory and is organized around:

- Getting Started
- CLI Reference
- Scripting Guide
- API Reference
- Roadmap

## 🤝 Contributing

We welcome contributions! If you want to help shape the future of Oxid, please check out our [Contributing Guidelines](CONTRIBUTING.md). 

There you will find all the necessary information about our Git workflow, how to use Atomic Commits, our Conventional Commits pattern, and step-by-step instructions on how to open a Pull Request.

## ⚠️ Notes

- Oxid does not document `build` or export pipelines as shipped features yet.
- Type definitions are available for editor integration, but JavaScript is still the official scripting runtime today.