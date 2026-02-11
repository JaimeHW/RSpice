use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::state::{Library, LibraryManager};

pub(super) const VERILOGA_LIBRARY_NAME: &str = "veriloga";
const VERILOGA_LIBRARY_CONFIG_FILE: &str = "veriloga_library.json";
const VERILOGA_LIBRARY_FORMAT_VERSION: u32 = 1;

#[derive(serde::Serialize, serde::Deserialize)]
struct PersistedVerilogALibrary {
    version: u32,
    library: Library,
}

pub(super) fn restore_global_veriloga_library(library_manager: &mut LibraryManager) {
    let Some(library) = load_global_veriloga_library() else {
        return;
    };
    install_loaded_veriloga_library(library_manager, library);
}

pub(super) fn save_global_veriloga_library(library_manager: &LibraryManager) -> Result<(), String> {
    let Some(library) = library_manager.get_library(VERILOGA_LIBRARY_NAME) else {
        return Ok(());
    };

    save_global_veriloga_library_to_path(&global_veriloga_library_path(), library)
}

fn global_veriloga_library_path() -> PathBuf {
    let config_root = dirs::config_dir()
        .or_else(dirs::home_dir)
        .unwrap_or_else(|| PathBuf::from("."));
    global_veriloga_library_path_for_root(&config_root)
}

fn global_veriloga_library_path_for_root(config_root: &Path) -> PathBuf {
    config_root
        .join("rspice")
        .join(VERILOGA_LIBRARY_CONFIG_FILE)
}

fn load_global_veriloga_library() -> Option<Library> {
    load_global_veriloga_library_from_path(&global_veriloga_library_path())
}

fn load_global_veriloga_library_from_path(path: &Path) -> Option<Library> {
    let text = std::fs::read_to_string(path).ok()?;
    let parsed: PersistedVerilogALibrary = serde_json::from_str(&text).ok()?;
    if parsed.version != VERILOGA_LIBRARY_FORMAT_VERSION {
        return None;
    }
    Some(parsed.library)
}

fn save_global_veriloga_library_to_path(path: &Path, library: &Library) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            format!(
                "failed to create Verilog-A library directory '{}': {}",
                parent.display(),
                e
            )
        })?;
    }

    let persisted = PersistedVerilogALibrary {
        version: VERILOGA_LIBRARY_FORMAT_VERSION,
        library: library.clone(),
    };
    let json = serde_json::to_string_pretty(&persisted)
        .map_err(|e| format!("failed to serialize Verilog-A library: {}", e))?;
    write_file_atomically(path, json.as_bytes())
}

fn install_loaded_veriloga_library(library_manager: &mut LibraryManager, mut library: Library) {
    library.name = VERILOGA_LIBRARY_NAME.to_string();
    library.read_only = false;
    library_manager.add_library(library);
}

fn write_file_atomically(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let temp_path = temporary_path_for(path);
    std::fs::write(&temp_path, bytes).map_err(|e| {
        format!(
            "failed to write temporary Verilog-A library file '{}': {}",
            temp_path.display(),
            e
        )
    })?;

    match std::fs::rename(&temp_path, path) {
        Ok(()) => Ok(()),
        Err(first_err) => {
            if path.exists() {
                std::fs::remove_file(path).map_err(|remove_err| {
                    let _ = std::fs::remove_file(&temp_path);
                    format!(
                        "failed to replace existing Verilog-A library '{}': {} (rename error: {})",
                        path.display(),
                        remove_err,
                        first_err
                    )
                })?;

                std::fs::rename(&temp_path, path).map_err(|second_err| {
                    let _ = std::fs::remove_file(&temp_path);
                    format!(
                        "failed to finalize Verilog-A library update '{}': {}",
                        path.display(),
                        second_err
                    )
                })?;
                Ok(())
            } else {
                let _ = std::fs::remove_file(&temp_path);
                Err(format!(
                    "failed to move Verilog-A library into place '{}': {}",
                    path.display(),
                    first_err
                ))
            }
        }
    }
}

