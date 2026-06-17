use serde_json::Value;

use crate::db::notes::Note;
use crate::db::Database;
use tauri::State;

#[tauri::command]
pub fn process_anthropic_reasoning(
    _text: String,
    _model_id: String,
    _agent_name: Option<String>,
    _config: Option<String>,
) -> Result<String, String> {
    Err("Anthropic reasoning not yet implemented".to_string())
}

#[tauri::command]
pub fn export_note(
    db: State<'_, Database>,
    note_id: String,
    _format: Option<String>,
) -> Result<String, String> {
    let note = db.get_note(&note_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "note not found".to_string())?;
    Ok(note.content)
}

#[tauri::command]
pub fn export_transcript(
    db: State<'_, Database>,
    note_id: String,
    _format: Option<String>,
) -> Result<String, String> {
    let note = db.get_note(&note_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "note not found".to_string())?;
    Ok(note.content)
}

#[tauri::command]
pub fn export_dictionary(
    db: State<'_, Database>,
    _words: Vec<String>,
) -> Result<String, String> {
    let entries = db.get_dictionary().map_err(|e| e.to_string())?;
    let words: Vec<String> = entries.into_iter().map(|e| e.word).collect();
    Ok(words.join("\n"))
}

#[tauri::command]
pub fn semantic_search_notes(
    db: State<'_, Database>,
    query: String,
    limit: Option<i64>,
) -> Result<Vec<Note>, String> {
    db.search_notes(&query, limit.unwrap_or(20)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn semantic_reindex_all() -> Result<(), String> {
    Err("semantic reindex requires ONNX — not yet integrated".to_string())
}

#[tauri::command]
pub fn semantic_search_conversations(
    db: State<'_, Database>,
    query: String,
    limit: Option<i64>,
) -> Result<Vec<crate::db::agent_conversations::AgentConversation>, String> {
    db.search_agent_conversations(&query, limit.unwrap_or(20))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_note_cloud_id(
    db: State<'_, Database>,
    id: String,
    cloud_id: String,
) -> Result<bool, String> {
    db.update_note_cloud_id(&id, &cloud_id).map_err(|e| e.to_string())
}

// Note files (markdown mirror)
#[tauri::command]
pub fn note_files_set_enabled(
    _enabled: bool,
    _custom_path: Option<String>,
    _options: Option<String>,
) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn note_files_set_path(_path: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn note_files_rebuild() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn note_files_get_default_path() -> Result<String, String> {
    let base = dirs::document_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    Ok(base.join("LightwisperNotes").to_string_lossy().to_string())
}

#[tauri::command]
pub fn note_files_pick_folder() -> Result<String, String> {
    Err("use dialog plugin on frontend".to_string())
}

#[tauri::command]
pub fn show_note_file(
    db: State<'_, Database>,
    note_id: String,
) -> Result<(), String> {
    let _note = db.get_note(&note_id)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn show_folder_in_explorer(_folder_name: String) -> Result<(), String> {
    Ok(())
}

// Cloud audio file transcription
#[tauri::command]
pub fn transcribe_audio_file_cloud(_file_path: String) -> Result<String, String> {
    Err("cloud transcription not yet implemented".to_string())
}

#[tauri::command]
pub fn transcribe_audio_file_byok(_options: Option<String>) -> Result<String, String> {
    Err("BYOK transcription not yet implemented".to_string())
}

// Referral
#[tauri::command]
pub fn get_referral_stats() -> Result<Value, String> {
    Ok(serde_json::json!({"referralCount": 0, "creditsEarned": 0}))
}

#[tauri::command]
pub fn send_referral_invite(_email: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_referral_invites() -> Result<Vec<Value>, String> {
    Ok(Vec::new())
}
