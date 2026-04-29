use super::*;

pub(super) fn handle_spectrum_interactions(
    ui: &Ui,
    response: egui::Response,
    plot_rect: Rect,
    state: &mut FftState,
) {
    if response.double_clicked() {
        state.freq_auto = true;
        state.mag_auto = true;
        state.clear_markers();
        state.update_auto_scale();
        return;
    }

    if response.clicked()
        && let Some(pos) = response.interact_pointer_pos()
        && plot_rect.contains(pos)
    {
        let freq = x_to_freq(pos.x, plot_rect, state);
        let modifiers = ui.input(|i| i.modifiers);
        if modifiers.alt && freq.is_finite() {
            state.add_marker(freq);
        }
    }

    if response.secondary_clicked()
        && let Some(pos) = response.interact_pointer_pos()
        && plot_rect.contains(pos)
    {
        let modifiers = ui.input(|i| i.modifiers);
        if modifiers.alt {
            let freq = x_to_freq(pos.x, plot_rect, state);
            let tolerance = marker_frequency_removal_tolerance(state, plot_rect, pos.x);
            state.remove_nearest_marker(freq, tolerance);
        }
    }

    if response.hovered() {
        let scroll_y = ui.input(|i| i.raw_scroll_delta.y);
        if scroll_y.abs() > f32::EPSILON {
            state.freq_auto = false;
            state.mag_auto = false;

            let zoom = (1.0f64 - (scroll_y as f64) * 0.0015).clamp(0.5, 1.5);
            let pointer = response
                .hover_pos()
                .unwrap_or(Pos2::new(plot_rect.center().x, plot_rect.center().y));
            let x_frac = ((pointer.x - plot_rect.min.x) / plot_rect.width()).clamp(0.0, 1.0) as f64;
            let y_frac =
                ((pointer.y - plot_rect.min.y) / plot_rect.height()).clamp(0.0, 1.0) as f64;

            zoom_frequency_range(state, zoom, x_frac);
            zoom_magnitude_range(state, zoom, y_frac);
        }
    }

    if response.dragged() {
        let delta = ui.input(|i| i.pointer.delta());
        if delta.length_sq() > 0.0 {
            state.freq_auto = false;
            state.mag_auto = false;

            pan_frequency_range(state, delta.x as f64, plot_rect.width() as f64);
            pan_magnitude_range(state, delta.y as f64, plot_rect.height() as f64);
        }
    }
}

fn marker_frequency_removal_tolerance(state: &FftState, plot_rect: Rect, pointer_x: f32) -> f64 {
    // Mirror waveform behavior (remove nearest within ~1% of visible span),
    // while adapting correctly for logarithmic frequency axes.
    let x_radius = (plot_rect.width() * 0.01).max(4.0);
    let x0 = (pointer_x - x_radius).clamp(plot_rect.min.x, plot_rect.max.x);
    let x1 = (pointer_x + x_radius).clamp(plot_rect.min.x, plot_rect.max.x);
    let f0 = x_to_freq(x0, plot_rect, state);
    let f1 = x_to_freq(x1, plot_rect, state);
    let tolerance = (f1 - f0).abs();
    if tolerance.is_finite() {
        tolerance.max(1e-12)
    } else {
        1e-12
    }
}

fn zoom_frequency_range(state: &mut FftState, factor: f64, center_frac: f64) {
    match state.freq_scale {
        FrequencyScale::Linear => {
            let range = state.freq_max - state.freq_min;
            if !range.is_finite() || range <= 0.0 {
                return;
            }
            let center = state.freq_min + center_frac * range;
            let new_range = (range * factor).max(1e-12);
            state.freq_min = center - center_frac * new_range;
            state.freq_max = state.freq_min + new_range;
        }
        FrequencyScale::Log => {
            let min = state.freq_min.max(1e-12);
            let max = state.freq_max.max(min * 1.000_001);
            let log_min = min.log10();
            let log_max = max.log10();
            let log_range = log_max - log_min;
            if !log_range.is_finite() || log_range <= 0.0 {
                return;
            }
            let center = log_min + center_frac * log_range;
            let new_range = (log_range * factor).max(1e-9);
            let new_min = center - center_frac * new_range;
            let new_max = new_min + new_range;
            state.freq_min = 10.0_f64.powf(new_min);
            state.freq_max = 10.0_f64.powf(new_max);
        }
    }
    if state.freq_max <= state.freq_min {
        state.freq_max = state.freq_min * 1.01;
    }
}

fn zoom_magnitude_range(state: &mut FftState, factor: f64, y_frac_from_top: f64) {
    let range = state.mag_max - state.mag_min;
    if !range.is_finite() || range <= 0.0 {
        return;
    }
    let center = state.mag_max - y_frac_from_top * range;
    let below_frac = 1.0 - y_frac_from_top;
    let above_frac = y_frac_from_top;
    let new_range = (range * factor).max(1e-12);

    state.mag_min = center - below_frac * new_range;
    state.mag_max = center + above_frac * new_range;
    if state.mag_scale == MagnitudeScale::Linear {
        state.mag_min = state.mag_min.max(0.0);
    }
    if state.mag_max <= state.mag_min {
        state.mag_max = state.mag_min + 1.0;
    }
}

