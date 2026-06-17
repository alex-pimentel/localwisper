use anyhow::Result;

use crate::db::Database;
use crate::db::notes::Note;

pub struct HybridSearch {
    db: Database,
}

impl HybridSearch {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn search_notes(&self, query: &str, limit: i64) -> Result<Vec<Note>> {
        self.db.search_notes(query, limit)
    }

    pub fn search_notes_semantic(&self, _query: &str) -> Result<Vec<Note>> {
        anyhow::bail!("semantic search requires ONNX — not yet integrated")
    }
}
