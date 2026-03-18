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
  /**
   * Base class for applications and gameplay objects.
   * The runtime currently instantiates whatever is returned by `main()`.
   */
  export class GameObject {
    onInit?(): void;
    onUpdate?(dt: number): void;
    onDraw?(): void;
  }
}

declare module "oxid/math" {
  /** A simple 2D vector/position container. */
  export class Transform2D {
    constructor(x: number, y: number);

    x: number;
    y: number;
  }
}

declare module "oxid/color" {
  /** RGBA color with normalized channels (commonly 0.0 to 1.0). */
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

  /** Draws an arc on screen. */
  export function drawArc(
    position: Transform2D,
    sides: number,
    radius: number,
    rotation: number,
    thickness: number,
    arc: number,
    color: Color,
  ): void;

  /** Draws a circle on screen. */
  export function drawCircle(
    x: number,
    y: number,
    radius: number,
    color: Color,
  ): void;

  /** Draws a rectangle on screen. */
  export function drawRectangle(
    x: number,
    y: number,
    width: number,
    height: number,
    color: Color,
  ): void;
}

declare module "oxid/input" {
  import type { Transform2D } from "oxid/math";

  /**
   * Returns true while the key remains pressed.
   * Accepts names like "A", "ArrowLeft", "Space", "Enter", "Escape", "LeftShift", "F1".
   */
  export function isKeyDown(key: string): boolean;

  /** Returns true only on the frame the key was pressed. */
  export function isKeyPressed(key: string): boolean;

  /** Returns true only on the frame the key was released. */
  export function isKeyReleased(key: string): boolean;

  /** Returns the current mouse position in screen coordinates. */
  export function mousePosition(): Transform2D;

  /**
   * Returns true while the mouse button remains pressed.
   * Accepted values are currently "left", "middle", and "right".
   */
  export function isMouseButtonDown(button: string): boolean;

  /** Returns true only on the frame the mouse button was pressed. */
  export function isMouseButtonPressed(button: string): boolean;

  /** Returns true only on the frame the mouse button was released. */
  export function isMouseButtonReleased(button: string): boolean;
}

declare module "oxid/text" {
  import type { Color } from "oxid/color";
  import type { Transform2D } from "oxid/math";

  /** Layout metrics for a single-line text measurement. */
  export class TextMetrics {
    constructor(width: number, height: number, offsetY: number);

    readonly width: number;
    readonly height: number;
    readonly offset_y: number;
  }

  /**
   * Draws single-line 2D text.
   * The y coordinate represents the text baseline.
   */
  export function drawText(
    text: string,
    position: Transform2D,
    fontSize: number,
    color: Color,
  ): void;

  /**
   * Draws multiline text using `\n` as separator.
   * The y coordinate represents the baseline of the first line.
   */
  export function drawMultilineText(
    text: string,
    position: Transform2D,
    fontSize: number,
    color: Color,
    lineDistance?: number,
  ): void;

  /** Measures a single-line string using the default font. */
  export function measureText(text: string, fontSize: number): TextMetrics;
}

declare module "oxid/texture" {
  import type { Transform2D } from "oxid/math";

  /** A loaded texture resource. */
  export class Texture2D {
    readonly path: string;
    readonly width: number;
    readonly height: number;
  }

  /** Loads a texture from disk and returns a reusable Texture2D object. */
  export function loadTexture(path: string): Texture2D;

  /** Draws a texture using its original size. */
  export function drawTexture(texture: Texture2D, position: Transform2D): void;

  /**
   * Draws a scaled texture.
   * Rotation is in radians and defaults to 0.
   */
  export function drawTextureScaled(
    texture: Texture2D,
    position: Transform2D,
    size: Transform2D,
    rotation?: number,
  ): void;
}