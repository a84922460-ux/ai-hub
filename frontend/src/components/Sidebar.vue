<template>
  <aside class="sidebar glass">
    <div class="section">
      <h3>对话历史</h3>
      <div class="conv-list">
        <div
          v-for="conv in chat.conversations"
          :key="conv.id"
          class="conv-item"
          :class="{ active: conv.id === chat.activeConvId }"
          @click="selectConv(conv.id)"
        >
          <span class="title">{{ conv.title }}</span>
          <span class="time">{{ formatTime(conv.updated_at) }}</span>
        </div>
      </div>
    </div>
    <div class="section">
      <h3>模型状态</h3>
      <div class="model-badge">✅ Gemini</div>
      <div class="model-badge">✅ DeepSeek</div>
      <div class="model-badge">⚠️ 智谱</div>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import { useChatStore } from "../store/chat";
const chat = useChatStore();

onMounted(() => {
  chat.loadConversations();
});

function selectConv(id: number) {
  chat.activeConvId = id;
  chat.loadMessages(id);
}

function formatTime(dateStr: string): string {
  const d = new Date(dateStr);
  return d.toLocaleString("zh-CN");
}
</script>

<style scoped>
.sidebar {
  width: 260px;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 20px;
}
.section h3 {
  font-size: 13px;
  color: var(--color-text-secondary);
  margin-bottom: 8px;
  text-transform: uppercase;
}
.conv-item {
  padding: 8px 12px;
  border-radius: 8px;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.conv-item:hover,
.conv-item.active {
  background: var(--color-border);
}
.title {
  font-size: 14px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 160px;
}
.time {
  font-size: 11px;
  color: var(--color-text-secondary);
}
.model-badge {
  font-size: 13px;
  padding: 4px 0;
}
</style>
