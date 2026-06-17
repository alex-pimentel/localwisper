use anyhow::Result;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConversation {
    pub id: String,
    pub title: Option<String>,
    pub note_id: Option<String>,
    pub is_archived: bool,
    pub is_synced: bool,
    pub cloud_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub id: i64,
    pub conversation_id: String,
    pub role: String,
    pub content: String,
    pub metadata: Option<String>,
    pub created_at: String,
}

impl Database {
    pub fn create_agent_conversation(&self, title: Option<&str>, note_id: Option<&str>) -> Result<AgentConversation> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO agent_conversations (id, title, note_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![id, title, note_id, now, now],
            )?;
            Ok(())
        })?;
        Ok(AgentConversation {
            id,
            title: title.map(|s| s.to_string()),
            note_id: note_id.map(|s| s.to_string()),
            is_archived: false,
            is_synced: false,
            cloud_id: None,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn get_agent_conversations(&self, limit: i64) -> Result<Vec<AgentConversation>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, title, note_id, is_archived, is_synced, cloud_id, created_at, updated_at
                 FROM agent_conversations ORDER BY updated_at DESC LIMIT ?1"
            )?;
            let rows = stmt.query_map(params![limit], |row| {
                Ok(AgentConversation {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    note_id: row.get(2)?,
                    is_archived: row.get::<_, i32>(3)? != 0,
                    is_synced: row.get::<_, i32>(4)? != 0,
                    cloud_id: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
        })
    }

    pub fn get_agent_conversation(&self, id: &str) -> Result<Option<AgentConversation>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, title, note_id, is_archived, is_synced, cloud_id, created_at, updated_at
                 FROM agent_conversations WHERE id = ?1"
            )?;
            let mut rows = stmt.query_map(params![id], |row| {
                Ok(AgentConversation {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    note_id: row.get(2)?,
                    is_archived: row.get::<_, i32>(3)? != 0,
                    is_synced: row.get::<_, i32>(4)? != 0,
                    cloud_id: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })?;
            Ok(rows.next().transpose()?)
        })
    }

    pub fn delete_agent_conversation(&self, id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            conn.execute("DELETE FROM agent_messages WHERE conversation_id = ?1", params![id])?;
            let affected = conn.execute("DELETE FROM agent_conversations WHERE id = ?1", params![id])?;
            Ok(affected > 0)
        })
    }

    pub fn update_agent_conversation_title(&self, id: &str, title: &str) -> Result<bool> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.with_conn(|conn| {
            let affected = conn.execute(
                "UPDATE agent_conversations SET title = ?1, updated_at = ?2 WHERE id = ?3",
                params![title, now, id],
            )?;
            Ok(affected > 0)
        })
    }

    pub fn add_agent_message(&self, conversation_id: &str, role: &str, content: &str, metadata: Option<&str>) -> Result<AgentMessage> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO agent_messages (conversation_id, role, content, metadata, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![conversation_id, role, content, metadata, now],
            )?;
            let id = conn.last_insert_rowid();
            // Update conversation timestamp
            conn.execute(
                "UPDATE agent_conversations SET updated_at = ?1 WHERE id = ?2",
                params![now, conversation_id],
            )?;
            Ok(AgentMessage {
                id,
                conversation_id: conversation_id.to_string(),
                role: role.to_string(),
                content: content.to_string(),
                metadata: metadata.map(|s| s.to_string()),
                created_at: now,
            })
        })
    }

    pub fn get_agent_messages(&self, conversation_id: &str) -> Result<Vec<AgentMessage>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, conversation_id, role, content, metadata, created_at
                 FROM agent_messages WHERE conversation_id = ?1 ORDER BY id ASC"
            )?;
            let rows = stmt.query_map(params![conversation_id], |row| {
                Ok(AgentMessage {
                    id: row.get(0)?,
                    conversation_id: row.get(1)?,
                    role: row.get(2)?,
                    content: row.get(3)?,
                    metadata: row.get(4)?,
                    created_at: row.get(5)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
        })
    }

    pub fn archive_agent_conversation(&self, id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute(
                "UPDATE agent_conversations SET is_archived = 1 WHERE id = ?1",
                params![id],
            )?;
            Ok(affected > 0)
        })
    }

    pub fn unarchive_agent_conversation(&self, id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute(
                "UPDATE agent_conversations SET is_archived = 0 WHERE id = ?1",
                params![id],
            )?;
            Ok(affected > 0)
        })
    }

    pub fn get_conversations_for_note(&self, note_id: &str, limit: i64) -> Result<Vec<AgentConversation>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, title, note_id, is_archived, is_synced, cloud_id, created_at, updated_at
                 FROM agent_conversations WHERE note_id = ?1 ORDER BY updated_at DESC LIMIT ?2"
            )?;
            let rows = stmt.query_map(params![note_id, limit], |row| {
                Ok(AgentConversation {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    note_id: row.get(2)?,
                    is_archived: row.get::<_, i32>(3)? != 0,
                    is_synced: row.get::<_, i32>(4)? != 0,
                    cloud_id: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
        })
    }

    pub fn search_agent_conversations(&self, query: &str, limit: i64) -> Result<Vec<AgentConversation>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, title, note_id, is_archived, is_synced, cloud_id, created_at, updated_at
                 FROM agent_conversations WHERE title LIKE ?1 ORDER BY updated_at DESC LIMIT ?2"
            )?;
            let pattern = format!("%{}%", query);
            let rows = stmt.query_map(params![pattern, limit], |row| {
                Ok(AgentConversation {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    note_id: row.get(2)?,
                    is_archived: row.get::<_, i32>(3)? != 0,
                    is_synced: row.get::<_, i32>(4)? != 0,
                    cloud_id: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
        })
    }

    pub fn update_agent_conversation_cloud_id(&self, id: &str, cloud_id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute(
                "UPDATE agent_conversations SET cloud_id = ?1 WHERE id = ?2",
                params![cloud_id, id],
            )?;
            Ok(affected > 0)
        })
    }

    pub fn get_pending_conversations(&self) -> Result<Vec<AgentConversation>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, title, note_id, is_archived, is_synced, cloud_id, created_at, updated_at
                 FROM agent_conversations WHERE is_synced = 0"
            )?;
            let rows = stmt.query_map([], |row| {
                Ok(AgentConversation {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    note_id: row.get(2)?,
                    is_archived: row.get::<_, i32>(3)? != 0,
                    is_synced: row.get::<_, i32>(4)? != 0,
                    cloud_id: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
        })
    }

    pub fn get_pending_conversation_deletes(&self) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    pub fn get_conversation_by_client_id(&self, _client_id: &str) -> Result<Option<AgentConversation>> {
        Ok(None)
    }

    pub fn upsert_conversation_from_cloud(&self, cloud_conv: &AgentConversation) -> Result<AgentConversation> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.with_conn(|conn| {
            conn.execute(
                "INSERT OR REPLACE INTO agent_conversations (id, title, note_id, is_archived, is_synced, cloud_id, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, 1, ?5, ?6, ?7)",
                params![cloud_conv.id, cloud_conv.title, cloud_conv.note_id, cloud_conv.is_archived as i32, cloud_conv.cloud_id, cloud_conv.created_at, now],
            )?;
            Ok(())
        })?;
        Ok(cloud_conv.clone())
    }

    pub fn mark_conversation_synced(&self, id: &str, cloud_id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute(
                "UPDATE agent_conversations SET is_synced = 1, cloud_id = ?1 WHERE id = ?2",
                params![cloud_id, id],
            )?;
            Ok(affected > 0)
        })
    }

    pub fn hard_delete_conversation(&self, id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            conn.execute("DELETE FROM agent_messages WHERE conversation_id = ?1", params![id])?;
            let affected = conn.execute("DELETE FROM agent_conversations WHERE id = ?1", params![id])?;
            Ok(affected > 0)
        })
    }
}
