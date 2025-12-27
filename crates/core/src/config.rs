//! Application configuration management.

use crate::error::{CoreError, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::info;

/// Application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Path to extensions directory
    #[serde(default = "default_extensions_dir")]
    pub extensions_dir: PathBuf,

    /// Global hotkey to activate the launcher
    #[serde(default = "default_hotkey")]
    pub hotkey: String,

    /// Theme preference
    #[serde(default)]
    pub theme: Theme,

    /// Whether to start on login
    #[serde(default)]
    pub start_on_login: bool,
}

fn default_extensions_dir() -> PathBuf {
    dirs().map(|d| d.join("extensions")).unwrap_or_else(|| PathBuf::from("extensions"))
}

fn default_hotkey() -> String {
    "Cmd+Space".to_string()
}

/// Theme preference.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            extensions_dir: default_extensions_dir(),
            hotkey: default_hotkey(),
            theme: Theme::default(),
            start_on_login: false,
        }
    }
}

/// Get the application data directory.
fn dirs() -> Option<PathBuf> {
    directories::ProjectDirs::from("", "", "OpenCMD").map(|d| d.data_dir().to_path_buf())
}

/// Configuration manager for loading/saving config.
pub struct ConfigManager {
    config: Config,
    config_path: PathBuf,
}

impl ConfigManager {
    /// Load configuration from disk or create default.
    pub async fn load() -> Result<Self> {
        let config_path = dirs()
            .map(|d| d.join("config.toml"))
            .unwrap_or_else(|| PathBuf::from("config.toml"));

        let config = if config_path.exists() {
            let content = tokio::fs::read_to_string(&config_path).await?;
            toml::from_str(&content).map_err(|e| CoreError::Config(e.to_string()))?
        } else {
            info!("No config found, using defaults");
            Config::default()
        };

        Ok(Self { config, config_path })
    }

    /// Get the current configuration.
    pub fn get(&self) -> &Config {
        &self.config
    }

    /// Get the extensions directory path.
    pub fn extensions_dir(&self) -> &PathBuf {
        &self.config.extensions_dir
    }

    /// Update configuration and persist to disk.
    pub async fn update(&mut self, config: Config) -> Result<()> {
        self.config = config;
        self.save().await
    }

    /// Save current configuration to disk.
    pub async fn save(&self) -> Result<()> {
        if let Some(parent) = self.config_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let content = toml::to_string_pretty(&self.config)
            .map_err(|e| CoreError::Config(e.to_string()))?;

        tokio::fs::write(&self.config_path, content).await?;
        info!("Configuration saved to {:?}", self.config_path);
        Ok(())
    }
}
