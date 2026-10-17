//! The press and the worker: what the frame that takes a press does, what it
//! leaves to the worker, and what Stop and an edit do to a pass in flight.

use super::engine::{add_operating_point, binned_library, enable_only, inverter_on_binned_pdk};
use super::*;

use crate::simulation::controller::PreflightStage;
use crate::simulation::execution::PreparationProgress;
use rspice_simulation_contract::run_set::RunSetDimensionKind;

/// The inverter on a binned PDK over three process sections and three
/// temperatures: nine points whose decks are worth preparing off the frame.
fn inverter_over_a_run_set() -> RSpiceApp {
    let mut app = inverter_on_binned_pdk(binned_library(None));
    enable_only(
        &mut app.state,
        &[
            RunSetDimensionKind::ProcessSection,
            RunSetDimensionKind::Temperature,
        ],
    );
    app
}

/// Wait until `workers` pipeline workers have ended, however they ended.
fn await_workers(app: &RSpiceApp, workers: usize) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(300);
    while app
        .simulation_controller
        .preflight_worker_stages()
        .iter()
        .filter(|(stage, _)| *stage == "finished")
        .count()
        < workers
    {
        assert!(
            std::time::Instant::now() < deadline,
            "the worker did not end"
        );
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

fn stated(app: &RSpiceApp, prefix: &str) -> bool {
    app.state
        .log_buffer
        .entries()
        .any(|entry| entry.message.starts_with(prefix))
}

/// The frame that takes a Run press runs the local checks and freezes the
/// run's inputs, and returns: preparing the run and building its decks are
/// the worker's, on another thread. Held before it starts, the worker has
/// prepared nothing when the press returns, and frames without its verdict
/// finish nothing. Released, it prepares and checks the run, and the frame
/// that finds the verdict queues it.
#[test]
fn a_press_returns_before_its_run_is_prepared_or_checked() {
    let mut app = inverter_over_a_run_set();
    let release = app.simulation_controller.hold_next_preflight_worker();

    run_and_queue(&mut app);

    assert!(app.state.workbench.preflight.run_awaits_check());
    assert_eq!(
        app.simulation_controller.preflight_stage(),
        Some(PreflightStage::Preparing(PreparationProgress::default()))
    );
    assert!(
        app.simulation_controller.retained_run_deck().is_none(),
        "nothing is prepared in the frame that takes the press"
    );
    poll(&mut app.state, &mut app.simulation_controller);
    assert!(
        app.state.workbench.preflight.checking.is_some(),
        "a frame without a verdict finishes nothing"
    );
    assert!(!app.state.simulation.execution.trigger_simulation);

    release.send(()).expect("the worker waits to prepare");
    app.settle_preflight();

    assert!(
        app.state.simulation.execution.trigger_simulation,
        "the authorized run is queued"
    );
    let prepared = app
        .state
        .workbench
        .preflight
        .report
        .as_ref()
        .and_then(|report| report.prepared.as_ref())
        .expect("the pass authorizes the run");
    assert_eq!(prepared.pvt_point_count, 9);
    let (_, retained) = app
        .simulation_controller
        .retained_run_deck()
        .expect("the authorized run is retained");
    assert_eq!(retained, prepared.snapshot_digest);

    await_workers(&app, 1);
    let ui = std::thread::current().id();
    let stages = app.simulation_controller.preflight_worker_stages();
    assert_eq!(
        stages.iter().map(|(stage, _)| *stage).collect::<Vec<_>>(),
        ["preparing", "engine checking", "finished"]
    );
    assert!(
        stages.iter().all(|(_, thread)| *thread != ui),
        "no stage runs on the thread that took the press: {stages:?}"
    );
}

/// Stop while the worker prepares ends the pass on the next frame. The worker
/// stops at its next stage boundary and never reaches the engine, and nothing
/// is left behind: no run retained, no report, no findings, nothing queued.
#[test]
fn stop_during_preparation_queues_nothing_and_leaves_nothing_behind() {
    use crate::workbench::app_state::diagnostic_store::DiagnosticOwner;

    let mut app = inverter_over_a_run_set();
    let release = app.simulation_controller.hold_next_preflight_worker();
    run_and_queue(&mut app);
    assert!(matches!(
        app.simulation_controller.preflight_stage(),
        Some(PreflightStage::Preparing(_))
    ));

    assert!(Command::StopSimulation.is_enabled(&app));
    Command::StopSimulation.execute(&mut app);
    poll(&mut app.state, &mut app.simulation_controller);
    release.send(()).expect("the worker waits to prepare");
    await_workers(&app, 1);
    poll(&mut app.state, &mut app.simulation_controller);

    assert!(app.state.workbench.preflight.checking.is_none());
    assert!(app.simulation_controller.preflight_stage().is_none());
    assert!(app.simulation_controller.retained_run_deck().is_none());
    assert!(app.state.workbench.preflight.report.is_none());
    assert!(!app.state.simulation.execution.trigger_simulation);
    assert!(
        !app.state
            .diagnostics
            .batches()
            .any(|batch| batch.inputs.owner() == DiagnosticOwner::Dispatch),
        "a stopped pass publishes nothing"
    );
    assert_eq!(
        app.simulation_controller
            .preflight_worker_stages()
            .iter()
            .map(|(stage, _)| *stage)
            .collect::<Vec<_>>(),
        ["preparing", "finished"],
        "the worker stops before the engine builds anything"
    );
    assert!(stated(&app, "Preflight cancelled"), "the stop is stated");
}

/// An edit while the worker prepares drops the retained run at its commit
/// boundary, as every edit does. The pass still finishes, and what it found —
/// the engine's refusal of the forced node — is published against the design
/// as it stood at the press, so it reads stale. Nothing is retained or queued.
#[test]
fn an_edit_during_preparation_queues_nothing_and_stales_what_the_pass_found() {
    use crate::state::{ComponentType, Point};
    use crate::workbench::app_state::diagnostic_store::{BatchStanding, DiagnosticOwner};

    let (mut app, _) = app_with_a_forced_capacitor_island(true);
    let release = app.simulation_controller.hold_next_preflight_worker();
    run_and_queue(&mut app);

    app.state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(300, 300));
    app.invalidate_simulation_preflight();
    release.send(()).expect("the worker waits to prepare");
    app.settle_preflight();

    assert!(!app.state.simulation.execution.trigger_simulation);
    assert!(app.simulation_controller.retained_run_deck().is_none());
    assert!(
        app.state.workbench.preflight.report.is_none(),
        "a stale pass retains no report"
    );
    let dispatch = app
        .state
        .diagnostics
        .batches()
        .find(|batch| batch.inputs.owner() == DiagnosticOwner::Dispatch)
        .expect("the pass publishes what it found")
        .clone();
    assert!(
        dispatch
            .records
            .iter()
            .any(|record| { record.message.contains("'OUT'") || record.details.contains("'OUT'") }),
        "the engine's refusal is among them: {:?}",
        dispatch
            .records
            .iter()
            .map(|record| &record.message)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        app.state.diagnostic_batch_standing(&dispatch),
        BatchStanding::Stale
    );
    assert!(stated(&app, "Preflight stale"), "the staleness is stated");
}

