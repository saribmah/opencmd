//! Tauri commands that expose core functionality to the frontend.

use crate::state::AppState;
use opencmd_protocol::{ExecuteRequest, ExecuteResult, Extension, SearchRequest, SearchResult};
use tauri::{AppHandle, Manager, State, WebviewWindow};
use tracing::info;

/// Initialize the application core.
#[tauri::command]
pub async fn initialize(state: State<'_, AppState>) -> Result<(), String> {
    info!("Initializing application...");
    state.get_or_init_core().await
}

/// Search for commands matching a query.
#[tauri::command]
pub async fn search(
    query: String,
    limit: Option<usize>,
    state: State<'_, AppState>,
) -> Result<Vec<SearchResult>, String> {
    let core_guard = state.core.read().await;
    let core = core_guard
        .as_ref()
        .ok_or_else(|| "App not initialized".to_string())?;

    let request = SearchRequest {
        query,
        limit: limit.unwrap_or(20),
    };

    core.search(request).await.map_err(|e| e.to_string())
}

/// Execute a command.
#[tauri::command]
pub async fn execute(
    extension_id: String,
    command_id: String,
    input: Option<String>,
    state: State<'_, AppState>,
) -> Result<ExecuteResult, String> {
    let core_guard = state.core.read().await;
    let core = core_guard
        .as_ref()
        .ok_or_else(|| "App not initialized".to_string())?;

    let request = ExecuteRequest {
        extension_id,
        command_id,
        input,
    };

    core.execute(request).await.map_err(|e| e.to_string())
}

/// Get all loaded extensions.
#[tauri::command]
pub async fn get_extensions(state: State<'_, AppState>) -> Result<Vec<Extension>, String> {
    let core_guard = state.core.read().await;
    let _core = core_guard
        .as_ref()
        .ok_or_else(|| "App not initialized".to_string())?;

    // TODO: Add method to ExtensionHost to get all extensions
    Ok(vec![])
}

/// Show the main window centered on screen.
#[tauri::command]
pub async fn show_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        center_window_horizontally(&window)?;
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Hide the main window.
#[tauri::command]
pub async fn hide_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Toggle the main window visibility.
#[tauri::command]
pub async fn toggle_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            window.hide().map_err(|e| e.to_string())?;
        } else {
            center_window_horizontally(&window)?;
            window.show().map_err(|e| e.to_string())?;
            window.set_focus().map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

/// Center window horizontally on the current monitor.
fn center_window_horizontally(window: &WebviewWindow) -> Result<(), String> {
    if let Some(monitor) = window.current_monitor().map_err(|e| e.to_string())? {
        let monitor_size = monitor.size();
        let window_size = window.outer_size().map_err(|e| e.to_string())?;
        
        let x = (monitor_size.width as i32 - window_size.width as i32) / 2;
        let y = 140; // Fixed distance from top, like Raycast
        
        window
            .set_position(tauri::PhysicalPosition::new(x, y))
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
