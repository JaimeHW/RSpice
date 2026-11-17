//! The project Library page through the whole application: its intents
//! carried out as the page carries them out, routing to the workspace that
//! edits a view, opening the transactions they name, and publishing the
//! library through the whole state.

use egui::{Rect, vec2};

use crate::workbench::RSpiceApp;
use crate::workbench::features;
use crate::workbench::shell_host::project::library_publication::commit_project_library_publication;
use crate::workbench::shell_host::project::library_publication::prepare_project_library_publication;
use crate::workbench::shell_host::project::library_publication::restore_project_from_publication;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_project::workbench::surfaces::project::library::{
    AUDIT_REPORT_ID, LibraryIntent, apply_library_intent, icon_for_view, library_context,
};
use rspice_app_state::state::{Cell, CellViewRef, Library, View, ViewType};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_ui_kit::panels::WorkbenchIcon;

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
        rspice_app_state::veriloga_library::BUILTIN_VERILOGA_MODELS[0]
            .registry_name
            .to_owned(),
    );
    builtin.file = Some("a_source_file_whose_name_fills_the_whole_of_the_strip.va".to_owned());
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
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
            egui::Shape::Text(text) if text.galley.text() == "read-only" => {
                Some(text.galley.rect.translate(text.pos.to_vec2()))
            }
            _ => None,
        })
        .expect("the access value is drawn");
    assert!(access.right() <= PAGE_WIDTH, "{access:?}");
}

/// Every kind of view the Library page lists opens the view that was asked
/// for: a real view of each kind, in a cell of its own, is the view in front
/// afterwards. Source and document views are cell views and open in Design,
/// which edits a Verilog-A module and states what the others are bound to; a
/// configuration opens on the Project workspace's Configuration page.
#[test]
fn every_kind_of_view_opens_the_view_that_was_asked_for() {
    assert_eq!(icon_for_view(ViewType::VerilogA), WorkbenchIcon::Code);
    assert_eq!(icon_for_view(ViewType::Spice), WorkbenchIcon::Code);
    assert_eq!(icon_for_view(ViewType::Config), WorkbenchIcon::Sliders);

    for (view_type, name) in [
        (ViewType::Spice, "spice"),
        (ViewType::Verilog, "verilog"),
        (ViewType::VerilogA, "veriloga"),
        (ViewType::VerilogAms, "vams"),
        (ViewType::Document, "document"),
        (ViewType::Extracted, "extracted"),
        (ViewType::Abstract, "abstract"),
        (ViewType::Custom, "custom"),
        (ViewType::Config, "config"),
    ] {
        let mut app = RSpiceApp::test_instance();
        let mut cell = Cell::new("block");
        cell.add_view(View::new(name, view_type));
        let mut library = Library::new("kinds");
        library.add_cell(cell);
        app.state.model.library_manager.add_library(library);
        app.state.workbench.activate(Workspace::Models);
        let reference = CellViewRef::new("kinds", "block", name);
        assert_ne!(app.state.model.workspace.content.active_view, reference);

        apply_and_answer(
            &egui::Context::default(),
            &mut app,
            LibraryIntent::OpenView(reference.clone(), view_type),
        );
        let content = &app.state.model.workspace.content;
        assert_eq!(content.active_view, reference, "{view_type:?}");
        assert_eq!(content.active_view_type(), Some(view_type), "{view_type:?}");
        if view_type == ViewType::Config {
            assert_eq!(app.state.workbench.workspace, Workspace::Project);
            assert_eq!(app.state.project.project_page, ProjectPage::Configuration);
        } else {
            assert_eq!(
                app.state.workbench.workspace,
                Workspace::Design,
                "{view_type:?} opens as a Design document"
            );
        }
    }
}

