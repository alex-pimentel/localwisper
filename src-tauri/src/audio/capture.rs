use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{Context, Result};
use cpal::{
    default_host,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, Stream, StreamConfig,
};
use tokio::sync::mpsc;

pub struct AudioCapture {
    stream: Option<Stream>,
    running: Arc<AtomicBool>,
    sample_rate: u32,
    channels: u16,
}

unsafe impl Send for AudioCapture {}

impl AudioCapture {
    pub fn new() -> Self {
        Self {
            stream: None,
            running: Arc::new(AtomicBool::new(false)),
            sample_rate: 0,
            channels: 0,
        }
    }

    pub fn start(
        &mut self,
        tx: mpsc::Sender<Vec<f32>>,
    ) -> Result<StreamConfig> {
        let host = default_host();
        let device = host
            .default_input_device()
            .context("no input device available")?;

        let config = device
            .default_input_config()
            .context("failed to get default input config")?;

        let stream_config = StreamConfig {
            channels: config.channels(),
            sample_rate: config.sample_rate(),
            buffer_size: BufferSize::Fixed(512),
        };

        let err_fn = |err| tracing::error!("audio capture stream error: {err}");
        let running = self.running.clone();
        running.store(true, Ordering::SeqCst);

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => {
                device.build_input_stream(
                    &stream_config,
                    move |data: &[f32], _| {
                        if running.load(Ordering::Relaxed) {
                            let chunk = data.to_vec();
                            let _ = tx.blocking_send(chunk);
                        }
                    },
                    err_fn,
                    None,
                )?
            }
            cpal::SampleFormat::I16 => {
                device.build_input_stream(
                    &stream_config,
                    move |data: &[i16], _| {
                        if running.load(Ordering::Relaxed) {
                            let chunk = data.iter().map(|s| *s as f32 / i16::MAX as f32).collect();
                            let _ = tx.blocking_send(chunk);
                        }
                    },
                    err_fn,
                    None,
                )?
            }
            cpal::SampleFormat::U16 => {
                device.build_input_stream(
                    &stream_config,
                    move |data: &[u16], _| {
                        if running.load(Ordering::Relaxed) {
                            let chunk = data
                                .iter()
                                .map(|s| (*s as f32 - u16::MAX as f32 / 2.0) / u16::MAX as f32 * 2.0)
                                .collect();
                            let _ = tx.blocking_send(chunk);
                        }
                    },
                    err_fn,
                    None,
                )?
            }
            _ => anyhow::bail!("unsupported sample format"),
        };

        stream.play().context("failed to start audio stream")?;

        self.sample_rate = config.sample_rate().0;
        self.channels = config.channels();
        self.stream = Some(stream);

        Ok(stream_config)
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(stream) = self.stream.take() {
            drop(stream);
        }
    }
}

impl Drop for AudioCapture {
    fn drop(&mut self) {
        self.stop();
    }
}
