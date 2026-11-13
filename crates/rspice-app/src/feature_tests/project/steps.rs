//! The project feature's own steps, run in its context lent from RSpiceApp.

use crate::state::{Cell, Library, View, ViewType};
use crate::workbench::app::RSpiceApp;
use crate::workbench::framework::dialogs::DialogKey;
use crate::workbench::framework::feature::{CommandSet, FeatureRegistry};
use crate::workbench::framework::requests::Drain;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::{ProjectCommand, ProjectRequest};

use super::with_cx;

#[test]
fn design_management_route_hook_opens_and_dismisses_the_transient_dialog_owner() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .workbench
        .replace_route(
            crate::workbench::SurfaceRoute::surface(crate::workbench::SurfaceId::DesignManagement),
            crate::workbench::RouteTransitionSource::Restore,
        )
        .expect("Design Management route is executable");
    assert!(!app.state.project.design_management.open);

    with_cx(
        &mut app,
        ProjectFeature::synchronize_design_management_route,
    );
    assert!(app.state.project.design_management.open);

    app.state
        .workbench
        .replace_route(
            crate::workbench::SurfaceRoute::surface(crate::workbench::SurfaceId::Design),
            crate::workbench::RouteTransitionSource::BrowserPop,
        )
        .expect("Design workspace route is executable");
    with_cx(
        &mut app,
        ProjectFeature::synchronize_design_management_route,
    );
    assert!(!app.state.project.design_management.open);
}

/// An open project with the RC low-pass drawn in front of the Design
/// workspace, saved to `file` when one is given, and Check and save opened
/// on it through its command, in the project feature's context.
fn checking(file: Option<&std::path::Path>) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);
    rspice_app_state::workbench::examples::load_example(
        "RC Lowpass Filter",
        &mut app.state.model.schematic,
    );
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(file) = file {
        assert!(
            app.state.save_project_to_path(file),
            "the project is saved to its own file"
        );
    }
    with_cx(&mut app, |cx| ProjectCommand::CheckAndSave.execute(cx));
    assert!(
        app.state.project.check_and_save.open,
        "Check and save opens"
    );
    app
}

/// The report Check and save opens on in the project feature's context is
/// the one the whole application state gives: the same findings, under the
/// same receipt.
#[test]
fn check_and_save_opens_in_the_project_context_on_the_report_the_application_gives() {
    let app = checking(None);
    let opened = app
        .state
        .project
        .check_and_save
        .report
        .clone()
        .expect("the dialog opens on a report");
    let whole = crate::workbench::app::dialogs::check_and_save_validation::CheckAndSaveValidationReport::collect_in(&app.state)
        .expect("the whole state gives a report");
    assert_eq!(opened, whole);
}

/// Where a commit left the dialog, the validated-revision journal and the log.
fn committed(app: &RSpiceApp) -> impl PartialEq + std::fmt::Debug {
    let dialog = &app.state.project.check_and_save;
    (
        (dialog.open, dialog.validation_error.clone()),
        app.state
            .model
            .schematic
            .document()
            .validated_revisions
            .records()
            .iter()
            .map(|revision| {
                (
                    revision.sequence(),
                    revision.is_accepted_baseline(),
                    revision.revision_note().to_owned(),
                    revision.view_identity().to_owned(),
                )
            })
            .collect::<Vec<_>>(),
        app.state
            .model
            .log_buffer
            .entries()
            .map(|entry| entry.message.clone())
            .collect::<Vec<_>>(),
    )
}

/// A save Check and save hands the shell ends where the save carried out in
/// place did, on the dialog, the journal and the log: saved, which closes the
/// form, and refused for want of a revision note.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_commit_handed_to_the_shell_ends_where_the_commit_in_place_did() {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time after epoch")
        .as_nanos();
    let file = std::env::temp_dir().join(format!(
        "rspice-check-and-save-{}-{unique}.rspiceproj",
        std::process::id()
    ));
    for (note, saves) in [("Checked the low-pass corner.", true), ("", false)] {
        let mut in_place = checking(Some(&file));
        in_place.state.project.check_and_save.revision_note = note.to_owned();
        in_place.commit_check_and_save();

        let mut handed = checking(Some(&file));
        handed.state.project.check_and_save.revision_note = note.to_owned();
        with_cx(&mut handed, |cx| {
            cx.wb.outbox.push(ProjectRequest::CommitCheckAndSave);
        });
        handed.after_contribution();

        assert_eq!(committed(&handed), committed(&in_place), "{note:?}");
        assert_eq!(handed.state.project.check_and_save.open, !saves, "{note:?}");
    }
    let _ = std::fs::remove_file(&file);
}

/// An open project with a writable library `work` holding the cell `amp`.
fn with_a_writable_cell() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    library.add_cell(amp);
    app.state.model.library_manager.add_library(library);
    app
}

/// The project's dialog step `key`, drawn for two frames through what the
/// feature registers for it while it is open, raises nothing: the outbox is
/// as it was, so the answer the registry runs after the step finds nothing
/// to carry out.
fn assert_the_step_raises_nothing(app: &mut RSpiceApp, key: &'static str) {
    let registry = FeatureRegistry::<ProjectFeature>::build();
    let dialog = registry
        .dialog(DialogKey(key))
        .unwrap_or_else(|| panic!("the project feature registers `{key}`"));
    assert!((dialog.is_open)(&app.state.project), "`{key}` is open");
    assert!(
        !app.state
            .workbench
            .outbox
            .waits_at(Drain::AfterContribution),
        "nothing waits for the answer before `{key}` draws"
    );
    let before = format!("{:?}", app.state.workbench.outbox);
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    for _ in 0..2 {
        let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
            with_cx(app, |cx| (dialog.render)(cx, ctx));
        });
    }
    assert!((dialog.is_open)(&app.state.project), "`{key}` stays open");
    assert!(
        !app.state
            .workbench
            .outbox
            .waits_at(Drain::AfterContribution),
        "`{key}` leaves nothing for the answer after it"
    );
    assert_eq!(
        format!("{:?}", app.state.workbench.outbox),
        before,
        "`{key}` raises nothing"
    );
}

#[test]
fn revision_history_raises_nothing_for_the_answer_after_it() {
    let mut app = with_a_writable_cell();
    with_cx(&mut app, |cx| ProjectCommand::RevisionHistory.execute(cx));
    assert_the_step_raises_nothing(&mut app, "project.project_revision_history");
}

#[test]
fn copy_cell_raises_nothing_for_the_answer_after_it() {
    let mut app = with_a_writable_cell();
    app.state
        .project
        .open_copy_cell_dialog(&app.state.model, "work", "amp")
        .expect("a writable cell opens Copy Cell");
    assert_the_step_raises_nothing(&mut app, "project.copy_cell");
}

#[test]
fn the_deletion_review_raises_nothing_for_the_answer_after_it() {
    let mut app = with_a_writable_cell();
    app.state
        .project
        .open_library_cell_deletion_review(&app.state.model, "work", "amp")
        .expect("a writable cell opens the deletion review");
    assert_the_step_raises_nothing(&mut app, "project.library_deletion_review");
}

#[test]
fn new_library_raises_nothing_for_the_answer_after_it() {
    let mut app = with_a_writable_cell();
    app.state.project.open_new_library_dialog(&app.state.model);
    assert_the_step_raises_nothing(&mut app, "project.new_library");
}
