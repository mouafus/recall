use crate::models::ClipboardItem;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, sync::Mutex};
use tauri::{AppHandle, Emitter, Manager};

#[derive(Serialize, Deserialize)]
struct PersistedHistory {
    items: Vec<ClipboardItem>,
}

pub struct History {
    pub items: Mutex<Vec<ClipboardItem>>,
    app: Mutex<Option<AppHandle>>,
    max_item: Mutex<usize>,
}

impl History {
    pub fn new() -> Self {
        History {
            items: Mutex::new(vec![]),
            app: Mutex::new(None),
            max_item: Mutex::new(200),
        }
    }

    fn history_path(app: &AppHandle) -> PathBuf {
        let dir = app.path().app_data_dir().expect("app_data_dir");
        let _ = std::fs::create_dir_all(&dir);
        dir.join("history.json")
    }

    fn load_from_disk(&self, app: &AppHandle) {
        let path = Self::history_path(app);
        if let Ok(bytes) = fs::read(&path) {
            if let Ok(persisted) = serde_json::from_slice::<PersistedHistory>(&bytes) {
                let mut items = self.items.lock().unwrap();
                *items = persisted.items;
                let max_history_item = *self.max_item.lock().unwrap();
                if items.len() > max_history_item {
                    items.truncate(max_history_item);
                }
            }
        }
    }

    fn save_to_disk(&self) {
        if let Some(app) = self.app.lock().unwrap().as_ref() {
            let path = Self::history_path(app);
            let items = self.items.lock().unwrap();
            let data = PersistedHistory {
                items: items.clone(),
            };
            if let Ok(bytes) = serde_json::to_vec_pretty(&data) {
                let _ = fs::write(path, bytes);
            }
        }
    }

    pub fn set_app(&self, app: AppHandle) {
        self.load_from_disk(&app);
        *self.app.lock().unwrap() = Some(app);
    }

    pub fn set_max_item(&self, max: usize) {
        {
            let mut m = self.max_item.lock().unwrap();
            *m = max;
        }
        {
            let mut items = self.items.lock().unwrap();
            if items.len() > max {
                items.truncate(max);
            }
        }
        self.save_to_disk();
    }

    pub fn add(
        &self,
        content: String,
        content_type: String,
        image_base64: Option<String>,
        image_width: Option<u32>,
        image_height: Option<u32>,
    ) {
        let max_history_item = *self.max_item.lock().unwrap();
        let mut items = self.items.lock().unwrap();

        if let Some(last) = items.first() {
            if last.content == content {
                return;
            }
        }

        let item = ClipboardItem {
            id: uuid::Uuid::new_v4(),
            content,
            content_type,
            timestamp: chrono::Utc::now().timestamp_millis(),
            image_base64,
            image_width,
            image_height,
        };

        items.insert(0, item.clone());

        if items.len() > max_history_item {
            items.truncate(max_history_item);
        }
        drop(items);

        if let Some(app) = self.app.lock().unwrap().as_ref() {
            let _ = app.emit("new-clipboard-item", item);
        }
        self.save_to_disk();
    }

    pub fn get_all(&self) -> Vec<ClipboardItem> {
        let items = self.items.lock().unwrap();
        items.clone()
    }

    pub fn promote_item(&self, id: uuid::Uuid) -> Option<ClipboardItem> {
        let mut items = self.items.lock().unwrap();
        if let Some(item) = items.iter().position(|item| item.id == id) {
            let item = items.remove(item);
            items.insert(0, item.clone());
            drop(items);
            self.save_to_disk();
            return Some(item);
        }
        None
    }

    pub fn remove_by_content(&self, content: &str) {
        let mut items = self.items.lock().unwrap();
        if let Some(pos) = items.iter().position(|item| item.content == content) {
            items.remove(pos);
            drop(items);
            self.save_to_disk();
        }
    }
}
