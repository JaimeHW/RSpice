//! The Overview's rows through the whole application: every row that names
//! an object opens it, by pointer and by keyboard, and the Cells table lists
//! every cell the project holds.

use egui::accesskit::{Action, ActionRequest, NodeId, Role, TreeId};
use egui::{Event, Key, Modifiers};

use super::surface::{click_announced, populated_project_app, project_frame, settled as settle};
use crate::workbench::{RSpiceApp, features};
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_project::workbench::surfaces::project::overview::{
    OverviewIntent, OverviewSnapshot,
};
use rspice_app_state::state::{
    Cell, CellViewRef, ComponentType, Library, Point, View, ViewType, workspace,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::state::workbench_session::ConsolePage;
use rspice_design::schematic::validated_revision::{
    ValidatedRevisionRequest, ValidationFindingCounts,
};

/// Every label and value `output` announces, by node.
fn announced(output: &egui::FullOutput) -> Vec<(NodeId, Role, String)> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .filter_map(|(id, node)| {
            let words = node.label().or(node.value())?;
            Some((*id, node.role(), words.to_owned()))
        })
        .collect()
}

/// The label of the row `output` announces holding every one of `parts`.
fn row_with(output: &egui::FullOutput, parts: &[&str]) -> String {
    announced(output)
        .into_iter()
        .map(|(_, _, label)| label)
        .find(|label| parts.iter().all(|part| label.contains(part)))
        .unwrap_or_else(|| {
            panic!(
                "no row announces {parts:?}: {:#?}",
                announced(output)
                    .into_iter()
                    .map(|(_, _, label)| label)
                    .collect::<Vec<_>>()
            )
        })
}

/// A library of `count` cells, `cell00` on, each with a schematic and a
/// symbol.
fn bench_library(count: usize) -> Library {
    let mut library = Library::new("bench");
    for index in 0..count {
        let mut cell = Cell::new(format!("cell{index:02}"));
        cell.add_view(View::new("schematic", ViewType::Schematic));
        cell.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(cell);
    }
    library
}

fn app_with(library: Library) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.model.library_manager.add_library(library);
    app.state.workbench.activate(Workspace::Project);
    app
}

fn key(key: Key) -> Event {
    Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    }
}

/// The Overview's snapshot as the page's frames on `ctx` keep it.
fn kept_snapshot(ctx: &egui::Context, app: &mut RSpiceApp) -> OverviewSnapshot {
    OverviewSnapshot::capture(ctx, &features::cx_with_foreign_answers(app, |_| true))
}

/// A project of forty cells lists forty, its top cell first: the table has
/// no cap and guesses no mix.
#[test]
fn a_forty_cell_project_lists_forty_cells_the_top_first() {
    let mut app = RSpiceApp::test_instance();
    let held = app.state.model.library_manager.total_cell_count();
    app.state
        .model
        .library_manager
        .add_library(bench_library(40 - held));
    let (ctx, mut app, settled) = settle(app, ProjectPage::Overview);

    let snapshot = kept_snapshot(&ctx, &mut app);
    assert_eq!(snapshot.cells.len(), 40);
    let project = &app.state.model.workspace.content.project;
    assert_eq!(
        (snapshot.cells[0].name.as_str(), snapshot.cells[0].role),
        (project.top_cell.as_str(), "Top")
    );
    let tables = announced(&settled)
        .into_iter()
        .filter(|(_, role, _)| *role == Role::Grid)
        .map(|(_, _, label)| label)
        .collect::<Vec<_>>();
    assert!(
        tables.iter().any(|label| label == "Table, 40 rows"),
        "{tables:?}"
    );
}

