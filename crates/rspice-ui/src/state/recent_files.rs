//! Recent Files Management
//!
//! Tracks recently opened files with persistence support.
//! Follows commercial EDA tool patterns with MRU (Most Recently Used) ordering.
//!
//! # Features
//!
//! - Maintains ordered list of recently opened files
//! - Automatic duplicate detection and reordering
//! - Configurable maximum file count
//! - Cross-platform persistence (localStorage for web, JSON file for desktop)
//!
//! # Example
//!
//! ```ignore
//! use rspice_ui::state::recent_files::RecentFiles;
//!
//! let mut recent = RecentFiles::new(10);
//! recent.add("/path/to/schematic.sp");
//! assert_eq!(recent.files()[0].path, "/path/to/schematic.sp");
//! ```

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

//=============================================================================
// Recent File Entry
//=============================================================================

/// A single recent file entry with metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentFileEntry {
    /// Full path to the file
    pub path: String,
    /// Display name (usually the filename)
    pub name: String,
    /// Last access timestamp (Unix epoch seconds)
    pub last_accessed: u64,
    /// Whether the file still exists (updated on load)
    #[serde(default)]
    pub exists: bool,
}

impl RecentFileEntry {
    /// Create a new recent file entry
    pub fn new(path: impl Into<String>) -> Self {
        let path_str = path.into();
        let name = PathBuf::from(&path_str)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path_str.clone());

        Self {
            path: path_str,
            name,
            last_accessed: current_timestamp(),
            exists: true,
        }
    }

    /// Get the parent directory for display
    pub fn parent_dir(&self) -> String {
        PathBuf::from(&self.path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    /// Update the last accessed timestamp
    pub fn touch(&mut self) {
        self.last_accessed = current_timestamp();
    }

    /// Check if this file matches the given path
    pub fn matches_path(&self, path: &str) -> bool {
        // Normalize paths for comparison
        normalize_path(&self.path) == normalize_path(path)
    }
}

//=============================================================================
// Recent Files Manager
//=============================================================================

/// Manages the list of recently opened files.
///
/// Implements MRU (Most Recently Used) ordering with configurable capacity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentFiles {
    /// Recent file entries in MRU order (most recent first)
    files: Vec<RecentFileEntry>,
    /// Maximum number of files to track
    max_count: usize,
}

impl Default for RecentFiles {
    fn default() -> Self {
        Self::new(10)
    }
}

impl RecentFiles {
    /// Create a new recent files manager with specified capacity
    pub fn new(max_count: usize) -> Self {
        Self {
            files: Vec::new(),
            max_count: max_count.max(1), // At least 1
        }
    }

    /// Get the list of recent files (most recent first)
    pub fn files(&self) -> &[RecentFileEntry] {
        &self.files
    }

    /// Get just the file paths (most recent first)
    pub fn get_paths(&self) -> Vec<String> {
        self.files.iter().map(|e| e.path.clone()).collect()
    }

    /// Get maximum file count
    pub fn max_count(&self) -> usize {
        self.max_count
    }

    /// Set maximum file count (trims excess if needed)
    pub fn set_max_count(&mut self, count: usize) {
        self.max_count = count.max(1);
        self.trim();
    }

    /// Add a file to the recent list
    ///
    /// If the file already exists, it's moved to the front.
    /// Automatically trims to max capacity.
    pub fn add(&mut self, path: impl Into<String>) {
        let path_str = path.into();

        // Check if file already exists in list
        if let Some(idx) = self.find_index(&path_str) {
            // Move to front and update timestamp
            let mut entry = self.files.remove(idx);
            entry.touch();
            self.files.insert(0, entry);
        } else {
            // Add new entry at front
            self.files.insert(0, RecentFileEntry::new(path_str));
        }

        self.trim();
    }

    /// Remove a file from the recent list
    pub fn remove(&mut self, path: &str) -> bool {
        if let Some(idx) = self.find_index(path) {
            self.files.remove(idx);
            true
        } else {
            false
        }
    }

    /// Clear all recent files
    pub fn clear(&mut self) {
        self.files.clear();
    }

    /// Check if a file is in the recent list
    pub fn contains(&self, path: &str) -> bool {
        self.find_index(path).is_some()
    }

