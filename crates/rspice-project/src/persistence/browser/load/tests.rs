//! Browser acquisition and restart policy without platform handles or storage.

use super::*;
use crate::persistence::{digest_bytes, serialized_project};
use crate::{ProjectFile, ProjectLibraries, ProjectWorkspace};
use std::path::{Path, PathBuf};

struct NoSourceFiles;
impl HierarchySourceFiles for NoSourceFiles {
    fn source_paths_match(&self, _: &Path, _: &Path) -> bool {
        panic!("no external sources")
    }
    fn configured_source_identity(&self, _: &Path) -> String {
        panic!("no external sources")
    }
    fn validate_source_file(
        &self,
        _: &Path,
        _: rspice_design::library::ViewType,
        _: &rspice_design::schematic::component::LibraryCellInstance,
    ) -> Result<(), String> {
        panic!("no external sources")
    }
}

fn fixture() -> (Vec<u8>, BrowserBindingReceipt, BrowserBindingMetadata) {
    use rspice_design::library::{Cell, Library, View, ViewType};
    let mut workspace = ProjectWorkspace::default();
    workspace.project.path = Some(PathBuf::from("native-origin.rspiceproj"));
    let mut cell = Cell::new(workspace.project.top_cell.clone());
    cell.add_view(View::new(
        workspace.active_view.view.clone(),
        ViewType::Schematic,
    ));
    let mut library = Library::new(workspace.project.root_library.clone());
    library.add_cell(cell);
    let mut libraries = ProjectLibraries::default();
    libraries.add_library(library);
    let file = ProjectFile::new(workspace, libraries);
    let (bytes, digest) = serialized_project(&file).unwrap();
    let receipt = BrowserBindingReceipt {
        binding_id: uuid::Uuid::new_v4(),
        project_id: file.workspace.project.id().to_string(),
        accepted_generation: 7,
        accepted_digest: digest,
        backend: BrowserBindingBackend::ExternalFile,
    };
    let metadata = BrowserBindingMetadata {
        schema_version: BROWSER_BINDING_SCHEMA_VERSION,
        binding_id: receipt.binding_id.to_string(),
        project_id: receipt.project_id.clone(),
        accepted_generation: 7,
        accepted_digest: digest.to_string(),
        backend: receipt.backend,
        display_name: "design.rspiceproj".to_owned(),
    };
    (bytes, receipt, metadata)
}

#[test]
fn restore_record_eviction_requires_identity_before_digest_admission() {
    let (_, receipt, mut metadata) = fixture();
    metadata.accepted_digest = "malformed".to_owned();
    for field in 0..4 {
        let mut foreign = metadata.clone();
        match field {
            0 => foreign.schema_version += 1,
            1 => foreign.binding_id = uuid::Uuid::new_v4().to_string(),
            2 => foreign.project_id = "different project".to_owned(),
            _ => foreign.backend = BrowserBindingBackend::Opfs,
        }
        assert!(matches!(
            BrowserRestoreCandidate::new(foreign, &receipt),
            Err(BrowserRestoreMetadataError::Unowned(_))
        ));
    }
    assert!(matches!(BrowserRestoreCandidate::new(metadata, &receipt),
        Err(BrowserRestoreMetadataError::InvalidOwned(message)) if message.starts_with("browser binding digest is invalid:")));
}

#[test]
fn changed_metadata_precedes_reconnect_and_preserves_distinct_generations() {
    let (_, receipt, metadata) = fixture();
    let mut changed = metadata.clone();
    changed.accepted_generation = 8;
    let candidate = BrowserRestoreCandidate::new(changed, &receipt).unwrap();
    match candidate.check_permission("denied").unwrap_err() {
        BrowserRestoreIssue::Conflict {
            binding,
            observed_digest,
            reason,
        } => {
            assert_eq!(binding.receipt, receipt);
            assert_eq!(binding.persisted_generation, Some(8));
            assert!(binding.durable_receipt().is_none());
            assert_eq!(observed_digest, receipt.accepted_digest);
            assert_eq!(
                reason,
                "another tab committed generation 8 while this session accepted generation 7"
            );
        }
        other => panic!("unexpected disposition: {other:?}"),
    }
    let candidate = BrowserRestoreCandidate::new(metadata, &receipt).unwrap();
    for permission in ["prompt", "denied", "unknown"] {
        match candidate.check_permission(permission).unwrap_err() {
            BrowserRestoreIssue::ReconnectRequired(binding) => {
                assert_eq!(binding.receipt, receipt);
                assert_eq!(binding.persisted_generation, Some(7));
            }
            other => panic!("unexpected disposition: {other:?}"),
        }
    }
    candidate.check_permission("granted").unwrap();
}

