import { Entity } from "oxid/core";
import { Vector2D } from "oxid/math";
import { Color } from "oxid/color";

import {
    drawCircle,
    drawRectangle,
    drawTriangleLines,
    drawPolygonLines,
} from "oxid/shapes";

import {
    drawText,
} from "oxid/text";

import {
    isKeyDown,
    isKeyPressed,
    isMouseButtonDown,
    mousePosition,
} from "oxid/input";

const CANVAS_WIDTH = 800;
const CANVAS_HEIGHT = 600;

const SHIP_HEIGHT = 25;
const SHIP_BASE = 22;

const SHIP_SPEED = 180;

const BULLET_SPEED = 420;
const BULLET_LIFETIME = 1.5;
const SHOOT_INTERVAL = 0.15;

const ASTEROID_COUNT = 10;
const ASTEROID_MIN_SIZE = 12;
const ASTEROID_MAX_SIZE = 45;

const STAR_COUNT = 80;

const COLORS = {
    background: new Color(0.008, 0.012, 0.025, 1),

    white: new Color(0.9, 0.93, 1, 1),
    gray: new Color(0.55, 0.6, 0.7, 1),

    bullet: new Color(0.85, 0.9, 1, 1),

    red: new Color(1, 0.25, 0.25, 1),
    green: new Color(0.35, 1, 0.55, 1),

    star: new Color(0.7, 0.8, 1, 0.65),
};

let bestScore = 0;

function randomRange(min, max) {
    return min + Math.random() * (max - min);
}

function randomInt(min, max) {
    return Math.floor(randomRange(min, max + 1));
}

function randomDirection() {
    const angle = randomRange(0, Math.PI * 2);

    return new Vector2D(
        Math.cos(angle),
        Math.sin(angle)
    );
}

function distanceSquared(a, b) {
    const dx = a.x - b.x;
    const dy = a.y - b.y;

    return dx * dx + dy * dy;
}

function wrapAround(position) {
    if (position.x < 0)
        position.x = CANVAS_WIDTH;

    if (position.x > CANVAS_WIDTH)
        position.x = 0;

    if (position.y < 0)
        position.y = CANVAS_HEIGHT;

    if (position.y > CANVAS_HEIGHT)
        position.y = 0;
}

function isOutsideCanvas(position, margin = 0) {
    return (
        position.x < -margin ||
        position.x > CANVAS_WIDTH + margin ||
        position.y < -margin ||
        position.y > CANVAS_HEIGHT + margin
    );
}

class Ship {
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

class Bullet {
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

class Asteroid {
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

export class MyApp extends Entity {
    ship;

    bullets = [];
    asteroids = [];
    stars = [];

    shootTimer = 0;

    score = 0;

    gameOver = false;
    gameWon = false;

    constructor() {
        super();

        this.ship = new Ship();

        this.createStars();
        this.createAsteroids();
    }

    onInit() {
    }

    onUpdate(dt) {
        if (this.gameOver || this.gameWon) {
            if (isKeyPressed("Enter")) {
                this.restart();
            }

            return;
        }

        this.shootTimer -= dt;

        this.ship.update(dt);

        this.updateBullets(dt);
        this.updateAsteroids(dt);

        this.handleCollisions();
        this.handleShooting();

        if (this.asteroids.length === 0) {
            this.gameWon = true;

            if (this.score > bestScore) {
                bestScore = this.score;
            }
        }
    }

    onDraw() {
        this.drawBackground();

        this.ship.draw();

        for (const bullet of this.bullets) {
            bullet.draw();
        }

        for (const asteroid of this.asteroids) {
            asteroid.draw();
        }

        this.drawHud();

        if (this.gameOver) {
            this.drawGameOver();
            return;
        }

        if (this.gameWon) {
            this.drawVictory();
        }
    }

    drawBackground() {
        drawRectangle(
            0,
            0,
            CANVAS_WIDTH,
            CANVAS_HEIGHT,
            COLORS.background
        );

        for (const star of this.stars) {
            drawCircle(
                star.position.x,
                star.position.y,
                star.size,
                star.color
            );
        }
    }

