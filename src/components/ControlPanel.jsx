import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { useTranslation } from 'react-i18next'
import { useNoteStore } from '../stores/noteStore'
import { useTranscriptionStore } from '../stores/transcriptionStore'
import { useChatStore } from '../stores/chatStore'
import { GeneralSettings, ModelSettings, ApiKeySettings, DebugSettings, HotkeySettings } from './SettingsPage'

const tabs = [
  { id: 'dictation', label: 'Dictation' },
  { id: 'notes', label: 'Notes' },
  { id: 'chat', label: 'Agent Chat' },
  { id: 'general', label: 'General' },
  { id: 'models', label: 'Models' },
  { id: 'hotkeys', label: 'Hotkeys' },
  { id: 'api-keys', label: 'API Keys' },
  { id: 'developer', label: 'Developer' },
]

export default function ControlPanel() {
  const { t } = useTranslation()
  const [activeTab, setActiveTab] = useState('dictation')

  return (
    <div className="control-panel">
      <header className="cp-header">
        <h1 className="cp-title">Lightwisper</h1>
      </header>

      <nav className="cp-nav">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className={`cp-nav-btn ${activeTab === tab.id ? 'active' : ''}`}
          >
            {tab.label}
          </button>
        ))}
      </nav>

      <main className="cp-content">
        {activeTab === 'dictation' && <DictationTab />}
        {activeTab === 'notes' && <NotesTab />}
        {activeTab === 'chat' && <ChatTab />}
        {activeTab === 'general' && <GeneralSettings />}
        {activeTab === 'models' && <ModelSettings />}
        {activeTab === 'hotkeys' && <HotkeySettings />}
        {activeTab === 'api-keys' && <ApiKeySettings />}
        {activeTab === 'developer' && <DebugSettings />}
      </main>
    </div>
  )
}

function DictationTab() {
  const { transcriptions, fetchTranscriptions, deleteTranscription, clearTranscriptions } = useTranscriptionStore()

  useEffect(() => {
    fetchTranscriptions(50)
  }, [fetchTranscriptions])

  return (
    <div>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 16 }}>
        <h2 className="section-title" style={{ margin: 0 }}>Transcriptions</h2>
        {transcriptions.length > 0 && (
          <button onClick={clearTranscriptions} className="setting-button" style={{ color: '#dc2626', fontSize: 12 }}>
            Clear All
          </button>
        )}
      </div>
      {transcriptions.length === 0 && <p style={{ color: '#888' }}>No transcriptions yet. Start dictating!</p>}
      {transcriptions.map((t) => (
        <div key={t.id} className="history-item">
          <div className="history-meta">
            <span className="history-time">{t.timestamp}</span>
            {t.agent_name && <span className="history-agent">{t.agent_name}</span>}
          </div>
          <p className="history-text">{t.original_text}</p>
          {t.processed_text && t.processed_text !== t.original_text && (
            <p className="history-processed">{t.processed_text}</p>
          )}
          <div className="history-actions">
            <button onClick={async () => {
              try {
                await navigator.clipboard.writeText(t.processed_text || t.original_text)
              } catch {}
            }} className="history-btn">Copy</button>
            <button onClick={() => deleteTranscription(t.id)} className="history-btn danger">Delete</button>
          </div>
        </div>
      ))}
    </div>
  )
}

