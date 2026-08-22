//! The studio's return edges: run → Results, and dataset → producing plan.
//!
//! Each case asserts the whole destination rather than the fact that
//! something happened. A hop that activates Results without carrying the run,
//! or opens the Analyses page without selecting the instance that produced the
//! result, has lost the object it was taken for — which is the defect these
//! edges exist to close.

use super::*;
use crate::product::{AnalysisInstanceId, ContentDigest, SimulationPlanId};
use crate::state::{
    AnalysisResult, AnalysisResultProvenance, AnalysisResultSourceDomain, AnalysisType,
    PreparedRunReceipt, PreparedRunTaskReceipt, PreparedSourceCheckReceipt,
};
use crate::ui::widgets::NotificationAction;
use crate::workbench::commands::result_navigation;

/// The active plan's identity and its first instance, which a seeded run is
/// sealed against so the return edge has something exact to resolve.
fn plan_binding(app: &RSpiceApp) -> (SimulationPlanId, AnalysisInstanceId) {
    let plan = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("the test instance owns a stable analysis plan");
    (
        plan.id(),
        plan.instances()
            .first()
            .expect("the default plan holds one instance")
            .id(),
    )
}

/// Which plan a seeded prepared run is sealed against.
#[derive(Clone, Copy, PartialEq, Eq)]
enum RunOrigin {
    /// The app's own active plan, so the return edge resolves.
    ActivePlan,
    /// A plan that is not the active one — a retained dataset from a plan the
    /// project has since switched away from.
    ForeignPlan,
    /// No plan at all: a deck run by hand.
    ManualDeck,
}

/// A project with one retained, prepared run sealed against `origin`.
fn app_with_prepared_run(origin: RunOrigin) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    let (active_plan, analysis_id) = plan_binding(&app);
    let project_revision = app.state.workspace.project.revision();
    let (domain, plan_id, source_check) = match origin {
        RunOrigin::ActivePlan => (
            AnalysisResultSourceDomain::SimulationPlan,
            Some(active_plan),
            PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0x33; 32])),
        ),
        RunOrigin::ForeignPlan => (
            AnalysisResultSourceDomain::SimulationPlan,
            Some(SimulationPlanId::new()),
            PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0x33; 32])),
        ),
        RunOrigin::ManualDeck => (
            AnalysisResultSourceDomain::ManualDeck,
            None,
            PreparedSourceCheckReceipt::ManualSourceCheck(ContentDigest::from_bytes([0x33; 32])),
        ),
    };
    let receipt = PreparedRunReceipt::new(
        domain,
        plan_id,
        project_revision,
        ContentDigest::from_bytes([0x11; 32]),
        ContentDigest::from_bytes([0x22; 32]),
        source_check,
        vec![
            PreparedRunTaskReceipt::new(
                analysis_id,
                project_revision,
                Vec::new(),
                1,
                ContentDigest::from_bytes([0x44; 32]),
            )
            .expect("valid prepared task"),
        ],
    )
    .expect("valid prepared run receipt");
    app.state
        .simulation
        .start_prepared_run(receipt)
        .add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "retained TRAN").with_provenance(
                AnalysisResultProvenance::new_with_source_domain(
                    AnalysisResultSourceDomain::SimulationPlan,
                    analysis_id,
                    project_revision,
                    ContentDigest::from_bytes([0x44; 32]),
                    Vec::new(),
                )
                .expect("valid prepared analysis provenance"),
            ),
        );
    app.state.simulation.active_run_idx = Some(0);
    app.state.simulation.active_analysis_idx = Some(0);
    app
}

#[test]
fn opening_a_run_in_results_needs_a_dataset_and_lands_on_the_selected_one() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    assert!(
        !Command::OpenRunInResults.is_enabled(&app),
        "with no retained dataset there is nothing for the hop to open"
    );
    assert_eq!(
        Command::OpenRunInResults.availability(&app),
        CommandAvailability::Disabled(
            "the selected run has no retained dataset to open in Results"
        ),
        "and the control has to say so rather than being silently inert"
    );

    let mut app = app_with_prepared_run(RunOrigin::ManualDeck);
    app.state.workbench.activate(Workspace::Simulate);
    assert!(Command::OpenRunInResults.is_enabled(&app));
    Command::OpenRunInResults.execute(&mut app);
    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert_eq!(
        app.state.simulation.active_run().map(|run| run.id),
        Some(1),
        "the selection the affordance made is the one the command lands on"
    );
}

