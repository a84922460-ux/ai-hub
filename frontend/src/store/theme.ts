import { defineStore } from "pinia";
import { ref, computed } from "vue";

export const useThemeStore = defineStore("theme", () => {
  const isDark = ref(true);

  const themeClass = computed(() => (isDark.value ? "dark" : "light"));

  function toggle() {
    isDark.value = !isDark.value;
    document.documentElement.setAttribute("data-theme", themeClass.value);
    localStorage.setItem("theme", themeClass.value);
  }

  function init() {
    const saved = localStorage.getItem("theme");
    if (saved) {
      isDark.value = saved === "dark";
    }
    document.documentElement.setAttribute("data-theme", themeClass.value);
  }

  return { isDark, themeClass, toggle, init };
});
