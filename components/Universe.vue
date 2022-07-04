<template>
  <div>
    <!-- <div id='fps'></div> -->
    <canvas id='canvas' ref='Universe' style='width: 100% height: 100%'></canvas>
    <p>{{ message }}</p>
  </div>
</template>

<script>
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
    var node = this.node;
    this.renderer.zoom_to(2);
    this.fps = 1000;
    this.playing = false;
    this.step = 1;

    this.lastFrame = performance.now();
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
    
    const render = () => {
      this.canvas_width = document.body.scrollWidth;
      this.canvas_height = document.body.scrollHeight;
      this.canvas.width = this.canvas_width;
      this.canvas.height = this.canvas_height;

      this.renderer.set_size(this.canvas_width, this.canvas_height, window.devicePixelRatio);

      const imagePtr = this.renderer.get_image_data(node);
      const image_data_array = new Uint8ClampedArray(this.memory.buffer, imagePtr, this.canvas_width * this.canvas_height * 4);

      const image_data = new ImageData(image_data_array, this.canvas_width, this.canvas_height);
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

    // temporary solution for unknown root cause
    const settleFrames = setInterval(render, 1000 / this.fps);
    setTimeout(() => {
      clearInterval(settleFrames);
    }, 1000);

    const advance = () => {
      node = Life.advance(node, this.step);
      render();
    };

    this.$nuxt.$on('playing', ($event) => {
      if ($event) {
        this.playing = true;
        this.animationLoop = setInterval(advance, 1000 / this.fps);
      }
      else {
        this.playing = false;
        clearInterval(this.animationLoop);
      }
    });

    this.$nuxt.$on('updateNode', ($event) => {
      node = $event;
      this.renderer.zoom_to(33);
      this.renderer.set_size(document.body.scrollWidth, document.body.scrollHeight, window.devicePixelRatio);
      this.renderer.center_view();
      if (!this.playing) render();
    });

    this.$nuxt.$on('advance', ($event) => {
      advance($event);
      if (!this.playing) render();
    });

    this.$nuxt.$on('doOffset' , ($event) => {
      switch ($event) {
        case 'left':
          this.renderer.move_offset(-100, 0);
          break;
        case 'right':
          this.renderer.move_offset(100, 0);
          break;
        case 'up':
          this.renderer.move_offset(0, -100);
          break;
        case 'down':
          this.renderer.move_offset(0, 100);
          break;
        default:
          break;
      }
      if (!this.playing) render();
    });

    this.$nuxt.$on('zoomOut', ($event) => {
      this.renderer.zoom_centered($event);
      if (!this.playing) render();
    });

    this.$nuxt.$on('centerView', () => {
      this.renderer.center_view();
      if (!this.playing) render();
    });

    var last_mouse_x = null;
    var last_mouse_y = null;
    const drag = (e) => {
      if (last_mouse_x !== null) {
        let dx = Math.round(e.clientX - last_mouse_x);
        let dy = Math.round(e.clientY - last_mouse_y);
        this.renderer.move_offset(dx, dy);
        last_mouse_x += dx;
        last_mouse_y += dy;
      }
    };
    this.canvas.onmousedown = (e) => {
      if (!this.playing) {
        this.renderLoop = setInterval(render, 1000 / this.fps);
      }

      // if (e.which === 3 || e.which === 2) {
      //   if (drawer.cell_width >= 1) {
      //     var coords = drawer.pixel2cell(e.clientX, e.clientY);
      //     mouse_set = !life.get_bit(coords.x, coords.y);
      //     window.addEventListener('mousemove', do_field_draw, true);
      //     do_field_draw(e);
      //   }
      // }
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
      window.removeEventListener('mousemove', drag, true);

      if (!this.playing) {
        clearInterval(this.renderLoop);
      }
    };
    this.canvas.onmousewheel = (e) => {
      e.preventDefault();
      this.renderer.zoom_at((e.wheelDelta || -e.detail) < 0, e.clientX, e.clientY - this.canvas.getBoundingClientRect().top);
      if (!this.playing) render();
      return false;
    };
  },
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