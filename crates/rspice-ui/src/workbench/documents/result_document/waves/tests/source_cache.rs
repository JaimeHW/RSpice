//! Restored history and nested source edits cannot reuse older plot evidence.

use super::*;
use crate::io::project_io::ProjectSimulationResults;
use crate::workbench::documents::result_document::frame_work::WorkCounts;

fn restored_spectrum(value: f64, samples: usize) -> SimulationState {
    let mut simulation = SimulationState::default();
    let run = simulation.start_run();
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![WaveformData::new(
            "|V(out)|",
            (1..=samples)
                .map(|sample| sample as f64)
                .collect::<Vec<_>>(),
            vec![value; samples],
            "#fff",
        )]),
    );
    run.restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.mark_running().unwrap();
    run.finish_lifecycle(crate::state::SimulationRunLifecycle::Completed)
        .unwrap();
    simulation.complete_run();
    ProjectSimulationResults::from_state(&simulation)
        .into_simulation_state()
        .unwrap()
}

fn spectrum_models(
    simulation: &SimulationState,
    results: &mut ResultsState,
) -> Arc<Vec<StripModel>> {
    results.viewer = super::super::super::ResultViewer::Bode;
    cached_models(
        simulation,
        results,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        &Tokens::default(),
    )
}

fn stats(results: &mut ResultsState, model: &StripModel) -> Option<(f64, f64, f64)> {
    let trace = &model.traces[0];
    results
        .derived
        .stats_or((trace_key(model, trace), u64::MAX, u64::MAX), || {
            crate::analysis::measurements::calculate_min_max_rms(&trace.y)
        })
}

#[test]
fn wave_cache_restored_history_replaces_same_version_and_display_sequence() {
    let mut results = ResultsState::default();
    let old = restored_spectrum(1.0, 3);
    let before = spectrum_models(&old, &mut results);
    let generation = results.models.generation();
    let restored = restored_spectrum(10.0, 3);
    assert_eq!(old.data_version, restored.data_version);
    assert_eq!(old.runs[0].id, restored.runs[0].id);
    assert_ne!(old.runs[0].dataset_id, restored.runs[0].dataset_id);

    let after = spectrum_models(&restored, &mut results);
    assert_eq!(
        after[0].analysis_key.dataset_id(),
        restored.runs[0].dataset_id
    );
    assert_eq!(after[0].traces[0].y.as_slice(), [20.0; 3]);
    assert_ne!(results.models.generation(), generation);
    assert_eq!(before[0].traces[0].y.as_slice(), [0.0; 3]);
}

#[test]
fn wave_cache_source_edits_refresh_conversions_ranges_stats_and_grid_shape() {
    let mut simulation = restored_spectrum(1.0, 3);
    let mut results = ResultsState::default();
    let before = spectrum_models(&simulation, &mut results);
    assert_eq!(
        pane_y_range(&mut results.derived, &before[0], &[0]),
        Some((-1.0, 1.0))
    );
    assert_eq!(stats(&mut results, &before[0]), Some((0.0, 0.0, 0.0)));
    let version = simulation.data_version;
    Arc::make_mut(&mut simulation.runs[0].analyses[0].waveforms[0].y)
        .copy_from_slice(&[1.0, 10.0, 100.0]);

    let after = spectrum_models(&simulation, &mut results);
    assert_eq!(simulation.data_version, version);
    assert_eq!(after[0].traces[0].y.as_slice(), [0.0, 20.0, 40.0]);
    assert_eq!(
        pane_y_range(&mut results.derived, &after[0], &[0]),
        Some((-3.2, 43.2))
    );
    let actual_stats = stats(&mut results, &after[0]).unwrap();
    assert_eq!((actual_stats.0, actual_stats.1), (0.0, 40.0));

    Arc::make_mut(&mut simulation.runs[0].analyses[0].waveforms[0].x)
        .copy_from_slice(&[3.0, 2.0, 1.0]);
    let reversed = spectrum_models(&simulation, &mut results);
    assert!(!reversed[0].traces[0].shape.is_single_ascending());
    assert_eq!(reversed[0].traces[0].x.as_slice(), [3.0, 2.0, 1.0]);
    assert_eq!(before[0].traces[0].x.as_slice(), [1.0, 2.0, 3.0]);
}

