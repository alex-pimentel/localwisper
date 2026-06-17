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

    /// Energy-based VAD. Returns true if speech is detected.
    /// Can be upgraded to Silero ONNX VAD later by swapping implementation.
    pub fn is_speech(&mut self, samples: &[f32]) -> bool {
        let energy: f32 = samples.iter().map(|s| s * s).sum::<f32>() / samples.len() as f32;
        let rms = energy.sqrt();

        match self.state {
            VadState::Silence => {
                if rms > self.config.threshold {
                    self.speech_frames += 1;
                    if (self.speech_frames as f64 * self.frame_ms) >= self.config.min_speech_duration_ms as f64 {
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
                    if (self.silence_frames as f64 * self.frame_ms) >= self.config.min_silence_duration_ms as f64 {
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
        // Need multiple frames to cross min_speech_duration
        for _ in 0..10 {
            vad.is_speech(&speech);
        }
        // After enough frames, should detect speech
        let result = vad.is_speech(&speech);
        assert!(result);
    }
}
