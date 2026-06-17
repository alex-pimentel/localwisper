use anyhow::Result;

use crate::db::notes::Note;
use crate::db::Database;

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

    pub fn search_notes_semantic(
        &mut self,
        query: &str,
        model: &mut super::embeddings::EmbeddingModel,
    ) -> Result<Vec<Note>> {
        let query_emb = model.embed(query)?;

        let all_notes = self.db.get_notes(None, 1000, None)?;

        let mut scored: Vec<(f32, Note)> = Vec::new();

        for note in all_notes {
            let content = format!("{} {}", note.title, note.content);
            if let Ok(note_emb) = model.embed(&content) {
                let similarity = cosine_similarity(&query_emb, &note_emb);
                if similarity > 0.3 {
                    scored.push((similarity, note));
                }
            }
        }

        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(10);

        Ok(scored.into_iter().map(|(_, note)| note).collect())
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a > 0.0 && norm_b > 0.0 {
        dot / (norm_a * norm_b)
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity_identical() {
        let a = vec![1.0, 2.0, 3.0];
        let sim = cosine_similarity(&a, &a);
        assert!((sim - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_zero() {
        let a = vec![0.0, 0.0];
        let b = vec![1.0, 2.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_partial() {
        let a = vec![1.0, 0.0];
        let b = vec![1.0, 1.0];
        let sim = cosine_similarity(&a, &b);
        let expected = 1.0 / 2.0_f32.sqrt();
        assert!((sim - expected).abs() < 1e-6);
    }
}
