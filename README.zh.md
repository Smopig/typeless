# Typeless

繁體中文 | [English](README.md)

跨平台 AI 語音輸入工具。按住快捷鍵、說話、放開——文字自動出現在任何 app 裡。

基於 Tauri v2（Rust + React）開發，使用 Groq Whisper 實現極速語音辨識。

---

## 運作原理

```
按住快捷鍵 → 麥克風錄音 → 放開
    → Groq Whisper 語音轉文字（~200ms）
    → [選用] LLM 文字潤飾（去除贅字、修正語法）
    → 貼入目前聚焦的 app
```

支援 VS Code、Slack、Gmail、瀏覽器、終端機——任何有文字游標的地方都能用。

---

## 功能特色

- **按住說話** — 按住快捷鍵錄音，放開即轉錄並注入文字
- **全系統通用** — 無需安裝插件或瀏覽器擴充功能，任何 app 都能用
- **中英混打** — 自然處理中英夾雜的語音輸入
- **自動標點** — 自動加上逗號、句號、中文標點（，。！？）
- **LLM 潤飾（選用）** — 自動移除 呃/嗯/啊/那個/um/uh，修正語法，保留原意
- **系統匣圖示變色** — 灰色（待機）→ 紅色（錄音中）→ 藍色（處理中）
- **系統匣常駐** — 只住在選單列，不佔 Dock
- **轉錄歷史** — 保留最近 200 筆，附一鍵複製

---

## 快速安裝（macOS App）

最快的方式——直接打包安裝成原生 `.app`：

```bash
git clone https://github.com/smopig/typeless
cd typeless
npm install
bash install-mac.sh   # 自動打包並安裝到 /Applications
```

安裝完成後，在 **Settings → System → Launch at login** 勾選開機自啟，之後完全不需要開終端機。

---

## 開發模式執行

```bash
npm install
npx tauri dev
```

---

## 首次設定步驟

