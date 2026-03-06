mod clipboard;
mod commands;
mod history;
mod models;
mod settings;

use tauri::Manager;
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_global_shortcut::GlobalShortcutExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let app_handle = app.handle().clone();

            let setting = settings::load_settings(&app_handle);

            let autostart_manager = app.autolaunch();
            if setting.autostart {
                let _ = autostart_manager.enable();
            } else {
                let _ = autostart_manager.disable();
            }

            crate::commands::HISTORY.set_app(app_handle.clone());
            crate::commands::HISTORY.set_max_item(setting.max_items);

            let _clipboard_watcher = clipboard::start_clipboard_watcher();

            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            {
                use tauri::menu::{Menu, MenuItem};
                use tauri::tray::TrayIconBuilder;

                let show_history =
                    MenuItem::with_id(app, "show_history", "Historique", true, None::<&str>)
                        .expect("failed to create 'Historique' menu item");
                let open_settings =
                    MenuItem::with_id(app, "open_settings", "Paramètres", true, None::<&str>)
                        .expect("failed to create 'Paramètres' menu item");
                let quit_item = MenuItem::with_id(app, "quit", "Quitter", true, None::<&str>)
                    .expect("failed to create 'Quitter' menu item");
                let menu = Menu::with_items(app, &[&show_history, &open_settings, &quit_item])
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
                        "open_settings" => {
                            if let Some(w) = app.get_webview_window("settings") {
                                let _ = w.show();
                                let _ = w.set_focus();
                            } else {
                                let _ = tauri::WebviewWindowBuilder::new(
                                    app,
                                    "settings",
                                    tauri::WebviewUrl::App("/settings".into()),
                                )
                                .title("Paramètres Recall")
                                .build();
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

                let _tray = tray_builder.build(app).expect("failed to build tray icon");
            }

            // Register global shortcut from settings
            let shortcut = setting.shortcut.clone();
            let _ = app
                .global_shortcut()
                .on_shortcut(shortcut.as_str(), move |_, _, _| {
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
            commands::hide_window,
            commands::get_settings,
            commands::update_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
