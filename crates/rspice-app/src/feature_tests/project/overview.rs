//! The project Overview through the whole application: what each fact of
//! the band states, what the outputs table holds, and what each linked
//! value and row raises, run as the dispatcher runs them.

use rspice_design::drc::{DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType};

use super::surface::populated_project_app;
use crate::workbench::examples::hierarchy_reference;
use crate::workbench::{RSpiceApp, features};
use rspice_app_project::workbench::surfaces::project::overview::{
    Action, Outputs, OverviewIntent, OverviewSnapshot, SummaryFact, Tone,
};
use rspice_app_simulate::workbench::surfaces::verify::raster::{
    seed_corner, seed_monte_carlo, seed_run_set,
};
use rspice_app_state::state::CellViewRef;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::{CommandAvailability, ids};
use rspice_app_workbench::workbench::state::workbench_session::ConsolePage;

/// The Overview, as the page takes it in the context the shell lends it,
/// with the dispatcher's answers for the commands it offers.
fn snapshot_of(app: &mut RSpiceApp) -> OverviewSnapshot {
    OverviewSnapshot::capture(
        &egui::Context::default(),
        &features::cx_with_foreign_answers(app, |_| true),
    )
}

/// `action` taken as its linked value, row or button takes it: raised in
/// the project feature's context and carried out as the shell does once
/// the page returns.
fn take(app: &mut RSpiceApp, action: &Action) {
    assert!(action.enabled, "{action:?}");
    action.intent.clone().raise(&mut features::cx(app));
    app.after_contribution();
}

/// The band's fact under `label`.
fn fact(snapshot: &OverviewSnapshot, label: &str) -> SummaryFact {
    snapshot
        .fact(label)
        .unwrap_or_else(|| panic!("the band states {label}: {:?}", snapshot.facts))
        .clone()
}

/// A project whose top schematic is the cell view on screen.
fn schematic_project() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let project = &app.state.model.workspace.content.project;
    let root = CellViewRef::new(
        &project.root_library,
        &project.top_cell,
        rspice_app_state::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    );
    app.state.model.workspace.content.active_view = root;
    app
}

/// `app` with its root checked, finding one of each `severities`.
fn checked(mut app: RSpiceApp, severities: &[(DrcSeverity, DrcViolationType)]) -> RSpiceApp {
    let mut result = DrcResult::new();
    for (index, (severity, kind)) in severities.iter().enumerate() {
        result.add_violation(DrcViolation {
            severity: *severity,
            ..DrcViolation::new(index + 1, *kind, "a finding", DrcLocation::Global)
        });
    }
    result.completed = true;
    app.state
        .publish_active_design_check_result(result)
        .expect("publish project-root design-check receipt");
    app
}

/// A deck-driven project's band states its deck, and the deck's value
/// opens it in Netlist.
#[test]
fn a_deck_driven_project_opens_its_deck() {
    let mut app = RSpiceApp::test_instance();
    assert!(
        app.state
            .open_test_deck("V1 out 0 1\n.op\n.end\n".to_owned(), None, "front_end.sp")
    );
    app.state.workbench.workspace = Workspace::Project;
    let deck = fact(&snapshot_of(&mut app), "Source deck");
    let link = deck.link.expect("the deck's value opens it");
    assert_eq!(link.label, "Open deck");

    take(&mut app, &link);

    assert_eq!(app.state.workbench.workspace, Workspace::Netlist);
}

/// The design's devices are counted through every instance of its
/// hierarchy, each kind spelled out.
#[test]
fn the_band_counts_the_devices_every_instance_places() {
    let mut app = RSpiceApp::test_instance();
    app.state = hierarchy_reference::build().state;
    let snapshot = snapshot_of(&mut app);
    // The top's source drives two amplifiers and two filters, a user's and
    // a vendor's, each a resistor and a capacitor.
    assert_eq!(
        fact(&snapshot, "Devices").value,
        "4 resistors \u{b7} 4 capacitors \u{b7} 1 source"
    );
    assert_eq!(
        fact(&snapshot, "Hierarchy").value,
        "2 levels \u{b7} 4 cells"
    );
}

/// The latest run's value opens Results on it.
#[test]
fn the_latest_run_opens_the_newest_run() {
    let mut app = populated_project_app();
    let latest = fact(&snapshot_of(&mut app), "Latest run");
    let link = latest.link.expect("a run with results links them");
    assert_eq!(link.label, "Open in Results");

    take(&mut app, &link);

    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert_eq!(app.state.model.simulation.view.active_run_idx, Some(0));
}

