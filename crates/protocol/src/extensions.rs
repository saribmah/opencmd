//! Extension-related types.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Type of extension determining its runtime behavior.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionType {
    /// Spawns and manages CLI processes with PTY support
    Runner,
    /// Runs in sandboxed WASM/Deno runtime with network access
    Integration,
    /// Simple scripts that run and return results
    Command,
}

/// Runtime environment for integration extensions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionRuntime {
    Wasm,
    Deno,
}

/// Configuration for runner-type extensions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunnerConfig {
    /// Command to execute
    pub command: String,
    /// Arguments to pass (supports {{prompt}} template)
    pub args: Vec<String>,
    /// Whether the runner is interactive
    #[serde(default)]
    pub interactive: bool,
    /// Whether to use PTY
    #[serde(default)]
    pub pty: bool,
}

/// A command exposed by an extension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionCommand {
    /// Unique identifier for this command within the extension
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Optional description
    #[serde(default)]
    pub description: Option<String>,
    /// Keywords for search/filtering
    #[serde(default)]
    pub keywords: Vec<String>,
}

/// Network permission for integration extensions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPermission {
    /// Allowed hosts
    pub network: Vec<String>,
}

/// Permission types for extensions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Permission {
    /// Simple permission string (e.g., "process:spawn", "pty")
    Simple(String),
    /// Network permission with allowed hosts
    Network(NetworkPermission),
}

/// Extension manifest loaded from manifest.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    /// Unique identifier for the extension
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Version string (semver)
    pub version: String,
    /// Type of extension
    #[serde(rename = "type")]
    pub extension_type: ExtensionType,
    /// Optional description
    #[serde(default)]
    pub description: Option<String>,
    /// Runner configuration (for runner type)
    #[serde(default)]
    pub runner: Option<RunnerConfig>,
    /// Runtime environment (for integration type)
    #[serde(default)]
    pub runtime: Option<ExtensionRuntime>,
    /// Main entry point (for integration type)
    #[serde(default)]
    pub main: Option<String>,
    /// Script to run (for command type)
    #[serde(default)]
    pub script: Option<String>,
    /// Commands exposed by this extension
    #[serde(default)]
    pub commands: Vec<ExtensionCommand>,
    /// Required permissions
    #[serde(default)]
    pub permissions: Vec<Permission>,
}

/// Represents a loaded extension at runtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
    /// Unique runtime ID
    pub id: Uuid,
    /// The extension manifest
    pub manifest: ExtensionManifest,
    /// Path to the extension directory
    pub path: String,
    /// Whether the extension is currently enabled
    pub enabled: bool,
}

/// Status of an extension.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionStatus {
    /// Extension is loaded and ready
    Ready,
    /// Extension is currently running
    Running,
    /// Extension encountered an error
    Error,
    /// Extension is disabled
    Disabled,
}
