import { invoke } from "@tauri-apps/api/core";
import type { AppSettings, HistoryEntry } from "./types";

export const getSettings = () => invoke<AppSettings>("get_settings");

export const saveSettings = (settings: AppSettings) =>
  invoke<void>("save_settings", { settings });

export const getInputDevices = () => invoke<string[]>("get_input_devices");

export const getHistory = () => invoke<HistoryEntry[]>("get_history");

export const clearHistory = () => invoke<void>("clear_history");

export const checkAccessibilityPermission = () =>
  invoke<boolean>("check_accessibility_permission");

export const requestAccessibilityPermission = () =>
  invoke<void>("request_accessibility_permission");
