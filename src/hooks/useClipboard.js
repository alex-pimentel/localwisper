import { invoke } from '@tauri-apps/api/core'
import { useState, useCallback } from 'react'

export function useClipboard() {
  const [error, setError] = useState(null)

  const readClipboard = useCallback(async () => {
    try {
      const text = await invoke('read_clipboard')
      return text
    } catch (e) {
      setError(e)
      return ''
    }
  }, [])

  const writeClipboard = useCallback(async (text) => {
    try {
      await invoke('write_clipboard', { text })
      setError(null)
    } catch (e) {
      setError(e)
    }
  }, [])

  const checkPasteTools = useCallback(async () => {
    try {
      return await invoke('check_paste_tools')
    } catch {
      return []
    }
  }, [])

  return { readClipboard, writeClipboard, checkPasteTools, error }
}
