# Lightwisper — Agentic Coder Guide

## Project Overview

Lightwisper is a Tauri v2 migration of [OpenWhispr](https://github.com/OpenWhispr/openwhispr), an Electron dictation app using whisper.cpp. **The core differentiator**: in-process whisper inference via `whisper-rs` instead of spawning subprocesses, and native audio capture via `cpal` instead of MediaRecorder + IPC.

## Architecture Files (read first)

Read these before making any changes:

1. `dev-docs/ARCHITECTURE.md` — Overall module structure, data flow, security model
2. `dev-docs/WHISPER_STRATEGY.md` — Why in-process, how whisper-rs is integrated
3. `dev-docs/AUDIO_PIPELINE.md` — CPAL capture, VAD, processing pipeline
4. `dev-docs/MIGRATION_STATUS.md` — Current progress and remaining tasks

## Technology Stack

| Layer | Technology |
|---|---|
| Desktop framework | Tauri v2 |
| Frontend | React 19 + Vite + Tailwind v4 + shadcn/ui |
| State | Zustand |
| i18n | i18next + react-i18next |
| Backend language | Rust (edition 2021) |
| Whisper bindings | `whisper-rs` (whisper.cpp compiled in build.rs) |
| Audio capture | `cpal` (ALSA/CoreAudio/WASAPI) |
| VAD | Silero VAD via `ort` (ONNX Runtime) |
| Embeddings | MiniLM via `ort` |
| Database | `rusqlite` with compile-time SQL |
| Vector search | Qdrant sidecar (only unavoidable external process) |
| Keychain | `keyring` crate (macOS Keychain, Windows DPAPI, Linux libsecret) |

## Module Layout

```
src-tauri/src/
├── lib.rs           # Tauri builder, plugins, state init — entry point
├── main.rs          # Windows subsystem attribute only
├── audio/           # CPAL capture, VAD, processing
├── whisper/         # whisper-rs engine, model mgmt, streaming
├── db/              # SQLite via rusqlite
├── nlp/             # ONNX Runtime for embeddings
├── commands/        # Tauri commands (one file per domain)
└── sidecars/        # External process lifecycle (Qdrant, llama-server)
```

## Key Constraints

### DO NOT
- **Do not** spawn `whisper.cpp` as a subprocess. Use `whisper-rs` in-process.
- **Do not** use `MediaRecorder` API. Use `cpal` in Rust for audio capture.
- **Do not** write temp audio files. Keep buffers in memory.
- **Do not** use `tauri-plugin-shell` for whisper — it defeats the purpose.
- **Do not** port Node.js patterns 1:1. Rethink in Rust idioms.
- **Do not** add new files without reading existing module structure first.
- **Do not** write comments in code. The code should be self-documenting.

### DO
- Read `dev-docs/ARCHITECTURE.md` before adding modules
- Add new Tauri commands in `commands/` following single-responsibility per file
- Use `thiserror` for error types. Return `Result<T, AppError>` from commands.
- Use `tauri::State` for managed state (WhisperEngine, Database, etc.)
- Scoped permissions via Tauri capabilities, not CSP hacks
- Keep unsafe blocks minimal and documented
- Use async where IO-bound, sync where CPU-bound (whisper is CPU)
- Test with `cargo test` — unit tests alongside modules
- Frontend calls backend via `@tauri-apps/api` `invoke()` only

## How to Implement a New Feature

1. Check `MIGRATION_STATUS.md` — is it already planned?
2. Read the relevant `dev-docs/*.md` for architecture decisions
3. Find the corresponding module in `src-tauri/src/`
4. Read `src/helpers/` from the original OpenWhispr for reference
5. Implement in Rust using the patterns from existing modules
6. Add Tauri command in `commands/`
7. Export command in `lib.rs`
8. Add capability permission in `src-tauri/capabilities/default.json`
9. Call from frontend via `invoke('command_name', { args })`

## Original OpenWhispr Reference

The source code is at `/home/alexgomes/projects/openwhispr/`.

Key files to reference:
- `main.js` — Electron main process (lifecycle, IPC setup)
- `preload.js` — All IPC bridge methods (~200 commands)
- `src/helpers/` — All backend managers
- `src/hooks/` — React hooks that call IPC
- `src/stores/` — Zustand state stores

## Compilation

```bash
# Development
cd lightwisper
cargo tauri dev

# Build for production
cargo tauri build

# Run Rust tests only
cargo test -p app

# Run all tests
cargo test
```

## Code Style

- No comments in code. The types and function names must be self-explanatory.
- Use enums instead of stringly-typed parameters.
- Match on `Result` and `Option` exhaustively — no `.unwrap()` in production code.
- Prefer iterators over for-loops.
- `snake_case` for Rust, `camelCase` for TypeScript.
- Modules re-export at `mod.rs` level — keep `lib.rs` imports clean.
