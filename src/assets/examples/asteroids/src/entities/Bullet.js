import { Vector2D } from "oxid/math";
import { BULLET_LIFETIME, BULLET_SPEED } from "../helpers/config";
import { isOutsideCanvas } from "../helpers/utils";
import { drawCircle } from "oxid/shapes";
import { COLORS } from "../helpers/colors";

export class Bullet {
    constructor(position, direction) {
        this.position = new Vector2D(
            position.x,
            position.y
        );

        this.direction = new Vector2D(
            direction.x,
            direction.y
        );

        this.life = 0;
        this.collided = false;
    }

    update(dt) {
        this.position.x +=
            this.direction.x *
            BULLET_SPEED *
            dt;

        this.position.y +=
            this.direction.y *
            BULLET_SPEED *
            dt;

        this.life += dt;
    }

    isExpired() {
        return (
            this.life >= BULLET_LIFETIME ||
            isOutsideCanvas(this.position, 4)
        );
    }

    draw() {
        drawCircle(
            this.position.x,
            this.position.y,
            2.5,
            COLORS.bullet
        );
    }
}