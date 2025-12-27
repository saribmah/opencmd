//! Extension registry for managing loaded extensions.

use crate::error::{ExtensionError, Result};
use opencmd_protocol::{Extension, ExtensionManifest, SearchResult};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{debug, info};
use uuid::Uuid;

/// Registry of loaded extensions.
pub struct ExtensionRegistry {
    extensions: HashMap<String, Extension>,
    matcher: SkimMatcherV2,
}

impl ExtensionRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
            matcher: SkimMatcherV2::default(),
        }
    }

    /// Register an extension from its manifest and path.
    pub fn register(&mut self, manifest: ExtensionManifest, path: PathBuf) -> Result<&Extension> {
        let id = manifest.id.clone();
        info!("Registering extension: {} from {:?}", id, path);

        let extension = Extension {
            id: Uuid::new_v4(),
            manifest,
            path: path.to_string_lossy().to_string(),
            enabled: true,
        };

        self.extensions.insert(id.clone(), extension);
        Ok(self.extensions.get(&id).unwrap())
    }

    /// Unregister an extension by ID.
    pub fn unregister(&mut self, extension_id: &str) -> Result<()> {
        self.extensions
            .remove(extension_id)
            .map(|_| ())
            .ok_or_else(|| ExtensionError::NotFound(extension_id.to_string()))
    }

    /// Get an extension by ID.
    pub fn get(&self, extension_id: &str) -> Option<&Extension> {
        self.extensions.get(extension_id)
    }

    /// Get all registered extensions.
    pub fn all(&self) -> impl Iterator<Item = &Extension> {
        self.extensions.values()
    }

    /// Search for commands matching a query.
    pub fn search(&self, query: &str, limit: usize) -> Vec<SearchResult> {
        let mut results: Vec<(SearchResult, i64)> = Vec::new();

        for extension in self.extensions.values() {
            if !extension.enabled {
                continue;
            }

            for command in &extension.manifest.commands {
                // Build searchable text
                let searchable = format!(
                    "{} {} {}",
                    command.name,
                    command.keywords.join(" "),
                    command.description.as_deref().unwrap_or("")
                );

                // Calculate match score
                let score = if query.is_empty() {
                    // Show all commands when query is empty
                    Some(100)
                } else {
                    self.matcher.fuzzy_match(&searchable, query)
                };

                if let Some(score) = score {
                    let result = SearchResult {
                        id: format!("{}:{}", extension.manifest.id, command.id),
                        extension_id: extension.manifest.id.clone(),
                        command_id: command.id.clone(),
                        name: command.name.clone(),
                        description: command.description.clone(),
                        icon: None,
                        score: score as f64,
                    };
                    results.push((result, score));
                }
            }
        }

        // Sort by score (descending)
        results.sort_by(|a, b| b.1.cmp(&a.1));

        // Take top N results
        results.into_iter().take(limit).map(|(r, _)| r).collect()
    }

    /// Get the number of registered extensions.
    pub fn len(&self) -> usize {
        self.extensions.len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.extensions.is_empty()
    }
}

impl Default for ExtensionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
