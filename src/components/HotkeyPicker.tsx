import { useState, useCallback, useRef } from "react";
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

const isMac = navigator.platform.toUpperCase().includes("MAC");

const DISPLAY_MAP: Record<string, string> = {
  CommandOrControl: isMac ? "⌘" : "Ctrl",
  Alt: isMac ? "⌥" : "Alt",
  Shift: isMac ? "⇧" : "Shift+",
  Period: ".",
  Comma: ",",
  Slash: "/",
  Semicolon: ";",
  Quote: "'",
  BracketLeft: "[",
  BracketRight: "]",
  Backslash: "\\",
  Minus: "-",
  Equal: "=",
  Space: "Space",
};

export function displayHotkey(stored: string): string {
  if (!stored) return "";
  return stored
    .split("+")
    .map((tok) => DISPLAY_MAP[tok] ?? tok)
    .join(isMac ? "" : "");
}

function formatHotkey(e: KeyboardEvent): string | null {
  const mods: string[] = [];
  if (e.ctrlKey || e.metaKey) mods.push("CommandOrControl");
  if (e.altKey) mods.push("Alt");
  if (e.shiftKey) mods.push("Shift");

  const key = e.key;
  if (["Control", "Meta", "Alt", "Shift", "Escape"].includes(key)) return null;

  const mapped = KEY_MAP[key] ?? (key.length === 1 ? key.toUpperCase() : key);
  const isFKey = /^F\d{1,2}$/.test(mapped);
  if (mods.length === 0 && !isFKey) return null;
  return mods.length > 0 ? [...mods, mapped].join("+") : mapped;
}

export function HotkeyPicker({ value, onChange }: Props) {
  const [capturing, setCapturing] = useState(false);
  const btnRef = useRef<HTMLButtonElement>(null);

  const startCapture = useCallback(() => {
    setCapturing(true);
    btnRef.current?.focus();
  }, []);

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      e.preventDefault();
      e.stopPropagation();
      if (e.key === "Escape") {
        setCapturing(false);
        return;
      }
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
        ref={btnRef}
        type="button"
        onClick={startCapture}
        onKeyDown={capturing ? handleKeyDown : undefined}
        onBlur={() => setCapturing(false)}
        className={`flex items-center gap-2 rounded-lg border px-3 py-2 text-sm focus:outline-none ${
          capturing
            ? "border-blue-500 bg-blue-50 ring-2 ring-blue-400"
            : "border-gray-300 bg-white hover:border-blue-400 cursor-pointer"
        }`}
      >
        <Keyboard size={14} className={capturing ? "text-blue-500" : "text-gray-400"} />
        <span className={capturing ? "text-blue-600 font-medium" : "text-gray-700"}>
          {capturing
            ? "⌨ Press your hotkey now… (Esc to cancel)"
            : value
            ? displayHotkey(value)
            : "Click here to set hotkey"}
        </span>
      </button>
      <p className="text-xs text-gray-400">
        Examples: {isMac ? "⌘⇧." : "Ctrl+Shift+."} · F5 · F6 · {isMac ? "⌥F" : "Alt+F"}
        &nbsp;— must include at least one non-modifier key
      </p>
    </div>
  );
}
