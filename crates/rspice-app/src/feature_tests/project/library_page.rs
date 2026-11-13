//! The project Library page through the whole application: its intents
//! carried out as the page carries them out, routing to the workspace that
//! edits a view, opening the transactions they name, and publishing the
//! library through the whole state.

use egui::{Rect, vec2};

use crate::state::{Cell, CellViewRef, Library, View, ViewType};
use crate::workbench::RSpiceApp;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::features;
use crate::workbench::project::workspace_state::ProjectPage;
use crate::workbench::state::Workspace;
use crate::workbench::surfaces::project::library::{
    AUDIT_REPORT_ID, LibraryIntent, apply_library_intent, icon_for_view, library_context,
};
use crate::workbench::surfaces::project::library_publication::{
    commit_project_library_publication, prepare_project_library_publication,
    rollback_project_library_publication,
};

/// Carry `intent` out as the page does, then answer what it raised, as
/// the shell does once the page returns.
fn apply_and_answer(ctx: &egui::Context, app: &mut RSpiceApp, intent: LibraryIntent) {
    apply_library_intent(ctx, &mut features::cx(app), intent);
    app.after_contribution();
}

/// The selection is the one thing in the strip whose length nobody chose:
/// it is cut, and Access stays on the page.
#[test]
fn a_long_selection_is_cut_so_access_stays_on_the_page() {
    const PAGE_WIDTH: f32 = 600.0;
    let mut app = RSpiceApp::test_instance();
    let builtin = &mut app.state.project.builtin_models;
    builtin.open = true;
    builtin.model = Some(
        rspice_app_models::veriloga_library::BUILTIN_VERILOGA_MODELS[0]
            .registry_name
            .to_owned(),
    );
    builtin.file = Some("a_source_file_whose_name_fills_the_whole_of_the_strip.va".to_owned());
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                egui::Pos2::ZERO,
                vec2(PAGE_WIDTH, 100.0),
            )),
            ..Default::default()
        },
        |ui| library_context(ui, &mut features::cx(&mut app)),
    );
    let access = output
        .shapes
        .iter()
        .find_map(|clipped| match &clipped.shape {
            egui::Shape::Text(text) if text.galley.text() == "read only" => {
                Some(text.galley.rect.translate(text.pos.to_vec2()))
            }
            _ => None,
        })
        .expect("the access value is drawn");
    assert!(access.right() <= PAGE_WIDTH, "{access:?}");
}

/// Source views are cell views: opening one lands in Design, which edits a
/// Verilog-A module and states what a SPICE or Verilog view is bound to.
/// The Netlist workspace they used to open shows the project's deck,
/// which is not the view that was asked for.
#[test]
fn source_views_route_to_the_design_workspace() {
    assert_eq!(icon_for_view(ViewType::VerilogA), WorkbenchIcon::Code);
    assert_eq!(icon_for_view(ViewType::Spice), WorkbenchIcon::Code);
    assert_eq!(icon_for_view(ViewType::Config), WorkbenchIcon::Sliders);

    for view_type in [ViewType::Spice, ViewType::Verilog, ViewType::VerilogA] {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.activate(Workspace::Models);
        let reference = app.state.model.workspace.content.active_view.clone();
        apply_and_answer(
            &egui::Context::default(),
            &mut app,
            LibraryIntent::OpenView(reference, view_type),
        );
        assert_eq!(
            app.state.workbench.workspace,
            Workspace::Design,
            "{view_type:?} opens as a Design document"
        );
    }
}

#[test]
fn configuration_views_route_to_the_project_configuration_page() {
    let mut app = RSpiceApp::test_instance();
    let reference = app.state.model.workspace.content.active_view.clone();
    apply_and_answer(
        &egui::Context::default(),
        &mut app,
        LibraryIntent::OpenView(reference, ViewType::Config),
    );
    assert_eq!(app.state.workbench.workspace, Workspace::Project);
    assert_eq!(app.state.project.project_page, ProjectPage::Configuration);
}

