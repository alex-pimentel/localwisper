use anyhow::Result;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub is_synced: bool,
    pub cloud_id: Option<String>,
    pub created_at: String,
}

impl Database {
    pub fn get_folders(&self) -> Result<Vec<Folder>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, name, is_synced, cloud_id, created_at FROM folders ORDER BY name ASC"
            )?;
            let rows = stmt.query_map([], |row| {
                Ok(Folder {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    is_synced: row.get::<_, i32>(2)? != 0,
                    cloud_id: row.get(3)?,
                    created_at: row.get(4)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
        })
    }

    pub fn create_folder(&self, name: &str) -> Result<Folder> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO folders (id, name, created_at) VALUES (?1, ?2, ?3)",
                params![id, name, now],
            )?;
            Ok(())
        })?;
        Ok(Folder {
            id,
            name: name.to_string(),
            is_synced: false,
            cloud_id: None,
            created_at: now,
        })
    }

    pub fn delete_folder(&self, id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            conn.execute("UPDATE notes SET folder_id = NULL WHERE folder_id = ?1", params![id])?;
            let affected = conn.execute("DELETE FROM folders WHERE id = ?1", params![id])?;
            Ok(affected > 0)
        })
    }

    pub fn rename_folder(&self, id: &str, name: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute(
                "UPDATE folders SET name = ?1 WHERE id = ?2",
                params![name, id],
            )?;
            Ok(affected > 0)
        })
    }

    pub fn get_folder_note_counts(&self) -> Result<Vec<(Folder, i64)>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT f.id, f.name, f.is_synced, f.cloud_id, f.created_at, COUNT(n.id) as note_count
                 FROM folders f LEFT JOIN notes n ON n.folder_id = f.id
                 GROUP BY f.id ORDER BY f.name ASC"
            )?;
            let rows = stmt.query_map([], |row| {
                let folder = Folder {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    is_synced: row.get::<_, i32>(2)? != 0,
                    cloud_id: row.get(3)?,
                    created_at: row.get(4)?,
                };
                let count: i64 = row.get(5)?;
                Ok((folder, count))
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
        })
    }

    pub fn get_pending_folders(&self) -> Result<Vec<Folder>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, name, is_synced, cloud_id, created_at FROM folders WHERE is_synced = 0"
            )?;
            let rows = stmt.query_map([], |row| {
                Ok(Folder {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    is_synced: row.get::<_, i32>(2)? != 0,
                    cloud_id: row.get(3)?,
                    created_at: row.get(4)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
        })
    }

    pub fn get_folder_by_client_id(&self, _client_id: &str) -> Result<Option<Folder>> {
        Ok(None)
    }

    pub fn upsert_folder_from_cloud(&self, cloud_folder: &Folder) -> Result<Folder> {
        self.with_conn(|conn| {
            conn.execute(
                "INSERT OR REPLACE INTO folders (id, name, is_synced, cloud_id, created_at) VALUES (?1, ?2, 1, ?3, ?4)",
                params![cloud_folder.id, cloud_folder.name, cloud_folder.cloud_id, cloud_folder.created_at],
            )?;
            Ok(())
        })?;
        Ok(cloud_folder.clone())
    }

    pub fn mark_folder_synced(&self, id: &str, cloud_id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute(
                "UPDATE folders SET is_synced = 1, cloud_id = ?1 WHERE id = ?2",
                params![cloud_id, id],
            )?;
            Ok(affected > 0)
        })
    }

    pub fn get_folder_id_map(&self) -> Result<Vec<(String, String)>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, COALESCE(cloud_id, '') FROM folders WHERE cloud_id IS NOT NULL"
            )?;
            let rows = stmt.query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
        })
    }

    pub fn get_pending_folder_deletes(&self) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    pub fn hard_delete_folder(&self, id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute("DELETE FROM folders WHERE id = ?1", params![id])?;
            Ok(affected > 0)
        })
    }
}
