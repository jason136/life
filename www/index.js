import { Universe } from "life";
import { memory } from "life/life_bg";

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();
console.log(width, height);

function main() {
    const canvas = document.getElementById("glCanvas");
    const gl = canvas.getContext("webgl");

    canvas.height = (CELL_SIZE + 1) * height + 1;
    canvas.width = (CELL_SIZE + 1) * width + 1;

    if (!gl) {
        alert("Unable to initialize WebGL. Your browser or machine may not support it.");
        return;
    }

    const vsSource = `
        attribute vec4 aVertexPosition;
        uniform mat4 uModelViewMatrix;
        uniform mat4 uProjectionMatrix;
        void main() {
            gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
        }
    `;
    const fsSource = `
        void main() {
            gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        }
    `;

    const shaderProgram = initShaderProgram(gl, vsSource, fsSource);
    const programInfo = {
        program: shaderProgram,
        attribLocations: {
            vertexPosition: gl.getAttribLocation(shaderProgram, "aVertexPosition"),
        }, 
        uniformLocations: {
            projectionMatrix: gl.getUniformLocation(shaderProgram, "uProjectionMatrix"),
            modelViewMatrix: gl.getUniformLocation(shaderProgram, "uModelViewMatrix"),
        },
    };

    const buffers = initBuffers(gl);
    drawScene(gl, programInfo, buffers);
};

main();

function initBuffers(gl) {
    const positionBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

    const positions = [
        1.0, 1.0,
        -1.0, 1.0,
        1.0, -1.0,
        -1.0, -1.0,
    ];

    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);

    return {
        position: positionBuffer,
    };
}

function drawScene(gl, programInfo, buffers) {
    gl.clearColor(0.0, 0.0, 0.0, 1.0);
    gl.clearDepth(1.0);
    gl.enable(gl.DEPTH_TEST);
    gl.depthFunc(gl.LEQUAL);

    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

    const fieldOfView = 45 * Math.PI / 180;
    const aspect = gl.canvas.clientWidth / gl.canvas.clientHeight;
    const zNear = 0.1;
    const zFar = 100.0;
    const projectionMatrix = mat4.create();

    mat4.perspective(projectionMatrix,
                    fieldOfView,
                    aspect,
                    zNear,
                    zFar);
    const modelViewMatrix = mat4.create();

    mat4.translate(modelViewMatrix, modelViewMatrix, [-0.0, 0.0, -6.0]);

    {
    const numComponents = 2;
    const type = gl.FLOAT;
    const normalize = false;
    const stride = 0;
    const offset = 0;
    gl.bindBuffer(gl.ARRAY_BUFFER, buffers.position);
    gl.vertexAttribPointer(
        programInfo.attribLocations.vertexPosition,
        numComponents,
        type,
        normalize,
        stride,
        offset);
    gl.enableVertexAttribArray(
        programInfo.attribLocations.vertexPosition);
    }

    gl.useProgram(programInfo.program);

    gl.uniformMatrix4fv(
        programInfo.uniformLocations.projectionMatrix,
        false,
        projectionMatrix);
    gl.uniformMatrix4fv(
        programInfo.uniformLocations.modelViewMatrix,
        false,
        modelViewMatrix);

    {
        const offset = 0;
        const vertexCount = 4;
        gl.drawArrays(gl.TRIANGLE_STRIP, offset, vertexCount);
    }
}

function initShaderProgram(gl, vsSource, fsSource) {
    const vertexShader = loadShader(gl, gl.VERTEX_SHADER, vsSource);
    const fragmentShader = loadShader(gl, gl.FRAGMENT_SHADER, fsSource);

    const shaderProgram = gl.createProgram();
    gl.attachShader(shaderProgram, vertexShader);
    gl.attachShader(shaderProgram, fragmentShader);
    gl.linkProgram(shaderProgram);

    if (!gl.getProgramParameter(shaderProgram, gl.LINK_STATUS)) {
        alert("Unable to initialize the shader program: " + gl.getProgramInfoLog(shaderProgram));
        return null;
    }

    return shaderProgram;
};
function loadShader(gl, type, source) {
    const shader = gl.createShader(type);

    gl.shaderSource(shader, source);
    gl.compileShader(shader);

    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        alert("An error occurred compiling the shaders: " + gl.getShaderInfoLog(shader));
        gl.deleteShader(shader);
        return null;
    }

    return shader;
};

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

const getIndex = (row, column) => {
    return row * width + column;
};
const bitIsSet = (n, arr) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) === mask;
};

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

// const renderLoop = () => {
//     fps.render();
//     // for (let i = 0; i < 9; i++) {
//     //     universe.tick();
//     // }
//     universe.tick();
//     drawGrid();
//     drawCells();

//     animationId = requestAnimationFrame(renderLoop);
// };

// let animationId = null;
// const isPaused = () => {
//     return animationId === null;
// };

// const playPauseButton = document.getElementById("play-pause");
// const play = () => {
//     playPauseButton.textContent = "⏸";
//     renderLoop();
// };
// const pause = () => {
//     playPauseButton.textContent = "▶";
//     cancelAnimationFrame(animationId);
//     animationId = null;
// };
// playPauseButton.addEventListener("click", () => {
//     if (isPaused()) {
//         play();
//     }
//     else {
//         pause();
//     }
// });

// play();