fn pan_frequency_range(state: &mut FftState, delta_x_pixels: f64, width_pixels: f64) {
    if width_pixels <= 0.0 {
        return;
    }
    match state.freq_scale {
        FrequencyScale::Linear => {
            let range = state.freq_max - state.freq_min;
            if !range.is_finite() || range <= 0.0 {
                return;
            }
            let shift = -delta_x_pixels / width_pixels * range;
            state.freq_min += shift;
            state.freq_max += shift;
        }
        FrequencyScale::Log => {
            let min = state.freq_min.max(1e-12);
            let max = state.freq_max.max(min * 1.000_001);
            let log_range = max.log10() - min.log10();
            if !log_range.is_finite() || log_range <= 0.0 {
                return;
            }
            let shift_log = -delta_x_pixels / width_pixels * log_range;
            let ratio = 10.0_f64.powf(shift_log);
            state.freq_min = min * ratio;
            state.freq_max = max * ratio;
        }
    }
}

fn pan_magnitude_range(state: &mut FftState, delta_y_pixels: f64, height_pixels: f64) {
    if height_pixels <= 0.0 {
        return;
    }
    let range = state.mag_max - state.mag_min;
    if !range.is_finite() || range <= 0.0 {
        return;
    }
    let shift = delta_y_pixels / height_pixels * range;
    state.mag_min += shift;
    state.mag_max += shift;
    if state.mag_scale == MagnitudeScale::Linear && state.mag_min < 0.0 {
        let adjust = -state.mag_min;
        state.mag_min += adjust;
        state.mag_max += adjust;
    }
}

pub(super) fn freq_to_x_for_trace(freq: f64, rect: Rect, state: &FftState) -> Option<f32> {
    if !freq.is_finite() {
        return None;
    }
    match state.freq_scale {
        FrequencyScale::Linear => {
            let range = state.freq_max - state.freq_min;
            if !range.is_finite() || range <= 0.0 {
                return None;
            }
            let t = (freq - state.freq_min) / range;
            Some(rect.min.x + t as f32 * rect.width())
        }
        FrequencyScale::Log => {
            if freq <= 0.0 {
                // Nonpositive frequencies cannot be represented on a log axis.
                return None;
            }
            let f_min = state.freq_min.max(1e-12);
            let f_max = state.freq_max.max(f_min * 1.000_001);
            let log_range = f_max.log10() - f_min.log10();
            if !log_range.is_finite() || log_range <= 0.0 {
                return None;
            }
            let t = (freq.log10() - f_min.log10()) / log_range;
            Some(rect.min.x + t as f32 * rect.width())
        }
    }
}

pub(super) fn freq_to_x(freq: f64, rect: Rect, state: &FftState) -> f32 {
    match state.freq_scale {
        FrequencyScale::Linear => {
            let range = state.freq_max - state.freq_min;
            if range <= 0.0 {
                return rect.center().x;
            }
            let t = (freq - state.freq_min) / range;
            rect.min.x + t as f32 * rect.width()
        }
        FrequencyScale::Log => {
            let f_min = state.freq_min.max(1e-12);
            let f_max = state.freq_max;
            if f_max <= f_min || freq <= 0.0 {
                return rect.min.x;
            }
            let log_range = f_max.log10() - f_min.log10();
            let t = (freq.log10() - f_min.log10()) / log_range;
            rect.min.x + t as f32 * rect.width()
        }
    }
}

pub(super) fn x_to_freq(x: f32, rect: Rect, state: &FftState) -> f64 {
    let t = ((x - rect.min.x) / rect.width()).clamp(0.0, 1.0) as f64;
    match state.freq_scale {
        FrequencyScale::Linear => state.freq_min + t * (state.freq_max - state.freq_min),
        FrequencyScale::Log => {
            let min = state.freq_min.max(1e-12);
            let max = state.freq_max.max(min * 1.000_001);
            let log_val = min.log10() + t * (max.log10() - min.log10());
            10.0_f64.powf(log_val)
        }
    }
}

pub(super) fn mag_to_y(point: &FftPoint, rect: Rect, state: &FftState) -> f32 {
    let value = state.display_magnitude(point);

    let (min, max) = (state.mag_min, state.mag_max);

    let range = max - min;
    if range <= 0.0 {
        return rect.center().y;
    }

    let t = (value - min) / range;
    rect.max.y - t as f32 * rect.height()
}

pub(super) fn handle_fft_info_splitter(ui: &mut Ui, layout: &FftLayout, state: &mut FftState) {
    let half_hit = INFO_SPLITTER_HIT_WIDTH * 0.5;
    let splitter_rect = Rect::from_min_max(
        Pos2::new(layout.info.min.x - half_hit, layout.info.min.y),
        Pos2::new(layout.info.min.x + half_hit, layout.info.max.y),
    );

    let splitter_id = ui.id().with("fft_info_pane_splitter");
    let mut response = ui.interact(splitter_rect, splitter_id, Sense::click_and_drag());
    response = response.on_hover_cursor(CursorIcon::ResizeHorizontal);

    if response.double_clicked() {
        state.info_pane_width = None;
    }

    if response.dragged() {
        let delta_x = ui.ctx().input(|i| i.pointer.delta().x);
        let next = next_fft_info_pane_width(
            state.info_pane_width,
            layout.info.width(),
            delta_x,
            layout.total,
        );
        state.info_pane_width = Some(next);
    }

    let stroke_color = if response.dragged() {
        Color32::from_rgb(115, 150, 220)
    } else if response.hovered() {
        Color32::from_rgb(90, 115, 165)
    } else {
        panel_border_color()
    };
    ui.painter().line_segment(
        [
            Pos2::new(layout.info.min.x, layout.info.min.y),
            Pos2::new(layout.info.min.x, layout.info.max.y),
        ],
        Stroke::new(INFO_SPLITTER_STROKE_WIDTH, stroke_color),
    );
}
