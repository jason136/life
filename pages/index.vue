<template>
  <div>
    <div id='container'>
      <Universe id='universe' :node='node' :renderer='renderer' :memory='memory' :life='Life'></Universe>
      <Controls id='controls'></Controls>

      <textarea v-model="message" placeholder="rle goes here"></textarea>
      <button v-on:click="loadRLE(message)">submit</button>
    </div>
    <p>{{  }}</p>
  </div>
</template>

<script>
import { init_panic_hook, Life, Renderer } from 'life';
import { memory } from 'life/life_bg';

import Universe from '../components/Universe.vue';
import Controls from '../components/Controls.vue';

export default {
    name: 'app',
    components: { Universe, Controls }, 

    data() {
      init_panic_hook();
      
      const items = [];
      const node = Life.construct(items);

      const renderer = Renderer.new();

      return {
        node,
        renderer,
        memory, 
        Life,

        message: '',
      }
    },

    mounted() {
      // glider
      // const items = [0, 0, 1, 1, 2, 1, 2, 0, 2, -1, ]

      // acorn
      // const items = [0, 0, 1, 0, 1, -2, 3, -1, 4, 0, 5, 0, 6, 0]

      // pulsar
      // const items = [1, 2, 1, 3, 1, 4, 2, 1, 3, 1, 4, 1, 1, -2, 1, -3, 1, -4, 2, -1, 3, -1, 4, -1, 
      //                 -1, -2, -1, -3, -1, -4, -2, -1, -3, -1, -4, -1, -1, 2, -1, 3, -1, 4, -2, 1, -3, 1, -4, 1, 
      //                 2, 6, 3, 6, 4, 6, 2, -6, 3, -6, 4, -6, -2, -6, -3, -6, -4, -6, -2, 6, -3, 6, -4, 6,
      //                 6, 2, 6, 3, 6, 4, 6, -2, 6, -3, 6, -4, -6, 2, -6, 3, -6, 4, -6, -2, -6, -3, -6, -4];

      // block layer
      const items = [0, 0, 1, 0, 2, 0, 4, 0, 0, 1, 3, 2, 4, 2, 1, 3, 2, 3, 4, 3, 0, 4, 2, 4, 4, 4]

      const node = Life.construct(items);
      console.log(Life.convert_rle(Life.expand(node)));
      $nuxt.$emit('updateNode', node);
    },

    methods: {
      loadRLE(message) {
        const items = Life.parse_rle(message);
        const node = Life.construct(items);
        console.log(items);
        console.log(node);
        console.log(node.hash());
        $nuxt.$emit('updateNode', node);
      }
    },
};
</script>

<style>
/* #container {
  position: relative;
} */
/* #universe {
  position: absolute;
  background-color:lightgrey
} */
#controls {
  position:absolute;
  left:0%;
  top:0%
}
</style>