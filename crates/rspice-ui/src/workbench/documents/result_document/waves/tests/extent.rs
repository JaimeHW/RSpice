//! The derived geometry of a strip: its shared X extent and its family
//! envelopes, and the memos both are resolved through.
//!
//! Both answers cost a walk of every visible sample, so each is resolved once
//! per model rebuild rather than per frame. That makes *when* they are
//! resolved part of their correctness: an extent taken before the reader's
//! visibility overrides describes a strip nobody is looking at.

use super::*;

/// The extent is resolved while the model is built now, so it has to be the
/// same answer the per-frame walk gave: the axis, the overview lane and every
/// viewport gesture are derived from it.
#[test]
fn the_strip_extent_matches_the_walk_it_replaced() {
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            WaveformData::new("V(a)", vec![0.0, 1.0, 2.5], vec![0.0, 1.0, 2.0], "#fff"),
            WaveformData::new("V(b)", vec![-3.0, 0.5], vec![1.0, 2.0], "#0af"),
        ]),
    );
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );

    let model = &models[0];
    let expected = model
        .traces
        .iter()
        .filter(|trace| trace.visible)
        .flat_map(|trace| trace.x.iter().copied())
        .filter(|value| value.is_finite())
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), x| {
            (low.min(x), high.max(x))
        });
    assert_eq!(model.x_range, Some(expected));
}

/// The extent is a memo of the visible traces, so hiding one has to move it.
/// A strip that kept an extent covering a trace it no longer draws would
/// scale every pane against data the reader cannot see.
#[test]
fn hiding_a_trace_moves_the_strip_extent() {
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            WaveformData::new("V(a)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff"),
            WaveformData::new("V(b)", vec![0.0, 40.0], vec![1.0, 2.0], "#0af"),
        ]),
    );
    let presentation = state.ui.preferences.result_presentation_policy();
    let before = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    assert_eq!(before[0].x_range, Some((0.0, 40.0)));

    state.simulation.runs[0].analyses[0].waveforms[1].visible = false;
    let after = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    assert_eq!(
        after[0].x_range,
        Some((0.0, 1.0)),
        "the strip kept an extent covering a trace it no longer draws"
    );
}

/// Nobody hides a trace by rewriting the retained dataset. The legend, the
/// design navigator and the inspector all call `toggle_visibility`, which
/// writes the session's override map and leaves the solver's data flag alone.
/// The extent is a memo of what the strip draws, so it has to move for the
/// path the product actually uses.
#[test]
fn hiding_a_trace_through_the_override_map_moves_the_strip_extent() {
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            WaveformData::new("V(a)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff"),
            WaveformData::new("V(b)", vec![0.0, 40.0], vec![1.0, 2.0], "#0af"),
        ]),
    );
    let presentation = state.ui.preferences.result_presentation_policy();
    let before = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    assert_eq!(before[0].x_range, Some((0.0, 40.0)));

    toggle_visibility(&mut state, 0, 1);
    assert!(
        state.simulation.runs[0].analyses[0].waveforms[1].visible,
        "the override path must not rewrite the retained data flag"
    );

    let after = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    assert!(
        !after[0]
            .traces
            .iter()
            .find(|trace| !trace.overlay && trace.waveform_index == 1)
            .expect("the overridden source still has a projected trace")
            .visible,
        "the override did not reach the projected trace at all"
    );
    assert_eq!(
        after[0].x_range,
        Some((0.0, 1.0)),
        "the strip kept an extent covering a trace the reader hid"
    );
}

/// Hiding the last trace leaves the strip with nothing to scale against, and
/// every caption, axis and viewport gesture reads that as "No data". An extent
/// baked before the overrides were applied reports a span instead.
#[test]
fn hiding_every_trace_through_the_override_map_leaves_the_strip_no_extent() {
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            WaveformData::new("V(a)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff"),
            WaveformData::new("V(b)", vec![0.0, 40.0], vec![1.0, 2.0], "#0af"),
        ]),
    );
    toggle_visibility(&mut state, 0, 0);
    toggle_visibility(&mut state, 0, 1);

    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    assert_eq!(models.len(), 1, "the strip itself stays available");
    assert!(
        models[0].traces.iter().all(|trace| !trace.visible),
        "the overrides did not reach the projected traces"
    );
    assert_eq!(
        models[0].x_range, None,
        "a strip drawing nothing still reported a domain"
    );
}

/// The extent walk belongs to the cache build, not to the frame. Resolving it
/// after the visibility overrides rather than during the projection must stay
/// exactly one walk per strip per rebuild — not two, and never one per frame.
#[test]
fn the_extent_is_walked_once_per_strip_per_cache_build() {
    use super::super::super::frame_work::{DatasetWalk, WorkCounts};

    let mut state = AppState::default();
    let run = state.simulation.start_run();
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran A").with_waveforms(vec![
            WaveformData::new("V(a)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff"),
        ]),
    );
    run.add_analysis(
        AnalysisResult::new(2, AnalysisType::Transient, "Tran B").with_waveforms(vec![
            WaveformData::new("V(b)", vec![0.0, 40.0], vec![1.0, 2.0], "#0af"),
        ]),
    );
    let presentation = state.ui.preferences.result_presentation_policy();

    let baseline = WorkCounts::reset();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    assert_eq!(models.len(), 2);
    assert_eq!(
        baseline.since().get(DatasetWalk::WaveXRange),
        2,
        "the cache build resolved the extent other than once per strip"
    );

    let baseline = WorkCounts::reset();
    for _ in 0..5 {
        let _ = cached_models(
            &state.simulation,
            &mut state.ui.results,
            presentation.complex_number_display(),
            &Tokens::default(),
        );
    }
    assert_eq!(
        baseline.since().get(DatasetWalk::WaveXRange),
        0,
        "a memo hit walked the dataset for an extent it already holds"
    );
}

/// The envelope walks every sample of every family member, so it is memoized
/// against the generation of models that produced it — and must rebuild when
/// that generation moves.
#[test]
fn family_envelopes_are_memoized_against_the_models_that_produced_them() {
    let mut active = SimulationRun::new(2);
    active.add_analysis(family_analysis(vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0]));
    let active_dataset = active.dataset_id;
    let manifest = FamilyManifest::from_analysis(&active.analyses[0])
        .unwrap()
        .unwrap();
    let selection = SourceSampleSelection::new(active_dataset, 41, vec![0, 1, 2, 3, 4, 5])
        .unwrap()
        .with_family_presentation(&manifest, &family_policy())
        .unwrap();
    let simulation = SimulationState {
        runs: vec![active],
        active_run_idx: Some(0),
        active_analysis_idx: Some(0),
        ..SimulationState::default()
    };
    let mut derived = DerivedSeries::default();
    let models = build_models(
        &simulation,
        &mut derived,
        &Tokens::default(),
        false,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        Some(&selection),
        &HashSet::new(),
    );
    let model = &models[0];
    let pane = model.unit_panes().into_iter().next().expect("one pane");

    let mut results = ResultsState::default();
    let first = super::super::extent::family_envelopes(&mut results, 7, model, &pane);
    let again = super::super::extent::family_envelopes(&mut results, 7, model, &pane);
    assert!(
        std::sync::Arc::ptr_eq(&first, &again),
        "the same generation rebuilt the envelope"
    );

    let next = super::super::extent::family_envelopes(&mut results, 8, model, &pane);
    assert!(
        !std::sync::Arc::ptr_eq(&first, &next),
        "a new generation of models served the previous envelope"
    );
    assert_eq!(
        next.series().len(),
        first.series().len(),
        "the rebuild is the same projection, not a different one"
    );
}
