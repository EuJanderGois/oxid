import { Entity } from "oxid/core";
import { Vector2D } from "oxid/math";
import { Color } from "oxid/color";

import {
    drawRectangle,
    drawLine,
} from "oxid/shapes";

import {
    drawText,
} from "oxid/text";

import {
    isKeyDown,
    isKeyPressed,
} from "oxid/input";


const CANVAS_WIDTH = 800;
const CANVAS_HEIGHT = 600;

const BOARD_WIDTH = 10;
const BOARD_HEIGHT = 20;

const CELL_SIZE = 24;

const BOARD_PIXEL_WIDTH =
    BOARD_WIDTH * CELL_SIZE;

const BOARD_PIXEL_HEIGHT =
    BOARD_HEIGHT * CELL_SIZE;

const BOARD_X =
    Math.floor(
        (CANVAS_WIDTH - BOARD_PIXEL_WIDTH) / 2
    );

const BOARD_Y =
    Math.floor(
        (CANVAS_HEIGHT - BOARD_PIXEL_HEIGHT) / 2
    );

const DROP_INTERVAL = 0.55;
const SOFT_DROP_INTERVAL = 0.045;

const COLORS = {
    background: new Color(0, 0, 0, 1),
    phosphor: new Color(0.2, 1, 0.35, 1),
};


// ---------------------------------------------------------
// Tetromino definitions
// ---------------------------------------------------------

const PIECES = [
    {
        name: "I",
        blocks: [
            [0, 1],
            [1, 1],
            [2, 1],
            [3, 1],
        ],
    },

    {
        name: "O",
        blocks: [
            [0, 0],
            [1, 0],
            [0, 1],
            [1, 1],
        ],
    },

    {
        name: "T",
        blocks: [
            [1, 0],
            [0, 1],
            [1, 1],
            [2, 1],
        ],
    },

    {
        name: "S",
        blocks: [
            [1, 0],
            [2, 0],
            [0, 1],
            [1, 1],
        ],
    },

    {
        name: "Z",
        blocks: [
            [0, 0],
            [1, 0],
            [1, 1],
            [2, 1],
        ],
    },

    {
        name: "J",
        blocks: [
            [0, 0],
            [0, 1],
            [1, 1],
            [2, 1],
        ],
    },

    {
        name: "L",
        blocks: [
            [2, 0],
            [0, 1],
            [1, 1],
            [2, 1],
        ],
    },
];


// ---------------------------------------------------------
// Utilities
// ---------------------------------------------------------

function randomInt(min, max) {
    return Math.floor(
        min + Math.random() * (max - min + 1)
    );
}

function createBoard() {
    const board = [];

    for (let y = 0; y < BOARD_HEIGHT; y++) {
        const row = [];

        for (let x = 0; x < BOARD_WIDTH; x++) {
            row.push(0);
        }

        board.push(row);
    }

    return board;
}

function copyBlocks(blocks) {
    return blocks.map(
        block => [block[0], block[1]]
    );
}


// ---------------------------------------------------------
// Piece
// ---------------------------------------------------------

class Piece {
    type;
    blocks;

    x;
    y;

    rotation;

    constructor(typeIndex) {
        this.type = PIECES[typeIndex];
        this.blocks = copyBlocks(
            this.type.blocks
        );

        this.x =
            Math.floor(
                BOARD_WIDTH / 2
            ) - 2;

        this.y = 0;

        this.rotation = 0;
    }

    rotate() {
        const rotated = [];

        for (const block of this.blocks) {
            const x = block[0];
            const y = block[1];

            rotated.push([
                -y,
                x,
            ]);
        }

        // Normalize negative coordinates.
        let minX = Infinity;
        let minY = Infinity;

        for (const block of rotated) {
            minX = Math.min(
                minX,
                block[0]
            );

            minY = Math.min(
                minY,
                block[1]
            );
        }

        for (const block of rotated) {
            block[0] -= minX;
            block[1] -= minY;
        }

        return rotated;
    }

