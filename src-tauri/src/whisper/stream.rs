use std::time::Instant;

use anyhow::Result;
use parking_lot::Mutex;

use super::engine::{TranscriptionResult, WhisperEngine};

const WHISPER_SAMPLE_RATE: u32 = 16000;
const MIN_SAMPLES_FOR_INFERENCE: usize = WHISPER_SAMPLE_RATE as usize * 2;
const MAX_SAMPLES_BEFORE_FLUSH: usize = WHISPER_SAMPLE_RATE as usize * 30;

pub struct RingBuffer {
    buffer: Vec<f32>,
    capacity: usize,
}

impl RingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, samples: &[f32]) {
        self.buffer.extend_from_slice(samples);
        if self.buffer.len() > self.capacity {
            let excess = self.buffer.len() - self.capacity;
            self.buffer.drain(0..excess);
        }
    }

    pub fn drain(&mut self) -> Vec<f32> {
        self.buffer.drain(..).collect()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

pub struct WhisperStream {
    engine: Mutex<WhisperEngine>,
    buffer: Mutex<RingBuffer>,
    last_transcription: Mutex<Instant>,
}

impl WhisperStream {
    pub fn new(engine: WhisperEngine) -> Self {
        Self {
            engine: Mutex::new(engine),
            buffer: Mutex::new(RingBuffer::new(MAX_SAMPLES_BEFORE_FLUSH)),
            last_transcription: Mutex::new(Instant::now()),
        }
    }

    /// Feed audio from the CPAL/VAD pipeline.
    /// Returns a transcription when enough speech has accumulated.
    pub fn feed(&self, samples: &[f32]) -> Option<Result<TranscriptionResult>> {
        let mut buf = self.buffer.lock();
        buf.push(samples);

        if buf.len() < MIN_SAMPLES_FOR_INFERENCE && self.last_transcription.lock().elapsed().as_secs() < 3 {
            return None;
        }

        if buf.len() >= MIN_SAMPLES_FOR_INFERENCE {
            let audio = buf.drain();
            let engine = self.engine.lock();
            let result = engine.transcribe(&audio, None);
            *self.last_transcription.lock() = Instant::now();
            Some(result)
        } else {
            None
        }
    }

    /// Force transcription of buffered audio.
    pub fn flush(&self) -> Option<Result<TranscriptionResult>> {
        let mut buf = self.buffer.lock();
        if buf.is_empty() {
            return None;
        }
        let audio = buf.drain();
        let engine = self.engine.lock();
        let result = engine.transcribe(&audio, None);
        *self.last_transcription.lock() = Instant::now();
        Some(result)
    }

    pub fn reset(&self) {
        self.buffer.lock().drain();
    }
}
