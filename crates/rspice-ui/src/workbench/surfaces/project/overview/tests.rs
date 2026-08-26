//! The overview reports what is current, never a retained copy of what was.
//!
//! These pin the three ways a stale value could be read as project status: a
//! check result kept past the edit that invalidated it, a completed failure
//! rendered in the shape of a success, and a display limit reported as though
//! it were the length of the history.

use super::*;

#[test]
fn stale_checks_never_reuse_retained_result_copy() {
    let copy = drc_copy(
        false,
        Some(DrcCounts {
            critical: 0,
            errors: 0,
            warnings: 0,
            info: 0,
        }),
    );
    assert_eq!(copy.state, "Not current");
    assert_eq!(copy.tone, Tone::Warn);
}

#[test]
fn current_check_blockers_are_counted_without_losing_advisories() {
    let copy = drc_copy(
        true,
        Some(DrcCounts {
            critical: 1,
            errors: 2,
            warnings: 3,
            info: 1,
        }),
    );
    assert_eq!(copy.state, "3 blocking findings");
    assert_eq!(copy.detail, "1 critical · 2 errors · 4 advisories");
    assert_eq!(copy.tone, Tone::Error);
}

#[test]
fn project_status_counts_areas_and_never_borrows_the_advisory_word() {
    let status = |tone| StatusSnapshot {
        area: "Area",
        state: String::new(),
        detail: String::new(),
        action: "",
        tone,
        intent: OverviewIntent::OpenProjectRoot,
        enabled: true,
        disabled_reason: None,
    };
    let statuses = [
        status(Tone::Ok),
        status(Tone::Warn),
        status(Tone::Warn),
        status(Tone::Error),
    ];

    assert_eq!(
        project_status_summary(&statuses, Tone::Error),
        "blocked · 3 of 4 areas"
    );
    assert_eq!(
        project_status_summary(&statuses[..3], Tone::Warn),
        "review · 2 of 3 areas"
    );
    assert_eq!(
        project_status_summary(&statuses[..1], Tone::Ok),
        "current · 1 area"
    );
    // "advisory" belongs to finding counts, which this summary never counts.
    for tone in [Tone::Error, Tone::Warn, Tone::Ok] {
        assert!(!project_status_summary(&statuses, tone).contains("advisor"));
    }
}

#[test]
fn operations_meta_never_reports_the_display_limit_as_the_history() {
    assert_eq!(operations_meta(5, 23), "5 of 23 retained");
    assert_eq!(operations_meta(3, 3), "3 retained");
    assert_eq!(operations_meta(0, 0), "0 retained");
}

#[test]
fn published_library_snapshots_are_retained_project_operations() {
    let mut app = RSpiceApp::test_instance();
    assert!(
        OverviewSnapshot::capture(&app)
            .operations
            .iter()
            .all(|operation| !operation.event.contains("Library published")),
        "a project with no publication must not claim one"
    );

    let candidate = app
        .prepare_project_library_publication(
            "analog-core-1.0.0",
            "release-engineer@example.test",
            "organization-release-authority",
            "Qualified library handoff",
        )
        .expect("publication prepares");
    app.commit_project_library_publication(candidate)
        .expect("durably accepted publication commits");

    let snapshot = OverviewSnapshot::capture(&app);
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
        Some(OverviewIntent::Command(Command::ProjectPage(
            ProjectPage::Library
        )))
    );
    assert!(
        snapshot.operation_total >= snapshot.operations.len(),
        "the retained total can never be smaller than the rows shown"
    );
}

#[test]
fn completed_failure_is_not_rendered_as_success() {
    assert_eq!(
        lifecycle_label(SimulationRunLifecycle::Completed, false),
        "completed with failures"
    );
    assert_eq!(
        lifecycle_tone(SimulationRunLifecycle::Completed, false),
        Tone::Error
    );
}

#[test]
fn run_title_does_not_duplicate_retained_sequence_prefix() {
    assert_eq!(
        run_title(12, "Run 12 (10:31:02 AM)"),
        "Run 12 (10:31:02 AM)"
    );
    assert_eq!(run_title(12, "corner sweep"), "Run 12 · corner sweep");
}