/// A cell's row opens its schematic: from the pointer with a click, and
/// from the keyboard with the table's own keys and Enter.
#[test]
fn a_cell_row_opens_its_schematic_by_pointer_and_by_keyboard() {
    let wanted = CellViewRef::new("bench", "cell01", workspace::DEFAULT_SCHEMATIC_VIEW);

    let (ctx, mut app, settled) = settle(app_with(bench_library(3)), ProjectPage::Overview);
    let row = row_with(&settled, &["Cell: cell01,", "Library: bench"]);
    click_announced(&ctx, &mut app, settled, &row);
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(app.state.model.workspace.content.active_view, wanted);

    let (ctx, mut app, settled) = settle(app_with(bench_library(3)), ProjectPage::Overview);
    let rows = kept_snapshot(&ctx, &mut app).cell_row_count();
    let (table, _, _) = announced(&settled)
        .into_iter()
        .find(|(_, role, label)| *role == Role::Grid && *label == format!("Table, {rows} rows"))
        .expect("the Cells table is one keyboard stop");
    let _ = project_frame(
        &ctx,
        &mut app,
        vec![Event::AccessKitActionRequest(ActionRequest {
            action: Action::Focus,
            target_tree: TreeId::ROOT,
            target_node: table,
            data: None,
        })],
    );
    let _ = project_frame(&ctx, &mut app, vec![Event::Text("cell01".to_owned())]);
    assert_eq!(app.state.workbench.workspace, Workspace::Project);
    let _ = project_frame(&ctx, &mut app, vec![key(Key::Enter)]);
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(app.state.model.workspace.content.active_view, wanted);
}

/// An unsaved cell carries the editor tab's neutral dot, said as words, and
/// no warning mark.
#[test]
fn an_unsaved_cell_says_so_without_a_warning() {
    let (ctx, mut app, _) = settle(app_with(bench_library(2)), ProjectPage::Overview);
    let top = kept_snapshot(&ctx, &mut app).cells[0].name.clone();
    app.state.model.schematic.with_undo("Place", |schematic| {
        schematic.add_component(ComponentType::Resistor, Point::new(0, 0));
    });
    // What an edit leaves on its document, which the editor's tab reads.
    app.state.model.workspace.content.set_active_dirty(true);
    assert_eq!(
        app.state.model.workspace.content.active_dirty(),
        Some(true),
        "the top schematic is open"
    );
    let mut output = project_frame(&ctx, &mut app, Vec::new());
    for _ in 0..2 {
        output = project_frame(&ctx, &mut app, Vec::new());
    }
    let row = row_with(&output, &[&format!("Cell: {top}, unsaved changes")]);
    assert!(!row.contains("Warning"), "{row}");
}

