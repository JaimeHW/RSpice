//! One selector for the analysis: the data browser picks what is shown, the
//! sheet tabs only how it is shown, and a plan's view keeps the pick for the
//! plan's next run.

use super::availability::{ac_analysis, transient_analysis};
use crate::workbench::RSpiceApp;
use rspice_app_results::workbench::docks::inspector::run_details;
use rspice_app_results::workbench::docks::navigator::results_actions::{
    select_result_analysis, select_result_dataset,
};
use rspice_app_results::workbench::documents::result_document::test_fixtures::digest;
use rspice_app_results::workbench::documents::result_document::{
    ResultViewer, prepare_viewer_state, viewer_availability, working_view,
};
use rspice_app_state::product::{AnalysisInstanceId, ObjectRevision, SimulationPlanId};
use rspice_app_state::state::{
    AnalysisResult, AnalysisResultProvenance, AnalysisResultSourceDomain, PreparedRunReceipt,
    PreparedRunReceiptInput, PreparedRunTaskReceipt, PreparedSourceCheckReceipt, SimulationRun,
    SimulationRunLifecycle,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::state::workbench_session::WorkspaceDocumentId;

/// The authored identities of a plan with a transient and an AC sweep, which
/// every run of the plan solves.
#[derive(Clone, Copy)]
struct Plan {
    id: SimulationPlanId,
    tran: AnalysisInstanceId,
    ac: AnalysisInstanceId,
}

impl Plan {
    fn new() -> Self {
        Self {
            id: SimulationPlanId::new(),
            tran: AnalysisInstanceId::new(),
            ac: AnalysisInstanceId::new(),
        }
    }

    /// A run of the plan, as sequence `sequence`, that has written the
    /// analyses `with` names, in the plan's order, and is still writing.
    fn running(self, sequence: u64, with: &[AnalysisInstanceId]) -> SimulationRun {
        let task = |authored, tag| {
            PreparedRunTaskReceipt::new(
                authored,
                ObjectRevision::INITIAL,
                Vec::new(),
                tag,
                digest(0x51),
            )
            .expect("task receipt")
        };
        let receipt = PreparedRunReceipt::new(PreparedRunReceiptInput {
            source_domain: AnalysisResultSourceDomain::SimulationPlan,
            simulation_plan_id: Some(self.id),
            project_revision: ObjectRevision::INITIAL,
            prepared_snapshot_digest: digest(0x41),
            source_content_digest: digest(0x42),
            source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest(0x43)),
            project_model_sources: Vec::new(),
            specifications: Vec::new(),
            specification_policy:
                rspice_results::specification::PreparedSpecificationPolicy::default(),
            // The transient's tag and the AC sweep's.
            tasks: vec![task(self.tran, 5), task(self.ac, 2)],
        })
        .expect("run receipt");
        let mut run = SimulationRun::new_prepared(sequence, receipt);
        run.mark_running().expect("the run starts");
        for authored in with {
            run.add_analysis(self.analysis(*authored));
        }
        run
    }

    /// A finished run of the plan that solved both analyses.
    fn run(self, sequence: u64) -> SimulationRun {
        let mut run = self.running(sequence, &[self.tran, self.ac]);
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .expect("the run finishes");
        run
    }

    fn analysis(self, authored: AnalysisInstanceId) -> AnalysisResult {
        let analysis = if authored == self.tran {
            transient_analysis()
        } else {
            ac_analysis()
        };
        analysis.with_provenance(
            AnalysisResultProvenance::new_with_source_domain(
                AnalysisResultSourceDomain::SimulationPlan,
                authored,
                ObjectRevision::INITIAL,
                // The receipt's prepared snapshot.
                digest(0x41),
                Vec::new(),
            )
            .expect("authored analysis provenance"),
        )
    }
}

/// Results open on `runs`, newest first, showing none of them yet.
fn results_app(runs: Vec<SimulationRun>) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Results);
    app.state.model.simulation.retained.runs = runs.into();
    app.state.model.simulation.view.active_run_idx = None;
    app.state.model.simulation.view.active_analysis_idx = None;
    app
}

