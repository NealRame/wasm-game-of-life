import {
    Cell,
    Universe,
} from "wasm-game-of-life";

import {
    memory,
} from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

function createController(tickCount) {
    const universe = Universe.new();
    const width = universe.width();
    const height = universe.height();

    const canvas = document.getElementById("game-of-life-canvas");
    const ctx = canvas.getContext("2d");

    canvas.width = (CELL_SIZE + 1) * width + 1;
    canvas.height = (CELL_SIZE + 1) * height + 1;

    let tick = 0;
    let frameId = 0;
    let running = false;

    function drawGrid() {
        ctx.beginPath();
        ctx.strokeStyle = GRID_COLOR;
    
        // Vertical lines.
        for (let i = 0; i <= width; i++) {
            ctx.moveTo(i*(CELL_SIZE + 1) + 1, 0);
            ctx.lineTo(i*(CELL_SIZE + 1) + 1, (CELL_SIZE + 1)*height + 1);
        }
    
        // Horizontal lines.
        for (let j = 0; j <= height; j++) {
            ctx.moveTo(0,                         j*(CELL_SIZE + 1) + 1);
            ctx.lineTo((CELL_SIZE + 1)*width + 1, j*(CELL_SIZE + 1) + 1);
        }
    
        ctx.stroke();
    }
    
    function getIndex(row, col) {
        return row*width + col;
    }
    
    function drawCells() {
        const cellsPtr = universe.cells();
        const cells = new Uint8Array(memory.buffer, cellsPtr, width*height);
    
        ctx.beginPath();
    
        for (let row = 0; row < height; row++) {
            for (let col = 0; col < width; col++) {
                const idx = getIndex(row, col);
        
                ctx.fillStyle = cells[idx] === Cell.Dead
                    ? DEAD_COLOR
                    : ALIVE_COLOR;
        
                ctx.fillRect(
                    col*(CELL_SIZE + 1) + 1,
                    row*(CELL_SIZE + 1) + 1,
                    CELL_SIZE,
                    CELL_SIZE
                );
            }
        }
    
        ctx.stroke();
    }

    function render() {
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
        if (running) return ;

        const boundingRect = canvas.getBoundingClientRect();
        const scaleX = canvas.width/boundingRect.width;
        const scaleY = canvas.height/boundingRect.height;
        const canvasLeft = (event.clientX - boundingRect.left)*scaleX;
        const canvasTop = (event.clientY - boundingRect.top)*scaleY;

        const row = Math.min(Math.floor(canvasTop/(CELL_SIZE + 1)), height - 1);
        const col = Math.min(Math.floor(canvasLeft/(CELL_SIZE + 1)), width - 1);

        universe.toggle_cell(row, col);
        render();
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
        clear() {
            universe.clear();
            render();
        },
        get running() {
            return running;
        }
    }
}


const controller = createController(5);

const playPauseButton = document.getElementById("play-pause");
const stepButton = document.getElementById("step");
const clearButton = document.getElementById("clear");

playPauseButton.addEventListener("click", event => {
    if (controller.running) {
        controller.stop();
        playPauseButton.classList.remove("play");
        playPauseButton.classList.add("pause");
        clearButton.disabled = false;
        stepButton.disabled = false;
    } else {
        controller.start();
        playPauseButton.classList.remove("pause");
        playPauseButton.classList.add("play");
        clearButton.disabled = true;
        stepButton.disabled = true;
    }
});

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