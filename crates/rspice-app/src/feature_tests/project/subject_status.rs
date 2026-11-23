//! Each subject of the project's health read from every place that states
//! it: the Overview's facts, the page each opens, and the status bar say it
//! in the same words.

#![cfg(not(target_arch = "wasm32"))]

use rspice_design::drc::{DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType};

use crate::workbench::examples::hierarchy_reference;
use crate::workbench::{RSpiceApp, features};
use rspice_app_project::workbench::project::DependencyLibrary;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_project::workbench::surfaces::project::overview::{
    OverviewIntent, OverviewSnapshot, SummaryFact, Tone,
};
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::state::CellViewRef;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;

use super::recovery_frames::{app_frame, offscreen, painted};

/// Every string the whole application painted, one per line, with the
/// Project workspace on `page` once its layout has settled.
fn painted_on(app: &mut RSpiceApp, page: ProjectPage) -> String {
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = page;
    let ctx = offscreen();
    for _ in 0..3 {
        app_frame(&ctx, app, Vec::new());
    }
    painted(&app_frame(&ctx, app, Vec::new()))
}

fn has_line(painted: &str, line: &str) -> bool {
    painted.lines().any(|painted| painted == line)
}

/// The Overview, as the page takes it in the context the shell lends it.
fn snapshot(app: &mut RSpiceApp) -> OverviewSnapshot {
    OverviewSnapshot::capture(
        &egui::Context::default(),
        &features::cx_with_foreign_answers(app, |_| true),
    )
}

/// The Overview's fact under `label`.
fn fact(app: &mut RSpiceApp, label: &str) -> Option<SummaryFact> {
    snapshot(app).fact(label).cloned()
}

/// `fact`'s link taken, as a click on its value takes it.
fn follow(app: &mut RSpiceApp, fact: &SummaryFact) {
    let link = fact.link.as_ref().expect("the value is a link");
    assert!(link.enabled, "{link:?}");
    link.intent.clone().raise(&mut features::cx(app));
    app.after_contribution();
}

/// A project whose top schematic is the cell view on screen.
fn schematic_project() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.model.workspace.content.active_view = CellViewRef::new(
        &app.state.model.workspace.content.project.root_library,
        &app.state.model.workspace.content.project.top_cell,
        rspice_app_state::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    );
    app
}

/// `project` with its checks run, finding what `findings` lists.
fn checked(mut app: RSpiceApp, findings: &[(DrcSeverity, &str)]) -> RSpiceApp {
    let mut result = DrcResult::new();
    for (index, (severity, message)) in findings.iter().enumerate() {
        result.add_violation(DrcViolation {
            severity: *severity,
            ..DrcViolation::new(
                index + 1,
                DrcViolationType::MissingGround,
                *message,
                DrcLocation::Global,
            )
        });
    }
    result.completed = true;
    app.state
        .publish_active_design_check_result(result)
        .expect("the checks publish");
    app
}

/// Findings read alike on the Overview and the status bar, and the
/// Overview's value opens the Problems panel that lists them.
#[test]
fn schematic_checks_read_alike_on_the_overview_and_the_status_bar() {
    let finding = "Net vbias1 has no DC path to ground";
    let mut app = checked(
        schematic_project(),
        &[
            (DrcSeverity::Error, finding),
            (DrcSeverity::Warning, finding),
        ],
    );
    let checks = fact(&mut app, "Checks").expect("a schematic project's checks");
    assert_eq!(
        (checks.value.as_str(), checks.tone),
        ("1 error \u{b7} 1 warning", Tone::Error)
    );

    let overview = painted_on(&mut app, ProjectPage::Overview);
    assert!(has_line(&overview, &checks.value), "the fact:\n{overview}");
    assert!(
        has_line(&overview, &format!("Problems \u{b7} {}", checks.value)),
        "the status bar:\n{overview}"
    );

    assert_eq!(
        checks.link.as_ref().map(|link| &link.intent),
        Some(&OverviewIntent::Command(ids::OPEN_PROBLEMS))
    );
    app.state.workbench.console_visible = false;
    follow(&mut app, &checks);
    assert!(app.state.workbench.console_visible, "Problems is shown");
    assert_eq!(
        app.state.workbench.console_page,
        rspice_app_workbench::workbench::state::workbench_session::ConsolePage::Problems
    );
}

/// Checks that have never run are neither passed nor a caution: the
/// Overview says "Not run", and its value runs them; the status bar, which
/// counts findings, says nothing.
#[test]
fn checks_that_have_not_run_read_not_run_and_unmarked_everywhere() {
    let mut app = schematic_project();
    let checks = fact(&mut app, "Checks").expect("a schematic project's checks");
    assert_eq!(
        (checks.value.as_str(), checks.tone),
        ("Not run", Tone::Neutral)
    );
    assert_eq!(
        checks.link.as_ref().map(|link| &link.intent),
        Some(&OverviewIntent::CheckSimulationRoot)
    );

    let overview = painted_on(&mut app, ProjectPage::Overview);
    assert!(has_line(&overview, "Not run"), "the fact:\n{overview}");
    assert!(!overview.contains("Problems \u{b7}"), "{overview}");
}

/// Running checks from the Overview opens and checks the root runs start
/// from, wherever the reader stood.
#[test]
fn run_checks_opens_and_checks_the_root_runs_start_from() {
    let mut app = RSpiceApp::test_instance();
    app.state = hierarchy_reference::build().state;
    let root = app
        .state
        .model
        .workspace
        .content
        .simulation_root_reference();
    let elsewhere = CellViewRef::new(&root.library, &root.cell, "symbol");
    app.state.model.workspace.content.active_view = elsewhere;

    let checks = fact(&mut app, "Checks").expect("a schematic project's checks");
    follow(&mut app, &checks);

    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(app.state.model.workspace.content.active_view, root);
}

