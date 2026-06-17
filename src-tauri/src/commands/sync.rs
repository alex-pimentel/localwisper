use tauri::State;

use crate::db::Database;
use crate::db::notes::Note;
use crate::db::agent_conversations::AgentConversation;

// Note sync
#[tauri::command]
pub fn get_pending_notes(
    db: State<'_, Database>,
) -> Result<Vec<Note>, String> {
    db.get_pending_notes().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_pending_note_deletes(
    db: State<'_, Database>,
) -> Result<Vec<String>, String> {
    db.get_pending_note_deletes().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_note_by_client_id(
    db: State<'_, Database>,
    client_note_id: String,
) -> Result<Option<Note>, String> {
    db.get_note_by_client_id(&client_note_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn upsert_note_from_cloud(
    db: State<'_, Database>,
    cloud_note: Note,
    local_folder_id: Option<String>,
) -> Result<Note, String> {
    db.upsert_note_from_cloud(&cloud_note, local_folder_id.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn mark_note_synced(
    db: State<'_, Database>,
    id: String,
    cloud_id: String,
) -> Result<bool, String> {
    db.mark_note_synced(&id, &cloud_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn mark_note_sync_error(
    db: State<'_, Database>,
    id: String,
) -> Result<bool, String> {
    db.mark_note_sync_error(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn hard_delete_note(
    db: State<'_, Database>,
    id: String,
) -> Result<bool, String> {
    db.hard_delete_note(&id).map_err(|e| e.to_string())
}

// Conversation sync
#[tauri::command]
pub fn get_pending_conversations(
    db: State<'_, Database>,
) -> Result<Vec<AgentConversation>, String> {
    db.get_pending_conversations().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_pending_conversation_deletes(
    db: State<'_, Database>,
) -> Result<Vec<String>, String> {
    db.get_pending_conversation_deletes().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_conversation_by_client_id(
    db: State<'_, Database>,
    client_id: String,
) -> Result<Option<AgentConversation>, String> {
    db.get_conversation_by_client_id(&client_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn upsert_conversation_from_cloud(
    db: State<'_, Database>,
    cloud_conv: AgentConversation,
) -> Result<AgentConversation, String> {
    db.upsert_conversation_from_cloud(&cloud_conv).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn mark_conversation_synced(
    db: State<'_, Database>,
    id: String,
    cloud_id: String,
) -> Result<bool, String> {
    db.mark_conversation_synced(&id, &cloud_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn hard_delete_conversation(
    db: State<'_, Database>,
    id: String,
) -> Result<bool, String> {
    db.hard_delete_conversation(&id).map_err(|e| e.to_string())
}

// Transcription sync
#[tauri::command]
pub fn get_pending_transcriptions(
    db: State<'_, Database>,
) -> Result<Vec<crate::db::transcriptions::Transcription>, String> {
    db.get_pending_transcriptions().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_transcription_by_client_id(
    db: State<'_, Database>,
    client_id: String,
) -> Result<Option<crate::db::transcriptions::Transcription>, String> {
    db.get_transcription_by_client_id(&client_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn upsert_transcription_from_cloud(
    db: State<'_, Database>,
    cloud_transcription: crate::db::transcriptions::Transcription,
) -> Result<crate::db::transcriptions::Transcription, String> {
    db.upsert_transcription_from_cloud(&cloud_transcription).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn mark_transcription_synced(
    db: State<'_, Database>,
    id: String,
    cloud_id: String,
) -> Result<bool, String> {
    db.mark_transcription_synced(&id, &cloud_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_pending_transcription_deletes(
    db: State<'_, Database>,
) -> Result<Vec<String>, String> {
    db.get_pending_transcription_deletes().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn hard_delete_transcription(
    db: State<'_, Database>,
    id: String,
) -> Result<bool, String> {
    db.hard_delete_transcription(&id).map_err(|e| e.to_string())
}
