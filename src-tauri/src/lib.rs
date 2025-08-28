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

            #[cfg(any(target_os = "linux"))]
            {
                use tauri::menu::{Menu, MenuItem};
                use tauri::tray::TrayIconBuilder;

                let show_history = MenuItem::with_id(app, "show_history", "Historique", true, None::<&str>)
                    .expect("failed to create 'Historique' menu item");
                let quit_item = MenuItem::with_id(app, "quit", "Quitter", true, None::<&str>)
                    .expect("failed to create 'Quitter' menu item");
                let menu = Menu::with_items(app, &[&show_history, &quit_item])
                    .expect("failed to build tray menu");

                
                let mut tray_builder = TrayIconBuilder::new()
                    .tooltip("Recall")
                    .menu(&menu)
                    .on_menu_event(|app, event| match event.id().as_ref() {
                        "show_history" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    });

                if let Some(icon) = app.default_window_icon() {
                    tray_builder = tray_builder.icon(icon.clone());
                }

                let _tray = tray_builder
                    .build(app)
                    .expect("failed to build tray icon");
            }

            let shortcut = "CmdOrCtrl+Shift+V";

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
