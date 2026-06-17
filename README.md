# Lightwisper

![Build](https://github.com/USERNAME/lightwisper/actions/workflows/build.yml/badge.svg)
![Rust](https://img.shields.io/badge/rust-1.83%2B-orange)
![Tauri](https://img.shields.io/badge/tauri-v2-purple)

High-performance dictation app — Tauri v2 + **whisper-rs in-process** instead of Electron subprocess-spawning.

> Forked from [OpenWhispr](https://github.com/OpenWhispr/openwhispr). Rebuilt from the ground up in Rust for performance, security, and bundle size.

## Performance

| Metric | Electron (OpenWhispr) | Lightwisper (Tauri) |
|---|---|---|
| Bundle size | ~150 MB | ~5 MB + model |
| RAM (idle) | ~200–400 MB | ~20–50 MB |
| Cold start | ~3–5 s | ~0.5–1 s |
| Transcrição via | whisper.cpp subprocess (IPC + disco) | **whisper-rs in-process (FFI)** |
| Captura áudio | MediaRecorder → Blob → IPC | **CPAL nativo (zero cópia)** |
| API Keys | safeStorage | **keyring (Keychain/DPAPI/libsecret)** |

## Architecture

```
cpal capture ──► rubato resample ──► VAD ──► whisper-rs ──► SQLite + FTS5
     │                                │
     └── callback a cada 10ms        └── descarta silêncio
```

- **Zero subprocessos para transcrição** — `whisper-rs` compila whisper.cpp no `build.rs`
- **Zero arquivos temporários de áudio** — buffers `Vec<f32>` direto na pipeline
- **Zero JS no pipeline de áudio** — CPAL + Rust do microfone ao texto

## Stack

| Layer | Tech |
|---|---|
| Desktop | Tauri v2 |
| Frontend | React 19 + Vite |
| Transcritor | whisper-rs (FFI nativo) |
| Áudio | cpAL (ALSA/CoreAudio/WASAPI) |
| DB | SQLite + FTS5 (rusqlite) |
| Secrets | keyring crate |
| Sidecars | Qdrant, llama-server |

## Quick Start

```bash
# System deps (Linux)
sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev \
  patchelf libasound2-dev libclang-dev cmake pkg-config

git clone https://github.com/USERNAME/lightwisper
cd lightwisper

npm install
npx tauri dev
```

## Download a Model

```bash
# Baixa o modelo base (~142 MB)
curl -L -o ~/.cache/lightwisper/whisper-models/ggml-base.bin \
  https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin
```

## CI/CD

A cada push para `main`, o GitHub Actions compila binários otimizados para:

- 🟦 Windows (`.msi` installer)
- 🍏 macOS (`.dmg`)
- 🐧 Linux (`.deb` / `.AppImage`)

Os binários ficam disponíveis em **Releases** na página do GitHub.

## Dev Docs

- [`dev-docs/ARCHITECTURE.md`](dev-docs/ARCHITECTURE.md) — Decisões arquiteturais
- [`dev-docs/WHISPER_STRATEGY.md`](dev-docs/WHISPER_STRATEGY.md) — In-process vs sidecar
- [`dev-docs/AUDIO_PIPELINE.md`](dev-docs/AUDIO_PIPELINE.md) — CPAL + VAD pipeline
- [`dev-docs/MIGRATION_STATUS.md`](dev-docs/MIGRATION_STATUS.md) — Progresso
- [`AGENTS.md`](AGENTS.md) — Guia para agentic coder

## License

MIT
