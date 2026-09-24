//! Retained snapshots share complete records and validation, but never edits.

use super::*;

fn validation_passes() -> usize {
    super::super::results::RESULT_VALIDATIONS.with(std::cell::Cell::get)
}

#[test]
fn result_snapshot_clones_share_complete_history_records() {
    let state = super::waveform_storage::complex_history();
    let snapshot = ProjectSimulationResults::from_state(&state);
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
        state.runs[0].analyses[0].waveforms = vec![WaveformData::new(
            "V(out)",
            (1..=samples).map(|n| n as f64).collect::<Vec<_>>(),
            vec![1.0; samples],
            "#ffffff",
        )];
        let accepted = ProjectSimulationResults::from_state(&state);
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
        std::sync::Arc::make_mut(&mut candidate.runs[0].analyses[0].waveforms[0].y)[1] = 9.0;
        let before = validation_passes();
        let failure = candidate.validate().unwrap_err();
        assert_eq!(candidate.validate().unwrap_err(), failure);
        accepted.validate().unwrap();
        assert_eq!(validation_passes() - before, 1);
        assert_eq!(serde_json::to_vec(&accepted).unwrap(), frozen);

        std::sync::Arc::make_mut(&mut candidate.runs[0].analyses[0].waveforms[0].y)[1] = 1.0;
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