/// What the quick view does before it draws: settle the sheet on the shown
/// analysis, and make, follow and read the run's working view.
fn frame(app: &mut RSpiceApp) {
    let mut parts = app.state.results_parts();
    prepare_viewer_state(&mut parts);
    working_view::sync(&mut parts);
    working_view::capture(&mut parts);
}

/// The sheet tabs the bar offers.
fn tabs(app: &RSpiceApp) -> Vec<ResultViewer> {
    ResultViewer::all()
        .filter(|viewer| viewer_availability(app.state.results_reads(), *viewer).available)
        .collect()
}

fn sheet(app: &RSpiceApp) -> ResultViewer {
    app.state.results.results.session.viewer
}

fn shown(app: &RSpiceApp) -> Option<usize> {
    app.state.model.simulation.view.active_analysis_idx
}

/// A run that has just started, shown as Results shows a new run: first in
/// the history, its document the one Results has open.
fn show_new_run(app: &mut RSpiceApp, run: SimulationRun) {
    let dataset = run.dataset_id;
    let simulation = &mut app.state.model.simulation;
    simulation.retained.runs.insert(0, run);
    simulation.view.active_run_idx = Some(0);
    simulation.view.active_analysis_idx = Some(0);
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(dataset));
}

/// Click an analysis in the data browser, then draw.
fn pick(app: &mut RSpiceApp, run: usize, analysis: usize) {
    assert!(select_result_analysis(
        &mut app.state.results_parts(),
        run,
        analysis
    ));
    frame(app);
}

const TRANSIENT_VIEWS: [ResultViewer; 4] = [
    ResultViewer::Waves,
    ResultViewer::Fft,
    ResultViewer::Table,
    ResultViewer::Eye,
];

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_data_browser_picks_the_analysis_and_the_tabs_offer_only_its_views() {
    let plan = Plan::new();
    let mut app = results_app(vec![plan.run(1)]);
    assert!(select_result_dataset(&mut app.state.results_parts(), 0));
    frame(&mut app);
    assert_eq!(shown(&app), Some(0), "a run opens on its curves");
    assert_eq!(sheet(&app), ResultViewer::Waves);
    assert_eq!(tabs(&app), TRANSIENT_VIEWS, "only the transient's views");

    // AC clicked while on Waves: AC on its default sheet, with only its
    // views as tabs.
    pick(&mut app, 0, 1);
    assert_eq!(shown(&app), Some(1));
    assert_eq!(sheet(&app), ResultViewer::Bode);
    let ac_tabs = tabs(&app);
    for view in [ResultViewer::Bode, ResultViewer::Smith, ResultViewer::Table] {
        assert!(ac_tabs.contains(&view), "AC offers {view:?}: {ac_tabs:?}");
    }
    for view in [ResultViewer::Waves, ResultViewer::Fft, ResultViewer::Eye] {
        assert!(!ac_tabs.contains(&view), "AC offers {view:?}: {ac_tabs:?}");
    }

    // The transient clicked: back to its default sheet.
    pick(&mut app, 0, 0);
    assert_eq!((shown(&app), sheet(&app)), (Some(0), ResultViewer::Waves));

    // Choosing a tab leaves the browser's analysis alone.
    pick(&mut app, 0, 1);
    for tab in [ResultViewer::Smith, ResultViewer::Table] {
        app.state.results.results.session.viewer = tab;
        frame(&mut app);
        assert_eq!((shown(&app), sheet(&app)), (Some(1), tab));
    }

    // Table on AC stays Table when the transient is clicked.
    pick(&mut app, 0, 0);
    assert_eq!((shown(&app), sheet(&app)), (Some(0), ResultViewer::Table));
}

