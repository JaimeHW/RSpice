//! Snapshot storage must share exact samples without sharing mutation authority.

use super::*;

pub(super) fn complex_history() -> SimulationState {
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![
            RetainedTrace::new("V(out)", vec![1.0, 2.0], vec![3.0, 4.0], "#ffffff")
                .with_complex_components("V(out)", vec![5.0, 6.0], vec![7.0, 8.0]),
        ]),
    );
    seal_legacy_unattributed(&mut run);
    run.mark_running().unwrap();
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .unwrap();
    let mut state = SimulationState::default();
    state.retained.runs.push(run);
    state
}

/// The records of `state`'s runs with their samples as they are, in memory: the form a hardcopy
/// worker's snapshot takes, which a saved file refuses.
pub(super) fn records_held_in_memory(state: &SimulationState) -> ProjectSimulationResults {
    rspice_formats::project_results::ProjectSimulationResultsData {
        runs: state
            .retained
            .runs
            .iter()
            .map(rspice_formats::project_results::ProjectSimulationRun::from)
            .collect(),
        next_run_id: state.retained.next_run_id,
        ..Default::default()
    }
    .into()
}

#[test]
fn waveform_storage_is_shared_by_capture_clone_and_restore() {
    use rspice_results::waveform::WaveformValues;
    let mut state = complex_history();
    state
        .store_retained_samples()
        .expect("the run is kept in the results store");
    let same = |left: &WaveformValues, right: &WaveformValues, what: &str| {
        assert!(
            WaveformValues::ptr_eq(left, right),
            "{what} copied its samples"
        );
    };
    let source = &state.retained.runs[0].analyses[0].waveforms[0];
    let source_complex = source.complex.as_ref().unwrap();
    let snapshot = crate::io::capture_simulation_results(&state);
    let candidate = snapshot.clone();
    let restored = crate::io::simulation_state_from_results(snapshot.clone()).unwrap();
    let restored = &restored.retained.runs[0].analyses[0].waveforms[0];
    let restored_complex = restored.complex.as_ref().unwrap();
    let record = |waveform: &rspice_formats::project_results::ProjectWaveformData| {
        let complex = waveform.complex.as_ref().unwrap();
        [&waveform.x, &waveform.y, &complex.real, &complex.imag].map(Clone::clone)
    };
    let records = [
        (
            "capture",
            record(&snapshot.runs[0].analyses[0].waveforms[0]),
        ),
        (
            "a snapshot clone",
            record(&candidate.runs[0].analyses[0].waveforms[0]),
        ),
        (
            "restore",
            [
                &restored.x,
                &restored.y,
                &restored_complex.real,
                &restored_complex.imag,
            ]
            .map(Clone::clone),
        ),
    ];
    for (what, [x, y, real, imag]) in records {
        same(&source.x, &x, what);
        same(&source.y, &y, what);
        same(&source_complex.real, &real, what);
        same(&source_complex.imag, &imag, what);
    }
}

#[test]
fn waveform_storage_detaches_edits_without_changing_accepted_samples_or_wire_format() {
    let mut state = complex_history();
    let accepted = records_held_in_memory(&state);
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
    let live = &mut state.retained.runs[0].analyses[0].waveforms[0];
    live.x.make_mut()[1] = 2.5;
    live.y.make_mut()[1] = 4.5;
    let complex = live.complex.as_mut().unwrap();
    complex.real.make_mut()[1] = 6.5;
    complex.imag.make_mut()[1] = 8.5;
    assert_eq!(serde_json::to_vec(&accepted).unwrap(), frozen);
    assert_eq!(serde_json::to_vec(&candidate).unwrap(), frozen);

    let draft = &mut candidate.runs[0].analyses[0].waveforms[0];
    draft.x.make_mut()[1] = 3.0;
    draft.y.make_mut()[1] = 5.0;
    let complex = draft.complex.as_mut().unwrap();
    complex.real.make_mut()[1] = 7.0;
    complex.imag.make_mut()[1] = 9.0;
    assert_eq!(serde_json::to_vec(&accepted).unwrap(), frozen);
    assert_eq!(live.x.read_whole().unwrap()[1], 2.5);
    assert_eq!(live.y.read_whole().unwrap()[1], 4.5);
    assert_eq!(
        live.complex.as_ref().unwrap().real.read_whole().unwrap()[1],
        6.5
    );
    assert_eq!(
        live.complex.as_ref().unwrap().imag.read_whole().unwrap()[1],
        8.5
    );
    assert!(
        candidate.validate().is_err(),
        "editing samples must invalidate their sealed digest"
    );
    let current = records_held_in_memory(&state);
    current.validate().unwrap();
    assert_ne!(serde_json::to_vec(&current).unwrap(), frozen);
    accepted.validate().unwrap();
}
