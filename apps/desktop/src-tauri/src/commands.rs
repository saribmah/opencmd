//! Tauri commands that expose core functionality to the frontend.

use crate::state::AppState;
use opencmd_protocol::{ExecuteRequest, ExecuteResult, Extension, SearchRequest, SearchResult};
use tauri::State;
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
    let core = core_guard
        .as_ref()
        .ok_or_else(|| "App not initialized".to_string())?;

    let host = core.extension_host.read().await;
    
    // Note: We'd need to add a method to ExtensionHost to get all extensions
    // For now, return empty vec
    Ok(vec![])
}
