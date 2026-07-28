//! One entry in the library search path.

use serde::{Deserialize, Serialize};

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

}

impl Default for LibraryPathEntry {
    fn default() -> Self {
        Self::new("")
    }
}
