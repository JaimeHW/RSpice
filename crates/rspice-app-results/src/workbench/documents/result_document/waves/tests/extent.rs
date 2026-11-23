//! The derived geometry of a strip: its shared X extent and its family
//! envelopes, and the memos both are resolved through.
//!
//! Both answers cost a walk of every visible sample, so each is resolved once
//! per model rebuild rather than per frame. That makes *when* they are
//! resolved part of their correctness: an extent taken before the reader's
//! visibility overrides describes a strip nobody is looking at.

use super::*;

/// The extent is a memo of the visible traces, so hiding one has to move it.
/// A strip that kept an extent covering a trace it no longer draws would
/// scale every pane against data the reader cannot see.
#[test]
fn hiding_a_trace_moves_the_strip_extent() {
    let mut state = ResultsTestHost::default();
    state.model.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            RetainedTrace::new("V(a)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff"),
            RetainedTrace::new("V(b)", vec![0.0, 40.0], vec![1.0, 2.0], "#0af"),
        ]),
    );
    let presentation = state.model.preferences.result_presentation_policy();
    let before = cached_models(
        &state.model.simulation,
        &mut state.results.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    assert_eq!(before[0].x_range, Some((0.0, 40.0)));

    state.model.simulation.retained.runs[0].analyses[0].waveforms[1].visible = false;
    let after = cached_models(
        &state.model.simulation,
        &mut state.results.results,
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
    let mut state = ResultsTestHost::default();
    state.model.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            RetainedTrace::new("V(a)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff"),
            RetainedTrace::new("V(b)", vec![0.0, 40.0], vec![1.0, 2.0], "#0af"),
        ]),
    );
    let presentation = state.model.preferences.result_presentation_policy();
    let before = cached_models(
        &state.model.simulation,
        &mut state.results.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    assert_eq!(before[0].x_range, Some((0.0, 40.0)));

    toggle_visibility(&mut state.results_parts(), 0, 1);
    assert!(
        state.model.simulation.retained.runs[0].analyses[0].waveforms[1].visible,
        "the override path must not rewrite the retained data flag"
    );

    let after = cached_models(
        &state.model.simulation,
        &mut state.results.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    assert!(
        !after[0]
            .traces
            .iter()
            .any(|trace| !trace.overlay && trace.waveform_index == 1 && trace.visible),
        "the override did not reach the projected trace at all"
    );
    assert_eq!(
        after[0].x_range,
        Some((0.0, 1.0)),
        "the strip kept an extent covering a trace the reader hid"
    );
}

/// Hiding the last trace leaves the strip with nothing to scale against, and
/// every caption, axis and viewport gesture reads that as nothing plotted. An extent
/// baked before the overrides were applied reports a span instead.
#[test]
fn hiding_every_trace_through_the_override_map_leaves_the_strip_no_extent() {
    let mut state = ResultsTestHost::default();
    state.model.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            RetainedTrace::new("V(a)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff"),
            RetainedTrace::new("V(b)", vec![0.0, 40.0], vec![1.0, 2.0], "#0af"),
        ]),
    );
    toggle_visibility(&mut state.results_parts(), 0, 0);
    toggle_visibility(&mut state.results_parts(), 0, 1);

    let presentation = state.model.preferences.result_presentation_policy();
    let models = cached_models(
        &state.model.simulation,
        &mut state.results.results,
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

/// The envelope walks every sample of every family member, so it is memoized
/// against the generation of models that produced it — and must rebuild when
/// that generation moves.
#[test]
fn family_envelopes_are_memoized_against_the_models_that_produced_them() {
    let mut active = SimulationRun::new(2);
    active.add_analysis(family_analysis(vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0]));
    let active_dataset = active.dataset_id;
    let manifest = FamilyManifest::from_metadata(
        active.analyses[0].analysis_type,
        active.analyses[0].family_metadata.as_ref(),
    )
    .unwrap()
    .unwrap();
    let selection = SourceSampleSelection::new(active_dataset, 41, vec![0, 1, 2, 3, 4, 5])
        .unwrap()
        .with_family_presentation(&manifest, &family_policy())
        .unwrap();
    let simulation = SimulationState {
        retained: crate::state::RetainedSimulationState {
            runs: vec![active].into(),
            ..Default::default()
        },
        view: crate::state::SimulationViewState {
            active_run_idx: Some(0),
            active_analysis_idx: Some(0),
            ..Default::default()
        },
        ..SimulationState::default()
    };
    let mut derived = DerivedSeries::default();
    let models = build_models(
        &simulation,
        &mut derived,
        &Tokens::default(),
        false,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        Default::default(),
        Some(&selection),
        &HashSet::new(),
    );
    let model = &models[0];
    let pane = model.unit_panes().into_iter().next().expect("one pane");

    let mut results = ResultsState::default();
    let first =
        super::super::extent::family_envelopes(&mut results.plans.envelopes, 7, model, &pane);
    let again =
        super::super::extent::family_envelopes(&mut results.plans.envelopes, 7, model, &pane);
    assert!(
        std::sync::Arc::ptr_eq(&first, &again),
        "the same generation rebuilt the envelope"
    );

    let next =
        super::super::extent::family_envelopes(&mut results.plans.envelopes, 8, model, &pane);
    assert!(
        !std::sync::Arc::ptr_eq(&first, &next),
        "a new generation of models served the previous envelope"
    );
    assert_eq!(
        next.series().len(),
        first.series().len(),
        "the rebuild is the same projection, not a different one"
    );

    // Retention discards the dataset the plan projects. Its own doc promises
    // to drop every memo about a discarded dataset, and names its two
    // deliberate exceptions; the viewer projections were neither.
    results.retain_datasets(&HashSet::new());
    assert!(
        results.plans.is_empty(),
        "a viewer projection outlived the dataset it was built from"
    );
}