/// The configuration the project marks for runs is the inspector's to
/// name; the Overview names one only when the plan runs another.
#[test]
fn the_overview_names_a_configuration_only_where_the_inspector_does_not() {
    let mut reference = RSpiceApp::test_instance();
    reference.state = hierarchy_reference::build().state;
    for mut app in [schematic_project(), reference] {
        assert_eq!(fact(&mut app, "Configuration"), None);
    }
}

/// A run that failed reads failed, marked; before any run the table's line
/// offers the plan's Run.
#[test]
fn the_latest_run_reads_its_outcome() {
    use rspice_app_project::workbench::surfaces::project::overview::Outputs;
    use rspice_app_state::state::{SimulationRunLifecycle, SimulationRunProvenance};

    let mut app = schematic_project();
    let before = snapshot(&mut app);
    assert_eq!(before.fact("Latest run"), None);
    let Outputs::NoRun(run) = before.outputs else {
        panic!("no run yet: {:?}", before.outputs);
    };
    assert_eq!(run.label, "Run");

    {
        let run = app.state.model.simulation.start_run();
        run.lifecycle = SimulationRunLifecycle::Failed;
        run.success = false;
        run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
            .expect("seal run");
    }
    let latest = fact(&mut app, "Latest run").expect("the newest run");
    assert!(latest.value.starts_with("Run "), "{latest:?}");
    assert!(latest.value.contains(" \u{b7} failed"), "{latest:?}");
    assert_eq!(latest.tone, Tone::Error);
    let overview = painted_on(&mut app, ProjectPage::Overview);
    assert!(
        has_line(&overview, &latest.value),
        "{:?} on the page:\n{overview}",
        latest.value
    );
}

/// A model file deleted since it was attached is counted on the Overview
/// in the words the Dependencies page says it, marked, and the value opens
/// that page standing on it.
#[test]
fn a_missing_model_file_is_counted_and_opens_dependencies_on_it() {
    let folder = unique_temp_dir("overview-dependencies");
    let mut app = schematic_project();
    let path = folder.join("missing.lib");
    std::fs::write(&path, ".model nmissing nmos level=1 vto=0.5\n").expect("the file writes");
    let name = app
        .state
        .model
        .model_library_manager
        .load_library_file(&path, None)
        .expect("the model file imports");
    std::fs::remove_file(&path).expect("the model file is deleted");

    let overview = painted_on(&mut app, ProjectPage::Overview);
    let dependencies = fact(&mut app, "Dependencies").expect("the Overview states them");
    assert_eq!(dependencies.tone, Tone::Error);
    assert!(
        dependencies.value.ends_with("1 not found"),
        "{dependencies:?}"
    );
    assert!(has_line(&overview, &dependencies.value), "{overview}");

    let page = painted_on(&mut app, ProjectPage::Dependencies);
    assert!(has_line(&page, "Not found"), "{page}");

    app.state.project.project_page = ProjectPage::Overview;
    app.state.project.dependencies.filter = "nothing lists this".to_owned();
    follow(&mut app, &dependencies);
    assert_eq!(app.state.project.project_page, ProjectPage::Dependencies);
    assert_eq!(
        app.state.project.project_dependency_selection,
        Some(DependencyLibrary::model(name))
    );
    assert!(app.state.project.dependencies.filter.is_empty());
    let _ = std::fs::remove_dir_all(folder);
}

/// A project never saved says so in the project inspector's Location,
/// which stands beside the Overview.
#[test]
fn a_project_never_saved_says_so_in_the_inspector() {
    let mut app = schematic_project();
    let overview = painted_on(&mut app, ProjectPage::Overview);
    assert!(has_line(&overview, "Location"), "{overview}");
    assert!(has_line(&overview, "Not saved"), "{overview}");
}

/// A netlist-first project's Overview states its deck, which its value
/// opens, never the pristine bootstrap schematic or its checks.
#[test]
fn a_netlist_first_overview_states_the_deck_and_no_drawing() {
    let mut app = RSpiceApp::test_instance();
    assert!(
        app.state
            .open_test_deck("V1 out 0 1\n.op\n.end\n".to_owned(), None, "front_end.sp")
    );
    let snapshot = snapshot(&mut app);
    assert!(snapshot.netlist_first);
    let deck = snapshot.fact("Source deck").expect("the deck is a fact");
    assert_eq!(deck.value, "front_end.sp \u{b7} no includes");
    for label in ["Hierarchy", "Devices", "Checks", "Configuration"] {
        assert_eq!(
            snapshot.fact(label),
            None,
            "a netlist-first project has no {label}"
        );
    }
    // The pristine bootstrap cell is not part of the design the reader made.
    let bootstrap = CellViewRef::new(
        &app.state.model.workspace.content.project.root_library,
        &app.state.model.workspace.content.project.top_cell,
        rspice_app_state::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    )
    .display_path();
    let overview = painted_on(&mut app, ProjectPage::Overview);
    assert!(has_line(&overview, &deck.value), "{overview}");
    assert!(!overview.contains(&bootstrap), "{overview}");

    let deck = deck.clone();
    follow(&mut app, &deck);
    assert_eq!(app.state.workbench.workspace, Workspace::Netlist);
}
