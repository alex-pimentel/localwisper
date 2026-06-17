use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhisperModel {
    pub name: &'static str,
    pub url: &'static str,
    pub size_mb: u64,
    pub description: &'static str,
}

pub static AVAILABLE_MODELS: &[WhisperModel] = &[
    WhisperModel {
        name: "tiny",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin",
        size_mb: 75,
        description: "Fastest, lowest quality (~75MB)",
    },
    WhisperModel {
        name: "base",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin",
        size_mb: 142,
        description: "Recommended balance (~142MB)",
    },
    WhisperModel {
        name: "small",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin",
        size_mb: 466,
        description: "Better quality (~466MB)",
    },
    WhisperModel {
        name: "medium",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin",
        size_mb: 1500,
        description: "High quality (~1.5GB)",
    },
    WhisperModel {
        name: "large-v3",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large-v3.bin",
        size_mb: 3000,
        description: "Best quality (~3GB)",
    },
    WhisperModel {
        name: "turbo",
        url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large-v3-turbo.bin",
        size_mb: 1600,
        description: "Fast with good quality (~1.6GB)",
    },
];

pub fn models_dir() -> Result<PathBuf> {
    let cache = dirs::cache_dir()
        .context("no cache directory found")?
        .join("lightwisper")
        .join("whisper-models");
    std::fs::create_dir_all(&cache)?;
    Ok(cache)
}

pub fn model_path(name: &str) -> Result<PathBuf> {
    Ok(models_dir()?.join(format!("ggml-{name}.bin")))
}

pub fn is_downloaded(name: &str) -> bool {
    model_path(name).map_or(false, |p| p.exists())
}

pub fn download_progress(name: &str) -> Option<u64> {
    let path = model_path(name).ok()?;
    if !path.exists() {
        return None;
    }
    let model = AVAILABLE_MODELS.iter().find(|m| m.name == name)?;
    let size_bytes = path.metadata().ok()?.len();
    let target_bytes = model.size_mb * 1024 * 1024;
    Some((size_bytes * 100 / target_bytes).min(100))
}

pub fn list_models() -> Vec<WhisperModel> {
    AVAILABLE_MODELS.to_vec()
}

pub fn list_downloaded() -> Vec<String> {
    let dir = match models_dir() {
        Ok(d) => d,
        _ => return vec![],
    };
    let mut downloaded = vec![];
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("ggml-") && name_str.ends_with(".bin") {
                let model_name = name_str
                    .strip_prefix("ggml-")
                    .and_then(|s| s.strip_suffix(".bin"))
                    .map(|s| s.to_string());
                if let Some(n) = model_name {
                    downloaded.push(n);
                }
            }
        }
    }
    downloaded
}

pub fn delete_model(name: &str) -> Result<()> {
    let path = model_path(name)?;
    if path.exists() {
        std::fs::remove_file(&path)?;
    }
    Ok(())
}

pub fn delete_all_models() -> Result<()> {
    let dir = models_dir()?;
    if dir.exists() {
        std::fs::remove_dir_all(&dir)?;
    }
    std::fs::create_dir_all(&dir)?;
    Ok(())
}
