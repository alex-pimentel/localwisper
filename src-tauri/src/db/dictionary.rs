use anyhow::Result;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub id: String,
    pub word: String,
    pub is_synced: bool,
    pub cloud_id: Option<String>,
    pub created_at: String,
}

impl Database {
    pub fn get_dictionary(&self) -> Result<Vec<DictionaryEntry>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, word, is_synced, cloud_id, created_at FROM dictionary ORDER BY word ASC"
            )?;
            let rows = stmt.query_map([], |row| {
                Ok(DictionaryEntry {
                    id: row.get(0)?,
                    word: row.get(1)?,
                    is_synced: row.get::<_, i32>(2)? != 0,
                    cloud_id: row.get(3)?,
                    created_at: row.get(4)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
        })
    }

    pub fn set_dictionary(&self, words: &[String]) -> Result<Vec<DictionaryEntry>> {
        self.with_conn(|conn| {
            conn.execute_batch("DELETE FROM dictionary")?;
            let mut entries = Vec::new();
            for word in words {
                let id = Uuid::new_v4().to_string();
                let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
                conn.execute(
                    "INSERT INTO dictionary (id, word, created_at) VALUES (?1, ?2, ?3)",
                    params![id, word, now],
                )?;
                entries.push(DictionaryEntry {
                    id,
                    word: word.clone(),
                    is_synced: false,
                    cloud_id: None,
                    created_at: now,
                });
            }
            Ok(entries)
        })
    }

    pub fn add_dictionary_word(&self, word: &str) -> Result<DictionaryEntry> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.with_conn(|conn| {
            conn.execute(
                "INSERT OR IGNORE INTO dictionary (id, word, created_at) VALUES (?1, ?2, ?3)",
                params![id, word, now],
            )?;
            Ok(())
        })?;
        Ok(DictionaryEntry {
            id,
            word: word.to_string(),
            is_synced: false,
            cloud_id: None,
            created_at: now,
        })
    }

    pub fn remove_dictionary_word(&self, id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute("DELETE FROM dictionary WHERE id = ?1", params![id])?;
            Ok(affected > 0)
        })
    }

    pub fn undo_learned_corrections(&self, words: &[String]) -> Result<u64> {
        self.with_conn(|conn| {
            let mut count = 0u64;
            for word in words {
                let affected = conn.execute("DELETE FROM dictionary WHERE word = ?1", params![word])?;
                count += affected as u64;
            }
            Ok(count)
        })
    }

    pub fn get_pending_dictionary(&self) -> Result<Vec<DictionaryEntry>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, word, is_synced, cloud_id, created_at FROM dictionary WHERE is_synced = 0"
            )?;
            let rows = stmt.query_map([], |row| {
                Ok(DictionaryEntry {
                    id: row.get(0)?,
                    word: row.get(1)?,
                    is_synced: row.get::<_, i32>(2)? != 0,
                    cloud_id: row.get(3)?,
                    created_at: row.get(4)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
        })
    }

    pub fn get_pending_dictionary_deletes(&self) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    pub fn get_dictionary_by_client_id(&self, _client_id: &str) -> Result<Option<DictionaryEntry>> {
        Ok(None)
    }

    pub fn upsert_dictionary_from_cloud(&self, cloud_entry: &DictionaryEntry) -> Result<DictionaryEntry> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.with_conn(|conn| {
            conn.execute(
                "INSERT OR REPLACE INTO dictionary (id, word, is_synced, cloud_id, created_at) VALUES (?1, ?2, 1, ?3, ?4)",
                params![cloud_entry.id, cloud_entry.word, cloud_entry.cloud_id, now],
            )?;
            Ok(())
        })?;
        Ok(cloud_entry.clone())
    }

    pub fn mark_dictionary_synced(&self, id: &str, cloud_id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute(
                "UPDATE dictionary SET is_synced = 1, cloud_id = ?1 WHERE id = ?2",
                params![cloud_id, id],
            )?;
            Ok(affected > 0)
        })
    }

    pub fn hard_delete_dictionary(&self, id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute("DELETE FROM dictionary WHERE id = ?1", params![id])?;
            Ok(affected > 0)
        })
    }

    pub fn clear_dictionary_cloud_id(&self, id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute(
                "UPDATE dictionary SET cloud_id = NULL WHERE id = ?1",
                params![id],
            )?;
            Ok(affected > 0)
        })
    }
}
