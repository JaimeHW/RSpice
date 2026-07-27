//! Global Verilog-A library persistence.
//!
//! A global behavioral library is one atomic document: the Library/Cell/View
//! catalog and every exact source closure owned by one of its Verilog-A views.
//! Persisting only the catalog leaves apparently valid views that cannot be
//! compiled after restart, so format version 2 retains and validates both
//! halves before publication or installation.
//!
//! Native builds publish the document under the user configuration directory.
//! Browser builds already persist the complete [`crate::workbench::app::AppState`] through
//! eframe storage; their dedicated filesystem entry points therefore remain
//! quiet no-ops and never call APIs that abort on `wasm32`.

#[cfg(not(target_arch = "wasm32"))]
use std::collections::BTreeSet;
#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};

#[cfg(not(target_arch = "wasm32"))]
use crate::state::{
    CellViewRef, Library, LibraryManager, ProjectSourceLanguage, ProjectSourceOwner,
    ProjectSourceRegistry, ProjectWorkspace, ViewType,
};
#[cfg(target_arch = "wasm32")]
use crate::state::{LibraryManager, ProjectWorkspace};

pub(in crate::workbench::app) const VERILOGA_LIBRARY_NAME: &str = "veriloga";
#[cfg(not(target_arch = "wasm32"))]
const VERILOGA_LIBRARY_CONFIG_FILE: &str = "veriloga_library.json";
#[cfg(not(target_arch = "wasm32"))]
const VERILOGA_LIBRARY_FORMAT_VERSION: u32 = 2;
#[cfg(not(target_arch = "wasm32"))]
const LEGACY_VERILOGA_LIBRARY_FORMAT_VERSION: u32 = 1;

#[cfg(not(target_arch = "wasm32"))]
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedVerilogALibrary {
    version: u32,
    library: Library,
    sources: ProjectSourceRegistry,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct LegacyPersistedVerilogALibrary {
    version: u32,
    library: Library,
}

/// Borrowed representation keeps periodic persistence allocation-bounded and
/// guarantees the catalog and source registry are serialized together.
#[cfg(not(target_arch = "wasm32"))]
#[derive(serde::Serialize)]
#[serde(deny_unknown_fields)]
struct PersistedVerilogALibraryRef<'a> {
    version: u32,
    library: &'a Library,
    sources: &'a ProjectSourceRegistry,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
struct LoadedVerilogALibrary {
    library: Library,
    /// Version 1 retained only the library catalog. Its source closures can be
    /// migrated only when the recoverable application session still owns an
    /// exact matching closure; inventing source for a missing view is unsafe.
    sources: Option<ProjectSourceRegistry>,
}

pub(in crate::workbench::app) fn restore_global_veriloga_library(
    library_manager: &mut LibraryManager,
    workspace: &mut ProjectWorkspace,
) {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (library_manager, workspace);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let loaded = match load_global_veriloga_library() {
            Ok(Some(loaded)) => loaded,
            Ok(None) => return,
            Err(error) => {
                log::warn!("Failed to restore global Verilog-A library: {error}");
                return;
            }
        };
        if let Err(error) =
            install_loaded_veriloga_library(library_manager, &mut workspace.project_sources, loaded)
        {
            log::warn!("Failed to install global Verilog-A library: {error}");
        }
    }
}

pub(in crate::workbench::app) fn save_global_veriloga_library(
    library_manager: &LibraryManager,
    workspace: &ProjectWorkspace,
) -> Result<(), String> {
    let Some(library) = library_manager.get_library(VERILOGA_LIBRARY_NAME) else {
        return Ok(());
    };

    // eframe persists both fields as part of AppState in a browser. The
    // periodic autosave also lands here, so the wasm no-op must stay quiet.
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (library, workspace);
        Ok(())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let sources = global_sources_from_registry(&workspace.project_sources)?;
        save_global_veriloga_library_to_path(&global_veriloga_library_path(), library, &sources)
    }
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
fn load_global_veriloga_library() -> Result<Option<LoadedVerilogALibrary>, String> {
    load_global_veriloga_library_from_path(&global_veriloga_library_path())
}

