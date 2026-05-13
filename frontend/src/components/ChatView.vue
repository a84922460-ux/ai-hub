<template>
  <main class="chat-area">
    <div class="messages" ref="msgContainer">
      <div
        v-for="msg in chat.messages"
        :key="msg.id"
        class="message-bubble"
        :class="msg.role"
      >
        <div class="content">{{ msg.content }}</div>
        <div class="meta">
          {{ msg.role === 'user' ? '你' : (msg.model_id || 'AI') }} · {{ formatTime(msg.created_at) }}
        </div>
      </div>
    </div>
    <div class="input-area glass">
      <input
        v-model="chat.input"
        placeholder="输入问题，由最优免费模型回答..."
        @keyup.enter="send"
      />
      <button @click="send">发送</button>
    </div>
  </main>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick } from "vue";
import { useChatStore } from "../store/chat";
const chat = useChatStore();
const msgContainer = ref<HTMLElement | null>(null);

onMounted(async () => {
  await chat.loadConversations();
  if (chat.conversations.length > 0) {
    chat.activeConvId = chat.conversations[0].id;
    await chat.loadMessages(chat.activeConvId);
  }
});

async function send() {
  const content = chat.input.trim();
  if (!content) return;
  chat.input = "";
  await chat.sendMessage(content);
  scrollToBottom();
}

function scrollToBottom() {
  nextTick(() => {
    if (msgContainer.value) {
      msgContainer.value.scrollTop = msgContainer.value.scrollHeight;
    }
  });
}

function formatTime(dateStr: string): string {
  const d = new Date(dateStr);
  return d.toLocaleTimeString("zh-CN", { hour: "2-digit", minute: "2-digit" });
}
</script>

<style scoped>
.chat-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 20px;
}
.messages {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 16px;
  overflow-y: auto;
}
.message-bubble {
  max-width: 70%;
  padding: 12px 16px;
  border-radius: var(--radius-card);
}
.message-bubble.user {
  align-self: flex-end;
  background: var(--color-accent);
  color: white;
}
.message-bubble.assistant {
  align-self: flex-start;
  background: var(--color-surface);
  backdrop-filter: var(--blur-glass);
}
.meta {
  font-size: 11px;
  opacity: 0.7;
  margin-top: 4px;
}
.input-area {
  display: flex;
  margin-top: 16px;
  padding: 8px;
  border-radius: var(--radius-card);
}
.input-area input {
  flex: 1;
  background: transparent;
  border: none;
  outline: none;
  color: var(--color-text-primary);
  padding: 8px;
}
.input-area button {
  background: var(--color-accent);
  border: none;
  color: white;
  padding: 8px 20px;
  border-radius: 8px;
  cursor: pointer;
}
</style>
