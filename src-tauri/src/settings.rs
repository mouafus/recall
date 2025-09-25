use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub max_items: usize,
    pub shortcut: String,
    pub autostart: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            max_items: 5000,
            shortcut: "CmdOrCtrl+Shift+V".into(),
            autostart: true,
        }
    }
}

fn settings_path(app: &tauri::AppHandle) -> PathBuf {
    let dir = app.path().app_data_dir().expect("app_data_dir");
    let _ = std::fs::create_dir_all(&dir);
    dir.join("settings.json")
}

pub fn load_settings(app: &tauri::AppHandle) -> Settings {
    let path = settings_path(app);
    if let Ok(bytes) = fs::read(&path) {
        if let Ok(s) = serde_json::from_slice::<Settings>(&bytes) {
            return s;
        }
    }
    Settings::default()
}

pub fn save_settings(app: &tauri::AppHandle, s: &Settings) -> std::io::Result<()> {
    let path = settings_path(app);
    let data = serde_json::to_vec_pretty(s)?;
    fs::write(path, data)
}
