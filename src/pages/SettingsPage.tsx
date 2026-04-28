import { useEffect, useState } from "react";
import { useForm, Controller } from "react-hook-form";
import { Save } from "lucide-react";

import { useAppStore } from "../store/appStore";
import { getSettings, saveSettings } from "../lib/commands";
import type { AppSettings } from "../lib/types";
import { ApiKeyField } from "../components/ApiKeyField";
import { HotkeyPicker } from "../components/HotkeyPicker";
import { AccessibilityBanner } from "../components/AccessibilityBanner";
import { StatusIndicator } from "../components/StatusIndicator";

const LANGUAGE_OPTIONS = [
  { value: "zh", label: "Chinese / 中文 (recommended for mixed)" },
  { value: "en", label: "English" },
  { value: "auto", label: "Auto-detect" },
];

export function SettingsPage() {
  const { accessibilityGranted, setSettings: storeSetSettings } = useAppStore();
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);

  const { register, handleSubmit, control, reset, watch } =
    useForm<AppSettings>();

  const polishEnabled = watch("polishEnabled");

  useEffect(() => {
    getSettings().then((s) => {
      reset(s);
      storeSetSettings(s);
    });
  }, [reset, storeSetSettings]);

  const onSubmit = async (data: AppSettings) => {
    setSaving(true);
    try {
      await saveSettings(data);
      storeSetSettings(data);
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } catch (e) {
      console.error(e);
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="flex h-full flex-col">
      {/* Header */}
      <div className="flex items-center justify-between border-b border-gray-200 px-5 py-4">
        <div>
          <h1 className="text-base font-semibold text-gray-900">Typeless</h1>
          <p className="text-xs text-gray-500">AI voice input for any app</p>
        </div>
        <StatusIndicator />
      </div>

      {/* Body */}
      <div className="flex-1 overflow-y-auto px-5 py-4">
        <form id="settings-form" onSubmit={handleSubmit(onSubmit)} className="flex flex-col gap-5">

          {!accessibilityGranted && <AccessibilityBanner />}

          {/* Hotkey */}
          <section className="flex flex-col gap-3">
            <h2 className="text-xs font-semibold uppercase tracking-wider text-gray-400">
              Voice Input
            </h2>
            <Controller
              name="hotkey"
              control={control}
              render={({ field }) => (
                <HotkeyPicker value={field.value} onChange={field.onChange} />
              )}
            />
            <div className="flex flex-col gap-1">
              <label className="text-sm font-medium text-gray-700">Language</label>
              <select
                {...register("languageHint")}
                className="rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:outline-none"
              >
                {LANGUAGE_OPTIONS.map((o) => (
                  <option key={o.value} value={o.value}>
                    {o.label}
                  </option>
                ))}
              </select>
            </div>
          </section>

          {/* Groq API */}
          <section className="flex flex-col gap-3">
            <h2 className="text-xs font-semibold uppercase tracking-wider text-gray-400">
              Groq API (Speech-to-Text)
            </h2>
            <Controller
              name="groqApiKey"
              control={control}
              render={({ field }) => (
                <ApiKeyField
                  label="Groq API Key"
                  value={field.value ?? ""}
                  onChange={field.onChange}
                  placeholder="gsk_…"
                  required
                />
              )}
            />
            <p className="text-xs text-gray-400">
              Free tier at{" "}
              <span className="text-blue-600">console.groq.com</span>.
              Uses whisper-large-v3-turbo (~200ms/request).
            </p>
          </section>

          {/* LLM Polish */}
          <section className="flex flex-col gap-3">
            <div className="flex items-center justify-between">
              <h2 className="text-xs font-semibold uppercase tracking-wider text-gray-400">
                AI Text Polish (Optional)
              </h2>
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="checkbox"
                  {...register("polishEnabled")}
                  className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                />
                <span className="text-sm text-gray-600">Enable</span>
              </label>
            </div>

            {polishEnabled && (
              <>
                <Controller
                  name="llmApiKey"
                  control={control}
                  render={({ field }) => (
                    <ApiKeyField
                      label="LLM API Key"
                      value={field.value ?? ""}
                      onChange={field.onChange}
                      placeholder="sk-… or any OpenAI-compatible key"
                    />
                  )}
                />
                <div className="flex flex-col gap-1">
                  <label className="text-sm font-medium text-gray-700">
                    Base URL
                  </label>
                  <input
                    {...register("llmBaseUrl")}
                    placeholder="https://api.openai.com/v1"
                    className="rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:outline-none"
                  />
                </div>
                <div className="flex flex-col gap-1">
                  <label className="text-sm font-medium text-gray-700">
                    Model
                  </label>
                  <input
                    {...register("llmModel")}
                    placeholder="gpt-4o-mini"
                    className="rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:outline-none"
                  />
                </div>
              </>
            )}
          </section>

          {/* System */}
          <section className="flex flex-col gap-3">
            <h2 className="text-xs font-semibold uppercase tracking-wider text-gray-400">
              System
            </h2>
            <label className="flex items-center gap-3 cursor-pointer">
              <input
                type="checkbox"
                {...register("autostart")}
                className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
              />
              <span className="text-sm text-gray-700">Launch at login</span>
            </label>
          </section>

        </form>
      </div>

      {/* Sticky footer — always visible */}
      <div className="border-t border-gray-200 bg-white px-5 py-3">
        <button
          type="submit"
          form="settings-form"
          disabled={saving}
          className="flex w-full items-center justify-center gap-2 rounded-lg bg-blue-600 px-4 py-2.5 text-sm font-medium text-white hover:bg-blue-700 disabled:opacity-60"
        >
          <Save size={15} />
          {saved ? "Saved!" : saving ? "Saving…" : "Save Settings"}
        </button>
      </div>
    </div>
  );
}
