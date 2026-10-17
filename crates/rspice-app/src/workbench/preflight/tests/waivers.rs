//! A waived error is a decision the run honours: the page, preflight and the
//! run gate let it through alike, and taking the waiver away refuses the run
//! again.

use super::*;
use crate::state::Point;
use crate::workbench::app_state::diagnostic_store::DiagnosticInputs;
use crate::workbench::app_state::waivers::{WaiverAction, WaiverTarget};
use rspice_simulation::execution::PreparationStage;

const REASON: &str = "sense is tied to SENSE on purpose";

/// The voltage divider, runnable as it stands, with two labels the deck
/// folds into one node: an error by default, and one a waiver may accept.
fn colliding_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    disable_global_process_axis(&mut app.state);
    crate::workbench::examples::load_example("Voltage Divider", &mut app.state.schematic);
    app.state.provision_test_project_technology_contract();
    app.state
        .schematic
        .add_net_label(Point::new(600, 600), "sense".to_owned());
    app.state
        .schematic
        .add_net_label(Point::new(600, 700), "SENSE".to_owned());
    app.state.sync_active_schematic_to_workspace();
    app
}

/// Run the Check command and return where the page would waive the case
/// collision it finds, and whether that finding is listed as waived.
fn collision(state: &mut AppState) -> (Option<WaiverTarget>, bool) {
    state
        .run_active_design_checks()
        .expect("the checks run on the drawing");
    let subject = state.workspace.content.active_schematic_reference();
    let batch = state
        .diagnostics
        .batch(&DiagnosticInputs::DesignCheck { subject })
        .expect("the checks published their findings");
    let record = batch
        .records
        .iter()
        .find(|record| &*record.key.code == "DRC-CASE-COLLIDING-NET-NAMES")
        .expect("the collision is found");
    let target = match &record.waiver_action {
        Some(WaiverAction::Waive(target)) => Some(target.clone()),
        _ => None,
    };
    (target, record.waived())
}

fn blocked(app: &RSpiceApp) -> bool {
    app.state
        .workbench
        .preflight
        .report
        .as_ref()
        .is_some_and(|report| !report.blockers.is_empty())
}

/// Through the Run command's own path: preflight, then the queue. The error
/// refuses the run; waived, it is run, and the snapshot the run executes is
/// prepared under the waiver.
#[test]
fn a_waived_error_is_run_and_an_unwaived_one_is_refused() {
    let mut app = colliding_app();
    let (target, waived) = collision(&mut app.state);
    assert!(!waived);
    let target = target.expect("a case collision may be waived");

    run_and_queue(&mut app);
    assert!(blocked(&app), "the error blocks preflight");
    assert!(
        !app.state.simulation.execution.trigger_simulation,
        "and the run is not queued"
    );

    app.state
        .waive_finding(&target, REASON)
        .expect("a case collision may be waived");
    assert!(
        app.state.workbench.preflight.report.is_none(),
        "the report that refused the run no longer stands"
    );
    assert!(collision(&mut app.state).1, "the page lists it waived");

    // Clean of local blockers, the pass ends with the engine building the
    // decks on a worker, and the run is queued when the frame loop finds
    // its verdict.
    run_and_queue(&mut app);
    app.settle_preflight();
    assert!(!blocked(&app), "{:?}", app.state.workbench.preflight.report);
    assert!(
        app.state.simulation.execution.trigger_simulation,
        "the waived error is run"
    );
    assert!(
        app.simulation_controller.retained_run_deck().is_some(),
        "under the snapshot preflight prepared"
    );
}

/// The run gate itself, which every run path passes whether or not preflight
/// ran first: it refuses the error, passes it waived, and refuses it again
/// once the waiver is taken away.
#[test]
fn the_run_gate_follows_the_waiver_both_ways() {
    let mut app = colliding_app();
    let target = collision(&mut app.state)
        .0
        .expect("a case collision may be waived");
    assert_eq!(
        gate(&mut app.simulation_controller, &app.state),
        Some(PreparationStage::DesignChecks)
    );

    app.state
        .waive_finding(&target, REASON)
        .expect("a case collision may be waived");
    assert_eq!(
        gate(&mut app.simulation_controller, &app.state),
        None,
        "a waived error travels with the run"
    );

    app.state
        .remove_finding_waiver(&target)
        .expect("the waiver is removed");
    assert_eq!(
        gate(&mut app.simulation_controller, &app.state),
        Some(PreparationStage::DesignChecks),
        "without its waiver the error refuses the run again"
    );
}

/// What the run gate does with the design: `None` where it prepares, the
/// stage it refused at otherwise.
fn gate(
    controller: &mut crate::simulation::SimulationController,
    state: &AppState,
) -> Option<PreparationStage> {
    controller
        .prepare_run_set_for_preflight(state)
        .err()
        .map(|error| error.stage())
}
