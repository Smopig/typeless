import { useEffect, useState } from "react";
import { Settings, Clock } from "lucide-react";

import { useAppStore } from "./store/appStore";
import { onStatusChanged, onTranscriptReady, onErrorOccurred } from "./lib/events";
import { checkAccessibilityPermission } from "./lib/commands";
import { SettingsPage } from "./pages/SettingsPage";
import { HistoryPage } from "./pages/HistoryPage";

type Tab = "settings" | "history";

export default function App() {
  const [tab, setTab] = useState<Tab>("settings");
  const { setStatus, addHistory, setError, setAccessibility } = useAppStore();

  // Subscribe to backend events
  useEffect(() => {
    const unsubs = [
      onStatusChanged((p) => setStatus(p.status)),
      onTranscriptReady((p) => addHistory(p)),
      onErrorOccurred((p) => {
        setError(p.message);
        setStatus("error");
        setTimeout(() => setError(null), 5000);
      }),
    ];

    return () => {
      unsubs.forEach((p) => p.then((unsub) => unsub()));
    };
  }, [setStatus, addHistory, setError]);

  // Check accessibility permission on mount and whenever window regains focus
  useEffect(() => {
    const check = () =>
      checkAccessibilityPermission()
        .then(setAccessibility)
        .catch(() => setAccessibility(true));
    check();
    window.addEventListener("focus", check);
    return () => window.removeEventListener("focus", check);
  }, [setAccessibility]);

  const errorMessage = useAppStore((s) => s.errorMessage);

  return (
    <div className="flex h-screen flex-col bg-gray-50">
      {/* Error toast */}
      {errorMessage && (
        <div className="absolute left-4 right-4 top-4 z-50 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-800 shadow-lg">
          {errorMessage}
        </div>
      )}

      {/* Tab nav */}
      <nav className="flex border-b border-gray-200 bg-white">
        {(["settings", "history"] as Tab[]).map((t) => (
          <button
            key={t}
            onClick={() => setTab(t)}
            className={`flex flex-1 items-center justify-center gap-1.5 py-3 text-sm font-medium transition-colors ${
              tab === t
                ? "border-b-2 border-blue-600 text-blue-600"
                : "text-gray-500 hover:text-gray-700"
            }`}
          >
            {t === "settings" ? <Settings size={15} /> : <Clock size={15} />}
            {t === "settings" ? "Settings" : "History"}
          </button>
        ))}
      </nav>

      {/* Page content */}
      <div className="flex-1 overflow-hidden">
        {tab === "settings" ? <SettingsPage /> : <HistoryPage />}
      </div>
    </div>
  );
}