    applyRotation(blocks) {
        this.blocks = blocks;

        this.rotation =
            (this.rotation + 1) % 4;
    }
}


// ---------------------------------------------------------
// Game
// ---------------------------------------------------------

export class MyApp extends Entity {
    board;

    currentPiece;
    nextPiece;

    score = 0;
    lines = 0;
    level = 1;

    dropTimer = 0;

    gameOver = false;

    leftWasPressed = false;
    rightWasPressed = false;

    constructor() {
        super();

        this.restart();
    }

    onInit() {
    }

    onUpdate(dt) {
        if (this.gameOver) {
            if (isKeyPressed("Enter")) {
                this.restart();
            }

            return;
        }

        this.handleInput();

        this.dropTimer += dt;

        const interval =
            isKeyDown("ArrowDown") ||
            isKeyDown("S")
                ? SOFT_DROP_INTERVAL
                : this.getDropInterval();

        if (this.dropTimer >= interval) {
            this.dropTimer = 0;

            this.moveDown();
        }
    }

    onDraw() {
        this.drawBoard();
        this.drawPiece();
        this.drawGhostPiece();
        this.drawHud();

        if (this.gameOver) {
            this.drawGameOver();
        }
    }


    // -----------------------------------------------------
    // Input
    // -----------------------------------------------------

    handleInput() {
        if (
            isKeyPressed("ArrowLeft") ||
            isKeyPressed("A")
        ) {
            this.moveHorizontal(-1);
        }

        if (
            isKeyPressed("ArrowRight") ||
            isKeyPressed("D")
        ) {
            this.moveHorizontal(1);
        }

        if (
            isKeyPressed("ArrowUp") ||
            isKeyPressed("W")
        ) {
            this.rotatePiece();
        }

        if (
            isKeyPressed("Space")
        ) {
            this.hardDrop();
        }
    }


    // -----------------------------------------------------
    // Movement
    // -----------------------------------------------------

    moveHorizontal(direction) {
        if (
            this.canMove(
                this.currentPiece,
                direction,
                0,
                this.currentPiece.blocks
            )
        ) {
            this.currentPiece.x +=
                direction;
        }
    }

    moveDown() {
        if (
            this.canMove(
                this.currentPiece,
                0,
                1,
                this.currentPiece.blocks
            )
        ) {
            this.currentPiece.y++;

            return;
        }

        this.lockPiece();

        this.clearLines();

        this.spawnPiece();
    }

    hardDrop() {
        while (
            this.canMove(
                this.currentPiece,
                0,
                1,
                this.currentPiece.blocks
            )
        ) {
            this.currentPiece.y++;

            this.score += 2;
        }

        this.lockPiece();

        this.clearLines();

        this.spawnPiece();

        this.dropTimer = 0;
    }


    // -----------------------------------------------------
    // Rotation
    // -----------------------------------------------------

    rotatePiece() {
        const rotated =
            this.currentPiece.rotate();

        if (
            this.canMove(
                this.currentPiece,
                0,
                0,
                rotated
            )
        ) {
            this.currentPiece.applyRotation(
                rotated
            );

            return;
        }

        // Simple wall kicks.
        if (
            this.canMove(
                this.currentPiece,
                -1,
                0,
                rotated
            )
        ) {
            this.currentPiece.x--;

            this.currentPiece.applyRotation(
                rotated
            );

            return;
        }

        if (
            this.canMove(
                this.currentPiece,
                1,
                0,
                rotated
            )
        ) {
            this.currentPiece.x++;

            this.currentPiece.applyRotation(
                rotated
            );
        }
    }


    // -----------------------------------------------------
    // Collision
    // -----------------------------------------------------

    canMove(
        piece,
        offsetX,
        offsetY,
        blocks
    ) {
        for (const block of blocks) {
            const x =
                piece.x +
                block[0] +
                offsetX;

            const y =
                piece.y +
                block[1] +
                offsetY;

            if (
                x < 0 ||
                x >= BOARD_WIDTH
            ) {
                return false;
            }

            if (
                y >= BOARD_HEIGHT
            ) {
                return false;
            }

            if (
                y >= 0 &&
                this.board[y][x] !== 0
            ) {
                return false;
            }
        }

        return true;
    }


