use std::path::PathBuf;

use keyring::Entry;
use tauri::Manager;

#[tauri::command]
pub fn get_openai_key() -> Result<Option<String>, String> {
    get_key("openai")
}

#[tauri::command]
pub fn save_openai_key(key: String) -> Result<(), String> {
    save_key("openai", &key)
}

#[tauri::command]
pub fn get_anthropic_key() -> Result<Option<String>, String> {
    get_key("anthropic")
}

#[tauri::command]
pub fn save_anthropic_key(key: String) -> Result<(), String> {
    save_key("anthropic", &key)
}

#[tauri::command]
pub fn get_gemini_key() -> Result<Option<String>, String> {
    get_key("gemini")
}

#[tauri::command]
pub fn save_gemini_key(key: String) -> Result<(), String> {
    save_key("gemini", &key)
}

#[tauri::command]
pub fn get_groq_key() -> Result<Option<String>, String> {
    get_key("groq")
}

#[tauri::command]
pub fn save_groq_key(key: String) -> Result<(), String> {
    save_key("groq", &key)
}

#[tauri::command]
pub fn get_xai_key() -> Result<Option<String>, String> {
    get_key("xai")
}

#[tauri::command]
pub fn save_xai_key(key: String) -> Result<(), String> {
    save_key("xai", &key)
}

#[tauri::command]
pub fn get_mistral_key() -> Result<Option<String>, String> {
    get_key("mistral")
}

#[tauri::command]
pub fn save_mistral_key(key: String) -> Result<(), String> {
    save_key("mistral", &key)
}

#[tauri::command]
pub fn get_corti_client_id() -> Result<Option<String>, String> {
    get_key("corti_client_id")
}

#[tauri::command]
pub fn save_corti_client_id(key: String) -> Result<(), String> {
    save_key("corti_client_id", &key)
}

#[tauri::command]
pub fn get_corti_client_secret() -> Result<Option<String>, String> {
    get_key("corti_client_secret")
}

#[tauri::command]
pub fn save_corti_client_secret(key: String) -> Result<(), String> {
    save_key("corti_client_secret", &key)
}

#[tauri::command]
pub fn get_custom_transcription_key() -> Result<Option<String>, String> {
    get_key("custom_transcription")
}

#[tauri::command]
pub fn save_custom_transcription_key(key: String) -> Result<(), String> {
    save_key("custom_transcription", &key)
}

#[tauri::command]
pub fn get_cleanup_custom_key() -> Result<Option<String>, String> {
    get_key("cleanup_custom")
}

#[tauri::command]
pub fn save_cleanup_custom_key(key: String) -> Result<(), String> {
    save_key("cleanup_custom", &key)
}

#[tauri::command]
pub fn get_ui_language() -> Result<Option<String>, String> {
    Ok(Some("en".to_string()))
}

#[tauri::command]
pub fn save_ui_language(language: String) -> Result<(), String> {
    let _ = language;
    Ok(())
}

