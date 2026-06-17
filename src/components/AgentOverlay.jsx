import { useState, useEffect, useRef } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useTranslation } from 'react-i18next'
import { useChatStore } from '../stores/chatStore'

export default function AgentOverlay() {
  const { t } = useTranslation()
  const { conversations, fetchConversations, createConversation, messages, fetchMessages, addMessage } = useChatStore()
  const [selectedId, setSelectedId] = useState(null)
  const [input, setInput] = useState('')
  const [transcribing, setTranscribing] = useState(false)
  const messagesEndRef = useRef(null)

  useEffect(() => {
    fetchConversations()
  }, [fetchConversations])

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [messages])

  useEffect(() => {
    let unlisten
    listen('voice-agent-result', (event) => {
      if (selectedId && event.payload) {
        addMessage(selectedId, 'assistant', event.payload)
        fetchMessages(selectedId)
      }
    }).then((fn) => { unlisten = fn })
    return () => { unlisten?.() }
  }, [selectedId])

  const handleNew = async () => {
    const c = await createConversation()
    if (c) {
      setSelectedId(c.id)
      await fetchMessages(c.id)
    }
  }

  const handleSelect = async (id) => {
    setSelectedId(id)
    await fetchMessages(id)
  }

  const handleSend = async () => {
    if (!input.trim() || !selectedId) return
    setTranscribing(true)
    try {
      await invoke('send_agent_message', { conversationId: selectedId, message: input })
      await fetchMessages(selectedId)
    } catch {
      // error
    }
    setTranscribing(false)
    setInput('')
  }

  const handleVoiceInput = async () => {
    if (!selectedId) return
    setTranscribing(true)
    try {
      const text = await invoke('start_voice_agent_dictation', { conversationId: selectedId })
      if (text) {
        await invoke('send_agent_message', { conversationId: selectedId, message: text })
        await fetchMessages(selectedId)
      }
    } catch {
      // voice input cancelled or failed
    }
    setTranscribing(false)
  }

  return (
    <div className="agent-overlay">
      <header className="agent-header">
        <h2 className="agent-title">Agent Chat</h2>
        <button onClick={handleNew} className="agent-new-btn">New</button>
      </header>

      <div className="agent-body">
        <aside className="agent-sidebar">
          {conversations.map((c) => (
            <div
              key={c.id}
              onClick={() => handleSelect(c.id)}
              className={`agent-conv-item ${selectedId === c.id ? 'active' : ''}`}
            >
              <span className="agent-conv-title">{c.title || 'Untitled'}</span>
            </div>
          ))}
        </aside>

        <main className="agent-chat">
          {!selectedId ? (
            <div className="agent-empty">
              <p>Select a conversation or start a new one.</p>
            </div>
          ) : (
            <>
              <div className="agent-messages">
                {messages.map((m, i) => (
                  <div key={m.id || i} className={`agent-msg agent-msg-${m.role}`}>
                    <div className="agent-msg-role">{m.role}</div>
                    <div className="agent-msg-content">{m.content}</div>
                  </div>
                ))}
                {transcribing && <div className="agent-msg agent-msg-assistant"><em>Transcribing...</em></div>}
                <div ref={messagesEndRef} />
              </div>

              <div className="agent-input-area">
                <input
                  value={input}
                  onChange={(e) => setInput(e.target.value)}
                  placeholder="Type a message..."
                  className="agent-input"
                  onKeyDown={(e) => e.key === 'Enter' && !e.shiftKey && handleSend()}
                  disabled={transcribing}
                />
                <button onClick={handleVoiceInput} disabled={transcribing} className="agent-mic-btn" title="Voice input">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                    <path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z" />
                    <path d="M19 10v2a7 7 0 0 1-14 0v-2" />
                    <line x1="12" x2="12" y1="19" y2="22" />
                  </svg>
                </button>
                <button onClick={handleSend} disabled={!input.trim() || transcribing} className="agent-send-btn">
                  Send
                </button>
              </div>
            </>
          )}
        </main>
      </div>
    </div>
  )
}
