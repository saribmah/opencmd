//! Command palette related types.

use serde::{Deserialize, Serialize};

/// A searchable command shown in the command palette.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Unique identifier (extension_id:command_id)
    pub id: String,
    /// Extension that provides this command
    pub extension_id: String,
    /// Command ID within the extension
    pub command_id: String,
    /// Display name
    pub name: String,
    /// Optional description
    #[serde(default)]
    pub description: Option<String>,
    /// Optional icon identifier
    #[serde(default)]
    pub icon: Option<String>,
    /// Search relevance score (higher = more relevant)
    #[serde(default)]
    pub score: f64,
}

/// Request to search for commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    /// The search query
    pub query: String,
    /// Maximum number of results to return
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    20
}

/// Request to execute a command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteRequest {
    /// Extension ID
    pub extension_id: String,
    /// Command ID within the extension
    pub command_id: String,
    /// Optional input/prompt
    #[serde(default)]
    pub input: Option<String>,
}

/// Result of command execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ExecuteResult {
    /// Command completed successfully
    Success {
        /// Optional output data
        #[serde(default)]
        output: Option<serde_json::Value>,
    },
    /// Command spawned a long-running process
    Process {
        /// Process/session ID for tracking
        session_id: String,
    },
    /// Command failed
    Error {
        /// Error message
        message: String,
    },
}
