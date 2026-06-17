#[tauri::command]
pub fn assembly_ai_streaming_warmup(_options: Option<String>) -> Result<(), String> {
    Err("AssemblyAI streaming not yet implemented".to_string())
}

#[tauri::command]
pub fn assembly_ai_streaming_start(_options: Option<String>) -> Result<(), String> {
    Err("AssemblyAI streaming not yet implemented".to_string())
}

#[tauri::command]
pub fn assembly_ai_streaming_send(_audio_buffer: Vec<f64>) -> Result<(), String> {
    Err("AssemblyAI streaming not yet implemented".to_string())
}

#[tauri::command]
pub fn assembly_ai_streaming_force_endpoint() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn assembly_ai_streaming_stop() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn assembly_ai_streaming_status() -> Result<String, String> {
    Ok("inactive".to_string())
}

#[tauri::command]
pub fn deepgram_streaming_warmup(_options: Option<String>) -> Result<(), String> {
    Err("Deepgram streaming not yet implemented".to_string())
}

#[tauri::command]
pub fn deepgram_streaming_start(_options: Option<String>) -> Result<(), String> {
    Err("Deepgram streaming not yet implemented".to_string())
}

#[tauri::command]
pub fn deepgram_streaming_send(_audio_buffer: Vec<f64>) -> Result<(), String> {
    Err("Deepgram streaming not yet implemented".to_string())
}

#[tauri::command]
pub fn deepgram_streaming_finalize() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn deepgram_streaming_stop() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn deepgram_streaming_status() -> Result<String, String> {
    Ok("inactive".to_string())
}

#[tauri::command]
pub fn corti_streaming_warmup(_options: Option<String>) -> Result<(), String> {
    Err("Corti streaming not yet implemented".to_string())
}

#[tauri::command]
pub fn corti_streaming_start(_options: Option<String>) -> Result<(), String> {
    Err("Corti streaming not yet implemented".to_string())
}

#[tauri::command]
pub fn corti_streaming_send(_audio_buffer: Vec<f64>) -> Result<(), String> {
    Err("Corti streaming not yet implemented".to_string())
}

#[tauri::command]
pub fn corti_streaming_finalize() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn corti_streaming_stop() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn corti_streaming_status() -> Result<String, String> {
    Ok("inactive".to_string())
}

// Meeting transcription (streaming, dual-channel)
#[tauri::command]
pub fn meeting_transcription_prepare(_options: Option<String>) -> Result<(), String> {
    Err("meeting transcription not yet implemented".to_string())
}

#[tauri::command]
pub fn meeting_transcription_start(_options: Option<String>) -> Result<(), String> {
    Err("meeting transcription not yet implemented".to_string())
}

#[tauri::command]
pub fn meeting_transcription_send(_buffer: Vec<f64>, _source: String) -> Result<(), String> {
    Err("meeting transcription not yet implemented".to_string())
}

#[tauri::command]
pub fn meeting_transcription_stop() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn meeting_transcription_cancel() -> Result<(), String> {
    Ok(())
}

// Dictation realtime streaming
#[tauri::command]
pub fn dictation_realtime_warmup(_options: Option<String>) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn dictation_realtime_start(_options: Option<String>) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn dictation_realtime_send(_buffer: Vec<f64>) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn dictation_realtime_stop() -> Result<(), String> {
    Ok(())
}