    // -----------------------------------------------------
    // Piece management
    // -----------------------------------------------------

    spawnPiece() {
        const typeIndex =
            randomInt(
                0,
                PIECES.length - 1
            );

        this.currentPiece =
            new Piece(typeIndex);

        this.currentPiece.x =
            Math.floor(
                (BOARD_WIDTH -
                    this.getPieceWidth(
                        this.currentPiece.blocks
                    )) / 2
            );

        this.currentPiece.y = 0;

        if (
            !this.canMove(
                this.currentPiece,
                0,
                0,
                this.currentPiece.blocks
            )
        ) {
            this.gameOver = true;
        }
    }

    getPieceWidth(blocks) {
        let maxX = 0;

        for (const block of blocks) {
            maxX = Math.max(
                maxX,
                block[0]
            );
        }

        return maxX + 1;
    }

    lockPiece() {
        for (
            const block
            of this.currentPiece.blocks
        ) {
            const x =
                this.currentPiece.x +
                block[0];

            const y =
                this.currentPiece.y +
                block[1];

            if (
                y >= 0 &&
                y < BOARD_HEIGHT &&
                x >= 0 &&
                x < BOARD_WIDTH
            ) {
                this.board[y][x] = 1;
            }
        }
    }


    // -----------------------------------------------------
    // Lines
    // -----------------------------------------------------

    clearLines() {
        let cleared = 0;

        for (
            let y = BOARD_HEIGHT - 1;
            y >= 0;
            y--
        ) {
            let complete = true;

            for (
                let x = 0;
                x < BOARD_WIDTH;
                x++
            ) {
                if (
                    this.board[y][x] === 0
                ) {
                    complete = false;
                    break;
                }
            }

            if (complete) {
                this.board.splice(y, 1);

                this.board.unshift(
                    new Array(
                        BOARD_WIDTH
                    ).fill(0)
                );

                cleared++;

                y++;
            }
        }

        if (cleared === 0) {
            return;
        }

        this.lines += cleared;

        switch (cleared) {
            case 1:
                this.score += 100 * this.level;
                break;

            case 2:
                this.score += 300 * this.level;
                break;

            case 3:
                this.score += 500 * this.level;
                break;

            case 4:
                this.score += 800 * this.level;
                break;
        }

        this.level =
            Math.floor(
                this.lines / 10
            ) + 1;
    }


    // -----------------------------------------------------
    // Ghost piece
    // -----------------------------------------------------

    getGhostY() {
        let y =
            this.currentPiece.y;

        while (
            this.canMove(
                {
                    x: this.currentPiece.x,
                    y: y,
                },
                0,
                1,
                this.currentPiece.blocks
            )
        ) {
            y++;
        }

        return y;
    }


    // -----------------------------------------------------
    // Drawing
    // -----------------------------------------------------