#[test]
fn cell_action_intents_open_exact_validated_transactions() {
    let mut app = RSpiceApp::test_instance();
    let mut library = Library::new("intent_test");
    let mut cell = Cell::new("filter");
    cell.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    let ctx = egui::Context::default();

    apply_and_answer(
        &ctx,
        &mut app,
        LibraryIntent::NewView {
            library: "intent_test".to_owned(),
            cell: "filter".to_owned(),
        },
    );
    assert!(app.state.project.new_view_dialog);
    assert_eq!(app.state.project.new_view_library, "intent_test");
    assert_eq!(app.state.project.new_view_cell, "filter");

    apply_and_answer(
        &ctx,
        &mut app,
        LibraryIntent::CopyCell {
            library: "intent_test".to_owned(),
            cell: "filter".to_owned(),
        },
    );
    assert!(app.state.project.copy_cell_dialog);
    assert_eq!(app.state.project.copy_cell_source_library, "intent_test");
    assert_eq!(app.state.project.copy_cell_source_cell, "filter");

    apply_and_answer(
        &ctx,
        &mut app,
        LibraryIntent::RenameCell {
            library: "intent_test".to_owned(),
            cell: "filter".to_owned(),
        },
    );
    assert!(app.state.project.rename_cell_dialog);
    assert_eq!(app.state.project.rename_cell_library, "intent_test");
    assert_eq!(app.state.project.rename_cell_current, "filter");

    apply_and_answer(
        &ctx,
        &mut app,
        LibraryIntent::DeleteView(CellViewRef::new("intent_test", "filter", "symbol")),
    );
    assert_eq!(
        app.state
            .project
            .library_deletion_review
            .target
            .as_ref()
            .map(|target| target.display_path()),
        Some("intent_test/filter/symbol".to_owned())
    );
    app.state.project.library_deletion_review.close();

    apply_and_answer(
        &ctx,
        &mut app,
        LibraryIntent::DeleteCell {
            library: "intent_test".to_owned(),
            cell: "filter".to_owned(),
        },
    );
    assert_eq!(
        app.state
            .project
            .library_deletion_review
            .target
            .as_ref()
            .map(|target| target.display_path()),
        Some("intent_test/filter".to_owned())
    );

    apply_and_answer(
        &ctx,
        &mut app,
        LibraryIntent::RenameView(CellViewRef::new("intent_test", "filter", "symbol")),
    );
    assert!(app.state.project.rename_view_dialog);
    assert_eq!(app.state.project.rename_view_library, "intent_test");
    assert_eq!(app.state.project.rename_view_cell, "filter");
    assert_eq!(app.state.project.rename_view_current, "symbol");

    apply_and_answer(&ctx, &mut app, LibraryIntent::NewLibrary);
    assert!(app.state.project.new_library_dialog);

    apply_and_answer(
        &ctx,
        &mut app,
        LibraryIntent::RenameLibrary("intent_test".to_owned()),
    );
    assert!(app.state.project.rename_library_dialog);
    assert_eq!(app.state.project.rename_library_current, "intent_test");

    apply_and_answer(
        &ctx,
        &mut app,
        LibraryIntent::DeleteLibrary("intent_test".to_owned()),
    );
    assert!(app.state.project.delete_library_dialog);
    assert_eq!(app.state.project.delete_library_target, "intent_test");
}

#[test]
fn stale_cell_action_intents_leave_all_transaction_dialogs_closed() {
    let mut app = RSpiceApp::test_instance();
    let ctx = egui::Context::default();

    for intent in [
        LibraryIntent::NewView {
            library: "missing".to_owned(),
            cell: "filter".to_owned(),
        },
        LibraryIntent::CopyCell {
            library: "missing".to_owned(),
            cell: "filter".to_owned(),
        },
        LibraryIntent::RenameCell {
            library: "missing".to_owned(),
            cell: "filter".to_owned(),
        },
        LibraryIntent::DeleteCell {
            library: "missing".to_owned(),
            cell: "filter".to_owned(),
        },
        LibraryIntent::DeleteView(CellViewRef::new("missing", "filter", "symbol")),
        LibraryIntent::RenameView(CellViewRef::new("missing", "filter", "symbol")),
        LibraryIntent::RenameLibrary("missing".to_owned()),
        LibraryIntent::DeleteLibrary("missing".to_owned()),
    ] {
        apply_and_answer(&ctx, &mut app, intent);
    }

    assert!(!app.state.project.new_view_dialog);
    assert!(!app.state.project.copy_cell_dialog);
    assert!(!app.state.project.rename_cell_dialog);
    assert!(!app.state.project.rename_view_dialog);
    assert!(!app.state.project.rename_library_dialog);
    assert!(!app.state.project.delete_library_dialog);
    assert!(app.state.project.library_deletion_review.target.is_none());
}

