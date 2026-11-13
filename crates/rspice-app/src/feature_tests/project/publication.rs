//! Project library publication and rollback through the whole application:
//! the durable writer's proof before a receipt commits, the rollback artifact
//! checked byte for byte, and the governance dialogs drawn at phone width.

#[cfg(not(target_arch = "wasm32"))]
use std::cell::RefCell;
#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};
#[cfg(not(target_arch = "wasm32"))]
use std::rc::Rc;

use crate::workbench::RSpiceApp;
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::surfaces::project::library_publication::{PublicationAction, publish_native};
use crate::workbench::surfaces::project::library_publication::{
    PublicationDraft, audit_text_valid, commit_project_library_publication, open_publication,
    open_rollback, prepare_project_library_publication, publication_filename, safe_file_stem, show,
    validate_rollback_artifact,
};
use crate::workbench::workflows::export_workflow::SaveDialogConfig;
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, ObservedExportDestination};

/// Every `(path, bytes)` the test double was asked to publish, shared so
/// the assertions can read them after the workflow has moved on.
#[cfg(not(target_arch = "wasm32"))]
type RecordedWrites = Rc<RefCell<Vec<(PathBuf, Vec<u8>)>>>;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
struct PublicationIo {
    destination: PathBuf,
    reopenable: bool,
    writes: RecordedWrites,
}

#[cfg(not(target_arch = "wasm32"))]
impl ExportWorkflowIo for PublicationIo {
    fn show_save_dialog(&self, _config: SaveDialogConfig<'_>) -> Result<Option<PathBuf>, String> {
        Ok(Some(self.destination.clone()))
    }

    fn write_text_file(&self, _path: &Path, _contents: &str) -> Result<(), String> {
        Err("text export is outside this test".to_owned())
    }

    fn write_bytes_file_observed(
        &self,
        destination: &ObservedExportDestination,
        contents: &[u8],
        _mime_type: &str,
    ) -> Result<(), String> {
        self.writes
            .borrow_mut()
            .push((destination.path().to_path_buf(), contents.to_vec()));
        Ok(())
    }

    fn write_waveform_csv(
        &self,
        _dataset: &crate::io::WaveformDataset,
        _path: &Path,
    ) -> Result<(), String> {
        Err("waveform export is outside this test".to_owned())
    }

    fn saved_paths_are_reopenable(&self) -> bool {
        self.reopenable
    }
}

fn complete_publication_draft(app: &RSpiceApp) -> PublicationDraft {
    let mut draft = PublicationDraft::new(&app.state.model);
    draft.label = "analog-core-1.0.0".to_owned();
    draft.actor_id = "release-engineer@example.test".to_owned();
    draft.authority_id = "organization-release-authority".to_owned();
    draft.reason = "Qualified library handoff".to_owned();
    draft
}