#[test]
fn recent_operation_timestamps_fail_closed_and_format_relative_age() {
    assert_eq!(unix_seconds_to_millis(f64::NAN), None);
    assert_eq!(unix_seconds_to_millis(-1.0), None);
    assert_eq!(unix_seconds_to_millis(1.25), Some(1_250));
    assert_eq!(relative_time(1_000, 999), "time n/a");
    assert_eq!(relative_time(1_000, 1_030), "now");
    assert_eq!(relative_time(1_000, 61_000), "1 min");
    assert_eq!(relative_time(1_000, 3_601_000), "1 h");
    assert_eq!(relative_time(1_000, 86_401_000), "1 d");
}

#[test]
fn overview_new_cell_action_opens_the_real_creation_workflow() {
    let mut app = RSpiceApp::test_instance();
    OverviewIntent::Command(Command::NewCell).execute(&mut app);
    assert!(app.state.dialogs.new_cell_dialog);
}

#[test]
fn overview_history_action_preserves_project_context() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Project;
    let active = app.state.workspace.active_view.clone();
    OverviewIntent::Command(Command::RevisionHistory).execute(&mut app);

    assert_eq!(app.state.workbench.workspace, Workspace::Project);
    assert_eq!(app.state.workspace.active_view, active);
    assert!(app.state.dialogs.project_revision_history.open);
}

#[test]
fn revision_history_is_available_from_project_overview() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Project;

    assert!(Command::RevisionHistory.is_enabled(&app));
}

#[test]
fn netlist_first_latest_run_leads_to_the_deck_rather_than_a_blocked_simulate() {
    let mut app = RSpiceApp::test_instance();
    assert!(
        crate::workbench::workflows::netlist_workflow::apply_imported_netlist(
            &mut app.state,
            "V1 out 0 1\n.op\n.end\n".to_owned(),
            None,
            "front_end.sp",
        )
    );
    app.state.workbench.workspace = Workspace::Project;

    let snapshot = OverviewSnapshot::capture(&app);
    let latest_run = snapshot
        .statuses
        .iter()
        .find(|status| status.area == "Latest run")
        .expect("the register always reports the latest run");
    assert_eq!(latest_run.action, "Open deck");
    assert_eq!(
        latest_run.intent,
        OverviewIntent::Command(Command::OpenWorkspace(Workspace::Netlist))
    );

    // Simulate's run preflight reads the schematic a netlist-first project
    // leaves pristine, so routing there would strand the reader.
    assert!(!app.state.can_run_simulation());
    latest_run.intent.clone().execute(&mut app);
    assert_eq!(app.state.workbench.workspace, Workspace::Netlist);
}

#[test]
fn run_destination_only_leaves_simulate_for_schematic_projects() {
    assert_eq!(run_destination(true, false), Workspace::Results);
    assert_eq!(run_destination(true, true), Workspace::Results);
    assert_eq!(run_destination(false, false), Workspace::Simulate);
    assert_eq!(run_destination(false, true), Workspace::Netlist);
}

#[test]
fn overview_run_without_a_dataset_returns_to_analyses() {
    let mut app = RSpiceApp::test_instance();
    app.state.simulation.start_run();

    OverviewIntent::OpenRun(0).execute(&mut app);

    assert_eq!(app.state.workbench.workspace, Workspace::Simulate);
}

#[test]
fn overview_run_action_opens_the_plan_without_starting_execution() {
    let mut app = RSpiceApp::test_instance();
    let retained_runs = app.state.simulation.runs.len();

    run_plan_intent().execute(&mut app);

    assert_eq!(app.state.workbench.workspace, Workspace::Simulate);
    assert_eq!(app.state.simulation.runs.len(), retained_runs);
    assert!(!app.state.simulation.is_running);
}