/// Press Run and let the pass authorize and queue the run, as the frames
/// before the one that dispatches it would. Returns the digest it authorized.
fn queued_by_its_check(app: &mut RSpiceApp) -> crate::product::ContentDigest {
    run_and_queue(app);
    app.settle_preflight();
    assert!(
        app.state.simulation.execution.trigger_simulation,
        "the pass queues the run it authorized"
    );
    app.state
        .workbench
        .preflight
        .report
        .as_ref()
        .and_then(|report| report.prepared.as_ref())
        .expect("the pass authorizes the run")
        .snapshot_digest
}

/// The frame that dispatches a queued run, with the preparations it ran.
fn dispatch_frame(
    app: &mut RSpiceApp,
) -> Vec<crate::simulation::execution::SimulationRunIntent> {
    use crate::simulation::controller::prepared_run::thread_preparations;

    let earlier = thread_preparations::take();
    assert!(
        earlier.is_empty(),
        "nothing is prepared on the UI thread before the dispatch: {earlier:?}"
    );
    app.simulation_controller
        .update(&mut app.state, &*app.export_workflow_io);
    thread_preparations::take()
}

/// Stop the run a dispatch started and wait for the engine to let it go.
fn stop_the_run(app: &mut RSpiceApp) {
    app.simulation_controller.abort();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(300);
    while app.simulation_controller.is_running() {
        assert!(
            std::time::Instant::now() < deadline,
            "the run did not stop"
        );
        app.simulation_controller
            .update(&mut app.state, &*app.export_workflow_io);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

const DESIGN_CHANGED: &str = "The design changed after it was checked; run again";

/// What a refused dispatch leaves: no run, the stated reason, the input that
/// moved named on the Console, and no report left to queue it again.
fn assert_refused_for(app: &RSpiceApp, moved: &str) {
    assert!(!app.state.simulation.execution.has_active_execution());
    assert!(app.state.simulation.retained.runs.is_empty());
    assert_eq!(app.state.simulation.execution.status, DESIGN_CHANGED);
    let stated = format!("{DESIGN_CHANGED} · {moved} changed");
    assert!(
        stated_exactly(app, &stated),
        "the Console names what moved: {stated}"
    );
    assert!(app.state.workbench.preflight.report.is_none());
    assert!(app.simulation_controller.retained_run_deck().is_none());
}

fn stated_exactly(app: &RSpiceApp, message: &str) -> bool {
    app.state
        .log_buffer
        .entries()
        .any(|entry| entry.message == message)
}

/// The frame that dispatches a checked run prepares nothing: the run the
/// pass prepared on its worker and authorized is the run that starts, with
/// the digest the pass reported.
#[test]
fn the_dispatch_frame_runs_the_checked_snapshot_and_prepares_nothing() {
    let mut app = inverter_over_a_run_set();
    let authorized = queued_by_its_check(&mut app);

    let prepared = dispatch_frame(&mut app);

    assert!(
        prepared.is_empty(),
        "the dispatch frame prepares nothing: {prepared:?}"
    );
    assert!(app.state.simulation.execution.has_active_execution());
    let dispatched = app
        .state
        .simulation
        .retained
        .runs
        .first()
        .and_then(|run| run.prepared_receipt())
        .expect("the dispatch starts a run with its receipt")
        .prepared_snapshot_digest();
    assert_eq!(dispatched, authorized, "the run is the snapshot the pass authorized");
    stop_the_run(&mut app);
}

/// An edit between the verdict and the dispatch refuses the run in the frame
/// that would have started it. The edit here is one no commit boundary
/// reported, which the dispatch check exists for; nothing is prepared to
/// find it.
#[test]
fn an_edit_after_the_check_refuses_the_dispatch() {
    let mut app = inverter_over_a_run_set();
    queued_by_its_check(&mut app);
    app.state.schematic.document_mut_for_test().components[0]
        .value
        .push('0');

    let prepared = dispatch_frame(&mut app);

    assert!(prepared.is_empty(), "the refusal prepares nothing: {prepared:?}");
    assert_refused_for(&app, "the design");
}

/// A change to either library after the check refuses the dispatch: the
/// cell library a hierarchy binds its masters through, and the model library
/// the decks seal their cards from.
#[test]
fn a_library_change_after_the_check_refuses_the_dispatch() {
    let mut cells = inverter_over_a_run_set();
    queued_by_its_check(&mut cells);
    cells
        .state
        .library_manager
        .add_library(rspice_design::library::Library::new("scratch"));
    assert!(dispatch_frame(&mut cells).is_empty());
    assert_refused_for(&cells, "the design");

    let mut models = inverter_over_a_run_set();
    queued_by_its_check(&mut models);
    models
        .state
        .model_library_manager
        .add_library(rspice_model_library::ModelLibrary {
            name: "scratch".to_owned(),
            ..Default::default()
        });
    assert!(dispatch_frame(&mut models).is_empty());
    assert_refused_for(&models, "the model library");
}

/// The operating point reads the selection only for the devices it reports
/// in detail. Selecting a symbol that writes no card while the check runs
/// moves nothing it reads, so the run still starts, as the one checked.
#[test]
fn a_selection_the_run_does_not_read_does_not_refuse_it() {
    let mut app = inverter_over_a_run_set();
    let authorized = queued_by_its_check(&mut app);
    let symbol = app
        .state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.kind.spice_prefix().is_empty())
        .expect("the inverter draws a symbol that writes no card")
        .id;
    app.state
        .schematic
        .session
        .editor
        .selection
        .components
        .insert(symbol);

    assert!(dispatch_frame(&mut app).is_empty());

    assert!(app.state.simulation.execution.has_active_execution());
    assert_eq!(
        app.state
            .simulation
            .retained
            .runs
            .first()
            .and_then(|run| run.prepared_receipt())
            .map(|receipt| receipt.prepared_snapshot_digest()),
        Some(authorized)
    );
    stop_the_run(&mut app);
}

/// Selecting a device that writes a card names it in the operating point's
/// report, so the operating point the check prepared is no longer the one the
/// design asks for, and the dispatch is refused.
#[test]
fn selecting_a_device_refuses_a_run_with_an_operating_point() {
    let mut app = inverter_over_a_run_set();
    queued_by_its_check(&mut app);
    let pull_down = app
        .state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.name == "MN")
        .expect("the inverter draws MN")
        .id;
    app.state
        .schematic
        .session
        .editor
        .selection
        .components
        .insert(pull_down);

    assert!(dispatch_frame(&mut app).is_empty());
    assert_refused_for(&app, "the devices or results the operating point reports");
}

