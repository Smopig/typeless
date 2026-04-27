import { useState, useCallback } from "react";
import { Keyboard } from "lucide-react";

interface Props {
  value: string;
  onChange: (hotkey: string) => void;
}

const KEY_MAP: Record<string, string> = {
  " ": "Space",
  ".": "Period",
  ",": "Comma",
  "/": "Slash",
  ";": "Semicolon",
  "'": "Quote",
  "[": "BracketLeft",
  "]": "BracketRight",
  "\\": "Backslash",
  "-": "Minus",
  "=": "Equal",
};

function formatHotkey(e: KeyboardEvent): string | null {
  const mods: string[] = [];
  if (e.ctrlKey || e.metaKey) mods.push("CommandOrControl");
  if (e.altKey) mods.push("Alt");
  if (e.shiftKey) mods.push("Shift");

  const key = e.key;
  if (["Control", "Meta", "Alt", "Shift"].includes(key)) return null;

  const mapped = KEY_MAP[key] ?? (key.length === 1 ? key.toUpperCase() : key);
  if (mods.length === 0) return null;
  return [...mods, mapped].join("+");
}

export function HotkeyPicker({ value, onChange }: Props) {
  const [capturing, setCapturing] = useState(false);

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      e.preventDefault();
      e.stopPropagation();
      const hotkey = formatHotkey(e.nativeEvent);
      if (hotkey) {
        onChange(hotkey);
        setCapturing(false);
      }
    },
    [onChange]
  );

  return (
    <div className="flex flex-col gap-1">
      <label className="text-sm font-medium text-gray-700">Hotkey</label>
      <button
        type="button"
        onKeyDown={capturing ? handleKeyDown : undefined}
        onFocus={() => setCapturing(true)}
        onBlur={() => setCapturing(false)}
        className={`flex items-center gap-2 rounded-lg border px-3 py-2 text-sm focus:outline-none ${
          capturing
            ? "border-blue-500 bg-blue-50 ring-1 ring-blue-500"
            : "border-gray-300 bg-white hover:border-gray-400"
        }`}
      >
        <Keyboard size={14} className="text-gray-400" />
        <span className={capturing ? "text-blue-600" : "text-gray-700"}>
          {capturing ? "Press hotkey combination…" : (value || "Click to set")}
        </span>
      </button>
      <p className="text-xs text-gray-400">
        Requires at least one modifier key (Cmd/Ctrl, Alt, Shift)
      </p>
    </div>
  );
}
