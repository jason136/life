<template>
  <div>
    <div id="fps"></div>
    <Universe id="universe" :node="node" :renderer="renderer" :memory="memory"></Universe>
    <p>{{  }}</p>
  </div>
</template>

<script>
import { init_panic_hook, Life, Renderer } from "life";
import { memory } from "life/life_bg";

import Universe from "../components/Universe.vue";

export default {
    name: "app",
    components: { Universe }, 

    data() {
      init_panic_hook();
      
      const items = [];
      const node = Life.construct(items);

      const renderer = Renderer.new();

      return {
        node,
        renderer,
        memory
      }
    },

    mounted() {

      const items = [-1, -1, -2, -2, -3, -3, 1, 1, 2, 2, 3, 3, -4, -4, -5, -5, -6, -6, -7, -7, -8, -8, -9, -9];
      const node = Life.construct(items);

      $nuxt.$emit('updateNode', node);
    },
};

</script>

<style>

</style>