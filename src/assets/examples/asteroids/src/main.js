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
    isKeyPressed,
    isMouseButtonDown,
} from "oxid/input";

import {
    ASTEROID_COUNT,
    ASTEROID_MAX_SIZE,
    ASTEROID_MIN_SIZE,
    CANVAS_HEIGHT,
    CANVAS_WIDTH,
    SHIP_HEIGHT,
    SHOOT_INTERVAL,
    STAR_COUNT
} from "./helpers/config";

import { COLORS } from "./helpers/colors";

import {
    distanceSquared,
    randomDirection,
    randomInt,
    randomRange,
} from "./helpers/utils";

let bestScore = 0;

import { Ship } from "./entity/Ship";
import { Bullet } from "./entity/Bullet";
import { Asteroid } from "./entity/Asteroid";

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