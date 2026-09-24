//! A model file found under a configured library path.
//!
//! Keeps both the resolved path and the library entry it came from, so a
//! duplicate model can name which library shadowed which.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

    /// Get file type (alias for extension for dialog compatibility)
    pub fn file_type(&self) -> &str {
        &self.extension
    }

    /// Get path as string (for dialog display)
    pub fn path_str(&self) -> String {
        self.path.to_string_lossy().to_string()
    }
}
