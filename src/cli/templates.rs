pub const MAIN_JS: &str = r#"import { Entity } from "oxid/core";
import { Vector2D } from "oxid/math";
import { drawText } from "oxid/text";
import { Color } from "oxid/color";

export class MyApp extends Entity {
    constructor() {
        super();
        this.pos = new Vector2D(300.0, 300.0);
        this.color = new Color(1.0, 1.0, 1.0, 1.0);
    }

    onDraw() {
        drawText("Hello Oxid!", this.pos, 32.0, this.color);
    }
}

export function main() {
    return new MyApp();
}
"#;

pub const TSCONFIG_JSON: &str = r#"{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "checkJs": true,
    "allowJs": true,
    "noEmit": true,
    "strict": false,
    "types": ["./oxid.d.ts"]
  },
  "include": ["./"]
}"#;

pub fn package_json(project_name: &str, locale: &str) -> String {
    format!(
        r#"{{
  "name": "{}",
  "oxid": {{
    "entry": "main.js",
    "title": "{} - Oxid Engine",
    "width": 800,
    "height": 600,
    "locale": "{}"
  }}
}}"#,
        project_name, project_name, locale
    )
}