1. App 啟動後只出現**選單列圖示** — 點擊它 → **Open Settings**
2. 前往 [console.groq.com](https://console.groq.com/) 申請 **Groq API key**（免費、不需信用卡）並填入
3. 設定快捷鍵（參考下方[如何設定快捷鍵](#如何設定快捷鍵)）
4. **僅 macOS：** 出現橘色警告時授予 Accessibility 權限
5. 點選 **Save Settings**

設定完成後，在任何地方按住快捷鍵說話，放開即完成。

---

## 如何設定快捷鍵

1. 在 Settings 畫面，用滑鼠**點一下 Hotkey 欄位**
2. 欄位變藍、顯示「Press hotkey combination…」表示進入捕捉模式
3. 直接在鍵盤上按下你想要的組合鍵：
   - 範例：按住 `Cmd + Shift`，再按 `.` → 儲存為 `⌘⇧.`
   - 或直接按單一功能鍵，例如 `F5` → 儲存為 `F5`
4. 欄位立即顯示新的快捷鍵（例如 `⌘⇧.`）
5. 點 **Save Settings** 讓設定生效

> **提示：** 避免使用 `Cmd+Space`（Spotlight）、`Cmd+Shift+Space`（中文輸入法）、或其他 app 已佔用的快捷鍵。

---

## 設定說明

### 語音輸入

| 設定項目 | 預設值 | 說明 |
|---------|--------|------|
| **Hotkey** | `⌘⇧.` | 按住此鍵開始錄音，放開即轉錄。可自訂（見上方說明）。 |
| **Language** | Chinese / 中文 | 告訴 Whisper 預期的語言。中文或中英混打請選「Chinese / 中文」；不確定時可選「Auto」（準確度略低）。 |

### Groq API（語音轉文字）

| 設定項目 | 說明 |
|---------|------|
| **Groq API Key** | 必填。在 [console.groq.com](https://console.groq.com/) 免費申請，key 開頭為 `gsk_`。使用 `whisper-large-v3-turbo` 模型，約 200ms 完成一次轉錄。 |

### AI 文字潤飾（選用）

這個區塊可以讓 LLM（大型語言模型）在語音轉文字後，先對文字進行清理再貼入。**完全選用**——Groq Whisper 本身已經能輸出相當準確的結果，不開啟也沒問題。

| 設定項目 | 說明 |
|---------|------|
| **Enable（啟用）** | 開關。開啟後，每次轉錄完會先呼叫 LLM 潤飾，再貼文字。關閉時直接貼 Whisper 原始結果。 |
| **LLM API Key** | 你選用的 LLM 服務的 API key。可以是 OpenAI 的 key（`sk-…`）、Anthropic 的 key、OpenRouter 的 key，或任何 OpenAI 相容格式的 key。 |
| **Base URL** | LLM API 的端點網址。預設是 OpenAI（`https://api.openai.com/v1`）。若要換服務，填入對應網址，例如：OpenRouter → `https://openrouter.ai/api/v1`，本地 Ollama → `http://localhost:11434/v1`。 |
| **Model** | 呼叫哪個模型進行潤飾。必須與你的 Base URL 服務提供的模型名稱一致，例如：`gpt-4o-mini`、`claude-haiku-4-5-20251001`、`llama3`（Ollama）等。 |

**潤飾做什麼：** 移除贅詞（呃 嗯 啊 那個 就是 um uh like），修正語音辨識帶來的語法錯誤，保留所有技術術語、專有名詞、程式碼識別字和網址。**不翻譯、不改變原意**。

### 系統

| 設定項目 | 說明 |
|---------|------|
| **Launch at login（開機自啟）** | 登入 macOS / Windows 時自動啟動 Typeless。建議設定完畢後開啟，之後完全不需要手動啟動。 |

---

## 系統匣圖示顏色說明

| 顏色 | 狀態 |
|------|------|
| 灰色 | 待機——等待快捷鍵 |
| 紅色 | 錄音中——麥克風啟動，請說話 |
| 藍色 | 處理中——轉錄或潤飾中 |

---

## macOS Accessibility 權限

macOS 需要 Accessibility 權限才能做兩件事：
1. **全域快捷鍵偵測**——即使 Typeless 不在前景也能感知快捷鍵
2. **貼上模擬**——透過 Apple Events 將文字注入其他 app

**授權步驟：**
1. 開啟 Typeless Settings——若缺少權限，頂部會出現**橘色警告橫幅**
2. 點選橫幅上的 **Open Accessibility Settings**
3. 在**系統設定 → 隱私權與安全性 → 輔助使用**中，找到 Typeless 並開啟
4. 切回 Typeless——橘色橫幅自動消失（不需要重新啟動）

> 若橫幅沒有自動消失，試著完全結束 Typeless 再重新開啟一次。

---

## 平台支援

| 平台 | 狀態 | 文字注入方式 |
|------|------|------------|
| macOS 12+ | ✅ | osascript Cmd+V（需要 Accessibility 權限） |
| Windows 10/11 | ✅ | SendInput Ctrl+V |
| Linux | ⚠️ 實驗性 | xdotool Ctrl+V |

---

## 常見問題

**按了快捷鍵但 macOS 沒有反應**
→ 缺少 Accessibility 權限。打開 Settings 查看是否有橘色橫幅。

**History 裡有內容但文字沒有貼入 app**
→ 同上，授予 Accessibility 權限即可解決。

**出現「Groq API key not configured」錯誤**
→ 打開 Settings 填入 Groq API key。

**辨識語言不對**
→ 將 Language 改為對應的語言。中文或中英混打請選「Chinese / 中文」。

**沒有標點符號**
→ 確認 Language 設定正確（不要用 Auto）。標點符號的提示是針對特定語言的。

**LLM 潤飾很慢**
→ 換用較快的模型（如 `gpt-4o-mini`），或直接關閉潤飾功能。

---

## 打包發布

```bash
# macOS — 快速安裝到 /Applications
bash install-mac.sh

# macOS — 通用二進位（Intel + Apple Silicon）
npx tauri build --target universal-apple-darwin

# Windows
npx tauri build

# 輸出位置：
# macOS:   src-tauri/target/universal-apple-darwin/release/bundle/dmg/*.dmg
# Windows: src-tauri/target/release/bundle/nsis/*.exe
```

---

## 專案結構

```
typeless/
├── install-mac.sh              # 一鍵打包安裝到 /Applications
├── src/                        # React 前端
│   ├── App.tsx                 # 根元件：事件訂閱、分頁佈局
│   ├── store/appStore.ts       # Zustand 全域狀態
│   ├── lib/
│   │   ├── commands.ts         # Tauri IPC 呼叫封裝
│   │   ├── events.ts           # Tauri 事件監聽封裝
│   │   └── types.ts            # 共用 TypeScript 型別
│   ├── components/             # StatusIndicator、HotkeyPicker、ApiKeyField …
│   └── pages/                  # SettingsPage、HistoryPage
└── src-tauri/src/              # Rust 後端
    ├── audio/                  # cpal 錄音、WAV 編碼、降採樣至 16kHz
    ├── stt/groq.rs             # Groq Whisper API 客戶端
    ├── llm/                    # LLM 潤飾（OpenAI 相容）
    ├── injection/              # Clipboard 貼上注入（Mac / Win / Linux）
    ├── hotkey/manager.rs       # 全域快捷鍵 + Pipeline 串接
    ├── tray/                   # 系統匣圖示（狀態對應顏色）
    ├── settings/store.rs       # 設定持久化（plugin-store）
    └── commands/               # Tauri IPC commands
```

---

## 授權

MIT
