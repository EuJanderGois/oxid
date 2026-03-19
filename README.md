# Oxid

> Zero-setup game engine with instant JavaScript scripting

**Oxid** is a CLI-first game engine with a native Rust runtime and instant JavaScript scripting.

Today the main workflow is intentionally small:

```bash
oxid new my-game
cd my-game
oxid run
```

The generated project is based on `package.json`, uses `oxid.entry` as the JavaScript entrypoint, and ships editor types through `oxid.d.ts`.

## Current scope

- Native runtime written in Rust
- JavaScript as the official scripting language
- Built-in editor types via `oxid.d.ts`
- CLI commands focused on `oxid new` and `oxid run`
- Runtime modules exposed as `oxid/core`, `oxid/math`, `oxid/color`, `oxid/shapes`, `oxid/input`, `oxid/text`, and `oxid/texture`
- Modular architecture that is still evolving

## Hello Oxid

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

## Documentation

The Docusaurus documentation lives in [`website/`](website) and is organized around:

- Getting Started
- CLI
- Scripting
- API Reference
- Roadmap

## Notes

- Oxid does not document `build` or export pipelines as shipped features yet.
- Type definitions are available for editor integration, but JavaScript is still the official scripting runtime today.
