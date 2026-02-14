use super::*;

#[test]
fn test_calculate_layout() {
    let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 600.0));
    let layout = calculate_layout(rect);

    // Verify layout regions don't overlap incorrectly
    assert!(layout.header.max.y <= layout.plot.min.y);
    assert!(layout.y_axis.max.x <= layout.plot.min.x);
    assert!(layout.plot.max.x <= layout.legend.min.x);
    assert!(layout.plot.max.y <= layout.x_axis.max.y);
}

#[test]
fn test_calculate_layout_small() {
    let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(300.0, 200.0));
    let layout = calculate_layout(rect);

    // Should still produce valid layout even if cramped
    assert!(layout.plot.width() >= 0.0);
    assert!(layout.plot.height() >= 0.0);
}

#[test]
fn test_layout_uses_fft_matched_chart_top_gap() {
    let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 600.0));
    let layout = calculate_layout(rect);

    assert!((layout.plot.min.y - layout.header.max.y - CHART_TOP_GAP).abs() < f32::EPSILON);
    assert!((layout.y_axis.min.y - layout.header.max.y - CHART_TOP_GAP).abs() < f32::EPSILON);
}

#[test]
fn test_visible_trace_index_window_expands_one_sample_past_view_bounds() {
    let trace = TraceData::new(
        "T",
        vec![0.0, 1.0, 2.0, 3.0, 4.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0],
    );
    let mut view = ViewTransform::default();
    view.x_min = 1.5;
    view.x_max = 2.5;

    let window = visible_trace_index_window(&trace, &view).expect("window");
    assert_eq!(window, (1, 4));
}

#[test]
fn test_visible_trace_index_window_returns_none_when_trace_is_outside_view() {
    let trace = TraceData::new("T", vec![0.0, 1.0, 2.0], vec![0.0, 0.0, 0.0]);
    let mut view = ViewTransform::default();
    view.x_min = 10.0;
    view.x_max = 11.0;

    assert!(visible_trace_index_window(&trace, &view).is_none());
}

#[test]
fn test_should_render_trace_directly_uses_visible_density_not_total_trace_length() {
    assert!(should_render_trace_directly(1000, 900));
    assert!(!should_render_trace_directly(1000, 5000));
}

#[test]
fn test_measurement_cursor_range_requires_dual_cursor_and_flag() {
    let mut state = WaveformViewerState::new();
    state.measurement_use_cursor_range = true;
    assert!(measurement_cursor_range(&state).is_none());

    state.cursors.place(4.0);
    state.cursors.place(1.0);
    assert_eq!(measurement_cursor_range(&state), Some((1.0, 4.0)));

    state.measurement_use_cursor_range = false;
    assert!(measurement_cursor_range(&state).is_none());
}

#[test]
fn test_measurement_trace_indices_respect_scope() {
    let mut state = WaveformViewerState::new();
    let mut t0 = TraceData::new("A", vec![0.0, 1.0], vec![0.0, 1.0]);
    let mut t1 = TraceData::new("B", vec![0.0, 1.0], vec![1.0, 2.0]);
    t0.visible = true;
    t1.visible = false;
    state.traces = vec![t0, t1];

    state.measurement_scope = MeasurementScope::Visible;
    assert_eq!(measurement_trace_indices(&state), vec![0]);

    state.measurement_scope = MeasurementScope::All;
    assert_eq!(measurement_trace_indices(&state), vec![0, 1]);

    state.measurement_scope = MeasurementScope::Selected;
    state.selected_trace = Some("B".to_string());
    assert_eq!(measurement_trace_indices(&state), vec![1]);

    state.selected_trace = Some("Missing".to_string());
    assert!(measurement_trace_indices(&state).is_empty());
}

