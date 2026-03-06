import { invoke } from "@tauri-apps/api/core";
import type { Settings } from "$lib/bindings";

export type { Settings };

export async function getSettings(): Promise<Settings> {
  return await invoke<Settings>("get_settings");
}

export async function updateSettings(s: Settings): Promise<void> {
  await invoke("update_settings", { s });
}
