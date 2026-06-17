# Whisper Integration Strategy

## In-Process Inference via whisper-rs

### Original Architecture (OpenWhispr)

```
MediaRecorder → Blob → IPC → write temp.wav → spawn whisper.cpp → read stdout → delete temp.wav
```

Costs per transcription:
- 1 IPC serialize/deserialize (audio blob)
- 1 disk write (~1-5MB)
- 1 process spawn (fork+execve)
- 1 process stdout read
- 1 disk delete

### New Architecture (Lightwisper)

```rust
// Audio comes from CPAL directly as Vec<f32>
// One FFI call, zero copies of the audio buffer
let model_path = model::resolve("base")?;
let ctx = whisper_context::WhisperContext::new(&model_path)?;
let params = WhisperFullParams::new(WhisperSamplingStrategy::Greedy);
ctx.full(params, &audio_samples)?;

// Read segments from context directly
let n_segments = ctx.n_segments();
for i in 0..n_segments {
    let text = ctx.segment_text(i)?;
    // text is a &str into whisper.cpp's internal buffer
}
```

### Why whisper-rs over Sidecar

1. **Latency**: ~50ms per transcription vs ~500ms+ with subprocess
2. **Memory**: Model loaded once, shared across calls (ARC)
3. **Simplicity**: No temp files, no PID management, no crash recovery
4. **Demo impact**: Shows FFI expertise (bindgen, unsafe, ABI)

### Implementation Plan

```rust
// whisper/engine.rs
pub struct WhisperEngine {
    ctx: WhisperContext,
    state: WhisperState,
}

impl WhisperEngine {
    pub fn new(model_path: &Path) -> Result<Self> {
        let ctx = WhisperContext::new(model_path)?;
        let state = ctx.create_state()?;
        Ok(Self { ctx, state })
    }

    pub fn transcribe(&mut self, samples: &[f32], lang: &str) -> Result<String> {
        let mut params = WhisperFullParams::new(WhisperSamplingStrategy::Greedy);
        params.set_language(Some(lang));
        // ... set other params

        self.state.full(params, samples)?;
        // collect segments
        Ok(segments)
    }
}
```

### Model Management

- Models stored in `~/.cache/lightwisper/whisper-models/`
- Download via HTTP with `reqwest` + streaming to file
- Auto-detect CPU features (AVX2, AVX512, NEON, Metal, CUDA)
- Hot-swap: `engine.reload("large-v3")` switches model at runtime

### Streaming (Real-time)

```rust
// whisper/stream.rs
pub struct WhisperStream {
    engine: WhisperEngine,
    ring_buffer: AudioRingBuffer,
    last_transcription: Instant,
}

impl WhisperStream {
    /// Called from CPAL callback, processes chunks as they arrive
    pub fn feed(&mut self, chunk: &[f32]) -> Option<String> {
        self.ring_buffer.push(chunk);
        if self.ring_buffer.len() >= MIN_SAMPLES_FOR_INFERENCE {
            let audio = self.ring_buffer.drain();
            let vad_pass = self.vad.is_speech(&audio);
            if vad_pass {
                Some(self.engine.transcribe(&audio, "en"))
            } else {
                None
            }
        } else {
            None
        }
    }
}
```

### Whisper.cpp Compilation Strategy

| Target | Feature | Build |
|---|---|---|
| macOS (Intel) | AVX2 + Accelerate | vendored in build.rs |
| macOS (Apple Silicon) | Metal + NEON | vendored in build.rs |
| Linux (x86_64) | AVX2 | vendored in build.rs |
| Linux (aarch64) | NEON | vendored in build.rs |
| Windows (x86_64) | AVX2 | vendored in build.rs |

`whisper-rs` supports `build.rs` compilation of whisper.cpp, which means:
- Single `cargo build` compiles everything
- No manual download of whisper.cpp binaries
- Platform-specific optimizations auto-detected
