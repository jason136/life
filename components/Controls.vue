<template>
  <div>
    <nav>
      <ul>
        <li>
          <button id='' v-on:click="$nuxt.$emit('doOffset', 'up')">up</button>
        </li>
        <li>
          <button id='' v-on:click="$nuxt.$emit('doOffset', 'down')">down</button>
        </li>
        <li>
          <button id='' v-on:click="$nuxt.$emit('doOffset', 'left')">left</button>
        </li>
        <li>
          <button id='' v-on:click="$nuxt.$emit('doOffset', 'right')">right</button>
        </li>
        <li>
          <button id='' v-on:click="$nuxt.$emit('zoomOut', false)">in</button>
        </li>
        <li>
          <button id='' v-on:click="$nuxt.$emit('zoomOut', true)">out</button>
        </li>
        <li>
          <button id='' v-on:click="$nuxt.$emit('centerView', true)">center</button>
        </li>
        <li>
          <button id='' v-on:click="$nuxt.$emit('fastForward', 1)">forward</button>
        </li>
      </ul>
    </nav>
  </div>
</template>

<script>
export default {
    mounted() {

    },
};
</script>

<style>
:root {
    --primary-color: #00c58e;
}

body {
  font-family: system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, Roboto,
  Helvetica Neue, Arial, Noto Sans, sans-serif, Apple Color Emoji,
  Segoe UI Emoji, Segoe UI Symbol, Noto Color Emoji;
  margin: 0;
  background-color: rgb(23, 24, 28);
}

a,
a:visited {
  text-decoration: none;
  color: inherit;
}

a:hover,
a.nuxt-link-exact-active {
  color: var(--primary-color);
}

main {
  margin: 0 auto;
  margin-top: 25vh;
  margin-bottom: 2rem;
  padding: 0 1rem;
  max-width: 1280px;
  text-align: center;
}

ul {
  background-color: #0b0b0b;
  list-style-type: none;
  padding: 0;
  display: flex;
  justify-content: center;
  flex-wrap: wrap;
}

li {
  color: azure;
  margin: 0 1.5rem;
  padding: 0.5rem;
  font-size: 1.5rem;
}

nav {
  padding: 0 2rem;
}
</style>