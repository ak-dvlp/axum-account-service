<script setup lang="ts">
import { onMounted, ref } from "vue";

type CurrentTheme = "light" | "dark";

const currentTheme = ref<CurrentTheme>("light");

function saveAndApply(theme: CurrentTheme) {
  currentTheme.value = theme;
  localStorage.setItem("currentTheme", theme);

  const html = document.documentElement;
  html.classList.remove("light", "dark");
  html.classList.add(theme === "dark" ? "dark" : "light");
}

function toggleTheme() {
  saveAndApply(currentTheme.value === "light" ? "dark" : "light");
}

onMounted(() => {
  const theme = localStorage.getItem("currentTheme") as CurrentTheme | null;

  if (theme && (theme === "light" || theme === "dark")) {
    saveAndApply(theme);
  } else {
    saveAndApply("light");
  }
});
</script>

<template>
  <button
    class="cursor-pointer rounded-lg p-2 transition-colors focus:ring-2 focus:ring-slate-500 focus:outline-none"
    :class="
      currentTheme === 'light'
        ? 'hover:bg-my-theme-dark'
        : 'hover:bg-my-theme-light'
    "
    @click="toggleTheme"
  >
    <span aria-hidden="true">
      {{ currentTheme === "dark" ? "🌙" : "☀️" }}
    </span>
  </button>
</template>
