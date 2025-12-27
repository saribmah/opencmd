//! OpenCMD Tauri application.
//!
//! This is a thin binding layer that exposes the core crate functionality
//! to the frontend via Tauri commands.

mod commands;
mod state;

use state::AppState;
use tauri::Manager;
use tracing::info;
use tracing_subscriber::EnvFilter;

/// Initialize logging.
fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_logging();
    info!("Starting OpenCMD...");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::search,
            commands::execute,
            commands::get_extensions,
            commands::initialize,
            commands::show_window,
            commands::hide_window,
            commands::toggle_window,
        ])
        .setup(|app| {
            // Try to register global hotkey (Cmd+Shift+O)
            // This may fail if the app doesn't have accessibility permissions
            use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
            use tracing::warn;

            let shortcut: Shortcut = "Alt+Shift+O".parse().unwrap();

            let shortcut_result = app.global_shortcut().on_shortcut(shortcut, |app_handle, _shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                        } else {
                            // Center horizontally
                            if let Some(monitor) = window.current_monitor().ok().flatten() {
                                let monitor_size = monitor.size();
                                if let Ok(window_size) = window.outer_size() {
                                    let x = (monitor_size.width as i32 - window_size.width as i32) / 2;
                                    let _ = window.set_position(tauri::PhysicalPosition::new(x, 140));
                                }
                            }
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
            });

            match shortcut_result.and_then(|_| app.global_shortcut().register(shortcut)) {
                Ok(_) => {
                    info!("Global shortcut registered: Cmd+Shift+O");
                }
                Err(e) => {
                    warn!("Failed to register global shortcut: {}. Grant accessibility permissions in System Settings.", e);
                    eprintln!("Problem opening the file: {:?}", e);
                    // Show window anyway so user can use the app
                    if let Some(window) = app.get_webview_window("main") {
                        // Center horizontally
                        if let Some(monitor) = window.current_monitor().ok().flatten() {
                            let monitor_size = monitor.size();
                            if let Ok(window_size) = window.outer_size() {
                                let x = (monitor_size.width as i32 - window_size.width as i32) / 2;
                                let _ = window.set_position(tauri::PhysicalPosition::new(x, 140));
                            }
                        }
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
