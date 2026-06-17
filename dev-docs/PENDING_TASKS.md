# Pending Tasks — Lightwisper

## 🔴 Priority High

### 1. Port the ~200 `electronAPI` methods to Tauri `invoke()`

The file `/home/alexgomes/projects/openwhispr/preload.js` exposes ~200 IPC methods.
Only ~15 have been ported as Tauri commands. Remaining by category:

| Category | Count | Examples |
|---|---|---|
| Streaming (AssemblyAI, Deepgram, Corti) | ~25 | `assemblyAiStreamingStart`, `deepgramStreamingSend` |
| Whisper / Parakeet / CUDA managers | ~20 | `transcribeLocalWhisper`, `downloadCudaWhisperBinary` |
| Diarization & Speaker Mapping | ~10 | `downloadDiarizationModels`, `setSpeakerMapping` |
| Google Calendar & Contacts | ~15 | `gcalStartOAuth`, `searchContacts` |
| Cloud API (OpenWhispr Cloud) | ~15 | `cloudHealthCheck`, `cloudTranscribe` |
| Agent conversations CRUD | ~20 | `createAgentConversation`, `getAgentMessages` |
| Sync operations (notes/folders/dictionary) | ~30 | `getPendingNotes`, `upsertFolderFromCloud` |
| Meeting detection | ~15 | `meetingDetectionGetPreferences`, `joinCalendarMeeting` |
| Auto-update | ~10 | `checkForUpdates`, `downloadUpdate` |
| Window management | ~10 | `windowMinimize`, `snapToMeetingMode` |
| Hotkey management | ~15 | `updateHotkey`, `registerMeetingHotkey` |
| Enterprise providers (Bedrock, Azure, Vertex) | ~15 | `testEnterpriseConnection`, `saveBedrockRegion` |

**How to port each method:**
1. Add the Rust handler in the appropriate `commands/<domain>.rs` file
2. Use `thiserror` for error types, return `Result<T, String>` from commands
3. Register the command in `lib.rs` `invoke_handler!` macro
4. Add capability permission in `src-tauri/capabilities/default.json` if needed
5. Call from frontend via `invoke('command_name', { args })`

Reference: already-ported commands in `commands/dictation.rs`, `commands/notes.rs`, `commands/transcription.rs`, `commands/settings.rs`.

### 2. Port the React frontend

The OpenWhispr frontend at `/home/alexgomes/projects/openwhispr/src/` has dozens of components, hooks, zustand stores, and i18n config. Key files to port:

| File/Dir | Purpose |
|---|---|
| `src/App.jsx` | Root component (already started) |
| `src/AppRouter.jsx` | Route definitions |
| `src/components/` | All UI components (shadcn/ui based) |
| `src/hooks/` | React hooks that call IPC |
| `src/stores/` | Zustand state stores |
| `src/i18n.ts` | Internationalization config |
| `src/config/` | App configuration |
| `src/services/` | Service layer (some can stay in frontend) |

**Constraint:** Replace every `window.electronAPI.*` call with `invoke('*')` from `@tauri-apps/api/core`. Replace `window.electronAPI.on*` event listeners with `listen('*')` from `@tauri-apps/api/event`.

---

## 🟡 Priority Medium

### 3. Replace energy-based VAD with Silero VAD via ONNX

**Current:** `audio/vad.rs` uses energy-based detection (RMS threshold). Works but lacks accuracy for noisy environments.

**Target:** Use Silero VAD model via `ort` (ONNX Runtime) crate.

**Steps:**
1. Re-add `ort = "2.0.0-rc.12"` to `Cargo.toml`
2. Download Silero VAD ONNX model on first run
3. Replace `VoiceActivityDetector::is_speech()` with ONNX inference
4. Keep energy-based VAD as fallback

Reference: Silero VAD model at `https://github.com/snakers4/silero-vad/raw/master/src/silero_vad/data/silero_vad.onnx`

### 4. Implement ONNX semantic embeddings

**Current:** `nlp/embeddings.rs` generates pseudo-random embeddings.

**Target:** Use `all-MiniLM-L6-v2` via `ort` crate for real semantic search.

**Steps:**
1. Download model on first use (HuggingFace)
2. Implement proper tokenizer (use `bert-sbert` or `huggingface/tokenizers` crate)
3. Run inference via `ort::Session`
4. Store embeddings in SQLite or Qdrant
5. Implement hybrid search (semantic + FTS5) in `nlp/search.rs`

### 5. Add unit tests

| Module | Tests needed |
|---|---|
| `db/transcriptions.rs` | CRUD operations, FTS5 search |
| `db/notes.rs` | CRUD, FTS5, folder filtering |
| `commands/dictation.rs` | Start/stop state management |
| `audio/capture.rs` | Mock CPAL for pipeline test |
| `audio/vad.rs` | More edge cases (silence, noise, partial speech) |

---

## 🟢 Priority Low

### 6. Real benchmark

After the app is functional, run benchmarks and fill the table in `README.md`:

```bash
# Measure cold start
hyperfine --warmup 3 'target/release/lightwisper'

# Measure RAM usage (run and measure in another terminal)
ps -o rss,vsz -p $(pgrep lightwisper)
```

### 7. Create custom icons

Current: Tauri placeholder icons. Replace with original icon for `tauri.conf.json`:
- `icons/32x32.png`, `icons/128x128.png`, `icons/128x128@2x.png`
- `icons/icon.icns` (macOS)
- `icons/icon.ico` (Windows)

### 8. Configure updater

`tauri-plugin-updater` is registered but needs:
1. A GitHub Release endpoint (already set up via CI/CD)
2. `pubkey` field in `tauri.conf.json`
3. Frontend logic to prompt user on update available

---

## Reference Files

| File | Contents |
|---|---|
| `AGENTS.md` | Rules, constraints, code style, module layout |
| `dev-docs/ARCHITECTURE.md` | Modules, data flow, security model |
| `dev-docs/WHISPER_STRATEGY.md` | Why whisper-rs in-process |
| `dev-docs/AUDIO_PIPELINE.md` | CPAL → VAD → Whisper pipeline |
| `dev-docs/MIGRATION_STATUS.md` | Detailed progress tracker |
| `/home/alexgomes/projects/openwhispr/preload.js` | ~200 IPC methods to port |
| `/home/alexgomes/projects/openwhispr/src/helpers/` | 85 Node.js backend files for reference |
| `/home/alexgomes/projects/openwhispr/src/hooks/` | React hooks calling IPC |
| `/home/alexgomes/projects/openwhispr/src/stores/` | Zustand stores |
| `/home/alexgomes/projects/openwhispr/src/components/` | UI components (shadcn/ui) |

## Starting a New Agent Session

```bash
cd /home/alexgomes/projects/lightwisper
cat AGENTS.md       # read first — rules and constraints
cat PENDING_TASKS.md # read this file — prioritize
```
