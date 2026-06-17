import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { useTranslation } from 'react-i18next'
import { useSettingsStore } from '../stores/settingsStore'
import { useWhisper } from '../hooks/useWhisper'

export function GeneralSettings() {
  const { t } = useTranslation()
  const { uiLanguage, setUiLanguage, activationMode, setActivationMode, autoStartEnabled, setAutoStartEnabled } = useSettingsStore()

  return (
    <div>
      <h3 className="section-title">General</h3>

      <div className="setting-row">
        <label className="setting-label">Interface Language</label>
        <select value={uiLanguage} onChange={(e) => setUiLanguage(e.target.value)} className="setting-select">
          <option value="en">English</option>
          <option value="es">Español</option>
          <option value="fr">Français</option>
          <option value="de">Deutsch</option>
          <option value="pt">Português</option>
          <option value="it">Italiano</option>
          <option value="ru">Русский</option>
          <option value="ja">日本語</option>
          <option value="zh-CN">简体中文</option>
          <option value="zh-TW">繁體中文</option>
        </select>
      </div>

      <div className="setting-row">
        <label className="setting-label">Activation Mode</label>
        <select value={activationMode} onChange={(e) => setActivationMode(e.target.value)} className="setting-select">
          <option value="tap">Tap to start/stop</option>
          <option value="hold">Hold to speak</option>
        </select>
      </div>

      <div className="setting-row">
        <label className="setting-label">
          <input type="checkbox" checked={autoStartEnabled} onChange={(e) => setAutoStartEnabled(e.target.checked)} />
          <span style={{ marginLeft: 8 }}>Launch on startup</span>
        </label>
      </div>
    </div>
  )
}

export function ModelSettings() {
  const { model, setModel, language, setLanguage } = useSettingsStore()
  const { models, downloaded, fetchModels, downloadModel, deleteModel, loading } = useWhisper()

  useEffect(() => {
    fetchModels()
  }, [fetchModels])

  return (
    <div>
      <h3 className="section-title">Speech-to-Text</h3>

      <div className="setting-row">
        <label className="setting-label">Model</label>
        <select value={model} onChange={(e) => setModel(e.target.value)} className="setting-select">
          <option value="tiny">Tiny (fastest)</option>
          <option value="base">Base (recommended)</option>
          <option value="small">Small</option>
          <option value="medium">Medium</option>
          <option value="large">Large (best quality)</option>
          <option value="turbo">Turbo</option>
        </select>
      </div>

      <div className="setting-row">
        <label className="setting-label">Language</label>
        <select value={language} onChange={(e) => setLanguage(e.target.value)} className="setting-select">
          <option value="auto">Auto-detect</option>
          <option value="en">English</option>
          <option value="es">Spanish</option>
          <option value="fr">French</option>
          <option value="de">German</option>
          <option value="pt">Portuguese</option>
          <option value="it">Italian</option>
          <option value="ru">Russian</option>
          <option value="ja">Japanese</option>
          <option value="zh">Chinese</option>
        </select>
      </div>

      <div className="setting-row">
        <label className="setting-label">Downloaded Models</label>
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: 8 }}>
          {downloaded.length === 0 && <span style={{ color: '#888' }}>None</span>}
          {downloaded.map((m) => (
            <div key={m} style={{ display: 'flex', alignItems: 'center', gap: 4, padding: '4px 8px', background: '#f0f0f0', borderRadius: 4, fontSize: 13 }}>
              <span>{m}</span>
              <button onClick={() => deleteModel(m)} style={{ color: '#dc2626', border: 'none', background: 'none', cursor: 'pointer', fontSize: 13, padding: 0 }}>&times;</button>
            </div>
          ))}
        </div>
      </div>

      <div className="setting-row">
        <label className="setting-label">Available Models</label>
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: 8 }}>
          {['tiny', 'base', 'small', 'medium', 'large', 'turbo'].filter((m) => !downloaded.includes(m)).map((m) => (
            <button key={m} onClick={() => downloadModel(m)} disabled={loading}
              style={{ padding: '4px 12px', fontSize: 13, border: '1px solid #ccc', borderRadius: 4, cursor: loading ? 'default' : 'pointer', opacity: loading ? 0.6 : 1 }}>
              Download {m}
            </button>
          ))}
        </div>
      </div>
    </div>
  )
}