    createStars() {
        this.stars = [];

        for (let i = 0; i < STAR_COUNT; i++) {
            const alpha =
                randomRange(0.25, 0.8);

            this.stars.push({
                position: new Vector2D(
                    randomRange(0, CANVAS_WIDTH),
                    randomRange(0, CANVAS_HEIGHT)
                ),

                size: randomRange(0.5, 1.5),

                color: new Color(
                    0.7,
                    0.8,
                    1,
                    alpha
                ),
            });
        }
    }

    createAsteroids() {
        this.asteroids = [];

        for (let i = 0; i < ASTEROID_COUNT; i++) {
            const position =
                this.createAsteroidPosition();

            const direction =
                randomDirection();

            const speed =
                randomRange(40, 100);

            const velocity = new Vector2D(
                direction.x * speed,
                direction.y * speed
            );

            const size = randomRange(
                ASTEROID_MIN_SIZE,
                ASTEROID_MAX_SIZE
            );

            const sides = randomInt(6, 10);

            this.asteroids.push(
                new Asteroid(
                    position,
                    velocity,
                    size,
                    sides
                )
            );
        }
    }

    createAsteroidPosition() {
        let position;

        do {
            position = new Vector2D(
                randomRange(40, CANVAS_WIDTH - 40),
                randomRange(40, CANVAS_HEIGHT - 40)
            );
        } while (
            distanceSquared(
                position,
                this.ship.position
            ) < 150 * 150
        );

        return position;
    }

    handleShooting() {
        if (
            isMouseButtonDown("left") &&
            this.shootTimer <= 0
        ) {
            this.shoot();

            this.shootTimer = SHOOT_INTERVAL;
        }
    }

    shoot() {
        const direction =
            this.ship.getDirection();

        const position =
            this.ship.getNosePosition();

        this.bullets.push(
            new Bullet(
                position,
                direction
            )
        );
    }

    updateBullets(dt) {
        for (
            let i = this.bullets.length - 1;
            i >= 0;
            i--
        ) {
            const bullet = this.bullets[i];

            bullet.update(dt);

            if (
                bullet.isExpired() ||
                bullet.collided
            ) {
                this.bullets.splice(i, 1);
            }
        }
    }

    updateAsteroids(dt) {
        for (const asteroid of this.asteroids) {
            asteroid.update(dt);
        }
    }

    handleCollisions() {
        this.handleBulletCollisions();
        this.handleShipCollisions();
    }

    handleBulletCollisions() {
        for (const bullet of this.bullets) {
            if (bullet.collided)
                continue;

            for (const asteroid of this.asteroids) {
                if (asteroid.collided)
                    continue;

                const radius = asteroid.size;

                if (
                    distanceSquared(
                        bullet.position,
                        asteroid.position
                    ) <= radius * radius
                ) {
                    bullet.collided = true;
                    asteroid.collided = true;

                    this.score +=
                        this.getAsteroidScore(
                            asteroid.size
                        );

                    this.splitAsteroid(asteroid);

                    break;
                }
            }
        }

        this.removeDestroyedAsteroids();
    }

    getAsteroidScore(size) {
        if (size >= 30)
            return 100;

        if (size >= 18)
            return 50;

        return 25;
    }

    handleShipCollisions() {
        for (const asteroid of this.asteroids) {
            const radius =
                asteroid.size +
                SHIP_HEIGHT * 0.5;

            if (
                distanceSquared(
                    this.ship.position,
                    asteroid.position
                ) <= radius * radius
            ) {
                this.gameOver = true;

                if (this.score > bestScore) {
                    bestScore = this.score;
                }

                return;
            }
        }
    }

