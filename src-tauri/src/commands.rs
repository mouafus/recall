use crate::history::History;
use crate::models;
use uuid::Uuid;

lazy_static::lazy_static! {
    pub static ref HISTORY: History = History::new();
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_history() -> Vec<models::ClipboardItem> {
    HISTORY.get_all()
}

#[tauri::command(rename_all = "snake_case")]
pub fn paste_item(id: Uuid, window: tauri::Window) -> Option<()> {
    if let Some(item) = HISTORY.get_item(id) {
        let mut clipboard = arboard::Clipboard::new().ok()?;
        clipboard.set_text(item.content.clone()).ok()?;
        window.hide().ok()?;
    }
    None
}
