# Typeless

繁體中文 | [English](README.md)

跨平台 AI 語音輸入工具。按住快捷鍵、說話、放開——文字自動出現在任何 app 裡。

基於 Tauri v2（Rust + React）開發，使用 Groq Whisper 實現極速語音辨識。

---

## 運作原理

```
按住快捷鍵 → 麥克風錄音 → 放開
    → Groq Whisper 語音轉文字（~200ms）
    → [選用] LLM 文字潤飾（去除贅字）
    → 貼入目前聚焦的 app
```

支援 VS Code、Slack、Gmail、瀏覽器、終端機——任何有文字游標的地方都能用。

---

## 功能特色

- **按住說話** — 按住快捷鍵錄音，放開即轉錄並注入文字
- **全系統通用** — 無需安裝插件或瀏覽器擴充功能
- **中英混打** — 設定 `language: "zh"` 讓 Whisper 自然處理中英夾雜
- **LLM 潤飾（選用）** — 自動移除 呃/嗯/啊/um/uh，修正語法，保留原意
- **系統匣常駐** — 只住在選單列 / 系統匣，不佔 Dock
- **轉錄歷史** — 保留最近 200 筆，附複製按鈕

---

## 平台支援

| 平台 | 狀態 | 文字注入方式 |
|------|------|-------------|
| macOS 12+ | ✅ | osascript Cmd+V |
| Windows 10/11 | ✅ | SendInput Ctrl+V |
| Linux | ⚠️ 實驗性 | xdotool Ctrl+V |

---

## 快速開始

### 環境需求

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/)（stable）
- [Groq API key](https://console.groq.com/)（有免費方案）

**macOS：** 只需 Xcode Command Line Tools  
**Windows：** 無額外需求  
**Linux：**
```bash
sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev libappindicator3-dev libasound2-dev
```

### 安裝與執行

```bash
git clone https://github.com/smopig/typeless
cd typeless
npm install
npx tauri dev
```

### 首次設定

1. App 啟動後只出現**系統匣圖示** — 右鍵 → **Open Settings**
2. 在 [console.groq.com](https://console.groq.com/) 取得 **Groq API key**（`gsk_…`）並填入
3. 設定快捷鍵（預設：`Ctrl/Cmd + Shift + .`）
4. **僅 macOS：** 依提示授予 Accessibility 權限，之後重新啟動 app
5. 點選 **Save Settings**

設定完成後，在任何地方按住快捷鍵、說話、放開即可。

---

## 設定說明

所有設定儲存於 OS 的 app data 目錄，重啟後自動載入。

| 設定項目 | 預設值 | 說明 |
|---------|--------|------|
| Groq API Key | — | 語音轉文字必填 |
| 快捷鍵 | `CommandOrControl+Shift+Period` | 按住錄音 |
| 語言 | `zh` | `zh`（中文/混打）、`en`、`auto` |
| LLM 潤飾 | 關閉 | 用 LLM 移除贅字 |
| LLM API Key | — | 任何 OpenAI 相容的 key |
| LLM Base URL | `https://api.openai.com/v1` | 支援 OpenRouter 等 |
| LLM 模型 | `gpt-4o-mini` | 任何 OpenAI 相容模型 |
| 開機自動啟動 | 關閉 | 隨系統啟動 |

### 推薦 LLM 潤飾模型

| 模型 | 速度 | 費用 |
|------|------|------|
| `gpt-4o-mini` | 快 | ~$0.001 / 次 |
| `claude-haiku-4-5-20251001` | 快 | ~$0.001 / 次 |
| 本地 Ollama | 較慢 | 免費 |

---

## macOS Accessibility 權限

macOS 需要 Accessibility 權限才能：
1. 在 app 未聚焦時偵測全域快捷鍵
2. 透過 Apple Events 模擬貼上動作

**授權步驟：**
1. 開啟設定 → 若缺少權限會出現警告橫幅
2. 點選 **Open Accessibility Settings**
3. 在「系統設定 → 隱私權與安全性 → 輔助使用」中啟用 **Typeless**
4. **重新啟動 app**（macOS 系統限制，授權後必須重啟才能生效）

---

## 專案結構

```
typeless/
├── src/                        # React 前端
│   ├── App.tsx                 # 根元件，訂閱後端事件
│   ├── store/appStore.ts       # Zustand 全域狀態
│   ├── lib/
│   │   ├── commands.ts         # Tauri IPC 呼叫封裝（invoke）
│   │   ├── events.ts           # Tauri 事件監聽封裝
│   │   └── types.ts            # 共用 TypeScript 型別
│   ├── components/             # StatusIndicator、HotkeyPicker、ApiKeyField …
│   └── pages/                  # SettingsPage、HistoryPage
└── src-tauri/src/              # Rust 後端
    ├── audio/                  # cpal 錄音 + WAV 編碼
    ├── stt/groq.rs             # Groq Whisper API 客戶端
    ├── llm/                    # LLM 潤飾（OpenAI 相容）
    ├── injection/              # Clipboard 貼上注入（Mac / Win / Linux）
    ├── hotkey/manager.rs       # 全域快捷鍵 + Pipeline 串接
    ├── tray/                   # 系統匣圖示與選單
    ├── settings/store.rs       # 設定持久化（plugin-store）
    └── commands/               # Tauri IPC commands
```

---

## 打包發布

```bash
# macOS（通用二進位：Intel + Apple Silicon）
npx tauri build --target universal-apple-darwin

# Windows
npx tauri build

# 輸出位置：
# macOS:   src-tauri/target/universal-apple-darwin/release/bundle/dmg/*.dmg
# Windows: src-tauri/target/release/bundle/nsis/*.exe
```

推送 `v*` tag 後，GitHub Actions 會自動建構並發布 Release。

---

## 開發說明

**音訊擷取** 跑在獨立的 `std::thread`（而非 tokio task），因為 `cpal` 使用 callback 模式，與 async runtime 不相容。

**文字注入** 採用 clipboard paste 而非純鍵盤模擬，因為 clipboard paste 在各種 app 中最可靠。注入後 200ms 自動還原原本的剪貼板內容。

**預設快捷鍵** 設為 `Cmd/Ctrl+Shift+.`（非 Space），因為 Space 在 macOS 上會與中文輸入法快捷鍵衝突。

**取樣率** — 音訊送出前一律降採樣至 16kHz（不管裝置原生取樣率），減少上傳大小，同時符合 Whisper 訓練資料的分佈。

---

## 授權

MIT
