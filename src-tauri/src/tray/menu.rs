use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Runtime,
};

pub fn setup<R: Runtime>(app: &AppHandle<R>) -> anyhow::Result<TrayIcon<R>> {
    let open = MenuItem::with_id(app, "open", "Open Settings", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "Quit Typeless", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&open, &separator, &quit])?;

    let icon = tauri::image::Image::from_bytes(include_bytes!("../../icons/tray-idle.png"))
        .expect("tray-idle.png embedded at compile time");

    let tray = TrayIconBuilder::with_id("main")
        .icon(icon)
        .tooltip("Typeless — idle")
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "open" => show_settings_window(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::DoubleClick { .. } = event {
                show_settings_window(tray.app_handle());
            }
        })
        .build(app)?;

    Ok(tray)
}

pub fn show_settings_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.show();
        let _ = win.set_focus();
    }
}

/// Update tray icon and tooltip to reflect current pipeline status.
pub fn set_status_icon<R: Runtime>(app: &AppHandle<R>, status: &str) {
    let icon_bytes: &[u8] = match status {
        "recording" => include_bytes!("../../icons/tray-recording.png"),
        "transcribing" | "polishing" => include_bytes!("../../icons/tray-processing.png"),
        _ => include_bytes!("../../icons/tray-idle.png"),
    };
    let tooltip = match status {
        "recording" => "Typeless — recording…",
        "transcribing" => "Typeless — transcribing…",
        "polishing" => "Typeless — polishing…",
        _ => "Typeless — idle",
    };
    if let Some(tray) = app.tray_by_id("main") {
        if let Ok(icon) = tauri::image::Image::from_bytes(icon_bytes) {
            let _ = tray.set_icon(Some(icon));
        }
        let _ = tray.set_tooltip(Some(tooltip));
    }
}

