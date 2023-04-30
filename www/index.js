import {
    Cell,
    Universe,
} from "wasm-game-of-life";

import {
    memory,
} from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

function createController(tickCount) {
    let universe = Universe.new(32, 32);

    universe.randomize();

    const canvas = document.getElementById("game-of-life-canvas");
    const ctx = canvas.getContext("2d");

    let tick = 0;
    let frameId = 0;
    let running = false;

    function drawGrid() {
        ctx.beginPath();
        ctx.strokeStyle = GRID_COLOR;
    
        // Vertical lines.
        for (let i = 0; i <= universe.width(); i++) {
            ctx.moveTo(i*(CELL_SIZE + 1) + 1, 0);
            ctx.lineTo(i*(CELL_SIZE + 1) + 1, (CELL_SIZE + 1)*universe.height() + 1);
        }
    
        // Horizontal lines.
        for (let j = 0; j <= universe.height(); j++) {
            ctx.moveTo(0,                         j*(CELL_SIZE + 1) + 1);
            ctx.lineTo((CELL_SIZE + 1)*universe.width() + 1, j*(CELL_SIZE + 1) + 1);
        }
    
        ctx.stroke();
    }
    
    function drawCells() {
        universe.render_to_context(ctx, {
            cellSize: CELL_SIZE,
            aliveCell: ALIVE_COLOR,
            deadCell: DEAD_COLOR,
        });
    }

    function render() {
        canvas.width = (CELL_SIZE + 1) * universe.width() + 1;
        canvas.height = (CELL_SIZE + 1) * universe.height() + 1;
        drawGrid();
        drawCells();
    }

    function renderLoop() {
        if (tick == 0) {
            universe.tick();
        }
        tick = (tick + 1)%tickCount;
        render();
        frameId = requestAnimationFrame(renderLoop);
    };

    canvas.addEventListener("click", event => {
        const boundingRect = canvas.getBoundingClientRect();
        const scaleX = canvas.width/boundingRect.width;
        const scaleY = canvas.height/boundingRect.height;
        const canvasLeft = (event.clientX - boundingRect.left)*scaleX;
        const canvasTop = (event.clientY - boundingRect.top)*scaleY;

        const row = Math.min(Math.floor(canvasTop/(CELL_SIZE + 1)), universe.height() - 1);
        const col = Math.min(Math.floor(canvasLeft/(CELL_SIZE + 1)), universe.width() - 1);

        if (event.altKey) {
            universe.set_cells([
                [row + 1, col - 1],
                [row - 1, col    ],
                [row + 1, col    ],
                [row    , col + 1],
                [row + 1, col + 1],
            ], Cell.Alive)
        } else if (event.shiftKey) {
            universe.set_cells([
                [row - 2, col - 1], [row - 3, col - 1], [row - 4, col - 1],
                [row - 1, col - 2], [row - 6, col - 2],
                [row - 1, col - 3], [row - 6, col - 3],
                [row - 1, col - 4], [row - 6, col - 4],
                [row - 2, col - 6], [row - 3, col - 6], [row - 4, col - 6],
                [row + 2, col - 1], [row + 3, col - 1], [row + 4, col - 1],
                [row + 1, col - 2], [row + 6, col - 2],
                [row + 1, col - 3], [row + 6, col - 3],
                [row + 1, col - 4], [row + 6, col - 4],
                [row + 2, col - 6], [row + 3, col - 6], [row + 4, col - 6],
                [row - 2, col + 1], [row - 3, col + 1], [row - 4, col + 1],
                [row - 1, col + 2], [row - 6, col + 2],
                [row - 1, col + 3], [row - 6, col + 3],
                [row - 1, col + 4], [row - 6, col + 4],
                [row - 2, col + 6], [row - 3, col + 6], [row - 4, col + 6],
                [row + 2, col + 1], [row + 3, col + 1], [row + 4, col + 1],
                [row + 1, col + 2], [row + 6, col + 2],
                [row + 1, col + 3], [row + 6, col + 3],
                [row + 1, col + 4], [row + 6, col + 4],
                [row + 2, col + 6], [row + 3, col + 6], [row + 4, col + 6],
            ], Cell.Alive)

        } else {
            universe.toggle_cell(row, col);
        }

        if (!running) {
            render();
        }
    });

    render();

    return {
        start() {
            if (!running) {
                running = true;
                frameId = requestAnimationFrame(renderLoop);
            }
        },
        stop() {
            if (running) {
                cancelAnimationFrame(frameId);
                running = false;
            }
        },
        step() {
            if (!running) {
                universe.tick();
                render();
            }
        },
        randomize() {
            if (!running) {
                universe.randomize();
                render();
            }
        },
        reset(s) {
            if (!running) {
                universe.free();
                universe = Universe.from_rle(s);
                render();
            }
        },
        clear() {
            universe.clear();
            render();
        },
        export() {
            return universe.to_rle();
        },
        get running() {
            return running;
        }
    }
}

const controller = createController(5);

const playPauseButton = document.querySelector("#play-pause");
const stepButton = document.querySelector("#step");
const randomizeButton = document.querySelector("#randomize");
const clearButton = document.querySelector("#clear");

const exportButton = document.querySelector("#export-button");
const importButton = document.querySelector("#import-button");
const ioBuffer = document.querySelector("#io > textarea");

playPauseButton.addEventListener("click", event => {
    if (controller.running) {
        controller.stop();
        playPauseButton.classList.remove("play");
        playPauseButton.classList.add("pause");
        clearButton.disabled = false;
        stepButton.disabled = false;
        exportButton.disabled = false;
    } else {
        controller.start();
        playPauseButton.classList.remove("pause");
        playPauseButton.classList.add("play");
        clearButton.disabled = true;
        stepButton.disabled = true;
        exportButton.disabled = true;
    }
});

randomizeButton.addEventListener("click", event => {
    if (!controller.running) {
        controller.randomize();
    }
})

clearButton.addEventListener("click", event => {
    if (!controller.running) {
        controller.clear();
    }
})

stepButton.addEventListener("click", event => {
    if (!controller.running) {
        controller.step();
    }
})

exportButton.addEventListener("click", event => {
    if (!controller.running) {
        const text = controller.export();
        ioBuffer.value = text;
        // const blob = new Blob([text], {type: "text/plain;charset=utf-8"});
        // saveAs(blob, "game-of-life.rle");
    }
})

importButton.addEventListener("click", event => {
    if (!controller.running) {
        const text = ioBuffer.value;
        controller.reset(text);
        // const blob = new Blob([text], {type: "text/plain;charset=utf-8"});
        // saveAs(blob, "game-of-life.rle");
    }
})