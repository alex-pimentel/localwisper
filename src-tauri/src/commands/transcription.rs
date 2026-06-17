use tauri::State;

use crate::db::Database;
use crate::db::transcriptions::Transcription;

#[tauri::command]
pub fn save_transcription(
    db: State<'_, Database>,
    text: String,
    raw_text: String,
    agent_name: Option<String>,
) -> Result<Transcription, String> {
    db.save_transcription(&text, &raw_text, agent_name.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_transcriptions(
    db: State<'_, Database>,
    limit: Option<i64>,
) -> Result<Vec<Transcription>, String> {
    db.get_transcriptions(limit.unwrap_or(50)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_transcription(
    db: State<'_, Database>,
    id: String,
) -> Result<bool, String> {
    db.delete_transcription(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn clear_transcriptions(
    db: State<'_, Database>,
) -> Result<u64, String> {
    db.clear_transcriptions().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_transcriptions(
    db: State<'_, Database>,
    query: String,
    limit: Option<i64>,
) -> Result<Vec<Transcription>, String> {
    db.search_transcriptions(&query, limit.unwrap_or(20)).map_err(|e| e.to_string())
}
