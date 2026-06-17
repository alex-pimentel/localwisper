#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "=== Lightwisper Transcription Benchmark ==="
echo ""

# Check for downloaded whisper models
MODEL_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/lightwisper/whisper-models"
if [ ! -d "$MODEL_DIR" ] || [ -z "$(ls -A "$MODEL_DIR" 2>/dev/null)" ]; then
    echo "ERROR: No whisper models found in $MODEL_DIR"
    echo ""
    echo "Download a model first using the app or manually:"
    echo "  mkdir -p \"$MODEL_DIR\""
    echo '  wget -O "$MODEL_DIR/ggml-base.bin" https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin'
    echo ""
    exit 1
fi

echo "Available models:"
for f in "$MODEL_DIR"/ggml-*.bin; do
    name="$(basename "$f" | sed 's/ggml-//;s/\.bin//')"
    size="$(du -h "$f" | cut -f1)"
    echo "  - $name ($size)"
done
echo ""

echo "Running benchmark..."
cd "$PROJECT_DIR/src-tauri"
RUST_LOG=warn cargo test benchmark_transcription_timing -- --ignored --nocapture 2>&1

echo ""
echo "Done."
