//! Application-level hardcopy source resolution, in the cases that read
//! which result document the Results workspace has open.

use super::*;

#[test]
fn a_results_document_that_is_no_longer_retained_is_not_printed() {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses.push(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            RetainedTrace::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#00ffff"),
        ]),
    );
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    state.workbench.activate(Workspace::Results);
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(DatasetId::new()));
    let active_run_key = format!(
        "project:{}:result-dataset:{}",
        state.model.workspace.content.project.id().as_uuid(),
        state.model.simulation.active_run().unwrap().dataset_id
    );

    assert!(!active_app_hardcopy_source_available(
        &state.hardcopy.report_authoring,
        &state.model,
        &state.workbench,
        &*state.results,
    ));
    let error = resolve_retained_hardcopy_source(
        &state,
        &active_run_key,
        HardcopyScope::ActivePlotDocument,
    )
    .unwrap_err();
    assert!(matches!(error, HardcopySourceError::SourceNotRetained(_)));
}

#[test]
fn print_does_not_guess_a_background_design_document() {
    let mut state = AppState::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(1, Point::new(0, 0), Point::new(10, 0)));
    assert!(
        state
            .workbench
            .documents
            .active(Workspace::Design)
            .is_none()
    );
    assert!(!active_app_hardcopy_source_available(
        &state.hardcopy.report_authoring,
        &state.model,
        &state.workbench,
        &*state.results,
    ));
}
