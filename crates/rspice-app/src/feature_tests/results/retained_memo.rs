//! Retained evidence rechecked through a result document a whole application holds.

use std::sync::Arc;

use crate::state::{
    AnalysisResult, AnalysisType, RetainedTrace, SimulationRunLifecycle, SimulationRunProvenance,
};
use crate::workbench::documents::result_document::{create_document, view_context};
use crate::workbench::state::WorkspaceDocumentId;
use crate::workbench::{AppState, RSpiceApp, ResultViewer};

fn analysis(kind: AnalysisType, id: u64, samples: usize) -> AnalysisResult {
    let (name, value) = match kind {
        AnalysisType::Noise => ("onoise", 1.0e-12),
        AnalysisType::Disto => ("HD2", 1.0),
        _ => ("V(out)", 1.0),
    };
    AnalysisResult::new(id, kind, name).with_waveforms(vec![RetainedTrace::new(
        name,
        (1..=samples)
            .map(|sample| sample as f64)
            .collect::<Vec<_>>(),
        vec![value; samples],
        "#55aaff",
    )])
}

fn retained_state(kind: AnalysisType, samples: usize) -> AppState {
    let mut state = AppState::default();
    let run = state.model.simulation.start_run();
    run.add_analysis(analysis(kind, 1, samples));
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.mark_running().unwrap();
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .unwrap();
    state.model.simulation.complete_run();
    state.model.simulation = crate::io::simulation_state_from_results(
        crate::io::capture_simulation_results(&state.model.simulation),
    )
    .unwrap();
    state.results.results.session.viewer = ResultViewer::Waves;
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(
            state.model.simulation.retained.runs[0].dataset_id,
        ));
    state
}

#[test]
fn retained_memo_immutable_pane_binding_rejects_changed_source_content() {
    let mut app = RSpiceApp::test_instance();
    app.state = retained_state(AnalysisType::Transient, 3);
    app.state.results.create_result_document =
        rspice_results_ui::create_document::CreateResultDocumentDialogState {
            open: true,
            name: "Pinned evidence".to_owned(),
            name_touched: true,
            dataset_id: Some(app.state.model.simulation.retained.runs[0].dataset_id),
            family_id: "waveform-worksheet".to_owned(),
            viewer_id: "viewer-waveform".to_owned(),
            layout_id: "single-pane".to_owned(),
            validation_error: None,
        };
    let document = create_document::commit(&mut app.state.results_parts()).unwrap();
    let bound = view_context::resolve_displayed_result_view(app.state.results_reads()).unwrap();
    assert!(
        matches!(bound.owner, view_context::ResultViewOwner::VisualizationPane { document_id, .. } if document_id == document)
    );
    let version = app.state.model.simulation.view.data_version;
    Arc::make_mut(&mut app.state.model.simulation.retained.runs[0].analyses[0].waveforms[0].y)
        .fill(2.0);
    let error = view_context::resolve_displayed_result_view(app.state.results_reads())
        .expect_err("the immutable pane must reject changed content even before the next frame");
    assert!(
        error.contains("no longer match the ones this pane was bound to"),
        "{error}"
    );
    assert!(bound.run(app.state.results_reads()).is_none());
    assert_eq!(app.state.model.simulation.view.data_version, version);
}