#[test]
fn wave_cache_unchanged_reads_and_clones_reuse_complete_models() {
    for samples in [3, 100_000] {
        let simulation = restored_spectrum(1.0, samples);
        let mut results = ResultsState::default();
        let first = spectrum_models(&simulation, &mut results);
        let generation = results.models.generation();
        let clone = simulation.clone();
        let mut cloned_results = results.clone();
        let work = WorkCounts::reset();
        for _ in 0..12 {
            assert!(Arc::ptr_eq(
                &first,
                &spectrum_models(&simulation, &mut results)
            ));
            assert!(Arc::ptr_eq(
                &first,
                &spectrum_models(&clone, &mut cloned_results)
            ));
        }
        assert_eq!(work.since().total(), 0);
        assert_eq!(results.models.generation(), generation);
        assert_eq!(cloned_results.models.generation(), generation);
    }
}

#[test]
fn wave_cache_cursor_copy_refreshes_values_without_frame_preparation() {
    let mut state = AppState::default();
    state.simulation = restored_spectrum(1.0, 3);
    let models = spectrum_models(&state.simulation, &mut state.ui.results);
    assert_eq!(models[0].traces[0].y.as_slice(), [0.0; 3]);
    state.ui.results.cursor_strip = Some(0);
    state.ui.results.cursors.a = Some(1.0);
    Arc::make_mut(&mut state.simulation.runs[0].analyses[0].waveforms[0].y).fill(10.0);

    let copied = copy_cursor_text(&mut state).expect("the cursor has a retained readout");
    let magnitude = copied
        .lines()
        .find(|line| line.ends_with(" dB"))
        .expect("the AC readout carries decibels");
    let (_, value) = magnitude.split_once(" = ").unwrap();
    let value = value
        .strip_suffix(" dB")
        .unwrap()
        .trim()
        .parse::<f64>()
        .unwrap();
    assert_eq!(value, 20.0, "the clipboard reused old data: {copied}");
}

#[test]
fn wave_cache_family_envelopes_follow_rebuilt_model_generations() {
    let mut simulation = SimulationState::default();
    simulation
        .start_run()
        .add_analysis(family_analysis(vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0]));
    let Some(AnalysisResultFamilyMetadata::Corner { x_values, .. }) =
        simulation.runs[0].analyses[0].family_metadata.as_mut()
    else {
        panic!("corner family fixture");
    };
    *x_values = vec![1.0, 2.0, 1.0, 2.0, 1.0, 2.0];
    let run = &simulation.runs[0];
    let manifest = FamilyManifest::from_analysis(&run.analyses[0])
        .unwrap()
        .unwrap();
    let selection = SourceSampleSelection::new(run.dataset_id, 41, vec![0, 1, 2, 3, 4, 5])
        .unwrap()
        .with_family_presentation(&manifest, &family_policy())
        .unwrap();
    let mut results = ResultsState::default();
    results.set_sample_selection(Some(selection));
    results.viewer = super::super::super::ResultViewer::Table;
    let models = cached_models(
        &simulation,
        &mut results,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        &Tokens::default(),
    );
    let generation = results.models.generation();
    let pane = models[0].unit_panes().remove(0);
    let old = super::super::extent::family_envelopes(&mut results, generation, &models[0], &pane);
    assert!(!old.series().is_empty());
    for sample in Arc::make_mut(&mut simulation.runs[0].analyses[0].waveforms[0].y) {
        *sample += 100.0;
    }
    let updated = cached_models(
        &simulation,
        &mut results,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        &Tokens::default(),
    );
    let next = results.models.generation();
    assert_ne!(generation, next);
    let pane = updated[0].unit_panes().remove(0);
    let new = super::super::extent::family_envelopes(&mut results, next, &updated[0], &pane);
    assert_eq!(
        new.series()[0].minimum[0],
        old.series()[0].minimum[0] + 100.0
    );
    assert_eq!(
        new.series()[0].maximum[0],
        old.series()[0].maximum[0] + 100.0
    );

    // Legend actions explicitly invalidate the model slot. Each rebuilt
    // projection must still advance past the envelope it replaces.
    let keys: Vec<_> = updated[0]
        .traces
        .iter()
        .filter_map(|trace| trace.family_visibility_key)
        .collect();
    assert_eq!(keys.len(), 3);
    let mut previous_generation = next;
    for key in keys {
        results.toggle_family_trace_visibility(key);
        let hidden = cached_models(
            &simulation,
            &mut results,
            ComplexNumberDisplay::MagnitudePhaseDegrees,
            &Tokens::default(),
        );
        let generation = results.models.generation();
        assert_ne!(generation, previous_generation);
        previous_generation = generation;
        let pane = hidden[0].unit_panes().remove(0);
        let envelope =
            super::super::extent::family_envelopes(&mut results, generation, &hidden[0], &pane);
        if hidden[0].traces.iter().all(|trace| !trace.visible) {
            assert!(envelope.series().is_empty());
        }
    }
}
