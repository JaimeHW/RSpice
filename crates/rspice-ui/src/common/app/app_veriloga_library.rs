//! Global Verilog-A library persistence. Native builds keep the library
//! as JSON under the user config directory; the browser build has no
//! filesystem (and the atomic-write temp naming reaches
//! `std::process::id`, which aborts on wasm32), so there the library
//! simply lives in memory for the session and both entry points are
//! quiet no-ops.

#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};

#[cfg(not(target_arch = "wasm32"))]
use crate::state::Library;
use crate::state::LibraryManager;

pub(super) const VERILOGA_LIBRARY_NAME: &str = "veriloga";
#[cfg(not(target_arch = "wasm32"))]
const VERILOGA_LIBRARY_CONFIG_FILE: &str = "veriloga_library.json";
#[cfg(not(target_arch = "wasm32"))]
const VERILOGA_LIBRARY_FORMAT_VERSION: u32 = 1;

#[cfg(not(target_arch = "wasm32"))]
#[derive(serde::Deserialize)]
struct PersistedVerilogALibrary {
    version: u32,
    library: Library,
}

/// Borrowed twin of [`PersistedVerilogALibrary`] so saving never clones
/// the library.
#[cfg(not(target_arch = "wasm32"))]
#[derive(serde::Serialize)]
struct PersistedVerilogALibraryRef<'a> {
    version: u32,
    library: &'a Library,
}

pub(super) fn restore_global_veriloga_library(library_manager: &mut LibraryManager) {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = library_manager;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let Some(library) = load_global_veriloga_library() else {
            return;
        };
        install_loaded_veriloga_library(library_manager, library);
    }
}

pub(super) fn save_global_veriloga_library(library_manager: &LibraryManager) -> Result<(), String> {
    let Some(library) = library_manager.get_library(VERILOGA_LIBRARY_NAME) else {
        return Ok(());
    };

    // The ~30 s autosave also lands here: the wasm no-op must stay quiet.
    #[cfg(target_arch = "wasm32")]
    {
        let _ = library;
        Ok(())
    }
    #[cfg(not(target_arch = "wasm32"))]
    save_global_veriloga_library_to_path(&global_veriloga_library_path(), library)
}

#[cfg(not(target_arch = "wasm32"))]
fn global_veriloga_library_path() -> PathBuf {
    let config_root = dirs::config_dir()
        .or_else(dirs::home_dir)
        .unwrap_or_else(|| PathBuf::from("."));
    global_veriloga_library_path_for_root(&config_root)
}

#[cfg(not(target_arch = "wasm32"))]
fn global_veriloga_library_path_for_root(config_root: &Path) -> PathBuf {
    config_root
        .join("rspice")
        .join(VERILOGA_LIBRARY_CONFIG_FILE)
}

#[cfg(not(target_arch = "wasm32"))]
fn load_global_veriloga_library() -> Option<Library> {
    load_global_veriloga_library_from_path(&global_veriloga_library_path())
}

#[cfg(not(target_arch = "wasm32"))]
fn load_global_veriloga_library_from_path(path: &Path) -> Option<Library> {
    let text = std::fs::read_to_string(path).ok()?;
    let parsed: PersistedVerilogALibrary = serde_json::from_str(&text).ok()?;
    if parsed.version != VERILOGA_LIBRARY_FORMAT_VERSION {
        return None;
    }
    Some(parsed.library)
}

#[cfg(not(target_arch = "wasm32"))]
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

    let persisted = PersistedVerilogALibraryRef {
        version: VERILOGA_LIBRARY_FORMAT_VERSION,
        library,
    };
    let json = serde_json::to_string_pretty(&persisted)
        .map_err(|e| format!("failed to serialize Verilog-A library: {}", e))?;

    // The periodic autosave calls this every ~30 s; skip the disk write
    // when nothing changed since the last save on this thread.
    thread_local! {
        static LAST_SAVED: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    }
    let hash = {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        json.hash(&mut hasher);
        path.hash(&mut hasher);
        hasher.finish()
    };
    if LAST_SAVED.with(|last| last.get()) == hash {
        return Ok(());
    }

    write_file_atomically(path, json.as_bytes())?;
    LAST_SAVED.with(|last| last.set(hash));
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn install_loaded_veriloga_library(library_manager: &mut LibraryManager, mut library: Library) {
    library.name = VERILOGA_LIBRARY_NAME.to_string();
    library.read_only = false;
    library_manager.add_library(library);
}

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
fn temporary_path_for(path: &Path) -> PathBuf {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(VERILOGA_LIBRARY_CONFIG_FILE);
    let nonce = crate::common::time_compat::unix_epoch().as_nanos();
    parent.join(format!(
        ".{}.{}.{}.tmp",
        file_name,
        std::process::id(),
        nonce
    ))
}
