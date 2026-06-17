use tauri::{Emitter, State};

use crate::db::Database;
use crate::db::dictionary::DictionaryEntry;

#[tauri::command]
pub fn get_dictionary(
    db: State<'_, Database>,
) -> Result<Vec<DictionaryEntry>, String> {
    db.get_dictionary().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_dictionary(
    db: State<'_, Database>,
    words: Vec<String>,
) -> Result<Vec<DictionaryEntry>, String> {
    db.set_dictionary(&words).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_dictionary_word(
    db: State<'_, Database>,
    word: String,
) -> Result<DictionaryEntry, String> {
    db.add_dictionary_word(&word).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn remove_dictionary_word(
    db: State<'_, Database>,
    id: String,
) -> Result<bool, String> {
    db.remove_dictionary_word(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn undo_learned_corrections(
    db: State<'_, Database>,
    words: Vec<String>,
) -> Result<u64, String> {
    db.undo_learned_corrections(&words).map_err(|e| e.to_string())
}

// Sync operations for dictionary
#[tauri::command]
pub fn get_pending_dictionary(
    db: State<'_, Database>,
) -> Result<Vec<DictionaryEntry>, String> {
    db.get_pending_dictionary().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_pending_dictionary_deletes(
    db: State<'_, Database>,
) -> Result<Vec<String>, String> {
    db.get_pending_dictionary_deletes().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_dictionary_by_client_id(
    db: State<'_, Database>,
    client_id: String,
) -> Result<Option<DictionaryEntry>, String> {
    db.get_dictionary_by_client_id(&client_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn upsert_dictionary_from_cloud(
    db: State<'_, Database>,
    cloud_entry: DictionaryEntry,
) -> Result<DictionaryEntry, String> {
    db.upsert_dictionary_from_cloud(&cloud_entry).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn mark_dictionary_synced(
    db: State<'_, Database>,
    id: String,
    cloud_id: String,
) -> Result<bool, String> {
    db.mark_dictionary_synced(&id, &cloud_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn hard_delete_dictionary(
    db: State<'_, Database>,
    id: String,
) -> Result<bool, String> {
    db.hard_delete_dictionary(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn clear_dictionary_cloud_id(
    db: State<'_, Database>,
    id: String,
) -> Result<bool, String> {
    db.clear_dictionary_cloud_id(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn broadcast_dictionary_updated(
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let _ = app_handle.emit("dictionary-updated", serde_json::json!({}));
    Ok(())
}
