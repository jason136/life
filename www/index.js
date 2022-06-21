import { init_panic_hook, Life, Renderer } from "life";
import { memory } from "life/life_bg";

// https://github.com/copy/life/blob/924c90afb529ad5d417f11d795bc1b400fff8d18/main.js

init_panic_hook();

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

var renderer = Renderer.new();

const canvas_width = canvas.scrollWidth;
const canvas_height = canvas.scrollHeight;

renderer.set_size(canvas_width, canvas_height, window.devicePixelRatio);
renderer.zoom_to(2);

console.log(`${canvas_width}, ${canvas_height}`);


const render = (node) => {
  ctx.canvas.width = window.innerWidth;
  ctx.canvas.height = window.innerHeight;

  const imagePtr = renderer.get_image_data(node);

  const image_data_array = new Uint8ClampedArray(memory.buffer, imagePtr, canvas_width * canvas_height * 4);
  console.log(image_data_array);

  const image_data = new ImageData(image_data_array, canvas_width, canvas_height);

  console.log(image_data);

  console.log('put data');
  ctx.putImageData(image_data, 0, 0);
}


const items = [-1, -1, -2, -2, -3, -3, 1, 1, 2, 2, 3, 3, -4, -4, -5, -5, -6, -6, -7, -7, -8, -8, -9, -9]; 
var node = Life.construct(items);

const renderLoop = () => {

  fps.render();
  
  console.log('generate successor');
  node = Life.ffwd(node, 1);

  console.log('render');
  render(node);

  // requestAnimationFrame(renderLoop);
}

render(node);
// requestAnimationFrame(renderLoop);





const create = document.getElementById("create");
create.addEventListener("click", () => {
  renderer.zoom_centered(false);
  canvas_size();
  render(node);
})

const forward = document.getElementById("forward");

forward.addEventListener("click", () => {
  renderer.zoom_centered(true);
  canvas_size();
  render(node);
})

const canvas_size = () => {
  const label = document.getElementById("label");
  label.textContent = `
        ${canvas_width}x${canvas_height}
        ${renderer.get_size()}
        `.trim();
}

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