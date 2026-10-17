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

/// A second Run press while the design is as the pass in flight froze it
/// joins that pass: one worker prepares one run, and it is queued once.
#[test]
fn a_second_run_press_on_the_unedited_design_joins_the_pass_in_flight() {
    let mut app = inverter_over_a_run_set();
    let release = app.simulation_controller.hold_next_preflight_worker();
    run(&mut app);
    run_and_queue(&mut app);
    assert!(
        app.state.workbench.preflight.run_awaits_check(),
        "the press asks the pass in flight for its run"
    );
    release.send(()).expect("the worker waits to prepare");
    app.settle_preflight();

    assert!(app.state.simulation.execution.trigger_simulation);
    await_workers(&app, 1);
    assert_eq!(
        app.simulation_controller
            .preflight_worker_stages()
            .iter()
            .filter(|(stage, _)| *stage == "preparing")
            .count(),
        1,
        "one pass prepares the run"
    );
    assert!(!stated(&app, "Preflight restarted"));
}

/// A Run press made after an edit, while the pass the edit outdated is still
/// preparing, is not absorbed into that pass, whose verdict would authorize
/// nothing: the stale pass stops, and the press checks, prepares and queues
/// the design as edited. The edit is the supply's value, so the deck the run
/// is retained with shows which design was prepared.
#[test]
fn a_run_press_after_an_edit_restarts_the_pass_on_the_edited_design() {
    let mut app = inverter_over_a_run_set();
    let release = app.simulation_controller.hold_next_preflight_worker();
    run_and_queue(&mut app);

    app.state
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.name == "VDD")
        .expect("the inverter draws VDD")
        .value = "3.25".to_owned();
    app.state.schematic.bump_topology_version();
    app.invalidate_simulation_preflight();
    run_and_queue(&mut app);
    release.send(()).expect("the stale worker waits to prepare");
    app.settle_preflight();

    assert!(stated(&app, "Preflight restarted"), "the restart is stated");
    assert!(
        !stated(&app, "Preflight stale"),
        "the press was not absorbed into the stale pass"
    );
    assert!(
        app.state.simulation.execution.trigger_simulation,
        "the edited design's run is queued"
    );
    let report = app
        .state
        .workbench
        .preflight
        .report
        .clone()
        .expect("the pass retains its report");
    let (topology_root, topology_revision, topology_closure) =
        app.state.configured_topology_revision();
    assert!(report.is_runnable_for(
        app.state.workspace.content.project.revision().get(),
        &topology_root,
        topology_revision,
        &topology_closure,
        app.state.active_plan_revision(),
    ));
    let (deck, digest) = app
        .simulation_controller
        .retained_run_deck()
        .expect("the edited design's run is retained");
    assert_eq!(
        Some(digest),
        report
            .prepared
            .as_ref()
            .map(|prepared| prepared.snapshot_digest)
    );
    assert!(
        deck.contains("3.25"),
        "the edited supply is prepared: {deck}"
    );
}

/// The frames that queue a run and dispatch it, driven on the application
/// they run in.
impl RSpiceApp {
    /// Press Run and let the pass authorize and queue the run, as the frames
    /// before the one that dispatches it would. Returns the digest it
    /// authorized.
    fn queued_by_its_check(&mut self) -> crate::product::ContentDigest {
        run_and_queue(self);
        self.settle_preflight();
        assert!(
            self.state.simulation.execution.trigger_simulation,
            "the pass queues the run it authorized"
        );
        self.state
            .workbench
            .preflight
            .report
            .as_ref()
            .and_then(|report| report.prepared.as_ref())
            .expect("the pass authorizes the run")
            .snapshot_digest
    }

    /// Prepare and queue a run as Automation does, in this frame and from
    /// the live session. Returns the digest it prepared, and what Automation
    /// keeps of what the run was prepared from for its evidence check.
    fn queued_as_automation_queues(
        &mut self,
    ) -> (
        crate::product::ContentDigest,
        crate::simulation::controller::PreparedRunSetInputs,
    ) {
        use crate::simulation::controller::prepared_run::thread_preparations;

        self.invalidate_simulation_preflight();
        let (prepared, inputs) = self
            .simulation_controller
            .prepare_run_set_for_evidence(&self.state)
            .expect("the design prepares");
        self.state.request_run_set_simulation();
        assert_eq!(
            thread_preparations::take(),
            [crate::simulation::execution::SimulationRunIntent::SimulateRunSet],
            "Automation prepares its run in its own frame"
        );
        (prepared.snapshot_digest, inputs)
    }

    /// Automation's evidence check, with the preparations it ran.
    fn evidence_check(
        &self,
        inputs: &crate::simulation::controller::PreparedRunSetInputs,
    ) -> (
        Result<(), crate::simulation::execution::PreparationError>,
        Vec<crate::simulation::execution::SimulationRunIntent>,
    ) {
        use crate::simulation::controller::prepared_run::thread_preparations;

        let earlier = thread_preparations::take();
        assert!(earlier.is_empty(), "{earlier:?}");
        let verdict = inputs.still_current(&self.state);
        (verdict, thread_preparations::take())
    }

