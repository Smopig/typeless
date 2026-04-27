use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;

pub async fn inject_via_clipboard(app: &AppHandle, text: &str) -> anyhow::Result<()> {
    // Save existing clipboard
    let previous = app.clipboard().read_text().ok();

    // Write transcript to clipboard
    app.clipboard().write_text(text)?;

    // Let the clipboard settle
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    // Simulate Cmd+V / Ctrl+V
    simulate_paste()?;

    // Restore original clipboard after paste completes
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    if let Some(prev) = previous {
        let _ = app.clipboard().write_text(&prev);
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn simulate_paste() -> anyhow::Result<()> {
    crate::injection::mac::simulate_paste()
}

#[cfg(target_os = "windows")]
fn simulate_paste() -> anyhow::Result<()> {
    crate::injection::windows::simulate_paste()
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn simulate_paste() -> anyhow::Result<()> {
    // Linux: use xdotool if available
    std::process::Command::new("xdotool")
        .args(["key", "--clearmodifiers", "ctrl+v"])
        .output()
        .ok();
    Ok(())
}
