<script setup lang="ts">
import ThemeToggle from "./components/ThemeToggle.vue";
</script>

<template>
  <div
    class="bg-my-body-background text-my-body-text fixed grid grid-rows-[3rem_auto] inset-0"
  >
    <header class="shadow-my-card flex justify-end p-1">
      <ThemeToggle />
    </header>

    <RouterView />
  </div>
</template>