#[cfg(not(target_arch = "wasm32"))]
fn load_global_veriloga_library_from_path(
    path: &Path,
) -> Result<Option<LoadedVerilogALibrary>, String> {
    crate::io::durable_file::reconcile_publication(path).map_err(|error| {
        format!(
            "failed to reconcile Verilog-A library '{}': {error}",
            path.display()
        )
    })?;
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(format!(
                "failed to read Verilog-A library '{}': {error}",
                path.display()
            ));
        }
    };
    let value: serde_json::Value = serde_json::from_str(&text).map_err(|error| {
        format!(
            "failed to parse Verilog-A library '{}': {error}",
            path.display()
        )
    })?;
    let version = value
        .get("version")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| {
            format!(
                "Verilog-A library '{}' has no valid format version",
                path.display()
            )
        })?;
    match version {
        version if version == u64::from(VERILOGA_LIBRARY_FORMAT_VERSION) => {
            let parsed: PersistedVerilogALibrary =
                serde_json::from_value(value).map_err(|error| {
                    format!(
                        "failed to parse Verilog-A library '{}': {error}",
                        path.display()
                    )
                })?;
            if parsed.version != VERILOGA_LIBRARY_FORMAT_VERSION {
                return Err("Verilog-A library version changed during parsing".to_owned());
            }
            validate_global_library_contract(&parsed.library, &parsed.sources)?;
            Ok(Some(LoadedVerilogALibrary {
                library: parsed.library,
                sources: Some(parsed.sources),
            }))
        }
        version if version == u64::from(LEGACY_VERILOGA_LIBRARY_FORMAT_VERSION) => {
            let parsed: LegacyPersistedVerilogALibrary =
                serde_json::from_value(value).map_err(|error| {
                    format!(
                        "failed to parse legacy Verilog-A library '{}': {error}",
                        path.display()
                    )
                })?;
            if parsed.version != LEGACY_VERILOGA_LIBRARY_FORMAT_VERSION {
                return Err("legacy Verilog-A library version changed during parsing".to_owned());
            }
            Ok(Some(LoadedVerilogALibrary {
                library: parsed.library,
                sources: None,
            }))
        }
        unsupported => Err(format!(
            "Verilog-A library '{}' uses unsupported format version {unsupported}; version {} is required",
            path.display(),
            VERILOGA_LIBRARY_FORMAT_VERSION
        )),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_global_veriloga_library_to_path(
    path: &Path,
    library: &Library,
    sources: &ProjectSourceRegistry,
) -> Result<(), String> {
    validate_global_library_contract(library, sources)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| {
            format!(
                "failed to create Verilog-A library directory '{}': {error}",
                parent.display()
            )
        })?;
    }

    let persisted = PersistedVerilogALibraryRef {
        version: VERILOGA_LIBRARY_FORMAT_VERSION,
        library,
        sources,
    };
    let json = serde_json::to_string_pretty(&persisted)
        .map_err(|error| format!("failed to serialize Verilog-A library: {error}"))?;
    let expected = crate::io::durable_file::observe_expected_content(path).map_err(|error| {
        format!(
            "failed to authorize Verilog-A library destination '{}': {error}",
            path.display()
        )
    })?;
    let serialized_digest: [u8; 32] = {
        use sha2::Digest as _;
        sha2::Sha256::digest(json.as_bytes()).into()
    };
    if expected == crate::io::durable_file::ExpectedContent::Digest(serialized_digest) {
        return Ok(());
    }
    publish_global_veriloga_library(path, expected, json.as_bytes())?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn global_sources_from_registry(
    registry: &ProjectSourceRegistry,
) -> Result<ProjectSourceRegistry, String> {
    let mut bundles = Vec::new();
    for bundle in registry.iter_bundles() {
        let ProjectSourceOwner::CellView { reference } = bundle.owner() else {
            continue;
        };
        if reference.library == VERILOGA_LIBRARY_NAME {
            bundles.push(bundle.clone());
        } else if reference
            .library
            .eq_ignore_ascii_case(VERILOGA_LIBRARY_NAME)
        {
            return Err(format!(
                "global Verilog-A source owner '{}' uses a non-canonical library name",
                reference.display_path()
            ));
        }
    }
    ProjectSourceRegistry::try_from_bundles(bundles)
        .map_err(|error| format!("global Verilog-A source registry is invalid: {error}"))
}

