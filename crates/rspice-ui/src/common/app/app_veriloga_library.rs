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
    crate::io::durable_file::reconcile_publication(path).ok()?;
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

    // The periodic autosave calls this every ~30 s; skip the disk write
    // when nothing changed since the last save on this thread.
    thread_local! {
        static LAST_SAVED: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    }
    let persisted = PersistedVerilogALibraryRef {
        version: VERILOGA_LIBRARY_FORMAT_VERSION,
        library,
    };
    let json = serde_json::to_string_pretty(&persisted)
        .map_err(|e| format!("failed to serialize Verilog-A library: {}", e))?;
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

    let expected = crate::io::durable_file::observe_expected_content(path).map_err(|error| {
        format!(
            "failed to authorize Verilog-A library destination '{}': {error}",
            path.display()
        )
    })?;
    publish_global_veriloga_library(path, expected, json.as_bytes())?;
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
fn publish_global_veriloga_library(
    path: &Path,
    expected: crate::io::durable_file::ExpectedContent,
    bytes: &[u8],
) -> Result<(), String> {
    crate::io::durable_file::compare_exchange_bytes(path, expected, bytes).map_err(|error| {
        format!(
            "failed to publish Verilog-A library '{}': {error}",
            path.display()
        )
    })
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn publication_rejects_late_external_change_without_losing_it() {
        let root = unique_temp_dir("late-change");
        let path = root.join(VERILOGA_LIBRARY_CONFIG_FILE);
        std::fs::write(&path, b"authorized predecessor").expect("write predecessor");
        let expected =
            crate::io::durable_file::observe_expected_content(&path).expect("observe destination");
        let library = Library::new(VERILOGA_LIBRARY_NAME).with_technology("test");
        let bytes = serde_json::to_vec_pretty(&PersistedVerilogALibraryRef {
            version: VERILOGA_LIBRARY_FORMAT_VERSION,
            library: &library,
        })
        .expect("serialize library");
        std::fs::write(&path, b"late external edit").expect("race destination");

        let result = publish_global_veriloga_library(&path, expected, &bytes);

        assert!(result.is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"late external edit");
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn save_and_load_round_trip_through_durable_publication() {
        let root = unique_temp_dir("round-trip");
        let path = root.join(VERILOGA_LIBRARY_CONFIG_FILE);
        let library = Library::new("source-name").with_technology("rf-cmos");

        save_global_veriloga_library_to_path(&path, &library).expect("save library");
        let loaded = load_global_veriloga_library_from_path(&path).expect("load library");

        assert_eq!(loaded.technology, "rf-cmos");
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    fn unique_temp_dir(label: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "rspice-veriloga-library-{label}-{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&root).expect("create fixture");
        root
    }
}