#[test]
fn overview_import_deck_uses_the_independent_netlist_project_workflow() {
    assert_eq!(
        import_deck_intent(),
        OverviewIntent::Command(Command::OpenNetlist)
    );
}

#[test]
fn netlist_first_overview_never_exposes_the_pristine_bootstrap_schematic() {
    let mut app = RSpiceApp::test_instance();
    assert!(
        crate::workbench::workflows::netlist_workflow::apply_imported_netlist(
            &mut app.state,
            "V1 out 0 1\n.op\n.end\n".to_owned(),
            None,
            "front_end.sp",
        )
    );
    app.state.workbench.workspace = Workspace::Project;

    let snapshot = OverviewSnapshot::capture(&app);
    let objects = design_objects(&snapshot);

    assert!(snapshot.netlist_first);
    assert_eq!(snapshot.statuses[0].area, "Source deck");
    assert!(
        snapshot
            .statuses
            .iter()
            .all(|status| status.area != "Schematic checks")
    );
    assert_eq!(snapshot.problem.code, "NOT RUN");
    assert_eq!(snapshot.problem.path, "front_end.sp");
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

#[test]
fn overview_model_closure_always_opens_the_project_catalog_scope() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.models_view.catalog_scope = ModelsCatalogScope::InstalledPacks;
    OverviewIntent::OpenProjectModels.execute(&mut app);

    assert_eq!(app.state.workbench.workspace, Workspace::Models);
    assert_eq!(
        app.state.workbench.models_view.catalog_scope,
        ModelsCatalogScope::Project
    );
}

#[test]
fn current_checks_without_a_retained_result_are_never_green() {
    let mut app = RSpiceApp::test_instance();
    app.state.workspace.active_view = CellViewRef::new(
        &app.state.workspace.project.root_library,
        &app.state.workspace.project.top_cell,
        crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    );
    let problem = problem_snapshot(&app);
    assert_eq!(problem.code, "NOT RUN");
    assert_eq!(problem.tone, Tone::Warn);
}

#[test]
fn project_problem_selects_the_highest_severity_finding() {
    use crate::services::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};

    let mut app = RSpiceApp::test_instance();
    app.state.workspace.active_view = CellViewRef::new(
        &app.state.workspace.project.root_library,
        &app.state.workspace.project.top_cell,
        crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    );
    let mut result = DrcResult::new();
    result.add_violation(
        DrcViolation::new(
            1,
            DrcViolationType::EmptyName,
            "informational finding",
            DrcLocation::Global,
        )
        .with_severity(DrcSeverity::Info),
    );
    result.add_violation(
        DrcViolation::new(
            2,
            DrcViolationType::MissingGround,
            "critical finding",
            DrcLocation::Global,
        )
        .with_severity(DrcSeverity::Critical),
    );
    result.completed = true;
    app.state
        .publish_active_design_check_result(result)
        .expect("publish project-root design-check receipt");

    let problem = problem_snapshot(&app);
    assert_eq!(problem.code, "CHK-002");
    assert_eq!(problem.message, "critical finding");
    assert_eq!(problem.tone, Tone::Error);
}

#[test]
fn design_register_rows_have_unique_truthful_identities() {
    let app = RSpiceApp::test_instance();
    let snapshot = OverviewSnapshot::capture(&app);
    let rows = design_objects(&snapshot);
    let unique = rows
        .iter()
        .map(|row| row.key.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();

    assert_eq!(unique.len(), rows.len());
    assert!(rows.len() <= 5);
    assert!(rows.first().is_some_and(|row| row.top));
}

#[test]
fn overview_does_not_invent_a_testbench_or_dut_without_a_configuration() {
    let app = RSpiceApp::test_instance();
    let snapshot = OverviewSnapshot::capture(&app);

    assert!(!snapshot.configuration.configured);
    assert_eq!(snapshot.configuration.root, "Not configured");
    assert_eq!(snapshot.configuration.dut, "Not configured");
    assert!(
        design_objects(&snapshot)
            .iter()
            .all(|row| !row.name.eq_ignore_ascii_case("Not configured"))
    );
}