#[tauri::command]
pub fn set_ui_language(_language: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_log_level() -> Result<String, String> {
    Ok("info".to_string())
}

#[tauri::command]
pub fn app_log(entry: String) -> Result<(), String> {
    tracing::info!("[renderer] {}", entry);
    Ok(())
}

#[tauri::command]
pub fn get_debug_state() -> Result<bool, String> {
    Ok(false)
}

#[tauri::command]
pub fn set_debug_logging(_enabled: bool) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn open_logs_folder() -> Result<(), String> {
    let log_dir = log_dir()?;
    std::fs::create_dir_all(&log_dir).map_err(|e| e.to_string())?;
    open::that(&log_dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_auto_start_enabled() -> Result<bool, String> {
    Ok(false)
}

#[tauri::command]
pub fn set_auto_start_enabled(_enabled: bool) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_ydotool_status() -> Result<String, String> {
    Ok("unknown".to_string())
}

#[tauri::command]
pub fn get_hyprland_config_status() -> Result<String, String> {
    Ok("not applicable".to_string())
}

#[tauri::command]
pub fn get_post_migration_state() -> Result<bool, String> {
    Ok(false)
}

#[tauri::command]
pub fn get_oauth_protocol_registered() -> Result<bool, String> {
    Ok(false)
}

#[tauri::command]
pub fn get_oauth_protocol() -> Result<Option<String>, String> {
    Ok(None)
}

#[tauri::command]
pub fn mark_bundle_migrated() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn mark_bundle_migration_dismissed() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_platform() -> String {
    if cfg!(target_os = "macos") {
        "darwin".to_string()
    } else if cfg!(target_os = "windows") {
        "win32".to_string()
    } else {
        "linux".to_string()
    }
}

#[tauri::command]
pub fn app_quit() -> Result<(), String> {
    std::process::exit(0);
}

#[tauri::command]
pub fn cleanup_app() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn open_external(url: String) -> Result<(), String> {
    open::that(&url).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn paste_text(_text: String, _options: Option<String>) -> Result<(), String> {
    Err("clipboard paste not yet implemented in Rust".to_string())
}

#[tauri::command]
pub fn hide_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.hide().map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn show_dictation_panel(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn check_accessibility_permission(_silent: Option<bool>) -> Result<bool, String> {
    Ok(true)
}

#[tauri::command]
pub fn prompt_accessibility_permission() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn read_clipboard() -> Result<String, String> {
    Err("clipboard read via JS API instead".to_string())
}

#[tauri::command]
pub fn write_clipboard(_text: String) -> Result<(), String> {
    Err("clipboard write via JS API instead".to_string())
}

#[tauri::command]
pub fn check_paste_tools() -> Result<Vec<String>, String> {
    let mut tools = Vec::new();
    if which::which("xdotool").is_ok() {
        tools.push("xdotool".to_string());
    }
    if which::which("wtype").is_ok() {
        tools.push("wtype".to_string());
    }
    if which::which("ydotool").is_ok() {
        tools.push("ydotool".to_string());
    }
    Ok(tools)
}

#[tauri::command]
pub fn check_accessibility_trusted() -> Result<bool, String> {
    Ok(true)
}

#[tauri::command]
pub fn request_microphone_access() -> Result<bool, String> {
    Ok(true)
}

#[tauri::command]
pub fn check_microphone_access() -> Result<bool, String> {
    Ok(true)
}

#[tauri::command]
pub fn check_system_audio_access() -> Result<bool, String> {
    Ok(true)
}

#[tauri::command]
pub fn request_system_audio_access() -> Result<bool, String> {
    Ok(true)
}

#[tauri::command]
pub fn open_microphone_settings() -> Result<(), String> {
    open_system_settings("microphone")
}

#[tauri::command]
pub fn open_sound_input_settings() -> Result<(), String> {
    open_system_settings("sound_input")
}

#[tauri::command]
pub fn open_accessibility_settings() -> Result<(), String> {
    open_system_settings("accessibility")
}

#[tauri::command]
pub fn open_system_audio_settings() -> Result<(), String> {
    open_system_settings("system_audio")
}

#[tauri::command]
pub fn toggle_media_playback() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn pause_media_playback() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn resume_media_playback() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn auth_clear_session() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn auth_get_token() -> Result<Option<String>, String> {
    Ok(None)
}

#[tauri::command]
pub fn auth_set_token(_token: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_app_version() -> Result<String, String> {
    Ok("0.1.0".to_string())
}

#[tauri::command]
pub fn proxy_xai_transcription(_data: String) -> Result<String, String> {
    Err("XAI proxy not yet implemented".to_string())
}

#[tauri::command]
pub fn proxy_mistral_transcription(_data: String) -> Result<String, String> {
    Err("Mistral proxy not yet implemented".to_string())
}

#[tauri::command]
pub fn proxy_corti_transcription(_data: String) -> Result<String, String> {
    Err("Corti proxy not yet implemented".to_string())
}

#[tauri::command]
pub fn save_all_keys_to_env() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn set_auto_learn_flag_enabled(_enabled: bool) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn sync_startup_preferences(_prefs: String) -> Result<(), String> {
    Ok(())
}

fn get_key(key_name: &str) -> Result<Option<String>, String> {
    match Entry::new("lightwisper", key_name) {
        Ok(entry) => match entry.get_password() {
            Ok(password) => Ok(Some(password)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(e.to_string()),
        },
        Err(_) => Ok(None),
    }
}

fn save_key(key_name: &str, value: &str) -> Result<(), String> {
    let entry = Entry::new("lightwisper", key_name).map_err(|e| e.to_string())?;
    entry.set_password(value).map_err(|e| e.to_string())
}

fn log_dir() -> Result<PathBuf, String> {
    let base = dirs::data_dir().ok_or_else(|| "no data dir".to_string())?;
    Ok(base.join("lightwisper").join("logs"))
}

fn open_system_settings(kind: &str) -> Result<(), String> {
    let url = if cfg!(target_os = "macos") {
        match kind {
            "microphone" => "x-apple.systempreferences:com.apple.preference.security?Privacy_Microphone",
            "sound_input" => "x-apple.systempreferences:com.apple.preference.sound?input",
            "accessibility" => "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility",
            "system_audio" => "x-apple.systempreferences:com.apple.preference.sound?input",
            _ => return Ok(()),
        }
    } else if cfg!(target_os = "windows") {
        match kind {
            "microphone" => "ms-settings:privacy-microphone",
            "sound_input" => "ms-settings:sound",
            "system_audio" => "ms-settings:sound",
            _ => return Ok(()),
        }
    } else {
        return Ok(());
    };
    open::that(url).map_err(|e| e.to_string())
}
