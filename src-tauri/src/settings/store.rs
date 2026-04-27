use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const STORE_PATH: &str = "settings.json";
const SETTINGS_KEY: &str = "settings";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub groq_api_key: String,
    pub llm_api_key: String,
    pub llm_base_url: String,
    pub llm_model: String,
    pub hotkey: String,
    pub language_hint: String,
    pub polish_enabled: bool,
    pub autostart: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            groq_api_key: String::new(),
            llm_api_key: String::new(),
            llm_base_url: "https://api.openai.com/v1".to_string(),
            llm_model: "gpt-4o-mini".to_string(),
            hotkey: "CommandOrControl+Shift+Period".to_string(),
            language_hint: "zh".to_string(),
            polish_enabled: false,
            autostart: false,
        }
    }
}

pub fn load(app: &AppHandle) -> anyhow::Result<AppSettings> {
    let store = app.store(STORE_PATH)?;
    match store.get(SETTINGS_KEY) {
        Some(val) => Ok(serde_json::from_value(val)?),
        None => Ok(AppSettings::default()),
    }
}

pub fn save(app: &AppHandle, settings: &AppSettings) -> anyhow::Result<()> {
    let store = app.store(STORE_PATH)?;
    store.set(SETTINGS_KEY, serde_json::to_value(settings)?);
    store.save()?;
    Ok(())
}
