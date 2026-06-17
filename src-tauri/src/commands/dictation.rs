use std::sync::Arc;

use parking_lot::Mutex;
use tauri::{Emitter, State};
use tokio::sync::mpsc;

use crate::audio::capture::AudioCapture;
use crate::audio::process::AudioProcessor;
use crate::whisper::engine::WhisperEngine;
use crate::whisper::stream::WhisperStream;

pub struct DictationState {
    pub capture: Arc<Mutex<Option<AudioCapture>>>,
    pub stream: Arc<Mutex<Option<WhisperStream>>>,
    pub engine: Arc<Mutex<Option<String>>>,
    pub is_recording: Arc<Mutex<bool>>,
    pub voice_agent_tx: Arc<Mutex<Option<tokio::sync::oneshot::Sender<String>>>>,
}

impl DictationState {
    pub fn new() -> Self {
        Self {
            capture: Arc::new(Mutex::new(None)),
            stream: Arc::new(Mutex::new(None)),
            engine: Arc::new(Mutex::new(None)),
            is_recording: Arc::new(Mutex::new(false)),
            voice_agent_tx: Arc::new(Mutex::new(None)),
        }
    }
}

unsafe impl Send for DictationState {}
unsafe impl Sync for DictationState {}

pub async fn start_recording_internal(
    state: &DictationState,
    app_handle: &tauri::AppHandle,
) -> Result<String, String> {
    let mut recording = state.is_recording.lock();
    if *recording {
        return Err("already recording".to_string());
    }

    let engine_name = state.engine.lock().clone();

    let (tx, mut rx) = mpsc::channel::<Vec<f32>>(32);

    let mut capture = AudioCapture::new();
    let stream_config = capture.start(tx).map_err(|e| e.to_string())?;
    let mut processor = AudioProcessor::new(stream_config.sample_rate.0).map_err(|e| e.to_string())?;

    let stream = match engine_name {
        Some(ref name) => {
            let engine = WhisperEngine::new(name).map_err(|e| e.to_string())?;
            Some(WhisperStream::new(engine))
        }
        None => return Err("whisper engine not initialized".to_string()),
    };

    let stream_for_task = Arc::clone(&state.stream);
    let app = app_handle.clone();

    tokio::spawn(async move {
        while let Some(chunk) = rx.recv().await {
            if let Some(processed) = processor.process(&chunk) {
                let stream_guard = stream_for_task.lock();
                if let Some(ref s) = *stream_guard {
                    if let Some(result) = s.feed(&processed) {
                        match result {
                            Ok(transcription) => {
                                let _ = app.emit("transcription-segment", &transcription.full_text);
                            }
                            Err(e) => {
                                let _ = app.emit("transcription-error", &e.to_string());
                            }
                        }
                    }
                }
            }
        }
    });

    *state.capture.lock() = Some(capture);
    *state.stream.lock() = stream;

    *recording = true;
    Ok("recording started".to_string())
}

#[tauri::command]
pub async fn start_dictation(
    state: State<'_, DictationState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    start_recording_internal(&state, &app_handle).await
}

#[tauri::command]
pub async fn stop_dictation(
    state: State<'_, DictationState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let mut recording = state.is_recording.lock();
    if !*recording {
        return Err("not recording".to_string());
    }

    let final_text = if let Some(ref mut stream) = *state.stream.lock() {
        if let Some(result) = stream.flush() {
            match result {
                Ok(t) => Some(t.full_text),
                Err(e) => {
                    let _ = app_handle.emit("transcription-error", &e.to_string());
                    None
                }
            }
        } else {
            None
        }
    } else {
        None
    };

    state.capture.lock().take();
    state.stream.lock().take();

    if let Some(tx) = state.voice_agent_tx.lock().take() {
        if let Some(text) = final_text {
            let _ = tx.send(text);
        }
    } else if let Some(text) = final_text {
        let _ = app_handle.emit("transcription-final", &text);
    }

    *recording = false;
    Ok("recording stopped".to_string())
}

#[tauri::command]
pub async fn get_recording_status(state: State<'_, DictationState>) -> Result<bool, String> {
    Ok(*state.is_recording.lock())
}

#[tauri::command]
pub async fn set_model(state: State<'_, DictationState>, model_name: String) -> Result<(), String> {
    *state.engine.lock() = Some(model_name);
    Ok(())
}

#[tauri::command]
pub async fn get_model(state: State<'_, DictationState>) -> Result<Option<String>, String> {
    Ok(state.engine.lock().clone())
}
