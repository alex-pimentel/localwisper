use anyhow::Result;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub prompt: Option<String>,
    pub icon: Option<String>,
}

impl Database {
    pub fn get_actions(&self) -> Result<Vec<Action>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, name, description, prompt, icon FROM actions ORDER BY name ASC",
            )?;
            let rows = stmt.query_map([], |row| {
                Ok(Action {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    prompt: row.get(3)?,
                    icon: row.get(4)?,
                })
            })?;
            rows.collect::<std::result::Result<Vec<_>, _>>()
                .map_err(Into::into)
        })
    }

    pub fn get_action(&self, id: &str) -> Result<Option<Action>> {
        self.with_conn(|conn| {
            let mut stmt = conn
                .prepare("SELECT id, name, description, prompt, icon FROM actions WHERE id = ?1")?;
            let mut rows = stmt.query_map(params![id], |row| {
                Ok(Action {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    prompt: row.get(3)?,
                    icon: row.get(4)?,
                })
            })?;
            Ok(rows.next().transpose()?)
        })
    }

    pub fn create_action(
        &self,
        name: &str,
        description: Option<&str>,
        prompt: Option<&str>,
        icon: Option<&str>,
    ) -> Result<Action> {
        let id = Uuid::new_v4().to_string();
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO actions (id, name, description, prompt, icon) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![id, name, description, prompt, icon],
            )?;
            Ok(())
        })?;
        Ok(Action {
            id,
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            prompt: prompt.map(|s| s.to_string()),
            icon: icon.map(|s| s.to_string()),
        })
    }

    pub fn update_action(
        &self,
        id: &str,
        name: Option<&str>,
        description: Option<&str>,
        prompt: Option<&str>,
        icon: Option<&str>,
    ) -> Result<bool> {
        self.with_conn(|conn| {
            if let Some(n) = name {
                conn.execute("UPDATE actions SET name = ?1 WHERE id = ?2", params![n, id])?;
            }
            if let Some(d) = description {
                conn.execute(
                    "UPDATE actions SET description = ?1 WHERE id = ?2",
                    params![d, id],
                )?;
            }
            if let Some(p) = prompt {
                conn.execute(
                    "UPDATE actions SET prompt = ?1 WHERE id = ?2",
                    params![p, id],
                )?;
            }
            if let Some(i) = icon {
                conn.execute("UPDATE actions SET icon = ?1 WHERE id = ?2", params![i, id])?;
            }
            Ok(true)
        })
    }

    pub fn delete_action(&self, id: &str) -> Result<bool> {
        self.with_conn(|conn| {
            let affected = conn.execute("DELETE FROM actions WHERE id = ?1", params![id])?;
            Ok(affected > 0)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_db() -> Database {
        let dir = std::env::temp_dir().join(format!("lightwisper_test_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        Database::open(&dir.join("test.db")).unwrap()
    }

    #[test]
    fn test_create_and_get_action() {
        let db = test_db();
        let action = db
            .create_action(
                "Summarize",
                Some("Summarize the text"),
                Some("Please summarize:"),
                None,
            )
            .unwrap();
        assert_eq!(action.name, "Summarize");

        let got = db.get_action(&action.id).unwrap().unwrap();
        assert_eq!(got.name, "Summarize");
    }

    #[test]
    fn test_get_actions() {
        let db = test_db();
        db.create_action("Action 1", None, None, None).unwrap();
        db.create_action("Action 2", None, None, None).unwrap();
        let actions = db.get_actions().unwrap();
        assert_eq!(actions.len(), 2);
    }

    #[test]
    fn test_update_action() {
        let db = test_db();
        let action = db.create_action("Old Name", None, None, None).unwrap();
        db.update_action(&action.id, Some("New Name"), None, None, None)
            .unwrap();
        let updated = db.get_action(&action.id).unwrap().unwrap();
        assert_eq!(updated.name, "New Name");
    }

    #[test]
    fn test_delete_action() {
        let db = test_db();
        let action = db.create_action("Delete Me", None, None, None).unwrap();
        assert!(db.delete_action(&action.id).unwrap());
        assert!(db.get_action(&action.id).unwrap().is_none());
    }
}
