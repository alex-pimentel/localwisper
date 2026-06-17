use tauri::State;

use crate::db::Database;
use crate::db::notes::Note;

#[tauri::command]
pub fn create_note(
    db: State<'_, Database>,
    title: String,
    content: String,
    note_type: Option<String>,
    folder_id: Option<String>,
) -> Result<Note, String> {
    db.save_note(&title, &content, &note_type.unwrap_or_else(|| "text".to_string()), folder_id.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_notes(
    db: State<'_, Database>,
    note_type: Option<String>,
    limit: Option<i64>,
    folder_id: Option<String>,
) -> Result<Vec<Note>, String> {
    db.get_notes(note_type.as_deref(), limit.unwrap_or(50), folder_id.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_note(
    db: State<'_, Database>,
    id: String,
) -> Result<Option<Note>, String> {
    db.get_note(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_note(
    db: State<'_, Database>,
    id: String,
    title: Option<String>,
    content: Option<String>,
) -> Result<bool, String> {
    db.update_note(&id, title.as_deref(), content.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_note(
    db: State<'_, Database>,
    id: String,
) -> Result<bool, String> {
    db.delete_note(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_notes(
    db: State<'_, Database>,
    query: String,
    limit: Option<i64>,
) -> Result<Vec<Note>, String> {
    db.search_notes(&query, limit.unwrap_or(20)).map_err(|e| e.to_string())
}
