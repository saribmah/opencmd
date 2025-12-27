//! Application state management.

use opencmd_core::AppCore;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Application state shared across Tauri commands.
pub struct AppState {
    /// The core application instance (lazily initialized).
    pub core: Arc<RwLock<Option<AppCore>>>,
    /// Tokio runtime for async operations.
    pub runtime: tokio::runtime::Runtime,
}

impl AppState {
    /// Create new application state.
    pub fn new() -> Self {
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

        Self {
            core: Arc::new(RwLock::new(None)),
            runtime,
        }
    }

    /// Get or initialize the core.
    pub async fn get_or_init_core(&self) -> Result<(), String> {
        let mut core_guard = self.core.write().await;
        if core_guard.is_none() {
            let core = AppCore::new().await.map_err(|e| e.to_string())?;
            core.initialize().await.map_err(|e| e.to_string())?;
            *core_guard = Some(core);
        }
        Ok(())
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
