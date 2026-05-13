import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export interface Conversation {
  id: number;
  title: string;
  model_id: string;
  updated_at: string;
  is_starred: boolean;
}

export interface Message {
  id: number;
  conversation_id: number;
  role: string;
  content: string;
  model_id?: string;
  created_at: string;
}

export const useChatStore = defineStore("chat", () => {
  const conversations = ref<Conversation[]>([]);
  const activeConvId = ref<number | null>(null);
  const messages = ref<Message[]>([]);
  const input = ref("");

  async function loadConversations() {
    const result = await invoke<Conversation[]>("get_conversations");
    conversations.value = result;
  }

  async function loadMessages(convId: number) {
    const result = await invoke<Message[]>("get_messages", { convId });
    messages.value = result;
  }

  async function newConversation(title: string, modelId: string) {
    const conv = await invoke<Conversation>("new_conversation", {
      title,
      modelId,
    });
    conversations.value.unshift(conv);
    activeConvId.value = conv.id;
    messages.value = [];
    return conv;
  }

  async function sendMessage(content: string) {
    if (!activeConvId.value) {
      await newConversation("新对话", "auto");
    }
    const convId = activeConvId.value!;
    // 发送用户消息
    await invoke<Message>("send_message", {
      convId,
      content,
    });
    await loadMessages(convId);
  }

  return {
    conversations,
    activeConvId,
    messages,
    input,
    loadConversations,
    loadMessages,
    newConversation,
    sendMessage,
  };
});
