use keyring::Entry;

macro_rules! enterprise_key_cmds {
    ($get_fn:ident, $save_fn:ident, $key_name:expr) => {
        #[tauri::command]
        pub fn $get_fn() -> Result<Option<String>, String> {
            get_enterprise_key($key_name)
        }

        #[tauri::command]
        pub fn $save_fn(value: String) -> Result<(), String> {
            save_enterprise_key($key_name, &value)
        }
    };
}

enterprise_key_cmds!(get_bedrock_region, save_bedrock_region, "bedrock_region");
enterprise_key_cmds!(get_bedrock_profile, save_bedrock_profile, "bedrock_profile");
enterprise_key_cmds!(get_bedrock_access_key_id, save_bedrock_access_key_id, "bedrock_access_key_id");
enterprise_key_cmds!(get_bedrock_secret_access_key, save_bedrock_secret_access_key, "bedrock_secret_access_key");
enterprise_key_cmds!(get_bedrock_session_token, save_bedrock_session_token, "bedrock_session_token");
enterprise_key_cmds!(get_azure_endpoint, save_azure_endpoint, "azure_endpoint");
enterprise_key_cmds!(get_azure_api_key, save_azure_api_key, "azure_api_key");
enterprise_key_cmds!(get_azure_deployment, save_azure_deployment, "azure_deployment");
enterprise_key_cmds!(get_azure_api_version, save_azure_api_version, "azure_api_version");
enterprise_key_cmds!(get_vertex_project, save_vertex_project, "vertex_project");
enterprise_key_cmds!(get_vertex_location, save_vertex_location, "vertex_location");
enterprise_key_cmds!(get_vertex_api_key, save_vertex_api_key, "vertex_api_key");

#[tauri::command]
pub fn test_enterprise_connection(_provider: String, _config: Option<String>) -> Result<String, String> {
    Err("enterprise connection test not yet implemented".to_string())
}

#[tauri::command]
pub fn process_enterprise_reasoning(
    _text: String,
    _model_id: String,
    _agent_name: Option<String>,
    _config: Option<String>,
) -> Result<String, String> {
    Err("enterprise reasoning not yet implemented".to_string())
}

fn get_enterprise_key(key_name: &str) -> Result<Option<String>, String> {
    match Entry::new("lightwisper", key_name) {
        Ok(entry) => match entry.get_password() {
            Ok(password) => Ok(Some(password)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(e.to_string()),
        },
        Err(_) => Ok(None),
    }
}

fn save_enterprise_key(key_name: &str, value: &str) -> Result<(), String> {
    let entry = Entry::new("lightwisper", key_name).map_err(|e| e.to_string())?;
    entry.set_password(value).map_err(|e| e.to_string())
}
