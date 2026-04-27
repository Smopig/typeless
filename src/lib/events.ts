import { listen } from "@tauri-apps/api/event";
import type {
  StatusPayload,
  TranscriptPayload,
  ErrorPayload,
} from "./types";

export const onStatusChanged = (
  cb: (payload: StatusPayload) => void
) => listen<StatusPayload>("status-changed", (e) => cb(e.payload));

export const onTranscriptReady = (
  cb: (payload: TranscriptPayload) => void
) => listen<TranscriptPayload>("transcript-ready", (e) => cb(e.payload));

export const onErrorOccurred = (
  cb: (payload: ErrorPayload) => void
) => listen<ErrorPayload>("error-occurred", (e) => cb(e.payload));
