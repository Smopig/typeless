pub async fn transcribe(
    wav_bytes: Vec<u8>,
    api_key: &str,
    language_hint: &str,
) -> anyhow::Result<String> {
    let client = reqwest::Client::new();

    let part = reqwest::multipart::Part::bytes(wav_bytes)
        .file_name("audio.wav")
        .mime_str("audio/wav")?;

    // Prompt guides Whisper to use correct punctuation for the target language
    let prompt = match language_hint {
        "zh" => "以下是普通話或粵語語音，請加上中文標點符號（，。！？；：「」）。",
        "en" => "Please use proper English punctuation including commas, periods, and question marks.",
        _ => "Please include proper punctuation marks.",
    };

    let mut form = reqwest::multipart::Form::new()
        .part("file", part)
        .text("model", "whisper-large-v3-turbo")
        .text("response_format", "text")
        .text("prompt", prompt);

    // "auto" means omit the language field → Whisper auto-detects
    if language_hint != "auto" {
        form = form.text("language", language_hint.to_string());
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
