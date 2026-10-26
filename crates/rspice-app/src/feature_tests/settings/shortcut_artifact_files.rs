//! Preparing and writing shortcut reference files over the product's own commands.

use std::cell::RefCell;

use egui::os::OperatingSystem;

use crate::state::command_catalog::{CommandPlatform, ShortcutContext};
use crate::state::shortcuts::ShortcutPreferences;
use crate::workbench::shortcuts::artifacts::io::*;
use crate::workbench::shortcuts::artifacts::projection::{
    ShortcutExportRequest, ShortcutExportScope, ShortcutReferenceModel,
    build_shortcut_reference_model, serialize_shortcut_reference_json,
};
use std::path::{Path, PathBuf};

fn reference_model() -> ShortcutReferenceModel {
    build_shortcut_reference_model(
        &ShortcutPreferences::default(),
        &ShortcutExportRequest {
            scope: ShortcutExportScope::CompleteResolved,
            include_platform_mappings: false,
            runtime_platform: CommandPlatform::Desktop,
            operating_system: OperatingSystem::Windows,
            current_contexts: vec![ShortcutContext::ResultsWorkspace],
        },
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap()
}

fn prepared_json() -> PreparedShortcutArtifact {
    prepare_shortcut_artifact(&reference_model(), ShortcutArtifactFormat::Json, None).unwrap()
}

#[test]
fn format_metadata_is_exact_and_complete() {
    assert_eq!(ShortcutArtifactFormat::ALL.len(), 3);
    assert_eq!(ShortcutArtifactFormat::Json.label(), "JSON");
    assert_eq!(ShortcutArtifactFormat::Json.extension(), "json");
    assert_eq!(ShortcutArtifactFormat::Json.mime_type(), "application/json");
    assert_eq!(
        ShortcutArtifactFormat::Json.default_filename(),
        "rspice-shortcuts.json"
    );
    assert_eq!(
        ShortcutArtifactFormat::Markdown.mime_type(),
        "text/markdown;charset=utf-8"
    );
    assert_eq!(
        ShortcutArtifactFormat::Markdown.default_filename(),
        "rspice-shortcuts.md"
    );
    assert_eq!(ShortcutArtifactFormat::Pdf.mime_type(), "application/pdf");
    assert_eq!(
        ShortcutArtifactFormat::Pdf.default_filename(),
        "rspice-shortcuts.pdf"
    );
    assert_eq!(
        ShortcutArtifactFormat::Json.max_bytes(),
        MAX_SHORTCUT_JSON_ARTIFACT_BYTES
    );
}

#[test]
fn filename_normalization_discards_paths_extensions_and_reserved_names() {
    assert_eq!(
        normalize_shortcut_artifact_filename(
            r"C:\private\project\My shortcuts.PDF",
            ShortcutArtifactFormat::Json,
        ),
        "My shortcuts.json"
    );
    assert_eq!(
        normalize_shortcut_artifact_filename("../../CON.json", ShortcutArtifactFormat::Pdf),
        "_CON.pdf"
    );
    assert_eq!(
        normalize_shortcut_artifact_filename("<>:*?|", ShortcutArtifactFormat::Markdown),
        "rspice-shortcuts.md"
    );
    let long = "electrical-engineering-shortcut-reference-".repeat(8);
    let normalized = normalize_shortcut_artifact_filename(&long, ShortcutArtifactFormat::Json);
    assert!(normalized.len() <= MAX_FILENAME_STEM_BYTES + ".json".len());
}

#[test]
fn preparation_preserves_serializer_bytes_exactly() {
    let model = reference_model();
    let artifact = prepare_shortcut_artifact(
        &model,
        ShortcutArtifactFormat::Json,
        Some("portable shortcuts"),
    )
    .unwrap();
    assert_eq!(artifact.filename(), "portable shortcuts.json");
    assert_eq!(
        artifact.bytes(),
        serialize_shortcut_reference_json(&model)
            .unwrap()
            .as_bytes()
    );
}

type FakeWrite = (PathBuf, ShortcutArtifactFormat, String, Vec<u8>);

struct FakeIo {
    selection: RefCell<Result<Option<PathBuf>, String>>,
    observe_error: RefCell<Option<String>>,
    publish_error: RefCell<Option<String>>,
    publication: RefCell<Option<ArtifactPublication>>,
    observations: RefCell<Vec<PathBuf>>,
    writes: RefCell<Vec<FakeWrite>>,
}

impl Default for FakeIo {
    fn default() -> Self {
        Self {
            selection: RefCell::new(Ok(None)),
            observe_error: RefCell::new(None),
            publish_error: RefCell::new(None),
            publication: RefCell::new(None),
            observations: RefCell::new(Vec::new()),
            writes: RefCell::new(Vec::new()),
        }
    }
}

impl FakeIo {
    fn selecting(path: impl Into<PathBuf>) -> Self {
        Self {
            selection: RefCell::new(Ok(Some(path.into()))),
            publication: RefCell::new(Some(ArtifactPublication::Durable)),
            ..Self::default()
        }
    }
}

impl ShortcutArtifactIo for FakeIo {
    fn choose_destination(
        &self,
        _format: ShortcutArtifactFormat,
        _default_filename: &str,
    ) -> Result<Option<PathBuf>, String> {
        self.selection.borrow().clone()
    }

    fn observe_destination(&self, path: &Path) -> Result<ObservedArtifactDestination, String> {
        if let Some(error) = self.observe_error.borrow().clone() {
            return Err(error);
        }
        self.observations.borrow_mut().push(path.to_path_buf());
        Ok(ObservedArtifactDestination::backend_managed(
            path.to_path_buf(),
        ))
    }

    fn publish_observed(
        &self,
        destination: &ObservedArtifactDestination,
        artifact: &PreparedShortcutArtifact,
    ) -> Result<ArtifactPublication, String> {
        if let Some(error) = self.publish_error.borrow().clone() {
            return Err(error);
        }
        self.writes.borrow_mut().push((
            destination.path.clone(),
            artifact.format(),
            artifact.format().mime_type().to_owned(),
            artifact.bytes().to_vec(),
        ));
        self.publication
            .borrow()
            .ok_or_else(|| "fake publication disposition is missing".to_owned())
    }
}

#[test]
fn cancellation_is_not_an_error_and_publishes_nothing() {
    let io = FakeIo::default();
    let outcome = export_shortcut_artifact_with_io(&prepared_json(), &io).unwrap();
    assert_eq!(outcome, ShortcutArtifactExportOutcome::Cancelled);
    assert!(io.writes.borrow().is_empty());
}

#[test]
fn backend_errors_are_typed_by_stage() {
    let picker = FakeIo {
        selection: RefCell::new(Err("picker unavailable".to_owned())),
        ..FakeIo::default()
    };
    assert_eq!(
        export_shortcut_artifact_with_io(&prepared_json(), &picker)
            .unwrap_err()
            .code(),
        "shortcut-artifact.export-picker"
    );

    let publish = FakeIo::selecting("shortcuts.json");
    *publish.publish_error.borrow_mut() = Some("sink rejected bytes".to_owned());
    assert_eq!(
        export_shortcut_artifact_with_io(&prepared_json(), &publish)
            .unwrap_err()
            .code(),
        "shortcut-artifact.export-publish"
    );
}

#[test]
fn fake_backend_receives_identical_bytes_and_truthful_download_outcome() {
    let artifact = prepared_json();
    let selected = PathBuf::from("folder").join("reference.JSON");
    let io = FakeIo::selecting(&selected);
    *io.publication.borrow_mut() = Some(ArtifactPublication::DownloadStarted);
    let outcome = export_shortcut_artifact_with_io(&artifact, &io).unwrap();
    assert_eq!(
        outcome,
        ShortcutArtifactExportOutcome::DownloadStarted {
            suggested_filename: "reference.JSON".to_owned()
        }
    );
    let writes = io.writes.borrow();
    assert_eq!(writes.len(), 1);
    assert_eq!(writes[0].1, ShortcutArtifactFormat::Json);
    assert_eq!(writes[0].2, "application/json");
    assert_eq!(writes[0].3, artifact.bytes());
    assert_eq!(writes[0].0, selected);
}

#[test]
fn picker_authority_cannot_be_retargeted_to_a_different_extension() {
    let artifact = prepared_json();
    for selected in ["authorized.txt", "missing-extension"] {
        let io = FakeIo::selecting(selected);
        let error = export_shortcut_artifact_with_io(&artifact, &io).unwrap_err();
        assert_eq!(error.code(), "shortcut-artifact.export-extension");
        assert!(io.observations.borrow().is_empty());
        assert!(io.writes.borrow().is_empty());
    }

    let exact = PathBuf::from("Authorized.JSON");
    let io = FakeIo::selecting(&exact);
    let outcome = export_shortcut_artifact_with_io(&artifact, &io).unwrap();
    assert_eq!(
        outcome,
        ShortcutArtifactExportOutcome::Published {
            path: exact.clone()
        }
    );
    assert_eq!(&*io.observations.borrow(), &[exact.clone()]);
    assert_eq!(io.writes.borrow()[0].0, exact);
}

#[test]
fn artifact_limits_and_text_encoding_fail_closed() {
    let too_large = vec![0; MAX_SHORTCUT_JSON_ARTIFACT_BYTES as usize + 1];
    assert_eq!(
        validate_artifact_bytes(ShortcutArtifactFormat::Json, &too_large)
            .unwrap_err()
            .code(),
        "shortcut-artifact.byte-limit"
    );
    assert_eq!(
        validate_artifact_bytes(ShortcutArtifactFormat::Markdown, &[0xff])
            .unwrap_err()
            .code(),
        "shortcut-artifact.invalid-utf8"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_cas_rejects_late_edit_and_preserves_external_text() {
    let root = unique_temp_dir("late-edit");
    let path = root.join("shortcuts.json");
    std::fs::write(&path, b"picker-time text").unwrap();
    let backend = NativeShortcutArtifactIo;
    let destination = backend.observe_destination(&path).unwrap();
    std::fs::write(&path, b"late external text").unwrap();

    let error = backend
        .publish_observed(&destination, &prepared_json())
        .unwrap_err();
    assert!(error.contains("destination changed"), "{error}");
    assert_eq!(std::fs::read(&path).unwrap(), b"late external text");
    std::fs::remove_dir_all(root).unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_cas_rejects_late_creation_and_preserves_external_binary() {
    let root = unique_temp_dir("late-creation");
    let path = root.join("shortcuts.pdf");
    let backend = NativeShortcutArtifactIo;
    let destination = backend.observe_destination(&path).unwrap();
    let external = [0x00, 0xff, 0x13, 0x37];
    std::fs::write(&path, external).unwrap();
    let artifact = PreparedShortcutArtifact {
        format: ShortcutArtifactFormat::Pdf,
        filename: "shortcuts.pdf".to_owned(),
        bytes: b"%PDF-1.7\nexact local bytes".to_vec(),
    };

    let error = backend
        .publish_observed(&destination, &artifact)
        .unwrap_err();
    assert!(error.contains("destination changed"), "{error}");
    assert_eq!(std::fs::read(&path).unwrap(), external);
    std::fs::remove_dir_all(root).unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_binary_publication_is_byte_exact() {
    let root = unique_temp_dir("binary-byte-identity");
    let path = root.join("shortcuts.pdf");
    let backend = NativeShortcutArtifactIo;
    let destination = backend.observe_destination(&path).unwrap();
    let bytes = vec![b'%', b'P', b'D', b'F', 0, 0xff, 0x13, 0x37];
    let artifact = PreparedShortcutArtifact {
        format: ShortcutArtifactFormat::Pdf,
        filename: "shortcuts.pdf".to_owned(),
        bytes: bytes.clone(),
    };

    assert_eq!(
        backend.publish_observed(&destination, &artifact).unwrap(),
        ArtifactPublication::Durable
    );
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    std::fs::remove_dir_all(root).unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
fn unique_temp_dir(label: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "rspice-shortcut-artifact-{label}-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4()
    ));
    std::fs::create_dir_all(&root).unwrap();
    root
}
