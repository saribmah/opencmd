//! Core application logic for OpenCMD.
//!
//! This crate provides:
//! - `AppCore`: Main orchestrator that coordinates all subsystems
//! - `Config`: Application configuration management
//! - `EventBus`: Pub/sub event system for component communication

mod config;
mod error;
mod event_bus;

pub use config::{Config, ConfigManager};
pub use error::{CoreError, Result};
pub use event_bus::{EventBus, EventSubscriber};

use opencmd_extension_host::ExtensionHost;
use opencmd_protocol::{AppEvent, ExecuteRequest, ExecuteResult, SearchRequest, SearchResult};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

// Implement the traits from extension-host for our types
impl opencmd_extension_host::ConfigProvider for ConfigManager {
    fn extensions_dir(&self) -> &PathBuf {
        ConfigManager::extensions_dir(self)
    }
}

impl opencmd_extension_host::EventPublisher for EventBus {
    fn publish(&self, event: AppEvent) {
        EventBus::publish(self, event)
    }
}

/// Main application core that orchestrates all subsystems.
pub struct AppCore {
    /// Configuration manager
    pub config: Arc<ConfigManager>,
    /// Event bus for pub/sub communication
    pub event_bus: Arc<EventBus>,
    /// Extension host for managing extensions
    pub extension_host: Arc<RwLock<ExtensionHost>>,
}

impl AppCore {
    /// Create a new AppCore instance.
    pub async fn new() -> Result<Self> {
        info!("Initializing OpenCMD core...");

        let config = Arc::new(ConfigManager::load().await?);
        let event_bus = Arc::new(EventBus::new());
        let extension_host = Arc::new(RwLock::new(
            ExtensionHost::new(config.clone(), event_bus.clone()).await?,
        ));

        Ok(Self {
            config,
            event_bus,
            extension_host,
        })
    }

    /// Initialize the application (load extensions, etc.)
    pub async fn initialize(&self) -> Result<()> {
        info!("Loading extensions...");
        let mut host = self.extension_host.write().await;
        host.discover_extensions().await?;
        info!("OpenCMD core initialized successfully");
        Ok(())
    }

    /// Search for commands matching the query.
    pub async fn search(&self, request: SearchRequest) -> Result<Vec<SearchResult>> {
        let host = self.extension_host.read().await;
        Ok(host.search(&request.query, request.limit).await?)
    }

    /// Execute a command.
    pub async fn execute(&self, request: ExecuteRequest) -> Result<ExecuteResult> {
        let host = self.extension_host.read().await;
        Ok(host.execute(&request).await?)
    }

    /// Shutdown the application.
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down OpenCMD core...");
        let mut host = self.extension_host.write().await;
        host.shutdown().await?;
        Ok(())
    }
}
