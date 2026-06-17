import { create } from 'zustand'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export const useTranscriptionStore = create((set, get) => ({
  transcriptions: [],
  currentTranscript: '',
  isRecording: false,
  loading: false,

  init: () => {
    const cleanups = []
    listen('transcription-segment', (event) => {
      set({ currentTranscript: event.payload })
    }).then(fn => cleanups.push(fn))
    listen('transcription-final', (event) => {
      set({ currentTranscript: event.payload })
    }).then(fn => cleanups.push(fn))
    listen('transcription-error', (event) => {
      console.error('transcription error:', event.payload)
    }).then(fn => cleanups.push(fn))
    return () => cleanups.forEach(fn => fn())
  },

  setCurrentTranscript: (text) => set({ currentTranscript: text }),

  startRecording: async () => {
    await invoke('start_dictation')
    set({ isRecording: true, currentTranscript: '' })
  },

  stopRecording: async () => {
    await invoke('stop_dictation')
    set({ isRecording: false })
  },

  getStatus: async () => {
    const status = await invoke('get_recording_status')
    set({ isRecording: status })
    return status
  },

  fetchTranscriptions: async (limit = 50) => {
    set({ loading: true })
    try {
      const items = await invoke('get_transcriptions', { limit })
      set({ transcriptions: items, loading: false })
    } catch {
      set({ loading: false })
    }
  },

  saveTranscription: async (text, rawText = null, agentName = null) => {
    const t = await invoke('save_transcription', {
      text,
      rawText: rawText || text,
      agentName,
    })
    set((s) => ({ transcriptions: [t, ...s.transcriptions] }))
    return t
  },

  deleteTranscription: async (id) => {
    await invoke('delete_transcription', { id })
    set((s) => ({ transcriptions: s.transcriptions.filter((t) => t.id !== id) }))
  },

  clearTranscriptions: async () => {
    await invoke('clear_transcriptions')
    set({ transcriptions: [] })
  },

  searchTranscriptions: async (query, limit = 20) => {
    const items = await invoke('search_transcriptions', { query, limit })
    set({ transcriptions: items })
    return items
  },

  updateText: async (id, text, rawText) => {
    return invoke('update_transcription_text', { id, text, rawText })
  },
}))
