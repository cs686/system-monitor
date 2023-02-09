<script setup lang="ts">
import { onMounted } from "vue";
import Greet from "./components/Greet.vue";
onMounted(() => {
  document.body.addEventListener("touchmove",(e) => {
    e.preventDefault()
  }, {passive: false})
})
</script>

<template>
  <div class="container">
    <Greet />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
