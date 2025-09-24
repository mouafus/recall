use crate::models::ClipboardItem;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};

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

    pub fn set_app(&self, app: AppHandle) {
        let mut app_handle = self.app.lock().unwrap();
        *app_handle = Some(app);
    }

    pub fn set_max_item(&self, max: usize) {
        let mut m = self.max_item.lock().unwrap();
        *m = max;
    }

    pub fn add(
        &self,
        content: String,
        content_type: String,
        image_base64: Option<String>,
        image_width: Option<u32>,
        image_height: Option<u32>,
    ) {
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

        let max_history_item = *self.max_item.lock().unwrap();
        if items.len() > max_history_item {
            items.truncate(max_history_item);
        }

        if let Some(app) = self.app.lock().unwrap().as_ref() {
            let _ = app.emit("new-clipboard-item", item);
        }
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
            return Some(item);
        }
        None
    }

    pub fn remove_by_content(&self, content: &str) {
        let mut items = self.items.lock().unwrap();
        if let Some(pos) = items.iter().position(|item| item.content == content) {
            items.remove(pos);
        }
    }
}
