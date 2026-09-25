import { Vector2D } from "oxid/math";
import { randomRange, wrapAround } from "../helpers/utils";
import { drawPolygonLines } from "oxid/shapes";
import { COLORS } from "../helpers/colors";

export class Asteroid {
    constructor(position, velocity, size, sides) {
        this.position = new Vector2D(
            position.x,
            position.y
        );

        this.velocity = new Vector2D(
            velocity.x,
            velocity.y
        );

        this.size = size;
        this.sides = sides;

        this.rotation = 0;
        this.rotationSpeed = randomRange(-2, 2);

        this.collided = false;
    }

    update(dt) {
        this.position.x +=
            this.velocity.x * dt;

        this.position.y +=
            this.velocity.y * dt;

        this.rotation +=
            this.rotationSpeed * dt;

        wrapAround(this.position);
    }

    draw() {
        const rotationDegrees =
            this.rotation * 180 / Math.PI;

        drawPolygonLines(
            this.position,
            this.sides,
            this.size,
            rotationDegrees,
            2,
            COLORS.white
        );
    }
}