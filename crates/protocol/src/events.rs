//! Event types for the pub/sub event bus.

use serde::{Deserialize, Serialize};

/// Events emitted by the application.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AppEvent {
    /// Extension was loaded
    ExtensionLoaded { extension_id: String },
    /// Extension was unloaded
    ExtensionUnloaded { extension_id: String },
    /// Extension status changed
    ExtensionStatusChanged {
        extension_id: String,
        status: super::ExtensionStatus,
    },
    /// Process output from a runner
    ProcessOutput {
        session_id: String,
        data: String,
    },
    /// Process exited
    ProcessExited {
        session_id: String,
        exit_code: Option<i32>,
    },
    /// Configuration changed
    ConfigChanged { key: String },
    /// Window visibility changed
    WindowVisibilityChanged { visible: bool },
}

/// Event subscription request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSubscription {
    /// Event types to subscribe to (empty = all)
    pub event_types: Vec<String>,
}
