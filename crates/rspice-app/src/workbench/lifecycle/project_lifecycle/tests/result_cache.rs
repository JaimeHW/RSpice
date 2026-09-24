//! Result-cache correctness through the real project snapshot and dirty paths.

use super::*;
use crate::state::{ExecutedDeck, ExecutedDeckArchive, ExecutedDeckPoint, SimulationState};
use crate::workbench::documents::result_document::frame_work::{DatasetWalk, WorkCounts};

fn deck(text: &str) -> ExecutedDeck {
    ExecutedDeck {
        run_id: 1,
        points: vec![ExecutedDeckPoint {
            label: "Tran".to_owned(),
            deck: text.into(),
            model_sources: Vec::new(),
        }],
    }
}

#[test]
fn unchanged_dirty_queries_do_not_rebuild_validate_or_hash_retained_results() {
    for samples in [2, 100_000] {
        let (mut state, _) = super::durable_content::retained_results();
        state.simulation.runs[0].analyses[0].waveforms = vec![crate::state::WaveformData::new(
            "V(out)",
            (0..samples).map(|n| n as f64).collect::<Vec<_>>(),
            vec![1.0; samples],
            "#ffffff",
        )];
        let baseline = snapshot(&state).unwrap();
        state.project_lifecycle.accepted = Some(AcceptedProject::new(baseline, None));
        assert!(!has_unsaved_changes(&state));
        let work = WorkCounts::reset();
        registry::RESULT_FINGERPRINT_PASSES.with(|passes| passes.set(0));
        for _ in 0..3 {
            assert!(!has_unsaved_changes(&state));
            assert!(!active_document_is_dirty(&state));
            assert_eq!(dirty_document_count(&state), 0);
            refresh_registry(&mut state).unwrap();
        }
        assert_eq!(
            work.since().get(DatasetWalk::DatasetDigest),
            0,
            "{samples} samples: unchanged queries rebuilt or revalidated a dataset"
        );
        assert_eq!(work.since().get(DatasetWalk::EvidenceValidation), 0);
        assert_eq!(
            registry::RESULT_FINGERPRINT_PASSES.with(std::cell::Cell::get),
            0,
            "{samples} samples: unchanged queries serialized and hashed retained results"
        );
        let mut oracle = snapshot(&state).unwrap();
        oracle.simulation_results = ProjectSimulationResults::from_state(&state.simulation);
        assert_eq!(
            working_fingerprints(&state).unwrap().content_digest(),
            registry::content_digest(&oracle).unwrap()
        );
    }
}

#[test]
fn every_persisted_result_input_invalidates_the_snapshot_and_matches_fresh_capture() {
    let (mut base, _) = super::durable_content::retained_results();
    let mut second_analysis = base.simulation.runs[0].analyses[0].clone();
    second_analysis.id = 2;
    second_analysis.label = "Second".to_owned();
    base.simulation.runs[0].analyses.push(second_analysis);
    let (other, _) = super::durable_content::retained_results();
    let mut second_run = other.simulation.runs[0].clone();
    second_run.id = 2;
    base.simulation.runs.push(second_run);
    base.simulation.next_run_id = 2;
    base.simulation.executed_decks.retain(deck("original deck"));
    let edits: [fn(&mut SimulationState); 11] = [
        |state| state.next_run_id = 7,
        |state| state.retained_dataset_limit = Some(3),
        |state| state.active_run_idx = Some(1),
        |state| state.active_analysis_idx = Some(1),
        |state| state.overlay_dataset_ids.push(state.runs[1].dataset_id),
        |state| state.runs[0].analyses[0].label.push_str(" changed"),
        |state| std::sync::Arc::make_mut(&mut state.runs[0].analyses[0].waveforms[0].y)[1] = 9.0,
        |state| state.executed_decks.retain(deck("changed deck")),
        |state| state.executed_decks.retain_runs(|_| false),
        |state| {
            state.executed_decks =
                ExecutedDeckArchive::restore(vec![deck("restored deck")]).unwrap()
        },
        |state| state.runs = super::durable_content::retained_results().0.simulation.runs,
    ];
    for (index, edit) in edits.into_iter().enumerate() {
        let mut state = base.clone();
        let before = state
            .project_lifecycle
            .result_cache
            .capture(&state.simulation);
        let frozen = serde_json::to_vec(&before).unwrap();
        edit(&mut state.simulation);
        let cached = state
            .project_lifecycle
            .result_cache
            .capture(&state.simulation);
        let fresh = ProjectSimulationResults::from_state(&state.simulation);
        assert!(
            !cached.shares_content_with(&before),
            "edit {index} reused an old snapshot"
        );
        assert_eq!(
            serde_json::to_vec(&cached).unwrap(),
            serde_json::to_vec(&fresh).unwrap(),
            "edit {index}"
        );
        cached.validate().unwrap();
        assert_eq!(
            serde_json::to_vec(&before).unwrap(),
            frozen,
            "edit {index} changed a retained snapshot"
        );
        assert!(
            cached.shares_content_with(
                &state
                    .project_lifecycle
                    .result_cache
                    .capture(&state.simulation)
            )
        );
    }
}

