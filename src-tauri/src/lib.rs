mod audio;
mod commands;
mod hotkey;
mod injection;
mod llm;
mod settings;
mod stt;
mod state;
mod tray;

use state::{AppState, SharedState};
use std::sync::Arc;
use parking_lot::Mutex;
use tauri::{Manager, WindowEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    let shared_state: SharedState = Arc::new(Mutex::new(AppState::new()));

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .manage(shared_state.clone())
        .setup(move |app| {
            // Build system tray (registered internally by Tauri's resource manager)
            tray::menu::setup(app.handle())?;

            // Load settings and register hotkey
            let settings = settings::store::load(app.handle()).unwrap_or_default();
            hotkey::manager::register(app.handle(), &settings.hotkey, shared_state.clone())
                .unwrap_or_else(|e| log::error!("Failed to register hotkey: {}", e));

            // Hide from dock on macOS (tray-only app)
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            // Open settings window on first run (no API key configured yet)
            if settings.groq_api_key.is_empty() {
                if let Some(win) = app.handle().get_webview_window("main") {
                    let _ = win.show();
                    let _ = win.set_focus();
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            // Hide instead of close when user clicks X
            if let WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap_or_default();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::settings::get_settings,
            commands::settings::save_settings,
            commands::audio::get_input_devices,
            commands::history::get_history,
            commands::history::clear_history,
            commands::permissions::check_accessibility_permission,
            commands::permissions::request_accessibility_permission,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
