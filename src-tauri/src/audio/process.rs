use anyhow::Result;
use rubato::{Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction};

use super::vad::VoiceActivityDetector;

const TARGET_SAMPLE_RATE: u32 = 16000;

pub struct AudioProcessor {
    resampler: SincFixedIn<f32>,
    vad: VoiceActivityDetector,
    target_sample_rate: u32,
}

impl AudioProcessor {
    pub fn new(input_sample_rate: u32) -> Result<Self> {
        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };

        let resampler = SincFixedIn::<f32>::new(
            TARGET_SAMPLE_RATE as f64 / input_sample_rate as f64,
            1.0,
            params,
            512,
            1,
        )?;

        let vad = VoiceActivityDetector::new(TARGET_SAMPLE_RATE);

        Ok(Self {
            resampler,
            vad,
            target_sample_rate: TARGET_SAMPLE_RATE,
        })
    }

    pub fn process(&mut self, samples: &[f32]) -> Option<Vec<f32>> {
        let resampled = self.resample(samples).ok()?;
        if self.vad.is_speech(&resampled) {
            Some(resampled)
        } else {
            None
        }
    }

    fn resample(&mut self, samples: &[f32]) -> Result<Vec<f32>> {
        let waves_in = vec![samples.to_vec()];
        let waves_out = self.resampler.process(&waves_in, None)?;
        Ok(waves_out.into_iter().next().unwrap_or_default())
    }

    pub fn reset_vad(&mut self) {
        self.vad.reset();
    }

    pub fn target_rate(&self) -> u32 {
        self.target_sample_rate
    }
}
