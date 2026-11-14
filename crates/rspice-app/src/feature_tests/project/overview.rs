//! The project Overview through the whole application: its snapshot, its
//! problem line, and the actions it raises, run as the dispatcher runs them.

use rspice_design::drc::DrcSeverity;
use std::collections::BTreeSet;

use crate::state::CellViewRef;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::project::commands::ProjectCommand;
use crate::workbench::project::workspace_state::ProjectPage;
use crate::workbench::shell_host::project::library_publication::commit_project_library_publication;
use crate::workbench::shell_host::project::library_publication::prepare_project_library_publication;
use crate::workbench::state::Workspace;
use crate::workbench::surfaces::project::overview::{
    OverviewIntent, OverviewSnapshot, ProblemSnapshot, Tone, design_objects, problem_snapshot,
    run_plan_intent,
};
use crate::workbench::{RSpiceApp, features};

/// The Overview's snapshot, as the page takes it in the context the shell
/// lends it, with the dispatcher's answers for the commands its rows offer.
fn snapshot_of(app: &mut RSpiceApp) -> OverviewSnapshot {
    OverviewSnapshot::capture(&features::cx_with_foreign_answers(app, |_| true))
}

/// The project root's finding, as the page reads it in its context.
fn problem_of(app: &mut RSpiceApp) -> Option<ProblemSnapshot> {
    problem_snapshot(&features::cx(app))
}

/// What the reader asked of the Overview, raised in the project feature's
/// context and carried out as the shell does once the page returns.
fn raise(app: &mut RSpiceApp, intent: OverviewIntent) {
    intent.raise(&mut features::cx(app));
    app.after_contribution();
}

#[test]
fn published_library_snapshots_are_retained_project_operations() {
    let mut app = RSpiceApp::test_instance();
    assert!(
        snapshot_of(&mut app)
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

    let snapshot = snapshot_of(&mut app);
    let published = snapshot
        .operations
        .iter()
        .find(|operation| operation.event.starts_with("Library published"))
        .expect("the committed publication reaches the operations register");
    assert_eq!(published.event, "Library published · analog-core-1.0.0");
    assert!(published.detail.contains("Qualified library handoff"));
    assert!(published.detail.contains("release-engineer@example.test"));
    assert_ne!(published.when, "time n/a");
    assert_eq!(
        published.intent,
        Some(OverviewIntent::Command(
            ProjectCommand::Page(ProjectPage::Library).key()
        ))
    );
    assert!(
        snapshot.operation_total >= snapshot.operations.len(),
        "the retained total can never be smaller than the rows shown"
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
        OverviewIntent::Command(Command::OpenWorkspace(Workspace::Netlist).key())
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
    let objects = design_objects(&snapshot);

    assert!(snapshot.netlist_first);
    assert_eq!(snapshot.statuses[0].area, "Source deck");
    assert!(
        snapshot
            .statuses
            .iter()
            .all(|status| status.area != "Schematic checks")
    );
    let problem = snapshot
        .problem
        .as_ref()
        .expect("an unvalidated deck is a finding the strip names");
    assert_eq!(problem.code, "NOT RUN");
    assert_eq!(problem.path, "front_end.sp");
    assert!(objects.first().is_some_and(|object| {
        object.top && object.name == "front_end.sp" && object.kind.starts_with("SPICE deck")
    }));
    // The pristine bootstrap views must not leak in under any spelling —
    // neither the root identity itself nor its schematic/symbol view paths.
    assert!(
        objects
            .iter()
            .all(|object| !object.name.starts_with(&snapshot.descriptor_root)),
        "{objects:?}"
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

#[test]
fn current_checks_without_a_retained_result_are_never_green() {
    let mut app = RSpiceApp::test_instance();
    app.state.model.workspace.content.active_view = CellViewRef::new(
        &app.state.model.workspace.content.project.root_library,
        &app.state.model.workspace.content.project.top_cell,
        crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    );
    let problem = problem_of(&mut app).expect("checks that have not run are a finding");
    assert_eq!(problem.code, "NOT RUN");
    assert_eq!(problem.tone, Tone::Warn);
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
        crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    );
    let mut result = DrcResult::default();
    result.completed = true;
    app.state
        .publish_active_design_check_result(result)
        .expect("publish project-root design-check receipt");

    assert!(problem_of(&mut app).is_none());
}

#[test]
fn project_problem_selects_the_highest_severity_finding() {
    use rspice_design::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};

    let mut app = RSpiceApp::test_instance();
    app.state.model.workspace.content.active_view = CellViewRef::new(
        &app.state.model.workspace.content.project.root_library,
        &app.state.model.workspace.content.project.top_cell,
        crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
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
        severity: DrcSeverity::Critical,
        ..DrcViolation::new(
            2,
            DrcViolationType::MissingGround,
            "critical finding",
            DrcLocation::Global,
        )
    });
    result.completed = true;
    app.state
        .publish_active_design_check_result(result)
        .expect("publish project-root design-check receipt");

    let problem = problem_of(&mut app).expect("a critical violation is a finding");
    assert_eq!(problem.code, "CHK-002");
    assert_eq!(problem.message, "critical finding");
    assert_eq!(problem.tone, Tone::Error);
}

#[test]
fn design_register_rows_have_unique_truthful_identities() {
    let mut app = RSpiceApp::test_instance();
    let snapshot = snapshot_of(&mut app);
    let rows = design_objects(&snapshot);
    let unique = rows
        .iter()
        .map(|row| row.key.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();

    assert_eq!(unique.len(), rows.len());
    assert!(rows.len() <= 5);
    assert!(rows.first().is_some_and(|row| row.top));
}

/// With no configuration used for runs the project runs Default: the
/// overview names it, roots it at the project's own top cell, and invents no
/// DUT.
#[test]
fn overview_states_default_without_a_configuration() {
    let mut app = RSpiceApp::test_instance();
    let snapshot = snapshot_of(&mut app);
    let project = &app.state.model.workspace.content.project;

    assert!(!snapshot.configuration.configured);
    assert_eq!(snapshot.configuration.state, "Default");
    assert_eq!(
        snapshot.configuration.root,
        format!("{}/{}/schematic", project.root_library, project.top_cell)
    );
    assert_eq!(snapshot.configuration.dut, "\u{2014}");
    assert!(snapshot.configuration.tone.problem().is_none());
    assert!(
        design_objects(&snapshot)
            .iter()
            .all(|row| row.name != "\u{2014}")
    );
}
