/**
 * Type definitions for the current Oxid scripting runtime.
 *
 * These declarations mirror the modules currently registered in Rust:
 * - oxid/core
 * - oxid/math
 * - oxid/color
 * - oxid/shapes
 * - oxid/input
 * - oxid/text
 * - oxid/texture
 *
 * Notes:
 * - The current runtime exposes subpath modules, not a single "oxid" barrel.
 * - Rotation for drawTextureScaled is in radians.
 * - drawArc rotation/arc follow the current runtime documentation.
 */

declare module "oxid/core" {
  export class GameObject {
    onInit?(): void;
    onUpdate?(dt: number): void;
    onDraw?(): void;
  }
}

declare module "oxid/math" {
  export class Transform2D {
    constructor(x: number, y: number);
    x: number;
    y: number;
  }
}

declare module "oxid/color" {
  export class Color {
    constructor(r: number, g: number, b: number, a: number);
    r: number;
    g: number;
    b: number;
    a: number;
  }
}

declare module "oxid/shapes" {
  import type { Color } from "oxid/color";
  import type { Transform2D } from "oxid/math";

  export function drawArc(
    position: Transform2D,
    sides: number,
    radius: number,
    rotation: number,
    thickness: number,
    arc: number,
    color: Color,
  ): void;

  export function drawCircle(x: number, y: number, radius: number, color: Color): void;
  export function drawRectangle(x: number, y: number, width: number, height: number, color: Color): void;
}

declare module "oxid/input" {
  import type { Transform2D } from "oxid/math";

  export function isKeyDown(key: string): boolean;
  export function isKeyPressed(key: string): boolean;
  export function isKeyReleased(key: string): boolean;
  export function mousePosition(): Transform2D;
  export function isMouseButtonDown(button: string): boolean;
  export function isMouseButtonPressed(button: string): boolean;
  export function isMouseButtonReleased(button: string): boolean;
}

declare module "oxid/text" {
  import type { Color } from "oxid/color";
  import type { Transform2D } from "oxid/math";

  export class TextMetrics {
    constructor(width: number, height: number, offsetY: number);
    readonly width: number;
    readonly height: number;
    readonly offset_y: number;
  }

  export function drawText(text: string, position: Transform2D, fontSize: number, color: Color): void;
  export function drawMultilineText(text: string, position: Transform2D, fontSize: number, color: Color, lineDistance?: number): void;
  export function measureText(text: string, fontSize: number): TextMetrics;
}

declare module "oxid/texture" {
  import type { Transform2D } from "oxid/math";

  export class Texture2D {
    readonly path: string;
    readonly width: number;
    readonly height: number;
  }

  export function loadTexture(path: string): Texture2D;
  export function drawTexture(texture: Texture2D, position: Transform2D): void;
  export function drawTextureScaled(texture: Texture2D, position: Transform2D, size: Transform2D, rotation?: number): void;
}
