//! Error types for the extension-host crate.

use thiserror::Error;

/// Extension host errors.
#[derive(Error, Debug)]
pub enum ExtensionError {
    #[error("Extension not found: {0}")]
    NotFound(String),

    #[error("Invalid manifest: {0}")]
    InvalidManifest(String),

    #[error("Command not found: {extension_id}:{command_id}")]
    CommandNotFound {
        extension_id: String,
        command_id: String,
    },

    #[error("Extension runtime error: {0}")]
    Runtime(String),

    #[error("PTY error: {0}")]
    Pty(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

/// Result type for extension operations.
pub type Result<T> = std::result::Result<T, ExtensionError>;