    /// Let the run a dispatch started finish, as the frames after it would.
    fn finish_the_run(&mut self) {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(300);
        while self.simulation_controller.is_running()
            || self.state.simulation.execution.has_active_execution()
        {
            assert!(std::time::Instant::now() < deadline, "the run did not end");
            self.simulation_controller
                .update(&mut self.state, &*self.export_workflow_io);
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    /// The frame that dispatches a queued run, with the preparations it ran.
    fn dispatch_frame(&mut self) -> Vec<crate::simulation::execution::SimulationRunIntent> {
        use crate::simulation::controller::prepared_run::thread_preparations;

        let earlier = thread_preparations::take();
        assert!(
            earlier.is_empty(),
            "nothing is prepared on the UI thread before the dispatch: {earlier:?}"
        );
        self.simulation_controller
            .update(&mut self.state, &*self.export_workflow_io);
        thread_preparations::take()
    }

    /// Stop the run a dispatch started and wait for the engine to let it go.
    fn stop_the_run(&mut self) {
        self.simulation_controller.abort();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(300);
        while self.simulation_controller.is_running() {
            assert!(std::time::Instant::now() < deadline, "the run did not stop");
            self.simulation_controller
                .update(&mut self.state, &*self.export_workflow_io);
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
}

const DESIGN_CHANGED: &str = "The design changed after it was checked; run again";

/// What a refused dispatch leaves: no run, the stated reason, the input that
/// moved named on the status bar, and no report left to queue it again.
fn assert_refused_for(app: &RSpiceApp, moved: &str) {
    assert!(!app.state.simulation.execution.has_active_execution());
    assert!(app.state.simulation.retained.runs.is_empty());
    assert_eq!(app.state.simulation.execution.status, DESIGN_CHANGED);
    let stated = format!("{DESIGN_CHANGED} · {moved} changed");
    assert_eq!(
        app.state.prompt_text(),
        Some(stated.as_str()),
        "the status bar names what moved"
    );
    assert!(app.state.workbench.preflight.report.is_none());
    assert!(app.simulation_controller.retained_run_deck().is_none());
}

/// The frame that dispatches a checked run prepares nothing: the run the
/// pass prepared on its worker and authorized is the run that starts, with
/// the digest the pass reported.
#[test]
fn the_dispatch_frame_runs_the_checked_snapshot_and_prepares_nothing() {
    let mut app = inverter_over_a_run_set();
    let authorized = app.queued_by_its_check();

    let prepared = app.dispatch_frame();

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
    assert_eq!(
        dispatched, authorized,
        "the run is the snapshot the pass authorized"
    );
    app.stop_the_run();
}

/// An edit between the verdict and the dispatch refuses the run in the frame
/// that would have started it. The edit here is one no commit boundary
/// reported, which the dispatch check exists for; nothing is prepared to
/// find it.
#[test]
fn an_edit_after_the_check_refuses_the_dispatch() {
    let mut app = inverter_over_a_run_set();
    app.queued_by_its_check();
    app.state.schematic.document_mut_for_test().components[0]
        .value
        .push('0');

    let prepared = app.dispatch_frame();

    assert!(
        prepared.is_empty(),
        "the refusal prepares nothing: {prepared:?}"
    );
    assert_refused_for(&app, "the design");
}

/// A change to either library after the check refuses the dispatch: the
/// cell library a hierarchy binds its masters through, and the model library
/// the decks seal their cards from.
#[test]
fn a_library_change_after_the_check_refuses_the_dispatch() {
    let mut cells = inverter_over_a_run_set();
    cells.queued_by_its_check();
    cells
        .state
        .library_manager
        .add_library(rspice_design::library::Library::new("scratch"));
    assert!(cells.dispatch_frame().is_empty());
    assert_refused_for(&cells, "the design");

    let mut models = inverter_over_a_run_set();
    models.queued_by_its_check();
    models
        .state
        .model_library_manager
        .add_library(rspice_model_library::ModelLibrary {
            name: "scratch".to_owned(),
            ..Default::default()
        });
    assert!(models.dispatch_frame().is_empty());
    assert_refused_for(&models, "the model library");
}

/// The operating point reads the selection only for the devices it reports
/// in detail. Selecting a symbol that writes no card while the check runs
/// moves nothing it reads, so the run still starts, as the one checked.
#[test]
fn a_selection_the_run_does_not_read_does_not_refuse_it() {
    let mut app = inverter_over_a_run_set();
    let authorized = app.queued_by_its_check();
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

    assert!(app.dispatch_frame().is_empty());

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
    app.stop_the_run();
}

/// Selecting a device that writes a card names it in the operating point's
/// report, so the operating point the check prepared is no longer the one the
/// design asks for, and the dispatch is refused.
#[test]
fn selecting_a_device_refuses_a_run_with_an_operating_point() {
    let mut app = inverter_over_a_run_set();
    app.queued_by_its_check();
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

    assert!(app.dispatch_frame().is_empty());
    assert_refused_for(&app, "the devices or results the operating point reports");
}

/// Automation prepares its run in its own frame, from the live session, and
/// queues it; the frame that dispatches it checks it as it checks a run a
/// Run press prepared. Unchanged, the run Automation prepared is the run that
/// starts, and nothing is prepared again; edited, it is refused.
#[test]
fn the_run_automation_prepares_is_checked_at_dispatch_as_a_run_press_is() {
    let mut unchanged = inverter_over_a_run_set();
    let (authorized, _) = unchanged.queued_as_automation_queues();
    assert!(unchanged.dispatch_frame().is_empty());
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
    unchanged.stop_the_run();

    let mut edited = inverter_over_a_run_set();
    edited.queued_as_automation_queues();
    edited.state.schematic.document_mut_for_test().components[0]
        .value
        .push('0');
    assert!(edited.dispatch_frame().is_empty());
    assert_refused_for(&edited, "the design");
}

/// Automation checks its evidence is still the design's on each frame it
/// waits for the dispatch and again when it publishes. On an unedited design
/// the check passes at each of those frames, before the dispatch, while the
/// run is in flight and once it has finished, and prepares nothing: the run
/// Automation dispatched does not expire the evidence it produces.
#[test]
fn an_automation_evidence_check_on_an_unedited_design_prepares_nothing() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    enable_only(&mut app.state, &[]);
    crate::workbench::examples::load_example("RC Lowpass Filter", &mut app.state.schematic);
    let (authorized, inputs) = app.queued_as_automation_queues();

    let (before_dispatch, prepared) = app.evidence_check(&inputs);
    assert_eq!(before_dispatch, Ok(()));
    assert!(prepared.is_empty(), "{prepared:?}");

    assert!(app.dispatch_frame().is_empty());
    assert!(app.state.simulation.execution.has_active_execution());
    let (in_flight, prepared) = app.evidence_check(&inputs);
    assert_eq!(in_flight, Ok(()));
    assert!(prepared.is_empty(), "{prepared:?}");

    app.finish_the_run();
    let run = app
        .state
        .simulation
        .retained
        .runs
        .first()
        .expect("the run is retained");
    assert!(run.success, "{:?}", run.analyses);
    assert_eq!(
        run.prepared_receipt()
            .map(|receipt| receipt.prepared_snapshot_digest()),
        Some(authorized)
    );
    let (at_publication, prepared) = app.evidence_check(&inputs);
    assert_eq!(at_publication, Ok(()));
    assert!(prepared.is_empty(), "{prepared:?}");
}

/// An edit made while Automation's run is in flight expires its evidence, and
/// the check names the input that moved in the words a dispatch refused for
/// it uses. Nothing is prepared to find it.
#[test]
fn an_edit_after_automation_prepared_its_run_is_named_by_its_evidence_check() {
    let mut app = inverter_over_a_run_set();
    let (_, inputs) = app.queued_as_automation_queues();
    assert!(app.dispatch_frame().is_empty());
    assert!(app.state.simulation.execution.has_active_execution());

    app.state.schematic.document_mut_for_test().components[0]
        .value
        .push('0');
    let (verdict, prepared) = app.evidence_check(&inputs);

    assert!(prepared.is_empty(), "{prepared:?}");
    let error = verdict.expect_err("the edited design is not the one the run solves");
    assert_eq!(
        error.stage(),
        crate::simulation::execution::PreparationStage::Authorization
    );
    assert_eq!(
        error.message(),
        format!("{DESIGN_CHANGED} · the design changed")
    );
    app.stop_the_run();
}

/// For the same edit, Automation's evidence check and the dispatch of the
/// run it queued refuse alike, in the same words: the evidence check is the
/// dispatch's own question, not a second judgement of the design.
#[test]
fn the_evidence_check_and_the_dispatch_agree_for_the_same_edit() {
    let edits: [(&str, fn(&mut RSpiceApp)); 3] = [
        ("the design", |app| {
            app.state.schematic.document_mut_for_test().components[0]
                .value
                .push('0');
        }),
        ("the model library", |app| {
            app.state
                .model_library_manager
                .add_library(rspice_model_library::ModelLibrary {
                    name: "scratch".to_owned(),
                    ..Default::default()
                });
        }),
        ("the simulation setup", |app| {
            app.state
                .sim_setup
                .set_reference_pvt(crate::product::ProcessCorner::TT, 125.0)
                .expect("a physical temperature");
        }),
    ];
    for (moved, edit) in edits {
        let mut app = inverter_over_a_run_set();
        let (_, inputs) = app.queued_as_automation_queues();
        edit(&mut app);

        let (verdict, prepared) = app.evidence_check(&inputs);
        assert!(prepared.is_empty(), "{moved}: {prepared:?}");
        let evidence = verdict.expect_err(moved).to_string();
        assert!(app.dispatch_frame().is_empty(), "{moved}");

        assert_refused_for(&app, moved);
        assert_eq!(
            app.state.prompt_text(),
            Some(evidence.as_str()),
            "the evidence check and the dispatch name {moved} alike"
        );
    }
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
