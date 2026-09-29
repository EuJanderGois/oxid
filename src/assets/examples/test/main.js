import { Entity } from "oxid/core";
import { Vector2D } from "oxid/math";
import { drawText, measureText } from "oxid/text";
import { Color } from "oxid/color";
import { isKeyDown } from "oxid/input";
import { getWindowHeight, getWindowWidth, setWindowSize } from "oxid/window";

export class MyApp extends Entity {
    constructor() {
        super();
        this.text = "Hello, Oxid!";
        this.fontSize = 32;
        this.pos = this.getTextCenter();
        this.color = new Color(1.0, 1.0, 1.0, 1.0);
    }

    getTextCenter() {
        return new Vector2D(
            (getWindowWidth() / 2) - (measureText(this.text, this.fontSize).width / 2),
            (getWindowHeight() / 2) - (measureText(this.text, this.fontSize).height /2)
        );
    }

    onUpdate() {
        if (isKeyDown("F")) {
            setWindowSize(new Vector2D(1280, 720));
            this.pos = this.getTextCenter();
        }
    }

    onDraw() {
        drawText(this.text, this.pos, this.fontSize, this.color);
    }
}

export function main() {
    return new MyApp();
}