/// Automation prepares its run in its own frame, from the live session, and
/// queues it; the frame that dispatches it checks it as it checks a run a
/// Run press prepared. Unchanged, the run Automation prepared is the run that
/// starts, and nothing is prepared again; edited, it is refused.
#[test]
fn the_run_automation_prepares_is_checked_at_dispatch_as_a_run_press_is() {
    use crate::simulation::controller::prepared_run::thread_preparations;

    fn prepared_as_automation_prepares(app: &mut RSpiceApp) -> crate::product::ContentDigest {
        app.invalidate_simulation_preflight();
        let prepared = app
            .simulation_controller
            .prepare_run_set_for_preflight(&app.state)
            .expect("the inverter prepares");
        app.state.request_run_set_simulation();
        assert_eq!(
            thread_preparations::take(),
            [crate::simulation::execution::SimulationRunIntent::SimulateRunSet],
            "Automation prepares its run in its own frame"
        );
        prepared.snapshot_digest
    }

    let mut unchanged = inverter_over_a_run_set();
    let authorized = prepared_as_automation_prepares(&mut unchanged);
    assert!(dispatch_frame(&mut unchanged).is_empty());
    assert_eq!(
        unchanged
            .state
            .simulation
            .retained
            .runs
            .first()
            .and_then(|run| run.prepared_receipt())
            .map(|receipt| receipt.prepared_snapshot_digest()),
        Some(authorized),
        "the run is the snapshot Automation prepared"
    );
    stop_the_run(&mut unchanged);

    let mut edited = inverter_over_a_run_set();
    prepared_as_automation_prepares(&mut edited);
    edited.state.schematic.document_mut_for_test().components[0]
        .value
        .push('0');
    assert!(dispatch_frame(&mut edited).is_empty());
    assert_refused_for(&edited, "the design");
}

