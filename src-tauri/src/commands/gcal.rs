use serde_json::Value;

// Google Calendar
#[tauri::command]
pub fn gcal_start_oauth() -> Result<(), String> {
    Err("Google Calendar OAuth not yet implemented".to_string())
}

#[tauri::command]
pub fn gcal_disconnect() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn gcal_get_connection_status() -> Result<Value, String> {
    Ok(serde_json::json!({"connected": false}))
}

#[tauri::command]
pub fn gcal_get_calendars() -> Result<Vec<Value>, String> {
    Ok(Vec::new())
}

#[tauri::command]
pub fn gcal_set_calendar_selection(_calendar_id: String, _is_selected: bool) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn gcal_set_primary_only(_value: bool) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn gcal_sync_events() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn gcal_get_upcoming_events(_window_minutes: Option<i64>) -> Result<Vec<Value>, String> {
    Ok(Vec::new())
}

#[tauri::command]
pub fn gcal_get_event(_event_id: String) -> Result<Option<Value>, String> {
    Ok(None)
}

// Contacts
#[tauri::command]
pub fn search_contacts(_query: String) -> Result<Vec<Value>, String> {
    Ok(Vec::new())
}

#[tauri::command]
pub fn upsert_contact(_contact: Value) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_md5_hash(text: String) -> Result<String, String> {
    let hash = format!("{:x}", md5::compute(text.as_bytes()));
    Ok(hash)
}

// Meeting detection
#[tauri::command]
pub fn meeting_detection_get_preferences() -> Result<Value, String> {
    Ok(serde_json::json!({
        "processDetection": true,
        "audioDetection": true,
        "calendarDetection": false,
    }))
}

#[tauri::command]
pub fn meeting_detection_set_preferences(_prefs: Value) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn sync_notification_preferences(_prefs: Value) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn set_speaker_diarization_enabled(_enabled: bool) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn set_meeting_session_speaker_config(_config: Value) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_whisper_vad_config() -> Result<Value, String> {
    Ok(serde_json::json!({
        "threshold": 0.5,
        "minSpeechDurationMs": 100,
        "minSilenceDurationMs": 500,
    }))
}

#[tauri::command]
pub fn set_whisper_vad_config(_config: Value) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_meeting_notification_data() -> Result<Option<Value>, String> {
    Ok(None)
}

#[tauri::command]
pub fn meeting_notification_ready() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn meeting_notification_respond(_detection_id: String, _action: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn join_calendar_meeting(_event_id: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_pending_meeting_note_navigation() -> Result<Option<Value>, String> {
    Ok(None)
}