function NotesTab() {
  const { notes, fetchNotes, createNote, updateNote, deleteNote } = useNoteStore()
  const [title, setTitle] = useState('')
  const [content, setContent] = useState('')
  const [editingId, setEditingId] = useState(null)

  useEffect(() => {
    fetchNotes()
  }, [fetchNotes])

  const handleCreate = async () => {
    if (!title.trim()) return
    await createNote(title, content)
    setTitle('')
    setContent('')
  }

  const handleUpdate = async () => {
    if (!editingId || !title.trim()) return
    await updateNote(editingId, title, content)
    setEditingId(null)
    setTitle('')
    setContent('')
    await fetchNotes()
  }

  const handleEdit = (note) => {
    setEditingId(note.id)
    setTitle(note.title)
    setContent(note.content || '')
  }

  const handleCancel = () => {
    setEditingId(null)
    setTitle('')
    setContent('')
  }

  return (
    <div>
      <h2 className="section-title" style={{ margin: 0 }}>Notes</h2>

      <div className="note-form">
        <input
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          placeholder="Note title..."
          className="setting-input"
          onKeyDown={(e) => e.key === 'Enter' && (editingId ? handleUpdate() : handleCreate())}
        />
        <textarea
          value={content}
          onChange={(e) => setContent(e.target.value)}
          placeholder="Note content..."
          className="setting-textarea"
          rows={3}
        />
        <div style={{ display: 'flex', gap: 8 }}>
          <button onClick={editingId ? handleUpdate : handleCreate} className="setting-button primary">
            {editingId ? 'Update' : 'Create'}
          </button>
          {editingId && (
            <button onClick={handleCancel} className="setting-button">Cancel</button>
          )}
        </div>
      </div>

      {notes.length === 0 && <p style={{ color: '#888', marginTop: 16 }}>No notes yet.</p>}
      {notes.map((note) => (
        <div key={note.id} className="history-item">
          <h3 style={{ fontWeight: 600, fontSize: 14, marginBottom: 4 }}>{note.title}</h3>
          <p style={{ color: '#666', fontSize: 13, whiteSpace: 'pre-wrap', wordBreak: 'break-word' }}>
            {(note.content || '').slice(0, 200)}
          </p>
          <div className="history-actions">
            <button onClick={() => handleEdit(note)} className="history-btn">Edit</button>
            <button onClick={() => deleteNote(note.id)} className="history-btn danger">Delete</button>
          </div>
        </div>
      ))}
    </div>
  )
}

function ChatTab() {
  const { conversations, fetchConversations, createConversation, deleteConversation, messages, fetchMessages } = useChatStore()
  const [selectedId, setSelectedId] = useState(null)
  const [input, setInput] = useState('')

  useEffect(() => {
    fetchConversations()
  }, [fetchConversations])

  const handleSelect = async (id) => {
    setSelectedId(id)
    await fetchMessages(id)
  }

  const handleNew = async () => {
    const c = await createConversation('New conversation')
    if (c) {
      setSelectedId(c.id)
      await fetchMessages(c.id)
    }
  }

  const handleSend = async () => {
    if (!input.trim() || !selectedId) return
    setInput('')
    await invoke('send_agent_message', { conversationId: selectedId, message: input })
    await fetchMessages(selectedId)
  }

  return (
    <div style={{ display: 'flex', gap: 16, height: 'calc(100vh - 200px)' }}>
      <div style={{ width: 240, flexShrink: 0, overflowY: 'auto', borderRight: '1px solid #e5e5e5', paddingRight: 12 }}>
        <button onClick={handleNew} className="setting-button primary" style={{ width: '100%', marginBottom: 12 }}>
          + New Chat
        </button>
        {conversations.map((c) => (
          <div
            key={c.id}
            onClick={() => handleSelect(c.id)}
            className={`history-item ${selectedId === c.id ? 'selected' : ''}`}
            style={{ cursor: 'pointer', padding: '8px 12px', background: selectedId === c.id ? '#f0f0f0' : undefined }}
          >
            <p style={{ fontWeight: 500, fontSize: 13, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
              {c.title || 'Untitled'}
            </p>
            <button onClick={(e) => { e.stopPropagation(); deleteConversation(c.id) }}
              className="history-btn danger" style={{ fontSize: 11 }}>Delete</button>
          </div>
        ))}
      </div>

      <div style={{ flex: 1, display: 'flex', flexDirection: 'column' }}>
        <div style={{ flex: 1, overflowY: 'auto', marginBottom: 12 }}>
          {!selectedId && <p style={{ color: '#888' }}>Select a conversation or start a new one.</p>}
          {messages.map((m, i) => (
            <div key={m.id || i} style={{ marginBottom: 12, padding: 12, background: m.role === 'user' ? '#f0f7ff' : '#f9f9f9', borderRadius: 8 }}>
              <strong style={{ fontSize: 12, color: '#888', textTransform: 'uppercase' }}>{m.role}</strong>
              <p style={{ marginTop: 4, fontSize: 14, whiteSpace: 'pre-wrap' }}>{m.content}</p>
            </div>
          ))}
        </div>

        <div style={{ display: 'flex', gap: 8 }}>
          <input
            value={input}
            onChange={(e) => setInput(e.target.value)}
            placeholder="Type a message..."
            className="setting-input"
            onKeyDown={(e) => e.key === 'Enter' && handleSend()}
            disabled={!selectedId}
          />
          <button onClick={handleSend} className="setting-button primary" disabled={!selectedId || !input.trim()}>
            Send
          </button>
        </div>
      </div>
    </div>
  )
}
