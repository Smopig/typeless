import { useAppStore } from "../store/appStore";
import type { AppStatus } from "../lib/types";

const STATUS_CONFIG: Record<
  AppStatus,
  { label: string; color: string; pulse: boolean }
> = {
  idle: { label: "Idle", color: "bg-gray-400", pulse: false },
  recording: { label: "Recording…", color: "bg-red-500", pulse: true },
  transcribing: { label: "Transcribing…", color: "bg-yellow-500", pulse: true },
  polishing: { label: "Polishing…", color: "bg-blue-500", pulse: true },
  error: { label: "Error", color: "bg-red-600", pulse: false },
};

export function StatusIndicator() {
  const status = useAppStore((s) => s.status);
  const { label, color, pulse } = STATUS_CONFIG[status];

  return (
    <div className="flex items-center gap-2">
      <span
        className={`h-2.5 w-2.5 rounded-full ${color} ${pulse ? "animate-pulse" : ""}`}
      />
      <span className="text-sm text-gray-600">{label}</span>
    </div>
  );
}
