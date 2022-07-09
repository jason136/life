<template>
  <div>
    <!-- <div id='fps'></div> -->
    <canvas id='canvas' ref='Universe' style='width: 100% height: 100%'></canvas>
    <p>{{ message }}</p>
  </div>
</template>

<script>
import { Renderer } from '../www/render';

export default {
  props: ['node', 'renderer', 'memory', 'life'],
  data() {
    return {
      message: "",
    };
  },
  mounted() {
    this.canvas = this.$refs['Universe'];
    this.ctx = this.canvas.getContext('2d');

    const Life = this.life;
    const renderer = this.renderer;
    var node = this.node;

    renderer.zoom_to(2);
    var fps = 1000;
    var playing = false;
    var step = 1;

    this.lastFrame = performance.now();
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
    
    const render = () => {
      const canvas_width = document.body.scrollWidth;
      const canvas_height = document.body.scrollHeight;
      this.canvas.width = canvas_width;
      this.canvas.height = canvas_height;

      renderer.set_size(canvas_width, canvas_height, window.devicePixelRatio);

      if (this.selected_cells.length > 0) {
        this.selected_cells.forEach(cell => {
          renderer.draw_cell(cell[0], cell[1]);
        });
      }

      const imagePtr = renderer.get_image_data(node);
      const image_data_array = new Uint8ClampedArray(this.memory.buffer, imagePtr, canvas_width * canvas_height * 4);

      const image_data = new ImageData(image_data_array, canvas_width, canvas_height);
      this.ctx.putImageData(image_data, 0, 0);

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
  
      this.message = `
      Frames per Second:
      latest = ${Math.round(fps)}
      avg of last 100 = ${Math.round(mean)}
      min of last 100 = ${Math.round(min)}
      max of last 100 = ${Math.round(max)}
      `.trim();
    };

    const advance = () => {
      node = Life.advance(node, step);
      render();
    };

    // temporary solution for unknown root cause
    const settleFrames = setInterval(render, 1000 / fps);
    setTimeout(() => {
      clearInterval(settleFrames);
    }, 1000);

    this.$nuxt.$on('playing', ($event) => {
      if ($event) {
        playing = true;
        this.animationLoop = setInterval(advance, 1000 / fps);
      }
      else {
        playing = false;
        clearInterval(this.animationLoop);
      }
    });

    this.$nuxt.$on('updateNode', ($event) => {
      node = $event;
      renderer.zoom_to(33);
      renderer.set_size(document.body.scrollWidth, document.body.scrollHeight, window.devicePixelRatio);
      renderer.center_view();
      if (!playing) render();
    });

    this.$nuxt.$on('advance', () => {
      advance(step);
      if (!playing) render();
    });

    this.$nuxt.$on('doOffset' , ($event) => {
      switch ($event) {
        case 'left':
          renderer.move_offset(-100, 0);
          break;
        case 'right':
          renderer.move_offset(100, 0);
          break;
        case 'up':
          renderer.move_offset(0, -100);
          break;
        case 'down':
          renderer.move_offset(0, 100);
          break;
        default:
          break;
      }
      if (!playing) render();
    });

    this.$nuxt.$on('zoomOut', ($event) => {
      renderer.zoom_centered($event);
      if (!playing) render();
    });

    this.$nuxt.$on('centerView', () => {

      for (var x = 0; x < Math.pow(2, node.level()); x++) {
        for (var y = 0; y < Math.pow(2, node.level()); y++) {
          var nx = x - Math.pow(2, node.level() - 1);
          var ny = y - Math.pow(2, node.level() - 1);
          //console.log(nx, ny);
          if (Life.is_alive(node, nx, ny)) {
            renderer.draw_cell(nx, ny);
          }
        }
      }
      console.log(node.level());
      render();

      // renderer.center_view();
      // if (!playing) render();
    });

    function getMousePos(canvas, evt) {
      var rect = canvas.getBoundingClientRect();
      return {
        x: evt.clientX - rect.left,
        y: evt.clientY - rect.top
      };
    }

    var last_mouse_x = null;
    var last_mouse_y = null;
    const drag = (e) => {
      if (last_mouse_x !== null) {
        let dx = Math.round(e.clientX - last_mouse_x);
        let dy = Math.round(e.clientY - last_mouse_y);
        renderer.move_offset(dx, dy);
        last_mouse_x += dx;
        last_mouse_y += dy;
      }
    };

    this.selected_cells = [];
    const draw = (e) => {
      const mouse_pos = getMousePos(this.canvas, e);
      var coords = renderer.pixel_to_cell(mouse_pos.x, mouse_pos.y);
      // console.log(`${coords[0]}, ${coords[1]}`);

      this.selected_cells = [];
      this.selected_cells.push(coords);

      // console.log(Life.is_alive(node, coords[0], coords[1]));

      Life.set_cell(node, coords[0], coords[1], false);

      if (!playing) render();
    }

    this.canvas.onmousedown = (e) => {
      e.preventDefault();

      if (!playing) {
        this.renderLoop = setInterval(render, 1000 / fps);
      }

      if (e.which === 3 || e.which === 2) {
        this.canvas.addEventListener('contextmenu', (e) => e.preventDefault());

        if (renderer.get_cell_width() >= 1) {
          window.addEventListener('mousemove', draw, true);
        }
      }
      if (e.which === 1) {
        last_mouse_x = e.clientX;
        last_mouse_y = e.clientY;
        window.addEventListener('mousemove', drag, true);
      }
      return false;
    };

    window.onmouseup = () => {
      last_mouse_x = null;
      last_mouse_y = null;
      this.selected_cells = [];
      window.removeEventListener('mousemove', draw, true);
      window.removeEventListener('mousemove', drag, true);
      this.canvas.removeEventListener('contextmenu', (e) => e.preventDefault());

      clearInterval(this.renderLoop);
      render();
    };

    this.canvas.onmousewheel = (e) => {
      e.preventDefault();
      const mouse_pos = getMousePos(this.canvas, e);
      renderer.zoom_at((e.wheelDelta || -e.detail) < 0, mouse_pos.x, mouse_pos.y);
      if (!playing) render();
      return false;
    };

    window.addEventListener('resize', () => {
      if (!playing) render();
    }, true);
  },
  destroy() {
    window.removeEventListener('mousemove', draw, true);
    window.removeEventListener('mousemove', drag, true);
    window.removeEventListener('resize', () => {
      if (!playing) render();
    }, true);
    this.canvas.removeEventListener('contextmenu', (e) => e.preventDefault());

    clearInterval(this.animationLoop)
    clearInterval(this.renderLoop);
  }
};
</script>

<style>
body {
  margin: 0 !important;
  padding: 0 !important;
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}
#fps {
  white-space: pre;
  font-family: monospace;
}

#canvas {
  width: 100%;
  height: 100%;
  display: block;
}
</style>