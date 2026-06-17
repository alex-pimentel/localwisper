use std::collections::HashMap;

use keyring::Entry;
use reqwest::Client;
use serde_json::{json, Value};
use tauri::State;
use tokio::sync::oneshot;

use crate::commands::dictation::{start_recording_internal, DictationState};
use crate::db::agent_conversations::{AgentConversation, AgentMessage};
use crate::db::Database;

#[tauri::command]
pub fn create_agent_conversation(
    db: State<'_, Database>,
    title: Option<String>,
    note_id: Option<String>,
) -> Result<AgentConversation, String> {
    db.create_agent_conversation(title.as_deref(), note_id.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_agent_conversations(
    db: State<'_, Database>,
    limit: Option<i64>,
) -> Result<Vec<AgentConversation>, String> {
    db.get_agent_conversations(limit.unwrap_or(50)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_agent_conversation(
    db: State<'_, Database>,
    id: String,
) -> Result<Option<AgentConversation>, String> {
    db.get_agent_conversation(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_agent_conversation(
    db: State<'_, Database>,
    id: String,
) -> Result<bool, String> {
    db.delete_agent_conversation(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_agent_conversation_title(
    db: State<'_, Database>,
    id: String,
    title: String,
) -> Result<bool, String> {
    db.update_agent_conversation_title(&id, &title).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_agent_message(
    db: State<'_, Database>,
    conversation_id: String,
    role: String,
    content: String,
    metadata: Option<String>,
) -> Result<AgentMessage, String> {
    db.add_agent_message(&conversation_id, &role, &content, metadata.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_agent_messages(
    db: State<'_, Database>,
    conversation_id: String,
) -> Result<Vec<AgentMessage>, String> {
    db.get_agent_messages(&conversation_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn archive_agent_conversation(
    db: State<'_, Database>,
    id: String,
) -> Result<bool, String> {
    db.archive_agent_conversation(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn unarchive_agent_conversation(
    db: State<'_, Database>,
    id: String,
) -> Result<bool, String> {
    db.unarchive_agent_conversation(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_conversations_for_note(
    db: State<'_, Database>,
    note_id: String,
    limit: Option<i64>,
) -> Result<Vec<AgentConversation>, String> {
    db.get_conversations_for_note(&note_id, limit.unwrap_or(20))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_agent_conversations(
    db: State<'_, Database>,
    query: String,
    limit: Option<i64>,
) -> Result<Vec<AgentConversation>, String> {
    db.search_agent_conversations(&query, limit.unwrap_or(20))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_agent_conversation_cloud_id(
    db: State<'_, Database>,
    id: String,
    cloud_id: String,
) -> Result<bool, String> {
    db.update_agent_conversation_cloud_id(&id, &cloud_id).map_err(|e| e.to_string())
}

const SERVICE_NAME: &str = "lightwisper";

fn get_key(key_name: &str) -> Result<Option<String>, String> {
    match Entry::new(SERVICE_NAME, key_name) {
        Ok(entry) => match entry.get_password() {
            Ok(password) => Ok(Some(password)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(e.to_string()),
        },
        Err(_) => Ok(None),
    }
}

fn build_message_history(messages: &[AgentMessage]) -> Vec<HashMap<String, String>> {
    messages
        .iter()
        .map(|m| {
            let mut map = HashMap::new();
            map.insert("role".to_string(), m.role.clone());
            map.insert("content".to_string(), m.content.clone());
            map
        })
        .collect()
}

async fn call_openai_compatible(
    client: &Client,
    api_url: &str,
    model: &str,
    api_key: &str,
    messages: &[HashMap<String, String>],
) -> Result<String, String> {
    let body = json!({
        "model": model,
        "messages": messages,
        "max_tokens": 2048,
    });

    let resp = client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("HTTP error: {}", e))?;

    let status = resp.status();
    let text = resp.text().await.map_err(|e| format!("read error: {}", e))?;

    if !status.is_success() {
        return Err(format!("API error ({}): {}", status, text));
    }

    let v: Value = serde_json::from_str(&text).map_err(|e| format!("parse error: {}", e))?;
    let content = v["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| "no content in response".to_string())?;

    Ok(content.to_string())
}

async fn call_anthropic(
    client: &Client,
    api_key: &str,
    messages: &[HashMap<String, String>],
) -> Result<String, String> {
    let body = json!({
        "model": "claude-3-haiku-20240307",
        "max_tokens": 2048,
        "messages": messages,
    });

    let resp = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("HTTP error: {}", e))?;

    let status = resp.status();
    let text = resp.text().await.map_err(|e| format!("read error: {}", e))?;

    if !status.is_success() {
        return Err(format!("Anthropic API error ({}): {}", status, text));
    }

    let v: Value = serde_json::from_str(&text).map_err(|e| format!("parse error: {}", e))?;
    let content = v["content"][0]["text"]
        .as_str()
        .ok_or_else(|| "no text in response".to_string())?;

    Ok(content.to_string())
}

async fn call_gemini(
    client: &Client,
    api_key: &str,
    messages: &[HashMap<String, String>],
) -> Result<String, String> {
    let contents: Vec<Value> = messages
        .iter()
        .map(|m| {
            let role = if m["role"] == "assistant" {
                "model"
            } else {
                "user"
            };
            json!({
                "role": role,
                "parts": [{"text": m["content"]}]
            })
        })
        .collect();

    let body = json!({ "contents": contents });

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

    let resp = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("HTTP error: {}", e))?;

    let status = resp.status();
    let text = resp.text().await.map_err(|e| format!("read error: {}", e))?;

    if !status.is_success() {
        return Err(format!("Gemini API error ({}): {}", status, text));
    }

    let v: Value = serde_json::from_str(&text).map_err(|e| format!("parse error: {}", e))?;
    let content = v["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or_else(|| "no text in response".to_string())?;

    Ok(content.to_string())
}

async fn call_ollama(
    client: &Client,
    model: &str,
    messages: &[HashMap<String, String>],
) -> Result<String, String> {
    let body = json!({
        "model": model,
        "messages": messages,
        "stream": false,
    });

    let resp = client
        .post("http://localhost:11434/api/chat")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ollama connection error: {} — is Ollama running?", e))?;

    let status = resp.status();
    let text = resp.text().await.map_err(|e| format!("read error: {}", e))?;

    if !status.is_success() {
        return Err(format!("Ollama API error ({}): {}", status, text));
    }

    let v: Value = serde_json::from_str(&text).map_err(|e| format!("parse error: {}", e))?;
    let content = v["message"]["content"]
        .as_str()
        .ok_or_else(|| "no content in Ollama response".to_string())?;

    Ok(content.to_string())
}

enum ProviderKind {
    Ollama { model: &'static str },
    OpenAICompatible { url: &'static str, model: &'static str },
    Anthropic,
    Gemini,
}

struct ProviderConfig {
    key_name: &'static str,
    name: &'static str,
    kind: ProviderKind,
    requires_key: bool,
}

impl ProviderConfig {
    const fn new_noauth(name: &'static str, kind: ProviderKind) -> Self {
        Self {
            key_name: "",
            name,
            kind,
            requires_key: false,
        }
    }

    const fn new(key_name: &'static str, name: &'static str, kind: ProviderKind) -> Self {
        Self {
            key_name,
            name,
            kind,
            requires_key: true,
        }
    }
}

async fn try_provider(
    client: &Client,
    config: &ProviderConfig,
    api_key: Option<&str>,
    messages: &[HashMap<String, String>],
) -> Result<String, String> {
    match &config.kind {
        ProviderKind::Ollama { model } => call_ollama(client, model, messages).await,
        ProviderKind::OpenAICompatible { url, model } => {
            call_openai_compatible(client, url, model, api_key.unwrap_or(""), messages).await
        }
        ProviderKind::Anthropic => call_anthropic(client, api_key.unwrap_or(""), messages).await,
        ProviderKind::Gemini => call_gemini(client, api_key.unwrap_or(""), messages).await,
    }
}

async fn try_all_providers(
    client: &Client,
    messages: &[HashMap<String, String>],
) -> Result<String, String> {
    let providers = [
        ProviderConfig::new_noauth("Ollama", ProviderKind::Ollama { model: "llama3.2" }),
        ProviderConfig::new("openai", "OpenAI", ProviderKind::OpenAICompatible {
            url: "https://api.openai.com/v1/chat/completions",
            model: "gpt-4o-mini",
        }),
        ProviderConfig::new("anthropic", "Anthropic", ProviderKind::Anthropic),
        ProviderConfig::new("gemini", "Gemini", ProviderKind::Gemini),
        ProviderConfig::new("groq", "Groq", ProviderKind::OpenAICompatible {
            url: "https://api.groq.com/openai/v1/chat/completions",
            model: "mixtral-8x7b-32768",
        }),
        ProviderConfig::new("xai", "xAI", ProviderKind::OpenAICompatible {
            url: "https://api.x.ai/v1/chat/completions",
            model: "grok-2",
        }),
        ProviderConfig::new("mistral", "Mistral", ProviderKind::OpenAICompatible {
            url: "https://api.mistral.ai/v1/chat/completions",
            model: "mistral-small-latest",
        }),
    ];

    let mut last_error = String::new();

    for config in &providers {
        let api_key = if config.requires_key {
            match get_key(config.key_name) {
                Ok(Some(k)) => Some(k),
                Ok(None) => {
                    last_error = format!("{}: no API key configured", config.name);
                    continue;
                }
                Err(e) => {
                    last_error = format!("{} key error: {}", config.name, e);
                    continue;
                }
            }
        } else {
            None
        };

        match try_provider(client, config, api_key.as_deref(), messages).await {
            Ok(response) => return Ok(response),
            Err(e) => {
                last_error = format!("{} error: {}", config.name, e);
            }
        }
    }

    Err(last_error)
}

// Send message to AI provider and get response
#[tauri::command]
pub async fn send_agent_message(
    db: State<'_, Database>,
    conversation_id: String,
    message: String,
) -> Result<String, String> {
    db.add_agent_message(&conversation_id, "user", &message, None)
        .map_err(|e| e.to_string())?;

    let messages = db
        .get_agent_messages(&conversation_id)
        .map_err(|e| e.to_string())?;

    let history = build_message_history(&messages);

    let client = Client::new();
    let response = try_all_providers(&client, &history).await?;

    db.add_agent_message(&conversation_id, "assistant", &response, None)
        .map_err(|e| e.to_string())?;

    Ok(response)
}

// Voice dictation routed to agent
#[tauri::command]
pub async fn start_voice_agent_dictation(
    state: State<'_, DictationState>,
    app_handle: tauri::AppHandle,
    _conversation_id: String,
) -> Result<String, String> {
    let (tx, rx) = oneshot::channel::<String>();
    *state.voice_agent_tx.lock() = Some(tx);

    start_recording_internal(&state, &app_handle).await?;

    let text = tokio::time::timeout(
        std::time::Duration::from_secs(120),
        rx,
    )
    .await
    .map_err(|_| "voice dictation timed out after 120 seconds".to_string())?
    .map_err(|_| "recording cancelled".to_string())?;

    Ok(text)
}

// Agent web search tool
#[tauri::command]
pub async fn agent_web_search(query: String, num_results: Option<u32>) -> Result<String, String> {
    let count = num_results.unwrap_or(5).min(10);
    let client = Client::new();

    // Try DuckDuckGo Lite API first (no API key needed)
    let url = format!(
        "https://lite.duckduckgo.com/lite/?q={}",
        urlencoding(&query)
    );

    match client.get(&url).send().await {
        Ok(resp) => {
            let html = resp.text().await.unwrap_or_default();
            let mut results = Vec::new();
            let mut in_result = false;
            let mut current = String::new();

            for line in html.lines() {
                if line.contains("class=\"result-link\"") {
                    in_result = true;
                    current.clear();
                }
                if in_result {
                    current.push_str(line);
                    if line.contains("</a>") {
                        // Extract text between > and <
                        if let Some(start) = current.find('>') {
                            if let Some(end) = current[start..].find("</a>") {
                                let text = &current[start + 1..start + end];
                                let clean = text
                                    .replace("<b>", "")
                                    .replace("</b>", "")
                                    .trim()
                                    .to_string();
                                if !clean.is_empty() && results.len() < count as usize {
                                    results.push(clean);
                                }
                            }
                        }
                        in_result = false;
                    }
                }
            }

            if results.is_empty() {
                // Fallback: try DuckDuckGo HTML scraping
                let fallback_url = format!("https://html.duckduckgo.com/html/?q={}", urlencoding(&query));
                match client.get(&fallback_url).header("User-Agent", "Mozilla/5.0").send().await {
                    Ok(fb_resp) => {
                        let fb_html = fb_resp.text().await.unwrap_or_default();
                        for line in fb_html.lines() {
                            if line.contains("class=\"result__a\"") && results.len() < count as usize {
                                if let Some(start) = line.find('>') {
                                    let after = &line[start + 1..];
                                    if let Some(end) = after.find("</a>") {
                                        let text = after[..end]
                                            .replace("<b>", "")
                                            .replace("</b>", "")
                                            .trim()
                                            .to_string();
                                        if !text.is_empty() {
                                            results.push(text);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => {}
                }
            }

            if results.is_empty() {
                Err("no search results found".to_string())
            } else {
                Ok(results.join("\n"))
            }
        }
        Err(e) => Err(format!("web search failed: {}", e)),
    }
}

fn urlencoding(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "+".to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}

#[tauri::command]
pub fn agent_open_note(
    db: State<'_, Database>,
    note_id: String,
) -> Result<Option<crate::db::notes::Note>, String> {
    db.get_note(&note_id).map_err(|e| e.to_string())
}

// Agent overlay
#[tauri::command]
pub fn toggle_agent_overlay() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn hide_agent_overlay() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn resize_agent_window(_width: f64, _height: f64) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_agent_window_bounds() -> Result<String, String> {
    Ok("{}".to_string())
}

#[tauri::command]
pub fn set_agent_window_bounds(_x: f64, _y: f64, _width: f64, _height: f64) -> Result<(), String> {
    Ok(())
}

// Dictation preview
#[tauri::command]
pub fn start_dictation_preview(_opts: Option<String>) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn stop_dictation_preview(_opts: Option<String>) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn dismiss_dictation_preview() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn complete_dictation_preview(_payload: Option<String>) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn hide_dictation_preview() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn resize_transcription_preview_window(_width: f64, _height: f64) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn acquire_recording_lock(_pipeline: String) -> Result<bool, String> {
    Ok(true)
}

#[tauri::command]
pub fn release_recording_lock(_pipeline: String) -> Result<(), String> {
    Ok(())
}

// Agent hotkeys
#[tauri::command]
pub fn update_agent_hotkey(_hotkey: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn update_voice_agent_hotkey(_hotkey: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_voice_agent_key() -> Result<Option<String>, String> {
    Ok(None)
}

#[tauri::command]
pub fn save_voice_agent_key(_key: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_agent_key() -> Result<Option<String>, String> {
    Ok(None)
}

#[tauri::command]
pub fn save_agent_key(_key: String) -> Result<(), String> {
    Ok(())
}

// Local reasoning via llama.cpp server
#[tauri::command]
pub async fn process_local_reasoning(
    text: String,
    _model_id: String,
    _agent_name: Option<String>,
    _config: Option<String>,
) -> Result<String, String> {
    let client = Client::new();
    let body = json!({
        "prompt": text,
        "n_predict": 512,
        "temperature": 0.7,
        "stop": ["</s>", "User:", "user:"],
        "cache_prompt": true,
    });

    let resp = client
        .post("http://localhost:8080/completion")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("llama.cpp connection error: {} — is the local server running?", e))?;

    let status = resp.status();
    let response_text = resp.text().await.map_err(|e| format!("read error: {}", e))?;

    if !status.is_success() {
        return Err(format!("llama.cpp API error ({}): {}", status, response_text));
    }

    let v: Value = serde_json::from_str(&response_text)
        .map_err(|e| format!("parse error: {}", e))?;

    v["content"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "no content in llama.cpp response".to_string())
}

#[tauri::command]
pub async fn check_local_reasoning_available() -> Result<bool, String> {
    let client = Client::new();
    match client
        .get("http://localhost:8080/health")
        .send()
        .await
    {
        Ok(resp) => Ok(resp.status().is_success()),
        Err(_) => Ok(false),
    }
}
