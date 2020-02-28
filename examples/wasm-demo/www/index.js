// If you only use `npm` you can simply
// import { Chart } from "wasm-demo" and remove `setup` call from `bootstrap.js`.
class Chart {}

const canvas = document.getElementById("canvas");
const coord = document.getElementById("coord");
// const plotType = document.getElementById("plot-type");
const status = document.getElementById("status");

let chart = null;

/** Main entry point */
export function main() {
    setupUI();
    setupCanvas();
}

/** This function is used in `bootstrap.js` to setup imports. */
export function setup(WasmChart) {
    Chart = WasmChart;
}

/** Add event listeners. */
function setupUI() {
    status.innerText = "WebAssembly loaded!";
    // plotType.addEventListener("change", updatePlot);
    window.addEventListener("resize", setupCanvas);
    window.addEventListener("mousemove", onMouseMove);
    //render new image every 2 seconds
    setInterval(()=>updatePlot(), 8000)
}

/** Setup canvas to properly handle high DPI and redraw current plot. */
function setupCanvas() {
    // const dpr = window.devicePixelRatio || 1;
    // const aspectRatio = canvas.width / canvas.height;
    // const size = Math.min(canvas.width, canvas.parentNode.offsetWidth);
    // canvas.style.width = size + "px";
    // canvas.style.height = size / aspectRatio + "px";
    
    // canvas.width = size * dpr;
    // canvas.height = size / aspectRatio * dpr;
    canvas.getContext("2d").scale(1, 1);
    updatePlot();
}

/** Update displayed coordinates. */
function onMouseMove(event) {
    if (chart) {
        const point = chart.coord(event.offsetX, event.offsetY);
        coord.innerText = (point)
            ? `(${point.x.toFixed(3)}, ${point.y.toFixed(3)})`
            : "Mouse pointer is out of range";
    }
}

//store transactions
let txs = {}
let socket = io.connect('http://localhost:8081');
let index = 0
socket.on('tx', function (tx) {
    index++
    txs[Object.keys(tx)] = tx[Object.keys(tx)]+index.toString().padStart(10, "0")
    // console.log(index);
})


/** Redraw currently selected plot. */
function updatePlot() {
    // const selected = plotType.selectedOptions[0];
    status.innerText = `Rendering ...`;
    chart = null;
    const start = performance.now();
    let nine = '9'.repeat(81)
    let A = 'A'.repeat(81)
    let B = 'B'.repeat(81)
    let C = 'C'.repeat(81)
    let D = 'D'.repeat(81)
    let E = 'E'.repeat(81)
    let F = 'F'.repeat(81)
    let arr = {
        [B]:nine+A+'0000000001',
        [C]:nine+B+'0000000002',
        [D]:A+C+'0000000003',
        [E]:B+C+'0000000004',
        [F]:E+B+'0000000005',
    }

    // Chart.render_tangle("canvas", arr, 5);

    //run "node zmqserver.js" to get txs
    //value > 4 fails
    Chart.render_tangle("canvas", txs, 4);
    const end = performance.now();
    status.innerText = `Rendered in ${Math.ceil(end - start)}ms`;
}
