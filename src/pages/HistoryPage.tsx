import { useEffect } from "react";
import { Copy, Trash2 } from "lucide-react";
import { useAppStore } from "../store/appStore";
import { getHistory, clearHistory } from "../lib/commands";
import type { HistoryEntry } from "../lib/types";

function HistoryCard({ entry }: { entry: HistoryEntry }) {
  const text = entry.polished ?? entry.raw;
  const date = new Date(entry.timestamp).toLocaleTimeString();

  const copy = () => {
    navigator.clipboard.writeText(text).catch(() => {});
  };

  return (
    <div className="flex flex-col gap-1.5 rounded-lg border border-gray-200 bg-white p-3">
      <div className="flex items-center justify-between gap-2">
        <span className="text-xs text-gray-400">{date}</span>
        {entry.polished && (
          <span className="rounded bg-blue-100 px-1.5 py-0.5 text-[10px] font-medium text-blue-700">
            polished
          </span>
        )}
        <button onClick={copy} className="ml-auto text-gray-400 hover:text-gray-600">
          <Copy size={13} />
        </button>
      </div>
      <p className="text-sm text-gray-800 leading-relaxed">{text}</p>
      {entry.polished && (
        <p className="text-xs text-gray-400 italic leading-relaxed">
          Raw: {entry.raw}
        </p>
      )}
    </div>
  );
}

export function HistoryPage() {
  const { history } = useAppStore();

  useEffect(() => {
    getHistory().then((entries) => {
      useAppStore.setState({ history: entries });
    });
  }, []);

  const handleClear = async () => {
    await clearHistory();
    useAppStore.setState({ history: [] });
  };

  return (
    <div className="flex h-full flex-col">
      <div className="flex items-center justify-between border-b border-gray-200 px-5 py-3">
        <span className="text-sm font-medium text-gray-700">
          {history.length} entries
        </span>
        {history.length > 0 && (
          <button
            onClick={handleClear}
            className="flex items-center gap-1 text-xs text-red-500 hover:text-red-700"
          >
            <Trash2 size={13} />
            Clear all
          </button>
        )}
      </div>
      <div className="flex-1 overflow-y-auto px-4 py-3">
        {history.length === 0 ? (
          <div className="flex h-full items-center justify-center">
            <p className="text-sm text-gray-400">
              No recordings yet. Hold the hotkey and speak.
            </p>
          </div>
        ) : (
          <div className="flex flex-col gap-2">
            {history.map((entry) => (
              <HistoryCard key={entry.timestamp} entry={entry} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
