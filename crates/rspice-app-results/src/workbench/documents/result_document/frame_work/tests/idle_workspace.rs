//! The workspace around the sheet on idle frames: a Latest document deciding
//! which run it follows, and the toolbar deciding whether it is historical.
//! Neither digests the netlist or a run again until one of them changes.

use rspice_results::waveform::column_work::ColumnWork;

use super::super::super::{ResultsTestHost, create_document, persistent_document};
use super::super::{DatasetWalk, WorkCounts};
use crate::product::{AnalysisInstanceId, ObjectRevision, ResultDocumentId, SimulationPlanId};
use crate::results::visualization_document::{
    DocumentEdit, ResultDocumentTracking, ResultDocumentTrackingMode,
};
use crate::state::{AnalysisResult, AnalysisResultProvenance, AnalysisType, RetainedTrace};
use crate::workbench::chrome::toolbar::results::results_document_is_historical;
use crate::workbench::documents::result_document::test_fixtures::{completed_prepared_run, digest};

/// Make `state` a Latest document's world: a netlist generated from the
/// current design, and one completed run of it holding `analysis`. Returns the
/// document, which follows the newest run of that analysis.
fn latest_document(state: &mut ResultsTestHost, analysis: AnalysisResult) -> ResultDocumentId {
    state
        .model
        .simulation
        .source
        .netlist_content
        .set("* latest\nV1 out 0 1\n.end\n".to_owned());
    let netlist = &mut state.model.netlist_artifacts;
    netlist.generation_error = None;
    netlist.generated_input_digest = Some(digest(0x60));
    netlist.current_generation_input_digest = Some(digest(0x60));
    let source = persistent_document::current_result_source_digest(state.results_reads())
        .expect("the netlist is generated from the current design");
    let plan_id = SimulationPlanId::new();
    let analysis_id = AnalysisInstanceId::new();
    let project_revision = state.model.workspace.content.project.revision();
    let mut run = completed_prepared_run(plan_id, project_revision, source);
    run.add_analysis(
        analysis.with_provenance(
            AnalysisResultProvenance::new(
                analysis_id,
                ObjectRevision::INITIAL,
                digest(0x51),
                Vec::new(),
            )
            .expect("analysis provenance"),
        ),
    );
    state.model.simulation.retained.runs = vec![run].into();
    assert!(state.model.simulation.select_run(0));
    assert!(state.model.simulation.select_analysis(0));
    let mut parts = state.results_parts();
    create_document::open(&mut parts);
    parts.create_result_document.layout_id = "single-pane".to_owned();
    let document_id = create_document::commit(&mut parts).expect("the document is created");
    let revision = state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("the project keeps the document")
        .revision();
    state
        .model
        .workspace
        .content
        .transact_visualization_document(
            document_id,
            revision,
            vec![DocumentEdit::SetTracking(ResultDocumentTracking::for_plan(
                ResultDocumentTrackingMode::Latest,
                plan_id,
                analysis_id,
            ))],
        )
        .expect("the document follows the latest run");
    document_id
}

/// One in-memory transient signal of `points` samples.
fn transient(points: usize) -> AnalysisResult {
    let x: Vec<f64> = (0..points).map(|sample| sample as f64 * 1.0e-9).collect();
    let y: Vec<f64> = x.iter().map(|time| (time * 1.0e6).sin()).collect();
    AnalysisResult::new(1, AnalysisType::Transient, "Transient")
        .with_waveforms(vec![RetainedTrace::new("V(out)", x, y, "#fff")])
}

/// Ask what a Latest document and the toolbar ask on each frame.
fn ask(state: &mut ResultsTestHost, document_id: ResultDocumentId) {
    let binding =
        persistent_document::refresh_latest_binding(&mut state.results_parts(), document_id);
    assert!(
        !matches!(binding, persistent_document::LatestBinding::Missing(_)),
        "the document is kept"
    );
    let _ = results_document_is_historical(state.results_reads());
}