#[test]
fn test_center_waveform_view_x_on_marker_preserves_span_and_clamps() {
    let mut view = ViewTransform::new(20.0, 40.0, -1.0, 1.0);
    let bounds = crate::waveform::state::DataBounds {
        x_min: 0.0,
        x_max: 100.0,
        y_min: -1.0,
        y_max: 1.0,
        valid: true,
    };

    center_waveform_view_x_on_marker(&mut view, &bounds, 10.0);
    assert!((view.x_max - view.x_min - 20.0).abs() < 1e-9);
    assert!((view.x_min - 0.0).abs() < 1e-9);
    assert!((view.x_max - 20.0).abs() < 1e-9);
}

#[test]
fn test_center_waveform_view_x_on_marker_ignores_non_finite_inputs() {
    let mut view = ViewTransform::new(5.0, 15.0, -1.0, 1.0);
    let bounds = crate::waveform::state::DataBounds {
        x_min: 0.0,
        x_max: 100.0,
        y_min: -1.0,
        y_max: 1.0,
        valid: true,
    };

    center_waveform_view_x_on_marker(&mut view, &bounds, f64::NAN);
    assert!((view.x_min - 5.0).abs() < 1e-9);
    assert!((view.x_max - 15.0).abs() < 1e-9);
}

#[test]
fn test_build_export_payload_routes_by_format() {
    let traces = vec![TraceData::new("V(out)", vec![0.0, 1e-6], vec![0.0, 1.0])];

    let mut csv_opts = super::super::export::ExportOptions::default();
    csv_opts.format = ExportFormat::Csv;
    let csv = build_export_payload(&traces, &csv_opts);
    assert!(csv.contains("Time,"));

    let mut tsv_opts = super::super::export::ExportOptions::default();
    tsv_opts.format = ExportFormat::Tsv;
    let tsv = build_export_payload(&traces, &tsv_opts);
    assert!(tsv.contains('\t'));

    let mut raw_opts = super::super::export::ExportOptions::default();
    raw_opts.format = ExportFormat::SpiceRaw;
    let raw = build_export_payload(&traces, &raw_opts);
    assert!(raw.contains("Title: RSpice Waveforms"));
    assert!(raw.contains("Values:"));
}

fn screen_to_data_y(layout: &ViewerLayout, view: &ViewTransform, screen_y: f32) -> f64 {
    let y_frac = (screen_y - layout.plot.min.y) as f64 / layout.plot.height() as f64;
    view.y_max - y_frac * view.y_range()
}

#[test]
fn test_build_trace_polyline_uses_visible_window_density() {
    let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(1200.0, 600.0));
    let layout = calculate_layout(rect);
    let mut view = ViewTransform::default();
    view.x_min = 10.0;
    view.x_max = 10.1;
    view.y_min = -1.2;
    view.y_max = 1.2;

    let n = 200_000usize;
    let dt = 20.0 / (n as f64 - 1.0);
    let x: Vec<f64> = (0..n).map(|i| i as f64 * dt).collect();
    let y: Vec<f64> = x
        .iter()
        .map(|t| (2.0 * std::f64::consts::PI * 5_000.0 * t).sin())
        .collect();
    let trace = TraceData::new("HF", x, y);

    let polyline = build_trace_polyline(&layout, &view, &trace);
    assert!(polyline.visible_samples > 900);
    assert!(polyline.visible_samples < 1100);
    assert!(polyline.points.len() > 200);
}

#[test]
fn test_build_trace_polyline_bucket_decimation_preserves_extrema() {
    let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(900.0, 500.0));
    let layout = calculate_layout(rect);
    let mut view = ViewTransform::default();
    view.x_min = 0.0;
    view.x_max = 1.0;
    view.y_min = -1.2;
    view.y_max = 1.2;

    let n = 200_000usize;
    let x: Vec<f64> = (0..n).map(|i| i as f64 / (n as f64 - 1.0)).collect();
    let y: Vec<f64> = x
        .iter()
        .map(|t| (2.0 * std::f64::consts::PI * 250.0 * t).sin())
        .collect();
    let trace = TraceData::new("HF", x, y);

    let polyline = build_trace_polyline(&layout, &view, &trace);
    assert!(polyline.points.len() >= layout.plot.width() as usize);

    let y_values: Vec<f64> = polyline
        .points
        .iter()
        .map(|p| screen_to_data_y(&layout, &view, p.y))
        .collect();
    let max_y = y_values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let min_y = y_values.iter().copied().fold(f64::INFINITY, f64::min);
    assert!(max_y > 0.95);
    assert!(min_y < -0.95);
}

