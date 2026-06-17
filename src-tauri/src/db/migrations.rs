use anyhow::Result;
use rusqlite::Connection;

pub fn run(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY
        );

        CREATE TABLE IF NOT EXISTS transcriptions (
            id TEXT PRIMARY KEY,
            client_id TEXT UNIQUE,
            timestamp TEXT NOT NULL DEFAULT (datetime('now')),
            original_text TEXT NOT NULL,
            processed_text TEXT,
            is_processed INTEGER DEFAULT 0,
            processing_method TEXT DEFAULT 'none',
            agent_name TEXT,
            error TEXT,
            audio_duration REAL,
            audio_path TEXT,
            is_synced INTEGER DEFAULT 0,
            cloud_id TEXT,
            sync_error TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS notes (
            id TEXT PRIMARY KEY,
            client_id TEXT UNIQUE,
            title TEXT NOT NULL DEFAULT 'Untitled',
            content TEXT NOT NULL DEFAULT '',
            note_type TEXT NOT NULL DEFAULT 'text',
            source_file TEXT,
            audio_duration REAL,
            folder_id TEXT,
            is_synced INTEGER DEFAULT 0,
            cloud_id TEXT,
            sync_error TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS folders (
            id TEXT PRIMARY KEY,
            client_id TEXT UNIQUE,
            name TEXT NOT NULL,
            is_synced INTEGER DEFAULT 0,
            cloud_id TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS dictionary (
            id TEXT PRIMARY KEY,
            client_id TEXT UNIQUE,
            word TEXT NOT NULL UNIQUE,
            is_synced INTEGER DEFAULT 0,
            cloud_id TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS actions (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            prompt TEXT,
            icon TEXT
        );

        CREATE TABLE IF NOT EXISTS agent_conversations (
            id TEXT PRIMARY KEY,
            title TEXT,
            note_id TEXT,
            is_archived INTEGER DEFAULT 0,
            is_synced INTEGER DEFAULT 0,
            cloud_id TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS agent_messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            conversation_id TEXT NOT NULL REFERENCES agent_conversations(id) ON DELETE CASCADE,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            metadata TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE INDEX IF NOT EXISTS idx_transcriptions_timestamp
            ON transcriptions(timestamp DESC);
        CREATE INDEX IF NOT EXISTS idx_notes_updated
            ON notes(updated_at DESC);
        CREATE INDEX IF NOT EXISTS idx_notes_folder
            ON notes(folder_id);
        CREATE INDEX IF NOT EXISTS idx_agent_messages_conv
            ON agent_messages(conversation_id);

        -- FTS5 for full-text search
        CREATE VIRTUAL TABLE IF NOT EXISTS notes_fts USING fts5(
            title, content, content=notes, content_rowid=rowid
        );

        CREATE VIRTUAL TABLE IF NOT EXISTS transcriptions_fts USING fts5(
            original_text, processed_text, content=transcriptions, content_rowid=rowid
        );

        -- Triggers to keep FTS in sync
        CREATE TRIGGER IF NOT EXISTS notes_ai AFTER INSERT ON notes BEGIN
            INSERT INTO notes_fts(rowid, title, content) VALUES (new.rowid, new.title, new.content);
        END;

        CREATE TRIGGER IF NOT EXISTS notes_ad AFTER DELETE ON notes BEGIN
            INSERT INTO notes_fts(notes_fts, rowid, title, content) VALUES('delete', old.rowid, old.title, old.content);
        END;

        CREATE TRIGGER IF NOT EXISTS notes_au AFTER UPDATE ON notes BEGIN
            INSERT INTO notes_fts(notes_fts, rowid, title, content) VALUES('delete', old.rowid, old.title, old.content);
            INSERT INTO notes_fts(rowid, title, content) VALUES (new.rowid, new.title, new.content);
        END;

        INSERT OR IGNORE INTO schema_version (version) VALUES (1);
        ",
    )?;
    Ok(())
}
