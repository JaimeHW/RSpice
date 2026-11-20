//! Retained snapshots share complete records and validation, but never edits.

use super::*;

fn validation_passes() -> usize {
    rspice_formats::project_results::RESULT_VALIDATIONS.with(std::cell::Cell::get)
}

#[test]
fn result_snapshot_clones_share_complete_history_records() {
    let state = super::waveform_storage::complex_history();
    let snapshot = crate::io::capture_simulation_results(&state);
    let candidate = snapshot.clone();
    assert_eq!(
        snapshot.runs.as_ptr(),
        candidate.runs.as_ptr(),
        "cloning a snapshot copied retained run records"
    );
    assert_eq!(
        snapshot.runs[0].analyses.as_ptr(),
        candidate.runs[0].analyses.as_ptr()
    );
    let mut edited = candidate;
    edited.runs[0].label.push_str(" changed");
    assert_ne!(snapshot.runs.as_ptr(), edited.runs.as_ptr());
    assert_ne!(snapshot.runs[0].label, edited.runs[0].label);
}

#[test]
fn immutable_result_validation_is_shared_and_mutation_invalidates_both_verdicts() {
    for samples in [2, 100_000] {
        let mut state = super::waveform_storage::complex_history();
        state.retained.runs[0].analyses[0].waveforms = vec![RetainedTrace::new(
            "V(out)",
            (1..=samples).map(|n| n as f64).collect::<Vec<_>>(),
            vec![1.0; samples],
            "#ffffff",
        )];
        let accepted = super::waveform_storage::records_held_in_memory(&state);
        let mut candidate = accepted.clone();
        let frozen = serde_json::to_vec(&accepted).unwrap();
        let before = validation_passes();
        for _ in 0..6 {
            accepted.validate().unwrap();
            candidate.validate().unwrap();
        }
        assert_eq!(
            validation_passes() - before,
            1,
            "{samples} samples: unchanged clones repeated numerical validation"
        );
        assert_eq!(serde_json::to_vec(&accepted).unwrap(), frozen);

        // Valid metadata changes must detach even when no sample changes.
        candidate.retained_dataset_limit = Some(1);
        let before = validation_passes();
        candidate.validate().unwrap();
        candidate.validate().unwrap();
        accepted.validate().unwrap();
        assert_eq!(validation_passes() - before, 1);
        assert_eq!(accepted.retained_dataset_limit, None);

        // A nested edit cannot inherit an earlier success. Repeating the
        // same invalid query is cached too, until the data is repaired.
        candidate.runs[0].analyses[0].waveforms[0].y.make_mut()[1] = 9.0;
        let before = validation_passes();
        let failure = candidate.validate().unwrap_err();
        assert_eq!(candidate.validate().unwrap_err(), failure);
        accepted.validate().unwrap();
        assert_eq!(validation_passes() - before, 1);
        assert_eq!(serde_json::to_vec(&accepted).unwrap(), frozen);

        candidate.runs[0].analyses[0].waveforms[0].y.make_mut()[1] = 1.0;
        let before = validation_passes();
        candidate.validate().unwrap();
        candidate.validate().unwrap();
        assert_eq!(validation_passes() - before, 1);

        let reopened: ProjectSimulationResults = serde_json::from_slice(&frozen).unwrap();
        let before = validation_passes();
        reopened.validate().unwrap();
        reopened.validate().unwrap();
        assert_eq!(
            validation_passes() - before,
            1,
            "serialized content cannot import cached validation authority"
        );
        assert_eq!(serde_json::to_vec(&reopened).unwrap(), frozen);
    }
}

/// A run of one AC analysis of `signals` traces on `points` frequencies, kept as the controller
/// keeps a finished analysis.
fn large_ac_run(id: u64, points: usize, signals: usize) -> SimulationRun {
    let frequencies: Vec<f64> = (1..=points).map(|n| n as f64 * 10.0).collect();
    let waveforms = (0..signals)
        .map(|k| {
            let y: Vec<f64> = frequencies
                .iter()
                .map(|f| (f * (k + 1) as f64 * 1.0e-4).sin())
                .collect();
            RetainedTrace::new(format!("V(n{k})"), frequencies.clone(), y, "#ffffff")
        })
        .collect();
    let mut run = SimulationRun::new(id);
    run.keep_analysis(AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(waveforms));
    super::seal_legacy_unattributed(&mut run);
    run.mark_running().unwrap();
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .unwrap();
    run
}

#[test]
fn a_project_of_twenty_large_runs_saves_under_a_mebibyte_and_reopens_every_run() {
    const POINTS: usize = 10_000;
    let mut libraries = crate::state::LibraryManager::with_primitives();
    let workspace = crate::state::ProjectWorkspace::new_bootstrapped(&mut libraries);
    let mut simulation = SimulationState::default();
    simulation.retained.runs = (1..=20)
        .map(|id| large_ac_run(id, POINTS, 4))
        .collect::<Vec<_>>()
        .into();
    simulation.retained.next_run_id = 20;
    let results = crate::io::capture_simulation_results(&simulation);
    let root = std::env::temp_dir().join(format!("rspice-twenty-runs-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let file = root.join("twenty.rspiceproj");
    crate::io::result_segments::keep_segments_now(results.stored_segments(), &file)
        .expect("every run's segment is kept beside the project");
    let project = ProjectSnapshot::new_with_simulation_results(workspace, libraries, results);
    let json = serialize_project_file(&project).expect("the project serializes");
    assert!(
        json.len() < 1 << 20,
        "twenty runs of {} samples each wrote a {} byte project file",
        POINTS * 4,
        json.len()
    );

    rspice_results::store_reader::keep_segments_in(
        &rspice_formats::project_results::segment_folder(&file),
    );
    let loaded = load_project_text(&json, Some(&file)).expect("the project reopens");
    let restored = crate::io::simulation_state_from_results(loaded.file.simulation_results)
        .expect("its runs restore");
    assert_eq!(restored.retained.runs.len(), 20);
    for (before, after) in simulation
        .retained
        .runs
        .iter()
        .zip(restored.retained.runs.iter())
    {
        let (before, after) = (&before.analyses[0], &after.analyses[0]);
        for (left, right) in before.waveforms.iter().zip(&after.waveforms) {
            assert!(right.y.is_stored());
            assert_eq!(right.y.len(), POINTS);
            assert_eq!(right.y.last(), left.y.last(), "{}", right.name);
        }
    }
    drop((simulation, restored));
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_saved_file_refuses_samples_held_in_memory_and_other_writers_keep_them() {
    let state = super::waveform_storage::complex_history();
    let record =
        rspice_formats::project_results::ProjectSimulationRun::from(&state.retained.runs[0]);
    let refused = rspice_formats::project_results::saving_file(|| serde_json::to_string(&record))
        .unwrap_err();
    assert!(
        refused
            .to_string()
            .contains(rspice_formats::project_results::SAMPLES_IN_MEMORY),
        "{refused}"
    );
    // A hardcopy worker's snapshot is not a saved file: it carries the samples themselves.
    assert!(
        serde_json::to_string(&record)
            .unwrap()
            .contains("[1.0,2.0]")
    );
    // The capture a save writes keeps the run in the store first, and names it.
    let captured = crate::io::capture_simulation_results(&state);
    let saved = rspice_formats::project_results::saving_file(|| serde_json::to_string(&captured))
        .expect("a captured run is written by reference");
    assert!(!saved.contains("[1.0,2.0]"), "{saved}");
    assert_eq!(captured.stored_segments().len(), 1);
}
