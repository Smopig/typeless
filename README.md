# Typeless

[繁體中文](README.zh.md) | English

A cross-platform AI voice input app. Hold a hotkey, speak, release — your words appear in any app, instantly.

Built with Tauri v2 (Rust + React), powered by Groq Whisper for fast speech-to-text.

---

## How It Works

```
Hold hotkey → Microphone records → Release
    → Groq Whisper STT (~200ms)
    → [Optional] LLM polish (remove filler words, fix grammar)
    → Text pasted into whatever app is currently focused
```

Works in VS Code, Slack, Gmail, browsers, terminals — anywhere with a text cursor.

---

## Features

- **Push-to-talk** — hold the hotkey to record, release to transcribe and inject
- **System-wide** — works in any app without plugins or browser extensions
- **Chinese/English mixed input** — natural code-switching with Groq Whisper
- **Auto-punctuation** — commas, periods, and Chinese punctuation（，。！？）added automatically
- **LLM polish (optional)** — removes 呃/嗯/啊/um/uh, fixes grammar, preserves meaning
- **Dynamic tray icon** — grey (idle) → red (recording) → blue (processing)
- **Tray-only** — lives in the menu bar / system tray, never in your Dock
- **Transcript history** — last 200 recordings with one-click copy

---

## Quick Start (Install as a macOS App)

The fastest way to install and run Typeless as a native `.app`:

```bash
git clone https://github.com/smopig/typeless
cd typeless
npm install
bash install-mac.sh   # builds release .app and installs to /Applications
```

After install, open **Settings → System → Launch at login** so Typeless starts automatically every time you boot. You never need to open a terminal again.

---

## Development Mode

```bash
npm install
npx tauri dev
```

---

## First-Time Setup

