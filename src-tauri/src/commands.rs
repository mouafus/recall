use crate::history::History;
use crate::models;
use crate::settings;
use base64::Engine;
use tauri::Manager;
use tauri_plugin_autostart::ManagerExt as _;
use tauri_plugin_global_shortcut::GlobalShortcutExt as _;
use uuid::Uuid;

lazy_static::lazy_static! {
    pub static ref HISTORY: History = History::new();
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_history() -> Vec<models::ClipboardItem> {
    HISTORY.get_all()
}

#[tauri::command(rename_all = "snake_case")]
pub fn paste_item(id: Uuid) -> Result<(), String> {
    let item = HISTORY.promote_item(id).ok_or("Item not found")?;
    let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    if item.content_type.starts_with("image") {
        if let (Some(b64), Some(w), Some(h)) =
            (&item.image_base64, item.image_width, item.image_height)
        {
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(b64)
                .map_err(|e| e.to_string())?;
            let img = arboard::ImageData {
                width: w as usize,
                height: h as usize,
                bytes: bytes.into(),
            };
            clipboard.set_image(img).map_err(|e| e.to_string())?;
        } else {
            clipboard.set_text(item.content.clone()).map_err(|e| e.to_string())?;
        }
    } else {
        clipboard.set_text(item.content.clone()).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn hide_window(window: tauri::Window) -> Option<()> {
    window.hide().ok()?;
    None
}

#[tauri::command]
pub fn get_settings(app: tauri::AppHandle) -> settings::Settings {
    settings::load_settings(&app)
}

#[tauri::command]
pub fn update_settings(app: tauri::AppHandle, s: settings::Settings) -> Result<(), String> {
    let old = settings::load_settings(&app);

    settings::save_settings(&app, &s).map_err(|e| e.to_string())?;

    let mgr = app.autolaunch();
    if s.autostart { mgr.enable().map_err(|e| e.to_string())?; } else { mgr.disable().map_err(|e| e.to_string())?; }

    HISTORY.set_max_item(s.max_items);

    let gs = app.global_shortcut();

    let _ = gs.unregister(old.shortcut.as_str());
    gs.on_shortcut(s.shortcut.as_str(), move |app, _, _| {
        if let Some(win) = app.get_webview_window("main") {
            let _ = win.show();
            let _ = win.set_focus();
        }
    }).map_err(|e| format!("register failed: {e}"))?;

    Ok(())
}