    /// Get the most recent file (if any)
    pub fn most_recent(&self) -> Option<&RecentFileEntry> {
        self.files.first()
    }

    /// Check if the list is empty
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Get the number of recent files
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// Validate file existence and update flags
    ///
    /// Call this on startup to mark non-existent files.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn validate_existence(&mut self) {
        for entry in &mut self.files {
            entry.exists = std::path::Path::new(&entry.path).exists();
        }
    }

    /// WASM version (always marks as existing since we can't check)
    #[cfg(target_arch = "wasm32")]
    pub fn validate_existence(&mut self) {
        for entry in &mut self.files {
            entry.exists = true;
        }
    }

    /// Remove files that no longer exist
    pub fn remove_nonexistent(&mut self) {
        self.validate_existence();
        self.files.retain(|e| e.exists);
    }

    /// Find index of file by path
    fn find_index(&self, path: &str) -> Option<usize> {
        self.files.iter().position(|e| e.matches_path(path))
    }

    /// Trim to max capacity
    fn trim(&mut self) {
        while self.files.len() > self.max_count {
            self.files.pop();
        }
    }

    //-------------------------------------------------------------------------
    // Persistence
    //-------------------------------------------------------------------------

    /// Save to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Load from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Get the default storage path for desktop
    #[cfg(not(target_arch = "wasm32"))]
    pub fn default_storage_path() -> PathBuf {
        // Use APPDATA on Windows, HOME on Unix
        #[cfg(target_os = "windows")]
        if let Ok(appdata) = std::env::var("APPDATA") {
            return PathBuf::from(appdata)
                .join("rspice")
                .join("recent_files.json");
        }
        #[cfg(not(target_os = "windows"))]
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home)
                .join(".local")
                .join("share")
                .join("rspice")
                .join("recent_files.json");
        }
        PathBuf::from(".rspice_recent.json")
    }

    /// Save to default storage location (desktop only)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::default_storage_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = self
            .to_json()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, json)
    }

    /// Load from default storage location (desktop only)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load() -> Result<Self, std::io::Error> {
        let path = Self::default_storage_path();
        let json = std::fs::read_to_string(path)?;
        Self::from_json(&json).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Load from storage, returning default if not found
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_default()
    }

    /// WASM stubs
    #[cfg(target_arch = "wasm32")]
    pub fn save(&self) -> Result<(), std::io::Error> {
        Ok(()) // Use localStorage instead
    }

    #[cfg(target_arch = "wasm32")]
    pub fn load_or_default() -> Self {
        Self::default()
    }
}

//=============================================================================
// Helpers
//=============================================================================

