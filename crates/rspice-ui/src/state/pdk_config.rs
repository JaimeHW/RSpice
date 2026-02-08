//! PDK Configuration Management
//!
//! Commercial-grade Process Design Kit (PDK) path configuration with persistent
//! storage, environment variable expansion, and automatic model file discovery.
//!
//! # Architecture
//!
//! Matches Cadence Spectre's model library management:
//! - **Library Paths**: User-configured directories to scan for model files
//! - **Environment Variables**: `$PDK_HOME`, `$MY_TECH` style variable support
//! - **File Discovery**: Automatic scanning for `.lib`, `.scs`, `.mod` files
//! - **Persistence**: JSON-based configuration saved to user's config directory
//!
//! # Usage
//!
//! ```rust
//! use rspice_ui::state::pdk_config::{PdkConfig, DiscoveredFile};
//!
//! let mut config = PdkConfig::new();
//! config.add_library_path("/path/to/pdk/models");
//! config.set_env_var("PDK_HOME", "/opt/tsmc180");
//!
//! // Expand paths with environment variables
//! let expanded = config.expand_path("$PDK_HOME/models/nmos.lib");
//!
//! // Discover model files in configured paths
//! let files = config.discover_model_files();
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// =============================================================================
// Constants
// =============================================================================

/// Supported model file extensions
pub const MODEL_FILE_EXTENSIONS: &[&str] = &["lib", "scs", "mod", "sp", "cir"];

/// Default configuration file name
pub const CONFIG_FILE_NAME: &str = "pdk_config.json";

/// Maximum directory recursion depth for scanning
pub const MAX_SCAN_DEPTH: usize = 10;

// =============================================================================
// Discovered File
// =============================================================================

/// A discovered model file from scanning library paths
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoveredFile {
    /// Absolute path to the file
    pub path: PathBuf,
    /// File extension (e.g., "lib", "scs")
    pub extension: String,
    /// File size in bytes
    pub size: u64,
    /// Parent library path that contained this file
    pub source_path: PathBuf,
    /// Available sections/corners (populated after parsing)
    pub sections: Vec<String>,
}

impl DiscoveredFile {
    /// Create a new discovered file entry
    pub fn new(path: PathBuf, source_path: PathBuf) -> Self {
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);

        Self {
            path,
            extension,
            size,
            source_path,
            sections: Vec::new(),
        }
    }

    /// Get the file name without path
    pub fn file_name(&self) -> &str {
        self.path.file_name().and_then(|n| n.to_str()).unwrap_or("")
    }

    /// Get the file stem (name without extension)
    pub fn file_stem(&self) -> &str {
        self.path.file_stem().and_then(|n| n.to_str()).unwrap_or("")
    }

    /// Check if this is a library file (.lib)
    pub fn is_lib(&self) -> bool {
        self.extension == "lib"
    }

    /// Check if this is a Spectre file (.scs)
    pub fn is_scs(&self) -> bool {
        self.extension == "scs"
    }

    /// Human-readable file size
    pub fn size_display(&self) -> String {
        if self.size < 1024 {
            format!("{} B", self.size)
        } else if self.size < 1024 * 1024 {
            format!("{:.1} KB", self.size as f64 / 1024.0)
        } else {
            format!("{:.1} MB", self.size as f64 / (1024.0 * 1024.0))
        }
    }

    /// Get file type (alias for extension for dialog compatibility)
    pub fn file_type(&self) -> &str {
        &self.extension
    }

    /// Get path as string (for dialog display)
    pub fn path_str(&self) -> String {
        self.path.to_string_lossy().to_string()
    }
}

// =============================================================================
// Library Path Entry
// =============================================================================

/// A configured library path with metadata
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LibraryPathEntry {
    /// The path (may contain environment variables like $PDK_HOME)
    pub path: String,
    /// Whether this path is enabled for scanning
    pub enabled: bool,
    /// Whether to scan subdirectories recursively
    pub recursive: bool,
    /// User-provided description/label
    pub label: Option<String>,
    /// Last scan timestamp (Unix epoch seconds)
    pub last_scanned: Option<u64>,
    /// Number of files found in last scan
    pub file_count: usize,
}

impl LibraryPathEntry {
    /// Create a new library path entry
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            enabled: true,
            recursive: true,
            label: None,
            last_scanned: None,
            file_count: 0,
        }
    }

    /// Create with a label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set recursive scanning
    pub fn with_recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }

    /// Set enabled state
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Get display name (label or path basename)
    pub fn display_name(&self) -> &str {
        self.label.as_deref().unwrap_or_else(|| {
            Path::new(&self.path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&self.path)
        })
    }
}