/// A cell with a symbol view offers Symbol & form in Cell actions; the
/// entry selects that symbol and opens the manager on the page that edits
/// it.
#[test]
fn cell_actions_open_a_cells_symbol_in_symbol_and_form() {
    use rspice_app_project::workbench::project::workspace_state::LibraryCellviewPage;

    let mut app = RSpiceApp::test_instance();
    let mut cell = Cell::new("filter");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    cell.add_view(View::new("symbol", ViewType::Symbol));
    let mut library = Library::new("forms");
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    app.state
        .model
        .library_manager
        .select_view("forms", "filter", "schematic");
    app.state.project.project_page = ProjectPage::Library;
    app.state.workbench.activate(Workspace::Project);
    let mut replay = super::super::replay::Replay::of(app);
    press_named(&mut replay, "Cell actions");
    press_named(&mut replay, "Open in Symbol & form");

    let state = &replay.app.state;
    assert_eq!(
        state.project.library_cellview_page,
        LibraryCellviewPage::SymbolForm
    );
    assert_eq!(
        state.model.library_manager.selected_view.as_deref(),
        Some("symbol"),
        "the symbol is the one Symbol & form shows"
    );
    assert_eq!(
        state.workbench.current_route().surface_id(),
        rspice_app_workbench::workbench::routing::surface_catalog::SurfaceId::LibraryCellviewManager
    );
}

/// Library actions holds Publish project… and Restore project from
/// publication… beside the library's own actions.
#[test]
fn library_actions_hold_publish_and_restore() {
    let mut app = RSpiceApp::test_instance();
    app.state.project.project_page = ProjectPage::Library;
    app.state.workbench.activate(Workspace::Project);
    let mut replay = super::super::replay::Replay::of(app);
    let menu = press_named(&mut replay, "Library actions");
    for entry in [
        "New library\u{2026}",
        "Publish project\u{2026}",
        "Restore project from publication\u{2026}",
    ] {
        assert!(
            menu.iter().any(|control| control.label == entry),
            "{entry} is in Library actions"
        );
    }
}

