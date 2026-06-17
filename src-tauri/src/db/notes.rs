use anyhow::Result;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub note_type: String,
    pub source_file: Option<String>,
    pub audio_duration: Option<f64>,
    pub folder_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Database {
    pub fn save_note(
        &self,
        title: &str,
        content: &str,
        note_type: &str,
        folder_id: Option<&str>,
    ) -> Result<Note> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let client_id = Uuid::new_v4().to_string();

        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO notes (id, client_id, title, content, note_type, folder_id, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![id, client_id, title, content, note_type, folder_id, now, now],
            )?;
            Ok(())
        })?;

        Ok(Note {
            id,
            title: title.to_string(),
            content: content.to_string(),
            note_type: note_type.to_string(),
            source_file: None,
            audio_duration: None,
            folder_id: folder_id.map(|s| s.to_string()),
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn get_notes(
        &self,
        note_type: Option<&str>,
        limit: i64,
        folder_id: Option<&str>,
    ) -> Result<Vec<Note>> {
        self.with_conn(|conn| {
            let (sql, field_params): (String, Vec<Box<dyn rusqlite::types::ToSql>>) = match (note_type, folder_id) {
                (Some(nt), Some(fid)) => (
                    "SELECT id, title, content, note_type, source_file, audio_duration, folder_id, created_at, updated_at
                     FROM notes WHERE note_type = ?1 AND folder_id = ?2 ORDER BY updated_at DESC LIMIT ?3"
                        .to_string(),
                    vec![Box::new(nt.to_string()), Box::new(fid.to_string()), Box::new(limit)],
                ),
                (Some(nt), None) => (
                    "SELECT id, title, content, note_type, source_file, audio_duration, folder_id, created_at, updated_at
                     FROM notes WHERE note_type = ?1 ORDER BY updated_at DESC LIMIT ?2"
                        .to_string(),
                    vec![Box::new(nt.to_string()), Box::new(limit)],
                ),
                (None, Some(fid)) => (
                    "SELECT id, title, content, note_type, source_file, audio_duration, folder_id, created_at, updated_at
                     FROM notes WHERE folder_id = ?1 ORDER BY updated_at DESC LIMIT ?2"
                        .to_string(),
                    vec![Box::new(fid.to_string()), Box::new(limit)],
                ),
                (None, None) => (
                    "SELECT id, title, content, note_type, source_file, audio_duration, folder_id, created_at, updated_at
                     FROM notes ORDER BY updated_at DESC LIMIT ?1"
                        .to_string(),
                    vec![Box::new(limit)],
                ),
            };

            let mut stmt = conn.prepare(&sql)?;
            let param_refs: Vec<&dyn rusqlite::types::ToSql> = field_params.iter().map(|p| p.as_ref()).collect();
            let rows = stmt.query_map(param_refs.as_slice(), |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    note_type: row.get(3)?,
                    source_file: row.get(4)?,
                    audio_duration: row.get(5)?,
                    folder_id: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
        })
    }

    pub fn update_note(&self, id: &str, title: Option<&str>, content: Option<&str>) -> Result<bool> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.with_conn(|conn| {
            if let Some(t) = title {
                conn.execute(
                    "UPDATE notes SET title = ?1, updated_at = ?2 WHERE id = ?3",
                    params![t, now, id],
                )?;
            }
            if let Some(c) = content {
                conn.execute(
                    "UPDATE notes SET content = ?1, updated_at = ?2 WHERE id = ?3",
                    params![c, now, id],
                )?;
            }
            Ok(true)
        })
    }

    pub fn delete_note(&self, id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
            Ok(affected > 0)
        })
    }

    pub fn get_note(&self, id: &str) -> Result<Option<Note>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, title, content, note_type, source_file, audio_duration, folder_id, created_at, updated_at
                 FROM notes WHERE id = ?1",
            )?;
            let mut rows = stmt.query_map(params![id], |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    note_type: row.get(3)?,
                    source_file: row.get(4)?,
                    audio_duration: row.get(5)?,
                    folder_id: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })?;
            Ok(rows.next().transpose()?)
        })
    }

    pub fn update_note_cloud_id(&self, id: &str, cloud_id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute(
                "UPDATE notes SET cloud_id = ?1 WHERE id = ?2",
                params![cloud_id, id],
            )?;
            Ok(affected > 0)
        })
    }

    pub fn get_pending_notes(&self) -> Result<Vec<Note>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, title, content, note_type, source_file, audio_duration, folder_id, created_at, updated_at
                 FROM notes WHERE is_synced = 0 ORDER BY updated_at DESC"
            )?;
            let rows = stmt.query_map([], |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    note_type: row.get(3)?,
                    source_file: row.get(4)?,
                    audio_duration: row.get(5)?,
                    folder_id: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
        })
    }

    pub fn get_pending_note_deletes(&self) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    pub fn get_note_by_client_id(&self, _client_id: &str) -> Result<Option<Note>> {
        Ok(None)
    }

    pub fn upsert_note_from_cloud(&self, cloud_note: &Note, _local_folder_id: Option<&str>) -> Result<Note> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.with_conn(|conn| {
            conn.execute(
                "INSERT OR REPLACE INTO notes (id, title, content, note_type, folder_id, is_synced, cloud_id, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, 1, ?6, ?7, ?8)",
                params![cloud_note.id, cloud_note.title, cloud_note.content, cloud_note.note_type,
                        cloud_note.folder_id, cloud_note.id, cloud_note.created_at, now],
            )?;
            Ok(())
        })?;
        Ok(cloud_note.clone())
    }

    pub fn mark_note_synced(&self, id: &str, cloud_id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute(
                "UPDATE notes SET is_synced = 1, cloud_id = ?1 WHERE id = ?2",
                params![cloud_id, id],
            )?;
            Ok(affected > 0)
        })
    }

    pub fn mark_note_sync_error(&self, id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute(
                "UPDATE notes SET is_synced = 0, sync_error = 'sync_failed' WHERE id = ?1",
                params![id],
            )?;
            Ok(affected > 0)
        })
    }

    pub fn hard_delete_note(&self, id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
            Ok(affected > 0)
        })
    }

    pub fn search_notes(&self, query: &str, limit: i64) -> Result<Vec<Note>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT n.id, n.title, n.content, n.note_type, n.source_file, n.audio_duration, n.folder_id, n.created_at, n.updated_at
                 FROM notes n
                 JOIN notes_fts fts ON n.rowid = fts.rowid
                 WHERE notes_fts MATCH ?1
                 ORDER BY rank
                 LIMIT ?2",
            )?;
            let rows = stmt.query_map(params![query, limit], |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    note_type: row.get(3)?,
                    source_file: row.get(4)?,
                    audio_duration: row.get(5)?,
                    folder_id: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
        })
    }
}
