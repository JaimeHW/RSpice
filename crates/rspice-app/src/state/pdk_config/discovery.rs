//! Scanning library paths for model files.
//!
//! Walks the configured paths, records what was found, and indexes the
//! result by extension so a lookup does not rescan the filesystem.

use std::path::{Path, PathBuf};

use super::*;

pub fn discover_model_files(config: &mut PdkConfig) -> &[DiscoveredFile] {
    config.discovered_files.clear();
    config.scan_errors.clear();

    let now = crate::time_compat::unix_epoch().as_secs();

    // Pre-compute expanded paths to avoid borrow conflicts
    let entry_data: Vec<(usize, String, PathBuf, bool)> = config
        .library_paths
        .iter()
        .enumerate()
        .filter(|(_, entry)| entry.enabled)
        .map(|(idx, entry)| {
            let expanded = super::expand_path(config, &entry.path);
            let path_buf = PathBuf::from(&expanded);
            (idx, expanded, path_buf, entry.recursive)
        })
        .collect();

    // Now process each entry
    for (idx, expanded_path, path, recursive) in entry_data {
        if !path.exists() {
            let original_path = &config.library_paths[idx].path;
            config.scan_errors.push(format!(
                "Path does not exist: {} (expanded from {})",
                expanded_path, original_path
            ));
            continue;
        }

        let mut files = Vec::new();
        let max_depth = if recursive { MAX_SCAN_DEPTH } else { 0 };

        if let Err(e) = scan_directory_recursive(&path, &path, max_depth, 0, &mut files) {
            config
                .scan_errors
                .push(format!("Error scanning {}: {}", expanded_path, e));
        }

        // Update entry metadata
        config.library_paths[idx].file_count = files.len();
        config.library_paths[idx].last_scanned = Some(now);
        config.discovered_files.extend(files);
    }

    // Sort by path for consistent ordering
    config.discovered_files.sort_by(|a, b| a.path.cmp(&b.path));

    &config.discovered_files
}

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
                    results.push(discovered_file(path, base_path.to_path_buf()));
                }
            }
        } else if file_type.is_dir() {
            // Skip hidden directories
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if !name_str.starts_with('.') {
                scan_directory_recursive(base_path, &path, max_depth, current_depth + 1, results)?;
            }
        }
    }

    Ok(())
}

pub(crate) fn discovered_file(path: PathBuf, source_path: PathBuf) -> DiscoveredFile {
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);

    DiscoveredFile {
        path,
        extension,
        size,
        source_path,
        sections: Vec::new(),
    }
}
