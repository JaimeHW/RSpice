use super::*;
use crate::analysis::fft::data::SpectrumAnalysis;

#[test]
fn test_layout_calculation() {
    let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0));
    let layout = calculate_layout(rect);

    assert!(layout.spectrum.width() > 0.0);
    assert!(layout.spectrum.height() > 0.0);
    let plot_rect = spectrum_plot_rect(layout.spectrum);
    assert!(plot_rect.min.x > layout.spectrum.min.x);
    assert!(plot_rect.max.y < layout.spectrum.max.y);
}

#[test]
fn test_layout_uses_two_stacked_header_rows() {
    let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0));
    let layout = calculate_layout(rect);

    assert!((layout.header_top.height() - HEADER_TOP_HEIGHT).abs() < f32::EPSILON);
    assert!((layout.header_main.height() - HEADER_MAIN_HEIGHT).abs() < f32::EPSILON);
    assert!((layout.header_main.min.y - layout.header_top.max.y).abs() < f32::EPSILON);
    assert!((layout.info.min.y - layout.header_main.max.y).abs() < f32::EPSILON);
    assert!(layout.spectrum.min.y >= layout.header_main.max.y);
}

#[test]
fn test_layout_clamps_header_rows_for_short_viewports() {
    let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 40.0));
    let layout = calculate_layout(rect);

    assert!((layout.header_top.height() - 34.0).abs() < f32::EPSILON);
    assert!((layout.header_main.height() - 6.0).abs() < f32::EPSILON);
    assert!((layout.info.height() - 0.0).abs() < f32::EPSILON);
}

#[test]
fn test_sync_manual_fft_time_window_auto_tracks_source_bounds() {
    let mut state = FftState::default();
    state.time_window_auto = true;
    state.time_window_start = -1.0;
    state.time_window_end = -0.5;

    sync_manual_fft_time_window(&mut state, Some((1.0, 3.0)));

    assert!((state.time_window_start - 1.0).abs() < f64::EPSILON);
    assert!((state.time_window_end - 3.0).abs() < f64::EPSILON);
}

#[test]
fn test_sync_manual_fft_time_window_manual_clamps_and_recovers_invalid_range() {
    let mut state = FftState::default();
    state.time_window_auto = false;
    state.time_window_start = 10.0;
    state.time_window_end = 5.0;

    sync_manual_fft_time_window(&mut state, Some((1.0, 3.0)));

    assert!((state.time_window_start - 1.0).abs() < f64::EPSILON);
    assert!((state.time_window_end - 3.0).abs() < f64::EPSILON);
}

#[test]
fn test_fft_surface_and_header_backgrounds_match_viewer_chrome() {
    assert_eq!(surface_bg_color(), header_bg_color());
    assert_eq!(surface_bg_color(), viewer_header_bg_color());
}

#[test]
fn test_fft_info_panel_background_matches_waveform_panel_surface() {
    assert_eq!(panel_bg_color(), Color32::from_rgb(30, 33, 40));
}

#[test]
fn test_info_outline_rect_aligns_to_plot_top_gutter() {
    let total = Rect::from_min_size(Pos2::ZERO, Vec2::new(900.0, 620.0));
    let layout = calculate_layout(total);
    let outline = info_outline_rect(&layout).expect("outline rect");
    assert!((outline.min.y - (layout.info.min.y + AXIS_TOP_GUTTER)).abs() < f32::EPSILON);
    assert!((outline.min.x - layout.info.min.x).abs() < f32::EPSILON);
    assert!((outline.max.x - layout.info.max.x).abs() < f32::EPSILON);
    assert!((outline.max.y - layout.info.max.y).abs() < f32::EPSILON);
}

#[test]
fn test_fft_supported_for_active_analysis_only_time_domain() {
    let mut state = AppState::default();
    assert!(!fft_supported_for_active_analysis(&state));

    let mut run = crate::state::SimulationRun::new(1);
    run.add_analysis(crate::state::AnalysisResult::new(
        1,
        AnalysisType::Transient,
        "tran",
    ));
    state.simulation.runs.push(run);
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
    assert!(fft_supported_for_active_analysis(&state));

    state.simulation.runs[0].analyses[0].analysis_type = AnalysisType::Ac;
    assert!(!fft_supported_for_active_analysis(&state));
}

