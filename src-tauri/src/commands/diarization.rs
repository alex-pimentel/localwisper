#[tauri::command]
pub fn download_diarization_models() -> Result<(), String> {
    Err("diarization not yet implemented".to_string())
}

#[tauri::command]
pub fn get_diarization_model_status() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({"downloaded": false}))
}

#[tauri::command]
pub fn delete_diarization_models() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn cancel_diarization_download() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_speaker_mappings(_note_id: String) -> Result<Vec<serde_json::Value>, String> {
    Ok(Vec::new())
}

#[tauri::command]
pub fn set_speaker_mapping(
    _note_id: String,
    _speaker_id: String,
    _display_name: Option<String>,
    _email: Option<String>,
    _profile_id: Option<String>,
) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn remove_speaker_mapping(_note_id: String, _speaker_id: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_speaker_profiles() -> Result<Vec<serde_json::Value>, String> {
    Ok(Vec::new())
}

#[tauri::command]
pub fn attach_speaker_email(_profile_id: String, _email: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn save_note_speaker_embeddings(_note_id: String, _embeddings: Vec<f64>) -> Result<(), String> {
    Ok(())
}
