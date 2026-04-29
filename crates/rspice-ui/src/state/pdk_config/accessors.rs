use std::collections::HashMap;

use super::*;

impl PdkConfig {
    // =========================================================================
    // Dialog Integration Accessors
    // =========================================================================

    /// Get library paths (immutable reference)
    pub fn library_paths(&self) -> &[LibraryPathEntry] {
        &self.library_paths
    }

    /// Get library paths (mutable reference)
    pub fn library_paths_mut(&mut self) -> &mut Vec<LibraryPathEntry> {
        &mut self.library_paths
    }

    /// Get environment variable overrides
    pub fn env_overrides(&self) -> &HashMap<String, String> {
        &self.environment_variables
    }

    /// Get discovered files
    pub fn discovered_files(&self) -> &[DiscoveredFile] {
        &self.discovered_files
    }

    /// Get recent files as string references
    pub fn recent_files(&self) -> Vec<String> {
        self.recent_files
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect()
    }

    /// Toggle path enabled state
    pub fn toggle_path_enabled(&mut self, index: usize) {
        if let Some(entry) = self.library_paths.get_mut(index) {
            entry.enabled = !entry.enabled;
        }
    }

    /// Toggle path recursive state
    pub fn toggle_path_recursive(&mut self, index: usize) {
        if let Some(entry) = self.library_paths.get_mut(index) {
            entry.recursive = !entry.recursive;
        }
    }

    /// Alias for set_env_var for dialog compatibility
    pub fn set_env_override(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.set_env_var(name, value);
    }

    /// Alias for remove_env_var for dialog compatibility
    pub fn remove_env_override(&mut self, name: &str) {
        self.remove_env_var(name);
    }

    /// Load configuration or return default if not found/error
    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_default()
    }
}
