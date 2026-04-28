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