#[test]
fn latest_tracking_digests_its_netlist_and_runs_once_per_change() {
    let mut state = ResultsTestHost::default();
    let document_id = latest_document(&mut state, transient(20_000));
    assert!(matches!(
        persistent_document::refresh_latest_binding(&mut state.results_parts(), document_id),
        persistent_document::LatestBinding::Current
    ));

    let work = WorkCounts::reset();
    let columns = ColumnWork::read();
    for _ in 0..10 {
        ask(&mut state, document_id);
    }
    let (work, columns) = (work.since(), columns.since());
    assert_eq!(
        work.get(DatasetWalk::DatasetDigest),
        0,
        "{:?}",
        work.nonzero()
    );
    assert_eq!(
        work.get(DatasetWalk::NetlistDigest),
        0,
        "{:?}",
        work.nonzero()
    );
    assert_eq!(
        columns.memory_views, 0,
        "no frame reads the run's samples to decide which run is the latest"
    );

    // An edit to the netlist is digested once, however often it is asked.
    state
        .model
        .simulation
        .source
        .netlist_content
        .edit(|text| text.push_str("* edited\n"));
    let work = WorkCounts::reset();
    for _ in 0..3 {
        ask(&mut state, document_id);
    }
    assert_eq!(work.since().get(DatasetWalk::NetlistDigest), 1);
}

/// The measurement form is drawn on every frame it is open. Its preview
/// measures the expression once, and again only when the expression changes.
#[test]
fn the_measurement_form_measures_its_expression_once_per_edit() {
    use crate::workbench::documents::result_document::document_edit::measurement;
    use rspice_results_ui::document_dialogs::measurement::MeasurementDraft;

    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs = vec![{
        let mut run = crate::state::SimulationRun::new(1);
        run.add_analysis(transient(20_000));
        run
    }]
    .into();
    assert!(state.model.simulation.select_run(0));
    assert!(state.model.simulation.select_analysis(0));
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
    let mut draft = MeasurementDraft {
        expression: "max(V(out))".to_owned(),
        ..MeasurementDraft::default()
    };
    let first = measurement::preview(state.results_reads(), &draft);
    assert!(
        matches!(&first.value, Ok(Some(_))),
        "the expression measures: {:?}",
        first.value
    );

    let (work, columns) = (WorkCounts::reset(), ColumnWork::read());
    for _ in 0..10 {
        let preview = measurement::preview(state.results_reads(), &draft);
        assert_eq!(preview.value, first.value);
    }
    assert_eq!(work.since().get(DatasetWalk::MeasurementPreview), 0);
    assert_eq!(
        columns.since().memory_views,
        0,
        "an unchanged draft is not measured again"
    );

    draft.expression = "min(V(out))".to_owned();
    let work = WorkCounts::reset();
    for _ in 0..3 {
        let _ = measurement::preview(state.results_reads(), &draft);
    }
    assert_eq!(work.since().get(DatasetWalk::MeasurementPreview), 1);
}

/// Rows per signal of the large runs: four signals of 1.25 million samples
/// are five million.
const LARGE_ROWS: usize = 1_250_000;
const LARGE_SIGNALS: usize = 4;

/// The large runs' time axis and signals.
fn large_signals() -> (Vec<f64>, Vec<(String, Vec<f64>)>) {
    let x: Vec<f64> = (0..LARGE_ROWS).map(|row| row as f64 * 1.0e-9).collect();
    let signals = (0..LARGE_SIGNALS)
        .map(|signal| {
            let rate = 1.0e5 * (signal + 1) as f64;
            let y = x.iter().map(|time| (time * rate).sin()).collect();
            (format!("V(s{signal})"), y)
        })
        .collect();
    (x, signals)
}

