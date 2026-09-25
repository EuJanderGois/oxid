import { Vector2D } from "oxid/math";
import { CANVAS_HEIGHT, CANVAS_WIDTH } from "./config";

export function randomRange(min, max) {
    return min + Math.random() * (max - min);
}

export function randomInt(min, max) {
    return Math.floor(randomRange(min, max + 1));
}

export function randomDirection() {
    const angle = randomRange(0, Math.PI * 2);

    return new Vector2D(
        Math.cos(angle),
        Math.sin(angle)
    );
}

export function distanceSquared(a, b) {
    const dx = a.x - b.x;
    const dy = a.y - b.y;

    return dx * dx + dy * dy;
}

export function wrapAround(position) {
    if (position.x < 0)
        position.x = CANVAS_WIDTH;

    if (position.x > CANVAS_WIDTH)
        position.x = 0;

    if (position.y < 0)
        position.y = CANVAS_HEIGHT;

    if (position.y > CANVAS_HEIGHT)
        position.y = 0;
}

export function isOutsideCanvas(position, margin = 0) {
    return (
        position.x < -margin ||
        position.x > CANVAS_WIDTH + margin ||
        position.y < -margin ||
        position.y > CANVAS_HEIGHT + margin
    );
}