use anyhow::Result;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transcription {
    pub id: String,
    pub timestamp: String,
    pub original_text: String,
    pub processed_text: Option<String>,
    pub is_processed: bool,
    pub processing_method: String,
    pub agent_name: Option<String>,
    pub error: Option<String>,
}

impl Database {
    pub fn save_transcription(
        &self,
        text: &str,
        raw_text: &str,
        agent_name: Option<&str>,
    ) -> Result<Transcription> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO transcriptions (id, timestamp, original_text, processed_text, is_processed, processing_method, agent_name)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    id,
                    now,
                    raw_text,
                    text,
                    if text != raw_text { 1 } else { 0 },
                    "cleanup",
                    agent_name,
                ],
            )?;
            Ok(())
        })?;

        Ok(Transcription {
            id,
            timestamp: now,
            original_text: raw_text.to_string(),
            processed_text: Some(text.to_string()),
            is_processed: text != raw_text,
            processing_method: "cleanup".to_string(),
            agent_name: agent_name.map(|s| s.to_string()),
            error: None,
        })
    }

    pub fn get_transcriptions(&self, limit: i64) -> Result<Vec<Transcription>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, timestamp, original_text, processed_text, is_processed, processing_method, agent_name, error
                 FROM transcriptions ORDER BY timestamp DESC LIMIT ?1",
            )?;
            let rows = stmt.query_map(params![limit], |row| {
                Ok(Transcription {
                    id: row.get(0)?,
                    timestamp: row.get(1)?,
                    original_text: row.get(2)?,
                    processed_text: row.get(3)?,
                    is_processed: row.get::<_, i32>(4)? != 0,
                    processing_method: row.get(5)?,
                    agent_name: row.get(6)?,
                    error: row.get(7)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
        })
    }

    pub fn delete_transcription(&self, id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute("DELETE FROM transcriptions WHERE id = ?1", params![id])?;
            Ok(affected > 0)
        })
    }

    pub fn clear_transcriptions(&self) -> Result<u64> {
        self.with_conn(|conn| {
            let affected = conn.execute("DELETE FROM transcriptions", [])?;
            Ok(affected as u64)
        })
    }

    pub fn search_transcriptions(&self, query: &str, limit: i64) -> Result<Vec<Transcription>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT t.id, t.timestamp, t.original_text, t.processed_text, t.is_processed, t.processing_method, t.agent_name, t.error
                 FROM transcriptions t
                 JOIN transcriptions_fts fts ON t.rowid = fts.rowid
                 WHERE transcriptions_fts MATCH ?1
                 ORDER BY rank
                 LIMIT ?2",
            )?;
            let rows = stmt.query_map(params![query, limit], |row| {
                Ok(Transcription {
                    id: row.get(0)?,
                    timestamp: row.get(1)?,
                    original_text: row.get(2)?,
                    processed_text: row.get(3)?,
                    is_processed: row.get::<_, i32>(4)? != 0,
                    processing_method: row.get(5)?,
                    agent_name: row.get(6)?,
                    error: row.get(7)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
        })
    }

    pub fn update_transcription_text(&self, id: &str, text: &str, raw_text: &str) -> Result<bool> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let is_processed = if text != raw_text { 1 } else { 0 };
        self.with_conn(|conn| {
            let affected = conn.execute(
                "UPDATE transcriptions SET original_text = ?1, processed_text = ?2, is_processed = ?3, updated_at = ?4 WHERE id = ?5",
                params![raw_text, text, is_processed, now, id],
            )?;
            Ok(affected > 0)
        })
    }

    pub fn get_pending_transcriptions(&self) -> Result<Vec<Transcription>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, timestamp, original_text, processed_text, is_processed, processing_method, agent_name, error
                 FROM transcriptions WHERE is_synced = 0 ORDER BY timestamp DESC"
            )?;
            let rows = stmt.query_map([], |row| {
                Ok(Transcription {
                    id: row.get(0)?,
                    timestamp: row.get(1)?,
                    original_text: row.get(2)?,
                    processed_text: row.get(3)?,
                    is_processed: row.get::<_, i32>(4)? != 0,
                    processing_method: row.get(5)?,
                    agent_name: row.get(6)?,
                    error: row.get(7)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
        })
    }

    pub fn get_transcription_by_client_id(&self, _client_id: &str) -> Result<Option<Transcription>> {
        Ok(None)
    }

    pub fn upsert_transcription_from_cloud(&self, cloud_t: &Transcription) -> Result<Transcription> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.with_conn(|conn| {
            conn.execute(
                "INSERT OR REPLACE INTO transcriptions (id, timestamp, original_text, processed_text, is_processed, processing_method, agent_name, error, is_synced, cloud_id, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 1, ?9, ?10)",
                params![cloud_t.id, cloud_t.timestamp, cloud_t.original_text, cloud_t.processed_text,
                        cloud_t.is_processed as i32, cloud_t.processing_method, cloud_t.agent_name, cloud_t.error,
                        cloud_t.id, now],
            )?;
            Ok(())
        })?;
        Ok(cloud_t.clone())
    }

    pub fn mark_transcription_synced(&self, id: &str, cloud_id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute(
                "UPDATE transcriptions SET is_synced = 1, cloud_id = ?1 WHERE id = ?2",
                params![cloud_id, id],
            )?;
            Ok(affected > 0)
        })
    }

    pub fn get_pending_transcription_deletes(&self) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    pub fn hard_delete_transcription(&self, id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute("DELETE FROM transcriptions WHERE id = ?1", params![id])?;
            Ok(affected > 0)
        })
    }

    pub fn get_transcription_by_id(&self, id: &str) -> Result<Option<Transcription>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, timestamp, original_text, processed_text, is_processed, processing_method, agent_name, error
                 FROM transcriptions WHERE id = ?1",
            )?;
            let mut rows = stmt.query_map(params![id], |row| {
                Ok(Transcription {
                    id: row.get(0)?,
                    timestamp: row.get(1)?,
                    original_text: row.get(2)?,
                    processed_text: row.get(3)?,
                    is_processed: row.get::<_, i32>(4)? != 0,
                    processing_method: row.get(5)?,
                    agent_name: row.get(6)?,
                    error: row.get(7)?,
                })
            })?;
            Ok(rows.next().transpose()?)
        })
    }
}