/// The retained notice outlives the run it points at, so taking its offer
/// re-resolves the run rather than trusting the record.
#[test]
fn a_notice_offer_selects_its_run_and_refuses_when_the_run_is_gone() {
    let mut app = app_with_prepared_run(RunOrigin::ManualDeck);
    app.state.workbench.activate(Workspace::Simulate);
    result_navigation::perform_notification_action(
        &mut app,
        NotificationAction::OpenRunInResults { run_sequence: 1 },
    );
    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert_eq!(app.state.simulation.active_run().map(|run| run.id), Some(1));

    app.state.workbench.activate(Workspace::Simulate);
    let before = app.state.log_buffer.revision();
    result_navigation::perform_notification_action(
        &mut app,
        NotificationAction::OpenRunInResults { run_sequence: 47 },
    );
    assert_eq!(
        app.state.workbench.workspace,
        Workspace::Simulate,
        "a lost run must not move the reader somewhere it cannot honour"
    );
    assert!(
        app.state.log_buffer.revision() > before,
        "and the refusal is stated rather than silently dropped"
    );
}

/// The status bar's engine chip and the split toggle both mean "the newest
/// materialized dataset". They resolve it here rather than each guessing.
#[test]
fn the_newest_retained_run_is_what_an_unnamed_results_hop_opens() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    assert!(
        !result_navigation::open_newest_retained_run(&mut app),
        "with no materialized dataset the hop reports that it cannot land"
    );
    assert_ne!(app.state.workbench.workspace, Workspace::Results);

    let mut app = app_with_prepared_run(RunOrigin::ManualDeck);
    app.state.workbench.activate(Workspace::Simulate);
    app.state.simulation.active_run_idx = None;
    assert!(result_navigation::open_newest_retained_run(&mut app));
    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert_eq!(app.state.simulation.active_run().map(|run| run.id), Some(1));
}

#[test]
fn the_producing_plan_hop_carries_the_instance_that_made_the_result() {
    let mut app = app_with_prepared_run(RunOrigin::ActivePlan);
    let (_, analysis_id) = plan_binding(&app);
    app.state.workbench.activate(Workspace::Results);
    app.state.workbench.active_analysis_instance = None;

    assert!(Command::OpenProducingPlan.is_enabled(&app));
    Command::OpenProducingPlan.execute(&mut app);
    assert_eq!(app.state.workbench.workspace, Workspace::Simulate);
    assert_eq!(
        app.state.workbench.simulation_page,
        SimulationPage::Analyses,
        "the plan opens on the page that owns its instances"
    );
    assert_eq!(
        app.state.workbench.active_analysis_instance,
        Some(analysis_id),
        "and the producing instance is selected on arrival"
    );
}

/// A hop whose producing instance is gone must not leave the previous one lit.
///
/// The instance was written only when the hop resolved one, so the
/// producer-gone branch landed on the plan with whatever had been highlighted
/// before still highlighted — while the console note beside it said nothing had
/// been selected. Removing the producer also moves the plan on from the
/// revision the run was prepared at, which the note now says as well: the plan
/// is the same object, and it is not the version that produced these numbers.
#[test]
fn a_hop_to_a_plan_that_no_longer_owns_the_producer_clears_the_highlight() {
    let mut app = app_with_prepared_run(RunOrigin::ActivePlan);
    let (_, analysis_id) = plan_binding(&app);
    app.state.workbench.active_analysis_instance = Some(analysis_id);
    app.state
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("the fixture owns a stable plan")
        .remove(analysis_id, Vec::new())
        .expect("the producing instance is removed from the plan");
    app.state.workbench.activate(Workspace::Results);
    let before = app.state.log_buffer.revision();

    Command::OpenProducingPlan.execute(&mut app);

    assert_eq!(app.state.workbench.workspace, Workspace::Simulate);
    assert!(
        app.state.workbench.active_analysis_instance.is_none(),
        "the hop selected nothing, so nothing may stay selected"
    );
    assert!(
        app.state.log_buffer.revision() > before,
        "and what the hop could not carry is stated"
    );
    assert!(
        app.state.log_buffer.entries().any(|entry| {
            entry
                .message
                .contains("has also been edited since this run")
        }),
        "a plan that has moved on since the run says so"
    );
}

