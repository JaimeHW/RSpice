use std::path::{Path, PathBuf};

use super::*;

impl PdkConfig {
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
}