1. The app starts as a **tray icon** in the menu bar — click it → **Open Settings**
2. Enter your **Groq API key** from [console.groq.com](https://console.groq.com/) (free tier, no credit card)
3. Set your preferred hotkey (see [How to Set Your Hotkey](#how-to-set-your-hotkey) below)
4. **macOS only:** grant Accessibility permission when the orange banner appears
5. Click **Save Settings**

Now hold your hotkey anywhere, speak, and release.

---

## How to Set Your Hotkey

1. In Settings, click the **Hotkey** field — it turns blue and shows "Press hotkey combination…"
2. While the field is active, press your desired key combination on the keyboard
   - Examples: hold `Cmd + Shift`, then press `.` → saves as `⌘⇧.`
   - Or press a single function key like `F5` → saves as `F5`
3. The field immediately shows the new hotkey in readable format (e.g. `⌘⇧.`)
4. Click **Save Settings** to apply

> **Tip:** Avoid `Cmd+Space` (Spotlight), `Cmd+Shift+Space` (Chinese IME), or any hotkey your other apps already use.

---

## Settings Reference

### Voice Input

| Setting | Default | Description |
|---------|---------|-------------|
| **Hotkey** | `⌘⇧.` | Hold this key combination to start recording. Release to transcribe. |
| **Language** | Chinese / 中文 | Tells Whisper which language to expect. Use "Chinese" for Chinese-English mixed input; "Auto" lets Whisper guess. |

### Groq API (Speech-to-Text)

| Setting | Description |
|---------|-------------|
| **Groq API Key** | Required. Get a free key at [console.groq.com](https://console.groq.com/). Starts with `gsk_`. Transcription uses `whisper-large-v3-turbo`, which processes ~1 minute of audio in ~200ms. |

### AI Text Polish (Optional)

This section lets an LLM clean up the raw transcript before it is pasted. It is completely optional — Groq Whisper already produces good output on its own.

| Setting | Description |
|---------|-------------|
| **Enable** | Toggle whether LLM polish runs after transcription. When off, the raw Whisper transcript is pasted directly. |
| **LLM API Key** | API key for your chosen LLM provider. Can be an OpenAI key (`sk-…`), an Anthropic key, an OpenRouter key, or any other OpenAI-compatible provider. |
| **Base URL** | The API endpoint. Default is OpenAI (`https://api.openai.com/v1`). Change this to use a different provider — for example `https://openrouter.ai/api/v1` for OpenRouter, or `http://localhost:11434/v1` for local Ollama. |
| **Model** | The model name to call for polishing. Must match what your provider expects — e.g. `gpt-4o-mini`, `claude-haiku-4-5-20251001`, `llama3` (Ollama), or any model available on your chosen Base URL. |

**What polish does:** removes filler words (呃 嗯 啊 那個 um uh like you know), fixes grammar errors introduced by speech recognition, and preserves all technical terms, proper nouns, code identifiers, and URLs. It does **not** translate or change meaning.

### System

| Setting | Description |
|---------|-------------|
| **Launch at login** | Start Typeless automatically when you log into macOS/Windows. Recommended once you've finished setup. |

---

## Tray Icon Colors

| Color | Meaning |
|-------|---------|
| Grey | Idle — waiting for hotkey |
| Red | Recording — microphone is active, speak now |
| Blue | Processing — transcribing or polishing |

---

## macOS Accessibility Permission

macOS requires Accessibility permission for two things:
1. **Global hotkey** — detecting the hotkey even when Typeless is not the focused app
2. **Paste simulation** — injecting text into other apps via Cmd+V

**How to grant it:**
1. Open Typeless Settings — you'll see an **orange warning banner** if permission is missing
2. Click **Open Accessibility Settings** in the banner
3. In **System Settings → Privacy & Security → Accessibility**, find Typeless and toggle it on
4. Switch back to Typeless — the banner disappears automatically (no restart needed)

> If the banner does not disappear after granting, try quitting and restarting Typeless once.

---

## Platform Support

| Platform | Status | Text Injection |
|----------|--------|----------------|
| macOS 12+ | ✅ | osascript Cmd+V (requires Accessibility) |
| Windows 10/11 | ✅ | SendInput Ctrl+V |
| Linux | ⚠️ Experimental | xdotool Ctrl+V |

---

## Troubleshooting

**Hotkey does nothing on macOS**
→ Accessibility permission is missing. Check for the orange banner in Settings.

**Text appears in History but not pasted into the app**
→ Same cause. Grant Accessibility permission.

**"Groq API key not configured" error**
→ Open Settings and add your Groq API key.

**Transcription is in the wrong language**
→ Change the Language setting. Use "Chinese / 中文" for Chinese or mixed Chinese-English.

**No punctuation in the transcript**
→ Make sure Language is set correctly (not "Auto"). The punctuation prompt is language-specific.

**LLM polish is very slow**
→ Switch to a faster model like `gpt-4o-mini`, or disable polish entirely.

---

## Building for Release

```bash
# macOS — quick install to /Applications
bash install-mac.sh

# macOS — universal binary (Intel + Apple Silicon)
npx tauri build --target universal-apple-darwin

# Windows
npx tauri build

# Output locations:
# macOS:   src-tauri/target/universal-apple-darwin/release/bundle/dmg/*.dmg
# Windows: src-tauri/target/release/bundle/nsis/*.exe
```

---

## Project Structure

```
typeless/
├── install-mac.sh              # Build + install to /Applications
├── src/                        # React frontend
│   ├── App.tsx                 # Root: event subscriptions, tab layout
│   ├── store/appStore.ts       # Zustand global state
│   ├── lib/
│   │   ├── commands.ts         # Tauri IPC wrappers (invoke)
│   │   ├── events.ts           # Tauri event listeners
│   │   └── types.ts            # Shared TypeScript types
│   ├── components/             # StatusIndicator, HotkeyPicker, ApiKeyField, …
│   └── pages/                  # SettingsPage, HistoryPage
└── src-tauri/src/              # Rust backend
    ├── audio/                  # cpal capture + WAV encoding + resampling
    ├── stt/groq.rs             # Groq Whisper API client
    ├── llm/                    # LLM polish (OpenAI-compatible)
    ├── injection/              # Clipboard paste injection (Mac/Win/Linux)
    ├── hotkey/manager.rs       # Global hotkey + full pipeline orchestration
    ├── tray/                   # System tray icon (colour changes with status)
    ├── settings/store.rs       # Persistent settings via plugin-store
    └── commands/               # Tauri IPC commands
```

---

## License

MIT
