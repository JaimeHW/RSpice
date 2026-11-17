//! The project Overview through the whole application: its snapshot, its
//! problem line, and the actions it raises, run as the dispatcher runs them.

use rspice_design::drc::DrcSeverity;

use crate::workbench::commands::vocabulary::ShellCommand;
use crate::workbench::shell_host::project::library_publication::commit_project_library_publication;
use crate::workbench::shell_host::project::library_publication::prepare_project_library_publication;
use crate::workbench::{RSpiceApp, features};
use rspice_app_project::workbench::project::commands::ProjectCommand;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_project::workbench::surfaces::project::overview::{
    OverviewIntent, OverviewSnapshot, ProblemSnapshot, Tone, blocking_findings, run_plan_intent,
};
use rspice_app_state::state::CellViewRef;
use rspice_app_state::workbench::workspace::Workspace;

/// The Overview's snapshot, as the page takes it in the context the shell
/// lends it, with the dispatcher's answers for the commands its rows offer.
fn snapshot_of(app: &mut RSpiceApp) -> OverviewSnapshot {
    snapshot_on(&egui::Context::default(), app)
}

/// The snapshot taken on `ctx`, which keeps what an earlier snapshot there
/// derived, as the page's frames do.
fn snapshot_on(ctx: &egui::Context, app: &mut RSpiceApp) -> OverviewSnapshot {
    OverviewSnapshot::capture(ctx, &features::cx_with_foreign_answers(app, |_| true))
}

/// The findings of the cell view runs start from that stop a run, as the
/// page reads them in its context.
fn findings_of(app: &mut RSpiceApp) -> Vec<ProblemSnapshot> {
    blocking_findings(&egui::Context::default(), &features::cx(app))
}

/// What the reader asked of the Overview, raised in the project feature's
/// context and carried out as the shell does once the page returns.
fn raise(app: &mut RSpiceApp, intent: OverviewIntent) {
    intent.raise(&egui::Context::default(), &mut features::cx(app));
    app.after_contribution();
}

#[test]
fn published_library_snapshots_are_retained_project_operations() {
    let mut app = RSpiceApp::test_instance();
    // One context throughout: the register taken before the publication is
    // kept there, and the publication has to replace it.
    let ctx = egui::Context::default();
    assert!(
        snapshot_on(&ctx, &mut app)
            .operations
            .iter()
            .all(|operation| !operation.event.contains("Library published")),
        "a project with no publication must not claim one"
    );

    let candidate = prepare_project_library_publication(
        &app.state,
        "analog-core-1.0.0",
        "release-engineer@example.test",
        "organization-release-authority",
        "Qualified library handoff",
    )
    .expect("publication prepares");
    commit_project_library_publication(&mut app.state, candidate)
        .expect("durably accepted publication commits");

    let snapshot = snapshot_on(&ctx, &mut app);
    let published = snapshot
        .operations
        .iter()
        .find(|operation| operation.event.starts_with("Library published"))
        .expect("the committed publication reaches the operations register");
    assert_eq!(published.event, "Library published · analog-core-1.0.0");
    assert!(published.detail.contains("Qualified library handoff"));
    assert!(published.detail.contains("release-engineer@example.test"));
    assert!(published.when(snapshot.now_ms).is_some());
    assert_eq!(
        published.intent,
        Some(OverviewIntent::Command(
            ProjectCommand::Page(ProjectPage::Library).key()
        ))
    );
}

#[test]
fn overview_new_cell_action_opens_the_real_creation_workflow() {
    let mut app = RSpiceApp::test_instance();
    raise(
        &mut app,
        OverviewIntent::Command(ProjectCommand::NewCell.key()),
    );
    assert!(app.state.project.new_cell_dialog);
}

#[test]
fn overview_history_action_preserves_project_context() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Project;
    let active = app.state.model.workspace.content.active_view.clone();
    raise(
        &mut app,
        OverviewIntent::Command(ProjectCommand::RevisionHistory.key()),
    );

    assert_eq!(app.state.workbench.workspace, Workspace::Project);
    assert_eq!(app.state.model.workspace.content.active_view, active);
    assert!(app.state.project.project_revision_history.open);
}

