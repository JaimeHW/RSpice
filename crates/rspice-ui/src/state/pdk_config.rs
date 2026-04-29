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
use std::path::PathBuf;

mod accessors;
mod discovered_file;
mod discovery;
mod errors;
mod library_path;
mod paths;
mod persistence;
mod recent;

pub use discovered_file::DiscoveredFile;
pub use errors::ConfigError;
pub use library_path::LibraryPathEntry;
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
}
