use tauri::AppHandle;
use tauri_plugin_autostart::ManagerExt as AutostartExt;
use tauri_plugin_global_shortcut::GlobalShortcutExt;

use crate::settings::store::{load, save, AppSettings};
use crate::state::SharedState;

#[tauri::command]
pub async fn get_settings(app: AppHandle) -> Result<AppSettings, String> {
    load(&app).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_settings(
    app: AppHandle,
    state: tauri::State<'_, SharedState>,
    settings: AppSettings,
) -> Result<(), String> {
    // Re-register hotkey if it changed
    let current = load(&app).map_err(|e| e.to_string())?;
    if current.hotkey != settings.hotkey {
        let _ = app.global_shortcut().unregister_all();
        let shared = state.inner().clone();
        crate::hotkey::manager::register(&app, &settings.hotkey, shared)
            .map_err(|e| e.to_string())?;
    }

    // Toggle autostart
    let autostart = app.autolaunch();
    if settings.autostart {
        autostart.enable().map_err(|e| e.to_string())?;
    } else {
        autostart.disable().map_err(|e| e.to_string())?;
    }

    save(&app, &settings).map_err(|e| e.to_string())
}
