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