/// Three different facts, three different refusals: a manual deck, a dataset
/// that predates receipts, and a plan that is no longer the active one are not
/// one "unavailable".
#[test]
fn each_reason_a_dataset_cannot_name_its_plan_is_stated_exactly() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    assert_eq!(
        Command::OpenProducingPlan.availability(&app),
        CommandAvailability::Disabled("no retained dataset is selected")
    );

    let mut legacy = RSpiceApp::test_instance();
    legacy.state.project_lifecycle.project_open = true;
    legacy
        .state
        .simulation
        .start_run()
        .add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "legacy"));
    legacy.state.simulation.active_run_idx = Some(0);
    assert_eq!(
        Command::OpenProducingPlan.availability(&legacy),
        CommandAvailability::Disabled(
            "this dataset predates prepared-run receipts, so it does not name a producing plan"
        )
    );

    let manual = app_with_prepared_run(RunOrigin::ManualDeck);
    assert_eq!(
        Command::OpenProducingPlan.availability(&manual),
        CommandAvailability::Disabled(
            "this dataset was produced by a manual deck, not a simulation plan"
        )
    );

    let foreign = app_with_prepared_run(RunOrigin::ForeignPlan);
    assert_eq!(
        Command::OpenProducingPlan.availability(&foreign),
        CommandAvailability::Disabled(
            "the plan that produced this dataset is not the active simulation plan"
        )
    );

    // A refused hop must not move anything: the console carries the reason.
    let mut refused = foreign;
    refused.state.workbench.activate(Workspace::Results);
    let before = refused.state.log_buffer.revision();
    Command::OpenProducingPlan.execute(&mut refused);
    assert_eq!(refused.state.workbench.workspace, Workspace::Results);
    assert!(refused.state.log_buffer.revision() > before);
}

/// A retained notice keeps the offer its expired toast carried, so the
/// notification center can take the reader to the same place.
#[test]
fn a_retained_notice_keeps_the_offer_its_toast_carried() {
    use crate::ui::widgets::{NotificationCategory, ToastKind, Toasts};

    let ctx = egui::Context::default();
    let mut toasts = Toasts::default();
    toasts.notify_with_action(
        &ctx,
        NotificationCategory::Job,
        ToastKind::Success,
        "Run 3 complete",
        "2 retained analyses in this immutable dataset.",
        NotificationAction::OpenRunInResults { run_sequence: 3 },
    );
    toasts.info(&ctx, "unrelated");

    let activity = toasts.activity();
    assert_eq!(
        activity[1].action(),
        Some(NotificationAction::OpenRunInResults { run_sequence: 3 }),
        "the offer is retained beside the record it announced"
    );
    assert_eq!(
        activity[0].action(),
        None,
        "and a notice with nowhere to go carries no control"
    );
}

/// The task-deck hop states which of the two absences it hit.
///
/// The route to the source a run's engine was actually handed existed only on
/// rows: the Jobs manager's history and the Netlist run strip. A reader who
/// was not already looking at one of those could not reach it. As a command it
/// has to answer the same question those rows answer before offering
/// themselves — and answer it with the reason, because "no run is selected"
/// and "that run's source was released" are different problems with different
/// remedies.
#[test]
fn the_task_deck_hop_is_refused_by_the_reason_it_would_have_failed() {
    use crate::state::{ExecutedDeck, ExecutedDeckPoint};

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    assert_eq!(
        Command::OpenTaskDeck.availability(&app),
        CommandAvailability::Disabled("no run is selected"),
    );

    let mut app = app_with_prepared_run(RunOrigin::ActivePlan);
    assert_eq!(
        Command::OpenTaskDeck.availability(&app),
        CommandAvailability::Disabled(
            "the source the selected run executed is no longer retained in this session"
        ),
        "a run whose deck was released has something to open and cannot"
    );

    // Taking it anyway says so rather than doing nothing, because the palette
    // is not modal and the archive can be evicted under it.
    let messages = app.state.log_buffer.len();
    Command::OpenTaskDeck.execute(&mut app);
    assert!(
        app.state.log_buffer.len() > messages,
        "a refused hop has to report the refusal"
    );
    assert_ne!(app.state.workbench.workspace, Workspace::Netlist);

    let sequence = app
        .state
        .simulation
        .active_run()
        .expect("the fixture selected a run")
        .id;
    app.state.simulation.executed_decks.retain(ExecutedDeck {
        run_id: sequence,
        points: vec![ExecutedDeckPoint {
            label: "nominal".to_owned(),
            deck: "task deck\nV1 1 0 1\nR1 1 0 1k\n.op\n.end\n".into(),
            model_sources: Vec::new(),
        }],
    });

    assert_eq!(
        Command::OpenTaskDeck.availability(&app),
        CommandAvailability::Available
    );
    Command::OpenTaskDeck.execute(&mut app);
    assert_eq!(app.state.workbench.workspace, Workspace::Netlist);
    assert_eq!(
        app.state
            .ui
            .netlist
            .executed_deck_view
            .as_ref()
            .map(|view| view.run_id),
        Some(sequence),
        "the hop has to land on the selected run's source, not on the working deck"
    );
}

