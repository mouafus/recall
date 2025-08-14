use crate::history::History;
use crate::models;
use uuid::Uuid;
use base64::Engine;

lazy_static::lazy_static! {
    pub static ref HISTORY: History = History::new();
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_history() -> Vec<models::ClipboardItem> {
    HISTORY.get_all()
}

#[tauri::command(rename_all = "snake_case")]
pub fn paste_item(id: Uuid, window: tauri::Window) -> Option<()> {
    if let Some(item) = HISTORY.promote_item(id) {
        let mut clipboard = arboard::Clipboard::new().ok()?;
        if item.content_type.starts_with("image") {
            if let (Some(b64), Some(w), Some(h)) = (&item.image_base64, item.image_width, item.image_height) {
                let bytes = base64::engine::general_purpose::STANDARD.decode(b64).ok()?;
                let img = arboard::ImageData {
                    width: w as usize,
                    height: h as usize,
                    bytes: bytes.into(),
                };
                clipboard.set_image(img).ok()?;
            } else {
                clipboard.set_text(item.content.clone()).ok()?;
            }
        } else {
            clipboard.set_text(item.content.clone()).ok()?;
        }
        window.hide().ok()?;
    }
    None
}

#[tauri::command]
pub fn hide_window(window: tauri::Window) -> Option<()> {
    window.hide().ok()?;
    None
}