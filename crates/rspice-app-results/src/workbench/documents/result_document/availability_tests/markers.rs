//! A marker the view cannot hold puts nothing on it, and an edit of a marker
//! the view no longer holds fails.

use super::*;

fn view_markers(state: &ResultsTestHost) -> usize {
    state
        .working_view()
        .expect("the shown run's working view")
        .markers()
        .len()
}

#[test]
fn an_invalid_placement_puts_nothing_on_the_view() {
    let mut state = transient_state();
    state.quick_frame();
    let analysis = active_analysis_key(&state);
    let samples = vec![0.0, 1.0].into();
    let place = |state: &mut ResultsTestHost, anchor: AnalysisPresentationKey, x: f64| {
        place_marker(
            &mut state.results_parts(),
            MarkerPlacement {
                analysis,
                anchor: marker_anchor_for(anchor, "V(out)"),
                trace_name: "V(out)".to_owned(),
                x,
                samples: &samples,
            },
        )
    };
    for x in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(place(&mut state, analysis, x), None);
    }
    let foreign = AnalysisPresentationKey::new(
        crate::product::DatasetId::new(),
        &state.model.simulation.retained.runs[0].analyses[0],
    );
    assert_eq!(
        place(&mut state, foreign, 0.75),
        None,
        "one marker cannot name two datasets"
    );
    assert_eq!(view_markers(&state), 0);
    assert!(state.results.results.session.document_markers.is_empty());

    assert!(place(&mut state, analysis, 0.75).is_some());
    assert_eq!(view_markers(&state), 1);
}

#[test]
fn an_edit_cannot_succeed_after_its_marker_has_been_removed() {
    let mut state = transient_state();
    state.quick_frame();
    let marker = state.place_marker("V(out)", 0.5).expect("a marker");
    remove_marker(&mut state.results_parts(), marker);
    assert!(
        commit_marker_edit(
            &mut state.results_parts(),
            marker,
            "Later edit",
            MarkerKind::Peak
        )
        .is_err()
    );
    assert_eq!(view_markers(&state), 0);
}
