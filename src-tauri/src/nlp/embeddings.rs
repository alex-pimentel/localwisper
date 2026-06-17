use std::path::PathBuf;

use anyhow::{Context, Result};

pub struct EmbeddingModel;

impl EmbeddingModel {
    pub fn new(_model_path: &std::path::Path) -> Result<Self> {
        tracing::warn!("embedding model loaded — ONNX runtime not integrated yet");
        Ok(Self)
    }

    pub fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let dim = Self::dimension();
        let hash = text.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
        let seed = hash as f32 / u64::MAX as f32;
        let embedding: Vec<f32> = (0..dim).map(|i| seed + (i as f32 * 0.01).sin()).collect();
        Ok(embedding)
    }

    pub fn dimension() -> usize {
        384
    }

    pub fn models_dir() -> Result<PathBuf> {
        let cache = dirs::cache_dir()
            .context("no cache directory")?
            .join("lightwisper")
            .join("embedding-models");
        std::fs::create_dir_all(&cache)?;
        Ok(cache)
    }

    pub fn is_available() -> bool {
        false
    }
}
