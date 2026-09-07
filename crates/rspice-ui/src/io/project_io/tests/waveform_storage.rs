//! Snapshot storage must share exact samples without sharing mutation authority.

use super::*;

pub(super) fn complex_history() -> SimulationState {
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![
            WaveformData::new("V(out)", vec![1.0, 2.0], vec![3.0, 4.0], "#ffffff")
                .with_complex_components("V(out)", vec![5.0, 6.0], vec![7.0, 8.0]),
        ]),
    );
    seal_legacy_unattributed(&mut run);
    run.mark_running().unwrap();
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .unwrap();
    let mut state = SimulationState::default();
    state.runs.push(run);
    state
}

#[test]
fn waveform_storage_is_shared_by_capture_clone_and_restore() {
    let state = complex_history();
    let source = &state.runs[0].analyses[0].waveforms[0];
    let snapshot = ProjectSimulationResults::from_state(&state);
    let captured = &snapshot.runs[0].analyses[0].waveforms[0];
    assert_eq!(
        source.x.as_ptr(),
        captured.x.as_ptr(),
        "capture copied X samples"
    );
    assert_eq!(
        source.y.as_ptr(),
        captured.y.as_ptr(),
        "capture copied Y samples"
    );
    let source_complex = source.complex.as_ref().unwrap();
    let captured_complex = captured.complex.as_ref().unwrap();
    assert_eq!(source_complex.real.as_ptr(), captured_complex.real.as_ptr());
    assert_eq!(source_complex.imag.as_ptr(), captured_complex.imag.as_ptr());
    let candidate = snapshot.clone();
    let copied = &candidate.runs[0].analyses[0].waveforms[0];
    assert_eq!(copied.x.as_ptr(), captured.x.as_ptr());
    assert_eq!(copied.y.as_ptr(), captured.y.as_ptr());
    assert_eq!(
        copied.complex.as_ref().unwrap().real.as_ptr(),
        captured_complex.real.as_ptr()
    );
    assert_eq!(
        copied.complex.as_ref().unwrap().imag.as_ptr(),
        captured_complex.imag.as_ptr()
    );
    let restored = candidate.into_simulation_state().unwrap();
    let restored = &restored.runs[0].analyses[0].waveforms[0];
    assert_eq!(restored.x.as_ptr(), source.x.as_ptr());
    assert_eq!(restored.y.as_ptr(), source.y.as_ptr());
    assert_eq!(
        restored.complex.as_ref().unwrap().real.as_ptr(),
        source_complex.real.as_ptr()
    );
    assert_eq!(
        restored.complex.as_ref().unwrap().imag.as_ptr(),
        source_complex.imag.as_ptr()
    );
}

#[test]
fn waveform_storage_detaches_edits_without_changing_accepted_samples_or_wire_format() {
    let mut state = complex_history();
    let accepted = ProjectSimulationResults::from_state(&state);
    let frozen = serde_json::to_vec(&accepted).unwrap();
    let mut candidate = accepted.clone();
    assert_eq!(
        serde_json::to_value(&accepted.runs[0].analyses[0].waveforms[0]).unwrap(),
        serde_json::json!({
            "name": "V(out)", "x": [1.0, 2.0], "y": [3.0, 4.0],
            "color": "#ffffff", "visible": true,
            "complex": {"source_name": "V(out)", "real": [5.0, 6.0], "imag": [7.0, 8.0]}
        })
    );
    let live = &mut state.runs[0].analyses[0].waveforms[0];
    std::sync::Arc::make_mut(&mut live.x)[1] = 2.5;
    std::sync::Arc::make_mut(&mut live.y)[1] = 4.5;
    let complex = live.complex.as_mut().unwrap();
    std::sync::Arc::make_mut(&mut complex.real)[1] = 6.5;
    std::sync::Arc::make_mut(&mut complex.imag)[1] = 8.5;
    assert_eq!(serde_json::to_vec(&accepted).unwrap(), frozen);
    assert_eq!(serde_json::to_vec(&candidate).unwrap(), frozen);

    let draft = &mut candidate.runs[0].analyses[0].waveforms[0];
    std::sync::Arc::make_mut(&mut draft.x)[1] = 3.0;
    std::sync::Arc::make_mut(&mut draft.y)[1] = 5.0;
    let complex = draft.complex.as_mut().unwrap();
    std::sync::Arc::make_mut(&mut complex.real)[1] = 7.0;
    std::sync::Arc::make_mut(&mut complex.imag)[1] = 9.0;
    assert_eq!(serde_json::to_vec(&accepted).unwrap(), frozen);
    assert_eq!(live.x[1], 2.5);
    assert_eq!(live.y[1], 4.5);
    assert_eq!(live.complex.as_ref().unwrap().real[1], 6.5);
    assert_eq!(live.complex.as_ref().unwrap().imag[1], 8.5);
    assert!(
        candidate.validate().is_err(),
        "editing samples must invalidate their sealed digest"
    );
    let current = ProjectSimulationResults::from_state(&state);
    current.validate().unwrap();
    assert_ne!(serde_json::to_vec(&current).unwrap(), frozen);
    accepted.validate().unwrap();
}