/// A running transient's strip spans its run from start to stop while it grows, so the trace
/// grows across an axis that stands still; a zoom taken while it runs is kept when it ends.
#[test]
fn a_running_transients_strip_frames_its_run_and_keeps_a_zoom_past_its_end() {
    use rspice_results_ui::waves::stack::{set_shared_x_view, shared_x_view};

    let trace = |last: f64| {
        let x = (0..=10)
            .map(|k| last * f64::from(k) / 10.0)
            .collect::<Vec<_>>();
        RetainedTrace::new("V(a)", x.clone(), x, "#fff")
    };
    let mut state = ResultsTestHost::default();
    state.model.simulation.execution.live_window = Some((0.0, 10.0));
    state.model.simulation.start_run().add_analysis(
        AnalysisResult::live_transient_partial(1, AnalysisType::Transient, "Tran")
            .with_waveforms(vec![trace(1.0)]),
    );
    let presentation = state.model.preferences.result_presentation_policy();
    let models = |state: &mut ResultsTestHost| {
        cached_models(
            &state.model.simulation,
            &mut state.results.results,
            presentation.complex_number_display(),
            &Tokens::default(),
        )
    };
    let running = models(&mut state);
    assert_eq!(running[0].x_range, Some((0.0, 10.0)));
    let key = running[0].analysis_key;

    for last in [4.0, 7.5] {
        state.model.simulation.retained.runs[0].analyses[0] =
            AnalysisResult::live_transient_partial(1, AnalysisType::Transient, "Tran")
                .with_waveforms(vec![trace(last)]);
        state.model.simulation.view.data_version += 1;
        assert_eq!(
            models(&mut state)[0].x_range,
            Some((0.0, 10.0)),
            "the frame stands still while the run has reached {last}"
        );
    }

    set_shared_x_view(&mut state.results.results.session, key, Some((2.0, 4.0)));
    state.model.simulation.retained.runs[0].analyses[0] =
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![trace(10.0)]);
    state.model.simulation.execution.live_window = None;
    state.model.simulation.view.data_version += 1;
    let ended = models(&mut state);
    assert_eq!(ended[0].analysis_key, key);
    assert_eq!(ended[0].x_range, Some((0.0, 10.0)));
    assert_eq!(
        shared_x_view(&state.results.results.session, key),
        Some((2.0, 4.0)),
        "the reader's zoom outlives the run's end"
    );
}

/// Following keeps a zoomed window's width and ends it at the run's newest time; a zoom of the
/// reader's ends following, and so does the run's end.
#[test]
fn following_a_running_transient_keeps_its_newest_rows_in_view_until_a_zoom() {
    use rspice_results_ui::waves::stack::{
        follow_leading_edge, set_shared_x_view, shared_x_view, start_following,
    };

    let trace = |last: f64| {
        let x = (0..=10)
            .map(|k| last * f64::from(k) / 10.0)
            .collect::<Vec<_>>();
        RetainedTrace::new("V(a)", x.clone(), x, "#fff")
    };
    let mut state = ResultsTestHost::default();
    state.model.simulation.execution.live_window = Some((0.0, 10.0));
    state.model.simulation.start_run().add_analysis(
        AnalysisResult::live_transient_partial(1, AnalysisType::Transient, "Tran")
            .with_waveforms(vec![trace(3.0)]),
    );
    let presentation = state.model.preferences.result_presentation_policy();
    let models = |state: &mut ResultsTestHost| {
        cached_models(
            &state.model.simulation,
            &mut state.results.results,
            presentation.complex_number_display(),
            &Tokens::default(),
        )
    };
    let running = models(&mut state);
    let key = running[0].analysis_key;
    let session = &mut state.results.results.session;
    set_shared_x_view(session, key, Some((0.0, 1.0)));
    start_following(session, &running[0]);
    assert_eq!(shared_x_view(session, key), Some((2.0, 3.0)));

    state.model.simulation.retained.runs[0].analyses[0] =
        AnalysisResult::live_transient_partial(1, AnalysisType::Transient, "Tran")
            .with_waveforms(vec![trace(6.0)]);
    state.model.simulation.view.data_version += 1;
    let grown = models(&mut state);
    let session = &mut state.results.results.session;
    follow_leading_edge(session, &grown[0]);
    assert_eq!(shared_x_view(session, key), Some((5.0, 6.0)));

    set_shared_x_view(session, key, Some((1.0, 2.0)));
    follow_leading_edge(session, &grown[0]);
    assert_eq!(
        shared_x_view(session, key),
        Some((1.0, 2.0)),
        "a zoom of the reader's ends following"
    );
}