#[test]
fn revision_history_is_available_from_project_overview() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Project;

    assert!(app.command_key_enabled(ProjectCommand::RevisionHistory.key()));
}

#[test]
fn netlist_first_latest_run_leads_to_the_deck_rather_than_a_blocked_simulate() {
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.apply_imported_netlist(
        "V1 out 0 1\n.op\n.end\n".to_owned(),
        None,
        "front_end.sp"
    ));
    app.state.workbench.workspace = Workspace::Project;

    let snapshot = snapshot_of(&mut app);
    let latest_run = snapshot
        .statuses
        .iter()
        .find(|status| status.area == "Latest run")
        .expect("the register always reports the latest run");
    assert_eq!(latest_run.action, "Open deck");
    assert_eq!(
        latest_run.intent,
        OverviewIntent::Command(ShellCommand::OpenWorkspace(Workspace::Netlist).key())
    );

    // Simulate's run preflight reads the schematic a netlist-first project
    // leaves pristine, so routing there would strand the reader.
    assert!(app.state.simulation_run_block_reason().is_some());
    raise(&mut app, latest_run.intent.clone());
    assert_eq!(app.state.workbench.workspace, Workspace::Netlist);
}

#[test]
fn overview_run_without_a_dataset_returns_to_analyses() {
    let mut app = RSpiceApp::test_instance();
    app.state.model.simulation.start_run();

    raise(&mut app, OverviewIntent::OpenRun(0));

    assert_eq!(app.state.workbench.workspace, Workspace::Simulate);
}

#[test]
fn overview_run_action_opens_the_plan_without_starting_execution() {
    let mut app = RSpiceApp::test_instance();
    let retained_runs = app.state.model.simulation.retained.runs.len();

    raise(&mut app, run_plan_intent());

    assert_eq!(app.state.workbench.workspace, Workspace::Simulate);
    assert_eq!(
        app.state.model.simulation.retained.runs.len(),
        retained_runs
    );
    assert!(!app.state.model.simulation.execution.is_running);
}

#[test]
fn netlist_first_overview_never_exposes_the_pristine_bootstrap_schematic() {
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.apply_imported_netlist(
        "V1 out 0 1\n.op\n.end\n".to_owned(),
        None,
        "front_end.sp"
    ));
    app.state.workbench.workspace = Workspace::Project;

    let snapshot = snapshot_of(&mut app);

    assert!(snapshot.netlist_first);
    assert_eq!(snapshot.statuses[0].area, "Source deck");
    assert!(
        snapshot
            .statuses
            .iter()
            .all(|status| status.area != "Schematic checks")
    );
    // An unvalidated deck is a state its row says, not a finding: the strip
    // names findings only.
    assert_eq!(snapshot.statuses[0].state(), "Not validated");
    assert_eq!(snapshot.statuses[0].tone(), Tone::Neutral);
    assert!(snapshot.problems.is_empty());
    let deck = snapshot.deck.as_ref().expect("the deck is the first row");
    assert_eq!(deck.name, "front_end.sp");
    assert_eq!(deck.role, "Deck");
    assert!(deck.views.starts_with("SPICE \u{b7} 3 lines"), "{deck:?}");
    assert_eq!(
        deck.intent,
        OverviewIntent::Command(ShellCommand::OpenWorkspace(Workspace::Netlist).key())
    );
    // The pristine bootstrap cell is not part of the design the reader made.
    let project = &app.state.model.workspace.content.project;
    assert!(
        snapshot.cells.iter().all(|cell| {
            !(cell.library == project.root_library && cell.name == project.top_cell)
        }),
        "{:?}",
        snapshot.cells
    );
}

/// The overview's model closure asks the shell to show the project's models;
/// the scope the Models workspace then opens on is the shell's to set, and
/// `feature_tests::models::studio` holds that half.
#[test]
fn overview_model_closure_opens_the_models_workspace() {
    let mut app = RSpiceApp::test_instance();
    raise(&mut app, OverviewIntent::OpenProjectModels);

    assert_eq!(app.state.workbench.workspace, Workspace::Models);
}

