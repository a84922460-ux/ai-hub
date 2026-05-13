<template>
  <header class="topbar glass">
    <span class="app-title">大模型自由集市</span>
    <div class="actions">
      <button class="btn-glass" @click="newChat">+ 新建对话</button>
      <button class="btn-glass" @click="theme.toggle()">
        {{ theme.isDark ? '☀️ 明亮' : '🌙 暗黑' }}
      </button>
    </div>
  </header>
</template>

<script setup lang="ts">
import { useThemeStore } from "../store/theme";
import { useChatStore } from "../store/chat";

const theme = useThemeStore();
const chat = useChatStore();

async function newChat() {
  await chat.newConversation("新对话", "auto");
}
</script>

<style scoped>
.topbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
  height: 56px;
  border-bottom: 1px solid var(--color-border);
}
.app-title {
  font-size: 18px;
  font-weight: 600;
}
.actions {
  display: flex;
  gap: 12px;
}
.btn-glass {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  color: var(--color-text-primary);
  padding: 6px 14px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  backdrop-filter: var(--blur-glass);
}
.btn-glass:hover {
  background: var(--color-accent);
  color: white;
}
</style>