    splitAsteroid(asteroid) {
        if (
            asteroid.size <=
            ASTEROID_MIN_SIZE
        ) {
            return;
        }

        const newSize =
            asteroid.size / 2;

        const perpendicular = new Vector2D(
            -asteroid.velocity.y,
            asteroid.velocity.x
        );

        const length = Math.sqrt(
            perpendicular.x *
                perpendicular.x +
            perpendicular.y *
                perpendicular.y
        );

        if (length > 0) {
            perpendicular.x /= length;
            perpendicular.y /= length;
        }

        const speed1 =
            randomRange(60, 160);

        const speed2 =
            randomRange(60, 160);

        const velocity1 = new Vector2D(
            perpendicular.x * speed1,
            perpendicular.y * speed1
        );

        const velocity2 = new Vector2D(
            -perpendicular.x * speed2,
            -perpendicular.y * speed2
        );

        this.asteroids.push(
            new Asteroid(
                asteroid.position,
                velocity1,
                newSize,
                randomInt(6, 10)
            )
        );

        this.asteroids.push(
            new Asteroid(
                asteroid.position,
                velocity2,
                newSize,
                randomInt(6, 10)
            )
        );
    }

    removeDestroyedAsteroids() {
        this.asteroids =
            this.asteroids.filter(
                asteroid =>
                    !asteroid.collided
            );
    }

    drawHud() {
        drawText(
            "ASTEROIDS",
            new Vector2D(20, 30),
            24,
            COLORS.white
        );

        drawText(
            "Mouse: aim",
            new Vector2D(20, 55),
            16,
            COLORS.gray
        );

        drawText(
            "Left click: shoot",
            new Vector2D(20, 78),
            16,
            COLORS.gray
        );

        drawText(
            "WASD / Arrows: move",
            new Vector2D(20, 101),
            16,
            COLORS.gray
        );

        drawText(
            "Score: " + this.score,
            new Vector2D(
                CANVAS_WIDTH - 150,
                30
            ),
            20,
            COLORS.white
        );

        drawText(
            "Best: " + bestScore,
            new Vector2D(
                CANVAS_WIDTH - 150,
                55
            ),
            16,
            COLORS.gray
        );

        drawText(
            "Asteroids: " +
                this.asteroids.length,
            new Vector2D(
                20,
                CANVAS_HEIGHT - 20
            ),
            18,
            COLORS.white
        );
    }

    drawGameOver() {
        const centerX =
            CANVAS_WIDTH / 2;

        drawText(
            "GAME OVER",
            new Vector2D(
                centerX - 70,
                CANVAS_HEIGHT / 2 - 45
            ),
            32,
            COLORS.red
        );

        drawText(
            "Score: " + this.score,
            new Vector2D(
                centerX - 45,
                CANVAS_HEIGHT / 2
            ),
            20,
            COLORS.white
        );

        drawText(
            "Best: " + bestScore,
            new Vector2D(
                centerX - 35,
                CANVAS_HEIGHT / 2 + 30
            ),
            18,
            COLORS.gray
        );

        drawText(
            "Press ENTER to try again.",
            new Vector2D(
                centerX - 105,
                CANVAS_HEIGHT / 2 + 70
            ),
            18,
            COLORS.white
        );
    }

    drawVictory() {
        const centerX =
            CANVAS_WIDTH / 2;

        drawText(
            "YOU WIN!",
            new Vector2D(
                centerX - 65,
                CANVAS_HEIGHT / 2 - 55
            ),
            32,
            COLORS.green
        );

        drawText(
            "Final score: " + this.score,
            new Vector2D(
                centerX - 70,
                CANVAS_HEIGHT / 2 - 10
            ),
            20,
            COLORS.white
        );

        drawText(
            "Best score: " + bestScore,
            new Vector2D(
                centerX - 70,
                CANVAS_HEIGHT / 2 + 20
            ),
            18,
            COLORS.gray
        );

        drawText(
            "Press ENTER to play again.",
            new Vector2D(
                centerX - 110,
                CANVAS_HEIGHT / 2 + 65
            ),
            18,
            COLORS.white
        );
    }

    restart() {
        this.ship = new Ship();

        this.bullets = [];

        this.shootTimer = 0;

        this.score = 0;

        this.gameOver = false;
        this.gameWon = false;

        this.createAsteroids();
    }
}

export function main() {
    return new MyApp();
}