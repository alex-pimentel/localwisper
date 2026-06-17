import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export function useUpdater() {
  const [updateAvailable, setUpdateAvailable] = useState(false)
  const [updateDownloaded, setUpdateDownloaded] = useState(false)
  const [downloadProgress, setDownloadProgress] = useState(0)
  const [checking, setChecking] = useState(false)

  const checkForUpdates = useCallback(async () => {
    setChecking(true)
    try {
      const result = await invoke('check_for_updates')
      setUpdateAvailable(result?.updateAvailable || false)
      return result
    } catch {
      return null
    } finally {
      setChecking(false)
    }
  }, [])

  const downloadUpdate = useCallback(async () => {
    try {
      const unlisten = await listen('update-download-progress', (event) => {
        setDownloadProgress(event.payload?.percent || 0)
      })
      await invoke('download_update')
      setUpdateDownloaded(true)
      unlisten()
    } catch {
      // download failed
    }
  }, [])

  const installUpdate = useCallback(async () => {
    try {
      await invoke('install_update')
    } catch {
      // install failed
    }
  }, [])

  return {
    updateAvailable,
    updateDownloaded,
    downloadProgress,
    checking,
    checkForUpdates,
    downloadUpdate,
    installUpdate,
  }
}
