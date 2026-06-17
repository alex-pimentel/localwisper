use std::sync::Arc;

use anyhow::{Context, Result};
use parking_lot::Mutex;
use whisper_rs::{WhisperContext, WhisperContextParameters, SamplingStrategy};

use super::model;

pub struct TranscriptionSegment {
    pub index: i32,
    pub text: String,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
}

pub struct TranscriptionResult {
    pub segments: Vec<TranscriptionSegment>,
    pub full_text: String,
    pub language: String,
}

pub struct WhisperEngine {
    ctx: Arc<Mutex<WhisperContext>>,
    model_name: String,
}

impl WhisperEngine {
    pub fn new(model_name: &str) -> Result<Self> {
        let path = model::model_path(model_name)
            .context("model path resolution failed")?;

        if !path.exists() {
            anyhow::bail!("model '{}' not downloaded. Download it first.", model_name);
        }

        let params = WhisperContextParameters::default();
        let ctx = WhisperContext::new_with_params(&path.to_string_lossy(), params)
            .context("failed to create whisper context")?;

        Ok(Self {
            ctx: Arc::new(Mutex::new(ctx)),
            model_name: model_name.to_string(),
        })
    }

    pub fn reload(&mut self, model_name: &str) -> Result<()> {
        let path = model::model_path(model_name)?;
        if !path.exists() {
            anyhow::bail!("model '{}' not downloaded", model_name);
        }

        let params = WhisperContextParameters::default();
        let ctx = WhisperContext::new_with_params(&path.to_string_lossy(), params)
            .context("failed to reload whisper context")?;

        *self.ctx.lock() = ctx;
        self.model_name = model_name.to_string();
        Ok(())
    }

    pub fn transcribe(
        &self,
        samples: &[f32],
        lang: Option<&str>,
    ) -> Result<TranscriptionResult> {
        let ctx = self.ctx.lock();
        let mut state = ctx
            .create_state()
            .context("failed to create whisper state")?;

        let mut params = whisper_rs::FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

        params.set_language(lang);

        params.set_n_threads(num_cpus::get() as i32);

        state
            .full(params, samples)
            .context("whisper inference failed")?;

        let n_segments = state
            .full_n_segments()
            .context("failed to get segment count")?;

        let mut segments = Vec::with_capacity(n_segments as usize);
        let mut full_text = String::new();

        for i in 0..n_segments {
            let text = state
                .full_get_segment_text(i)
                .context("failed to get segment text")?;
            let start = state
                .full_get_segment_t0(i)
                .context("failed to get segment start time")?;
            let end = state
                .full_get_segment_t1(i)
                .context("failed to get segment end time")?;

            segments.push(TranscriptionSegment {
                index: i,
                text: text.clone(),
                start_timestamp: start,
                end_timestamp: end,
            });

            if !full_text.is_empty() {
                full_text.push(' ');
            }
            full_text.push_str(&text);
        }

        let language = lang.unwrap_or("en").to_string();

        Ok(TranscriptionResult {
            segments,
            full_text,
            language,
        })
    }

    pub fn sample_rate() -> u32 {
        WHISPER_SAMPLE_RATE
    }

    pub fn model_name(&self) -> &str {
        &self.model_name
    }
}

const WHISPER_SAMPLE_RATE: u32 = 16000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation_fails_without_model() {
        let result = WhisperEngine::new("nonexistent-model");
        assert!(result.is_err());
    }
}
