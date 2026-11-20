//! What the reader placed on a plot outlives what does not move it.

use crate::state::{
    AnalysisResult, AnalysisType, RetainedTrace, SimulationRun, SimulationRunLifecycle,
};
use crate::workbench::documents::result_document::prepare_viewer_state;
use crate::workbench::documents::result_document::test_host::ResultsTestHost;
use rspice_results::result_presentation::{AnalysisPresentationKey, SourceWaveformPresentationKey};

fn analysis(id: u64, analysis_type: AnalysisType, samples: usize) -> AnalysisResult {
    let x = (0..samples)
        .map(|index| index as f64 * 1e-6)
        .collect::<Vec<_>>();
    let y: Vec<f64> = x.iter().map(|x| (x * 1e5).sin()).collect();
    AnalysisResult::new(id, analysis_type, "analysis")
        .with_waveforms(vec![RetainedTrace::new("V(out)", x, y, "#55aaff")])
}

fn run(id: u64, analyses: Vec<AnalysisResult>) -> SimulationRun {
    let mut run = SimulationRun::new(id);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses = analyses;
    run
}

fn prepare(state: &mut ResultsTestHost) {
    prepare_viewer_state(&mut state.results_parts());
}

/// An overlay, another transient, a hidden trace and a hundred live updates
/// each moved the data version, and every move cleared the A/B cursors, so
/// no reading held while a run streamed. Only an analysis that reads X in
/// another quantity clears them now.
#[test]
fn cursors_survive_an_overlay_an_analysis_change_a_hide_and_a_hundred_live_updates() {
    let mut state = ResultsTestHost::default();
    let earlier = run(1, vec![analysis(1, AnalysisType::Transient, 8)]);
    let overlay = earlier.dataset_id;
    let live = run(
        2,
        vec![
            analysis(1, AnalysisType::Transient, 8),
            analysis(2, AnalysisType::Transient, 8),
            analysis(3, AnalysisType::Ac, 8),
        ],
    );
    state.model.simulation.retained.runs = vec![live, earlier].into();
    assert!(state.model.simulation.select_run(0));
    prepare(&mut state);
    state.results.results.session.cursors.a = Some(1e-6);
    state.results.results.session.cursors.b = Some(5e-6);
    let placed = state.results.results.session.cursors;
    let cursors = |state: &ResultsTestHost| state.results.results.session.cursors;

    assert!(state.model.simulation.toggle_dataset_overlay(overlay));
    prepare(&mut state);
    assert_eq!(cursors(&state), placed, "an overlay keeps the cursors");

    assert!(state.model.simulation.select_analysis(1));
    prepare(&mut state);
    assert_eq!(
        cursors(&state),
        placed,
        "another transient keeps the cursors"
    );

    let shown = state.model.simulation.active_run().unwrap();
    let key = SourceWaveformPresentationKey::new(
        AnalysisPresentationKey::new(shown.dataset_id, &shown.analyses[1]),
        "V(out)",
    );
    assert!(!state.results.results.toggle_waveform_visibility(key, true));
    prepare(&mut state);
    assert_eq!(cursors(&state), placed, "a hidden trace keeps the cursors");

    for update in 0..100 {
        let run = state
            .model
            .simulation
            .retained
            .run_by_sequence_mut(2)
            .unwrap();
        run.analyses[1] = analysis(2, AnalysisType::Transient, 9 + update);
        assert!(state.model.simulation.refresh_run_samples(2));
        prepare(&mut state);
    }
    assert_eq!(cursors(&state), placed, "live updates keep the cursors");
    assert_eq!(
        state.model.simulation.view.active_analysis_idx,
        Some(1),
        "no update chose another analysis"
    );

    assert!(state.model.simulation.select_analysis(2));
    prepare(&mut state);
    assert!(
        !cursors(&state).any(),
        "an AC analysis reads X in hertz, so seconds-placed cursors go"
    );
}
