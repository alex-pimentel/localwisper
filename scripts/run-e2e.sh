#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "=== Lightwisper E2E Transcription Tests ==="
echo ""

MODEL_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/lightwisper/whisper-models"
if [ ! -d "$MODEL_DIR" ] || [ -z "$(ls -A "$MODEL_DIR" 2>/dev/null)" ]; then
    echo "ERROR: No whisper models found in $MODEL_DIR"
    exit 1
fi

echo "Running E2E tests..."
cd "$PROJECT_DIR/src-tauri"
RUST_LOG=warn cargo test e2e_transcribe -- --ignored --nocapture 2>&1

echo ""
echo "Done."
