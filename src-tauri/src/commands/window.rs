use tauri::Manager;

#[tauri::command]
pub fn window_minimize(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.minimize().map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn window_maximize(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.maximize().map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn window_close(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.close().map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn window_is_maximized(app_handle: tauri::AppHandle) -> Result<bool, String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.is_maximized().map_err(|e| e.to_string())
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub fn snap_to_meeting_mode(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.set_size(tauri::LogicalSize::new(300.0, 60.0)).map_err(|e| e.to_string())?;
        window.set_position(tauri::LogicalPosition::new(0.0, 0.0)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn restore_from_meeting_mode(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.set_size(tauri::LogicalSize::new(400.0, 500.0)).map_err(|e| e.to_string())?;
        window.center().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn start_window_drag(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.start_dragging().map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn stop_window_drag() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn set_main_window_interactivity(
    app_handle: tauri::AppHandle,
    interactive: bool,
) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.set_ignore_cursor_events(!interactive).map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn set_notification_interactivity(
    app_handle: tauri::AppHandle,
    _interactive: bool,
) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.set_ignore_cursor_events(!true).map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn resize_main_window(app_handle: tauri::AppHandle, size_key: String) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        let size = match size_key.as_str() {
            "small" => tauri::LogicalSize::new(300.0, 200.0),
            "medium" => tauri::LogicalSize::new(400.0, 500.0),
            "large" => tauri::LogicalSize::new(600.0, 700.0),
            _ => tauri::LogicalSize::new(400.0, 500.0),
        };
        window.set_size(size).map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}

// Hotkey management
#[tauri::command]
pub fn update_hotkey(_hotkey: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn set_hotkey_listening_mode(_enabled: bool, _new_hotkey: Option<String>) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_hotkey_mode_info() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "isUsingGnome": false,
        "isUsingHyprland": false,
        "isUsingNativeShortcut": false,
    }))
}

#[tauri::command]
pub fn register_cancel_hotkey(_key: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn unregister_cancel_hotkey() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn register_meeting_hotkey(_hotkey: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_dictation_key() -> Result<Option<String>, String> {
    Ok(None)
}

#[tauri::command]
pub fn get_active_dictation_key() -> Result<Option<String>, String> {
    Ok(None)
}

#[tauri::command]
pub fn get_effective_default_hotkey() -> Result<String, String> {
    if cfg!(target_os = "macos") {
        Ok("GLOBE".to_string())
    } else {
        Ok("`".to_string())
    }
}

#[tauri::command]
pub fn save_dictation_key(_key: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_activation_mode() -> Result<Option<String>, String> {
    Ok(Some("tap".to_string()))
}

#[tauri::command]
pub fn save_activation_mode(_mode: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_stt_config() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "provider": "local",
        "model": "base",
    }))
}

#[tauri::command]
pub fn get_note_recording_config() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "autoSave": true,
        "format": "txt",
    }))
}
