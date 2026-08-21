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