/// The run the worker prepares from the inputs frozen at the press is the run
/// the frame would have prepared from the live state: the same snapshot, by
/// digest, and the same deck, byte for byte. One example runs a single point;
/// the other a Run Set of nine.
#[test]
fn a_run_prepared_off_the_ui_thread_is_the_one_the_frame_would_have_prepared() {
    let mut lowpass = RSpiceApp::test_instance();
    lowpass.state.project_lifecycle.authority.open_session();
    enable_only(&mut lowpass.state, &[]);
    crate::workbench::examples::load_example("RC Lowpass Filter", &mut lowpass.state.schematic);
    add_operating_point(&mut lowpass.state);

    for (label, mut app, points) in [
        ("RC lowpass", lowpass, 1),
        ("inverter over a Run Set", inverter_over_a_run_set(), 9),
    ] {
        run(&mut app);
        app.settle_preflight();
        let prepared = app
            .state
            .workbench
            .preflight
            .report
            .as_ref()
            .and_then(|report| report.prepared.clone())
            .unwrap_or_else(|| panic!("{label}: the pass authorizes the run"));
        assert_eq!(prepared.pvt_point_count, points, "{label}");
        let (deck, digest) = app
            .simulation_controller
            .retained_run_deck()
            .map(|(deck, digest)| (deck.to_owned(), digest))
            .unwrap_or_else(|| panic!("{label}: the run is retained"));
        assert_eq!(digest, prepared.snapshot_digest, "{label}");

        let synchronous = app
            .simulation_controller
            .prepare_run_set_for_preflight(&app.state)
            .unwrap_or_else(|error| panic!("{label}: {error}"));
        assert_eq!(synchronous.snapshot_digest, digest, "{label}");
        let (synchronous_deck, _) = app
            .simulation_controller
            .retained_run_deck()
            .expect("the synchronous run is retained");
        assert_eq!(synchronous_deck, deck, "{label}");
    }
}