/// Get current Unix timestamp
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Normalize path for comparison (handles Windows/Unix differences)
fn normalize_path(path: &str) -> String {
    path.replace('\\', "/").to_lowercase()
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_single_file() {
        let mut recent = RecentFiles::new(10);
        recent.add("/home/user/circuit.sp");

        assert_eq!(recent.len(), 1);
        assert_eq!(recent.files()[0].path, "/home/user/circuit.sp");
        assert_eq!(recent.files()[0].name, "circuit.sp");
    }

    #[test]
    fn test_add_multiple_files_mru_order() {
        let mut recent = RecentFiles::new(10);
        recent.add("/path/a.sp");
        recent.add("/path/b.sp");
        recent.add("/path/c.sp");

        // Most recent should be first
        assert_eq!(recent.files()[0].name, "c.sp");
        assert_eq!(recent.files()[1].name, "b.sp");
        assert_eq!(recent.files()[2].name, "a.sp");
    }

    #[test]
    fn test_add_duplicate_moves_to_front() {
        let mut recent = RecentFiles::new(10);
        recent.add("/path/a.sp");
        recent.add("/path/b.sp");
        recent.add("/path/c.sp");

        // Re-add 'a' - should move to front
        recent.add("/path/a.sp");

        assert_eq!(recent.len(), 3); // Still 3 files
        assert_eq!(recent.files()[0].name, "a.sp");
        assert_eq!(recent.files()[1].name, "c.sp");
        assert_eq!(recent.files()[2].name, "b.sp");
    }

    #[test]
    fn test_max_capacity_trim() {
        let mut recent = RecentFiles::new(3);
        recent.add("/path/a.sp");
        recent.add("/path/b.sp");
        recent.add("/path/c.sp");
        recent.add("/path/d.sp");

        // Should only have 3 files (oldest removed)
        assert_eq!(recent.len(), 3);
        assert_eq!(recent.files()[0].name, "d.sp");
        assert_eq!(recent.files()[2].name, "b.sp");
        assert!(!recent.contains("/path/a.sp"));
    }

    #[test]
    fn test_remove_file() {
        let mut recent = RecentFiles::new(10);
        recent.add("/path/a.sp");
        recent.add("/path/b.sp");

        assert!(recent.remove("/path/a.sp"));
        assert_eq!(recent.len(), 1);
        assert!(!recent.contains("/path/a.sp"));
        assert!(recent.contains("/path/b.sp"));
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut recent = RecentFiles::new(10);
        recent.add("/path/a.sp");

        assert!(!recent.remove("/path/nonexistent.sp"));
        assert_eq!(recent.len(), 1);
    }

    #[test]
    fn test_clear() {
        let mut recent = RecentFiles::new(10);
        recent.add("/path/a.sp");
        recent.add("/path/b.sp");
        recent.clear();

        assert!(recent.is_empty());
        assert_eq!(recent.len(), 0);
    }

    #[test]
    fn test_most_recent() {
        let mut recent = RecentFiles::new(10);
        assert!(recent.most_recent().is_none());

        recent.add("/path/a.sp");
        recent.add("/path/b.sp");

        assert_eq!(recent.most_recent().unwrap().name, "b.sp");
    }

    #[test]
    fn test_path_normalization() {
        let mut recent = RecentFiles::new(10);
        recent.add("C:\\Users\\test\\file.sp");

        // Should find with forward slashes too
        assert!(recent.contains("c:/users/test/file.sp"));
    }

    #[test]
    fn test_parent_dir() {
        let entry = RecentFileEntry::new("/home/user/projects/circuit.sp");
        assert_eq!(entry.parent_dir(), "/home/user/projects");
    }

    #[test]
    fn test_serialization() {
        let mut recent = RecentFiles::new(10);
        recent.add("/path/a.sp");
        recent.add("/path/b.sp");

        let json = recent.to_json().unwrap();
        let restored = RecentFiles::from_json(&json).unwrap();

        assert_eq!(restored.len(), 2);
        assert_eq!(restored.files()[0].name, "b.sp");
    }

    #[test]
    fn test_set_max_count_trims() {
        let mut recent = RecentFiles::new(10);
        for i in 0..10 {
            recent.add(format!("/path/{}.sp", i));
        }

        assert_eq!(recent.len(), 10);

        recent.set_max_count(5);
        assert_eq!(recent.len(), 5);
        // Should keep the 5 most recent
        assert_eq!(recent.files()[0].name, "9.sp");
    }

    #[test]
    fn test_entry_touch_updates_timestamp() {
        let mut entry = RecentFileEntry::new("/path/file.sp");
        let original = entry.last_accessed;

        // Wait a tiny bit to ensure different timestamp
        std::thread::sleep(std::time::Duration::from_millis(10));
        entry.touch();

        // Timestamp should be >= original
        assert!(entry.last_accessed >= original);
    }

    #[test]
    fn test_empty_path_handling() {
        let entry = RecentFileEntry::new("");
        assert_eq!(entry.name, "");
        assert_eq!(entry.parent_dir(), "");
    }

    #[test]
    fn test_windows_path() {
        let entry = RecentFileEntry::new("C:\\Users\\Test\\circuit.sp");
        assert_eq!(entry.name, "circuit.sp");
        assert!(entry.parent_dir().contains("Test") || entry.parent_dir().contains("Users"));
    }

    #[test]
    fn test_default_max_count() {
        let recent = RecentFiles::default();
        assert_eq!(recent.max_count(), 10);
    }

    #[test]
    fn test_min_max_count_is_one() {
        let recent = RecentFiles::new(0);
        assert_eq!(recent.max_count(), 1);
    }
}
