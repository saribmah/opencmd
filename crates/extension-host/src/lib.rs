//! Extension host for loading, managing, and running extensions.
//!
//! This crate provides:
//! - Extension discovery and loading from manifest.json files
//! - Runtime management for different extension types (runners, integrations, commands)
//! - PTY support for interactive CLI tools
//! - Sandboxing for WASM/Deno runtimes (future)

mod error;
mod host;
mod registry;
mod runner;

pub use error::{ExtensionError, Result};
pub use host::{ConfigProvider, EventPublisher, ExtensionHost};
pub use registry::ExtensionRegistry;
pub use runner::RunnerSession;