/// The producer-log hop refuses to choose which quantity the reader meant.
///
/// The two context menus that offer this reveal act on the row they were
/// opened on. The palette has no row, so it acts on the Data Browser's
/// check-marks — and the interesting case is the plural one: revealing "the"
/// producer log of five quantities is five destinations, and silently taking
/// the first would be the studio deciding what was asked for.
#[test]
fn the_producer_log_hop_refuses_rather_than_choosing_a_quantity() {
    use crate::state::AnalysisResultPayload;
    use crate::workbench::documents::result_document::{
        AnalysisPresentationKey, ResultArtifactPresentationKey, ResultBrowserSelectionKey,
    };

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    assert_eq!(
        Command::RevealProducerLog.availability(&app),
        CommandAvailability::Disabled("no run is selected"),
    );

    let mut app = app_with_prepared_run(RunOrigin::ActivePlan);
    assert_eq!(
        Command::RevealProducerLog.availability(&app),
        CommandAvailability::Disabled(
            "check-mark the Data Browser quantity whose producer log to reveal"
        ),
    );

    // A typed artifact the retained analysis actually holds, so the resolved
    // path is the dataset's own rather than a string this test invented.
    let mut values = std::collections::BTreeMap::new();
    values.insert("settling_time".to_owned(), 1.0 / 3.0);
    app.state
        .simulation
        .active_run_mut()
        .expect("the fixture selected a run")
        .analyses[0]
        .result_payload = Some(AnalysisResultPayload::ScalarMeasurements { values });
    let run = app
        .state
        .simulation
        .active_run()
        .expect("the fixture selected a run");
    let analysis = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    let artifact = ResultBrowserSelectionKey::Artifact(ResultArtifactPresentationKey::new(
        analysis,
        "payload/scalar-measurements",
    ));
    let second = ResultBrowserSelectionKey::Artifact(ResultArtifactPresentationKey::new(
        analysis,
        "payload/waveforms",
    ));

    app.state
        .ui
        .results
        .checked_result_quantities
        .insert(artifact.clone());
    app.state
        .ui
        .results
        .checked_result_quantities
        .insert(second);
    assert_eq!(
        Command::RevealProducerLog.availability(&app),
        CommandAvailability::Disabled("check-mark exactly one Data Browser quantity"),
        "two check-marks are two destinations, and the command must not pick one"
    );

    app.state.ui.results.checked_result_quantities.clear();
    app.state
        .ui
        .results
        .checked_result_quantities
        .insert(artifact.clone());
    assert_eq!(
        Command::RevealProducerLog.availability(&app),
        CommandAvailability::Available
    );

    let expected =
        crate::workbench::documents::result_document::result_browser_selection_stable_path(
            &artifact,
            &app.state.simulation.runs,
        )
        .expect("the check-marked artifact resolves a stable path");
    Command::RevealProducerLog.execute(&mut app);
    assert_eq!(
        app.state.workbench.console_page,
        crate::workbench::state::ConsolePage::Console
    );
    let filter = app
        .state
        .workbench
        .console_producer_filter
        .as_ref()
        .expect("the hop installs a producer filter");
    assert_eq!(filter.producer, expected);
    assert_eq!(filter.label(), "payload/scalar-measurements");
}