/// A run that kept no results has none to open: its value is plain text.
#[test]
fn a_run_that_kept_no_results_offers_none_to_open() {
    let mut app = RSpiceApp::test_instance();
    app.state.model.simulation.start_run();
    let latest = fact(&snapshot_of(&mut app), "Latest run");
    assert_eq!(latest.link, None, "{latest:?}");
}

/// The analyses the newest run ran are stated in one fact, in the order
/// the run ran them, joined as every fact of the band joins its parts.
#[test]
fn the_band_states_the_analyses_the_newest_run_ran() {
    let mut app = populated_project_app();
    let analyses = fact(&snapshot_of(&mut app), "Analyses").value;
    let tran = analyses.find("Transient").expect("the transient is stated");
    let ac = analyses
        .find("AC analysis")
        .expect("the AC sweep is stated");
    assert!(tran < ac, "{analyses}");
    assert!(analyses[..ac].ends_with(" \u{b7} "), "{analyses}");
    assert!(!analyses.contains(';'), "{analyses}");
}

/// Before any run the table offers the plan's Run, as the dispatcher
/// answers it for Simulate's.
#[test]
fn the_run_offered_before_any_run_is_the_plans_run() {
    let mut app = schematic_project();
    let Outputs::NoRun(run) = snapshot_of(&mut app).outputs else {
        panic!("no run yet: the table offers one");
    };
    assert_eq!(run.intent, OverviewIntent::Command(ids::RUN_SIMULATION));
    assert_eq!(
        run.enabled,
        app.command_availability(ids::RUN_SIMULATION) == CommandAvailability::Available
    );
}

/// A corner run judged against the plan's limits states each
/// specification's spread across the corners, the corner its worst value
/// was at, its limit, margin and verdict, as Verify's sheet states them; its
/// row opens Verify on that specification.
#[test]
fn a_failing_specification_states_its_spread_limit_margin_and_verdict() {
    let mut app = RSpiceApp::test_instance();
    seed_corner(&mut app.state.model);
    let outputs = snapshot_of(&mut app).outputs;
    let Outputs::Judged { spread, rows } = &outputs else {
        panic!("the corner run is judged: {outputs:?}");
    };
    assert_eq!(*spread, Some("Worst corner"));
    let gain = rows
        .iter()
        .find(|row| row.output == "gain")
        .unwrap_or_else(|| panic!("gain is a specification: {rows:?}"));
    // The corners gave gain from 39.4 dB to 44.2 dB; the worst corner is
    // the one Verify's sheet names.
    assert_eq!(gain.min.as_deref(), Some("39.4 dB"));
    assert_eq!(gain.max.as_deref(), Some("44.2 dB"));
    let sheet = app
        .state
        .model
        .simulation
        .verification_sheet()
        .expect("the corner run is judged");
    let worst = sheet
        .rows
        .iter()
        .find(|row| row.name == "gain")
        .and_then(|row| row.worst.as_ref())
        .expect("gain has a worst value");
    assert_eq!(gain.worst_at, sheet.worst_at(worst));
    assert!(gain.worst_at.is_some(), "{gain:?}");
    assert!(
        gain.limit
            .as_deref()
            .is_some_and(|limit| limit.contains("40"))
    );
    assert!(
        gain.margin
            .as_deref()
            .is_some_and(|margin| margin.contains("0.6"))
    );
    assert_eq!(gain.verdict, Some(("Fail", Tone::Error)));

    take(&mut app, &gain.action);

    assert_eq!(app.state.workbench.workspace, Workspace::Verify);
    assert_eq!(
        app.state.simulate.selected_specification.as_deref(),
        Some("gain")
    );
}

/// A run of one point is read by its one value: there is no spread.
#[test]
fn a_run_of_one_point_states_its_one_value() {
    let mut app = RSpiceApp::test_instance();
    seed_run_set(&mut app.state.model, &["TT"], &[1.8], &[27.0]);
    let outputs = snapshot_of(&mut app).outputs;
    let Outputs::Judged { spread, rows } = &outputs else {
        panic!("the run is judged: {outputs:?}");
    };
    assert_eq!(*spread, None);
    assert!(!rows.is_empty());
    assert!(rows.iter().all(|row| row.value.is_some()), "{rows:?}");
}

