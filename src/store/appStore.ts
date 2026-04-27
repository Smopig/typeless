import { create } from "zustand";
import type { AppSettings, AppStatus, HistoryEntry } from "../lib/types";

interface AppStore {
  status: AppStatus;
  settings: AppSettings | null;
  history: HistoryEntry[];
  errorMessage: string | null;
  accessibilityGranted: boolean;

  setStatus: (status: AppStatus) => void;
  setSettings: (settings: AppSettings) => void;
  addHistory: (entry: HistoryEntry) => void;
  setError: (message: string | null) => void;
  setAccessibility: (granted: boolean) => void;
}

export const useAppStore = create<AppStore>((set) => ({
  status: "idle",
  settings: null,
  history: [],
  errorMessage: null,
  accessibilityGranted: true,

  setStatus: (status) => set({ status }),
  setSettings: (settings) => set({ settings }),
  addHistory: (entry) =>
    set((s) => ({
      history: [entry, ...s.history].slice(0, 200),
    })),
  setError: (errorMessage) => set({ errorMessage }),
  setAccessibility: (accessibilityGranted) => set({ accessibilityGranted }),
}));