fn temporary_path_for(path: &Path) -> PathBuf {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(VERILOGA_LIBRARY_CONFIG_FILE);
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    parent.join(format!(
        ".{}.{}.{}.tmp",
        file_name,
        std::process::id(),
        nonce
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Cell, View, ViewType};

    fn sample_library(name: &str) -> Library {
        let mut library = Library::new(name);
        library.technology = "test-tech".to_string();
        library.read_only = true;
        library
            .metadata
            .insert("owner".to_string(), "unit-test".to_string());

        let mut cell = Cell::new("amp");
        cell.description = "amplifier".to_string();
        cell.add_view(View::new("veriloga", ViewType::VerilogA));
        library.add_cell(cell);
        library
    }

    #[test]
    fn test_global_veriloga_library_path_for_root() {
        let root = PathBuf::from("config-root");
        let path = global_veriloga_library_path_for_root(&root);
        assert!(path.ends_with(Path::new("rspice").join(VERILOGA_LIBRARY_CONFIG_FILE)));
    }

    #[test]
    fn test_load_library_from_missing_path_returns_none() {
        let temp = tempfile::tempdir().expect("tempdir");
        let path = temp.path().join("missing.json");
        assert!(load_global_veriloga_library_from_path(&path).is_none());
    }

    #[test]
    fn test_load_library_from_invalid_json_returns_none() {
        let temp = tempfile::tempdir().expect("tempdir");
        let path = temp.path().join("broken.json");
        std::fs::write(&path, "{ invalid json }").expect("write invalid json");
        assert!(load_global_veriloga_library_from_path(&path).is_none());
    }

    #[test]
    fn test_load_library_rejects_wrong_version() {
        let temp = tempfile::tempdir().expect("tempdir");
        let path = temp.path().join("version-mismatch.json");
        let persisted = PersistedVerilogALibrary {
            version: VERILOGA_LIBRARY_FORMAT_VERSION + 1,
            library: sample_library("test"),
        };
        let text = serde_json::to_string(&persisted).expect("serialize");
        std::fs::write(&path, text).expect("write persisted json");
        assert!(load_global_veriloga_library_from_path(&path).is_none());
    }

    #[test]
    fn test_save_and_load_library_round_trip() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path().join("cfg");
        let path = global_veriloga_library_path_for_root(&root);
        let library = sample_library("custom-name");

        save_global_veriloga_library_to_path(&path, &library).expect("save library");
        let loaded = load_global_veriloga_library_from_path(&path).expect("load library");

        assert_eq!(loaded.name, "custom-name");
        assert_eq!(loaded.technology, "test-tech");
        assert_eq!(loaded.cells.len(), 1);
        assert_eq!(loaded.get_cell("amp").map(|cell| cell.views.len()), Some(1));
        assert_eq!(loaded.metadata.get("owner"), Some(&"unit-test".to_string()));
    }

    #[test]
    fn test_save_replaces_existing_file_contents() {
        let temp = tempfile::tempdir().expect("tempdir");
        let path = temp.path().join("veriloga.json");
        std::fs::write(&path, "stale-data").expect("seed stale file");

        let library = sample_library("replacement");
        save_global_veriloga_library_to_path(&path, &library).expect("save replacement");

        let loaded = load_global_veriloga_library_from_path(&path).expect("load replacement");
        assert_eq!(loaded.name, "replacement");
    }

    #[test]
    fn test_install_loaded_library_normalizes_name_and_writable() {
        let mut manager = LibraryManager::new();
        let library = sample_library("foreign-name");
        install_loaded_veriloga_library(&mut manager, library);

        let restored = manager
            .get_library(VERILOGA_LIBRARY_NAME)
            .expect("normalized library present");
        assert_eq!(restored.name, VERILOGA_LIBRARY_NAME);
        assert!(!restored.read_only);
    }

    #[test]
    fn test_save_global_library_noop_when_missing_user_library() {
        let manager = LibraryManager::new();
        assert!(save_global_veriloga_library(&manager).is_ok());
    }
}