#[test]
fn test_spectrum_plot_rect_reserves_axis_gutters() {
    let spectrum = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 400.0));
    let plot = spectrum_plot_rect(spectrum);
    assert!((plot.min.x - spectrum.min.x - AXIS_LEFT_GUTTER).abs() < f32::EPSILON);
    assert!((plot.max.x - spectrum.max.x + AXIS_RIGHT_GUTTER).abs() < f32::EPSILON);
    assert!((plot.min.y - spectrum.min.y - AXIS_TOP_GUTTER).abs() < f32::EPSILON);
    assert!((plot.max.y - spectrum.max.y + AXIS_BOTTOM_GUTTER).abs() < f32::EPSILON);
}

#[test]
fn test_axis_titles_are_farther_from_plot_than_tick_labels() {
    let spectrum = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 400.0));
    let plot = spectrum_plot_rect(spectrum);

    let x_tick = x_tick_label_position(plot.center().x, plot);
    let x_axis = x_axis_title_position(spectrum, plot);
    assert!(x_tick.y > plot.max.y);
    assert!(x_axis.y > x_tick.y);

    let y_tick = y_tick_label_position(plot.center().y, plot);
    let y_axis = y_axis_title_position(spectrum, plot, 28.0, 14.0);
    assert!(y_tick.x < plot.min.x);
    assert!(y_axis.x < y_tick.x);
}

#[test]
fn test_y_axis_title_position_tracks_y_value_label_width() {
    let spectrum = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 400.0));
    let plot = spectrum_plot_rect(spectrum);

    let narrow = y_axis_title_position(spectrum, plot, 14.0, 12.0);
    let wide = y_axis_title_position(spectrum, plot, 40.0, 12.0);
    assert!(wide.x < narrow.x);
}

#[test]
fn test_info_content_rect_is_centered_within_panel() {
    let total = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0));
    let layout = calculate_layout(total);
    let content = info_content_rect(&layout);
    let lane = Rect::from_min_max(
        Pos2::new(layout.spectrum.max.x, layout.info.min.y),
        layout.info.max,
    );
    let inner_lane = lane.shrink(INFO_PANEL_PADDING);

    assert!((content.center().x - inner_lane.center().x).abs() < f32::EPSILON);
    assert!((content.min.x - inner_lane.min.x).abs() < f32::EPSILON);
    assert!((content.max.x - inner_lane.max.x).abs() < f32::EPSILON);
}

#[test]
fn test_layout_spectrum_and_info_panel_are_edge_aligned_without_gap() {
    let total = Rect::from_min_size(Pos2::ZERO, Vec2::new(980.0, 620.0));
    let layout = calculate_layout(total);
    assert!((layout.spectrum.max.x - layout.info.min.x).abs() < f32::EPSILON);
}

#[test]
fn test_layout_fft_cursor_labels_avoids_line_collisions_and_label_overlap() {
    let plot = Rect::from_min_size(Pos2::ZERO, Vec2::new(600.0, 320.0));
    let requests = vec![
        VerticalLabelRequest {
            anchor_x: 220.0,
            size: Vec2::new(66.0, 16.0),
        },
        VerticalLabelRequest {
            anchor_x: 230.0,
            size: Vec2::new(66.0, 16.0),
        },
    ];
    let line_x = vec![220.0, 230.0, 250.0];
    let data = FftData::default();
    let state = FftState::new();

    let placements = layout_fft_cursor_labels(plot, &requests, &line_x, &data, &state);
    assert_eq!(placements.len(), requests.len());
    assert!(!placements[0].rect.intersects(placements[1].rect));
    for placement in &placements {
        for x in &line_x {
            assert!(!(*x >= placement.rect.min.x && *x <= placement.rect.max.x));
        }
    }
}