/// Each kind of Activity row opens what it names: a run opens its results,
/// a validated revision opens Revision history on it, a checkpoint opens
/// Recovery on it and a library publication opens the Library page.
#[test]
fn each_activity_row_opens_what_it_names() {
    // A run.
    let (ctx, mut app, settled) = settle(populated_project_app(), ProjectPage::Overview);
    let snapshot = kept_snapshot(&ctx, &mut app);
    let newest = snapshot
        .operations
        .iter()
        .find(|operation| operation.intent == Some(OverviewIntent::OpenRun(0)))
        .expect("the newest run is listed");
    let row = row_with(&settled, &[&format!("Operation: {}", newest.event)]);
    click_announced(&ctx, &mut app, settled, &row);
    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert_eq!(app.state.model.simulation.view.active_run_idx, Some(0));

    // A library publication.
    let (ctx, mut app, settled) = settle(populated_project_app(), ProjectPage::Overview);
    let row = row_with(&settled, &["Operation: Library published"]);
    click_announced(&ctx, &mut app, settled, &row);
    assert_eq!(app.state.project.project_page, ProjectPage::Library);

    // A validated revision: the older of two, so the history stands on the
    // one the row names rather than on its newest.
    let mut app = populated_project_app();
    let project_id = app.state.model.workspace.content.project.id().to_string();
    let mut revisions = Vec::new();
    for (revision, note) in [(1, "First review"), (2, "Second review")] {
        app.state.model.schematic.with_undo("Place", |schematic| {
            schematic.add_component(ComponentType::Resistor, Point::new(40 * revision, 0));
        });
        let validation_receipt_digest = app
            .state
            .model
            .schematic
            .editor_ref()
            .design
            .validated_design_content_digest()
            .expect("the drawing has a digest");
        revisions.push(
            app.state
                .model
                .schematic
                .append_validated_revision(ValidatedRevisionRequest {
                    project_id: project_id.clone(),
                    project_revision: revision as u64,
                    view_identity: "user/top/schematic".to_owned(),
                    revision_note: note.to_owned(),
                    author: "Test engineer".to_owned(),
                    validation_receipt_digest,
                    finding_counts: ValidationFindingCounts::default(),
                    dependencies: Vec::new(),
                    advisory_dispositions: Vec::new(),
                })
                .expect("the revision is recorded"),
        );
    }
    let (ctx, mut app, settled) = settle(app, ProjectPage::Overview);
    let row = row_with(&settled, &["First review"]);
    click_announced(&ctx, &mut app, settled, &row);
    let history = &app.state.project.project_revision_history;
    assert!(history.open);
    let selected = history
        .selected()
        .expect("the history stands on the revision");
    assert!(
        selected.ends_with(&revisions[0].as_uuid().to_string()),
        "{selected:?}"
    );

    // A checkpoint.
    #[cfg(not(target_arch = "wasm32"))]
    {
        use rspice_app_services::workbench::lifecycle::project_checkpoint::{
            ProjectCheckpointReason, fixture_summary,
        };
        let mut app = populated_project_app();
        let now = rspice_app_state::time_compat::checked_unix_time_ms().expect("clock");
        let name = app.state.model.workspace.content.project.name().to_owned();
        let catalog = [(3, 6 * 60_000), (2, 2 * 3_600_000)]
            .into_iter()
            .map(|(revision, age)| {
                fixture_summary(
                    &name,
                    revision,
                    ProjectCheckpointReason::Manual,
                    now - age,
                    1,
                )
            })
            .collect::<Vec<_>>();
        let older = catalog[1].checkpoint_id().to_string();
        let recovery = &mut app.state.project.project_checkpoint_recovery;
        recovery.project_id = Some(app.state.model.workspace.content.project.id().to_string());
        recovery.initialized = true;
        recovery.checkpoints = catalog;
        let (ctx, mut app, settled) = settle(app, ProjectPage::Overview);
        let row = row_with(&settled, &["Detail: Revision 2"]);
        click_announced(&ctx, &mut app, settled, &row);
        assert_eq!(app.state.project.project_page, ProjectPage::Recovery);
        assert_eq!(app.state.project.project_checkpoint_selection, Some(older));
    }
}

/// A finding under the status rows opens the Problems page listing it,
/// standing on it, whatever filter was left there before.
#[test]
fn a_finding_opens_the_problems_page_on_it() {
    let mut app = populated_project_app();
    app.state.workbench.problems_view.text = "something else".to_owned();
    let (ctx, mut app, settled) = settle(app, ProjectPage::Overview);
    let finding = kept_snapshot(&ctx, &mut app)
        .problems
        .first()
        .and_then(|problem| problem.finding)
        .expect("the finding that stops a run is listed with its identity");
    let row = row_with(&settled, &["R3 has an unconnected pin"]);
    click_announced(&ctx, &mut app, settled, &row);
    assert_eq!(app.state.workbench.console_page, ConsolePage::Problems);
    assert!(app.state.workbench.console_visible);
    assert_eq!(
        app.state.workbench.problems_view.text,
        "R3 has an unconnected pin"
    );
    assert_eq!(app.state.model.diagnostics.current_finding(), Some(finding));
}

/// The cards' links open the pages that list everything: All cells the
/// Library page, All runs Results.
#[test]
fn the_card_links_open_the_pages_that_list_everything() {
    let (ctx, mut app, settled) = settle(populated_project_app(), ProjectPage::Overview);
    click_announced(&ctx, &mut app, settled, "All cells");
    assert_eq!(app.state.project.project_page, ProjectPage::Library);

    let (ctx, mut app, settled) = settle(populated_project_app(), ProjectPage::Overview);
    click_announced(&ctx, &mut app, settled, "All runs");
    assert_eq!(app.state.workbench.workspace, Workspace::Results);
}