#[test]
fn test_build_trace_polyline_ignores_non_finite_samples() {
    let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 480.0));
    let layout = calculate_layout(rect);
    let mut view = ViewTransform::default();
    view.x_min = 0.0;
    view.x_max = 4.0;
    view.y_min = -2.0;
    view.y_max = 2.0;

    let trace = TraceData::new(
        "NF",
        vec![0.0, 1.0, 2.0, 3.0, 4.0],
        vec![0.0, f64::NAN, 1.0, f64::INFINITY, -1.0],
    );

    let polyline = build_trace_polyline(&layout, &view, &trace);
    assert!(polyline.points.len() >= 2);
    assert!(polyline
        .points
        .iter()
        .all(|p| p.x.is_finite() && p.y.is_finite()));
}

#[test]
fn test_color_constants() {
    // Verify color functions return distinguishable colors
    assert_ne!(grid_major_color(), grid_minor_color());
    assert_ne!(cursor1_color(), cursor2_color());
    assert_ne!(box_select_fill(), box_select_stroke());
}

#[test]
fn test_layout_dimensions() {
    assert!(Y_AXIS_WIDTH > 0.0);
    assert!(X_AXIS_HEIGHT > 0.0);
    assert!(HEADER_HEIGHT > 0.0);
    assert!(LEGEND_WIDTH_MIN > 0.0);
    assert!(LEGEND_WIDTH_MAX >= LEGEND_WIDTH_MIN);
    assert!(LEGEND_WIDTH_FRACTION > 0.0);
    assert!((LEGEND_TRACE_SOLO_WIDTH - LEGEND_TRACE_CONTROL_WIDTH).abs() < f32::EPSILON);
}

#[test]
fn test_active_solo_trace_index_detects_single_visible_trace() {
    let mut traces = vec![
        TraceData::new("A", vec![0.0], vec![0.0]),
        TraceData::new("B", vec![0.0], vec![0.0]),
        TraceData::new("C", vec![0.0], vec![0.0]),
    ];
    traces[0].visible = false;
    traces[1].visible = true;
    traces[2].visible = false;
    assert_eq!(active_solo_trace_index(&traces), Some(1));
}

#[test]
fn test_active_solo_trace_index_returns_none_for_ambiguous_visibility() {
    let mut traces = vec![
        TraceData::new("A", vec![0.0], vec![0.0]),
        TraceData::new("B", vec![0.0], vec![0.0]),
    ];
    traces[0].visible = true;
    traces[1].visible = true;
    assert_eq!(active_solo_trace_index(&traces), None);

    traces[0].visible = false;
    traces[1].visible = false;
    assert_eq!(active_solo_trace_index(&traces), None);
}

#[test]
fn test_layout_legend_width_tracks_dynamic_policy() {
    let wide = Rect::from_min_size(Pos2::ZERO, Vec2::new(1800.0, 700.0));
    let wide_layout = calculate_layout(wide);
    let wide_legend_width = wide_layout.legend.width();
    assert!(wide_legend_width <= LEGEND_WIDTH_MAX + f32::EPSILON);
    assert!(wide_legend_width >= LEGEND_WIDTH_MIN - f32::EPSILON);

    let narrow = Rect::from_min_size(Pos2::ZERO, Vec2::new(340.0, 260.0));
    let narrow_layout = calculate_layout(narrow);
    let narrow_legend_width = narrow_layout.legend.width();
    assert!(narrow_legend_width <= LEGEND_WIDTH_MAX + f32::EPSILON);
    assert!(narrow_legend_width >= LEGEND_WIDTH_MIN - f32::EPSILON);

    // Width should increase for wider layouts (up to max clamp).
    assert!(wide_legend_width >= narrow_legend_width);
}