#[test]
fn test_collect_fft_cursor_label_obstacles_samples_top_band_trace_points() {
    let plot = Rect::from_min_size(Pos2::ZERO, Vec2::new(600.0, 320.0));
    let mut state = FftState::new();
    state.freq_scale = FrequencyScale::Linear;
    state.freq_min = 0.0;
    state.freq_max = 1000.0;
    state.mag_scale = MagnitudeScale::DB;
    state.mag_min = -120.0;
    state.mag_max = 20.0;

    let freqs = vec![0.0, 100.0, 200.0, 300.0, 400.0, 500.0];
    let mags = vec![1.0; freqs.len()];
    let phases = vec![0.0; freqs.len()];
    let data = FftData::from_spectrum("top", &freqs, &mags, &phases, 1000.0);
    let obstacles = collect_fft_cursor_label_obstacles(plot, &data, &state, plot.min.y + 64.0);

    assert!(!obstacles.is_empty());
    assert!(
        obstacles
            .iter()
            .all(|r| r.max.y <= plot.min.y + 64.0 + 1e-3)
    );
}

#[test]
fn test_render_info_panel_handles_small_height_with_scroll() {
    let mut state = FftState::new();
    load_demo_data(&mut state);
    if let Some(f0) = state
        .analysis
        .as_ref()
        .and_then(|a| a.fundamental_frequency)
    {
        state.add_marker(f0);
    }
    assert!(state.analysis.is_some());
    assert!(state.source_cache.is_some());

    let ctx = egui::Context::default();
    let output = ctx.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let layout = calculate_layout(Rect::from_min_size(Pos2::ZERO, Vec2::new(640.0, 96.0)));
            render_info_panel(ui, &layout, &mut state);
        });
    });

    assert!(
        !output.shapes.is_empty(),
        "render should produce clipped shapes for constrained-height info panels"
    );
}

#[test]
fn test_marker_frequency_removal_tolerance_linear_is_span_relative() {
    let plot = Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 300.0));
    let mut state = FftState::new();
    state.freq_scale = FrequencyScale::Linear;
    state.freq_min = 0.0;
    state.freq_max = 100_000.0;

    let tol = marker_frequency_removal_tolerance(&state, plot, plot.center().x);
    // 1% of plot width each side => 2% of visible span in linear mode.
    assert!((tol - 2_000.0).abs() < 5.0, "unexpected tolerance: {}", tol);
}

#[test]
fn test_marker_frequency_removal_tolerance_log_is_positive_and_finite() {
    let plot = Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 300.0));
    let mut state = FftState::new();
    state.freq_scale = FrequencyScale::Log;
    state.freq_min = 10.0;
    state.freq_max = 1_000_000.0;

    let left_tol = marker_frequency_removal_tolerance(&state, plot, plot.min.x + 20.0);
    let right_tol = marker_frequency_removal_tolerance(&state, plot, plot.max.x - 20.0);
    assert!(left_tol.is_finite() && left_tol > 0.0);
    assert!(right_tol.is_finite() && right_tol > 0.0);
}

#[test]
fn test_render_info_panel_handles_empty_state_with_scroll_container() {
    let mut state = FftState::new();
    let ctx = egui::Context::default();
    let output = ctx.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let layout = calculate_layout(Rect::from_min_size(Pos2::ZERO, Vec2::new(480.0, 88.0)));
            render_info_panel(ui, &layout, &mut state);
        });
    });

    assert!(
        !output.shapes.is_empty(),
        "empty-state info panel should still render header/body within constrained space"
    );
}

#[test]
fn test_center_fft_frequency_view_on_marker_linear_preserves_span() {
    let mut state = FftState::new();
    state.freq_scale = FrequencyScale::Linear;
    state.freq_min = 100.0;
    state.freq_max = 1_100.0;
    let span_before = state.freq_max - state.freq_min;

    center_fft_frequency_view_on_marker(&mut state, 2_000.0);

    let span_after = state.freq_max - state.freq_min;
    assert!((span_after - span_before).abs() < 1e-9);
    assert!((state.freq_min + state.freq_max) * 0.5 >= 1_999.0);
    assert!(!state.freq_auto);
}

#[test]
fn test_center_fft_frequency_view_on_marker_linear_clamps_to_zero() {
    let mut state = FftState::new();
    state.freq_scale = FrequencyScale::Linear;
    state.freq_min = 0.0;
    state.freq_max = 100.0;

    center_fft_frequency_view_on_marker(&mut state, 1.0);
    assert!(state.freq_min >= 0.0);
    assert!(state.freq_max > state.freq_min);
}

