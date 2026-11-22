//! Shared-axis viewport transformations stay inside the retained domain.

use super::*;

#[test]
fn overview_preserves_gaps_and_reuses_bounded_knots_until_source_changes() {
    let mut values = vec![1.0; 10_000];
    values[5011] = f64::NAN;
    let x = (0..10_000).map(f64::from).collect::<Vec<_>>();
    let mut cache = DerivedSeries::default();
    let mut builds = 0;
    for _ in 0..3 {
        let indices = cache.overview_indices_or(1, || {
            builds += 1;
            rspice_results::waveform::display_sample_indices(&values, SHARED_X_OVERVIEW_POINTS)
        });
        assert!(indices.len() <= SHARED_X_OVERVIEW_POINTS);
        let segments = overview_segments(&x, &values, &indices);
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

    // Dragging an edge past its neighbour cannot collapse the window: it
    // stops at the plots' own floor about the edge that holds.
    let crossed = resized_shared_x_view(XScale::Linear, (0.0, 100.0), (20.0, 60.0), true, 0.95)
        .expect("a crossed drag still yields a view");
    let (low, high) = floored_span((60.0, 60.0), XScale::Linear).expect("a floor at 60");
    assert!(
        (crossed.1 - 60.0).abs() < 1.0e-12,
        "the held edge moved: {crossed:?}"
    );
    assert!(((crossed.1 - crossed.0) - (high - low)).abs() < 1.0e-15);
}

/// The ruler's wheel, its handles and the panes' own gestures stop at one
/// floor, where three once stood: a two-hundredth of the sweep for a handle,
/// a millionth of it for the wheel, a billionth of the value for a pane.
#[test]
fn every_zoom_stops_at_the_one_floor() {
    let full = (0.0, 5.0e-3);
    let deepest =
        zoomed_shared_x_view(XScale::Linear, full, full, 0.5, 1.0e-30).expect("a deepest zoom");
    let (low, high) = floored_span((2.5e-3, 2.5e-3), XScale::Linear).expect("a floor");
    assert!(
        ((deepest.1 - deepest.0) - (high - low)).abs() < 1.0e-18,
        "the ruler's wheel stopped at {:e}, the plots' floor is {:e}",
        deepest.1 - deepest.0,
        high - low
    );
    let handle = resized_shared_x_view(XScale::Linear, full, full, false, 0.0).expect("a view");
    let (low, high) = floored_span((0.0, 0.0), XScale::Linear).expect("a floor at zero");
    assert!(((handle.1 - handle.0) - (high - low)).abs() < 1.0e-18);
}

/// A focused ruler takes Left and Right itself: the viewport pans, and egui
/// does not also move the focus to the widget to its right.
#[test]
fn a_focused_ruler_keeps_its_arrows() {
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    let mut current = (2.0, 4.0);
    let mut beside = None;
    for frame in 0..6 {
        // Tab from nothing focuses the first widget, the ruler; Right
        // follows a few frames later.
        let key = match frame {
            1 => Some(egui::Key::Tab),
            5 => Some(egui::Key::ArrowRight),
            _ => None,
        };
        let events = key
            .map(|key| egui::Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            })
            .into_iter()
            .collect();
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(600.0, 200.0),
            )),
            time: Some(f64::from(frame) / 60.0),
            events,
            ..Default::default()
        };
        let _ = ctx.run_ui(input, |ui| {
            let text = |value: f64| format!("{value}");
            let ruler_rect =
                egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(420.0, 60.0));
            let output = ui
                .scope_builder(egui::UiBuilder::new().max_rect(ruler_rect), |ui| {
                    show_ruler(
                        ui,
                        Ruler {
                            id: egui::Id::new("ruler-under-test"),
                            axis: Axis::linear(current.0, current.1, "s"),
                            scale: XScale::Linear,
                            name: "Time",
                            caption: rspice_ui_kit::plot::x_axis_caption("Time", "s"),
                            cursor_text: &text,
                            overview: None,
                            hover: None,
                        },
                        SharedXAxis {
                            full_domain: (0.0, 10.0),
                            current,
                            height: WAVE_RULER_HEIGHT_WITHOUT_OVERVIEW,
                            left_margin: 56.0,
                            right_margin: 0.0,
                            quantity_policy: QuantityPresentationPolicy::default(),
                            significant_digits: 4,
                            cursors: None,
                        },
                    )
                })
                .inner;
            if let Some(SharedXViewChange::Range(range)) = output.viewport {
                current = range;
            }
            let button = ui.put(
                egui::Rect::from_min_size(egui::pos2(460.0, 10.0), egui::vec2(100.0, 24.0)),
                egui::Button::new("beside the ruler"),
            );
            beside = Some(button.id);
        });
    }
    let focused = ctx.memory(|memory| memory.focused());
    assert!(
        (current.0 - 2.5).abs() < 1.0e-9,
        "Right did not pan the ruler: {current:?}"
    );
    assert!(focused.is_some(), "the focus went nowhere");
    assert_ne!(focused, beside, "Right moved the focus off the ruler");
}