#[test]
fn test_waveform_right_pane_width_bounds_reserve_plot_width() {
    let total = Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 600.0));
    let (_min, max) = waveform_right_pane_width_bounds(total);
    let remaining_plot_width = total.width() - Y_AXIS_WIDTH - max;
    assert!(remaining_plot_width >= LEGEND_MIN_PLOT_WIDTH - f32::EPSILON);
}

#[test]
fn test_resolve_waveform_right_pane_width_uses_auto_hint_when_not_overridden() {
    let total = Rect::from_min_size(Pos2::ZERO, Vec2::new(1200.0, 700.0));
    let resolved = resolve_waveform_right_pane_width(total, None, 320.0);
    assert!((resolved - clamp_waveform_right_pane_width(total, 320.0)).abs() < f32::EPSILON);
}

#[test]
fn test_resolve_waveform_right_pane_width_clamps_manual_override() {
    let total = Rect::from_min_size(Pos2::ZERO, Vec2::new(700.0, 500.0));
    let resolved = resolve_waveform_right_pane_width(total, Some(9_999.0), 0.0);
    let (_min, max) = waveform_right_pane_width_bounds(total);
    assert!((resolved - max).abs() < f32::EPSILON);
}

#[test]
fn test_next_waveform_right_pane_width_drag_direction_matches_splitter_motion() {
    let total = Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 600.0));
    let base = 220.0;

    // Dragging pointer right should shrink the right pane.
    let shrink = next_waveform_right_pane_width(Some(base), base, 12.0, total);
    assert!(shrink < base);

    // Dragging pointer left should expand the right pane.
    let grow = next_waveform_right_pane_width(Some(base), base, -12.0, total);
    assert!(grow > base);
}

#[test]
fn test_legend_inner_rect_uses_tighter_horizontal_inset() {
    let legend = Rect::from_min_size(Pos2::new(10.0, 20.0), Vec2::new(200.0, 300.0));
    let inner = legend_inner_rect(legend);
    assert!((inner.min.x - (legend.min.x + LEGEND_INSET_X)).abs() < f32::EPSILON);
    assert!((inner.max.x - (legend.max.x - LEGEND_INSET_X)).abs() < f32::EPSILON);
    assert!((inner.min.y - (legend.min.y + LEGEND_INSET_Y)).abs() < f32::EPSILON);
    assert!((inner.max.y - (legend.max.y - LEGEND_INSET_Y)).abs() < f32::EPSILON);
}

#[test]
fn test_calculate_legend_trace_row_layout_wide_keeps_all_columns_stable() {
    let layout = calculate_legend_trace_row_layout(180.0, 4.0);
    assert!(layout.show_swatch);
    assert!(layout.show_solo);
    assert!(layout.name_width >= LEGEND_TRACE_LABEL_MIN_WIDTH);
    // 180 - (swatch+spacing=14) - (checkbox+spacing=26) - (solo+spacing=26) = 114
    assert!((layout.name_width - 114.0).abs() < f32::EPSILON);
}

#[test]
fn test_calculate_legend_trace_row_layout_narrow_hides_optional_columns() {
    let layout = calculate_legend_trace_row_layout(88.0, 4.0);
    assert!(!layout.show_swatch);
    assert!(!layout.show_solo);
    assert!(layout.name_width >= LEGEND_TRACE_LABEL_MIN_WIDTH);
}

#[test]
fn test_calculate_legend_find_row_layout_hides_clear_when_narrow() {
    let layout = calculate_legend_find_row_layout(48.0, 4.0);
    assert!(!layout.show_clear);
    assert!((layout.edit_width - 48.0).abs() < f32::EPSILON);
}

