//! Shared-axis viewport transformations stay inside the retained domain.

use super::*;

#[test]
fn overview_preserves_gaps_and_reuses_bounded_knots_until_source_changes() {
    let mut values = vec![1.0; 10_000];
    values[5011] = f64::NAN;
    let wave = rspice_results::waveform::RetainedWaveform::new(
        "gaps",
        (0..10_000).map(f64::from).collect::<Vec<_>>(),
        values,
    );
    let mut cache = DerivedSeries::default();
    let mut builds = 0;
    for _ in 0..3 {
        let indices = cache.overview_indices_or(1, || {
            builds += 1;
            wave.display_sample_indices(SHARED_X_OVERVIEW_POINTS)
        });
        assert!(indices.len() <= SHARED_X_OVERVIEW_POINTS);
        let segments = overview_segments(&wave.x, &wave.y, &indices);
        assert!(segments.len() >= 2);
        for segment in segments {
            for pair in segment.windows(2) {
                assert!(!(pair[0].0 < 5011.0 && pair[1].0 > 5011.0));
            }
        }
    }
    assert_eq!(builds, 1);
    cache.ensure_version(1);
    cache.overview_indices_or(1, || {
        builds += 1;
        Vec::new()
    });
    assert_eq!(builds, 2);
}

#[test]
fn shared_x_navigator_preserves_view_width_and_clamps_to_the_full_domain() {
    let linear = panned_shared_x_view(XScale::Linear, (0.0, 10.0), (2.0, 6.0), 0.7)
        .expect("a zoomed linear view can pan");
    assert!((linear.0 - 6.0).abs() < 1.0e-9);
    assert!((linear.1 - 10.0).abs() < 1.0e-9);

    let logarithmic = panned_shared_x_view(XScale::Log10, (1.0, 1.0e6), (10.0, 1.0e3), 1.0 / 6.0)
        .expect("a zoomed logarithmic view can pan");
    assert!((logarithmic.0 / 100.0 - 1.0).abs() < 1.0e-9);
    assert!((logarithmic.1 / 1.0e4 - 1.0).abs() < 1.0e-9);

    assert!(panned_shared_x_view(XScale::Linear, (0.0, 10.0), (0.0, 10.0), 0.1).is_none());
}

#[test]
fn shared_x_zoom_stays_pointer_anchored_and_inside_the_retained_domain() {
    let zoomed = zoomed_shared_x_view(XScale::Linear, (0.0, 100.0), (20.0, 60.0), 0.4, 0.5)
        .expect("a finite zoomed view");
    assert!((zoomed.0 - 30.0).abs() < 1.0e-9);
    assert!((zoomed.1 - 50.0).abs() < 1.0e-9);

    let full = zoomed_shared_x_view(XScale::Log10, (1.0, 1.0e6), (10.0, 1.0e3), 1.0, 100.0)
        .expect("zoom-out clamps to the retained range");
    assert!((full.0 - 1.0).abs() < 1.0e-9);
    assert!((full.1 / 1.0e6 - 1.0).abs() < 1.0e-9);
}

#[test]
fn shared_x_click_recenters_without_changing_window_width() {
    let view = recentered_shared_x_view(XScale::Linear, (0.0, 10.0), (1.0, 5.0), 0.8)
        .expect("a zoomed view can recenter");
    assert!((view.0 - 6.0).abs() < 1.0e-9);
    assert!((view.1 - 10.0).abs() < 1.0e-9);
    assert!(recentered_shared_x_view(XScale::Linear, (0.0, 10.0), (0.0, 10.0), 0.5).is_none());
}

#[test]
fn dragging_one_overview_handle_holds_the_opposite_edge() {
    let start = resized_shared_x_view(XScale::Linear, (0.0, 100.0), (20.0, 60.0), true, 0.1)
        .expect("the leading edge follows the pointer");
    assert!((start.0 - 10.0).abs() < 1.0e-9);
    assert!((start.1 - 60.0).abs() < 1.0e-9);

    let end = resized_shared_x_view(XScale::Linear, (0.0, 100.0), (20.0, 60.0), false, 0.9)
        .expect("the trailing edge follows the pointer");
    assert!((end.0 - 20.0).abs() < 1.0e-9);
    assert!((end.1 - 90.0).abs() < 1.0e-9);

    // Dragging an edge past its neighbour cannot collapse the window, and it
    // cannot magnify past what the zoom controls allow either.
    let crossed = resized_shared_x_view(XScale::Linear, (0.0, 100.0), (20.0, 60.0), true, 0.95)
        .expect("a crossed drag still yields a view");
    assert!(crossed.1 > crossed.0);
    assert!((crossed.1 - crossed.0 - 100.0 * SHARED_X_MIN_WINDOW).abs() < 1.0e-9);
}
