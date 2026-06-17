use serde_json::Value;

#[tauri::command]
pub fn cloud_health_check() -> Result<Value, String> {
    Ok(serde_json::json!({"status": "unavailable"}))
}

#[tauri::command]
pub fn cloud_transcribe(
    _audio_buffer: Vec<f64>,
    _opts: Option<String>,
) -> Result<String, String> {
    Err("cloud transcription not yet implemented".to_string())
}

#[tauri::command]
pub fn cloud_reason(
    _text: String,
    _opts: Option<String>,
) -> Result<String, String> {
    Err("cloud reasoning not yet implemented".to_string())
}

#[tauri::command]
pub fn cloud_streaming_usage(
    _text: String,
    _audio_duration_seconds: f64,
    _opts: Option<String>,
) -> Result<Value, String> {
    Ok(serde_json::json!({"usage": 0}))
}

#[tauri::command]
pub fn cloud_usage() -> Result<Value, String> {
    Ok(serde_json::json!({"transcriptionSeconds": 0, "aiTokens": 0}))
}

#[tauri::command]
pub fn cloud_checkout(_opts: Option<String>) -> Result<String, String> {
    Err("cloud checkout not yet implemented".to_string())
}

#[tauri::command]
pub fn cloud_billing_portal() -> Result<String, String> {
    Err("cloud billing portal not yet implemented".to_string())
}

#[tauri::command]
pub fn cloud_switch_plan(_opts: Option<String>) -> Result<(), String> {
    Err("cloud plan switching not yet implemented".to_string())
}

#[tauri::command]
pub fn cloud_preview_switch(_opts: Option<String>) -> Result<Value, String> {
    Ok(serde_json::json!({"preview": {}}))
}

#[tauri::command]
pub fn cloud_api_request(_opts: Option<String>) -> Result<Value, String> {
    Err("cloud API request not yet implemented".to_string())
}

// llama.cpp
#[tauri::command]
pub fn llama_cpp_check() -> Result<bool, String> {
    Ok(which::which("llama-server").is_ok())
}

#[tauri::command]
pub fn llama_cpp_install() -> Result<(), String> {
    Err("llama.cpp install not yet implemented".to_string())
}

#[tauri::command]
pub fn llama_cpp_uninstall() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn llama_server_start(_model_id: String) -> Result<(), String> {
    Err("llama-server start not yet implemented".to_string())
}

#[tauri::command]
pub fn llama_server_stop() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn llama_server_status() -> Result<String, String> {
    Ok("inactive".to_string())
}

#[tauri::command]
pub fn llama_gpu_reset() -> Result<(), String> {
    Ok(())
}

// Vulkan GPU
#[tauri::command]
pub fn detect_vulkan_gpu() -> Result<Value, String> {
    Ok(serde_json::json!({"available": false}))
}

#[tauri::command]
pub fn get_llama_vulkan_status() -> Result<String, String> {
    Ok("not available".to_string())
}

#[tauri::command]
pub fn download_llama_vulkan_binary() -> Result<(), String> {
    Err("Vulkan download not yet implemented".to_string())
}

#[tauri::command]
pub fn cancel_llama_vulkan_download() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn delete_llama_vulkan_binary() -> Result<(), String> {
    Ok(())
}

// Notification
#[tauri::command]
pub fn notify_limit_reached(_data: Value) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn notify_activation_mode_changed(_mode: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn notify_hotkey_changed(_hotkey: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn notify_floating_icon_auto_hide_changed(_enabled: bool) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn notify_panel_start_position_changed(_position: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn notify_start_minimized_changed(_enabled: bool) -> Result<(), String> {
    Ok(())
}

// Agent cloud stream
#[tauri::command]
pub fn start_agent_stream(
    _messages: Vec<Value>,
    _opts: Option<String>,
) -> Result<(), String> {
    Err("agent cloud stream not yet implemented".to_string())
}