#[test]
fn restore_checks_bytes_before_decoding_and_project_identity_before_acceptance() {
    let (bytes, receipt, metadata) = fixture();
    for invalid in [vec![0xff], b"not JSON".to_vec()] {
        let candidate = BrowserRestoreCandidate::new(metadata.clone(), &receipt).unwrap();
        match candidate
            .decode(invalid.clone(), "granted", NoSourceFiles)
            .unwrap_err()
        {
            BrowserRestoreIssue::Conflict {
                binding,
                observed_digest,
                reason,
            } => {
                assert_eq!(binding.receipt, receipt);
                assert_eq!(observed_digest, digest_bytes(&invalid));
                assert_eq!(reason, "canonical browser project changed outside RSpice");
            }
            other => panic!("unexpected disposition: {other:?}"),
        }
        let exact = BrowserBindingReceipt {
            accepted_digest: digest_bytes(&invalid),
            ..receipt.clone()
        };
        let exact_metadata = BrowserBindingMetadata {
            accepted_digest: exact.accepted_digest.to_string(),
            ..metadata.clone()
        };
        let candidate = BrowserRestoreCandidate::new(exact_metadata, &exact).unwrap();
        assert!(
            matches!(candidate.decode(invalid, "granted", NoSourceFiles),
            Err(BrowserRestoreIssue::Retryable(message)) if message.starts_with("canonical browser project is not UTF-8:") || message.starts_with("canonical browser project is invalid:"))
        );
    }
    let (different, _, _) = fixture();
    let mismatched = BrowserBindingReceipt {
        accepted_digest: digest_bytes(&different),
        ..receipt.clone()
    };
    let mismatched_metadata = BrowserBindingMetadata {
        accepted_digest: mismatched.accepted_digest.to_string(),
        ..metadata.clone()
    };
    let candidate = BrowserRestoreCandidate::new(mismatched_metadata, &mismatched).unwrap();
    assert!(
        matches!(candidate.decode(different, "granted", NoSourceFiles),
        Err(BrowserRestoreIssue::Conflict { reason, .. }) if reason == "canonical browser project identity no longer matches its binding")
    );

    let candidate = BrowserRestoreCandidate::new(metadata, &receipt).unwrap();
    candidate.check_permission("granted").unwrap();
    let (decoded, binding) = candidate.decode(bytes, "granted", NoSourceFiles).unwrap();
    assert_eq!(
        decoded.file.workspace.project.id().to_string(),
        receipt.project_id
    );
    assert!(decoded.file.workspace.project.path.is_none());
    assert_eq!(binding.durable_receipt(), Some(receipt));
}

#[test]
fn explicit_browser_open_uses_exact_owned_bytes_without_restart_authority() {
    let (bytes, receipt, _) = fixture();
    let original_pointer = bytes.as_ptr();
    let captured = ProjectBytes::from_bytes(bytes).unwrap();
    let (bytes, digest) = captured.into_parts();
    assert_eq!(bytes.as_ptr(), original_pointer);
    assert_eq!(digest, receipt.accepted_digest);
    let (decoded, binding) = BrowserBinding::open(
        ProjectBytes::from_bytes(bytes).unwrap(),
        "selected.rspiceproj".to_owned(),
        NoSourceFiles,
    )
    .unwrap();
    assert_eq!(
        decoded.file.workspace.project.id().to_string(),
        receipt.project_id
    );
    assert_eq!(binding.receipt.project_id, receipt.project_id);
    assert_eq!(binding.receipt.accepted_digest, digest);
    assert_eq!(binding.receipt.accepted_generation, 1);
    assert_eq!(binding.receipt.backend, BrowserBindingBackend::ExternalFile);
    assert_eq!(binding.display_name, "selected.rspiceproj");
    assert!(binding.persisted_generation.is_none());
    assert!(binding.durable_receipt().is_none());
    assert!(
        BrowserBinding::open(
            ProjectBytes::from_bytes(vec![0xff]).unwrap(),
            String::new(),
            NoSourceFiles
        )
        .unwrap_err()
        .starts_with("selected project is not valid UTF-8:")
    );
}