export function ApiKeySettings() {
  const { t } = useTranslation()
  const { apiKeys, saveApiKey, getApiKey } = useSettingsStore()
  const [localKeys, setLocalKeys] = useState({})

  useEffect(() => {
    const providers = ['openai', 'anthropic', 'gemini', 'groq', 'xai', 'mistral']
    providers.forEach(async (p) => {
      const val = await getApiKey(p)
      if (val) setLocalKeys((prev) => ({ ...prev, [p]: val }))
    })
  }, [getApiKey])

  const providers = [
    { id: 'openai', name: 'OpenAI', url: 'https://platform.openai.com/api-keys' },
    { id: 'anthropic', name: 'Anthropic', url: 'https://console.anthropic.com/' },
    { id: 'gemini', name: 'Google Gemini', url: 'https://aistudio.google.com/apikey' },
    { id: 'groq', name: 'Groq', url: 'https://console.groq.com/keys' },
    { id: 'xai', name: 'xAI', url: 'https://console.x.ai/' },
    { id: 'mistral', name: 'Mistral', url: 'https://console.mistral.ai/' },
  ]

  const handleSave = async (id, value) => {
    setLocalKeys((prev) => ({ ...prev, [id]: value }))
    if (value) await saveApiKey(id, value)
  }

  return (
    <div>
      <h3 className="section-title">API Keys</h3>
      {providers.map((p) => (
        <div key={p.id} className="setting-row">
          <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
            <label className="setting-label" style={{ minWidth: 100 }}>{p.name}</label>
            <input
              type="password"
              value={localKeys[p.id] || ''}
              onChange={(e) => handleSave(p.id, e.target.value)}
              placeholder={`Enter ${p.name} API key`}
              className="setting-input"
              style={{ flex: 1 }}
            />
            <a href={p.url} target="_blank" rel="noopener noreferrer" style={{ fontSize: 12, color: '#2563eb' }}>
              Get key
            </a>
          </div>
        </div>
      ))}
    </div>
  )
}

export function DebugSettings() {
  const { debugLogging, setDebugLogging } = useSettingsStore()

  const handleOpenLogs = async () => {
    try {
      await invoke('open_logs_folder')
    } catch {}
  }

  return (
    <div>
      <h3 className="section-title">Debug & Developer</h3>

      <div className="setting-row">
        <label className="setting-label">
          <input type="checkbox" checked={debugLogging} onChange={(e) => setDebugLogging(e.target.checked)} />
          <span style={{ marginLeft: 8 }}>Debug logging</span>
        </label>
      </div>

      <div className="setting-row">
        <button onClick={handleOpenLogs} className="setting-button">
          Open Logs Folder
        </button>
      </div>
    </div>
  )
}

export function HotkeySettings() {
  const { dictationKey, setDictationKey } = useSettingsStore()
  const [listening, setListening] = useState(false)

  const handleStartCapture = () => {
    setListening(true)
  }

  const handleKeyDown = (e) => {
    if (!listening) return
    e.preventDefault()
    const parts = []
    if (e.ctrlKey) parts.push('Ctrl')
    if (e.altKey) parts.push('Alt')
    if (e.shiftKey) parts.push('Shift')
    if (e.metaKey) parts.push('Cmd')
    const key = e.key === ' ' ? 'Space' : e.key
    if (!['Control', 'Alt', 'Shift', 'Meta'].includes(key)) {
      parts.push(key.length === 1 ? key.toUpperCase() : key)
    }
    if (parts.length > 1) {
      const combo = parts.join('+')
      setDictationKey(combo)
      setListening(false)
    }
  }

  useEffect(() => {
    if (listening) {
      document.addEventListener('keydown', handleKeyDown)
      return () => document.removeEventListener('keydown', handleKeyDown)
    }
  }, [listening])

  return (
    <div>
      <h3 className="section-title">Hotkeys</h3>

      <div className="setting-row">
        <label className="setting-label">Dictation Hotkey</label>
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <div style={{ padding: '6px 12px', background: '#f0f0f0', borderRadius: 4, minWidth: 120, textAlign: 'center', fontFamily: 'monospace', fontSize: 14 }}>
            {listening ? 'Press keys...' : dictationKey || 'Alt+R'}
          </div>
          <button onClick={listening ? () => setListening(false) : handleStartCapture}
            className="setting-button">
            {listening ? 'Cancel' : 'Change'}
          </button>
        </div>
      </div>
    </div>
  )
}
