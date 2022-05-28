import { init_panic_hook, Node } from "life";
import { memory } from "life/life_bg";

const CELL_SIZE = 0.1;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

init_panic_hook();

const items = [10, 15, 20, 25, 25, 40, 5, 7, 4, 7, 2, 8, 55, 52, 47, 58];
const node = Node.construct(items);
console.log(node);
console.log(node.hash());
console.log(node.level());
console.log(node.population());

console.log("new population: ", node.population());

const create = document.getElementById("create");
create.addEventListener("click", () => {
  const items1 = [1, 1, 2, 2, 3, 3, 4, 4, -1, -1, -2, -2, -3, -3, -400, -400, 500, 500];;
  const node1 = Node.construct(items1);
  console.log(node1.hash());
  console.log(node1.level());
  console.log(node1.population());
  const items2 = Node.expand(node1, 0, 0);
  console.log(items2);

  // Node.ffwd(node1, 10);
  // console.log(node1.level());
  // console.log(node1.population());
})

const forward = document.getElementById("forward");
forward.addEventListener("click", () => {
  var node5 = Node.ffwd(node, 10);
  console.log(node5.level());
  console.log(node5.population());

  const items = Node.expand(node5, 0, 0);
  console.log(items);

  // Node.ffwd(node1, 10);
  // console.log(node1.level());
  // console.log(node1.population());
})


// function drawSquare(gl, x, y) {
//     const positionBuffer = gl.createBuffer();
//     gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

//     const positions = [
//         CELL_SIZE / 2 + x, CELL_SIZE / 2  - y,
//         -CELL_SIZE / 2 + x, CELL_SIZE / 2 - y,
//         CELL_SIZE / 2 + x, -CELL_SIZE / 2 - y,
//         -CELL_SIZE / 2 + x, -CELL_SIZE / 2 - y,
//     ];

//     gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);

//     return {
//         position: positionBuffer,
//     };
// };

// const drawGrid = () => {
//     ctx.beginPath();
//     ctx.strokeStyle = GRID_COLOR;

//     for (let x = 0; x <= width; x++) {
//         ctx.moveTo(x * (CELL_SIZE + 1) + 1, 0);
//         ctx.lineTo(x * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
//     }

//     for (let y = 0; y <= height; y++) {
//         ctx.moveTo(0, y * (CELL_SIZE + 1) + 1);
//         ctx.lineTo((CELL_SIZE + 1) * width + 1, y * (CELL_SIZE + 1) + 1);
//     }

//     ctx.stroke();
// };

// const getIndex = (row, column) => {
//     return row * width + column;
// };
// const bitIsSet = (n, arr) => {
//     const byte = Math.floor(n / 8);
//     const mask = 1 << (n % 8);
//     return (arr[byte] & mask) === mask;
// };

// const drawCells = () => {
//     const cellsPtr = universe.cells();
//     const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);

//     ctx.beginPath();

//     ctx.fillStyle = ALIVE_COLOR;
//     for (let row = 0; row < height; row++) {
//         for (let col = 0; col < width; col++) {
//             const idx = getIndex(row, col);
//             if (bitIsSet(idx, cells)) continue;
//             ctx.fillRect(
//                 col * (CELL_SIZE + 1) + 1,
//                 row * (CELL_SIZE + 1) + 1,
//                 CELL_SIZE,
//                 CELL_SIZE
//             );
//         }
//     }
//     ctx.fillStyle = DEAD_COLOR;
//     for (let row = 0; row < height; row++) {
//         for (let col = 0; col < width; col++) {
//             const idx = getIndex(row, col);
//             if (!bitIsSet(idx, cells)) continue;
//             ctx.fillRect(
//                 col * (CELL_SIZE + 1) + 1,
//                 row * (CELL_SIZE + 1) + 1,
//                 CELL_SIZE,
//                 CELL_SIZE
//             );
//         }
//     }
//     ctx.stroke();
// };

// canvas.addEventListener("click", event => {
//     const boundingRect = canvas.getBoundingClientRect();
//     const scaleX = canvas.width / boundingRect.width;
//     const scaleY = canvas.height / boundingRect.height;

//     const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
//     const canvasTop = (event.clientY - boundingRect.top) * scaleY;

//     const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
//     const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

//     universe.toggle_cell(row, col);

//     if (event.shiftKey) {
//         universe.toggle_cell(row, col);
//     }

//     drawGrid();
//     drawCells();
// });

const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;
    
        this.frames.push(fps);
        if (this.frames.length > 100) {
          this.frames.shift();
        }
    
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
          sum += this.frames[i];
          min = Math.min(this.frames[i], min);
          max = Math.max(this.frames[i], max);
        }
        let mean = sum / this.frames.length;
    
        this.fps.textContent = `
        Frames per Second:
        latest = ${Math.round(fps)}
        avg of last 100 = ${Math.round(mean)}
        min of last 100 = ${Math.round(min)}
        max of last 100 = ${Math.round(max)}
        `.trim();
    }
};