/// The control named `label`, whatever its role, pressed and released at
/// its centre; what the frames after it published.
fn press_named(
    replay: &mut super::super::replay::Replay,
    label: &str,
) -> Vec<super::super::replay::Control> {
    let controls = replay.frames(Vec::new());
    let at = controls
        .iter()
        .find(|control| control.label == label)
        .unwrap_or_else(|| {
            panic!(
                "no control named {label:?}; the controls are {:?}",
                controls
                    .iter()
                    .map(|control| &control.label)
                    .collect::<Vec<_>>()
            )
        })
        .center;
    let press = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    replay.frames(vec![egui::Event::PointerMoved(at), press(true)]);
    replay.frames(vec![press(false)])
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
fn publish_and_restore_intents_open_their_dialogs() {
    let mut app = RSpiceApp::test_instance();
    let ctx = egui::Context::default();

    apply_and_answer(&ctx, &mut app, LibraryIntent::PublishProject);
    assert!(super::library_publication::publication_dialog_open(&ctx));
    assert!(!super::library_publication::restore_dialog_open(&ctx));

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
    apply_and_answer(&ctx, &mut app, LibraryIntent::RestoreFromPublication);

    assert!(!super::library_publication::publication_dialog_open(&ctx));
    assert!(super::library_publication::restore_dialog_open(&ctx));
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
        rspice_app_state::product::ContentDigest::from_bytes(sha2::Sha256::digest(&bytes).into())
    );
    let text = std::str::from_utf8(&bytes).expect("publication is UTF-8");
    let artifact = rspice_app_services::io::project_io::load_project_text(text, None)
        .expect("artifact validates");
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
    let tampered_descriptor: rspice_app_state::state::ProjectDescriptor =
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
        .preflight_project_library_mutation(
            rspice_app_state::state::ProjectLibraryMutation::CreateCell {
                library: writable_library.clone(),
                cell: "post_publication_change".to_owned(),
            },
        )
        .expect("post-publication mutation preflights");
    app.state
        .model
        .library_manager
        .get_library_mut(&writable_library)
        .expect("writable library")
        .add_cell(rspice_app_state::state::Cell::new(
            "post_publication_change",
        ));
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
    let tampered_result = restore_project_from_publication(
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

    restore_project_from_publication(
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
        Some(rspice_app_state::state::ProjectLibraryMutation::RollbackPublication {
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
        .preflight_project_library_mutation(
            rspice_app_state::state::ProjectLibraryMutation::CreateCell {
                library: writable_library.clone(),
                cell: "intervening_change".to_owned(),
            },
        )
        .expect("intervening mutation preflights");
    app.state
        .model
        .library_manager
        .get_library_mut(&writable_library)
        .expect("writable library")
        .add_cell(rspice_app_state::state::Cell::new("intervening_change"));
    app.state
        .model
        .publish_project_library_mutation(prepared_mutation);
    let revision_after_mutation = app.state.model.workspace.content.project.revision();

    let error = commit_project_library_publication(&mut app.state, candidate)
        .expect_err("stale publication cannot commit");

    assert_eq!(
        error,
        "The project changed while it was being published; publish it again."
    );
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

/// In the library browser a click on a view selects it, and a double click
/// opens it where it is edited: a symbol view in the symbol editor, in
/// Design, as a library manager opens a cell view.
#[test]
fn a_double_click_on_a_symbol_view_opens_it_in_the_symbol_editor() {
    use egui::accesskit::Role;

    use super::super::replay::Replay;

    let mut app = RSpiceApp::test_instance();
    let mut view = View::new("symbol", ViewType::Symbol);
    rspice_design::symbol::generated::store_generated_symbol(
        &mut view,
        &[rspice_app_state::state::PortSpec {
            name: "a".to_owned(),
            direction: rspice_app_state::state::PortDirection::InOut,
        }],
        |_| None,
    )
    .expect("the symbol stores");
    let mut cell = Cell::new("amp");
    cell.add_view(view);
    let mut library = Library::new("authoring");
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    app.state
        .model
        .library_manager
        .select_cell("authoring", "amp");
    app.state.project.project_page = ProjectPage::Library;
    app.state.workbench.activate(Workspace::Project);
    let mut replay = Replay::of(app);
    replay.frames(Vec::new());

    replay.click("symbol", Role::Button);
    let state = &replay.app.state;
    assert_eq!(
        state.workbench.workspace,
        Workspace::Project,
        "a click opens nothing"
    );
    assert_eq!(
        state.model.library_manager.selected_view.as_deref(),
        Some("symbol"),
        "a click selects the view"
    );

    let at = super::super::replay::only(&replay.frames(Vec::new()), "symbol", Role::Button).center;
    let press = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    replay.frame(vec![egui::Event::PointerMoved(at), press(true)]);
    replay.frame(vec![press(false)]);
    replay.frame(vec![press(true)]);
    replay.frame(vec![press(false)]);
    replay.frames(Vec::new());
    let state = &replay.app.state;
    assert_eq!(state.workbench.workspace, Workspace::Design);
    assert_eq!(
        state.model.workspace.content.active_view,
        CellViewRef::new("authoring", "amp", "symbol"),
        "the double click opens the symbol"
    );
    assert!(
        state.model.load_active_symbol_document().is_ok(),
        "the symbol editor has it in front"
    );
}

/// A library clicked in the rail is the one the page shows, even when the
/// built-in library is open: the rail selects the way the page's own Library
/// column does. The rail lists the built-in library as the column does.
#[test]
fn a_library_clicked_in_the_rail_leaves_the_built_in_library() {
    use super::super::replay::Replay;
    use rspice_app_state::veriloga_library::BUILTIN_VERILOGA_LIBRARY;

    let mut app = RSpiceApp::test_instance();
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    let mut library = Library::new("authoring");
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    app.state.project.project_page = ProjectPage::Library;
    app.state.workbench.activate(Workspace::Project);
    let mut replay = Replay::of(app);
    replay.frames(Vec::new());
    let ctx = replay.ctx().clone();
    apply_library_intent(
        &ctx,
        &mut features::cx(&mut replay.app),
        LibraryIntent::BuiltinModels(
            rspice_app_project::workbench::surfaces::project::library::builtin_models::BuiltinModelsIntent::ShowLibrary,
        ),
    );

    let controls = replay.frames(Vec::new());
    let named = |label: &str| {
        let mut found = controls
            .iter()
            .filter(|control| control.label == label)
            .collect::<Vec<_>>();
        found.sort_by(|left, right| left.center.x.total_cmp(&right.center.x));
        found
    };
    // The rail announces a row's meta after its name; the column does not.
    assert_eq!(
        named(&format!("{BUILTIN_VERILOGA_LIBRARY}, read-only")).len(),
        1,
        "the rail lists the built-in library"
    );
    assert_eq!(
        named(BUILTIN_VERILOGA_LIBRARY).len(),
        1,
        "the Library column lists the built-in library"
    );
    let authoring = named("authoring");
    assert_eq!(authoring.len(), 2, "the rail and the column list it");
    let rail_row = authoring[0].center;
    let press = |pressed| egui::Event::PointerButton {
        pos: rail_row,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    replay.frames(vec![egui::Event::PointerMoved(rail_row), press(true)]);
    replay.frames(vec![press(false)]);

    let state = &replay.app.state;
    assert!(
        !state.project.builtin_models.open,
        "the page leaves the built-in library"
    );
    assert_eq!(
        state.model.library_manager.selected_library.as_deref(),
        Some("authoring")
    );
}

/// Every string one whole application frame painted, one per line.
fn painted_frame(app: &mut RSpiceApp) -> String {
    fn collect(shape: &egui::epaint::Shape, text: &mut String) {
        match shape {
            egui::epaint::Shape::Text(shape) => {
                text.push_str(shape.galley.text());
                text.push('\n');
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, text);
                }
            }
            _ => {}
        }
    }
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut output = None;
    for _ in 0..3 {
        output = Some(ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1600.0, 900.0))),
                ..Default::default()
            },
            |ui| {
                let ctx = ui.ctx().clone();
                app.prepare_frame(&ctx);
                app.render_frame_chrome(ui);
                app.render_frame_dialogs(&ctx);
            },
        ));
    }
    let mut text = String::new();
    for clipped in &output.expect("three frames ran").shapes {
        collect(&clipped.shape, &mut text);
    }
    text
}

