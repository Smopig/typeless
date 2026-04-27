import { ShieldAlert } from "lucide-react";
import { requestAccessibilityPermission } from "../lib/commands";

export function AccessibilityBanner() {
  return (
    <div className="flex items-start gap-3 rounded-lg border border-amber-200 bg-amber-50 p-3">
      <ShieldAlert size={18} className="mt-0.5 flex-shrink-0 text-amber-600" />
      <div className="flex-1 min-w-0">
        <p className="text-sm font-medium text-amber-800">
          Accessibility permission required
        </p>
        <p className="mt-0.5 text-xs text-amber-700">
          Typeless needs Accessibility access to inject text into other apps.
          After granting, restart the app.
        </p>
        <button
          onClick={() => requestAccessibilityPermission()}
          className="mt-2 rounded-md bg-amber-600 px-3 py-1 text-xs font-medium text-white hover:bg-amber-700"
        >
          Open Accessibility Settings
        </button>
      </div>
    </div>
  );
}