#[cfg(not(target_arch = "wasm32"))]
fn validate_global_library_contract(
    library: &Library,
    sources: &ProjectSourceRegistry,
) -> Result<(), String> {
    if library.name != VERILOGA_LIBRARY_NAME {
        return Err(format!(
            "global Verilog-A library identity must be '{VERILOGA_LIBRARY_NAME}', found '{}'",
            library.name
        ));
    }
    sources
        .validate()
        .map_err(|error| format!("global Verilog-A source registry is invalid: {error}"))?;

    let mut expected = BTreeSet::new();
    for (cell_key, cell) in &library.cells {
        if cell_key != &cell.name {
            return Err(format!(
                "global Verilog-A cell map key '{cell_key}' does not match embedded name '{}'",
                cell.name
            ));
        }
        for (view_key, view) in &cell.views {
            if view_key != &view.name {
                return Err(format!(
                    "global Verilog-A view map key '{view_key}' does not match embedded name '{}'",
                    view.name
                ));
            }
            let reference = CellViewRef::new(&library.name, &cell.name, &view.name);
            reference.validate_name_segments().map_err(|error| {
                format!(
                    "global Verilog-A view owner '{}' is invalid: {error}",
                    reference.display_path()
                )
            })?;
            if view.view_type == ViewType::VerilogA {
                expected.insert(reference.key());
            }
        }
    }

    let mut found = BTreeSet::new();
    for bundle in sources.iter_bundles() {
        if bundle.language() != ProjectSourceLanguage::VerilogA {
            return Err(format!(
                "global Verilog-A source bundle '{}' has language {}",
                bundle.id(),
                bundle.language()
            ));
        }
        let ProjectSourceOwner::CellView { reference } = bundle.owner() else {
            return Err(format!(
                "global Verilog-A source bundle '{}' is not owned by a cell view",
                bundle.id()
            ));
        };
        if reference.library != VERILOGA_LIBRARY_NAME {
            return Err(format!(
                "global Verilog-A source bundle '{}' is owned by '{}'",
                bundle.id(),
                reference.display_path()
            ));
        }
        if !expected.contains(&reference.key()) {
            return Err(format!(
                "global Verilog-A source owner '{}' does not identify an exact Verilog-A view",
                reference.display_path()
            ));
        }
        found.insert(reference.key());
    }
    if let Some(missing) = expected.difference(&found).next() {
        return Err(format!(
            "global Verilog-A view '{}' has no exact executable source bundle",
            missing
        ));
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn install_loaded_veriloga_library(
    library_manager: &mut LibraryManager,
    registry: &mut ProjectSourceRegistry,
    mut loaded: LoadedVerilogALibrary,
) -> Result<(), String> {
    // Version 1 normalized the top-level name during installation. Preserve
    // that migration, then require exact names and owners everywhere below it.
    loaded.library.name = VERILOGA_LIBRARY_NAME.to_owned();
    loaded.library.read_only = false;
    let sources = match loaded.sources {
        Some(sources) => sources,
        None => global_sources_from_registry(registry)?,
    };
    validate_global_library_contract(&loaded.library, &sources)?;

    let mut next_libraries = library_manager.clone();
    let mut next_sources = registry.clone();
    let stale_ids = next_sources
        .iter_bundles()
        .filter_map(|bundle| match bundle.owner() {
            ProjectSourceOwner::CellView { reference }
                if reference.library == VERILOGA_LIBRARY_NAME =>
            {
                Some(bundle.id())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    for id in stale_ids {
        next_sources.remove_bundle(id);
    }
    for bundle in sources.iter_bundles() {
        next_sources.insert_bundle(bundle.clone()).map_err(|error| {
            format!(
                "global Verilog-A source bundle '{}' conflicts with the active registry: {error}",
                bundle.id()
            )
        })?;
    }
    next_sources
        .validate()
        .map_err(|error| format!("merged project source registry is invalid: {error}"))?;
    next_libraries.add_library(loaded.library);

    *library_manager = next_libraries;
    *registry = next_sources;
    Ok(())
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
    use crate::state::{Cell, ProjectSourceBundle, ProjectSourceFile, View};

    fn global_fixture() -> (Library, ProjectSourceRegistry, CellViewRef) {
        let reference = CellViewRef::new(VERILOGA_LIBRARY_NAME, "precision_amp", "veriloga");
        let mut view = View::new("veriloga", ViewType::VerilogA);
        view.metadata.insert(
            "veriloga.module".to_owned(),
            "rspice_precision_amp_va".to_owned(),
        );
        view.metadata
            .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
        let mut cell = Cell::new("precision_amp");
        cell.add_view(view);
        let mut library = Library::new(VERILOGA_LIBRARY_NAME).with_technology("rf-cmos");
        library.add_cell(cell);
        let helper = ProjectSourceFile::try_new(
            "models/gain.va",
            "// exact retained helper source closure\n",
        )
        .unwrap();
        let dependency =
            crate::state::ProjectSourceDependency::try_new("precision_amp.va", "models/gain.va")
                .unwrap();
        let mut bundle = ProjectSourceBundle::try_new(
            ProjectSourceOwner::cell_view(reference.clone()),
            ProjectSourceLanguage::VerilogA,
            "precision_amp.va",
            "`include \"constants.vams\"\n`include \"disciplines.vams\"\n`include \"models/gain.va\"\nmodule rspice_precision_amp_va(p, n); inout p, n; electrical p, n; analog I(p,n) <+ 2.0 * V(p,n); endmodule\n",
            [helper],
            [dependency],
        )
        .unwrap();
        bundle.mark_validated().unwrap();
        let sources = ProjectSourceRegistry::try_from_bundles([bundle]).unwrap();
        (library, sources, reference)
    }

    #[test]
    fn publication_rejects_late_external_change_without_losing_it() {
        let root = unique_temp_dir("late-change");
        let path = root.join(VERILOGA_LIBRARY_CONFIG_FILE);
        std::fs::write(&path, b"authorized predecessor").expect("write predecessor");
        let expected =
            crate::io::durable_file::observe_expected_content(&path).expect("observe destination");
        let (library, sources, _) = global_fixture();
        let bytes = serde_json::to_vec_pretty(&PersistedVerilogALibraryRef {
            version: VERILOGA_LIBRARY_FORMAT_VERSION,
            library: &library,
            sources: &sources,
        })
        .expect("serialize library");
        std::fs::write(&path, b"late external edit").expect("race destination");

        let result = publish_global_veriloga_library(&path, expected, &bytes);

        assert!(result.is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"late external edit");
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn create_save_restart_and_compile_round_trip_retains_exact_source_closure() {
        let root = unique_temp_dir("round-trip");
        let path = root.join(VERILOGA_LIBRARY_CONFIG_FILE);
        let (library, sources, reference) = global_fixture();
        let original = sources
            .bundle_for_owner(&ProjectSourceOwner::cell_view(reference.clone()))
            .unwrap();

        save_global_veriloga_library_to_path(&path, &library, &sources).expect("save library");
        let loaded = load_global_veriloga_library_from_path(&path)
            .expect("load library")
            .expect("persisted library");
        let mut restarted_libraries = LibraryManager::with_primitives();
        let mut restarted_sources = ProjectSourceRegistry::default();
        install_loaded_veriloga_library(&mut restarted_libraries, &mut restarted_sources, loaded)
            .expect("install exact global document");

        let restored = restarted_sources
            .bundle_for_owner(&ProjectSourceOwner::cell_view(reference))
            .expect("restored exact source bundle");
        assert_eq!(restored.id(), original.id());
        assert_eq!(restored.closure_digest(), original.closure_digest());
        assert_eq!(restored.root().exact_bytes(), original.root().exact_bytes());
        assert_eq!(restored.files(), original.files());
        let receipt = crate::workbench::code_workspace::compile_project_bundle_receipt(
            crate::product::ProjectId::new(),
            restored,
            Some("rspice_precision_amp_va"),
        )
        .expect("restored closure compiles without filesystem access");
        assert_eq!(receipt.token.bundle_id, original.id());
        assert_eq!(receipt.token.closure_digest, original.closure_digest());
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn identical_documents_publish_to_each_destination_and_recover_external_deletion() {
        let root = unique_temp_dir("destination-identity");
        let first = root.join("first.json");
        let second = root.join("second.json");
        let (library, sources, _) = global_fixture();

        save_global_veriloga_library_to_path(&first, &library, &sources).unwrap();
        save_global_veriloga_library_to_path(&second, &library, &sources).unwrap();
        assert_eq!(
            std::fs::read(&first).unwrap(),
            std::fs::read(&second).unwrap()
        );

        std::fs::remove_file(&first).unwrap();
        save_global_veriloga_library_to_path(&first, &library, &sources).unwrap();
        assert_eq!(
            std::fs::read(&first).unwrap(),
            std::fs::read(&second).unwrap()
        );
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn version_one_migrates_only_with_exact_recoverable_session_sources() {
        let root = unique_temp_dir("version-one");
        let path = root.join(VERILOGA_LIBRARY_CONFIG_FILE);
        let (library, sources, reference) = global_fixture();
        let legacy = serde_json::json!({
            "version": LEGACY_VERILOGA_LIBRARY_FORMAT_VERSION,
            "library": library,
        });
        std::fs::write(&path, serde_json::to_vec_pretty(&legacy).unwrap()).unwrap();
        let loaded = load_global_veriloga_library_from_path(&path)
            .unwrap()
            .expect("legacy document parses");
        let mut libraries = LibraryManager::default();
        let mut recoverable_sources = sources;

        install_loaded_veriloga_library(&mut libraries, &mut recoverable_sources, loaded)
            .expect("legacy catalog joins its exact session source");

        assert!(libraries.get_library(VERILOGA_LIBRARY_NAME).is_some());
        assert!(
            recoverable_sources
                .bundle_for_owner(&ProjectSourceOwner::cell_view(reference))
                .is_some()
        );
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn missing_or_mismatched_source_owner_rejects_save_and_preserves_install_targets() {
        let (library, sources, reference) = global_fixture();
        let empty = ProjectSourceRegistry::default();
        assert!(
            validate_global_library_contract(&library, &empty)
                .unwrap_err()
                .contains("no exact executable source bundle")
        );

        let bundle = sources
            .bundle_for_owner(&ProjectSourceOwner::cell_view(reference))
            .unwrap();
        let wrong = ProjectSourceBundle::try_new(
            ProjectSourceOwner::cell_view(CellViewRef::new(
                VERILOGA_LIBRARY_NAME,
                "other_cell",
                "veriloga",
            )),
            ProjectSourceLanguage::VerilogA,
            bundle.root().logical_path(),
            bundle.root().content(),
            bundle.files().iter().cloned(),
            bundle.dependencies().iter().cloned(),
        )
        .unwrap();
        let wrong_sources = ProjectSourceRegistry::try_from_bundles([wrong]).unwrap();
        assert!(
            validate_global_library_contract(&library, &wrong_sources)
                .unwrap_err()
                .contains("does not identify an exact Verilog-A view")
        );

        let mut existing_libraries = LibraryManager::default();
        existing_libraries.add_library(Library::new("keep"));
        let existing_sources = ProjectSourceRegistry::default();
        let before_libraries = serde_json::to_value(&existing_libraries).unwrap();
        let mut attempted_sources = existing_sources.clone();
        let result = install_loaded_veriloga_library(
            &mut existing_libraries,
            &mut attempted_sources,
            LoadedVerilogALibrary {
                library,
                sources: Some(wrong_sources),
            },
        );
        assert!(result.is_err());
        assert_eq!(
            serde_json::to_value(&existing_libraries).unwrap(),
            before_libraries
        );
        assert_eq!(attempted_sources, existing_sources);
    }

    #[test]
    fn unsupported_format_is_rejected_instead_of_partially_restored() {
        let root = unique_temp_dir("future-version");
        let path = root.join(VERILOGA_LIBRARY_CONFIG_FILE);
        std::fs::write(&path, br#"{"version":999,"library":{},"sources":{}}"#).unwrap();

        let error = load_global_veriloga_library_from_path(&path).unwrap_err();

        assert!(error.contains("unsupported format version 999"));
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
