use keyring::Entry;
use serde::{Deserialize, Serialize};

const SERVICE_NAME: &str = "lightwisper";

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeys {
    pub openai: Option<String>,
    pub anthropic: Option<String>,
    pub gemini: Option<String>,
    pub groq: Option<String>,
    pub xai: Option<String>,
    pub mistral: Option<String>,
}

fn key_entry(key_name: &str) -> Result<Entry, String> {
    Entry::new(SERVICE_NAME, key_name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_api_key(key_name: String, value: String) -> Result<(), String> {
    let entry = key_entry(&key_name)?;
    entry.set_password(&value).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_api_key(key_name: String) -> Result<Option<String>, String> {
    let entry = match key_entry(&key_name) {
        Ok(e) => e,
        Err(_) => return Ok(None),
    };
    match entry.get_password() {
        Ok(password) => Ok(Some(password)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn delete_api_key(key_name: String) -> Result<(), String> {
    let entry = key_entry(&key_name)?;
    entry.delete_credential().map_err(|e| e.to_string())
}
