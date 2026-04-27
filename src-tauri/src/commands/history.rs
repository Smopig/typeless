use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const STORE_PATH: &str = "history.json";
const HISTORY_KEY: &str = "history";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub raw: String,
    pub polished: Option<String>,
    pub timestamp: u64,
}

#[tauri::command]
pub async fn get_history(app: AppHandle) -> Result<Vec<HistoryEntry>, String> {
    let store = app.store(STORE_PATH).map_err(|e| e.to_string())?;
    match store.get(HISTORY_KEY) {
        Some(val) => serde_json::from_value(val).map_err(|e| e.to_string()),
        None => Ok(vec![]),
    }
}

#[tauri::command]
pub async fn clear_history(app: AppHandle) -> Result<(), String> {
    let store = app.store(STORE_PATH).map_err(|e| e.to_string())?;
    store.delete(HISTORY_KEY);
    store.save().map_err(|e| e.to_string())
}

pub fn append_entry(app: &AppHandle, entry: HistoryEntry) {
    let Ok(store) = app.store(STORE_PATH) else { return };
    let mut history: Vec<HistoryEntry> = match store.get(HISTORY_KEY) {
        Some(val) => serde_json::from_value(val).unwrap_or_default(),
        None => vec![],
    };
    history.insert(0, entry);
    history.truncate(200);
    let _ = store.set(HISTORY_KEY, serde_json::to_value(history).unwrap());
    let _ = store.save();
}
