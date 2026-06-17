use crate::whisper::model;

#[tauri::command]
pub fn check_whisper_installation() -> Result<bool, String> {
    Ok(model::list_downloaded().len() > 0)
}

#[tauri::command]
pub fn download_whisper_model(model_name: String) -> Result<String, String> {
    let path = model::model_path(&model_name).map_err(|e| e.to_string())?;
    if path.exists() {
        return Ok("already downloaded".to_string());
    }
    Err("download via HTTP not yet implemented".to_string())
}

#[tauri::command]
pub fn check_model_status(model_name: String) -> Result<bool, String> {
    Ok(model::is_downloaded(&model_name))
}

#[tauri::command]
pub fn list_whisper_models() -> Result<Vec<model::WhisperModel>, String> {
    Ok(model::list_models())
}

#[tauri::command]
pub fn delete_whisper_model(model_name: String) -> Result<(), String> {
    model::delete_model(&model_name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_all_whisper_models() -> Result<(), String> {
    model::delete_all_models().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn cancel_whisper_download() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn check_ffmpeg_availability() -> Result<bool, String> {
    Ok(which::which("ffmpeg").is_ok())
}

#[tauri::command]
pub fn get_audio_diagnostics() -> Result<String, String> {
    Ok("audio diagnostics not yet implemented".to_string())
}

#[tauri::command]
pub fn transcribe_local_whisper(
    _audio_data: Vec<u8>,
    _options: Option<String>,
) -> Result<String, String> {
    Err("local whisper transcription uses the dictation pipeline".to_string())
}

#[tauri::command]
pub fn list_downloaded_models() -> Result<Vec<String>, String> {
    Ok(model::list_downloaded())
}

#[tauri::command]
pub fn get_model_download_progress(model_name: String) -> Result<Option<u64>, String> {
    Ok(model::download_progress(&model_name))
}

// Whisper server (for faster repeated transcriptions)
#[tauri::command]
pub fn whisper_server_start(_model_name: String) -> Result<(), String> {
    Err("whisper server not needed — using in-process engine".to_string())
}

#[tauri::command]
pub fn whisper_server_stop() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn whisper_server_status() -> Result<String, String> {
    Ok("in-process engine always active".to_string())
}

// CUDA GPU acceleration
#[tauri::command]
pub fn list_gpus() -> Result<Vec<String>, String> {
    Ok(Vec::new())
}

#[tauri::command]
pub fn set_gpu_device_index(_purpose: String, _index: u32) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_gpu_device_index(_purpose: String) -> Result<Option<u32>, String> {
    Ok(None)
}

#[tauri::command]
pub fn detect_gpu() -> Result<String, String> {
    Ok("gpu detection not yet implemented".to_string())
}

#[tauri::command]
pub fn get_cuda_whisper_status() -> Result<String, String> {
    Ok("not applicable — using whisper-rs".to_string())
}

#[tauri::command]
pub fn download_cuda_whisper_binary() -> Result<(), String> {
    Err("CUDA whisper binary not needed".to_string())
}

#[tauri::command]
pub fn cancel_cuda_whisper_download() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn delete_cuda_whisper_binary() -> Result<(), String> {
    Ok(())
}

// Parakeet functions
#[tauri::command]
pub fn transcribe_local_parakeet(
    _audio_data: Vec<u8>,
    _options: Option<String>,
) -> Result<String, String> {
    Err("Parakeet not yet integrated".to_string())
}

#[tauri::command]
pub fn check_parakeet_installation() -> Result<bool, String> {
    Ok(false)
}

#[tauri::command]
pub fn download_parakeet_model(_model_name: String) -> Result<(), String> {
    Err("Parakeet download not yet implemented".to_string())
}

#[tauri::command]
pub fn check_parakeet_model_status(_model_name: String) -> Result<bool, String> {
    Ok(false)
}

#[tauri::command]
pub fn list_parakeet_models() -> Result<Vec<String>, String> {
    Ok(Vec::new())
}

#[tauri::command]
pub fn delete_parakeet_model(_model_name: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn delete_all_parakeet_models() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn cancel_parakeet_download() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_parakeet_diagnostics() -> Result<String, String> {
    Ok("Parakeet not integrated".to_string())
}

#[tauri::command]
pub fn parakeet_server_start(_model_name: String) -> Result<(), String> {
    Err("Parakeet server not needed".to_string())
}

#[tauri::command]
pub fn parakeet_server_stop() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn parakeet_server_status() -> Result<String, String> {
    Ok("not running".to_string())
}

// Model management (generic)
#[tauri::command]
pub fn model_get_all() -> Result<Vec<serde_json::Value>, String> {
    let models: Vec<serde_json::Value> = model::list_models().into_iter().map(|m| {
        serde_json::json!({
            "name": m.name,
            "size_mb": m.size_mb,
            "description": m.description,
            "downloaded": model::is_downloaded(m.name),
        })
    }).collect();
    Ok(models)
}

#[tauri::command]
pub fn model_check(model_id: String) -> Result<bool, String> {
    Ok(model::is_downloaded(&model_id))
}

#[tauri::command]
pub fn model_download(model_id: String) -> Result<(), String> {
    download_whisper_model(model_id).map(|_| ())
}

#[tauri::command]
pub fn model_delete(model_id: String) -> Result<(), String> {
    model::delete_model(&model_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn model_delete_all() -> Result<(), String> {
    model::delete_all_models().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn model_check_runtime() -> Result<String, String> {
    Ok("whisper-rs".to_string())
}

#[tauri::command]
pub fn model_cancel_download(_model_id: String) -> Result<(), String> {
    Ok(())
}

// Misc
#[tauri::command]
pub fn open_whisper_models_folder() -> Result<(), String> {
    let dir = model::models_dir().map_err(|e| e.to_string())?;
    open::that(dir).map_err(|e| e.to_string())
}
