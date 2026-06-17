import { useSettingsStore } from '../stores/settingsStore'

export function useHotkey() {
  const dictationKey = useSettingsStore((s) => s.dictationKey)
  const setDictationKey = useSettingsStore((s) => s.setDictationKey)
  const agentKey = useSettingsStore((s) => s.agentKey)
  const voiceAgentKey = useSettingsStore((s) => s.voiceAgentKey)

  return {
    hotkey: dictationKey || 'Alt+R',
    dictationKey,
    agentKey,
    voiceAgentKey,
    setDictationKey,
    setAgentKey: setDictationKey,
    setVoiceAgentKey: useSettingsStore((s) => s.setVoiceAgentKey),
  }
}