    drawBoard() {
        // Outer arcade frame.
        drawLine(
            new Vector2D(
                BOARD_X - 4,
                BOARD_Y - 4
            ),
            new Vector2D(
                BOARD_X +
                    BOARD_PIXEL_WIDTH +
                    4,
                BOARD_Y - 4
            ),
            2,
            COLORS.phosphor
        );

        drawLine(
            new Vector2D(
                BOARD_X - 4,
                BOARD_Y +
                    BOARD_PIXEL_HEIGHT +
                    4
            ),
            new Vector2D(
                BOARD_X +
                    BOARD_PIXEL_WIDTH +
                    4,
                BOARD_Y +
                    BOARD_PIXEL_HEIGHT +
                    4
            ),
            2,
            COLORS.phosphor
        );

        drawLine(
            new Vector2D(
                BOARD_X - 4,
                BOARD_Y - 4
            ),
            new Vector2D(
                BOARD_X - 4,
                BOARD_Y +
                    BOARD_PIXEL_HEIGHT +
                    4
            ),
            2,
            COLORS.phosphor
        );

        drawLine(
            new Vector2D(
                BOARD_X +
                    BOARD_PIXEL_WIDTH +
                    4,
                BOARD_Y - 4
            ),
            new Vector2D(
                BOARD_X +
                    BOARD_PIXEL_WIDTH +
                    4,
                BOARD_Y +
                    BOARD_PIXEL_HEIGHT +
                    4
            ),
            2,
            COLORS.phosphor
        );

        for (
            let y = 0;
            y < BOARD_HEIGHT;
            y++
        ) {
            for (
                let x = 0;
                x < BOARD_WIDTH;
                x++
            ) {
                if (
                    this.board[y][x] === 0
                ) {
                    continue;
                }

                this.drawCell(
                    x,
                    y
                );
            }
        }
    }

    drawCell(x, y) {
        const pixelX =
            BOARD_X +
            x * CELL_SIZE;

        const pixelY =
            BOARD_Y +
            y * CELL_SIZE;

        drawRectangle(
            pixelX + 1,
            pixelY + 1,
            CELL_SIZE - 2,
            CELL_SIZE - 2,
            COLORS.phosphor
        );
    }

    drawPiece() {
        for (
            const block
            of this.currentPiece.blocks
        ) {
            const x =
                this.currentPiece.x +
                block[0];

            const y =
                this.currentPiece.y +
                block[1];

            if (y < 0) {
                continue;
            }

            this.drawCell(
                x,
                y
            );
        }
    }

    drawGhostPiece() {
        const ghostY =
            this.getGhostY();

        // We intentionally do not use a
        // third color. The ghost is represented
        // by an outline using the same phosphor.
        for (
            const block
            of this.currentPiece.blocks
        ) {
            const x =
                BOARD_X +
                (
                    this.currentPiece.x +
                    block[0]
                ) * CELL_SIZE;

            const y =
                BOARD_Y +
                (
                    ghostY +
                    block[1]
                ) * CELL_SIZE;

            drawLine(
                new Vector2D(
                    x + 2,
                    y + 2
                ),
                new Vector2D(
                    x + CELL_SIZE - 2,
                    y + 2
                ),
                1,
                COLORS.phosphor
            );

            drawLine(
                new Vector2D(
                    x + 2,
                    y + CELL_SIZE - 2
                ),
                new Vector2D(
                    x + CELL_SIZE - 2,
                    y + CELL_SIZE - 2
                ),
                1,
                COLORS.phosphor
            );

            drawLine(
                new Vector2D(
                    x + 2,
                    y + 2
                ),
                new Vector2D(
                    x + 2,
                    y + CELL_SIZE - 2
                ),
                1,
                COLORS.phosphor
            );

            drawLine(
                new Vector2D(
                    x + CELL_SIZE - 2,
                    y + 2
                ),
                new Vector2D(
                    x + CELL_SIZE - 2,
                    y + CELL_SIZE - 2
                ),
                1,
                COLORS.phosphor
            );
        }
    }


    // -----------------------------------------------------
    // HUD
    // -----------------------------------------------------

