pub async fn transcribe(
    wav_bytes: Vec<u8>,
    api_key: &str,
    language_hint: &str,
) -> anyhow::Result<String> {
    let client = reqwest::Client::new();

    let part = reqwest::multipart::Part::bytes(wav_bytes)
        .file_name("audio.wav")
        .mime_str("audio/wav")?;

    // zh-TW: Whisper only knows "zh" but we nudge it toward Traditional via prompt
    let (whisper_lang, prompt) = match language_hint {
        "zh-TW" => (
            Some("zh"),
            "以下是繁體中文語音（台灣用語）。請務必以繁體中文輸出，不得使用簡體字。\
             正確使用全形標點符號（，。！？；：「」）。",
        ),
        "zh" => (
            Some("zh"),
            "以下是普通話或粵語語音，請加上中文標點符號（，。！？；：「」）。",
        ),
        "en" => (
            Some("en"),
            "Please use proper English punctuation including commas, periods, and question marks.",
        ),
        _ => (None, "Please include proper punctuation marks."),
    };

    let mut form = reqwest::multipart::Form::new()
        .part("file", part)
        .text("model", "whisper-large-v3-turbo")
        .text("response_format", "text")
        .text("prompt", prompt);

    if let Some(lang) = whisper_lang {
        form = form.text("language", lang);
    }

    let text = client
        .post("https://api.groq.com/openai/v1/audio/transcriptions")
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(text.trim().to_string())
}
