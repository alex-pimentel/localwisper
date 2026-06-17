import { create } from 'zustand'
import { invoke } from '@tauri-apps/api/core'

const API_KEY_METHODS = [
  'get_openai_key', 'save_openai_key',
  'get_anthropic_key', 'save_anthropic_key',
  'get_gemini_key', 'save_gemini_key',
  'get_groq_key', 'save_groq_key',
  'get_xai_key', 'save_xai_key',
  'get_mistral_key', 'save_mistral_key',
]

export const useSettingsStore = create((set, get) => ({
  model: 'base',
  language: 'auto',
  agentName: '',
  uiLanguage: 'en',
  hotkey: null,
  activationMode: 'tap',
  autoStartEnabled: false,
  debugLogging: false,
  autoLearnEnabled: false,
  hasCompletedOnboarding: false,
  dictationKey: null,
  agentKey: null,
  voiceAgentKey: null,
  apiKeys: {},
  whisperVadConfig: { threshold: 0.5, minSpeechDurationMs: 100, minSilenceDurationMs: 500 },
  floatingIconAutoHide: false,
  panelStartPosition: 'bottom-right',
  loading: true,

  hydrate: async () => {
    try {
      const [uiLanguage, activationMode] = await Promise.all([
        invoke('get_ui_language').catch(() => 'en'),
        invoke('get_activation_mode').catch(() => 'tap'),
      ])
      set({ uiLanguage: uiLanguage || 'en', activationMode: activationMode || 'tap', loading: false })
    } catch {
      set({ loading: false })
    }
  },

  setModel: (model) => set({ model }),
  setLanguage: (language) => set({ language }),
  setAgentName: (agentName) => set({ agentName }),
  setUiLanguage: async (language) => {
    set({ uiLanguage: language })
    await invoke('set_ui_language', { _language: language }).catch(() => {})
  },
  setHotkey: (hotkey) => set({ hotkey }),
  setActivationMode: async (mode) => {
    set({ activationMode: mode })
    await invoke('save_activation_mode', { _mode: mode }).catch(() => {})
  },
  setAutoStartEnabled: async (enabled) => {
    set({ autoStartEnabled: enabled })
    await invoke('set_auto_start_enabled', { _enabled: enabled }).catch(() => {})
  },
  setDebugLogging: async (enabled) => {
    set({ debugLogging: enabled })
    await invoke('set_debug_logging', { _enabled: enabled }).catch(() => {})
  },
  setAutoLearnEnabled: async (enabled) => {
    set({ autoLearnEnabled: enabled })
  },
  setHasCompletedOnboarding: (val) => set({ hasCompletedOnboarding: val }),

  saveApiKey: async (keyName, value) => {
    const method = `save_${keyName}_key`
    await invoke(method, { key: value, value }).catch(() => {})
    set((s) => ({ apiKeys: { ...s.apiKeys, [keyName]: value } }))
  },

  getApiKey: async (keyName) => {
    const method = `get_${keyName}_key`
    const value = await invoke(method).catch(() => null)
    if (value) {
      set((s) => ({ apiKeys: { ...s.apiKeys, [keyName]: value } }))
    }
    return value
  },

  setDictationKey: async (key) => {
    set({ dictationKey: key })
    await invoke('save_dictation_key', { _key: key }).catch(() => {})
  },
  setAgentKey: async (key) => {
    set({ agentKey: key })
    await invoke('save_agent_key', { _key: key }).catch(() => {})
  },
  setVoiceAgentKey: async (key) => {
    set({ voiceAgentKey: key })
    await invoke('save_voice_agent_key', { _key: key }).catch(() => {})
  },
}))
