import { Entity } from "oxid/core";
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
