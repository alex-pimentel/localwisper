# Audio Pipeline

## From Microphone to Text — Zero-Copy Architecture

### Capture Layer (CPAL)

```rust
// audio/capture.rs
pub struct AudioCapture {
    stream: Option<Stream>,
    sender: mpsc::Sender<Vec<f32>>,
}

impl AudioCapture {
    pub fn start(config: CaptureConfig) -> Result<Self> {
        let host = cpal::default_host();
        let device = host.default_input_device()?;
        let config = device.default_input_config()?;

        let (tx, rx) = mpsc::channel::<Vec<f32>>(32);

        let stream = device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &InputCallbackInfo| {
                let _ = tx.blocking_send(data.to_vec());
            },
            |err| eprintln!("audio error: {err}"),
            None,
        )?;

        stream.play()?;
        Ok(Self { stream: Some(stream), sender: tx })
    }

    pub fn stream(&self) -> mpsc::Receiver<Vec<f32>> { /* clone rx */ }

    pub fn stop(&mut self) { self.stream.take().map(|s| s.drop()); }
}
```

### Voice Activity Detection (Silero VAD via ONNX)

Precisa de VAD para não enviar silêncio para o Whisper:

```rust
// audio/vad.rs
pub struct SileroVad {
    session: ort::Session,
    state: HashMap<String, ort::Value>,
    threshold: f32,
}

impl SileroVad {
    pub fn is_speech(&mut self, samples: &[f32]) -> bool {
        // 1. Resample to 16kHz
        // 2. Run Silero VAD ONNX model
        // 3. Return speech probability > threshold
    }
}
```

### Audio Processing Pipeline

```rust
// audio/process.rs
pub struct AudioProcessor {
    resampler: SincResampler,  // CPAL sample rate → 16kHz
    vad: SileroVad,
    gain: f32,
}

impl AudioProcessor {
    pub fn process(&mut self, raw: &[f32]) -> Option<Vec<f32>> {
        let resampled = self.resampler.process(raw);
        let adjusted = resampled.iter().map(|s| s * self.gain).collect::<Vec<_>>();
        if self.vad.is_speech(&adjusted) {
            Some(adjusted)
        } else {
            None  // skip silence — no token generation wasted
        }
    }
}
```

### Data Flow (Recording)

```
CPAL callback (every ~10ms)
  │
  ▼
AudioProcessor.process(raw_samples)
  │
  ├── VAD: silence? → discard
  │
  └── VAD: speech? → append to RingBuffer
                          │
                          ▼
                    RingBuffer.len() >= MIN_CHUNK?
                          │
                          ├── No → wait for more audio
                          │
                          └── Yes → WhisperEngine.transcribe(buffer)
                                      │
                                      ▼
                                    text → Tauri event → React UI
```

### Why CPAL instead of MediaRecorder?

| Aspect | MediaRecorder (JS) | CPAL (Rust) |
|---|---|---|
| Latency | ~100ms chunks | ~10ms callbacks |
| Format | WebM/opus → needs decode | Raw f32 — zero decode |
| Thread | Main thread → IPC | Dedicated audio thread |
| Control | Browser API | Direct hardware access |
| Portability | Same API everywhere | ALSA/CoreAudio/WASAPI |