/// The Results workspace around a Latest document over `analysis`: the
/// document active, its first trace selected for the inspector.
fn large_workspace(analysis: AnalysisResult) -> (ResultsTestHost, ResultDocumentId) {
    use crate::workbench::state::{Workspace, WorkspaceDocumentId};
    use rspice_results_ui::selection::SelectedResultTrace;

    let mut state = ResultsTestHost::default();
    let document_id = latest_document(&mut state, analysis);
    state.workbench.activate(Workspace::Results);
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::VisualizationDocument(document_id));
    let run = state
        .model
        .simulation
        .active_run()
        .expect("the run is active");
    let key = super::super::super::AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    state.results.results.session.selected_trace =
        Some(SelectedResultTrace::from_identity(key, "V(s0)"));
    (state, document_id)
}

/// One frame of the workspace: the data browser, the toolbar's Results
/// controls and what it decides about them, the Latest document, and the
/// inspector, side by side as the shell lays them out.
fn workspace_frame(
    ctx: &egui::Context,
    state: &mut ResultsTestHost,
    controller: &mut crate::simulation::SimulationController,
    document_id: ResultDocumentId,
) {
    use crate::workbench::chrome::toolbar::results::{
        offered_tool_groups, results_tracking_button,
    };
    use crate::workbench::docks::{inspector, navigator};
    use crate::workbench::documents::result_document::document_edit::compare::compare_availability;

    let mut run_details = None;
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_680.0, 1_020.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let full = ui.max_rect();
                let left = egui::Rect::from_min_max(
                    full.min,
                    egui::pos2(full.left() + 300.0, full.bottom()),
                );
                let right = egui::Rect::from_min_max(
                    egui::pos2(full.right() - 320.0, full.top()),
                    full.max,
                );
                let bar = egui::Rect::from_min_max(
                    egui::pos2(left.right(), full.top()),
                    egui::pos2(right.left(), full.top() + 40.0),
                );
                let sheet = egui::Rect::from_min_max(
                    egui::pos2(left.right(), bar.bottom()),
                    egui::pos2(right.left(), full.bottom()),
                );
                let child =
                    |ui: &mut egui::Ui, rect| ui.new_child(egui::UiBuilder::new().max_rect(rect));
                navigator::results::results(
                    &mut child(ui, left),
                    &mut state.results_parts(),
                    &mut run_details,
                );
                let layout = crate::workbench::layout::LayoutSpec::resolve(
                    1_680.0,
                    1_020.0,
                    &state.workbench,
                );
                let _ = results_tracking_button(
                    &mut child(ui, bar),
                    &mut state.results_parts(),
                    layout,
                );
                let _ = offered_tool_groups(state.results_reads()).count();
                let _ = compare_availability(state.results_reads());
                let _ = results_document_is_historical(state.results_reads());
                persistent_document::show(
                    &mut child(ui, sheet),
                    &mut state.results_parts(),
                    controller,
                    document_id,
                );
                inspector::results::results(
                    &mut child(ui, right),
                    &mut state.results_parts(),
                    &mut run_details,
                );
            });
        },
    );
}

