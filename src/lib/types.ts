export interface AppSettings {
  groqApiKey: string;
  llmApiKey: string;
  llmBaseUrl: string;
  llmModel: string;
  hotkey: string;
  languageHint: string;
  polishEnabled: boolean;
  autostart: boolean;
}

export interface HistoryEntry {
  raw: string;
  polished: string | null;
  timestamp: number;
}

export type AppStatus =
  | "idle"
  | "recording"
  | "transcribing"
  | "polishing"
  | "error";

export interface StatusPayload {
  status: AppStatus;
}

export interface TranscriptPayload {
  raw: string;
  polished: string | null;
  timestamp: number;
}

export interface ErrorPayload {
  message: string;
}
