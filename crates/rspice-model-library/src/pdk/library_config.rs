//! Portable PDK library search paths and model-file discovery records.
//!
//! The host supplies filesystem metadata; these records retain the exact scan result.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    /// Get the file name without path
    pub fn file_name(&self) -> &str {
        self.path.file_name().and_then(|n| n.to_str()).unwrap_or("")
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
