import { create } from 'zustand'
import { invoke } from '@tauri-apps/api/core'

export const useStreamingProvidersStore = create((set, get) => ({
  assemblyAiStatus: 'inactive',
  deepgramStatus: 'inactive',
  cortiStatus: 'inactive',

  assemblyAiStreamingStart: async (options = {}) => {
    await invoke('assembly_ai_streaming_start', { options: JSON.stringify(options) })
    set({ assemblyAiStatus: 'active' })
  },
  assemblyAiStreamingStop: async () => {
    await invoke('assembly_ai_streaming_stop')
    set({ assemblyAiStatus: 'inactive' })
  },
  assemblyAiStreamingStatus: async () => {
    const status = await invoke('assembly_ai_streaming_status')
    set({ assemblyAiStatus: status })
    return status
  },

  deepgramStreamingStart: async (options = {}) => {
    await invoke('deepgram_streaming_start', { options: JSON.stringify(options) })
    set({ deepgramStatus: 'active' })
  },
  deepgramStreamingStop: async () => {
    await invoke('deepgram_streaming_stop')
    set({ deepgramStatus: 'inactive' })
  },
  deepgramStreamingStatus: async () => {
    const status = await invoke('deepgram_streaming_status')
    set({ deepgramStatus: status })
    return status
  },

  cortiStreamingStart: async (options = {}) => {
    await invoke('corti_streaming_start', { options: JSON.stringify(options) })
    set({ cortiStatus: 'active' })
  },
  cortiStreamingStop: async () => {
    await invoke('corti_streaming_stop')
    set({ cortiStatus: 'inactive' })
  },
  cortiStreamingStatus: async () => {
    const status = await invoke('corti_streaming_status')
    set({ cortiStatus: status })
    return status
  },
}))