#[test]
fn test_center_fft_frequency_view_on_marker_log_preserves_log_span() {
    let mut state = FftState::new();
    state.freq_scale = FrequencyScale::Log;
    state.freq_min = 10.0;
    state.freq_max = 1_000_000.0;
    let span_before = state.freq_max.log10() - state.freq_min.log10();

    center_fft_frequency_view_on_marker(&mut state, 1_000.0);

    let span_after = state.freq_max.log10() - state.freq_min.log10();
    assert!((span_after - span_before).abs() < 1e-9);
    let center_log = (state.freq_min.log10() + state.freq_max.log10()) * 0.5;
    assert!((center_log - 1_000.0f64.log10()).abs() < 1e-9);
    assert!(!state.freq_auto);
}

#[test]
fn test_clip_line_segment_inside_rect_is_unchanged() {
    let rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(10.0, 10.0));
    let clipped = clip_line_segment_to_rect(Pos2::new(2.0, 3.0), Pos2::new(8.0, 9.0), rect)
        .expect("segment should remain visible");
    assert!((clipped[0].x - 2.0).abs() < 1e-6);
    assert!((clipped[0].y - 3.0).abs() < 1e-6);
    assert!((clipped[1].x - 8.0).abs() < 1e-6);
    assert!((clipped[1].y - 9.0).abs() < 1e-6);
}

#[test]
fn test_clip_line_segment_fully_below_rect_is_rejected() {
    let rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(10.0, 10.0));
    let clipped = clip_line_segment_to_rect(Pos2::new(1.0, 12.0), Pos2::new(9.0, 14.0), rect);
    assert!(clipped.is_none());
}

#[test]
fn test_clip_line_segment_crossing_bottom_is_trimmed() {
    let rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(10.0, 10.0));
    let clipped = clip_line_segment_to_rect(Pos2::new(2.0, 8.0), Pos2::new(8.0, 14.0), rect)
        .expect("segment should intersect bottom edge");

    // Intersects y=10 at t=1/3 -> x=4.
    assert!((clipped[0].x - 2.0).abs() < 1e-6);
    assert!((clipped[0].y - 8.0).abs() < 1e-6);
    assert!((clipped[1].x - 4.0).abs() < 1e-5);
    assert!((clipped[1].y - 10.0).abs() < 1e-6);
}

#[test]
fn test_clip_line_segment_outside_left_and_right_is_clipped_to_vertical_edges() {
    let rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(10.0, 10.0));
    let clipped = clip_line_segment_to_rect(Pos2::new(-5.0, 5.0), Pos2::new(15.0, 5.0), rect)
        .expect("segment crosses plotting area");
    assert!((clipped[0].x - 0.0).abs() < 1e-6);
    assert!((clipped[0].y - 5.0).abs() < 1e-6);
    assert!((clipped[1].x - 10.0).abs() < 1e-6);
    assert!((clipped[1].y - 5.0).abs() < 1e-6);
}

#[test]
fn test_segment_is_trivially_outside_rect_below() {
    let rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(10.0, 10.0));
    assert!(segment_is_trivially_outside_rect(
        1.0, 12.0, 9.0, 14.0, rect
    ));
}

#[test]
fn test_segment_is_trivially_outside_rect_false_for_crossing_segment() {
    let rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(10.0, 10.0));
    assert!(!segment_is_trivially_outside_rect(
        2.0, 8.0, 8.0, 14.0, rect
    ));
}

#[test]
fn test_freq_to_x_linear() {
    let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(100.0, 100.0));
    let mut state = FftState::new();
    state.freq_min = 0.0;
    state.freq_max = 1000.0;
    state.freq_scale = FrequencyScale::Linear;

    let x = freq_to_x(500.0, rect, &state);
    assert!((x - 50.0).abs() < 0.1);
}

#[test]
fn test_freq_to_x_log() {
    let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(100.0, 100.0));
    let mut state = FftState::new();
    state.freq_min = 10.0;
    state.freq_max = 10000.0;
    state.freq_scale = FrequencyScale::Log;

    // 100Hz is 1 decade from 10, which is 1/3 of 3 decades
    let x = freq_to_x(100.0, rect, &state);
    assert!((x - 100.0 / 3.0).abs() < 1.0);
}

