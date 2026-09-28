//! Browser save authority exercised without a browser or editor session.

use super::*;
use crate::persistence::digest_bytes;
use crate::{ProjectLibraries, ProjectWorkspace};
use std::cell::Cell;

fn prepare(
    lifecycle: &mut ProjectLifecycle,
    project_copy: bool,
) -> (PreparedBrowserSave, String, BrowserSavePublication) {
    use rspice_design::library::{Cell, Library, View, ViewType};
    let workspace = ProjectWorkspace::default();
    let mut cell = Cell::new(workspace.project.top_cell.clone());
    cell.add_view(View::new(
        workspace.active_view.view.clone(),
        ViewType::Schematic,
    ));
    let mut library = Library::new(workspace.project.root_library.clone());
    library.add_cell(cell);
    let mut libraries = ProjectLibraries::default();
    libraries.add_library(library);
    let mut file = ProjectFile::new(workspace, libraries);
    file.workspace.project.path = Some(PathBuf::from("original.rspiceproj"));
    let project_id = file.workspace.project.id().to_string();
    let transaction = lifecycle.begin_save().unwrap();
    let context = lifecycle.browser_operation_context(&project_id, None);
    let mut prepared = StagedBrowserSave::new(
        transaction,
        context,
        file,
        SaveScope::AllDocuments,
        ProjectDocumentId::ProjectConfiguration,
        project_copy,
        "copy.rspiceproj",
    )
    .unwrap()
    .bind(None, || BrowserBindingBackend::ExternalFile)
    .unwrap();
    let bytes = prepared.take_bytes();
    let decoded: ProjectFile = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        decoded.workspace.project.id(),
        prepared.candidate().workspace.project.id()
    );
    let intent = prepared.intent();
    let publication = BrowserSavePublication {
        receipt: BrowserBindingReceipt {
            binding_id: intent.binding_id,
            project_id: intent.project_id.clone(),
            accepted_generation: intent.accepted_generation,
            accepted_digest: digest_bytes(&bytes),
            backend: intent.backend,
        },
        display_name: "saved.rspiceproj".to_owned(),
        durable: true,
    };
    (prepared, project_id, publication)
}

#[test]
fn late_browser_completion_preserves_the_current_transaction() {
    for context_only in [false, true] {
        let mut lifecycle = ProjectLifecycle::default();
        let (prepared, project_id, publication) = prepare(&mut lifecycle, false);
        let current = if context_only {
            lifecycle.accept_content();
            prepared.transaction()
        } else {
            lifecycle.cancel_browser_operation();
            lifecycle.begin_save().unwrap()
        };
        let released = Cell::new(false);
        assert!(matches!(
            lifecycle.complete_browser_save(prepared, &project_id, None, publication, || released
                .set(true)),
            Err(ProjectLifecycleError::TransactionInProgress)
        ));
        assert!(released.get());
        assert!(lifecycle.is_current_transaction(current));
    }
}

#[test]
fn canonical_completion_requires_exact_bytes_and_defers_transaction_end_until_adoption() {
    let mut lifecycle = ProjectLifecycle::default();
    let (prepared, project_id, mut publication) = prepare(&mut lifecycle, false);
    publication.receipt.accepted_digest = digest_bytes(b"unrelated bytes");
    let released = Cell::new(false);
    assert!(matches!(
        lifecycle.complete_browser_save(prepared, &project_id, None, publication, || released
            .set(true)),
        Err(ProjectLifecycleError::InvalidState(_))
    ));
    assert!(released.get());
    assert!(!lifecycle.operation_in_progress());

    let (prepared, project_id, publication) = prepare(&mut lifecycle, false);
    let transaction = prepared.transaction();
    let receipt = publication.receipt.clone();
    released.set(false);
    let completion = lifecycle
        .complete_browser_save(prepared, &project_id, None, publication, || {
            released.set(true)
        })
        .unwrap()
        .unwrap();
    assert!(!released.get());
    assert!(lifecycle.is_current_transaction(transaction));
    assert_eq!(completion.binding.durable_receipt(), Some(receipt.clone()));
    assert_eq!(
        serialized_project(&completion.candidate).unwrap().1,
        receipt.accepted_digest
    );
    assert!(completion.candidate.workspace.project.path.is_none());
    lifecycle.accept_content();
    lifecycle.cancel_transaction();
    assert!(!lifecycle.operation_in_progress());
}

#[test]
fn independent_copy_forks_content_and_never_returns_canonical_adoption() {
    let mut lifecycle = ProjectLifecycle::default();
    let (prepared, project_id, mut publication) = prepare(&mut lifecycle, true);
    assert_ne!(
        prepared.candidate().workspace.project.id().to_string(),
        project_id
    );
    assert_eq!(
        prepared.candidate().workspace.project.path,
        Some(PathBuf::from("copy.rspiceproj"))
    );
    // A copy completion does not establish canonical binding authority.
    publication.receipt.project_id = "unrelated".to_owned();
    let released = Cell::new(false);
    let generation = lifecycle.accepted_generation();
    let completion = lifecycle
        .complete_browser_save(prepared, &project_id, None, publication, || {
            released.set(true)
        })
        .unwrap();
    assert!(completion.is_none());
    assert!(released.get());
    assert_eq!(lifecycle.accepted_generation(), generation);
    assert!(!lifecycle.operation_in_progress());
}
