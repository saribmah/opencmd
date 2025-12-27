//! OpenCMD Tauri application.
//!
//! This is a thin binding layer that exposes the core crate functionality
//! to the frontend via Tauri commands.

mod commands;
mod state;

use state::AppState;
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
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::search,
            commands::execute,
            commands::get_extensions,
            commands::initialize,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