/// The analysis the reader picked is the plan's view's: a re-run shows it
/// once it has written it and shows what it writes until then, and a run of
/// the plan opened later opens on it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_shown_analysis_survives_a_rerun_and_opens_a_later_run() {
    let plan = Plan::new();
    let mut app = results_app(vec![plan.run(1)]);
    assert!(select_result_dataset(&mut app.state.results_parts(), 0));
    frame(&mut app);
    pick(&mut app, 0, 1);
    assert_eq!(sheet(&app), ResultViewer::Bode);
    let dataset = app.state.model.simulation.retained.runs[0].dataset_id;
    let view = working_view::view_of_run(app.state.results_reads(), dataset)
        .expect("the plan's runs are drawn through its view");
    let kept = app
        .state
        .model
        .workspace
        .content
        .visualization_document(view)
        .and_then(|view| view.shown_pane()?.binding)
        .map(|binding| binding.analysis_id);
    assert_eq!(kept, Some(plan.ac), "the view keeps the reader's pick");

    // The plan runs again: the transient is written first.
    show_new_run(&mut app, plan.running(2, &[plan.tran]));
    frame(&mut app);
    assert_eq!(
        (shown(&app), sheet(&app)),
        (Some(0), ResultViewer::Waves),
        "the run shows what it writes until the AC sweep arrives"
    );
    app.state.model.simulation.retained.runs[0].add_analysis(plan.analysis(plan.ac));
    frame(&mut app);
    assert_eq!(
        (shown(&app), sheet(&app)),
        (Some(1), ResultViewer::Bode),
        "the AC sweep is shown once written"
    );
    app.state.results.results.session.viewer = ResultViewer::Smith;
    frame(&mut app);
    assert_eq!(
        (shown(&app), sheet(&app)),
        (Some(1), ResultViewer::Smith),
        "the settled run keeps the reader's tab"
    );

    // A run of the plan the reader opens later opens on the AC sweep too,
    // on the reader's tab while the sweep has it. A reopened project opens
    // its runs the same way: the view and its pick are saved with it.
    {
        let simulation = &mut app.state.model.simulation;
        simulation.retained.runs[0]
            .finish_lifecycle(SimulationRunLifecycle::Completed)
            .expect("the re-run finishes");
        simulation.retained.runs.insert(0, plan.run(3));
        simulation.view.active_run_idx = Some(1);
    }
    assert!(select_result_dataset(&mut app.state.results_parts(), 0));
    frame(&mut app);
    assert_eq!((shown(&app), sheet(&app)), (Some(1), ResultViewer::Smith));
    app.state.results.results.session.viewer = ResultViewer::Waves;
    assert!(select_result_dataset(&mut app.state.results_parts(), 1));
    frame(&mut app);
    assert_eq!(
        (shown(&app), sheet(&app)),
        (Some(1), ResultViewer::Bode),
        "a run opened on a tab its kept analysis lacks opens on the analysis' default sheet"
    );
}

/// The inventory the Manifest sheet held is in Run details…, and no sheet
/// is the run's inventory any more.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn run_details_lists_the_inventory_and_no_sheet_does() {
    assert!(
        ResultViewer::all().all(|viewer| viewer.label() != "Manifest"),
        "a sheet is still the run's inventory"
    );
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.add_analysis(transient_analysis());
    run.add_analysis(ac_analysis());
    let mut app = results_app(vec![run]);
    let run_id = app.state.model.simulation.retained.runs[0].id;
    run_details::open(
        &mut app.state.results.run_details,
        &app.state.model.simulation,
        run_id,
    );
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut nodes = Vec::new();
    for _ in 0..2 {
        let output = ctx.run_ui(egui::RawInput::default(), |ui| {
            run_details::show(
                ui.ctx(),
                &mut app.state.results.run_details,
                &app.state.model,
            );
        });
        nodes = output
            .platform_output
            .accesskit_update
            .map(|update| update.nodes)
            .unwrap_or_default();
    }
    let rows = nodes
        .iter()
        .filter_map(|(_, node)| Some((node.label()?.to_owned(), node.value()?.to_owned())))
        .collect::<Vec<_>>();
    let row = |label: &str| {
        rows.iter()
            .find(|(row, _)| row == label)
            .map(|(_, value)| value.as_str())
    };
    assert_eq!(
        row("State"),
        Some("Completed"),
        "the summary says how the run ended: {rows:?}"
    );
    // The inventory speaks each row as its cells, in words that need no
    // heads: what each analysis kept, and how it ended.
    let spoken = nodes
        .iter()
        .filter_map(|(_, node)| node.label())
        .collect::<Vec<_>>();
    for analysis in ["Transient", "AC Analysis"] {
        assert!(
            spoken
                .iter()
                .any(|row| row.starts_with(&format!("{analysis}, "))
                    && row.contains("signal")
                    && row.ends_with(", Complete")),
            "the inventory lists {analysis}: {spoken:?}"
        );
    }
}