/// Checks that have not run are neither green nor a caution: the row says
/// "Not run" unmarked, and the strip, which names findings, stays away.
#[test]
fn checks_without_a_retained_result_read_not_run_unmarked() {
    let mut app = RSpiceApp::test_instance();
    app.state.model.workspace.content.active_view = CellViewRef::new(
        &app.state.model.workspace.content.project.root_library,
        &app.state.model.workspace.content.project.top_cell,
        rspice_app_state::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    );
    assert_eq!(findings_of(&mut app), Vec::new());
    let snapshot = snapshot_of(&mut app);
    let checks = &snapshot.statuses[0];
    assert_eq!(checks.area, "Schematic checks");
    assert_eq!(checks.state(), "Not run");
    assert_eq!(checks.tone(), Tone::Neutral);
}

/// A root that checks clean has no strip at all: "No current schematic
/// findings" was a report of the absence of a problem.
#[test]
fn a_clean_project_root_states_no_problem() {
    use rspice_design::drc::DrcResult;

    let mut app = RSpiceApp::test_instance();
    app.state.model.workspace.content.active_view = CellViewRef::new(
        &app.state.model.workspace.content.project.root_library,
        &app.state.model.workspace.content.project.top_cell,
        rspice_app_state::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    );
    let mut result = DrcResult::default();
    result.completed = true;
    app.state
        .publish_active_design_check_result(result)
        .expect("publish project-root design-check receipt");

    assert!(findings_of(&mut app).is_empty());
}

/// The strip under the status rows lists every finding that stops a run,
/// the worst first and each in the order the checks found it, by its words
/// and with its identity in the Problems list; a finding that does not stop
/// a run is the Problems list's alone.
#[test]
fn the_strip_lists_every_finding_that_stops_a_run_worst_first() {
    use rspice_design::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};

    let mut app = RSpiceApp::test_instance();
    app.state.model.workspace.content.active_view = CellViewRef::new(
        &app.state.model.workspace.content.project.root_library,
        &app.state.model.workspace.content.project.top_cell,
        rspice_app_state::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    );
    let mut result = DrcResult::new();
    result.add_violation(DrcViolation {
        severity: DrcSeverity::Info,
        ..DrcViolation::new(
            1,
            DrcViolationType::EmptyName,
            "informational finding",
            DrcLocation::Global,
        )
    });
    result.add_violation(DrcViolation {
        severity: DrcSeverity::Error,
        ..DrcViolation::new(
            2,
            DrcViolationType::UnconnectedPin,
            "error finding",
            DrcLocation::Global,
        )
    });
    result.add_violation(DrcViolation {
        severity: DrcSeverity::Warning,
        ..DrcViolation::new(
            3,
            DrcViolationType::DanglingWire,
            "warning finding",
            DrcLocation::Global,
        )
    });
    result.add_violation(DrcViolation {
        severity: DrcSeverity::Critical,
        ..DrcViolation::new(
            4,
            DrcViolationType::MissingGround,
            "critical finding",
            DrcLocation::Global,
        )
    });
    result.completed = true;
    app.state
        .publish_active_design_check_result(result)
        .expect("publish project-root design-check receipt");

    let findings = findings_of(&mut app);
    assert_eq!(
        findings
            .iter()
            .map(|finding| finding.message.as_str())
            .collect::<Vec<_>>(),
        ["critical finding", "error finding"]
    );
    assert!(findings.iter().all(|finding| finding.tone == Tone::Error));
    let critical = findings[0]
        .finding
        .expect("a check's finding has an identity");
    assert!(
        app.state
            .model
            .diagnostics
            .batches()
            .flat_map(|batch| &batch.records)
            .any(|record| record.canonical.diagnostic_id == critical
                && record.message == "critical finding"),
        "the identity is the one the Problems list holds"
    );
    assert_eq!(snapshot_of(&mut app).problems, findings);
}
