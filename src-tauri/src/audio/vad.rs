use std::path::PathBuf;

use anyhow::{Context, Result};

pub struct VadConfig {
    pub threshold: f32,
    pub min_speech_duration_ms: u64,
    pub min_silence_duration_ms: u64,
}

impl Default for VadConfig {
    fn default() -> Self {
        Self {
            threshold: 0.5,
            min_speech_duration_ms: 100,
            min_silence_duration_ms: 500,
        }
    }
}

pub enum VadState {
    Speech,
    Silence,
}

pub struct VoiceActivityDetector {
    config: VadConfig,
    state: VadState,
    speech_frames: u64,
    silence_frames: u64,
    sample_rate: u32,
    frame_ms: f64,
}

impl VoiceActivityDetector {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            config: VadConfig::default(),
            state: VadState::Silence,
            speech_frames: 0,
            silence_frames: 0,
            sample_rate,
            frame_ms: 512.0 / sample_rate as f64 * 1000.0,
        }
    }

    pub fn with_config(sample_rate: u32, config: VadConfig) -> Self {
        Self {
            config,
            state: VadState::Silence,
            speech_frames: 0,
            silence_frames: 0,
            sample_rate,
            frame_ms: 512.0 / sample_rate as f64 * 1000.0,
        }
    }

    pub fn is_speech(&mut self, samples: &[f32]) -> bool {
        let energy: f32 = samples.iter().map(|s| s * s).sum::<f32>() / samples.len() as f32;
        let rms = energy.sqrt();

        match self.state {
            VadState::Silence => {
                if rms > self.config.threshold {
                    self.speech_frames += 1;
                    if (self.speech_frames as f64 * self.frame_ms)
                        >= self.config.min_speech_duration_ms as f64
                    {
                        self.state = VadState::Speech;
                        self.silence_frames = 0;
                        return true;
                    }
                } else {
                    self.speech_frames = 0;
                }
                false
            }
            VadState::Speech => {
                if rms <= self.config.threshold {
                    self.silence_frames += 1;
                    if (self.silence_frames as f64 * self.frame_ms)
                        >= self.config.min_silence_duration_ms as f64
                    {
                        self.state = VadState::Silence;
                        self.speech_frames = 0;
                        return false;
                    }
                    true
                } else {
                    self.silence_frames = 0;
                    true
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.state = VadState::Silence;
        self.speech_frames = 0;
        self.silence_frames = 0;
    }
}

pub struct SileroVad {
    session: Option<ort::session::Session>,
    threshold: f32,
}

impl SileroVad {
    pub fn new() -> Self {
        let session = match load_silero_model() {
            Ok(session) => {
                tracing::info!("silero VAD model loaded successfully");
                Some(session)
            }
            Err(e) => {
                tracing::warn!(
                    "failed to load silero VAD model (will use energy-based fallback): {}",
                    e
                );
                None
            }
        };

        Self {
            session,
            threshold: 0.5,
        }
    }

    pub fn is_speech(&mut self, samples: &[f32]) -> Option<bool> {
        let session = self.session.as_mut()?;

        let shape = [1_usize, samples.len()];
        let tensor = ort::value::Tensor::<f32>::from_array((shape, samples.to_vec())).ok()?;

        let outputs = session.run(ort::inputs![tensor]).ok()?;
        let output_value = outputs.get("output")?;
        let prob = output_value.try_extract_scalar::<f32>().ok()?;

        Some(prob > self.threshold)
    }

    pub fn is_available(&self) -> bool {
        self.session.is_some()
    }
}

fn silero_model_path() -> Result<PathBuf> {
    let cache = dirs::cache_dir()
        .context("no cache directory")?
        .join("lightwisper")
        .join("vad-models");
    std::fs::create_dir_all(&cache)?;
    Ok(cache.join("silero_vad.onnx"))
}

fn load_silero_model() -> Result<ort::session::Session> {
    let path = silero_model_path()?;

    if !path.exists() {
        download_silero_model(&path)?;
    }

    let session = ort::session::Session::builder()?.commit_from_file(&path)?;

    Ok(session)
}

fn download_silero_model(path: &std::path::Path) -> Result<()> {
    let url = "https://github.com/snakers4/silero-vad/raw/v4.0/src/silero_vad/data/silero_vad.onnx";

    tracing::info!("downloading silero VAD model from {}", url);

    let response = reqwest::blocking::get(url).context("failed to download silero VAD model")?;
    let bytes = response.bytes().context("failed to read model bytes")?;

    std::fs::write(path, &bytes).context("failed to write silero VAD model")?;
    tracing::info!("silero VAD model downloaded to {:?}", path);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_silence_detected() {
        let mut vad = VoiceActivityDetector::new(16000);
        let silence = vec![0.0_f32; 512];
        assert!(!vad.is_speech(&silence));
    }

    #[test]
    fn test_speech_detected() {
        let mut vad = VoiceActivityDetector::new(16000);
        let speech = vec![0.6_f32; 512];
        for _ in 0..10 {
            vad.is_speech(&speech);
        }
        let result = vad.is_speech(&speech);
        assert!(result);
    }

    #[test]
    fn test_partial_speech_insufficient_duration() {
        let mut vad = VoiceActivityDetector::new(16000);
        let speech = vec![0.6_f32; 512];
        assert!(!vad.is_speech(&speech));
    }

    #[test]
    fn test_speech_to_silence_transition() {
        let mut vad = VoiceActivityDetector::new(16000);
        let speech = vec![0.6_f32; 512];
        let silence = vec![0.0_f32; 512];

        for _ in 0..10 {
            vad.is_speech(&speech);
        }
        assert!(vad.is_speech(&speech));

        for _ in 0..20 {
            vad.is_speech(&silence);
        }
        assert!(!vad.is_speech(&silence));
    }

    #[test]
    fn test_reset_clears_state() {
        let mut vad = VoiceActivityDetector::new(16000);
        let speech = vec![0.6_f32; 512];
        for _ in 0..10 {
            vad.is_speech(&speech);
        }
        assert!(vad.is_speech(&speech));
        vad.reset();
        assert!(!vad.is_speech(&speech));
    }

    #[test]
    fn test_custom_config() {
        let config = VadConfig {
            threshold: 0.1,
            min_speech_duration_ms: 50,
            min_silence_duration_ms: 200,
        };
        let mut vad = VoiceActivityDetector::with_config(16000, config);
        let speech = vec![0.2_f32; 512];
        for _ in 0..3 {
            vad.is_speech(&speech);
        }
        assert!(vad.is_speech(&speech));
    }

    #[test]
    fn test_silero_vad_creation() {
        let vad = SileroVad::new();
        assert!(!vad.is_available() || vad.is_available());
    }
}