#[test]
fn audit_history_intent_opens_the_project_library_receipt_view() {
    let mut app = RSpiceApp::test_instance();
    let ctx = egui::Context::default();

    apply_and_answer(&ctx, &mut app, LibraryIntent::ShowAuditHistory);

    assert!(ctx.data(|data| {
        data.get_temp::<bool>(egui::Id::new(AUDIT_REPORT_ID))
            .unwrap_or(false)
    }));
}

#[test]
fn publication_and_rollback_intents_open_the_governed_transactions() {
    let mut app = RSpiceApp::test_instance();
    let ctx = egui::Context::default();

    apply_and_answer(&ctx, &mut app, LibraryIntent::PublishLibrary);
    assert!(super::library_publication::publication_dialog_open(&ctx));
    assert!(!super::library_publication::rollback_dialog_open(&ctx));

    let candidate = prepare_project_library_publication(
        &app.state,
        "governance-intent-1.0.0",
        "release-engineer@example.test",
        "organization-release-authority",
        "Exercise exact rollback reachability",
    )
    .expect("publication candidate prepares");
    commit_project_library_publication(&mut app.state, candidate)
        .expect("test-owned durable boundary commits");
    apply_and_answer(&ctx, &mut app, LibraryIntent::RollbackPublication);

    assert!(!super::library_publication::publication_dialog_open(&ctx));
    assert!(super::library_publication::rollback_dialog_open(&ctx));
}

