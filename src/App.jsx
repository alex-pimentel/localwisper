import { useState, useEffect, useRef } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useTranslation } from 'react-i18next'
import { useHotkey } from './hooks/useHotkey'
import { useAudioRecording } from './hooks/useAudioRecording'
import { useSettingsStore } from './stores/settingsStore'
import { useTranscriptionStore } from './stores/transcriptionStore'
import './index.css'

function MicIcon({ size = 16 }) {
  return (
    <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2} strokeLinecap="round" strokeLinejoin="round">
      <path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z" />
      <path d="M19 10v2a7 7 0 0 1-14 0v-2" />
      <line x1="12" x2="12" y1="19" y2="22" />
    </svg>
  )
}

function SoundWaveIcon({ size = 16 }) {
  return (
    <div className="flex items-center justify-center gap-[3px]">
      <div className="bg-white rounded-full" style={{ width: size * 0.25, height: size * 0.6 }} />
      <div className="bg-white rounded-full" style={{ width: size * 0.25, height: size }} />
      <div className="bg-white rounded-full" style={{ width: size * 0.25, height: size * 0.6 }} />
    </div>
  )
}

function VoiceWaveIndicator({ isListening }) {
  return (
    <div className="flex items-center justify-center gap-0.5">
      {[...Array(4)].map((_, i) => (
        <div
          key={i}
          className="w-0.5 bg-white rounded-full transition-[height] duration-150"
          style={{
            height: isListening ? 16 : 8,
            animation: isListening ? `pulse ${0.6 + i * 0.1}s ease-in-out infinite` : 'none',
            animationDelay: `${i * 0.1}s`,
          }}
        />
      ))}
    </div>
  )
}

function Tooltip({ children, content, align = 'center' }) {
  const [visible, setVisible] = useState(false)
  const alignClass = align === 'right' ? 'right-0' : align === 'left' ? 'left-0' : 'left-1/2 -translate-x-1/2'

  return (
    <div className="relative inline-block">
      <div onMouseEnter={() => setVisible(true)} onMouseLeave={() => setVisible(false)}>
        {children}
      </div>
      {visible && (
        <div className={`absolute bottom-full ${alignClass} mb-2 px-1.5 py-1 text-[10px] text-popover-foreground bg-popover border border-border rounded-md z-50 shadow-lg whitespace-nowrap`}>
          {content}
        </div>
      )}
    </div>
  )
}

export default function App() {
  const { t } = useTranslation()
  const { hotkey } = useHotkey()
  const { isRecording, toggleRecording } = useAudioRecording()
  const [isHovered, setIsHovered] = useState(false)
  const [isCommandMenuOpen, setIsCommandMenuOpen] = useState(false)
  const [transcriptionHistory, setTranscriptionHistory] = useState([])
  const buttonRef = useRef(null)
  const menuRef = useRef(null)

  const floatingIconAutoHide = useSettingsStore((s) => s.floatingIconAutoHide)
  const panelStartPosition = useSettingsStore((s) => s.panelStartPosition) || 'bottom-right'

  useEffect(() => {
    useTranscriptionStore.getState().init()
    const cleanups = []
    listen('transcription-final', (event) => {
      setTranscriptionHistory((prev) => [event.payload, ...prev].slice(0, 20))
    }).then((fn) => cleanups.push(fn))
    return () => cleanups.forEach((fn) => fn())
  }, [])

  useEffect(() => {
    if (!isCommandMenuOpen) return
    const handler = (e) => {
      if (menuRef.current && !menuRef.current.contains(e.target) &&
          buttonRef.current && !buttonRef.current.contains(e.target)) {
        setIsCommandMenuOpen(false)
      }
    }
    document.addEventListener('mousedown', handler)
    return () => document.removeEventListener('mousedown', handler)
  }, [isCommandMenuOpen])

  useEffect(() => {
    const handler = (e) => {
      if (e.key === 'Escape') {
        if (isCommandMenuOpen) setIsCommandMenuOpen(false)
        else invoke('hide_window').catch(() => {})
      }
    }
    document.addEventListener('keydown', handler)
    return () => document.removeEventListener('keydown', handler)
  }, [isCommandMenuOpen])

  useEffect(() => {
    let timeout
    if (floatingIconAutoHide && !isRecording && transcriptionHistory.length === 0) {
      timeout = setTimeout(() => invoke('hide_window').catch(() => {}), 500)
    }
    return () => clearTimeout(timeout)
  }, [isRecording, floatingIconAutoHide, transcriptionHistory.length])

  const micState = isRecording ? 'recording' : isHovered ? 'hover' : 'idle'

  const getButtonClass = () => {
    const base = 'rounded-full w-10 h-10 flex items-center justify-center relative overflow-hidden border-2 border-white/70 cursor-pointer'
    if (micState === 'recording') return `${base} bg-[#2563eb]`
    return `${base} bg-black/50`
  }

  return (
    <div className="dictation-window">
      <div className={`fixed bottom-1 z-50 ${
        panelStartPosition === 'bottom-left' ? 'left-1' :
        panelStartPosition === 'center' ? 'left-1/2 -translate-x-1/2' : 'right-1'
      }`}>
        <div
          className="relative flex items-center gap-2"
          onMouseEnter={() => setIsHovered(true)}
          onMouseLeave={() => {
            setIsHovered(false)
            if (!isCommandMenuOpen) setIsHovered(false)
          }}
        >
          <Tooltip
            content={isRecording ? t('app.mic.recording') : hotkey}
            align={panelStartPosition === 'bottom-left' ? 'left' : panelStartPosition === 'center' ? 'center' : 'right'}
          >
            <button
              ref={buttonRef}
              onClick={() => toggleRecording()}
              onContextMenu={(e) => {
                e.preventDefault()
                setIsCommandMenuOpen((prev) => !prev)
              }}
              className={getButtonClass()}
            >
              {micState === 'idle' || micState === 'hover' ? (
                <MicIcon size={micState === 'idle' ? 14 : 16} />
              ) : micState === 'recording' ? (
                <VoiceWaveIndicator isListening={true} />
              ) : null}

              {micState === 'recording' && (
                <div className="absolute inset-0 rounded-full border-2 border-primary/50 animate-pulse" />
              )}
            </button>
          </Tooltip>

          {isCommandMenuOpen && (
            <div
              ref={menuRef}
              className="absolute bottom-full right-0 mb-3 w-48 rounded-lg border border-border bg-popover text-popover-foreground shadow-lg"
            >
              <button
                className="w-full px-3 py-2 text-left text-sm font-medium hover:bg-muted focus:bg-muted focus:outline-none"
                onClick={() => {
                  toggleRecording()
                  setIsCommandMenuOpen(false)
                }}
              >
                {isRecording ? t('app.commandMenu.stopListening') : t('app.commandMenu.startListening')}
              </button>
              <div className="h-px bg-border" />
              <button
                className="w-full px-3 py-2 text-left text-sm hover:bg-muted focus:bg-muted focus:outline-none"
                onClick={() => {
                  setIsCommandMenuOpen(false)
                  invoke('hide_window').catch(() => {})
                }}
              >
                {t('app.commandMenu.hideForNow')}
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