/// The work [`super::IDLE_FRAMES`] frames of the workspace do once the first
/// has built what it needs and the selected trace's statistics are known,
/// with how far `store_reads` counted over them.
fn idle_workspace_work(
    state: &mut ResultsTestHost,
    document_id: ResultDocumentId,
    store_reads: impl Fn() -> u64,
) -> (WorkCounts, ColumnWork, u64) {
    use crate::workbench::documents::result_document::trace_statistics::trace_statistics;

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut controller = crate::simulation::SimulationController::new();
    workspace_frame(&ctx, state, &mut controller, document_id);
    let run = state
        .model
        .simulation
        .active_run()
        .expect("the run is active");
    let key = super::super::super::AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    let waveform = run.analyses[0].waveforms[0].clone();
    let known = (0..3_000).any(|_| {
        let known = trace_statistics(state.results_reads(), key, 0, &waveform, &ctx).is_some();
        if !known {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        known
    });
    assert!(known, "the selected trace's statistics are computed");
    // A plot too large to draw on a frame is drawn off it, reading its
    // curves there, and the Measurements table measures off it, reading the
    // traces it measures; that is the first frames' work, so idle frames are
    // counted once the picture has been taken and the measuring has ended.
    let settled = (0..3_000).any(|_| {
        workspace_frame(&ctx, state, &mut controller, document_id);
        let busy = rspice_ui_kit::plot::drawing_off_frame(&ctx)
            || rspice_results_ui::waves::readout::measuring_off_frame(&ctx);
        if busy {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        !busy
    });
    assert!(
        settled,
        "the plots' pictures are drawn and the measurements measured"
    );
    let (work, columns, reads) = (WorkCounts::reset(), ColumnWork::read(), store_reads());
    for _ in 0..super::IDLE_FRAMES {
        workspace_frame(&ctx, state, &mut controller, document_id);
    }
    (work.since(), columns.since(), store_reads() - reads)
}

/// Idle frames of the workspace over a run of five million samples in
/// memory walk no record and read no sample. (Their painters are still handed
/// the columns: a slice of memory is a pointer, and the caches they consult
/// with it compare its address and length.)
#[test]
fn idle_frames_of_the_workspace_over_a_large_run_in_memory_walk_no_record() {
    let (x, signals) = large_signals();
    let analysis = AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(
        signals
            .into_iter()
            .map(|(name, y)| RetainedTrace::new(name, x.clone(), y, "#fff"))
            .collect(),
    );
    let (mut state, document_id) = large_workspace(analysis);
    let (work, _, _) = idle_workspace_work(&mut state, document_id, || 0);
    assert_eq!(work.total(), 0, "{:?}", work.nonzero());
    assert_eq!(work.total_samples(), 0, "{:?}", work.nonzero_samples());
}

/// Idle frames of the workspace over a stored run of five million samples
/// read no stored column whole, counted where the store is read.
#[test]
fn idle_frames_of_the_workspace_over_a_large_stored_run_read_no_column_whole() {
    use crate::workbench::documents::result_document::test_fixtures::stored_transient;

    let (x, signals) = large_signals();
    let (analysis, columns) = stored_transient(x, signals, Some([9; 32]));
    let (mut state, document_id) = large_workspace(analysis);
    let whole_reads = || {
        columns
            .iter()
            .map(|column| column.whole_reads())
            .sum::<u64>()
    };
    let (work, read, store_reads) = idle_workspace_work(&mut state, document_id, whole_reads);
    assert_eq!(work.total(), 0, "{:?}", work.nonzero());
    assert_eq!(work.total_samples(), 0, "{:?}", work.nonzero_samples());
    assert_eq!(
        read,
        ColumnWork::default(),
        "a stored column was read whole"
    );
    assert_eq!(store_reads, 0, "the store was read whole on an idle frame");
}

/// Idle frames of the workspace over a strip of two thousand curves find
/// no pane's extremes or legend again and make none of its curves: what a
/// frame does is bounded by what it draws, not by the curves the strip holds.
#[test]
fn idle_frames_over_a_strip_of_many_curves_summarise_no_pane_again() {
    const CURVES: usize = 2_000;
    let x: Vec<f64> = (0..64).map(|row| row as f64 * 1.0e-9).collect();
    let analysis = AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(
        (0..CURVES)
            .map(|signal| {
                let rate = 1.0e7 * (signal + 1) as f64;
                let y: Vec<f64> = x.iter().map(|time| (time * rate).sin()).collect();
                RetainedTrace::new(format!("V(s{signal})"), x.clone(), y, "#fff")
            })
            .collect(),
    );
    let (mut state, document_id) = large_workspace(analysis);
    let (work, columns, _) = idle_workspace_work(&mut state, document_id, || 0);
    assert_eq!(
        work.get(DatasetWalk::WavePaneSummary),
        0,
        "an idle frame summarised a pane's curves again"
    );
    assert_eq!(work.total(), 0, "{:?}", work.nonzero());
    assert_eq!(
        columns.memory_views, 0,
        "an idle frame made the strip's curves"
    );
}
