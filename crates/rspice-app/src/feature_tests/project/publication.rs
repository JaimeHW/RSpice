//! Publish project and Restore project from publication through the whole
//! application: the durable writer's proof before a publication is recorded,
//! the published file checked byte for byte, the checkpoint a restore takes
//! first, and the dialogs drawn at phone width.

#[cfg(not(target_arch = "wasm32"))]
use std::cell::RefCell;
#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};
#[cfg(not(target_arch = "wasm32"))]
use std::rc::Rc;

use crate::workbench::RSpiceApp;
use crate::workbench::shell_host::project::library_publication::commit_project_library_publication;
use crate::workbench::shell_host::project::library_publication::prepare_project_library_publication;
use crate::workbench::shell_host::project::library_publication::publication_filename;
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::shell_host::project::library_publication::publish_native;
use crate::workbench::shell_host::project::library_publication::show;
use crate::workbench::shell_host::project::library_publication::validate_restore_file;
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_project::workbench::surfaces::project::library_publication::PublicationAction;
use rspice_app_project::workbench::surfaces::project::library_publication::{
    PublicationDraft, PublicationReadiness, audit_text_valid, open_publication, open_restore,
    safe_file_stem,
};
use rspice_app_services::workbench::workflows::export_workflow::SaveDialogConfig;
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_services::workbench::workflows::export_workflow::{
    ExportWorkflowIo, ObservedExportDestination,
};

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
        _dataset: &rspice_app_services::io::WaveformDataset,
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
fn publication_and_restore_fields_are_strict_audit_text() {
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
    validate_restore_file(&app.state, receipt.publication_id(), bytes)
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

    assert!(error.contains("cannot save a file that can be opened again"));
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
fn a_restore_file_is_checked_byte_for_byte() {
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

    validate_restore_file(&app.state, receipt.publication_id(), &bytes).expect("exact bytes pass");
    let mut tampered = bytes;
    tampered[0] ^= 0x01;
    assert_eq!(
        validate_restore_file(&app.state, receipt.publication_id(), &tampered)
            .expect_err("tampered bytes fail"),
        "The chosen file is not the file published as #0001."
    );
    assert_eq!(
        validate_restore_file(&app.state, uuid::Uuid::new_v4(), &tampered)
            .expect_err("foreign receipt fails"),
        "The project no longer has that publication."
    );
}

#[test]
fn the_dialogs_render_accessible_fields_at_phone_width() {
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
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
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
    for field in ["Label", "Published by", "Approved by", "Reason"] {
        assert!(
            labels.iter().any(|label| label == field),
            "{field}: {labels:?}"
        );
    }

    open_restore(&ctx, &app.state.model);
    let output = ctx.run_ui(egui::RawInput::default(), |ctx| show(ctx, &mut app));
    let labels = output
        .platform_output
        .accesskit_update
        .expect("restore dialog exposes AccessKit")
        .nodes
        .into_iter()
        .filter_map(|(_, node)| node.label().map(str::to_owned))
        .collect::<Vec<_>>();
    assert!(labels.iter().any(|label| label.contains("Choose file")));
    assert!(labels.iter().any(|label| label == "Restore project"));
    for field in ["Restored by", "Approved by", "Reason"] {
        assert!(
            labels.iter().any(|label| label == field),
            "{field}: {labels:?}"
        );
    }
}

#[test]
fn publication_filename_is_platform_safe_and_typed() {
    let app = RSpiceApp::test_instance();
    let filename = publication_filename(&app.state, "analog / core 1.0");
    assert!(filename.ends_with(".rspiceproj"));
    assert!(!filename.contains('/'));
    assert_eq!(safe_file_stem("***"), "project");
}

/// Publish project and Restore project from publication are disabled, each
/// with the reason for the state it is in: a browser without the file
/// pickers cannot do either, and a project with no publication has nothing
/// to restore.
#[test]
fn publish_and_restore_say_why_they_cannot_run() {
    let ready = PublicationReadiness {
        project_open: true,
        file_pickers: true,
        read_only: false,
        simulation_running: false,
        has_publications: true,
    };
    assert_eq!(ready.publish_blocker(), None);
    assert_eq!(ready.restore_blocker(), None);
    let browser = PublicationReadiness {
        file_pickers: false,
        ..ready
    };
    assert_eq!(
        browser.publish_blocker(),
        Some("This browser cannot save a file to disk; publish from the desktop app.")
    );
    assert_eq!(
        browser.restore_blocker(),
        Some("This browser cannot open a file from disk; restore from the desktop app.")
    );
    let closed = PublicationReadiness {
        project_open: false,
        ..ready
    };
    assert_eq!(
        closed.publish_blocker(),
        Some("Open a project to publish it.")
    );
    assert_eq!(
        closed.restore_blocker(),
        Some("Open a project to restore it from a publication.")
    );
    let unpublished = PublicationReadiness {
        has_publications: false,
        ..ready
    };
    assert_eq!(unpublished.publish_blocker(), None);
    assert_eq!(
        unpublished.restore_blocker(),
        Some("The project has no publications to restore from.")
    );
}

/// The publisher and approver typed once are offered again for the rest of
/// the session, in Publish and in Restore alike.
#[test]
fn the_publisher_and_approver_are_remembered_for_the_session() {
    use rspice_app_project::workbench::surfaces::project::library_publication::{
        PUBLICATION_DIALOG_ID, RESTORE_DIALOG_ID, RestoreDraft, remember_identity,
    };

    let mut app = RSpiceApp::test_instance();
    let candidate = prepare_project_library_publication(
        &app.state,
        "analog-core-1.0.0",
        "release-engineer@example.test",
        "organization-release-authority",
        "Qualified library handoff",
    )
    .expect("candidate prepares");
    commit_project_library_publication(&mut app.state, candidate).expect("receipt commits");
    let ctx = egui::Context::default();
    remember_identity(&ctx, " Ada Lovelace ", "Design review");

    open_publication(&ctx, &app.state.model);
    let publish = ctx
        .data(|data| data.get_temp::<PublicationDraft>(egui::Id::new(PUBLICATION_DIALOG_ID)))
        .expect("Publish project is open");
    assert_eq!(
        (publish.actor_id.as_str(), publish.authority_id.as_str()),
        ("Ada Lovelace", "Design review")
    );
    open_restore(&ctx, &app.state.model);
    let restore = ctx
        .data(|data| data.get_temp::<RestoreDraft>(egui::Id::new(RESTORE_DIALOG_ID)))
        .expect("Restore project from publication is open");
    assert_eq!(
        (restore.actor_id.as_str(), restore.authority_id.as_str()),
        ("Ada Lovelace", "Design review")
    );
    assert!(restore.reason.is_empty(), "a reason is the restore's own");
}

/// Restore project from publication takes a checkpoint of the project as it
/// stands, then puts the published project back: the library added since is
/// gone, and the checkpoint holding it is on the Recovery page's list.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_restore_takes_a_checkpoint_first_and_says_what_it_replaces() {
    use crate::workbench::shell_host::project::library_publication::{
        restore_replaces, restore_with_checkpoint,
    };
    use rspice_app_project::workbench::surfaces::project::library_publication::{
        RESTORE_DIALOG_ID, RestoreDraft,
    };
    use rspice_app_services::workbench::lifecycle::project_checkpoint::{
        ProjectCheckpointReason, list,
    };
    use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;

    let root = unique_temp_dir("publication-restore-checkpoint");
    let mut app = RSpiceApp::test_instance();
    assert!(
        app.state
            .save_project_to_path(&root.join("published.rspiceproj"))
    );
    let candidate = prepare_project_library_publication(
        &app.state,
        "analog-core-1.0.0",
        "release-engineer@example.test",
        "organization-release-authority",
        "Qualified library handoff",
    )
    .expect("candidate prepares");
    let bytes = candidate.artifact_bytes().to_vec();
    let receipt =
        commit_project_library_publication(&mut app.state, candidate).expect("receipt commits");
    app.state
        .model
        .library_manager
        .add_library(rspice_app_state::state::Library::new("added_since"));
    let revision_before = app.state.model.workspace.content.project.revision().get();

    let replaces = restore_replaces(&app.state, &receipt);
    assert_eq!(replaces.newer_runs, 0, "no run was recorded since");
    assert_eq!(
        replaces.model_libraries,
        app.state.model.model_library_manager.library_count()
    );

    let ctx = egui::Context::default();
    open_restore(&ctx, &app.state.model);
    let mut draft = ctx
        .data(|data| data.get_temp::<RestoreDraft>(egui::Id::new(RESTORE_DIALOG_ID)))
        .expect("Restore project from publication is open");
    draft.actor_id = "Ada Lovelace".to_owned();
    draft.authority_id = "Design review".to_owned();
    draft.reason = "Back to the handoff".to_owned();
    draft.artifact_name = Some("published.rspiceproj".to_owned());
    draft.artifact_bytes = Some(bytes);

    let message = restore_with_checkpoint(&mut app, &draft).expect("the restore runs");
    assert!(
        message.contains("Before restore from publication"),
        "{message}"
    );
    assert!(
        app.state
            .model
            .library_manager
            .get_library("added_since")
            .is_none(),
        "the published project is back"
    );
    let catalog = list(&app.state.model).expect("the checkpoints list");
    let checkpoint = catalog
        .checkpoints
        .iter()
        .find(|checkpoint| checkpoint.reason() == ProjectCheckpointReason::PublicationRestore)
        .expect("the restore took a checkpoint first");
    assert_eq!(checkpoint.project_revision(), revision_before);
    let _ = std::fs::remove_dir_all(root);
}
