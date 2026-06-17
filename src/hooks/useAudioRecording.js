import { invoke } from '@tauri-apps/api/core'
import { useSettingsStore } from '../stores/settingsStore'
import { useTranscriptionStore } from '../stores/transcriptionStore'
import { useCallback } from 'react'

export function useAudioRecording() {
  const isRecording = useTranscriptionStore((s) => s.isRecording)
  const startRecording = useTranscriptionStore((s) => s.startRecording)
  const stopRecording = useTranscriptionStore((s) => s.stopRecording)

  const toggleRecording = useCallback(async () => {
    if (isRecording) {
      await stopRecording()
    } else {
      await startRecording()
    }
  }, [isRecording, startRecording, stopRecording])

  return { isRecording, toggleRecording, startRecording, stopRecording }
}