/// A Monte Carlo run is read by its spread across its trials, and the
/// trial its worst value came from.
#[test]
fn a_monte_carlo_run_states_its_spread_across_trials() {
    let mut app = RSpiceApp::test_instance();
    seed_monte_carlo(&mut app.state.model);
    let outputs = snapshot_of(&mut app).outputs;
    let Outputs::Judged { spread, rows } = &outputs else {
        panic!("the trials are judged: {outputs:?}");
    };
    assert_eq!(*spread, Some("Worst trial"));
    assert!(
        rows.iter()
            .all(|row| row.min.is_some() && row.max.is_some()),
        "{rows:?}"
    );
    assert!(
        rows.iter().any(|row| row
            .worst_at
            .as_deref()
            .is_some_and(|at| at.starts_with("trial "))),
        "{rows:?}"
    );
}

/// A run's measurements fill the table when the plan sets no limit, value
/// only, as the console spells them.
#[test]
fn measurements_without_limits_state_their_values_only() {
    let mut app = populated_project_app();
    let outputs = snapshot_of(&mut app).outputs;
    let Outputs::Measured(rows) = &outputs else {
        panic!("the plan sets no limit: {outputs:?}");
    };
    let names = rows
        .iter()
        .map(|row| row.output.as_str())
        .collect::<Vec<_>>();
    assert_eq!(names, ["gain_db", "nf_db"]);
    assert!(rows.iter().all(|row| !row.value.is_empty()), "{rows:?}");
}

/// A measurement reads in its own unit, spelled as Verify spells a value:
/// four significant digits behind an engineering prefix.
#[test]
fn a_measurement_reads_in_its_own_unit() {
    use rspice_app_state::state::{
        AnalysisResult, AnalysisType, SimulationRunLifecycle, SimulationRunProvenance,
    };
    use rspice_core::analysis::{MeasurementUnit, MeasurementUnits};

    let mut app = RSpiceApp::test_instance();
    let mut idd = rspice_core::MeasureResult::success("idd", 1.86e-3);
    idd.units = Some(MeasurementUnits {
        value: MeasurementUnit::Known("A".into()),
        raw_value: MeasurementUnit::Known("A".into()),
        axis: MeasurementUnit::Known("s".into()),
    });
    {
        let run = app.state.model.simulation.start_run();
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.success = true;
        run.analyses
            .push(AnalysisResult::new(1, AnalysisType::DcOp, "op").with_measurements(vec![idd]));
        run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
            .expect("seal run");
    }
    let outputs = snapshot_of(&mut app).outputs;
    let Outputs::Measured(rows) = &outputs else {
        panic!("the plan sets no limit: {outputs:?}");
    };
    assert_eq!(rows[0].value, "1.86 mA");
}

/// A root that checks clean reads Passed, unmarked, and leads nowhere:
/// there is nothing to show.
#[test]
fn a_clean_project_root_reads_passed_unmarked() {
    let mut app = checked(schematic_project(), &[]);
    let checks = fact(&snapshot_of(&mut app), "Checks");
    assert_eq!(
        (checks.value.as_str(), checks.tone, checks.link),
        ("Passed", Tone::Neutral, None)
    );
}

/// The Checks fact counts what stops a run apart from what only advises,
/// is the failure only for what stops a run, and opens Problems on them.
#[test]
fn the_checks_fact_counts_what_stops_a_run_apart_from_advice() {
    let mut app = checked(
        schematic_project(),
        &[
            (DrcSeverity::Info, DrcViolationType::EmptyName),
            (DrcSeverity::Error, DrcViolationType::UnconnectedPin),
            (DrcSeverity::Warning, DrcViolationType::DanglingWire),
            (DrcSeverity::Critical, DrcViolationType::MissingGround),
        ],
    );
    let checks = fact(&snapshot_of(&mut app), "Checks");
    assert_eq!(checks.value, "2 errors \u{b7} 1 warning \u{b7} 1 note");
    assert_eq!(checks.tone, Tone::Error);
    let link = checks.link.expect("findings are shown in Problems");
    assert_eq!(link.label, "Show in Problems");

    app.state.workbench.console_visible = false;
    take(&mut app, &link);

    assert!(app.state.workbench.console_visible);
    assert_eq!(app.state.workbench.console_page, ConsolePage::Problems);
}