impl Default for LibraryPathEntry {
    fn default() -> Self {
        Self::new("")
    }
}

// =============================================================================
// PDK Configuration
// =============================================================================

/// PDK configuration with library paths and environment variables
///
/// Provides persistent storage and automatic model file discovery
/// matching Cadence Spectre's model library management workflow.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PdkConfig {
    /// Configured library search paths
    pub library_paths: Vec<LibraryPathEntry>,

    /// Environment variable overrides (e.g., PDK_HOME -> /opt/tsmc180)
    pub environment_variables: HashMap<String, String>,

    /// Recently loaded files for quick access
    pub recent_files: Vec<PathBuf>,

    /// Maximum number of recent files to remember
    #[serde(default = "default_max_recent")]
    pub max_recent_files: usize,

    /// Discovered files from last scan (not persisted by default)
    #[serde(skip)]
    pub discovered_files: Vec<DiscoveredFile>,

    /// Scan errors from last discovery (not persisted)
    #[serde(skip)]
    pub scan_errors: Vec<String>,
}

fn default_max_recent() -> usize {
    20
}

impl Default for PdkConfig {
    fn default() -> Self {
        Self {
            library_paths: Vec::new(),
            environment_variables: HashMap::new(),
            recent_files: Vec::new(),
            max_recent_files: default_max_recent(),
            discovered_files: Vec::new(),
            scan_errors: Vec::new(),
        }
    }
}

impl PdkConfig {
    /// Create a new empty configuration
    pub fn new() -> Self {
        Self::default()
    }

    // =========================================================================
    // Library Path Management
    // =========================================================================

    /// Add a library search path
    pub fn add_library_path(&mut self, path: impl Into<String>) {
        let entry = LibraryPathEntry::new(path);
        if !self.library_paths.iter().any(|e| e.path == entry.path) {
            self.library_paths.push(entry);
        }
    }

    /// Add a library path entry with full configuration
    pub fn add_library_path_entry(&mut self, entry: LibraryPathEntry) {
        if !self.library_paths.iter().any(|e| e.path == entry.path) {
            self.library_paths.push(entry);
        }
    }

    /// Remove a library path by index
    pub fn remove_library_path(&mut self, index: usize) -> Option<LibraryPathEntry> {
        if index < self.library_paths.len() {
            Some(self.library_paths.remove(index))
        } else {
            None
        }
    }

    /// Get enabled library paths
    pub fn enabled_paths(&self) -> impl Iterator<Item = &LibraryPathEntry> {
        self.library_paths.iter().filter(|e| e.enabled)
    }

    /// Move a library path up in priority
    pub fn move_path_up(&mut self, index: usize) {
        if index > 0 && index < self.library_paths.len() {
            self.library_paths.swap(index, index - 1);
        }
    }

    /// Move a library path down in priority
    pub fn move_path_down(&mut self, index: usize) {
        if index + 1 < self.library_paths.len() {
            self.library_paths.swap(index, index + 1);
        }
    }

    // =========================================================================
    // Environment Variables
    // =========================================================================

    /// Set an environment variable override
    pub fn set_env_var(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.environment_variables.insert(name.into(), value.into());
    }

    /// Remove an environment variable
    pub fn remove_env_var(&mut self, name: &str) -> Option<String> {
        self.environment_variables.remove(name)
    }

    /// Get an environment variable (checks overrides first, then system)
    pub fn get_env_var(&self, name: &str) -> Option<String> {
        self.environment_variables
            .get(name)
            .cloned()
            .or_else(|| std::env::var(name).ok())
    }

    /// Expand environment variables in a path string
    ///
    /// Supports:
    /// - `$VAR` and `${VAR}` syntax
    /// - Nested resolution
    /// - Both config overrides and system environment
    pub fn expand_path(&self, path: &str) -> String {
        let mut result = path.to_string();
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 10; // Prevent infinite recursion

        // Keep expanding until no more variables or max iterations
        while iterations < MAX_ITERATIONS {
            let before = result.clone();
            result = self.expand_path_once(&result);
            if result == before {
                break;
            }
            iterations += 1;
        }

        result
    }

