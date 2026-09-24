//! Retained evidence must be rechecked without relying on a painted frame.

use std::sync::Arc;

use super::super::*;
use crate::io::project_io::ProjectSimulationResults;
use crate::state::{AnalysisType, SimulationRunLifecycle, SimulationRunProvenance, WaveformData};

fn analysis(kind: AnalysisType, id: u64, samples: usize) -> AnalysisResult {
    let (name, value) = match kind {
        AnalysisType::Noise => ("onoise", 1.0e-12),
        AnalysisType::Disto => ("HD2", 1.0),
        _ => ("V(out)", 1.0),
    };
    AnalysisResult::new(id, kind, name).with_waveforms(vec![WaveformData::new(
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
    let run = state.simulation.start_run();
    run.add_analysis(analysis(kind, 1, samples));
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.mark_running().unwrap();
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .unwrap();
    state.simulation.complete_run();
    state.simulation = ProjectSimulationResults::from_state(&state.simulation)
        .into_simulation_state()
        .unwrap();
    state.ui.results.viewer = ResultViewer::Waves;
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(
            state.simulation.runs[0].dataset_id,
        ));
    state
}

fn key(state: &AppState) -> AnalysisPresentationKey {
    let run = &state.simulation.runs[0];
    AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
}

#[test]
fn retained_memo_validity_tracks_corruption_and_repair_without_a_version_bump() {
    let mut state = retained_state(AnalysisType::Transient, 3);
    let key = key(&state);
    let version = state.simulation.data_version;
    assert!(retained_evidence_is_valid(&state, key));
    Arc::make_mut(&mut state.simulation.runs[0].analyses[0].waveforms[0].y).pop();
    assert!(
        state.simulation.runs[0].analyses[0]
            .validate_retained_evidence()
            .is_err()
    );
    assert!(
        !retained_evidence_is_valid(&state, key),
        "a stale success hid corrupt evidence"
    );
    Arc::make_mut(&mut state.simulation.runs[0].analyses[0].waveforms[0].y).push(1.0);
    assert!(
        retained_evidence_is_valid(&state, key),
        "a stale failure hid repaired evidence"
    );
    assert_eq!(state.simulation.data_version, version);
}

#[test]
fn retained_memo_absent_analysis_becomes_available_without_frame_preparation() {
    let mut state = retained_state(AnalysisType::Transient, 3);
    let next = analysis(AnalysisType::Transient, 2, 3);
    let key = AnalysisPresentationKey::new(state.simulation.runs[0].dataset_id, &next);
    assert!(!retained_evidence_is_valid(&state, key));
    let version = state.simulation.data_version;
    state.simulation.runs[0].analyses.push(next);
    assert!(
        retained_evidence_is_valid(&state, key),
        "a cached absence hid the retained analysis"
    );
    assert_eq!(state.simulation.data_version, version);
}

fn gate_tracks_coordinates(kind: AnalysisType, gate: StructuralGate) {
    let mut state = retained_state(kind, 3);
    let answer = |state: &AppState| {
        let run = &state.simulation.runs[0];
        analysis_answers_structural_gate(state, run.dataset_id, &run.analyses[0], gate)
    };
    assert!(answer(&state));
    let version = state.simulation.data_version;
    Arc::make_mut(&mut state.simulation.runs[0].analyses[0].waveforms[0].x)[0] = -1.0;
    assert!(!structural_gate_is_answered_directly(
        gate,
        &state.simulation.runs[0].analyses[0]
    ));
    assert!(
        !answer(&state),
        "the gate kept accepting an invalid frequency grid"
    );
    Arc::make_mut(&mut state.simulation.runs[0].analyses[0].waveforms[0].x)[0] = 1.0;
    assert!(
        answer(&state),
        "the gate kept rejecting the repaired frequency grid"
    );
    assert_eq!(state.simulation.data_version, version);
}

#[test]
fn retained_memo_noise_shape_tracks_the_retained_frequency_grid() {
    gate_tracks_coordinates(AnalysisType::Noise, StructuralGate::OrdinaryNoiseSpectrum);
}

#[test]
fn retained_memo_structural_gate_tracks_the_retained_frequency_grid() {
    gate_tracks_coordinates(AnalysisType::Disto, StructuralGate::BodeResponse);
}

#[test]
fn retained_memo_restored_history_invalidates_an_older_resolved_view() {
    let mut state = retained_state(AnalysisType::Transient, 3);
    let old = view_context::resolve_displayed_result_view(&state).unwrap();
    assert!(old.run(&state).is_some());
    let mut incoming = state.simulation.clone();
    Arc::make_mut(&mut incoming.runs[0].analyses[0].waveforms[0].y).fill(2.0);
    let restored = ProjectSimulationResults::from_state(&incoming)
        .into_simulation_state()
        .unwrap();
    assert_eq!(state.simulation.data_version, restored.data_version);
    assert_eq!(
        state.simulation.runs[0].dataset_id,
        restored.runs[0].dataset_id
    );
    state.simulation = restored;
    assert!(
        old.run(&state).is_none(),
        "an old resolved view accepted changed retained content"
    );
    let current = view_context::resolve_displayed_result_view(&state).unwrap();
    assert_ne!(current.dataset_digest, old.dataset_digest);
    assert!(current.run(&state).is_some());
}

#[test]
fn retained_memo_immutable_pane_binding_rejects_changed_source_content() {
    let mut app = RSpiceApp::test_instance();
    app.state = retained_state(AnalysisType::Transient, 3);
    app.state.workbench.create_result_document =
        crate::workbench::state::CreateResultDocumentDialogState {
            open: true,
            name: "Pinned evidence".to_owned(),
            name_touched: true,
            dataset_id: Some(app.state.simulation.runs[0].dataset_id),
            family_id: "waveform-worksheet".to_owned(),
            viewer_id: "viewer-waveform".to_owned(),
            layout_id: "single-pane".to_owned(),
            validation_error: None,
        };
    let document = create_document::commit(&mut app).unwrap();
    let bound = view_context::resolve_displayed_result_view(&app.state).unwrap();
    assert!(
        matches!(bound.owner, view_context::ResultViewOwner::VisualizationPane { document_id, .. } if document_id == document)
    );
    let version = app.state.simulation.data_version;
    Arc::make_mut(&mut app.state.simulation.runs[0].analyses[0].waveforms[0].y).fill(2.0);
    let error = view_context::resolve_displayed_result_view(&app.state)
        .expect_err("the immutable pane must reject changed content even before the next frame");
    assert!(error.contains("immutable binding"), "{error}");
    assert!(bound.run(&app.state).is_none());
    assert_eq!(app.state.simulation.data_version, version);
}

fn read_all(state: &AppState) {
    let run = &state.simulation.runs[0];
    assert!(retained_evidence_is_valid(state, key(state)));
    let _ = retained_dataset_digest(state, run);
    assert!(analysis_answers_structural_gate(
        state,
        run.dataset_id,
        &run.analyses[1],
        StructuralGate::OrdinaryNoiseSpectrum
    ));
    assert!(analysis_answers_structural_gate(
        state,
        run.dataset_id,
        &run.analyses[2],
        StructuralGate::BodeResponse
    ));
}

#[test]
fn retained_memo_unchanged_clones_reuse_their_evidence_without_dataset_walks() {
    for samples in [3, 100_000] {
        let mut state = retained_state(AnalysisType::Transient, samples);
        state.simulation.runs[0].analyses.extend([
            analysis(AnalysisType::Noise, 2, samples),
            analysis(AnalysisType::Disto, 3, samples),
        ]);
        read_all(&state);
        let mut clone = AppState::default();
        clone.simulation = state.simulation.clone();
        clone.ui.results = state.ui.results.clone();
        let work = frame_work::WorkCounts::reset();
        for _ in 0..12 {
            read_all(&state);
            read_all(&clone);
        }
        assert_eq!(work.since().total(), 0);
        let digest = retained_dataset_digest(&state, &state.simulation.runs[0]);
        Arc::make_mut(&mut clone.simulation.runs[0].analyses[0].waveforms[0].y).fill(2.0);
        assert_eq!(
            retained_dataset_digest(&state, &state.simulation.runs[0]),
            digest
        );
        assert_ne!(
            retained_dataset_digest(&clone, &clone.simulation.runs[0]),
            digest
        );
    }
}
