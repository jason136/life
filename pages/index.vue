<template>
  <div>
    <div id='container'>
      <Universe id='universe' :node='node' :renderer='renderer' :memory='memory' :life='Life'></Universe>
      <Controls id='controls'></Controls>

      <textarea v-model="pattern" placeholder="rle goes here"></textarea>
      <button v-on:click="loadRLE(pattern)">submit</button>
    </div>
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
      const items = [];
      const node = Life.construct(items);

      const renderer = Renderer.new();
      
      return {
        node,
        renderer,
        memory, 
        Life,

        pattern: '',
      }
    },

    beforeCreate() {
      init_panic_hook();
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

      // gosper gun
      const items = [24, 0, 22, 1, 24, 1, 12, 2, 13, 2, 20, 2, 21, 2, 34, 2, 35, 2, 11, 3, 15, 3, 20, 3, 21, 3, 34, 3, 35, 3, 0, 4, 1, 4, 10, 4, 16, 4, 20, 4, 21, 4, 0, 5, 1, 5, 10, 5, 14, 5, 16, 5, 17, 5, 22, 5, 24, 5, 10, 6, 16, 6, 24, 6, 11, 7, 15, 7, 12, 8, 13, 8]

      var node = Life.construct(items);
      console.log(node.level());
      console.log(Life.expand(node));
      console.log(Life.convert_rle(Life.expand(node), 'gosper gun'));
    
      $nuxt.$emit('updateNode', node);
      $nuxt.$emit('centerView');
    },

    methods: {
      loadRLE(pattern) {
        const items = Life.parse_rle(pattern);
        const node = Life.construct(items);
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