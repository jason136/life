import { init_panic_hook, Node, Life, Renderer } from "life";
import { memory } from "life/life_bg";

// import { Renderer } from "./render";

// https://github.com/copy/life/blob/924c90afb529ad5d417f11d795bc1b400fff8d18/main.js

init_panic_hook();

const canvas = document.getElementById("canvas");
const context = canvas.getContext("2d");

const canvas_width = canvas.width;
const canvas_height = canvas.height;


const items5 = [1, 1, 2, 2, 3, 3, 4, 4, -1, -1, -2, -2, -3, -3, -100000000, -100000000, 100000000, 100000000];
var node5 = Life.construct(items5);

var renderer = Renderer.new();
renderer.set_center(canvas_width / 2, canvas_height / 2);
renderer.set_size(canvas_width, canvas_height);
renderer.set_zoom(1.0);

console.log(renderer);

renderer.update_image_data(node5);
const imagePtr = renderer.image_data_ptr();

// const int32_data = new Int32Array(memory.buffer, imagePtr, canvas_width * canvas_height);
// const image_data_data = new Uint8ClampedArray(int32_data.buffer);
const image_data_data = new Uint8ClampedArray(memory.buffer, imagePtr, canvas_width * canvas_height);
console.log(image_data_data);
const image_data = new ImageData(image_data_data, canvas_width, canvas_height);

context.putImageData(image_data, 0, 0);




const create = document.getElementById("create");
create.addEventListener("click", () => {
  const items1 = [1, 1, 2, 2, 3, 3, 4, 4, -1, -1, -2, -2, -3, -3, -100000000, -100000000, 100000000, 100000000];
  const node1 = Life.construct(items1);
  console.log(node1.hash());
  console.log(node1.level());
  console.log(node1.population());
  const items2 = Life.expand(node1, 0, 0);
  console.log(items2);
})

const forward = document.getElementById("forward");

forward.addEventListener("click", () => {
  for (let x = 0; x < 100; x++) {
    node5 = Life.ffwd(node5, 10);
    console.log(node5.population());
  }

  const items = Life.expand(node5, 0, 0);
  console.log(items);
})

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