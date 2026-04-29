use std::path::{Path, PathBuf};

use super::*;

impl PdkConfig {
    // =========================================================================
    // Persistence
    // =========================================================================

    /// Get the default configuration file path
    pub fn default_config_path() -> PathBuf {
        dirs::config_dir()
            .or_else(dirs::home_dir)
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rspice")
            .join(CONFIG_FILE_NAME)
    }

    /// Load configuration from the default path
    pub fn load() -> Result<Self, ConfigError> {
        Self::load_from(&Self::default_config_path())
    }

    /// Load configuration from a specific path
    pub fn load_from(path: &Path) -> Result<Self, ConfigError> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(path).map_err(|e| ConfigError::Io(e.to_string()))?;

        serde_json::from_str(&content).map_err(|e| ConfigError::Parse(e.to_string()))
    }

    /// Save configuration to the default path
    pub fn save(&self) -> Result<(), ConfigError> {
        self.save_to(&Self::default_config_path())
    }

    /// Save configuration to a specific path
    pub fn save_to(&self, path: &Path) -> Result<(), ConfigError> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| ConfigError::Io(e.to_string()))?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| ConfigError::Serialize(e.to_string()))?;

        std::fs::write(path, content).map_err(|e| ConfigError::Io(e.to_string()))
    }

    /// Check if configuration has any content
    pub fn is_empty(&self) -> bool {
        self.library_paths.is_empty()
            && self.environment_variables.is_empty()
            && self.recent_files.is_empty()
    }

    /// Get total discovered file count
    pub fn total_file_count(&self) -> usize {
        self.discovered_files.len()
    }
}
