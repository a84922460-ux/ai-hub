<template>
  <div class="app-container" :data-theme="theme.themeClass">
    <TopBar />
    <div class="main-layout">
      <Sidebar />
      <ChatView />
    </div>
  </div>
</template>

<script setup lang="ts">
import { useThemeStore } from "./store/theme";
import TopBar from "./components/TopBar.vue";
import Sidebar from "./components/Sidebar.vue";
import ChatView from "./components/ChatView.vue";

const theme = useThemeStore();
</script>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: var(--color-bg);
}
.main-layout {
  display: flex;
  flex: 1;
  overflow: hidden;
}
</style>
