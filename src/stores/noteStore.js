import { create } from 'zustand'
import { invoke } from '@tauri-apps/api/core'

export const useNoteStore = create((set, get) => ({
  notes: [],
  currentNote: null,
  loading: false,

  fetchNotes: async (noteType = null, limit = 50, folderId = null) => {
    set({ loading: true })
    try {
      const notes = await invoke('get_notes', { noteType, limit, folderId })
      set({ notes, loading: false })
    } catch {
      set({ loading: false })
    }
  },

  getNote: async (id) => {
    const note = await invoke('get_note', { id })
    set({ currentNote: note })
    return note
  },

  createNote: async (title, content, noteType = 'text', folderId = null) => {
    const note = await invoke('create_note', { title, content, noteType, folderId })
    set((s) => ({ notes: [note, ...s.notes] }))
    return note
  },

  updateNote: async (id, title, content) => {
    const result = await invoke('update_note', { id, title, content })
    if (result) {
      set((s) => ({
        notes: s.notes.map((n) => n.id === id ? { ...n, title: title || n.title, content: content || n.content } : n),
      }))
    }
    return result
  },

  deleteNote: async (id) => {
    const result = await invoke('delete_note', { id })
    if (result) {
      set((s) => ({ notes: s.notes.filter((n) => n.id !== id) }))
    }
    return result
  },

  searchNotes: async (query, limit = 20) => {
    const notes = await invoke('search_notes', { query, limit })
    set({ notes })
    return notes
  },

  semanticSearch: async (query, limit = 20) => {
    return invoke('semantic_search_notes', { query, limit })
  },

  exportNote: async (noteId, format = 'txt') => {
    return invoke('export_note', { noteId, format })
  },
}))
