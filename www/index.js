import {
    Cell,
    Universe,
} from "wasm-game-of-life";


const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const playPauseButton = document.querySelector("#play-pause");
const stepButton = document.querySelector("#step");
const randomizeButton = document.querySelector("#randomize");
const clearButton = document.querySelector("#clear");

const moveLeftButton = document.querySelector("#move-left");
const moveRightButton = document.querySelector("#move-right");
const moveUpButton = document.querySelector("#move-up");
const moveDownButton = document.querySelector("#move-down");

const mouseRowInput = document.querySelector("#position > #mouse-row");
const mouseColInput = document.querySelector("#position > #mouse-col");

const sizeWidthInput = document.querySelector("#size > #width");
const sizeHeightInput = document.querySelector("#size > #height");

const exportButton = document.querySelector("#export-button");
const importButton = document.querySelector("#import-button");
const ioBuffer = document.querySelector("#io > textarea");

function throttle(fn, wait) {
    let inThrottle = false;
    return function(...args) {
        if (!inThrottle) {
            fn.apply(this, args);
            inThrottle = true;
            setTimeout(() => inThrottle = false, wait);
        }
    }
}

function createController(tickCount) {
    let universe = Universe.new(32, 32);

    universe.randomize();

    const canvas = document.getElementById("game-of-life-canvas");
    const ctx = canvas.getContext("2d");

    let tick = 0;
    let frameId = 0;
    let running = false;

    let mouseHover = false;
    let mouseRow = 0;
    let mouseCol = 0;

    function drawCells() {
        universe.render_to_context(ctx, {
            cellSize: CELL_SIZE,
            aliveCell: ALIVE_COLOR,
            deadCell: DEAD_COLOR,
        });
    }

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

    function drawCursor() {
        ctx.beginPath();
        ctx.fillStyle = "rgba(230, 64, 60, 0.25)";
        ctx.fillRect(
            mouseCol*(CELL_SIZE + 1) + 1,
            0,
            CELL_SIZE + 1,
            (CELL_SIZE + 1)*universe.height() + 1,
        );
        ctx.fillRect(
            0,
            mouseRow*(CELL_SIZE + 1) + 1,
            (CELL_SIZE + 1)*universe.width() + 1,
            CELL_SIZE + 1,
        );
    }

    function render() {
        canvas.width = (CELL_SIZE + 1) * universe.width() + 1;
        canvas.height = (CELL_SIZE + 1) * universe.height() + 1;
        drawGrid();
        drawCells();
        if (mouseHover && !running) {
            drawCursor();
        }
    }

    function renderLoop() {
        if (tick == 0) {
            universe.tick();
        }
        tick = (tick + 1)%tickCount;
        render();
        frameId = requestAnimationFrame(renderLoop);
    };

    const mousePosition = () => {
        const boundingRect = canvas.getBoundingClientRect();
        const scaleX = canvas.width/boundingRect.width;
        const scaleY = canvas.height/boundingRect.height;
        const canvasLeft = (event.clientX - boundingRect.left)*scaleX;
        const canvasTop = (event.clientY - boundingRect.top)*scaleY;

        return [
            /* col = */ Math.min(Math.floor(canvasLeft/(CELL_SIZE + 1)), universe.width() - 1),
            /* row = */ Math.min(Math.floor(canvasTop/(CELL_SIZE + 1)), universe.height() - 1),
        ]
    }

    canvas.addEventListener("mouseenter", event => {
        mouseHover = true;
        render();
    })

    canvas.addEventListener("mouseleave", event => {
        mouseHover = false;
        render();
    })

    canvas.addEventListener("mousemove", throttle(event => {
        if (!running) {
            [mouseCol, mouseRow] = mousePosition()
            mouseRowInput.value = mouseRow;
            mouseColInput.value = mouseCol;
            render()
        }
    }, 60))

    canvas.addEventListener("click", event => {
        const [col, row] = mousePosition()

        if (event.altKey) {
            universe.set_cells([
                [col - 1, row + 1, ],
                [col    , row - 1, ],
                [col    , row + 1, ],
                [col + 1, row    , ],
                [col + 1, row + 1, ],
            ], Cell.Alive)
        } else if (event.shiftKey) {
            universe.set_cells([
                [col - 1, row - 2, ], [col - 1, row - 3, ], [col - 1, row - 4, ],
                [col - 2, row - 1, ], [col - 2, row - 6, ],
                [col - 3, row - 1, ], [col - 3, row - 6, ],
                [col - 4, row - 1, ], [col - 4, row - 6, ],
                [col - 6, row - 2, ], [col - 6, row - 3, ], [col - 6, row - 4, ],
                [col - 1, row + 2, ], [col - 1, row + 3, ], [col - 1, row + 4, ],
                [col - 2, row + 1, ], [col - 2, row + 6, ],
                [col - 3, row + 1, ], [col - 3, row + 6, ],
                [col - 4, row + 1, ], [col - 4, row + 6, ],
                [col - 6, row + 2, ], [col - 6, row + 3, ], [col - 6, row + 4, ],
                [col + 1, row - 2, ], [col + 1, row - 3, ], [col + 1, row - 4, ],
                [col + 2, row - 1, ], [col + 2, row - 6, ],
                [col + 3, row - 1, ], [col + 3, row - 6, ],
                [col + 4, row - 1, ], [col + 4, row - 6, ],
                [col + 6, row - 2, ], [col + 6, row - 3, ], [col + 6, row - 4, ],
                [col + 1, row + 2, ], [col + 1, row + 3, ], [col + 1, row + 4, ],
                [col + 2, row + 1, ], [col + 2, row + 6, ],
                [col + 3, row + 1, ], [col + 3, row + 6, ],
                [col + 4, row + 1, ], [col + 4, row + 6, ],
                [col + 6, row + 2, ], [col + 6, row + 3, ], [col + 6, row + 4, ],
            ], Cell.Alive)

        } else {
            universe.toggle_cell(col, row);
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
        reset_rle(s) {
            if (!running) {
                universe.free();
                universe = Universe.from_rle(s);
                sizeWidthInput.value = universe.width();
                sizeHeightInput.value = universe.height();
                render();
            }
        },
        reset_life_106(s) {
            if (!running) {
                universe.free();
                universe = Universe.from_life_106(s);
                sizeWidthInput.value = universe.width();
                sizeHeightInput.value = universe.height();
                render();
            }
        },
        setWidth(w) {
            if (!running) {
                universe.set_width(w);
                render();
            }
        },
        setHeight(h) {
            if (!running) {
                universe.set_height(h);
                render();
            }
        },
        moveUp() {
            if (!running) {
                universe.translate(0, -1);
                render();
            }
        },
        moveDown() {
            if (!running) {
                universe.translate(0, 1);
                render();
            }
        },
        moveLeft() {
            if (!running) {
                universe.translate(-1, 0);
                render();
            }
        },
        moveRight() {
            if (!running) {
                universe.translate(1, 0);
                render();
            }
        },
        clear() {
            universe.clear();
            render();
        },
        export_rle() {
            return universe.to_rle();
        },
        export_life_106() {
            return universe.to_life_106();
        },
        get running() {
            return running;
        }
    }
}

const controller = createController(5);

playPauseButton.addEventListener("click", event => {
    if (controller.running) {
        controller.stop();
        playPauseButton.classList.remove("play");
        playPauseButton.classList.add("pause");
        clearButton.disabled = false;
        stepButton.disabled = false;
        exportButton.disabled = false;
        moveLeftButton.disabled = false;
        moveRightButton.disabled = false;
        moveDownButton.disabled = false;
        moveUpButton.disabled = false;
    } else {
        controller.start();
        playPauseButton.classList.remove("pause");
        playPauseButton.classList.add("play");
        clearButton.disabled = true;
        stepButton.disabled = true;
        exportButton.disabled = true;
        moveLeftButton.disabled = true;
        moveRightButton.disabled = true;
        moveDownButton.disabled = true;
        moveUpButton.disabled = true;
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

moveDownButton.addEventListener("click", event => {
    if (!controller.running) {
        controller.moveDown();
    }
})

moveUpButton.addEventListener("click", event => {
    if (!controller.running) {
        controller.moveUp();
    }
})

moveLeftButton.addEventListener("click", event => {
    if (!controller.running) {
        controller.moveLeft();
    }
})

moveRightButton.addEventListener("click", event => {
    if (!controller.running) {
        controller.moveRight();
    }
})

sizeWidthInput.addEventListener("change", event => {
    if (!controller.running) {
        const width = parseInt(sizeWidthInput.value);
        controller.setWidth(width);
    }
})

sizeHeightInput.addEventListener("change", event => {
    if (!controller.running) {
        const height = parseInt(sizeHeightInput.value);
        controller.setHeight(height);
    }
})

exportButton.addEventListener("click", event => {
    if (!controller.running) {
        const text = controller.export_rle();
        // const text = controller.export_life_106();
        // ioBuffer.value = text;
        const buf = Buffer.from(new TextEncoder().encode(text));
        ioBuffer.value = `data:text/life_rle;base64,${buf.toString("base64")}`;
        // ioBuffer.value = `data:text/life_106;base64,${buf.toString("base64")}`;
    }
})

importButton.addEventListener("click", async event => {
    if (!controller.running) {
        const text = ioBuffer.value;
        const res = await fetch(text);

        console.log(res.headers.get("content-type"));

        res.headers.forEach((v, k) => {
            console.log(`${k}: ${v}`);
        });


        if (text.startsWith("data:text/life_106;base64,")) {
            const data = await res.text();
            controller.reset_life_106(data);
        } else if (text.startsWith("data:text/life_rle;base64,")) {
            const buf = await fetch(text);
            const data = await res.text();
            controller.reset_rle(data);
        } else {
            controller.reset_rle(text);
        }
    }
})