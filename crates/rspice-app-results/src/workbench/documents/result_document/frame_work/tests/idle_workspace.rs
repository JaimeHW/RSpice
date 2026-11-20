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
    state.model.simulation.source.netlist_content = "* latest\nV1 out 0 1\n.end\n".to_owned();
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
        .push_str("* edited\n");
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
