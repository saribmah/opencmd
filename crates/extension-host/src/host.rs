//! Extension host for managing the extension lifecycle.

use crate::error::{ExtensionError, Result};
use crate::registry::ExtensionRegistry;
use crate::runner::RunnerSession;
use opencmd_protocol::{
    AppEvent, ExecuteRequest, ExecuteResult, ExtensionManifest, ExtensionType, SearchResult,
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

// Forward reference to avoid circular dependency
// In real code, this would be properly abstracted
pub trait ConfigProvider: Send + Sync {
    fn extensions_dir(&self) -> &PathBuf;
}

pub trait EventPublisher: Send + Sync {
    fn publish(&self, event: AppEvent);
}

/// Extension host that manages loading, running, and communicating with extensions.
pub struct ExtensionHost {
    registry: ExtensionRegistry,
    extensions_dir: PathBuf,
    event_publisher: Option<Arc<dyn EventPublisher>>,
    /// Active runner sessions
    runner_sessions: HashMap<String, RunnerSession>,
}

impl ExtensionHost {
    /// Create a new extension host.
    pub async fn new<C, E>(config: Arc<C>, event_bus: Arc<E>) -> Result<Self>
    where
        C: ConfigProvider + 'static,
        E: EventPublisher + 'static,
    {
        Ok(Self {
            registry: ExtensionRegistry::new(),
            extensions_dir: config.extensions_dir().clone(),
            event_publisher: Some(event_bus),
            runner_sessions: HashMap::new(),
        })
    }

    /// Create a new extension host with just a directory path.
    pub fn with_dir(extensions_dir: PathBuf) -> Self {
        Self {
            registry: ExtensionRegistry::new(),
            extensions_dir,
            event_publisher: None,
            runner_sessions: HashMap::new(),
        }
    }

    /// Discover and load extensions from the extensions directory.
    pub async fn discover_extensions(&mut self) -> Result<()> {
        info!("Discovering extensions in {:?}", self.extensions_dir);

        if !self.extensions_dir.exists() {
            info!("Extensions directory does not exist, creating...");
            tokio::fs::create_dir_all(&self.extensions_dir).await?;
            return Ok(());
        }

        let mut entries = tokio::fs::read_dir(&self.extensions_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                if let Err(e) = self.load_extension(&path).await {
                    warn!("Failed to load extension from {:?}: {}", path, e);
                }
            }
        }

        info!("Loaded {} extensions", self.registry.len());
        Ok(())
    }

    /// Load a single extension from a directory.
    pub async fn load_extension(&mut self, path: &PathBuf) -> Result<()> {
        let manifest_path = path.join("manifest.json");

        if !manifest_path.exists() {
            return Err(ExtensionError::InvalidManifest(format!(
                "No manifest.json found in {:?}",
                path
            )));
        }

        let manifest_content = tokio::fs::read_to_string(&manifest_path).await?;
        let manifest: ExtensionManifest = serde_json::from_str(&manifest_content)?;

        debug!("Loaded manifest for extension: {}", manifest.id);

        // Validate manifest based on type
        self.validate_manifest(&manifest)?;

        let extension = self.registry.register(manifest.clone(), path.clone())?;

        // Publish event
        if let Some(ref publisher) = self.event_publisher {
            publisher.publish(AppEvent::ExtensionLoaded {
                extension_id: manifest.id,
            });
        }

        Ok(())
    }

    /// Validate an extension manifest.
    fn validate_manifest(&self, manifest: &ExtensionManifest) -> Result<()> {
        match manifest.extension_type {
            ExtensionType::Runner => {
                if manifest.runner.is_none() {
                    return Err(ExtensionError::InvalidManifest(
                        "Runner extension must have 'runner' configuration".to_string(),
                    ));
                }
            }
            ExtensionType::Integration => {
                if manifest.runtime.is_none() {
                    return Err(ExtensionError::InvalidManifest(
                        "Integration extension must have 'runtime' specified".to_string(),
                    ));
                }
                if manifest.main.is_none() {
                    return Err(ExtensionError::InvalidManifest(
                        "Integration extension must have 'main' entry point".to_string(),
                    ));
                }
            }
            ExtensionType::Command => {
                if manifest.script.is_none() {
                    return Err(ExtensionError::InvalidManifest(
                        "Command extension must have 'script' specified".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    /// Search for commands matching a query.
    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        Ok(self.registry.search(query, limit))
    }

    /// Execute a command.
    pub async fn execute(&self, request: &ExecuteRequest) -> Result<ExecuteResult> {
        let extension = self
            .registry
            .get(&request.extension_id)
            .ok_or_else(|| ExtensionError::NotFound(request.extension_id.clone()))?;

        // Verify command exists
        let command = extension
            .manifest
            .commands
            .iter()
            .find(|c| c.id == request.command_id)
            .ok_or_else(|| ExtensionError::CommandNotFound {
                extension_id: request.extension_id.clone(),
                command_id: request.command_id.clone(),
            })?;

        info!(
            "Executing command: {}:{} ",
            request.extension_id, request.command_id
        );

        match extension.manifest.extension_type {
            ExtensionType::Runner => {
                self.execute_runner(extension, request).await
            }
            ExtensionType::Integration => {
                self.execute_integration(extension, request).await
            }
            ExtensionType::Command => {
                self.execute_command(extension, request).await
            }
        }
    }

    /// Execute a runner extension (spawn PTY process).
    async fn execute_runner(
        &self,
        extension: &opencmd_protocol::Extension,
        request: &ExecuteRequest,
    ) -> Result<ExecuteResult> {
        let runner_config = extension
            .manifest
            .runner
            .as_ref()
            .ok_or_else(|| ExtensionError::Runtime("No runner config".to_string()))?;

        let session = RunnerSession::spawn(runner_config, request.input.as_deref())?;
        let session_id = session.id.clone();

        // Note: In real implementation, we'd store the session and set up output streaming
        // self.runner_sessions.insert(session_id.clone(), session);

        Ok(ExecuteResult::Process { session_id })
    }

    /// Execute an integration extension (WASM/Deno).
    async fn execute_integration(
        &self,
        _extension: &opencmd_protocol::Extension,
        _request: &ExecuteRequest,
    ) -> Result<ExecuteResult> {
        // TODO: Implement WASM/Deno runtime
        Ok(ExecuteResult::Error {
            message: "Integration runtime not yet implemented".to_string(),
        })
    }

    /// Execute a command extension (simple script).
    async fn execute_command(
        &self,
        extension: &opencmd_protocol::Extension,
        _request: &ExecuteRequest,
    ) -> Result<ExecuteResult> {
        let script = extension
            .manifest
            .script
            .as_ref()
            .ok_or_else(|| ExtensionError::Runtime("No script specified".to_string()))?;

        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(script)
            .output()
            .await?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(ExecuteResult::Success {
                output: Some(serde_json::Value::String(stdout.to_string())),
            })
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Ok(ExecuteResult::Error {
                message: stderr.to_string(),
            })
        }
    }

    /// Get a runner session by ID.
    pub fn get_session(&self, session_id: &str) -> Option<&RunnerSession> {
        self.runner_sessions.get(session_id)
    }

    /// Shutdown the extension host.
    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down extension host...");

        // Kill all running sessions
        for (id, session) in self.runner_sessions.drain() {
            debug!("Killing session: {}", id);
            let _ = session.kill().await;
        }

        Ok(())
    }
}
