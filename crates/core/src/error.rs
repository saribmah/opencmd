//! Error types for the core crate.

use thiserror::Error;

/// Core application errors.
#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Extension error: {0}")]
    Extension(#[from] opencmd_extension_host::ExtensionError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for core operations.
pub type Result<T> = std::result::Result<T, CoreError>;
