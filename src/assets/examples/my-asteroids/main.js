
import { Entity } from "oxid/core";
import { Vector2D } from "oxid/math";
import { Color } from "oxid/color";
import {
    drawTextureScaled,
    loadTexture,
} from "oxid/texture";
import { drawRectangle } from "oxid/shapes";
import {
    isKeyDown,
    isMouseButtonPressed,
    mousePosition,
} from "oxid/input";

const CANVAS_WIDTH = 800;
const CANVAS_HEIGHT = 600;

class Shoot {
    texture = loadTexture("./assets/shoot.png");
    speed = 360;
    size = new Vector2D(24, 24);

    constructor(position, direction) {
        this.position = position;
        this.direction = direction;
        this.rotation = Math.atan2(direction.y, direction.x) + Math.PI / 2;
    }

    update(dt) {
        this.position.x += this.direction.x * this.speed * dt;
        this.position.y += this.direction.y * this.speed * dt;
    }

    isOutsideCanvas() {
        return (
            this.position.x + this.size.x < 0 ||
            this.position.x > CANVAS_WIDTH ||
            this.position.y + this.size.y < 0 ||
            this.position.y > CANVAS_HEIGHT
        );
    }

    draw() {
        drawTextureScaled(
            this.texture,
            this.position,
            this.size,
            this.rotation
        );
    }
}

class Player {
    position = new Vector2D(400 - 37.5, 300 - 37.5);
    speed = 180;
    rotation = 0;

    texture = loadTexture("./assets/spaceship.png");
    size = new Vector2D(75, 75);
}

export class MyApp extends Entity {
    player;
    shoots = [];

    constructor() {
        super();
        this.player = new Player();
    }

    onInit() {}

    onUpdate(dt) {
        this.updatePlayer(dt);
        this.updateShooting(dt);
    }

    updatePlayer(dt) {
        const step = this.player.speed * dt;

        if (isKeyDown("ArrowRight") || isKeyDown("D"))
            this.player.position.x += step;

        if (isKeyDown("ArrowLeft") || isKeyDown("A"))
            this.player.position.x -= step;

        if (isKeyDown("ArrowDown") || isKeyDown("S"))
            this.player.position.y += step;

        if (isKeyDown("ArrowUp") || isKeyDown("W"))
            this.player.position.y -= step;

        const mouse = mousePosition();

        const center = new Vector2D(
            this.player.position.x + this.player.size.x / 2,
            this.player.position.y + this.player.size.y / 2
        );

        const dx = mouse.x - center.x;
        const dy = mouse.y - center.y;

        const length = Math.sqrt(dx * dx + dy * dy);

        if (length > 0) {
            this.player.rotation =
                Math.atan2(dy, dx) + Math.PI / 2;
        }

        if (isMouseButtonPressed("left")) {
            this.shoot(center, mouse);
        }
    }

    shoot(origin, target) {
        const dx = target.x - origin.x;
        const dy = target.y - origin.y;

        const length = Math.sqrt(dx * dx + dy * dy);

        if (length === 0)
            return;

        const direction = new Vector2D(
            dx / length,
            dy / length
        );

        const shootPosition = new Vector2D(
            origin.x - 12,
            origin.y - 12
        );

        this.shoots.push(
            new Shoot(shootPosition, direction)
        );
    }

    updateShooting(dt) {
        for (let i = this.shoots.length - 1; i >= 0; i--) {
            const shoot = this.shoots[i];

            shoot.update(dt);

            if (shoot.isOutsideCanvas()) {
                this.shoots.splice(i, 1);
            }
        }
    }

    onDraw() {
        drawRectangle(
            this.player.position.x,
            this.player.position.y,
            this.player.size.x,
            this.player.size.y,
            new Color(0, 0, 0, 0.1)
        );

        drawTextureScaled(
            this.player.texture,
            this.player.position,
            this.player.size,
            this.player.rotation
        );

        for (const shoot of this.shoots) {
            shoot.draw();
        }
    }
}

export function main() {
    return new MyApp();
}