    drawHud() {
        const leftX =
            BOARD_X -
            150;

        drawText(
            "TETRIS",
            new Vector2D(
                leftX,
                BOARD_Y + 25
            ),
            24,
            COLORS.phosphor
        );

        drawText(
            "SCORE",
            new Vector2D(
                leftX,
                BOARD_Y + 75
            ),
            14,
            COLORS.phosphor
        );

        drawText(
            String(this.score),
            new Vector2D(
                leftX,
                BOARD_Y + 100
            ),
            20,
            COLORS.phosphor
        );

        drawText(
            "LINES",
            new Vector2D(
                leftX,
                BOARD_Y + 145
            ),
            14,
            COLORS.phosphor
        );

        drawText(
            String(this.lines),
            new Vector2D(
                leftX,
                BOARD_Y + 170
            ),
            20,
            COLORS.phosphor
        );

        drawText(
            "LEVEL",
            new Vector2D(
                leftX,
                BOARD_Y + 215
            ),
            14,
            COLORS.phosphor
        );

        drawText(
            String(this.level),
            new Vector2D(
                leftX,
                BOARD_Y + 240
            ),
            20,
            COLORS.phosphor
        );

        const controlsX =
            BOARD_X +
            BOARD_PIXEL_WIDTH +
            30;

        drawText(
            "CONTROLS",
            new Vector2D(
                controlsX,
                BOARD_Y + 25
            ),
            14,
            COLORS.phosphor
        );

        drawText(
            "A / D",
            new Vector2D(
                controlsX,
                BOARD_Y + 55
            ),
            14,
            COLORS.phosphor
        );

        drawText(
            "MOVE",
            new Vector2D(
                controlsX,
                BOARD_Y + 75
            ),
            12,
            COLORS.phosphor
        );

        drawText(
            "W",
            new Vector2D(
                controlsX,
                BOARD_Y + 110
            ),
            14,
            COLORS.phosphor
        );

        drawText(
            "ROTATE",
            new Vector2D(
                controlsX,
                BOARD_Y + 130
            ),
            12,
            COLORS.phosphor
        );

        drawText(
            "S",
            new Vector2D(
                controlsX,
                BOARD_Y + 165
            ),
            14,
            COLORS.phosphor
        );

        drawText(
            "DROP",
            new Vector2D(
                controlsX,
                BOARD_Y + 185
            ),
            12,
            COLORS.phosphor
        );

        drawText(
            "SPACE",
            new Vector2D(
                controlsX,
                BOARD_Y + 220
            ),
            14,
            COLORS.phosphor
        );

        drawText(
            "HARD DROP",
            new Vector2D(
                controlsX,
                BOARD_Y + 240
            ),
            12,
            COLORS.phosphor
        );
    }

    drawGameOver() {
        const centerX =
            BOARD_X +
            BOARD_PIXEL_WIDTH / 2;

        const centerY =
            BOARD_Y +
            BOARD_PIXEL_HEIGHT / 2;

        drawRectangle(
            centerX - 125,
            centerY - 55,
            250,
            110,
            COLORS.background
        );

        drawLine(
            new Vector2D(
                centerX - 125,
                centerY - 55
            ),
            new Vector2D(
                centerX + 125,
                centerY - 55
            ),
            2,
            COLORS.phosphor
        );

        drawLine(
            new Vector2D(
                centerX - 125,
                centerY + 55
            ),
            new Vector2D(
                centerX + 125,
                centerY + 55
            ),
            2,
            COLORS.phosphor
        );

        drawLine(
            new Vector2D(
                centerX - 125,
                centerY - 55
            ),
            new Vector2D(
                centerX - 125,
                centerY + 55
            ),
            2,
            COLORS.phosphor
        );

        drawLine(
            new Vector2D(
                centerX + 125,
                centerY - 55
            ),
            new Vector2D(
                centerX + 125,
                centerY + 55
            ),
            2,
            COLORS.phosphor
        );

        drawText(
            "GAME OVER",
            new Vector2D(
                centerX - 78,
                centerY - 10
            ),
            24,
            COLORS.phosphor
        );

        drawText(
            "ENTER: RESTART",
            new Vector2D(
                centerX - 82,
                centerY + 25
            ),
            14,
            COLORS.phosphor
        );
    }


    // -----------------------------------------------------
    // Game state
    // -----------------------------------------------------

    getDropInterval() {
        return Math.max(
            0.08,
            DROP_INTERVAL -
                (this.level - 1) *
                0.04
        );
    }

    restart() {
        this.board = createBoard();

        this.score = 0;
        this.lines = 0;
        this.level = 1;

        this.dropTimer = 0;

        this.gameOver = false;

        this.spawnPiece();
    }
}


export function main() {
    return new MyApp();
}