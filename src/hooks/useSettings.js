import { invoke } from '@tauri-apps/api/core'
import { useSettingsStore } from '../stores/settingsStore'

export function useSettings() {
  const store = useSettingsStore()

  return {
    ...store,
    refreshApiKey: async (keyName) => {
      return store.getApiKey(keyName)
    },
    loadAllApiKeys: async () => {
      const providers = ['openai', 'anthropic', 'gemini', 'groq', 'xai', 'mistral']
      await Promise.all(providers.map((p) => store.getApiKey(p).catch(() => null)))
    },
  }
}
