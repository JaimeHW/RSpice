use serde::{Deserialize, Serialize};
use std::path::Path;

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
