use parking_lot::Mutex;
use serde::Serialize;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::audio::{capturer::AudioCapturer, capturer::resample, encoder::pcm_to_wav_bytes};
use crate::commands::history::{append_entry, HistoryEntry};
use crate::settings::store::load as load_settings;
use crate::state::{AppStatus, SharedState};

#[derive(Clone, Serialize)]
pub struct StatusPayload {
    pub status: String,
}

#[derive(Clone, Serialize)]
pub struct TranscriptPayload {
    pub raw: String,
    pub polished: Option<String>,
    pub timestamp: u64,
}

#[derive(Clone, Serialize)]
pub struct ErrorPayload {
    pub message: String,
}

type CaptureHandle = Arc<Mutex<Option<AudioCapturer>>>;

pub fn register(
    app: &AppHandle,
    hotkey: &str,
    state: SharedState,
) -> anyhow::Result<()> {
    let capture_handle: CaptureHandle = Arc::new(Mutex::new(None));

    app.global_shortcut().on_shortcut(hotkey, {
        let app = app.clone();
        let state = state.clone();
        let capture_handle = capture_handle.clone();

        move |_app, _shortcut, event| {
            match event.state() {
                ShortcutState::Pressed => {
                    let mut s = state.lock();
                    if s.recording {
                        return;
                    }
                    s.recording = true;
                    s.status = AppStatus::Recording;
                    drop(s);

                    let _ = app.emit("status-changed", StatusPayload { status: "recording".into() });

                    match AudioCapturer::start() {
                        Ok(capturer) => {
                            *capture_handle.lock() = Some(capturer);
                        }
                        Err(e) => {
                            log::error!("Failed to start audio capturer: {}", e);
                            let mut s = state.lock();
                            s.recording = false;
                            s.status = AppStatus::Error;
                            let _ = app.emit("error-occurred", ErrorPayload { message: e.to_string() });
                        }
                    }
                }

                ShortcutState::Released => {
                    let capturer = capture_handle.lock().take();
                    let Some(capturer) = capturer else { return };

                    {
                        let mut s = state.lock();
                        s.recording = false;
                        s.status = AppStatus::Transcribing;
                    }
                    let _ = app.emit("status-changed", StatusPayload { status: "transcribing".into() });

                    let app = app.clone();
                    let state = state.clone();

                    tokio::spawn(async move {
                        if let Err(e) = run_pipeline(app.clone(), state.clone(), capturer).await {
                            log::error!("Pipeline error: {}", e);
                            let mut s = state.lock();
                            s.status = AppStatus::Error;
                            let _ = app.emit("error-occurred", ErrorPayload { message: e.to_string() });
                        }
                        let mut s = state.lock();
                        s.status = AppStatus::Idle;
                        let _ = app.emit("status-changed", StatusPayload { status: "idle".into() });
                    });
                }
            }
        }
    })?;

    Ok(())
}

async fn run_pipeline(
    app: AppHandle,
    _state: SharedState,
    capturer: AudioCapturer,
) -> anyhow::Result<()> {
    // Stop recording → get PCM samples
    let (samples, native_rate) = capturer.stop()?;
    if samples.is_empty() {
        anyhow::bail!("No audio recorded");
    }

    // Resample to 16kHz for Whisper
    let samples_16k = resample(&samples, native_rate, 16000);
    let wav_bytes = pcm_to_wav_bytes(&samples_16k, 16000)?;

    // Load current settings
    let settings = load_settings(&app)?;
    if settings.groq_api_key.is_empty() {
        anyhow::bail!("Groq API key not configured. Open Settings to add it.");
    }

    // STT
    let raw = crate::stt::groq::transcribe(wav_bytes, &settings.groq_api_key, &settings.language_hint).await?;
    log::info!("Transcript: {}", raw);

    // Optional LLM polish
    let polished = if settings.polish_enabled && !settings.llm_api_key.is_empty() {
        let _ = app.emit("status-changed", StatusPayload { status: "polishing".into() });
        match crate::llm::polisher::polish(&raw, &settings.llm_api_key, &settings.llm_base_url, &settings.llm_model).await {
            Ok(p) => Some(p),
            Err(e) => {
                log::warn!("LLM polish failed, using raw transcript: {}", e);
                None
            }
        }
    } else {
        None
    };

    let text_to_inject = polished.as_deref().unwrap_or(&raw);

    // Inject text
    crate::injection::clipboard::inject_via_clipboard(&app, text_to_inject).await?;

    // Persist to history store and emit to frontend
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    append_entry(&app, HistoryEntry { raw: raw.clone(), polished: polished.clone(), timestamp });

    let _ = app.emit("transcript-ready", TranscriptPayload {
        raw,
        polished,
        timestamp,
    });

    Ok(())
}
