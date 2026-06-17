# Lightwisper Architecture

## High-Performance Tauri v2 Migration of OpenWhispr

### Core Philosophy

Replace Electron + Node.js sidecar-spawning architecture with a **native Rust pipeline** that runs speech-to-text inference **in-process** using `whisper-rs`, eliminating all I/O and IPC overhead of the original design.

### Why Not a 1:1 Port

The original OpenWhispr:
1. Records audio via JS `MediaRecorder API` → Blob → ArrayBuffer
2. Sends blob through Electron IPC to main process
3. Writes temp WAV file to disk
4. Spawns `whisper.cpp` as child process
5. Reads result back from stdout
6. Deletes temp file

**This is ~6 round-trips across process/IO boundaries per transcription.**

Lightwisper replaces this with:
```rust
// Single in-process call — no IPC, no disk I/O, no subprocess
let ctx = WhisperContext::new(&model_path)?;
ctx.full(params, &cpal_audio_buffer)?;
```

### Architecture Diagram

```
┌──────────────────────────────────────────────────┐
│                   Tauri Shell                      │
│  ┌──────────┐   ┌──────────────────────────────┐  │
│  │  React UI  │   │       Rust Backend            │  │
│  │  (Vite)    │◄──►  ┌──────────────────────┐   │  │
│  │            │invoke│   tauri::commands      │   │  │
│  │  shadcn/ui │   │  │  (domínio-específicos)  │   │  │
│  │  zustand   │   │  └──────────────────────┘   │  │
│  │  i18next   │   │         │                    │  │
│  └──────────┘   │         ▼                    │  │
│                 │  ┌──────────────────────┐   │  │
│                 │  │   Audio Pipeline       │   │  │
│                 │  │  ┌─────┐ ┌─────┐ ┌───┐ │   │  │
│                 │  │  │CPAL  │→│VAD   │→│SRC │ │   │  │
│                 │  │  │capture│ │detect│ │resam│ │   │  │
│                 │  │  └─────┘ └─────┘ └───┘ │   │  │
│                 │  └──────────┬───────────┘   │  │
│                 │             ▼               │  │
│                 │  ┌──────────────────────┐   │  │
│                 │  │   Whisper Engine       │   │  │
│                 │  │  (whisper-rs in-proc)  │   │  │
│                 │  └──────────────────────┘   │  │
│                 │             │               │  │
│                 │             ▼               │  │
│                 │  ┌──────────────────────┐   │  │
│                 │  │   SQLite (rusqlite)    │   │  │
│                 │  │   + ONNX Embeddings    │   │  │
│                 │  │   + Sidecars (Qdrant)  │   │  │
│                 │  └──────────────────────┘   │  │
│                 └──────────────────────────────┘  │
└──────────────────────────────────────────────────┘
```

### Key Decisions

| Decision | Rationale |
|---|---|
| `whisper-rs` in-process | Elimina IPC/IO por transcrição. 1 chamada FFI vs 6 hops |
| `cpal` for audio capture | Latência de ms vs MediaRecorder + IPC. Callbacks nativos por plataforma |
| Silero VAD via ONNX | Detecção de fala em Rust, sem depender de JS |
| `ort` crate | ONNX Runtime nativo no Rust, substitui onnxruntime-node worker |
| Sidecars apenas quando inevitável | Qdrant, llama-server — processos que não podem ser library |
| Scoped Tauri commands | Segurança por design, não por allowlist |

### Module Map

```
src-tauri/src/
├── lib.rs                 # Tauri builder, plugin registration, state init
├── main.rs                # Entry point
├── audio/
│   ├── mod.rs
│   ├── capture.rs         # CPAL microphone stream
│   ├── vad.rs             # Silero VAD via ONNX
│   └── process.rs         # Resampling, gain, filtering
├── whisper/
│   ├── mod.rs
│   ├── engine.rs          # WhisperContext wrapper, full()/stream()
│   ├── model.rs           # Download, cache, hot-swap
│   └── stream.rs          # Real-time streaming transcription
├── db/
│   ├── mod.rs
│   ├── transcriptions.rs  # CRUD transcription history
│   ├── notes.rs           # Notes CRUD + semantic search
│   └── migrations.rs      # Schema management
├── nlp/
│   ├── mod.rs
│   ├── embeddings.rs      # MiniLM via ONNX
│   └── search.rs          # Hybrid search (FTS5 + vector)
├── commands/
│   ├── mod.rs
│   ├── dictation.rs       # Tauri commands for dictation
│   ├── notes.rs           # Note commands
│   ├── transcription.rs   # History commands
│   └── settings.rs        # Settings commands
└── sidecars/
    ├── mod.rs
    ├── qdrant.rs           # Qdrant process lifecycle
    └── llama.rs            # llama-server process lifecycle
```

### Security Model

- All file system access goes through Tauri's scoped filesystem plugin
- Commands validate input at the type level (no raw string IDs)
- Audio capture requests user permission via `tauri-plugin-dialog`
- API keys stored encrypted via OS keychain (`keyring` crate)
- CSP is strict — no inline scripts, no remote fetch from renderer
