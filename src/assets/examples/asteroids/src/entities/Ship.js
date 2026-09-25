import { Vector2D } from "oxid/math";
import { CANVAS_HEIGHT, CANVAS_WIDTH, SHIP_BASE, SHIP_HEIGHT, SHIP_SPEED } from "../helpers/config";
import { isKeyDown, mousePosition } from "oxid/input";
import { wrapAround } from "../helpers/utils";
import { drawTriangleLines } from "oxid/shapes";
import { COLORS } from "../helpers/colors";

export class Ship {
    constructor() {
        this.position = new Vector2D(
            CANVAS_WIDTH / 2,
            CANVAS_HEIGHT / 2
        );

        this.rotation = 0;
    }

    update(dt) {
        const step = SHIP_SPEED * dt;

        if (
            isKeyDown("ArrowRight") ||
            isKeyDown("D")
        ) {
            this.position.x += step;
        }

        if (
            isKeyDown("ArrowLeft") ||
            isKeyDown("A")
        ) {
            this.position.x -= step;
        }

        if (
            isKeyDown("ArrowDown") ||
            isKeyDown("S")
        ) {
            this.position.y += step;
        }

        if (
            isKeyDown("ArrowUp") ||
            isKeyDown("W")
        ) {
            this.position.y -= step;
        }

        const mouse = mousePosition();

        const dx = mouse.x - this.position.x;
        const dy = mouse.y - this.position.y;

        if (dx !== 0 || dy !== 0) {
            this.rotation =
                Math.atan2(dy, dx) + Math.PI / 2;
        }

        wrapAround(this.position);
    }

    getDirection() {
        return new Vector2D(
            Math.sin(this.rotation),
            -Math.cos(this.rotation)
        );
    }

    getNosePosition() {
        const direction = this.getDirection();

        return new Vector2D(
            this.position.x +
                direction.x * SHIP_HEIGHT,

            this.position.y +
                direction.y * SHIP_HEIGHT
        );
    }

    draw() {
        const direction = this.getDirection();

        const right = new Vector2D(
            Math.cos(this.rotation),
            Math.sin(this.rotation)
        );

        const nose = new Vector2D(
            this.position.x +
                direction.x * SHIP_HEIGHT,

            this.position.y +
                direction.y * SHIP_HEIGHT
        );

        const left = new Vector2D(
            this.position.x -
                direction.x * (SHIP_HEIGHT * 0.5) -
                right.x * (SHIP_BASE * 0.5),

            this.position.y -
                direction.y * (SHIP_HEIGHT * 0.5) -
                right.y * (SHIP_BASE * 0.5)
        );

        const rightVertex = new Vector2D(
            this.position.x -
                direction.x * (SHIP_HEIGHT * 0.5) +
                right.x * (SHIP_BASE * 0.5),

            this.position.y -
                direction.y * (SHIP_HEIGHT * 0.5) +
                right.y * (SHIP_BASE * 0.5)
        );

        drawTriangleLines(
            nose,
            left,
            rightVertex,
            2,
            COLORS.white
        );
    }
}