import { invoke } from "@tauri-apps/api/core";

export interface Settings {
  max_items: number;
  shortcut: string;
  autostart: boolean;
}

export async function getSettings(): Promise<Settings> {
  return await invoke<Settings>("get_settings");
}

export async function updateSettings(s: Settings): Promise<void> {
  await invoke("update_settings", { s });
}