#[test]
fn test_freq_to_x_log_supports_sub_hz_ranges() {
    let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(120.0, 100.0));
    let mut state = FftState::new();
    state.freq_min = 1e-3;
    state.freq_max = 1e3;
    state.freq_scale = FrequencyScale::Log;

    // 1 Hz is centered across six decades (1e-3..1e3).
    let x = freq_to_x(1.0, rect, &state);
    assert!((x - rect.center().x).abs() < 1.0);
}

#[test]
fn test_freq_to_x_for_trace_log_rejects_nonpositive_frequency() {
    let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(100.0, 100.0));
    let mut state = FftState::new();
    state.freq_min = 10.0;
    state.freq_max = 10_000.0;
    state.freq_scale = FrequencyScale::Log;

    assert!(freq_to_x_for_trace(0.0, rect, &state).is_none());
    assert!(freq_to_x_for_trace(-1.0, rect, &state).is_none());
    assert!(freq_to_x_for_trace(100.0, rect, &state).is_some());
}

#[test]
fn test_format_freq() {
    assert!(format_freq(1000.0).contains("kHz"));
    assert!(format_freq(1e6).contains("MHz"));
    assert!(format_freq(1e9).contains("GHz"));
}

#[test]
fn test_load_demo_data() {
    let mut state = FftState::new();
    load_demo_data(&mut state);

    assert!(state.has_data());
    assert!(state.analysis.is_some());
    assert!(state.source_cache.is_some());
}

#[test]
fn test_refresh_fft_from_source_waveform_reference_mode_preserves_large_uniform_input() {
    let mut app_state = AppState::default();
    let fs = 2_000_000.0;
    let n = crate::analysis::fft::DEFAULT_MAX_FFT_POINTS * 3;
    let time: Vec<f64> = (0..n).map(|i| i as f64 / fs).collect();
    let values: Vec<f64> = (0..n)
        .map(|i| (2.0 * PI * 250_000.0 * i as f64 / fs).sin())
        .collect();
    app_state
        .simulation
        .waveforms
        .push(crate::state::WaveformData::new(
            "V(out)", time, values, "#4aa3ff",
        ));
    app_state
        .fft_state
        .set_input_fidelity(InputFidelity::Reference);

    refresh_fft_from_source_waveform(&mut app_state, "V(out)");

    let source = app_state
        .fft_state
        .source_cache
        .as_ref()
        .expect("source cache");
    assert_eq!(source.decimation_factor, 1);
    assert_eq!(source.samples.len(), n);
}

#[test]
fn test_refresh_fft_from_source_waveform_interactive_mode_caps_large_uniform_input() {
    let mut app_state = AppState::default();
    let fs = 2_000_000.0;
    let n = crate::analysis::fft::DEFAULT_MAX_FFT_POINTS * 3;
    let time: Vec<f64> = (0..n).map(|i| i as f64 / fs).collect();
    let values: Vec<f64> = (0..n)
        .map(|i| (2.0 * PI * 250_000.0 * i as f64 / fs).sin())
        .collect();
    app_state
        .simulation
        .waveforms
        .push(crate::state::WaveformData::new(
            "V(out)", time, values, "#4aa3ff",
        ));
    app_state
        .fft_state
        .set_input_fidelity(InputFidelity::Interactive);

    refresh_fft_from_source_waveform(&mut app_state, "V(out)");

    let source = app_state
        .fft_state
        .source_cache
        .as_ref()
        .expect("source cache");
    assert!(source.samples.len() <= crate::analysis::fft::DEFAULT_MAX_FFT_POINTS);
    assert!(source.decimation_factor > 1);
}

#[test]
fn test_refresh_fft_from_source_waveform_syncs_auto_n_control_to_effective_samples() {
    let mut app_state = AppState::default();
    let fs = 2_000_000.0;
    let n = crate::analysis::fft::DEFAULT_MAX_FFT_POINTS * 3;
    let time: Vec<f64> = (0..n).map(|i| i as f64 / fs).collect();
    let values: Vec<f64> = (0..n)
        .map(|i| (2.0 * PI * 250_000.0 * i as f64 / fs).sin())
        .collect();
    app_state
        .simulation
        .waveforms
        .push(crate::state::WaveformData::new(
            "V(out)", time, values, "#4aa3ff",
        ));
    app_state
        .fft_state
        .set_input_fidelity(InputFidelity::Interactive);
    app_state.fft_state.sample_count_auto = true;
    app_state.fft_state.sample_count = 2048;

    refresh_fft_from_source_waveform(&mut app_state, "V(out)");

    let source = app_state
        .fft_state
        .source_cache
        .as_ref()
        .expect("source cache");
    assert_eq!(app_state.fft_state.sample_count, source.samples.len());
}