#[test]
fn test_calculate_legend_find_row_layout_shows_clear_when_wide() {
    let layout = calculate_legend_find_row_layout(96.0, 4.0);
    assert!(layout.show_clear);
    assert!(
        (layout.edit_width - (96.0 - LEGEND_TRACE_SOLO_WIDTH - 4.0 - LEGEND_FIND_RIGHT_GUARD))
            .abs()
            < f32::EPSILON
    );
}

#[test]
fn test_legend_row_rect_is_exact_width_and_height() {
    let rect = legend_row_rect(12.0, 24.0, 140.0);
    assert!((rect.min.x - 12.0).abs() < f32::EPSILON);
    assert!((rect.min.y - 24.0).abs() < f32::EPSILON);
    assert!((rect.width() - 140.0).abs() < f32::EPSILON);
    assert!((rect.height() - LEGEND_ROW_HEIGHT).abs() < f32::EPSILON);
}

#[test]
fn test_truncate_legend_trace_name_applies_ellipsis_when_needed() {
    let mut truncated = String::new();
    let ctx = egui::Context::default();
    let _ = ctx.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            truncated = truncate_legend_trace_name(
                ui.painter(),
                "NET_SUPER_LONG_HIERARCHICAL_NAME_OUT",
                FontId::proportional(10.0),
                30.0,
            );
        });
    });
    assert!(truncated.ends_with("..."));
    assert!(truncated.len() < "NET_SUPER_LONG_HIERARCHICAL_NAME_OUT".len());
}

#[test]
fn test_truncate_legend_trace_name_keeps_short_names_intact() {
    let mut rendered = String::new();
    let ctx = egui::Context::default();
    let _ = ctx.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            rendered =
                truncate_legend_trace_name(ui.painter(), "NET1", FontId::proportional(10.0), 120.0);
        });
    });
    assert_eq!(rendered, "NET1");
}

#[test]
fn test_apply_legend_selection_sets_selected_trace_and_highlight() {
    let mut state = WaveformViewerState::new();
    state.traces = vec![
        TraceData::new("NET1", vec![0.0, 1.0], vec![0.0, 1.0]),
        TraceData::new("NET2", vec![0.0, 1.0], vec![1.0, 0.0]),
    ];
    state.traces[0].highlighted = true;

    apply_legend_selection(&mut state, Some("NET2".to_string()));

    assert_eq!(state.selected_trace.as_deref(), Some("NET2"));
    assert!(!state.traces[0].highlighted);
    assert!(state.traces[1].highlighted);
}

#[test]
fn test_trace_value_at_cursor_requires_trace_intersection() {
    let trace = TraceData::new("T", vec![0.0, 1.0, 2.0], vec![0.0, 2.0, 4.0]);
    assert!(trace_value_at_cursor(&trace, -0.1).is_none());
    assert!(trace_value_at_cursor(&trace, 2.1).is_none());
    let inside = trace_value_at_cursor(&trace, 0.5).expect("inside cursor");
    assert!((inside - 1.0).abs() < 1e-9);
}

#[test]
fn test_collect_cursor_trace_readouts_uses_only_visible_traces() {
    let mut state = WaveformViewerState::new();
    let mut a = TraceData::new("A", vec![0.0, 1.0], vec![0.0, 1.0]);
    let mut b = TraceData::new("B", vec![0.0, 1.0], vec![1.0, 2.0]);
    let mut c = TraceData::new("C", vec![0.0, 1.0], vec![2.0, 3.0]);
    a.visible = true;
    b.visible = false;
    c.visible = true;
    state.traces = vec![a, b, c];

    let rows = collect_cursor_trace_readouts(&state, Some(0.5), None);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].trace_index, 0);
    assert_eq!(rows[1].trace_index, 2);
}

