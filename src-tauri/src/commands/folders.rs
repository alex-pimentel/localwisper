use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::Database;
use crate::db::folders::Folder;

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderNoteCount {
    pub folder: Folder,
    pub note_count: i64,
}

#[tauri::command]
pub fn get_folders(
    db: State<'_, Database>,
) -> Result<Vec<Folder>, String> {
    db.get_folders().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_folder(
    db: State<'_, Database>,
    name: String,
) -> Result<Folder, String> {
    db.create_folder(&name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_folder(
    db: State<'_, Database>,
    id: String,
) -> Result<bool, String> {
    db.delete_folder(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn rename_folder(
    db: State<'_, Database>,
    id: String,
    name: String,
) -> Result<bool, String> {
    db.rename_folder(&id, &name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_folder_note_counts(
    db: State<'_, Database>,
) -> Result<Vec<FolderNoteCount>, String> {
    let items = db.get_folder_note_counts().map_err(|e| e.to_string())?;
    Ok(items.into_iter().map(|(f, c)| FolderNoteCount { folder: f, note_count: c }).collect())
}

// Sync operations
#[tauri::command]
pub fn get_pending_folders(
    db: State<'_, Database>,
) -> Result<Vec<Folder>, String> {
    db.get_pending_folders().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_folder_by_client_id(
    db: State<'_, Database>,
    client_id: String,
) -> Result<Option<Folder>, String> {
    db.get_folder_by_client_id(&client_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn upsert_folder_from_cloud(
    db: State<'_, Database>,
    cloud_folder: Folder,
) -> Result<Folder, String> {
    db.upsert_folder_from_cloud(&cloud_folder).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn mark_folder_synced(
    db: State<'_, Database>,
    id: String,
    cloud_id: String,
) -> Result<bool, String> {
    db.mark_folder_synced(&id, &cloud_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_folder_id_map(
    db: State<'_, Database>,
) -> Result<Vec<(String, String)>, String> {
    db.get_folder_id_map().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_pending_folder_deletes(
    db: State<'_, Database>,
) -> Result<Vec<String>, String> {
    db.get_pending_folder_deletes().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn hard_delete_folder(
    db: State<'_, Database>,
    id: String,
) -> Result<bool, String> {
    db.hard_delete_folder(&id).map_err(|e| e.to_string())
}
