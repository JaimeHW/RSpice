use std::path::PathBuf;

use super::*;

impl PdkConfig {
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
}