#[test]
fn runtime_changes_and_clones_reuse_results_but_invalid_edits_do_not() {
    let (mut state, _) = super::durable_content::retained_results();
    let baseline = snapshot(&state).unwrap();
    let retained = baseline.simulation_results.clone();
    state.project_lifecycle.accepted = Some(AcceptedProject::new(baseline, None));
    assert!(!has_unsaved_changes(&state));
    state.simulation.progress = 0.75;
    state.simulation.status = "runtime status".to_owned();
    state.simulation.data_version += 1;
    state.simulation.netlist_content = "runtime preview".to_owned();
    assert!(
        snapshot(&state)
            .unwrap()
            .simulation_results
            .shares_content_with(&retained)
    );
    let fork = state.clone();
    assert!(
        snapshot(&fork)
            .unwrap()
            .simulation_results
            .shares_content_with(&retained)
    );

    let elapsed = state.simulation.runs[0].elapsed_time;
    state.simulation.runs[0].elapsed_time = -1.0;
    assert!(snapshot(&state).is_err());
    assert!(has_unsaved_changes(&state));
    assert!(active_document_is_dirty(&state));
    retained.validate().unwrap();
    assert!(!has_unsaved_changes(&fork));
    state.simulation.runs[0].elapsed_time = elapsed;
    assert!(
        !has_unsaved_changes(&state),
        "returning to accepted content must be clean"
    );

    std::sync::Arc::make_mut(&mut state.simulation.runs[0].analyses[0].waveforms[0].x)[1] =
        f64::NAN;
    assert!(snapshot(&state).is_err());
    assert!(has_unsaved_changes(&state));
    std::sync::Arc::make_mut(&mut state.simulation.runs[0].analyses[0].waveforms[0].x)[1] = 1.0;
    assert!(!has_unsaved_changes(&state));
}

#[test]
fn result_fingerprint_cache_preserves_signed_zero_in_annotation_edits() {
    use crate::workbench::documents::result_document::marker_anchor_for;
    let (mut state, key) = super::durable_content::retained_results();
    state
        .ui
        .results
        .add_marker(
            key,
            marker_anchor_for(key, "V(out)"),
            "V(out)".to_owned(),
            0.0,
        )
        .unwrap();
    let baseline = snapshot(&state).unwrap();
    state.project_lifecycle.accepted = Some(AcceptedProject::new(baseline, None));
    assert!(!has_unsaved_changes(&state));
    state.ui.results.markers[0].x = -0.0;
    assert!(has_unsaved_changes(&state));
    let current = snapshot(&state).unwrap();
    assert_eq!(
        working_fingerprints(&state).unwrap().content_digest(),
        registry::content_digest(&current).unwrap()
    );
    state.ui.results.markers[0].x = 0.0;
    assert!(!has_unsaved_changes(&state));
}