#[test]
fn publication_and_rollback_fields_are_strict_audit_text() {
    assert!(audit_text_valid("release-engineer@example.test"));
    assert!(!audit_text_valid(""));
    assert!(!audit_text_valid(" leading"));
    assert!(!audit_text_valid("trailing "));
    assert!(!audit_text_valid("line\nbreak"));
    assert!(!audit_text_valid(&"x".repeat(241)));

    let app = RSpiceApp::test_instance();
    let mut draft = PublicationDraft::new(&app.state.model);
    assert!(draft.field_error().is_some());
    draft = complete_publication_draft(&app);
    assert!(draft.field_error().is_none());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_writer_persists_exact_candidate_before_committing_receipt() {
    let mut app = RSpiceApp::test_instance();
    let draft = complete_publication_draft(&app);
    let writes = Rc::new(RefCell::new(Vec::new()));
    app.export_workflow_io = Box::new(PublicationIo {
        destination: PathBuf::from("release").join("analog-core"),
        reopenable: true,
        writes: Rc::clone(&writes),
    });

    let action = publish_native(&mut app, &draft).expect("publication succeeds");
    assert!(matches!(action, PublicationAction::Published(_)));
    let [receipt] = app
        .state
        .model
        .workspace
        .content
        .project
        .library_publications()
    else {
        panic!("exactly one publication receipt must commit");
    };
    let writes = writes.borrow();
    let [(path, bytes)] = writes.as_slice() else {
        panic!("exactly one durable writer call is expected");
    };
    assert_eq!(
        path,
        &PathBuf::from("release").join("analog-core.rspiceproj")
    );
    assert_eq!(bytes.len() as u64, receipt.snapshot_byte_len());
    validate_rollback_artifact(&app.state, receipt.publication_id(), bytes)
        .expect("written bytes match committed receipt");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn download_only_writer_fails_before_bytes_or_receipt_are_published() {
    let mut app = RSpiceApp::test_instance();
    let draft = complete_publication_draft(&app);
    let writes = Rc::new(RefCell::new(Vec::new()));
    app.export_workflow_io = Box::new(PublicationIo {
        destination: PathBuf::from("download.rspiceproj"),
        reopenable: false,
        writes: Rc::clone(&writes),
    });

    let error = publish_native(&mut app, &draft)
        .expect_err("download-only backend cannot authorize a receipt");

    assert!(error.contains("cannot prove a durable"));
    assert!(writes.borrow().is_empty());
    assert!(
        app.state
            .model
            .workspace
            .content
            .project
            .library_publications()
            .is_empty()
    );
}

#[test]
fn rollback_artifact_validation_is_exact_and_fail_closed() {
    let mut app = RSpiceApp::test_instance();
    let candidate = prepare_project_library_publication(
        &app.state,
        "analog-core-1.0.0",
        "release-engineer@example.test",
        "organization-release-authority",
        "Qualified library handoff",
    )
    .expect("candidate prepares");
    let bytes = candidate.artifact_bytes().to_vec();
    let receipt = commit_project_library_publication(&mut app.state, candidate)
        .expect("receipt commits after test-owned durable boundary");

    validate_rollback_artifact(&app.state, receipt.publication_id(), &bytes)
        .expect("exact bytes pass");
    let mut tampered = bytes;
    tampered[0] ^= 0x01;
    assert!(
        validate_rollback_artifact(&app.state, receipt.publication_id(), &tampered)
            .expect_err("tampered bytes fail")
            .contains("does not match")
    );
    assert!(
        validate_rollback_artifact(&app.state, uuid::Uuid::new_v4(), &tampered)
            .expect_err("foreign receipt fails")
            .contains("no longer retained")
    );
}

#[test]
fn governance_dialogs_render_accessible_fields_at_phone_width() {
    let mut app = RSpiceApp::test_instance();
    let candidate = prepare_project_library_publication(
        &app.state,
        "analog-core-1.0.0",
        "release-engineer@example.test",
        "organization-release-authority",
        "Qualified library handoff",
    )
    .expect("candidate prepares");
    commit_project_library_publication(&mut app.state, candidate).expect("fixture receipt commits");
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    open_publication(&ctx, &app.state.model);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(390.0, 844.0),
            )),
            ..Default::default()
        },
        |ctx| show(ctx, &mut app),
    );
    let labels = output
        .platform_output
        .accesskit_update
        .expect("publication dialog exposes AccessKit")
        .nodes
        .into_iter()
        .filter_map(|(_, node)| node.label().map(str::to_owned))
        .collect::<Vec<_>>();
    assert!(labels.iter().any(|label| label == "Publication label"));
    assert!(labels.iter().any(|label| label == "Actor identity"));
    assert!(labels.iter().any(|label| label == "Authority identity"));

    open_rollback(&ctx, &app.state.model);
    let output = ctx.run_ui(egui::RawInput::default(), |ctx| show(ctx, &mut app));
    let labels = output
        .platform_output
        .accesskit_update
        .expect("rollback dialog exposes AccessKit")
        .nodes
        .into_iter()
        .filter_map(|(_, node)| node.label().map(str::to_owned))
        .collect::<Vec<_>>();
    assert!(labels.iter().any(|label| label.contains("Choose artifact")));
    assert!(
        labels
            .iter()
            .any(|label| label.contains("Restore publication"))
    );
}

#[test]
fn publication_filename_is_platform_safe_and_typed() {
    let app = RSpiceApp::test_instance();
    let filename = publication_filename(&app.state, "analog / core 1.0");
    assert!(filename.ends_with(".rspiceproj"));
    assert!(!filename.contains('/'));
    assert_eq!(safe_file_stem("***"), "project-library");
}
