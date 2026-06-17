import { invoke } from '@tauri-apps/api/core'
import { useState, useCallback, useEffect } from 'react'

export function useWhisper() {
  const [models, setModels] = useState([])
  const [downloaded, setDownloaded] = useState([])
  const [loading, setLoading] = useState(false)

  const fetchModels = useCallback(async () => {
    try {
      const allModels = await invoke('list_whisper_models')
      setModels(allModels)
      const dlModels = await invoke('list_downloaded_models')
      setDownloaded(dlModels)
    } catch (e) {
      console.error('fetchModels error:', e)
    }
  }, [])

  const downloadModel = useCallback(async (modelName) => {
    setLoading(true)
    try {
      await invoke('download_whisper_model', { modelName })
      await fetchModels()
    } catch (e) {
      console.error('download error:', e)
    }
    setLoading(false)
  }, [fetchModels])

  const deleteModel = useCallback(async (modelName) => {
    await invoke('delete_whisper_model', { modelName })
    await fetchModels()
  }, [fetchModels])

  const checkInstallation = useCallback(async () => {
    return invoke('check_whisper_installation')
  }, [])

  const checkFfmpeg = useCallback(async () => {
    return invoke('check_ffmpeg_availability')
  }, [])

  useEffect(() => { fetchModels() }, [fetchModels])

  return { models, downloaded, loading, fetchModels, downloadModel, deleteModel, checkInstallation, checkFfmpeg }
}
