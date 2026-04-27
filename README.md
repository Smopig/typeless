# Typeless

A cross-platform AI voice input app. Hold a hotkey, speak, release — your words appear in any app.

Built with Tauri v2 (Rust + React), powered by Groq Whisper for fast speech-to-text.

---

## How It Works

```
Hold hotkey → Record (microphone) → Release
    → Groq Whisper STT (~200ms)
    → [Optional] LLM polish (remove filler words)
    → Paste text into focused app
```

Works in VS Code, Slack, Gmail, browsers, terminals — anywhere with a text cursor.

---

## Features

- **Push-to-talk** — hold the hotkey to record, release to transcribe and inject
- **System-wide** — works in any app without plugins or browser extensions
- **Chinese/English mixed input** — `language: "zh"` hint enables natural code-switching
- **LLM polish (optional)** — removes 呃/嗯/啊/um/uh, fixes grammar, preserves meaning
- **Tray-only** — lives in the menu bar / system tray, never in your dock
- **Transcript history** — last 200 recordings with copy button

---

## Platform Support

| Platform | Status | Text Injection Method |
|----------|--------|-----------------------|
| macOS 12+ | ✅ | osascript Cmd+V |
| Windows 10/11 | ✅ | SendInput Ctrl+V |
| Linux | ⚠️ Experimental | xdotool Ctrl+V |

---

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) (stable)
- A [Groq API key](https://console.groq.com/) (free tier available)

**macOS system deps:** none beyond Xcode CLT  
**Windows system deps:** none  
**Linux system deps:**
```bash
sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev libappindicator3-dev libasound2-dev
```

### Install & Run

```bash
git clone https://github.com/smopig/typeless
cd typeless
npm install
npx tauri dev
```

### First-Time Setup

1. The app starts as a **tray icon** — right-click → **Open Settings**
2. Enter your **Groq API key** (`gsk_…`) from [console.groq.com](https://console.groq.com/)
3. Set your hotkey (default: `Ctrl/Cmd + Shift + .`)
4. **macOS only:** grant Accessibility permission when prompted, then restart the app
5. Click **Save Settings**

Now hold your hotkey anywhere, speak, and release.

---

## Configuration

All settings are saved to your OS app data directory and persist across restarts.

| Setting | Default | Description |
|---------|---------|-------------|
| Groq API Key | — | Required for STT |
| Hotkey | `CommandOrControl+Shift+Period` | Hold to record |
| Language | `zh` | `zh` (Chinese/mixed), `en`, or `auto` |
| LLM Polish | Off | Remove filler words via LLM |
| LLM API Key | — | Any OpenAI-compatible key |
| LLM Base URL | `https://api.openai.com/v1` | Works with OpenRouter, etc. |
| LLM Model | `gpt-4o-mini` | Any OpenAI-compatible model |
| Launch at login | Off | Start with your OS |

### Recommended LLM models for polish

| Model | Speed | Cost |
|-------|-------|------|
| `gpt-4o-mini` | Fast | ~$0.001/request |
| `claude-haiku-4-5-20251001` | Fast | ~$0.001/request |
| local Ollama | Slow | Free |

---

## macOS Accessibility Permission

macOS requires Accessibility permission for:
1. Global hotkey detection (even when app is not focused)
2. Paste simulation via Apple Events

**Steps:**
1. Open Settings → you'll see a warning banner if permission is missing
2. Click **Open Accessibility Settings**
3. Enable **Typeless** in System Settings → Privacy & Security → Accessibility
4. **Restart the app** (macOS requires a restart after granting — this is an OS limitation)

---

## Project Structure

```
typeless/
├── src/                        # React frontend
│   ├── App.tsx                 # Root component, event subscriptions
│   ├── store/appStore.ts       # Zustand global state
│   ├── lib/
│   │   ├── commands.ts         # Tauri IPC wrappers (invoke)
│   │   ├── events.ts           # Tauri event listeners
│   │   └── types.ts            # Shared TypeScript types
│   ├── components/             # StatusIndicator, HotkeyPicker, ApiKeyField, …
│   └── pages/                  # SettingsPage, HistoryPage
└── src-tauri/src/              # Rust backend
    ├── audio/                  # cpal capture + WAV encoding
    ├── stt/groq.rs             # Groq Whisper API client
    ├── llm/                    # LLM polish (OpenAI-compatible)
    ├── injection/              # Clipboard paste injection (Mac/Win/Linux)
    ├── hotkey/manager.rs       # Global hotkey + pipeline orchestration
    ├── tray/                   # System tray icon and menu
    ├── settings/store.rs       # Persistent settings via plugin-store
    └── commands/               # Tauri IPC commands
```

---

## Building for Release

```bash
# macOS (universal binary — Intel + Apple Silicon)
npx tauri build --target universal-apple-darwin

# Windows
npx tauri build

# Output:
# macOS: src-tauri/target/universal-apple-darwin/release/bundle/dmg/*.dmg
# Windows: src-tauri/target/release/bundle/nsis/*.exe
```

CI/CD via GitHub Actions automatically builds and releases on version tags (`v*`).

---

## Development Notes

**Audio capture** runs on a dedicated `std::thread` (not a tokio task) because `cpal` uses a callback model that is incompatible with async runtimes.

**Text injection** uses clipboard paste rather than raw keyboard simulation because clipboard paste works reliably across all apps. The original clipboard is restored 200ms after injection.

**Default hotkey** is `Cmd/Ctrl+Shift+.` (not Space) — Space conflicts with Chinese IME on macOS.

**Sample rate** — audio is resampled to 16kHz before sending to Whisper (regardless of device native rate), which minimises upload size and matches Whisper's training distribution.

---

## License

MIT
