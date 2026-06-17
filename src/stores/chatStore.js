import { create } from 'zustand'
import { invoke } from '@tauri-apps/api/core'

export const useChatStore = create((set, get) => ({
  conversations: [],
  currentConversation: null,
  messages: [],
  loading: false,

  fetchConversations: async (limit = 50) => {
    set({ loading: true })
    try {
      const conversations = await invoke('get_agent_conversations', { limit })
      set({ conversations, loading: false })
    } catch {
      set({ loading: false })
    }
  },

  getConversation: async (id) => {
    const conversation = await invoke('get_agent_conversation', { id })
    set({ currentConversation: conversation })
    return conversation
  },

  createConversation: async (title = null, noteId = null) => {
    const conversation = await invoke('create_agent_conversation', { title, noteId })
    set((s) => ({ conversations: [conversation, ...s.conversations] }))
    return conversation
  },

  deleteConversation: async (id) => {
    await invoke('delete_agent_conversation', { id })
    set((s) => ({
      conversations: s.conversations.filter((c) => c.id !== id),
      currentConversation: s.currentConversation?.id === id ? null : s.currentConversation,
    }))
  },

  updateTitle: async (id, title) => {
    return invoke('update_agent_conversation_title', { id, title })
  },

  fetchMessages: async (conversationId) => {
    const messages = await invoke('get_agent_messages', { conversationId })
    set({ messages })
    return messages
  },

  addMessage: async (conversationId, role, content, metadata = null) => {
    const message = await invoke('add_agent_message', { conversationId, role, content, metadata })
    set((s) => ({ messages: [...s.messages, message] }))
    return message
  },

  archiveConversation: async (id) => {
    return invoke('archive_agent_conversation', { id })
  },

  unarchiveConversation: async (id) => {
    return invoke('unarchive_agent_conversation', { id })
  },

  searchConversations: async (query, limit = 20) => {
    const conversations = await invoke('search_agent_conversations', { query, limit })
    return conversations
  },
}))
