use std::fs;
use std::path::{Path, PathBuf};

use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};
use serde::Deserialize;
use tauri::State;

use crate::audio::wav;
use crate::db::Database;
use crate::whisper::engine::WhisperEngine;

#[tauri::command]
pub fn save_transcription_audio(
    id: String,
    _audio_data: Vec<u8>,
    _metadata: Option<String>,
) -> Result<(), String> {
    let audio_dir = audio_storage_dir()?;
    fs::create_dir_all(&audio_dir).map_err(|e| e.to_string())?;
    let path = audio_dir.join(format!("{}.wav", id));
    // Audio data would be written here
    let _ = path;
    Ok(())
}

#[tauri::command]
pub fn get_audio_path(id: String) -> Result<String, String> {
    let audio_dir = audio_storage_dir()?;
    let path = audio_dir.join(format!("{}.wav", id));
    if path.exists() {
        Ok(path.to_string_lossy().to_string())
    } else {
        Err("audio file not found".to_string())
    }
}

#[tauri::command]
pub fn show_audio_in_folder(id: String) -> Result<(), String> {
    let path = get_audio_path(id)?;
    if let Some(parent) = PathBuf::from(&path).parent() {
        open::that(parent).map_err(|e| e.to_string())
    } else {
        Err("invalid path".to_string())
    }
}

#[tauri::command]
pub fn get_audio_buffer(id: String) -> Result<Vec<u8>, String> {
    let path = get_audio_path(id)?;
    fs::read(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_transcription_audio(id: String) -> Result<(), String> {
    let audio_dir = audio_storage_dir()?;
    let path = audio_dir.join(format!("{}.wav", id));
    if path.exists() {
        fs::remove_file(&path).map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn get_audio_storage_usage() -> Result<u64, String> {
    let audio_dir = audio_storage_dir()?;
    if !audio_dir.exists() {
        return Ok(0);
    }
    let total: u64 = fs::read_dir(&audio_dir)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .map(|m| m.len())
        .sum();
    Ok(total)
}

#[tauri::command]
pub fn delete_all_audio() -> Result<(), String> {
    let audio_dir = audio_storage_dir()?;
    if audio_dir.exists() {
        fs::remove_dir_all(&audio_dir).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn retry_transcription(
    db: State<'_, Database>,
    id: String,
    _settings: Option<String>,
) -> Result<String, String> {
    let t = db
        .get_transcription_by_id(&id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "transcription not found".to_string())?;
    Ok(t.original_text)
}

#[tauri::command]
pub fn update_transcription_text(
    db: State<'_, Database>,
    id: String,
    text: String,
    raw_text: String,
) -> Result<bool, String> {
    db.update_transcription_text(&id, &text, &raw_text)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_transcription_by_id(
    db: State<'_, Database>,
    id: String,
) -> Result<Option<crate::db::transcriptions::Transcription>, String> {
    db.get_transcription_by_id(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn select_audio_file() -> Result<String, String> {
    Err("use dialog plugin on frontend".to_string())
}

#[tauri::command]
pub fn get_file_size(file_path: String) -> Result<u64, String> {
    fs::metadata(&file_path)
        .map(|m| m.len())
        .map_err(|e| e.to_string())
}

fn resample_to_16khz(samples: &[f32], input_rate: u32) -> Result<Vec<f32>, String> {
    if input_rate == 16000 {
        return Ok(samples.to_vec());
    }

    let params = SincInterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: SincInterpolationType::Linear,
        oversampling_factor: 256,
        window: WindowFunction::BlackmanHarris2,
    };

    let mut resampler = SincFixedIn::<f32>::new(16000.0 / input_rate as f64, 1.0, params, 512, 1)
        .map_err(|e| format!("resampler creation failed: {}", e))?;

    let waves_in = vec![samples.to_vec()];
    let mut waves_out = resampler
        .process(&waves_in, None)
        .map_err(|e| format!("resampling failed: {}", e))?;

    Ok(waves_out.remove(0))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FileTranscribeOptions {
    model: Option<String>,
    language: Option<String>,
}

#[tauri::command]
pub fn transcribe_audio_file(file_path: String, options: Option<String>) -> Result<String, String> {
    let opts: FileTranscribeOptions = options
        .as_deref()
        .map(|s| serde_json::from_str(s).map_err(|e| e.to_string()))
        .transpose()?
        .unwrap_or(FileTranscribeOptions {
            model: None,
            language: None,
        });

    let wav_info = wav::read_wav(Path::new(&file_path)).map_err(|e| e.to_string())?;
    let samples = resample_to_16khz(&wav_info.samples, wav_info.sample_rate)?;

    let model_name = opts.model.as_deref().unwrap_or("base");
    let engine = WhisperEngine::new(model_name)
        .map_err(|e| format!("failed to load model '{}': {}", model_name, e))?;

    let result = engine
        .transcribe(&samples, opts.language.as_deref())
        .map_err(|e| e.to_string())?;

    Ok(result.full_text)
}

fn audio_storage_dir() -> Result<PathBuf, String> {
    let base = dirs::data_dir().ok_or_else(|| "no data dir".to_string())?;
    Ok(base.join("lightwisper").join("audio"))
}

// Listening events — not real command implementations, just event listening stubs
// These are handled via listen() on the frontend
