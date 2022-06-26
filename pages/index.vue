<template>
  <div>
    <div id='container'>
      <Universe id='universe' :node='node' :renderer='renderer' :memory='memory' :life='Life'></Universe>
      <Controls id='controls'></Controls>
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
        Life
      }
    },

    mounted() {

      const items = [0, 1, 0, 2, 0, 3];
      const node = Life.construct(items);

      $nuxt.$emit('updateNode', node);
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