#[test]
fn publication_exports_exact_pre_receipt_project_and_commits_one_audited_revision() {
    use sha2::Digest as _;

    let mut app = RSpiceApp::test_instance();
    let project_id = app.state.model.workspace.content.project.id();
    let project_revision = app.state.model.workspace.content.project.revision();
    let library_revision = app.state.model.library_manager.revision();

    let candidate = prepare_project_library_publication(
        &app.state,
        "analog-core-1.0.0",
        "release-engineer@example.test",
        "organization-release-authority",
        "Qualified library handoff",
    )
    .expect("publication prepares");
    let bytes = candidate.artifact_bytes().to_vec();
    let receipt = commit_project_library_publication(&mut app.state, candidate)
        .expect("durably accepted publication commits");

    assert_eq!(receipt.project_id(), project_id);
    assert_eq!(receipt.source_project_revision(), project_revision);
    assert_eq!(
        receipt.receipt_project_revision().get(),
        project_revision.get() + 1
    );
    assert_eq!(receipt.library_revision(), library_revision);
    assert_eq!(receipt.snapshot_byte_len(), bytes.len() as u64);
    assert_eq!(
        receipt.snapshot_digest(),
        crate::product::ContentDigest::from_bytes(sha2::Sha256::digest(&bytes).into())
    );
    let text = std::str::from_utf8(&bytes).expect("publication is UTF-8");
    let artifact =
        crate::io::project_io::load_project_text(text, None).expect("artifact validates");
    assert_eq!(artifact.file.workspace.project.id(), project_id);
    assert_eq!(artifact.file.workspace.project.revision(), project_revision);
    assert!(
        artifact
            .file
            .workspace
            .project
            .library_publications()
            .is_empty()
    );
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .project
            .library_publications(),
        [receipt.clone()]
    );
    assert!(app.state.model.workspace.content.project_metadata_dirty);
    let mut tampered_descriptor = serde_json::to_value(&app.state.model.workspace.content.project)
        .expect("descriptor serializes");
    tampered_descriptor["library_publications"][0]["reason"] =
        serde_json::json!("silently changed reason");
    let tampered_descriptor: crate::state::ProjectDescriptor =
        serde_json::from_value(tampered_descriptor).expect("wire shape remains valid");
    assert!(
        tampered_descriptor.validate().is_err(),
        "publication receipt content is digest sealed"
    );

    let writable_library = app
        .state
        .model
        .library_manager
        .libraries_sorted()
        .into_iter()
        .find(|library| !library.read_only)
        .expect("writable project library")
        .name
        .clone();
    let prepared = app
        .state
        .model
        .preflight_project_library_mutation(crate::state::ProjectLibraryMutation::CreateCell {
            library: writable_library.clone(),
            cell: "post_publication_change".to_owned(),
        })
        .expect("post-publication mutation preflights");
    app.state
        .model
        .library_manager
        .get_library_mut(&writable_library)
        .expect("writable library")
        .add_cell(crate::state::Cell::new("post_publication_change"));
    app.state.model.publish_project_library_mutation(prepared);
    assert!(
        app.state
            .model
            .library_manager
            .get_library(&writable_library)
            .and_then(|library| library.get_cell("post_publication_change"))
            .is_some()
    );

    let mut tampered = bytes.clone();
    let last = tampered.last_mut().expect("nonempty publication artifact");
    *last ^= 0x01;
    let revision_before_rejection = app.state.model.workspace.content.project.revision();
    let tampered_result = rollback_project_library_publication(
        &mut app.state,
        receipt.publication_id(),
        &tampered,
        "release-engineer@example.test",
        "organization-release-authority",
        "Tampered artifact must fail",
    );
    assert!(tampered_result.is_err());
    assert_eq!(
        app.state.model.workspace.content.project.revision(),
        revision_before_rejection
    );
    assert!(
        app.state
            .model
            .library_manager
            .get_library(&writable_library)
            .and_then(|library| library.get_cell("post_publication_change"))
            .is_some(),
        "rejected rollback must not partially replace the catalog"
    );

    rollback_project_library_publication(
        &mut app.state,
        receipt.publication_id(),
        &bytes,
        "release-engineer@example.test",
        "organization-release-authority",
        "Restore the qualified handoff",
    )
    .expect("exact publication rollback succeeds");
    assert!(
        app.state
            .model
            .library_manager
            .get_library(&writable_library)
            .and_then(|library| library.get_cell("post_publication_change"))
            .is_none()
    );
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .project
            .library_publications(),
        [receipt.clone()],
        "rollback retains immutable publication lineage"
    );
    assert!(matches!(
        app.state
            .model.workspace
            .content.project
            .library_mutation_audit()
            .last()
            .map(|entry| entry.mutation()),
        Some(crate::state::ProjectLibraryMutation::RollbackPublication {
            publication_id,
            actor_id,
            authority_id,
            reason,
            ..
        }) if *publication_id == receipt.publication_id()
            && actor_id == "release-engineer@example.test"
            && authority_id == "organization-release-authority"
            && reason == "Restore the qualified handoff"
    ));

    let revision_after_first = app.state.model.workspace.content.project.revision();
    let duplicate = prepare_project_library_publication(
        &app.state,
        "ANALOG-CORE-1.0.0",
        "release-engineer@example.test",
        "organization-release-authority",
        "Duplicate labels are forbidden",
    );
    assert!(duplicate.is_err());
    assert_eq!(
        app.state.model.workspace.content.project.revision(),
        revision_after_first,
        "rejected publication must not partially advance project state"
    );
}

#[test]
fn publication_commit_rejects_intervening_catalog_change_atomically() {
    let mut app = RSpiceApp::test_instance();
    let candidate = prepare_project_library_publication(
        &app.state,
        "stale-candidate-1.0.0",
        "release-engineer@example.test",
        "organization-release-authority",
        "Prepared before an intervening edit",
    )
    .expect("publication prepares");
    let writable_library = app
        .state
        .model
        .library_manager
        .libraries_sorted()
        .into_iter()
        .find(|library| !library.read_only)
        .expect("writable project library")
        .name
        .clone();
    let prepared_mutation = app
        .state
        .model
        .preflight_project_library_mutation(crate::state::ProjectLibraryMutation::CreateCell {
            library: writable_library.clone(),
            cell: "intervening_change".to_owned(),
        })
        .expect("intervening mutation preflights");
    app.state
        .model
        .library_manager
        .get_library_mut(&writable_library)
        .expect("writable library")
        .add_cell(crate::state::Cell::new("intervening_change"));
    app.state
        .model
        .publish_project_library_mutation(prepared_mutation);
    let revision_after_mutation = app.state.model.workspace.content.project.revision();

    let error = commit_project_library_publication(&mut app.state, candidate)
        .expect_err("stale publication cannot commit");

    assert!(error.contains("stale"));
    assert!(
        app.state
            .model
            .workspace
            .content
            .project
            .library_publications()
            .is_empty()
    );
    assert_eq!(
        app.state.model.workspace.content.project.revision(),
        revision_after_mutation
    );
}
