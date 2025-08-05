mod clipboard;
mod commands;
mod history;
mod models;

use tauri::Manager;
use tauri_plugin_global_shortcut::GlobalShortcutExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let app_handle = app.handle().clone();

            crate::commands::HISTORY.set_app(app_handle.clone());

            let _clipboard_watcher = clipboard::start_clipboard_watcher();

            let shortcut = "CmdOrCtrl+Shift+U";

            let _ = app
                .global_shortcut()
                .on_shortcut(shortcut, move |_, _, _| {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                })
                .expect("Failed to set up global shortcut handler");


            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_history,
            commands::paste_item,
            commands::hide_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