    /// Single pass of environment variable expansion
    fn expand_path_once(&self, path: &str) -> String {
        let mut result = String::with_capacity(path.len());
        let mut chars = path.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '$' {
                // Check for ${VAR} or $VAR syntax
                let var_name = if chars.peek() == Some(&'{') {
                    chars.next(); // consume '{'
                    let mut name = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch == '}' {
                            chars.next(); // consume '}'
                            break;
                        }
                        name.push(chars.next().unwrap());
                    }
                    name
                } else {
                    // $VAR - read until non-alphanumeric/underscore
                    let mut name = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            name.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    name
                };

                // Expand the variable
                if !var_name.is_empty() {
                    if let Some(value) = self.get_env_var(&var_name) {
                        result.push_str(&value);
                    } else {
                        // Keep original if not found
                        result.push('$');
                        result.push_str(&var_name);
                    }
                } else {
                    result.push('$');
                }
            } else {
                result.push(c);
            }
        }

        result
    }

    /// Expand a path and convert to PathBuf
    pub fn expand_path_buf(&self, path: &str) -> PathBuf {
        PathBuf::from(self.expand_path(path))
    }

    // =========================================================================
    // File Discovery
    // =========================================================================

    /// Discover model files in all enabled library paths
    pub fn discover_model_files(&mut self) -> &[DiscoveredFile] {
        self.discovered_files.clear();
        self.scan_errors.clear();

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        // Pre-compute expanded paths to avoid borrow conflicts
        let entry_data: Vec<(usize, String, PathBuf, bool)> = self
            .library_paths
            .iter()
            .enumerate()
            .filter(|(_, entry)| entry.enabled)
            .map(|(idx, entry)| {
                let expanded = self.expand_path(&entry.path);
                let path_buf = PathBuf::from(&expanded);
                (idx, expanded, path_buf, entry.recursive)
            })
            .collect();

        // Now process each entry
        for (idx, expanded_path, path, recursive) in entry_data {
            if !path.exists() {
                let original_path = &self.library_paths[idx].path;
                self.scan_errors.push(format!(
                    "Path does not exist: {} (expanded from {})",
                    expanded_path, original_path
                ));
                continue;
            }

            let mut files = Vec::new();
            let max_depth = if recursive { MAX_SCAN_DEPTH } else { 0 };

            if let Err(e) = Self::scan_directory_recursive(&path, &path, max_depth, 0, &mut files) {
                self.scan_errors
                    .push(format!("Error scanning {}: {}", expanded_path, e));
            }

            // Update entry metadata
            self.library_paths[idx].file_count = files.len();
            self.library_paths[idx].last_scanned = Some(now);
            self.discovered_files.extend(files);
        }

        // Sort by path for consistent ordering
        self.discovered_files.sort_by(|a, b| a.path.cmp(&b.path));

        &self.discovered_files
    }

    /// Recursively scan a directory for model files
    fn scan_directory_recursive(
        base_path: &Path,
        current_dir: &Path,
        max_depth: usize,
        current_depth: usize,
        results: &mut Vec<DiscoveredFile>,
    ) -> Result<(), std::io::Error> {
        if current_depth > max_depth {
            return Ok(());
        }

        let entries = std::fs::read_dir(current_dir)?;

        for entry in entries.flatten() {
            let path = entry.path();
            let file_type = entry.file_type()?;

            if file_type.is_file() {
                // Check if it's a model file
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext_lower = ext.to_lowercase();
                    if MODEL_FILE_EXTENSIONS.contains(&ext_lower.as_str()) {
                        results.push(DiscoveredFile::new(path, base_path.to_path_buf()));
                    }
                }
            } else if file_type.is_dir() {
                // Skip hidden directories
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if !name_str.starts_with('.') {
                    Self::scan_directory_recursive(
                        base_path,
                        &path,
                        max_depth,
                        current_depth + 1,
                        results,
                    )?;
                }
            }
        }

        Ok(())
    }

    /// Get discovered files filtered by extension
    pub fn files_by_extension(&self, ext: &str) -> Vec<&DiscoveredFile> {
        let ext_lower = ext.to_lowercase();
        self.discovered_files
            .iter()
            .filter(|f| f.extension == ext_lower)
            .collect()
    }

    /// Get all library files (.lib)
    pub fn lib_files(&self) -> Vec<&DiscoveredFile> {
        self.files_by_extension("lib")
    }

    /// Get all Spectre files (.scs)
    pub fn scs_files(&self) -> Vec<&DiscoveredFile> {
        self.files_by_extension("scs")
    }

    // =========================================================================
    // Recent Files
    // =========================================================================

    /// Add a file to the recent files list
    pub fn add_recent_file(&mut self, path: impl Into<PathBuf>) {
        let path = path.into();

        // Remove if already exists (will re-add at front)
        self.recent_files.retain(|p| p != &path);

        // Add to front
        self.recent_files.insert(0, path);

        // Trim to max size
        self.recent_files.truncate(self.max_recent_files);
    }

    /// Clear recent files
    pub fn clear_recent_files(&mut self) {
        self.recent_files.clear();
    }

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

// =============================================================================
// Error Types
// =============================================================================

