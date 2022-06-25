<template>
  <div>
    <!-- <div id="fps"></div> -->
    <canvas id="canvas" ref="Universe" style="width: 100% height: 100%"></canvas>
    <p>{{ diag }}</p>
  </div>
</template>

<script>

export default {
  props: ['node', 'renderer', 'memory'], 

  data() {
    const diag = '';

    return {
      diag
    }
  },

  mounted() {
    this.canvas = this.$refs['Universe'];
    this.ctx = this.canvas.getContext('2d');

    var node = this.node;
    const renderer = this.renderer;

    renderer.zoom_to(2);

    setInterval(() => {
      this.canvas_width = document.body.scrollWidth;
      this.canvas_height = document.body.scrollHeight;

      this.canvas.width = this.canvas_width;
      this.canvas.height = this.canvas_height;

      console.log(`${this.canvas_width}, ${this.canvas_height}`);

      renderer.set_size(this.canvas_width, this.canvas_height, window.devicePixelRatio);

      const imagePtr = renderer.get_image_data(node);
      const image_data_array = new Uint8ClampedArray(this.memory.buffer, imagePtr, this.canvas_width * this.canvas_height * 4);
      const image_data = new ImageData(image_data_array, this.canvas_width, this.canvas_height);
      
      // console.log(image_data_array);

      this.ctx.putImageData(image_data, 0, 0);

      this.diag = renderer.log_properties(); 
    }, 15);

    this.$nuxt.$on('updateNode', ($event) => {
      node = $event;
    })

    var last_mouse_x = null;
    var last_mouse_y = null;

    function drag(e) {
      if (last_mouse_x !== null) {
        let dx = Math.round(e.clientX - last_mouse_x);
        let dy = Math.round(e.clientY - last_mouse_y);

        renderer.move_offset(dx, dy);

        last_mouse_x += dx;
        last_mouse_y += dy;
      }
    }

    this.canvas.onmousedown = (e) => {
      if(e.which === 3 || e.which === 2) {
        if(drawer.cell_width >= 1) {
          var coords = drawer.pixel2cell(e.clientX, e.clientY);

          mouse_set = !life.get_bit(coords.x, coords.y);

          window.addEventListener("mousemove", do_field_draw, true);
          do_field_draw(e);
        }
      }
      else if(e.which === 1) {
        last_mouse_x = e.clientX;
        last_mouse_y = e.clientY;
        //console.log("start", e.clientX, e.clientY);

        window.addEventListener("mousemove", drag, true);
      }
      return false;
    };

    this.canvas.onmouseup = () => {
      last_mouse_x = null;
      last_mouse_y = null;

      window.removeEventListener("mousemove", drag, true);
    };

    this.canvas.onmousewheel = (e) => {
      e.preventDefault();
      renderer.zoom_at((e.wheelDelta || -e.detail) < 0, e.clientX, e.clientY - this.canvas.getBoundingClientRect().top);
      return false;
    }
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