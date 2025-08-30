<script setup lang="ts">
// 不需要导入任何组件，路由会自动处理组件的加载
</script>

<template>
  <div id="app">
    <router-view />
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  height: 100%;
  overflow: hidden;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
    'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

/* 覆盖 style.css 中可能影响布局的样式 */
body {
  display: block !important;
  place-items: unset !important;
  min-width: unset !important;
  min-height: unset !important;
  background-color: transparent !important;
}

#app {
  width: 100vw !important;
  height: 100vh !important;
  overflow: hidden !important;
  max-width: none !important;
  margin: 0 !important;
  padding: 0 !important;
  text-align: initial !important;
}
</style>