/// Configuration error types
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigError {
    /// I/O error reading/writing file
    Io(String),
    /// JSON parse error
    Parse(String),
    /// JSON serialize error  
    Serialize(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(msg) => write!(f, "I/O error: {}", msg),
            ConfigError::Parse(msg) => write!(f, "Parse error: {}", msg),
            ConfigError::Serialize(msg) => write!(f, "Serialize error: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // =========================================================================
    // PdkConfig Basic Tests
    // =========================================================================

    #[test]
    fn test_pdk_config_new() {
        let config = PdkConfig::new();
        assert!(config.library_paths.is_empty());
        assert!(config.environment_variables.is_empty());
        assert!(config.recent_files.is_empty());
        assert!(config.is_empty());
    }

    #[test]
    fn test_pdk_config_default() {
        let config = PdkConfig::default();
        assert_eq!(config.max_recent_files, 20);
        assert!(config.discovered_files.is_empty());
        assert!(config.scan_errors.is_empty());
    }

    // =========================================================================
    // Library Path Management Tests
    // =========================================================================

    #[test]
    fn test_add_library_path() {
        let mut config = PdkConfig::new();
        config.add_library_path("/path/to/pdk");

        assert_eq!(config.library_paths.len(), 1);
        assert_eq!(config.library_paths[0].path, "/path/to/pdk");
        assert!(config.library_paths[0].enabled);
        assert!(config.library_paths[0].recursive);
    }

    #[test]
    fn test_add_duplicate_library_path() {
        let mut config = PdkConfig::new();
        config.add_library_path("/path/to/pdk");
        config.add_library_path("/path/to/pdk");

        // Should not add duplicate
        assert_eq!(config.library_paths.len(), 1);
    }

    #[test]
    fn test_add_library_path_entry() {
        let mut config = PdkConfig::new();
        let entry = LibraryPathEntry::new("/custom/path")
            .with_label("My PDK")
            .with_recursive(false)
            .with_enabled(true);

        config.add_library_path_entry(entry);

        assert_eq!(config.library_paths.len(), 1);
        assert_eq!(config.library_paths[0].label, Some("My PDK".to_string()));
        assert!(!config.library_paths[0].recursive);
    }

    #[test]
    fn test_remove_library_path() {
        let mut config = PdkConfig::new();
        config.add_library_path("/path/one");
        config.add_library_path("/path/two");

        let removed = config.remove_library_path(0);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().path, "/path/one");
        assert_eq!(config.library_paths.len(), 1);
        assert_eq!(config.library_paths[0].path, "/path/two");
    }

    #[test]
    fn test_remove_library_path_out_of_bounds() {
        let mut config = PdkConfig::new();
        config.add_library_path("/path/one");

        assert!(config.remove_library_path(5).is_none());
        assert_eq!(config.library_paths.len(), 1);
    }

    #[test]
    fn test_enabled_paths() {
        let mut config = PdkConfig::new();
        config.add_library_path_entry(LibraryPathEntry::new("/path/one").with_enabled(true));
        config.add_library_path_entry(LibraryPathEntry::new("/path/two").with_enabled(false));
        config.add_library_path_entry(LibraryPathEntry::new("/path/three").with_enabled(true));

        let enabled: Vec<_> = config.enabled_paths().collect();
        assert_eq!(enabled.len(), 2);
        assert_eq!(enabled[0].path, "/path/one");
        assert_eq!(enabled[1].path, "/path/three");
    }

    #[test]
    fn test_move_path_up() {
        let mut config = PdkConfig::new();
        config.add_library_path("/path/one");
        config.add_library_path("/path/two");
        config.add_library_path("/path/three");

        config.move_path_up(2);

        assert_eq!(config.library_paths[0].path, "/path/one");
        assert_eq!(config.library_paths[1].path, "/path/three");
        assert_eq!(config.library_paths[2].path, "/path/two");
    }

    #[test]
    fn test_move_path_up_first_element() {
        let mut config = PdkConfig::new();
        config.add_library_path("/path/one");
        config.add_library_path("/path/two");

        config.move_path_up(0); // Should do nothing

        assert_eq!(config.library_paths[0].path, "/path/one");
        assert_eq!(config.library_paths[1].path, "/path/two");
    }

    #[test]
    fn test_move_path_down() {
        let mut config = PdkConfig::new();
        config.add_library_path("/path/one");
        config.add_library_path("/path/two");
        config.add_library_path("/path/three");

        config.move_path_down(0);

        assert_eq!(config.library_paths[0].path, "/path/two");
        assert_eq!(config.library_paths[1].path, "/path/one");
        assert_eq!(config.library_paths[2].path, "/path/three");
    }

    #[test]
    fn test_move_path_down_last_element() {
        let mut config = PdkConfig::new();
        config.add_library_path("/path/one");
        config.add_library_path("/path/two");

        config.move_path_down(1); // Should do nothing

        assert_eq!(config.library_paths[0].path, "/path/one");
        assert_eq!(config.library_paths[1].path, "/path/two");
    }

    // =========================================================================
    // Environment Variable Tests
    // =========================================================================

    #[test]
    fn test_set_env_var() {
        let mut config = PdkConfig::new();
        config.set_env_var("PDK_HOME", "/opt/tsmc180");

        assert_eq!(
            config.environment_variables.get("PDK_HOME"),
            Some(&"/opt/tsmc180".to_string())
        );
    }

    #[test]
    fn test_get_env_var_from_config() {
        let mut config = PdkConfig::new();
        config.set_env_var("MY_VAR", "my_value");

        assert_eq!(config.get_env_var("MY_VAR"), Some("my_value".to_string()));
    }

    #[test]
    fn test_get_env_var_not_found() {
        let config = PdkConfig::new();
        // This test depends on the variable not existing in the system
        // Using a very unlikely variable name
        assert!(config.get_env_var("RSPICE_NONEXISTENT_VAR_12345").is_none());
    }

    #[test]
    fn test_remove_env_var() {
        let mut config = PdkConfig::new();
        config.set_env_var("MY_VAR", "my_value");

        let removed = config.remove_env_var("MY_VAR");
        assert_eq!(removed, Some("my_value".to_string()));
        assert!(config.environment_variables.get("MY_VAR").is_none());
    }

    #[test]
    fn test_remove_nonexistent_env_var() {
        let mut config = PdkConfig::new();
        assert!(config.remove_env_var("DOESNT_EXIST").is_none());
    }

    // =========================================================================
    // Path Expansion Tests
    // =========================================================================

    #[test]
    fn test_expand_path_no_variables() {
        let config = PdkConfig::new();
        let result = config.expand_path("/simple/path/to/file.lib");
        assert_eq!(result, "/simple/path/to/file.lib");
    }

    #[test]
    fn test_expand_path_simple_variable() {
        let mut config = PdkConfig::new();
        config.set_env_var("PDK_HOME", "/opt/tsmc180");

        let result = config.expand_path("$PDK_HOME/models/nmos.lib");
        assert_eq!(result, "/opt/tsmc180/models/nmos.lib");
    }

    #[test]
    fn test_expand_path_braced_variable() {
        let mut config = PdkConfig::new();
        config.set_env_var("PDK_HOME", "/opt/tsmc180");

        let result = config.expand_path("${PDK_HOME}/models/nmos.lib");
        assert_eq!(result, "/opt/tsmc180/models/nmos.lib");
    }

    #[test]
    fn test_expand_path_multiple_variables() {
        let mut config = PdkConfig::new();
        config.set_env_var("PDK_HOME", "/opt/pdk");
        config.set_env_var("VERSION", "v1.0");

        let result = config.expand_path("$PDK_HOME/$VERSION/models");
        assert_eq!(result, "/opt/pdk/v1.0/models");
    }

    #[test]
    fn test_expand_path_nested_variables() {
        let mut config = PdkConfig::new();
        config.set_env_var("BASE", "/opt");
        config.set_env_var("PDK_HOME", "$BASE/tsmc180");

        let result = config.expand_path("$PDK_HOME/models");
        assert_eq!(result, "/opt/tsmc180/models");
    }

    #[test]
    fn test_expand_path_unknown_variable_preserved() {
        let config = PdkConfig::new();
        let result = config.expand_path("$UNKNOWN_VAR_XYZ/path");
        // Unknown variables should be preserved
        assert_eq!(result, "$UNKNOWN_VAR_XYZ/path");
    }

    #[test]
    fn test_expand_path_dollar_only() {
        let config = PdkConfig::new();
        let result = config.expand_path("path/with/$/dollar");
        assert_eq!(result, "path/with/$/dollar");
    }

    #[test]
    fn test_expand_path_empty_braces() {
        let config = PdkConfig::new();
        let result = config.expand_path("path/${}/empty");
        // Empty braces should just produce $
        assert_eq!(result, "path/$/empty");
    }

    #[test]
    fn test_expand_path_mixed_syntax() {
        let mut config = PdkConfig::new();
        config.set_env_var("A", "alpha");
        config.set_env_var("B", "beta");

        let result = config.expand_path("$A/${B}/file");
        assert_eq!(result, "alpha/beta/file");
    }

    #[test]
    fn test_expand_path_buf() {
        let mut config = PdkConfig::new();
        config.set_env_var("HOME", "/users/test");

        let result = config.expand_path_buf("$HOME/models");
        assert_eq!(result, PathBuf::from("/users/test/models"));
    }

    // =========================================================================
    // Recent Files Tests
    // =========================================================================

    #[test]
    fn test_add_recent_file() {
        let mut config = PdkConfig::new();
        config.add_recent_file("/path/to/file1.lib");
        config.add_recent_file("/path/to/file2.lib");

        assert_eq!(config.recent_files.len(), 2);
        // Most recent should be first
        assert_eq!(config.recent_files[0], PathBuf::from("/path/to/file2.lib"));
        assert_eq!(config.recent_files[1], PathBuf::from("/path/to/file1.lib"));
    }

    #[test]
    fn test_add_recent_file_duplicate_moves_to_front() {
        let mut config = PdkConfig::new();
        config.add_recent_file("/path/to/file1.lib");
        config.add_recent_file("/path/to/file2.lib");
        config.add_recent_file("/path/to/file1.lib"); // Duplicate

        assert_eq!(config.recent_files.len(), 2);
        assert_eq!(config.recent_files[0], PathBuf::from("/path/to/file1.lib"));
        assert_eq!(config.recent_files[1], PathBuf::from("/path/to/file2.lib"));
    }

    #[test]
    fn test_add_recent_file_max_limit() {
        let mut config = PdkConfig::new();
        config.max_recent_files = 3;

        for i in 0..5 {
            config.add_recent_file(format!("/path/file{}.lib", i));
        }

        assert_eq!(config.recent_files.len(), 3);
        // Most recent 3 should be kept
        assert_eq!(config.recent_files[0], PathBuf::from("/path/file4.lib"));
        assert_eq!(config.recent_files[1], PathBuf::from("/path/file3.lib"));
        assert_eq!(config.recent_files[2], PathBuf::from("/path/file2.lib"));
    }

    #[test]
    fn test_clear_recent_files() {
        let mut config = PdkConfig::new();
        config.add_recent_file("/path/to/file1.lib");
        config.add_recent_file("/path/to/file2.lib");

        config.clear_recent_files();

        assert!(config.recent_files.is_empty());
    }

    // =========================================================================
    // File Discovery Tests
    // =========================================================================

    #[test]
    fn test_discover_model_files_empty_paths() {
        let mut config = PdkConfig::new();
        let files = config.discover_model_files();
        assert!(files.is_empty());
    }

    #[test]
    fn test_discover_model_files_nonexistent_path() {
        let mut config = PdkConfig::new();
        config.add_library_path("/nonexistent/path/12345");

        let files = config.discover_model_files();

        assert!(files.is_empty());
        assert!(!config.scan_errors.is_empty());
    }

    #[test]
    fn test_discover_model_files_with_temp_dir() {
        let temp_dir = TempDir::new().unwrap();

        // Create test files
        fs::write(temp_dir.path().join("nmos.lib"), "* NMOS model").unwrap();
        fs::write(temp_dir.path().join("pmos.scs"), "* PMOS model").unwrap();
        fs::write(temp_dir.path().join("readme.txt"), "Not a model").unwrap();

        let mut config = PdkConfig::new();
        config.add_library_path(temp_dir.path().to_string_lossy().to_string());

        let files = config.discover_model_files();

        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|f| f.file_name() == "nmos.lib"));
        assert!(files.iter().any(|f| f.file_name() == "pmos.scs"));
    }

    #[test]
    fn test_discover_model_files_recursive() {
        let temp_dir = TempDir::new().unwrap();
        let sub_dir = temp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();

        fs::write(temp_dir.path().join("top.lib"), "* Top model").unwrap();
        fs::write(sub_dir.join("nested.lib"), "* Nested model").unwrap();

        let mut config = PdkConfig::new();
        config.add_library_path_entry(
            LibraryPathEntry::new(temp_dir.path().to_string_lossy().to_string())
                .with_recursive(true),
        );

        let files = config.discover_model_files();

        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|f| f.file_name() == "top.lib"));
        assert!(files.iter().any(|f| f.file_name() == "nested.lib"));
    }

    #[test]
    fn test_discover_model_files_non_recursive() {
        let temp_dir = TempDir::new().unwrap();
        let sub_dir = temp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();

        fs::write(temp_dir.path().join("top.lib"), "* Top model").unwrap();
        fs::write(sub_dir.join("nested.lib"), "* Nested model").unwrap();

        let mut config = PdkConfig::new();
        config.add_library_path_entry(
            LibraryPathEntry::new(temp_dir.path().to_string_lossy().to_string())
                .with_recursive(false),
        );

        let files = config.discover_model_files();

        // Should only find top-level file
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].file_name(), "top.lib");
    }

    #[test]
    fn test_discover_model_files_disabled_path() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("model.lib"), "* Model").unwrap();

        let mut config = PdkConfig::new();
        config.add_library_path_entry(
            LibraryPathEntry::new(temp_dir.path().to_string_lossy().to_string())
                .with_enabled(false),
        );

        let files = config.discover_model_files();

        assert!(files.is_empty());
    }

    #[test]
    fn test_discover_model_files_skips_hidden_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let hidden_dir = temp_dir.path().join(".hidden");
        fs::create_dir(&hidden_dir).unwrap();

        fs::write(temp_dir.path().join("visible.lib"), "* Visible").unwrap();
        fs::write(hidden_dir.join("hidden.lib"), "* Hidden").unwrap();

        let mut config = PdkConfig::new();
        config.add_library_path(temp_dir.path().to_string_lossy().to_string());

        let files = config.discover_model_files();

        // Should only find visible file
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].file_name(), "visible.lib");
    }

    #[test]
    fn test_files_by_extension() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("model1.lib"), "* Lib").unwrap();
        fs::write(temp_dir.path().join("model2.lib"), "* Lib").unwrap();
        fs::write(temp_dir.path().join("model3.scs"), "* Scs").unwrap();

        let mut config = PdkConfig::new();
        config.add_library_path(temp_dir.path().to_string_lossy().to_string());
        config.discover_model_files();

        let lib_files = config.files_by_extension("lib");
        assert_eq!(lib_files.len(), 2);

        let scs_files = config.files_by_extension("scs");
        assert_eq!(scs_files.len(), 1);
    }

    #[test]
    fn test_lib_files_convenience() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("a.lib"), "* Lib").unwrap();
        fs::write(temp_dir.path().join("b.scs"), "* Scs").unwrap();

        let mut config = PdkConfig::new();
        config.add_library_path(temp_dir.path().to_string_lossy().to_string());
        config.discover_model_files();

        assert_eq!(config.lib_files().len(), 1);
        assert_eq!(config.scs_files().len(), 1);
    }

    // =========================================================================
    // DiscoveredFile Tests
    // =========================================================================

    #[test]
    fn test_discovered_file_new() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.lib");
        fs::write(&file_path, "* Test content here").unwrap();

        let df = DiscoveredFile::new(file_path.clone(), temp_dir.path().to_path_buf());

        assert_eq!(df.path, file_path);
        assert_eq!(df.extension, "lib");
        assert_eq!(df.file_name(), "test.lib");
        assert_eq!(df.file_stem(), "test");
        assert!(df.is_lib());
        assert!(!df.is_scs());
        assert!(df.size > 0);
    }

    #[test]
    fn test_discovered_file_size_display() {
        let df = DiscoveredFile {
            path: PathBuf::from("/test.lib"),
            extension: "lib".to_string(),
            size: 512,
            source_path: PathBuf::from("/"),
            sections: Vec::new(),
        };
        assert_eq!(df.size_display(), "512 B");

        let df2 = DiscoveredFile {
            size: 2048,
            ..df.clone()
        };
        assert_eq!(df2.size_display(), "2.0 KB");

        let df3 = DiscoveredFile {
            size: 1_500_000,
            ..df
        };
        assert_eq!(df3.size_display(), "1.4 MB");
    }

    // =========================================================================
    // LibraryPathEntry Tests
    // =========================================================================

    #[test]
    fn test_library_path_entry_new() {
        let entry = LibraryPathEntry::new("/path/to/lib");
        assert_eq!(entry.path, "/path/to/lib");
        assert!(entry.enabled);
        assert!(entry.recursive);
        assert!(entry.label.is_none());
    }

    #[test]
    fn test_library_path_entry_display_name_with_label() {
        let entry = LibraryPathEntry::new("/path/to/lib").with_label("My PDK");
        assert_eq!(entry.display_name(), "My PDK");
    }

    #[test]
    fn test_library_path_entry_display_name_without_label() {
        let entry = LibraryPathEntry::new("/path/to/lib");
        assert_eq!(entry.display_name(), "lib");
    }

    // =========================================================================
    // Persistence Tests
    // =========================================================================

    #[test]
    fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("pdk_config.json");

        let mut config = PdkConfig::new();
        config.add_library_path("/path/to/pdk");
        config.set_env_var("PDK_HOME", "/opt/pdk");
        config.add_recent_file("/recent/file.lib");

        // Save
        config.save_to(&config_path).unwrap();

        // Load
        let loaded = PdkConfig::load_from(&config_path).unwrap();

        assert_eq!(loaded.library_paths.len(), 1);
        assert_eq!(loaded.library_paths[0].path, "/path/to/pdk");
        assert_eq!(
            loaded.environment_variables.get("PDK_HOME"),
            Some(&"/opt/pdk".to_string())
        );
        assert_eq!(loaded.recent_files.len(), 1);
    }

    #[test]
    fn test_load_nonexistent_returns_default() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("nonexistent.json");

        let loaded = PdkConfig::load_from(&config_path).unwrap();

        assert!(loaded.is_empty());
    }

    #[test]
    fn test_save_creates_parent_directories() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir
            .path()
            .join("nested")
            .join("dir")
            .join("config.json");

        let config = PdkConfig::new();
        config.save_to(&config_path).unwrap();

        assert!(config_path.exists());
    }

    #[test]
    fn test_config_error_display() {
        let io_err = ConfigError::Io("file not found".to_string());
        assert_eq!(io_err.to_string(), "I/O error: file not found");

        let parse_err = ConfigError::Parse("invalid json".to_string());
        assert_eq!(parse_err.to_string(), "Parse error: invalid json");

        let ser_err = ConfigError::Serialize("cannot serialize".to_string());
        assert_eq!(ser_err.to_string(), "Serialize error: cannot serialize");
    }

    // =========================================================================
    // Edge Case Tests
    // =========================================================================

    #[test]
    fn test_expand_path_prevents_infinite_recursion() {
        let mut config = PdkConfig::new();
        // Create a circular reference
        config.set_env_var("A", "$B");
        config.set_env_var("B", "$A");

        // Should not hang - will stop after MAX_ITERATIONS
        let result = config.expand_path("$A");
        // Result will be either $A or $B depending on iteration count
        assert!(result.contains('$'));
    }

    #[test]
    fn test_total_file_count() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("a.lib"), "* A").unwrap();
        fs::write(temp_dir.path().join("b.lib"), "* B").unwrap();

        let mut config = PdkConfig::new();
        config.add_library_path(temp_dir.path().to_string_lossy().to_string());
        config.discover_model_files();

        assert_eq!(config.total_file_count(), 2);
    }

    #[test]
    fn test_is_empty() {
        let config = PdkConfig::new();
        assert!(config.is_empty());

        let mut config2 = PdkConfig::new();
        config2.add_library_path("/test");
        assert!(!config2.is_empty());
    }

    #[test]
    fn test_all_model_extensions() {
        let temp_dir = TempDir::new().unwrap();

        // Create files with all supported extensions
        for ext in MODEL_FILE_EXTENSIONS {
            fs::write(temp_dir.path().join(format!("model.{}", ext)), "* Model").unwrap();
        }
        // Also create an unsupported file
        fs::write(temp_dir.path().join("readme.md"), "# Readme").unwrap();

        let mut config = PdkConfig::new();
        config.add_library_path(temp_dir.path().to_string_lossy().to_string());
        let files = config.discover_model_files();

        // Should find all model files but not readme.md
        assert_eq!(files.len(), MODEL_FILE_EXTENSIONS.len());
    }

    #[test]
    fn test_discovered_file_equality() {
        let df1 = DiscoveredFile {
            path: PathBuf::from("/a.lib"),
            extension: "lib".to_string(),
            size: 100,
            source_path: PathBuf::from("/"),
            sections: vec!["TT".to_string()],
        };

        let df2 = DiscoveredFile {
            path: PathBuf::from("/a.lib"),
            extension: "lib".to_string(),
            size: 100,
            source_path: PathBuf::from("/"),
            sections: vec!["TT".to_string()],
        };

        assert_eq!(df1, df2);
    }

    #[test]
    fn test_config_serialization_round_trip() {
        let mut config = PdkConfig::new();
        config.add_library_path_entry(
            LibraryPathEntry::new("/path/to/pdk")
                .with_label("Test PDK")
                .with_recursive(false)
                .with_enabled(true),
        );
        config.set_env_var("PDK_HOME", "/opt/pdk");
        config.set_env_var("VERSION", "1.0");
        config.add_recent_file("/recent/a.lib");
        config.add_recent_file("/recent/b.lib");
        config.max_recent_files = 10;

        let json = serde_json::to_string(&config).unwrap();
        let loaded: PdkConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(loaded.library_paths.len(), 1);
        assert_eq!(loaded.library_paths[0].path, "/path/to/pdk");
        assert_eq!(loaded.library_paths[0].label, Some("Test PDK".to_string()));
        assert!(!loaded.library_paths[0].recursive);
        assert_eq!(loaded.environment_variables.len(), 2);
        assert_eq!(loaded.recent_files.len(), 2);
        assert_eq!(loaded.max_recent_files, 10);
    }
}
