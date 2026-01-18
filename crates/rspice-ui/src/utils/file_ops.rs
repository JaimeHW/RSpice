//! Platform-specific file operations
//!
//! Provides cross-platform file dialog abstractions.
//! - Desktop: Uses rfd for native file dialogs  
//! - Web: Stub implementations (file operations not yet supported)

use std::path::PathBuf;

/// Result type for file operations
pub type FileResult<T> = Result<T, FileError>;

/// File operation errors
#[derive(Debug, Clone)]
pub enum FileError {
    /// User cancelled the operation
    Cancelled,
    /// Operation not supported on this platform  
    NotSupported,
    /// IO error
    Io(String),
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::Cancelled => write!(f, "Operation cancelled"),
            FileError::NotSupported => write!(f, "Not supported on this platform"),
            FileError::Io(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

// ============================================================================
// Desktop implementation using rfd
// ============================================================================

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
pub mod native {
    use super::*;

    /// Open file dialog and return selected path
    pub async fn pick_file(filters: &[(&str, &[&str])]) -> FileResult<PathBuf> {
        let mut dialog = rfd::AsyncFileDialog::new();
        for (name, extensions) in filters {
            dialog = dialog.add_filter(*name, extensions);
        }

        dialog
            .pick_file()
            .await
            .map(|f| f.path().to_path_buf())
            .ok_or(FileError::Cancelled)
    }

    /// Save file dialog and return selected path  
    pub async fn save_file(filters: &[(&str, &[&str])], default_name: &str) -> FileResult<PathBuf> {
        let mut dialog = rfd::AsyncFileDialog::new().set_file_name(default_name);
        for (name, extensions) in filters {
            dialog = dialog.add_filter(*name, extensions);
        }

        dialog
            .save_file()
            .await
            .map(|f| f.path().to_path_buf())
            .ok_or(FileError::Cancelled)
    }

    /// Read file contents
    pub fn read_file(path: &PathBuf) -> FileResult<String> {
        std::fs::read_to_string(path).map_err(|e| FileError::Io(e.to_string()))
    }

    /// Write file contents
    pub fn write_file(path: &PathBuf, content: &str) -> FileResult<()> {
        std::fs::write(path, content).map_err(|e| FileError::Io(e.to_string()))
    }
}

// ============================================================================
// Web stub implementation
// ============================================================================

#[cfg(target_arch = "wasm32")]
pub mod native {
    use super::*;

    /// Open file dialog - not yet implemented for web
    pub async fn pick_file(_filters: &[(&str, &[&str])]) -> FileResult<PathBuf> {
        log::warn!("File picker not yet implemented for web");
        Err(FileError::NotSupported)
    }

    /// Save file dialog - not yet implemented for web  
    pub async fn save_file(
        _filters: &[(&str, &[&str])],
        _default_name: &str,
    ) -> FileResult<PathBuf> {
        log::warn!("File save dialog not yet implemented for web");
        Err(FileError::NotSupported)
    }

    /// Read file - not supported on web without File API
    pub fn read_file(_path: &PathBuf) -> FileResult<String> {
        Err(FileError::NotSupported)
    }

    /// Write file - not supported on web
    pub fn write_file(_path: &PathBuf, _content: &str) -> FileResult<()> {
        Err(FileError::NotSupported)
    }
}

// Fallback for non-desktop, non-wasm builds
#[cfg(all(not(target_arch = "wasm32"), not(feature = "desktop")))]
pub mod native {
    use super::*;

    pub async fn pick_file(_filters: &[(&str, &[&str])]) -> FileResult<PathBuf> {
        Err(FileError::NotSupported)
    }

    pub async fn save_file(
        _filters: &[(&str, &[&str])],
        _default_name: &str,
    ) -> FileResult<PathBuf> {
        Err(FileError::NotSupported)
    }

    pub fn read_file(_path: &PathBuf) -> FileResult<String> {
        Err(FileError::NotSupported)
    }

    pub fn write_file(_path: &PathBuf, _content: &str) -> FileResult<()> {
        Err(FileError::NotSupported)
    }
}

// Re-export for convenience
pub use native::*;