/// A library page over `work`, whose cells `amp` (with a schematic) and
/// `bias` (with none) are in a writable library beside the read-only `ref`,
/// with `amp` selected.
fn filtered_library_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    let mut amp = Cell::new("amp");
    let mut schematic = View::new("schematic", ViewType::Schematic);
    schematic.modified = true;
    amp.add_view(schematic);
    amp.add_view(View::new("symbol", ViewType::Symbol));
    let mut work = Library::new("work");
    work.add_cell(amp);
    work.add_cell(Cell::new("bias"));
    app.state.model.library_manager.add_library(work);
    let mut reference = Library::new("ref");
    reference.read_only = true;
    app.state.model.library_manager.add_library(reference);
    app.state
        .model
        .library_manager
        .select_view("work", "amp", "schematic");
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Library;
    app
}

/// The rows state only the exception: a read-only library and a modified
/// view are marked; a writable library and a saved view say nothing, and the
/// band does not repeat the library's access.
#[test]
fn library_rows_mark_only_a_read_only_library_and_a_modified_view() {
    let mut app = filtered_library_app();
    let text = painted_frame(&mut app);
    let lines = text.lines().collect::<Vec<_>>();
    for retired in ["RW", "RO", "saved", "writable", "working changes"] {
        assert!(!lines.contains(&retired), "{retired:?} in:\n{text}");
    }
    assert!(lines.contains(&"read-only"), "{text}");
    assert!(lines.contains(&"modified"), "{text}");
}

/// A filter that hides the selected cell empties the View column and takes
/// the cell's actions off the band; the Cell column counts "1 of 2" and says
/// when nothing matches. A new project starts with no filter.
#[test]
fn a_filter_states_what_it_hides_and_never_acts_on_a_hidden_cell() {
    let mut app = filtered_library_app();
    let shown = painted_frame(&mut app);
    assert!(shown.contains("Cell actions"), "{shown}");

    app.state.project.project_library_filter = "bias".to_owned();
    let hidden = painted_frame(&mut app);
    let lines = hidden.lines().collect::<Vec<_>>();
    assert!(lines.contains(&"1 of 2"), "{hidden}");
    assert!(lines.contains(&"No cell selected"), "{hidden}");
    assert!(!hidden.contains("Cell actions"), "{hidden}");
    assert!(!hidden.contains("Open schematic"), "{hidden}");

    app.state.project.project_library_filter = "zzz".to_owned();
    let none = painted_frame(&mut app);
    assert!(
        none.contains("No cells match \u{201c}zzz\u{201d}"),
        "{none}"
    );

    app.state.project.on_project_changed();
    assert!(app.state.project.project_library_filter.is_empty());
}