#[test]
fn test_collect_cursor_trace_readouts_populates_both_cursor_values() {
    let mut state = WaveformViewerState::new();
    state.traces = vec![TraceData::new(
        "N1",
        vec![0.0, 0.5, 1.0],
        vec![0.0, 1.0, 2.0],
    )];

    let rows = collect_cursor_trace_readouts(&state, Some(0.25), Some(0.75));
    assert_eq!(rows.len(), 1);
    let row = &rows[0];
    let c1 = row.c1_value.expect("c1");
    let c2 = row.c2_value.expect("c2");
    assert!((c1 - 0.5).abs() < 1e-9);
    assert!((c2 - 1.5).abs() < 1e-9);
}

#[test]
fn test_axis_positions_follow_fft_lane_model() {
    let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 600.0));
    let layout = calculate_layout(rect);

    let x_tick = x_tick_label_position(&layout, layout.plot.center().x);
    let x_title = x_axis_title_position(&layout);
    assert!(x_tick.y > layout.plot.max.y);
    assert!(x_title.y > x_tick.y);

    let y_tick = y_tick_label_position(&layout, layout.plot.center().y);
    let y_title = y_axis_title_position(&layout, 28.0, 14.0);
    assert!(y_tick.x < layout.plot.min.x);
    assert!(y_title.x < y_tick.x);
}

#[test]
fn test_y_axis_title_position_tracks_y_value_label_width() {
    let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 600.0));
    let layout = calculate_layout(rect);
    let narrow = y_axis_title_position(&layout, 14.0, 12.0);
    let wide = y_axis_title_position(&layout, 40.0, 12.0);
    assert!(wide.x < narrow.x);
}

#[test]
fn test_y_axis_title_text_has_no_square_brackets() {
    let mut state = WaveformViewerState::new();
    state.y_axis_unit = "V".to_string();

    let title = y_axis_title_text(&state, "m");
    assert_eq!(title, "mV");
    assert!(!title.contains('['));
    assert!(!title.contains(']'));
}

#[test]
fn test_layout_waveform_cursor_labels_avoids_line_collisions_and_overlap() {
    let layout = calculate_layout(Rect::from_min_size(Pos2::ZERO, Vec2::new(920.0, 520.0)));
    let state = WaveformViewerState::new();
    let requests = vec![
        VerticalLabelRequest {
            anchor_x: layout.plot.center().x - 8.0,
            size: Vec2::new(84.0, 16.0),
        },
        VerticalLabelRequest {
            anchor_x: layout.plot.center().x + 8.0,
            size: Vec2::new(84.0, 16.0),
        },
    ];
    let lines = vec![layout.plot.center().x - 8.0, layout.plot.center().x + 8.0];

    let placements = layout_waveform_cursor_labels(&layout, &state, &requests, &lines);
    assert_eq!(placements.len(), requests.len());
    assert!(!placements[0].rect.intersects(placements[1].rect));
    for placement in &placements {
        for x in &lines {
            assert!(!(*x >= placement.rect.min.x && *x <= placement.rect.max.x));
        }
    }
}

#[test]
fn test_layout_waveform_cursor_labels_moves_below_dense_top_trace_band() {
    let layout = calculate_layout(Rect::from_min_size(Pos2::ZERO, Vec2::new(920.0, 520.0)));
    let mut state = WaveformViewerState::new();
    state.view.x_min = 0.0;
    state.view.x_max = 1.0;
    state.view.y_min = -1.0;
    state.view.y_max = 1.0;

    let mut x = Vec::new();
    let mut y = Vec::new();
    for i in 0..=120 {
        let t = i as f64 / 120.0;
        x.push(t);
        y.push(0.95);
    }
    state.traces.push(TraceData::new("TopBand", x, y));

    let requests = vec![VerticalLabelRequest {
        anchor_x: layout.plot.center().x,
        size: Vec2::new(90.0, 16.0),
    }];
    let lines = vec![layout.plot.center().x];
    let placements = layout_waveform_cursor_labels(&layout, &state, &requests, &lines);

    assert_eq!(placements.len(), 1);
    assert!(placements[0].rect.min.y > layout.plot.min.y + 2.0);
}
