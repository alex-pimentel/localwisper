#[tauri::command]
pub fn check_for_updates() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({"hasUpdate": false}))
}

#[tauri::command]
pub fn download_update() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn install_update() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_update_status() -> Result<String, String> {
    Ok("up-to-date".to_string())
}

#[tauri::command]
pub fn get_update_info() -> Result<Option<serde_json::Value>, String> {
    Ok(None)
}

#[tauri::command]
pub fn get_update_notification_data() -> Result<Option<serde_json::Value>, String> {
    Ok(None)
}

#[tauri::command]
pub fn update_notification_ready() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn update_notification_respond(_action: String) -> Result<(), String> {
    Ok(())
}
