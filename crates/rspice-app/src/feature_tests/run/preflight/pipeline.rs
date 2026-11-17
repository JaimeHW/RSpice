//! The press and the worker: what the frame that takes a press does, what it
//! leaves to the worker, and what Stop and an edit do to a pass in flight.

use super::engine::{add_operating_point, binned_library, enable_only, inverter_on_binned_pdk};
use super::*;

use rspice_app_runs::simulation::controller::PreflightStage;
use rspice_simulation::execution::PreparationProgress;
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
        .model
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

    start(&mut app.state, &mut app.simulation_controller, true);

    assert!(app.state.runs.preflight.run_awaits_check());
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
        app.state.runs.preflight.checking.is_some(),
        "a frame without a verdict finishes nothing"
    );
    assert!(!app.state.model.simulation.execution.trigger_simulation);

    release.send(()).expect("the worker waits to prepare");
    app.settle_preflight();

    assert!(
        app.state.model.simulation.execution.trigger_simulation,
        "the authorized run is queued"
    );
    let prepared = app
        .state
        .runs
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
    use rspice_app_state::workbench::app_state::diagnostic_store::DiagnosticOwner;

    let mut app = inverter_over_a_run_set();
    let release = app.simulation_controller.hold_next_preflight_worker();
    start(&mut app.state, &mut app.simulation_controller, true);
    assert!(matches!(
        app.simulation_controller.preflight_stage(),
        Some(PreflightStage::Preparing(_))
    ));

    assert!(app.command_key_enabled(SimulateCommand::Stop.key()));
    app.execute_command_key(SimulateCommand::Stop.key());
    poll(&mut app.state, &mut app.simulation_controller);
    release.send(()).expect("the worker waits to prepare");
    await_workers(&app, 1);
    poll(&mut app.state, &mut app.simulation_controller);

    assert!(app.state.runs.preflight.checking.is_none());
    assert!(app.simulation_controller.preflight_stage().is_none());
    assert!(app.simulation_controller.retained_run_deck().is_none());
    assert!(app.state.runs.preflight.report.is_none());
    assert!(!app.state.model.simulation.execution.trigger_simulation);
    assert!(
        !app.state
            .model
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
    use rspice_app_state::state::{ComponentType, Point};
    use rspice_app_state::workbench::app_state::diagnostic_store::{
        BatchStanding, DiagnosticOwner,
    };

    let (mut app, _) = app_with_a_forced_capacitor_island(true);
    let release = app.simulation_controller.hold_next_preflight_worker();
    start(&mut app.state, &mut app.simulation_controller, true);

    app.state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(300, 300));
    app.invalidate_simulation_preflight();
    release.send(()).expect("the worker waits to prepare");
    app.settle_preflight();

    assert!(!app.state.model.simulation.execution.trigger_simulation);
    assert!(app.simulation_controller.retained_run_deck().is_none());
    assert!(
        app.state.runs.preflight.report.is_none(),
        "a stale pass retains no report"
    );
    let dispatch = app
        .state
        .model
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
    start(&mut app.state, &mut app.simulation_controller, false);
    start(&mut app.state, &mut app.simulation_controller, true);
    assert!(
        app.state.runs.preflight.run_awaits_check(),
        "the press asks the pass in flight for its run"
    );
    release.send(()).expect("the worker waits to prepare");
    app.settle_preflight();

    assert!(app.state.model.simulation.execution.trigger_simulation);
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
    start(&mut app.state, &mut app.simulation_controller, true);

    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.name == "VDD")
        .expect("the inverter draws VDD")
        .value = "3.25".to_owned();
    app.state.model.schematic.bump_topology_version();
    app.invalidate_simulation_preflight();
    start(&mut app.state, &mut app.simulation_controller, true);
    release.send(()).expect("the stale worker waits to prepare");
    app.settle_preflight();

    assert!(stated(&app, "Preflight restarted"), "the restart is stated");
    assert!(
        !stated(&app, "Preflight stale"),
        "the press was not absorbed into the stale pass"
    );
    assert!(
        app.state.model.simulation.execution.trigger_simulation,
        "the edited design's run is queued"
    );
    let report = app
        .state
        .runs
        .preflight
        .report
        .clone()
        .expect("the pass retains its report");
    let (topology_root, topology_revision, topology_closure) =
        app.state.model.configured_topology_revision();
    assert!(report.is_runnable_for(
        app.state.model.workspace.content.project.revision().get(),
        &topology_root,
        topology_revision,
        &topology_closure,
        app.state.model.active_plan_revision(),
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
    pub(super) fn queued_by_its_check(&mut self) -> rspice_app_state::product::ContentDigest {
        start(&mut self.state, &mut self.simulation_controller, true);
        self.settle_preflight();
        assert!(
            self.state.model.simulation.execution.trigger_simulation,
            "the pass queues the run it authorized"
        );
        self.state
            .runs
            .preflight
            .report
            .as_ref()
            .and_then(|report| report.prepared.as_ref())
            .expect("the pass authorizes the run")
            .snapshot_digest
    }

    /// Prepare and queue a run in this frame, from the live session, rather
    /// than on the preflight worker. Returns the digest it prepared.
    fn queued_from_this_frame(&mut self) -> rspice_app_state::product::ContentDigest {
        use rspice_app_runs::simulation::controller::prepared_run::thread_preparations;

        self.invalidate_simulation_preflight();
        let prepared = self
            .simulation_controller
            .prepare_run_set_for_preflight(&self.state)
            .expect("the design prepares");
        self.state.request_run_set_simulation();
        assert_eq!(
            thread_preparations::take(),
            [rspice_simulation::execution::SimulationRunIntent::SimulateRunSet],
            "the run is prepared in this frame"
        );
        prepared.snapshot_digest
    }

    /// Let the run a dispatch started finish, as the frames after it would.
    pub(super) fn finish_the_run(&mut self) {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(300);
        while self.simulation_controller.is_running()
            || self.state.model.simulation.execution.has_active_execution()
        {
            assert!(std::time::Instant::now() < deadline, "the run did not end");
            self.simulation_controller
                .update(&mut self.state, &*self.export_workflow_io);
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    /// The frame that dispatches a queued run, with the preparations it ran.
    pub(super) fn dispatch_frame(
        &mut self,
    ) -> Vec<rspice_simulation::execution::SimulationRunIntent> {
        use rspice_app_runs::simulation::controller::prepared_run::thread_preparations;

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
    assert!(!app.state.model.simulation.execution.has_active_execution());
    assert!(app.state.model.simulation.retained.runs.is_empty());
    assert_eq!(app.state.model.simulation.execution.status, DESIGN_CHANGED);
    let stated = format!("{DESIGN_CHANGED} · {moved} changed");
    assert_eq!(
        app.state.prompt_text(),
        Some(stated.as_str()),
        "the status bar names what moved"
    );
    assert!(app.state.runs.preflight.report.is_none());
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
    let dispatched = app
        .state
        .model
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
    app.state.model.schematic.document_mut_for_test().components[0]
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
        .model
        .library_manager
        .add_library(rspice_design::library::Library::new("scratch"));
    assert!(cells.dispatch_frame().is_empty());
    assert_refused_for(&cells, "the design");

    let mut models = inverter_over_a_run_set();
    models.queued_by_its_check();
    models
        .state
        .model
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
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.kind.spice_prefix().is_empty())
        .expect("the inverter draws a symbol that writes no card")
        .id;
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .components
        .insert(symbol);

    assert!(app.dispatch_frame().is_empty());

    assert_eq!(
        app.state
            .model
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
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.name == "MN")
        .expect("the inverter draws MN")
        .id;
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .components
        .insert(pull_down);

    assert!(app.dispatch_frame().is_empty());
    assert_refused_for(&app, "the devices or results the operating point reports");
}

/// A run prepared in its own frame, from the live session, and queued is
/// checked by the frame that dispatches it as a run a Run press prepared is.
/// Unchanged, it is the run that starts, and nothing is prepared again;
/// edited, it is refused.
#[test]
fn a_run_prepared_in_the_frame_is_checked_at_dispatch_as_a_run_press_is() {
    let mut unchanged = inverter_over_a_run_set();
    let authorized = unchanged.queued_from_this_frame();
    assert!(unchanged.dispatch_frame().is_empty());
    assert_eq!(
        unchanged
            .state
            .model
            .simulation
            .retained
            .runs
            .first()
            .and_then(|run| run.prepared_receipt())
            .map(|receipt| receipt.prepared_snapshot_digest()),
        Some(authorized),
        "the run is the snapshot the frame prepared"
    );
    unchanged.stop_the_run();

    let mut edited = inverter_over_a_run_set();
    edited.queued_from_this_frame();
    edited
        .state
        .model
        .schematic
        .document_mut_for_test()
        .components[0]
        .value
        .push('0');
    assert!(edited.dispatch_frame().is_empty());
    assert_refused_for(&edited, "the design");
}

/// The RC lowpass example, its transient alone at the reference point: a run
/// that finishes in moments.
fn rc_lowpass() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    enable_only(&mut app.state, &[]);
    rspice_app_state::workbench::examples::load_example(
        "RC Lowpass Filter",
        &mut app.state.model.schematic,
    );
    app
}

/// Edit the plan's transient as `edit` does.
pub(super) fn edit_the_transient(
    state: &mut AppState,
    edit: impl FnOnce(&mut rspice_simulation_contract::drafts::TranSetup),
) {
    use rspice_app_state::simulation::plan::AnalysisDraft;

    let plan = state
        .model
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("a default state owns a stable plan");
    let transient = plan
        .instances()
        .iter()
        .find(|instance| matches!(instance.draft(), AnalysisDraft::Transient(_)))
        .expect("the example runs a transient")
        .id();
    plan.edit(transient, |draft| {
        if let AnalysisDraft::Transient(draft) = draft {
            edit(draft);
        }
    })
    .expect("the transient takes the edit");
}

impl RSpiceApp {
    /// Run the RC lowpass's transient to 20 µs, capturing a checkpoint every
    /// 10 µs. Returns the digest of the last checkpoint the run captured.
    fn ran_capturing_checkpoints(&mut self) -> rspice_app_state::product::ContentDigest {
        edit_the_transient(&mut self.state, |draft| {
            draft.stop = "20u".to_owned();
            draft.step = "100n".to_owned();
            draft.max_step = "100n".to_owned();
            draft.checkpoint = Some(
                rspice_simulation_contract::tran_checkpoint::TranCheckpointDraft {
                    interval: Some("10u".to_owned()),
                    resume: Vec::new(),
                },
            );
        });
        self.queued_from_this_frame();
        assert!(self.dispatch_frame().is_empty());
        self.finish_the_run();
        let run = self
            .state
            .model
            .simulation
            .retained
            .runs
            .first()
            .expect("the run is retained");
        assert!(run.success, "{:?}", run.analyses);
        run.analyses
            .iter()
            .find_map(|analysis| analysis.transient_checkpoint.as_ref())
            .expect("the run captured a checkpoint")
            .digest()
    }

    /// Run the RC lowpass's transient as [`Self::ran_capturing_checkpoints`]
    /// does, then queue the transient resuming from the checkpoint that run
    /// captured at 20 µs and running on to 40 µs. Returns the digest prepared.
    fn queued_resuming_from_a_checkpoint(&mut self) -> rspice_app_state::product::ContentDigest {
        let image = self.ran_capturing_checkpoints();
        edit_the_transient(&mut self.state, |draft| {
            draft.stop = "40u".to_owned();
            if let Some(checkpoint) = draft.checkpoint.as_mut() {
                checkpoint.resume = vec![image];
            }
        });
        self.queued_from_this_frame()
    }
}

/// A transient that resumes from a checkpoint reads the run that holds it,
/// and nothing else of the history. The run it dispatches joins the history
/// beside that run and captures checkpoints of its own.
#[test]
fn a_run_resuming_from_a_checkpoint_joins_the_run_it_resumes_from() {
    let mut app = rc_lowpass();
    let authorized = app.queued_resuming_from_a_checkpoint();

    assert!(app.dispatch_frame().is_empty());
    app.finish_the_run();
    let runs = &app.state.model.simulation.retained.runs;
    app.finish_the_run();
    let runs = &app.state.model.simulation.retained.runs;
    assert_eq!(runs.len(), 2, "the run joins the one it resumes from");
    assert!(runs[0].success, "{:?}", runs[0].analyses);
    assert_eq!(
        runs[0]
            .prepared_receipt()
            .map(|receipt| receipt.prepared_snapshot_digest()),
        Some(authorized)
    );
    assert!(
        runs[0]
            .analyses
            .iter()
            .any(|analysis| analysis.transient_checkpoint.is_some()),
        "the resumed run captures a checkpoint of its own"
    );
}

/// A run added to the history that holds nothing the transient resumes from
/// changes nothing it reads: the dispatch starts the run the frame prepared.
#[test]
fn an_unrelated_run_added_to_the_history_leaves_a_resuming_run_current() {
    let mut app = rc_lowpass();
    let authorized = app.queued_resuming_from_a_checkpoint();
    app.state.model.simulation.start_run();

    assert!(app.dispatch_frame().is_empty());
    assert_eq!(
        app.state
            .model
            .simulation
            .retained
            .runs
            .first()
            .and_then(|run| run.prepared_receipt())
            .map(|receipt| receipt.prepared_snapshot_digest()),
        Some(authorized),
        "the run is the snapshot the frame prepared"
    );
    app.stop_the_run();
}

/// Removing the run a checkpoint resumes from leaves nothing to resume from;
/// replacing it with a run whose transient no longer carries the prepared
/// provenance a checkpoint is resumed against, as a result read from a file
/// does not, leaves a checkpoint that cannot be resumed. Either way what the
/// transient resumes from has changed: the dispatch of the run it queued
/// refuses it, naming what moved, and nothing is prepared to find it.
#[test]
fn removing_or_replacing_the_run_a_checkpoint_resumes_from_refuses_the_dispatch() {
    /// An edit of the history, beside its name.
    type Edit = (&'static str, fn(&mut AppState));
    let edits: [Edit; 2] = [
        ("removed", |state| {
            state.model.simulation.retained.runs.remove(0);
        }),
        ("replaced", |state| {
            let mut run = state.model.simulation.retained.runs.remove(0);
            for analysis in &mut run.analyses {
                analysis.provenance = None;
            }
            state.model.simulation.retained.runs.insert(0, run);
        }),
    ];
    for (edited, edit) in edits {
        let mut app = rc_lowpass();
        app.queued_resuming_from_a_checkpoint();
        edit(&mut app.state);
        let retained = app.state.model.simulation.retained.runs.len();

        assert!(app.dispatch_frame().is_empty(), "{edited}");

        assert!(
            !app.state.model.simulation.execution.has_active_execution(),
            "{edited}"
        );
        assert_eq!(
            app.state.model.simulation.retained.runs.len(),
            retained,
            "{edited}: no run starts"
        );
        assert_eq!(
            app.state.model.simulation.execution.status, DESIGN_CHANGED,
            "{edited}"
        );
        let stated =
            format!("{DESIGN_CHANGED} · the retained runs a checkpoint resumes from changed");
        assert_eq!(
            app.state.prompt_text(),
            Some(stated.as_str()),
            "{edited}: the dispatch names what moved"
        );
    }
}

/// A run prepared in the frame is refused at its dispatch for an edit to
/// the design, the model library or the simulation setup, naming the input
/// that moved, as a checked run is.
#[test]
fn an_edit_after_the_frame_prepared_the_run_refuses_its_dispatch() {
    /// An edit, beside the input a refusal names it by.
    type Edit = (&'static str, fn(&mut AppState));
    let edits: [Edit; 3] = [
        ("the design", |state| {
            state.model.schematic.document_mut_for_test().components[0]
                .value
                .push('0');
        }),
        ("the model library", |state| {
            state
                .model
                .model_library_manager
                .add_library(rspice_model_library::ModelLibrary {
                    name: "scratch".to_owned(),
                    ..Default::default()
                });
        }),
        ("the simulation setup", |state| {
            state
                .model
                .sim_setup
                .set_reference_pvt(rspice_app_state::product::ProcessCorner::TT, 125.0)
                .expect("a physical temperature");
        }),
    ];
    for (moved, edit) in edits {
        let mut app = inverter_over_a_run_set();
        app.queued_from_this_frame();
        edit(&mut app.state);

        assert!(app.dispatch_frame().is_empty(), "{moved}");
        assert_refused_for(&app, moved);
    }
}

/// The run the worker prepares from the inputs frozen at the press is the run
/// the frame would have prepared from the live state: the same snapshot, by
/// digest, and the same deck, byte for byte. One example runs a single point;
/// the other a Run Set of nine.
#[test]
fn a_run_prepared_off_the_ui_thread_is_the_one_the_frame_would_have_prepared() {
    let mut lowpass = RSpiceApp::test_instance();
    lowpass.state.open_project_session();
    enable_only(&mut lowpass.state, &[]);
    rspice_app_state::workbench::examples::load_example(
        "RC Lowpass Filter",
        &mut lowpass.state.model.schematic,
    );
    add_operating_point(&mut lowpass.state);

    for (label, mut app, points) in [
        ("RC lowpass", lowpass, 1),
        ("inverter over a Run Set", inverter_over_a_run_set(), 9),
    ] {
        start(&mut app.state, &mut app.simulation_controller, false);
        app.settle_preflight();
        let prepared = app
            .state
            .runs
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