#[test]
fn test_refresh_fft_from_source_waveform_applies_manual_time_window_and_sample_target() {
    let mut app_state = AppState::default();
    let fs = 100_000.0;
    let n = 100_000usize;
    let time: Vec<f64> = (0..n).map(|i| i as f64 / fs).collect();
    let values: Vec<f64> = (0..n)
        .map(|i| (2.0 * PI * 5_000.0 * i as f64 / fs).sin())
        .collect();
    app_state
        .simulation
        .waveforms
        .push(crate::state::WaveformData::new(
            "V(out)", time, values, "#4aa3ff",
        ));
    app_state
        .fft_state
        .set_input_fidelity(InputFidelity::Reference);
    app_state.fft_state.time_window_auto = false;
    app_state.fft_state.time_window_start = 0.2;
    app_state.fft_state.time_window_end = 0.4;
    app_state.fft_state.sample_count_auto = false;
    app_state.fft_state.sample_count = 2048;

    refresh_fft_from_source_waveform(&mut app_state, "V(out)");

    let source = app_state
        .fft_state
        .source_cache
        .as_ref()
        .expect("source cache");
    assert_eq!(source.decimation_factor, 1);
    assert_eq!(source.samples.len(), 2048);
    assert!(source.original_count > 15_000);
    assert!(source.original_count < 25_000);
}

#[test]
fn test_current_fft_source_time_bounds_uses_selected_source() {
    let mut app_state = AppState::default();
    app_state
        .simulation
        .waveforms
        .push(crate::state::WaveformData::new(
            "A",
            vec![0.0, 1.0, 2.0],
            vec![0.0, 0.0, 0.0],
            "#123456",
        ));
    app_state
        .simulation
        .waveforms
        .push(crate::state::WaveformData::new(
            "B",
            vec![10.0, 11.0, 12.0],
            vec![0.0, 0.0, 0.0],
            "#abcdef",
        ));
    app_state
        .fft_state
        .set_selected_source(Some("B".to_string()));

    let bounds = current_fft_source_time_bounds(&app_state).expect("bounds");
    assert!((bounds.0 - 10.0).abs() < 1e-12);
    assert!((bounds.1 - 12.0).abs() < 1e-12);
}

#[test]
fn test_x_to_freq_linear_inverse() {
    let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(200.0, 100.0));
    let mut state = FftState::new();
    state.freq_scale = FrequencyScale::Linear;
    state.freq_min = 10.0;
    state.freq_max = 1010.0;

    let f = 610.0;
    let x = freq_to_x(f, rect, &state);
    let back = x_to_freq(x, rect, &state);
    assert!((back - f).abs() < 1e-3);
}

#[test]
fn test_x_to_freq_log_inverse() {
    let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(300.0, 100.0));
    let mut state = FftState::new();
    state.freq_scale = FrequencyScale::Log;
    state.freq_min = 10.0;
    state.freq_max = 10_000.0;

    let f = 500.0;
    let x = freq_to_x(f, rect, &state);
    let back = x_to_freq(x, rect, &state);
    assert!((back - f).abs() / f < 1e-6);
}

#[test]
fn test_frequency_ticks_log_has_major_decades() {
    let mut state = FftState::new();
    state.freq_scale = FrequencyScale::Log;
    state.freq_min = 10.0;
    state.freq_max = 1_000_000.0;

    let ticks = frequency_ticks(&state, 10);
    assert!(
        ticks
            .iter()
            .any(|t| t.major && (t.value - 10.0).abs() < 1e-9)
    );
    assert!(
        ticks
            .iter()
            .any(|t| t.major && (t.value - 1000.0).abs() < 1e-9)
    );
    assert!(
        ticks
            .iter()
            .any(|t| t.major && (t.value - 100000.0).abs() < 1e-9)
    );
}

