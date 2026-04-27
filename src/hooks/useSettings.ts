import { useEffect } from "react";
import { getSettings } from "../lib/commands";
import { useAppStore } from "../store/appStore";

export function useSettingsLoader() {
  const setSettings = useAppStore((s) => s.setSettings);

  useEffect(() => {
    getSettings()
      .then(setSettings)
      .catch((e) => console.error("Failed to load settings:", e));
  }, [setSettings]);
}
