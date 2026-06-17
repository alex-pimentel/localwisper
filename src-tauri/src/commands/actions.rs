use tauri::State;

use crate::db::actions::Action;
use crate::db::Database;

#[tauri::command]
pub fn get_actions(
    db: State<'_, Database>,
) -> Result<Vec<Action>, String> {
    db.get_actions().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_action(
    db: State<'_, Database>,
    id: String,
) -> Result<Option<Action>, String> {
    db.get_action(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_action(
    db: State<'_, Database>,
    name: String,
    description: Option<String>,
    prompt: Option<String>,
    icon: Option<String>,
) -> Result<Action, String> {
    db.create_action(&name, description.as_deref(), prompt.as_deref(), icon.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_action(
    db: State<'_, Database>,
    id: String,
    name: Option<String>,
    description: Option<String>,
    prompt: Option<String>,
    icon: Option<String>,
) -> Result<bool, String> {
    db.update_action(&id, name.as_deref(), description.as_deref(), prompt.as_deref(), icon.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_action(
    db: State<'_, Database>,
    id: String,
) -> Result<bool, String> {
    db.delete_action(&id).map_err(|e| e.to_string())
}