#[test]
fn test_frequency_ticks_log_contains_minor_subdivisions() {
    let mut state = FftState::new();
    state.freq_scale = FrequencyScale::Log;
    state.freq_min = 10.0;
    state.freq_max = 100.0;

    let ticks = frequency_ticks(&state, 10);
    assert!(
        ticks
            .iter()
            .any(|t| !t.major && (t.value - 20.0).abs() < 1e-9)
    );
    assert!(
        ticks
            .iter()
            .any(|t| !t.major && (t.value - 50.0).abs() < 1e-9)
    );
    assert!(
        ticks
            .iter()
            .any(|t| !t.major && (t.value - 90.0).abs() < 1e-9)
    );
}

#[test]
fn test_frequency_ticks_linear_contains_minor_gridlines() {
    let mut state = FftState::new();
    state.freq_scale = FrequencyScale::Linear;
    state.freq_min = 0.0;
    state.freq_max = 10.0;

    let ticks = frequency_ticks(&state, 5);
    let major_count = ticks.iter().filter(|t| t.major).count();
    let minor_count = ticks.iter().filter(|t| !t.major).count();

    assert!(major_count >= 3);
    assert!(minor_count > 0);
    assert!(
        ticks
            .iter()
            .filter(|t| !t.major)
            .all(|t| t.label.is_empty())
    );
}

#[test]
fn test_magnitude_ticks_contains_minor_gridlines() {
    let mut state = FftState::new();
    state.mag_scale = MagnitudeScale::DB;
    state.mag_min = -120.0;
    state.mag_max = 0.0;

    let ticks = magnitude_ticks(&state, 8);
    assert!(ticks.iter().any(|t| t.major));
    assert!(ticks.iter().any(|t| !t.major));
}

#[test]
fn test_linear_ticks_minor_do_not_overlap_major_values() {
    let ticks = linear_ticks(-5.0, 5.0, 5, |v| format!("{v:.1}"));
    let majors: Vec<f64> = ticks.iter().filter(|t| t.major).map(|t| t.value).collect();
    let epsilon = 1e-9;
    for minor in ticks.iter().filter(|t| !t.major) {
        assert!(
            majors
                .iter()
                .all(|&major| (major - minor.value).abs() > epsilon)
        );
    }
}

#[test]
fn test_linear_ticks_minor_count_is_capped() {
    let ticks = linear_ticks(0.0, 1_000_000_000.0, 1_000_000, |v| format!("{v:.0}"));
    let minor_count = ticks.iter().filter(|t| !t.major).count();
    assert!(minor_count <= MAX_LINEAR_MINOR_TICKS);
}

#[test]
fn test_magnitude_to_linear_dbm_conversion() {
    let mut state = FftState::new();
    state.mag_scale = MagnitudeScale::DBm;
    state.z0 = 50.0;
    // 13.0103 dBm ~= 1 Vrms into 50 ohm
    let v = magnitude_to_linear(13.0103, &state);
    assert!((v - 1.0).abs() < 1e-2);
}

#[test]
fn test_magnitude_to_linear_dbc_uses_fundamental_db_reference() {
    let mut state = FftState::new();
    state.mag_scale = MagnitudeScale::DBc;
    state.analysis = Some(SpectrumAnalysis {
        fundamental_frequency: Some(1_000.0),
        fundamental_db: Some(-6.0),
        harmonics: Vec::new(),
        thd_percent: None,
        thd_db: None,
        sfdr_db: None,
        snr_db: None,
        sinad_db: None,
        noise_floor_db: None,
    });
    // 0 dBc should map to the same absolute magnitude as -6 dB.
    let v = magnitude_to_linear(0.0, &state);
    assert!((v - 10.0_f64.powf(-6.0 / 20.0)).abs() < 1e-12);
}

#[test]
fn test_format_marker_magnitude_supports_dbc_units() {
    let mut state = FftState::new();
    state.mag_scale = MagnitudeScale::DBc;
    state.analysis = Some(SpectrumAnalysis {
        fundamental_frequency: Some(1_000.0),
        fundamental_db: Some(0.0),
        harmonics: Vec::new(),
        thd_percent: None,
        thd_db: None,
        sfdr_db: None,
        snr_db: None,
        sinad_db: None,
        noise_floor_db: None,
    });
    let point = FftPoint::new(2_000.0, 0.5, 0.0);
    let text = format_marker_magnitude(&state, &point);
    assert!(text.contains("dBc"));
}
