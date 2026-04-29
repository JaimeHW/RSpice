use super::*;
use super::{
    interactions::{freq_to_x, handle_spectrum_interactions, mag_to_y},
    markers::{
        PlotCursorLabelSpec, render_fft_cursor_labels, render_fundamental_marker,
        render_harmonic_marker, render_peak_marker, render_user_marker,
    },
    trace::render_trace,
};

// =============================================================================
// Spectrum Rendering
// =============================================================================

pub(super) fn render_spectrum(ui: &mut Ui, layout: &FftLayout, state: &mut FftState) {
    let response = ui.allocate_rect(layout.spectrum, Sense::click_and_drag());
    render_spectrum_core(ui, layout, state);
    handle_spectrum_interactions(ui, response, spectrum_plot_rect(layout.spectrum), state);
}

pub(super) fn render_spectrum_core(ui: &mut Ui, layout: &FftLayout, state: &mut FftState) {
    let painter = ui.painter().clone();
    let rect = layout.spectrum;
    let plot_rect = spectrum_plot_rect(rect);

    // Draw non-plot surface and plot area separately so axis gutters
    // never share the same background as waveform data.
    painter.rect_filled(rect, Rounding::ZERO, surface_bg_color());
    painter.rect_filled(plot_rect, Rounding::ZERO, chart_bg_color());

    // Grid
    let grid_metrics = if state.show_grid {
        render_grid(&painter, rect, plot_rect, state)
    } else {
        GridLabelMetrics::default()
    };

    // Spectrum trace
    state.ensure_peak_cache();
    if let Some(ref data) = state.data {
        render_trace(&painter, plot_rect, data, state);
        let mut cursor_labels: Vec<PlotCursorLabelSpec> =
            Vec::with_capacity(state.marker_count().saturating_add(1));
        let mut line_x_positions: Vec<f32> = Vec::new();

        // Fundamental marker
        if let Some(ref analysis) = state.analysis {
            if let Some(fund_freq) = analysis.fundamental_frequency
                && let Some(x) = render_fundamental_marker(&painter, plot_rect, fund_freq, state)
            {
                line_x_positions.push(x);
                cursor_labels.push(PlotCursorLabelSpec {
                    anchor_x: x,
                    text: "f0".to_string(),
                    color: fundamental_color(),
                    font: FontId::proportional(10.0),
                });
            }

            // Harmonic markers
            if state.show_harmonics {
                for (freq, db) in &analysis.harmonics {
                    if let Some(x) = render_harmonic_marker(&painter, plot_rect, *freq, *db, state)
                    {
                        line_x_positions.push(x);
                    }
                }
            }
        }

        // Peak markers
        if state.show_peaks {
            for peak_index in state.cached_peak_indices().iter().copied().take(10) {
                if let Some(peak) = data.points.get(peak_index) {
                    render_peak_marker(&painter, plot_rect, peak, state);
                }
            }
        }

        for (slot_idx, marker_freq) in state.marker_frequencies.iter().copied().enumerate() {
            if let Some(label) =
                render_user_marker(plot_rect, marker_freq, data, state, &painter, slot_idx)
            {
                line_x_positions.push(label.anchor_x);
                cursor_labels.push(label);
            }
        }

        if !cursor_labels.is_empty() {
            render_fft_cursor_labels(
                &painter,
                plot_rect,
                data,
                state,
                &cursor_labels,
                &line_x_positions,
            );
        }
    }

    // Axis labels rendered in gutters so they never overlap trace data.
    painter.text(
        x_axis_title_position(rect, plot_rect),
        egui::Align2::CENTER_BOTTOM,
        "Frequency",
        FontId::proportional(10.0),
        text_color(),
    );

    let y_axis_title = state.mag_scale.display_name();
    let y_axis_title_font = FontId::proportional(10.0);
    let y_axis_title_width = measure_text_width(
        &painter,
        y_axis_title,
        y_axis_title_font.clone(),
        text_color(),
    );
    painter.text(
        y_axis_title_position(
            rect,
            plot_rect,
            grid_metrics.max_y_tick_label_width,
            y_axis_title_width,
        ),
        egui::Align2::LEFT_CENTER,
        y_axis_title,
        y_axis_title_font,
        text_color(),
    );

    // Plot border
    painter.rect_stroke(
        plot_rect,
        Rounding::ZERO,
        Stroke::new(1.0, panel_border_color()),
    );
}

#[derive(Debug, Clone)]
struct AxisTick {
    value: f64,
    label: String,
    major: bool,
}

#[derive(Debug, Clone, Copy, Default)]
struct GridLabelMetrics {
    max_y_tick_label_width: f32,
}

fn render_grid(
    painter: &egui::Painter,
    _spectrum_rect: Rect,
    plot_rect: Rect,
    state: &FftState,
) -> GridLabelMetrics {
    let freq_ticks = frequency_ticks(state, 10);
    let mag_ticks = magnitude_ticks(state, 8);
    let tick_font = FontId::proportional(9.0);
    let mut metrics = GridLabelMetrics::default();

    for tick in &freq_ticks {
        let x = freq_to_x(tick.value, plot_rect, state);
        if !x.is_finite() || x < plot_rect.min.x || x > plot_rect.max.x {
            continue;
        }
        let stroke = if tick.major {
            Stroke::new(1.0, grid_major_color())
        } else {
            Stroke::new(0.5, grid_minor_color())
        };
        painter.line_segment(
            [Pos2::new(x, plot_rect.min.y), Pos2::new(x, plot_rect.max.y)],
            stroke,
        );
        if tick.major {
            painter.text(
                x_tick_label_position(x, plot_rect),
                egui::Align2::CENTER_TOP,
                &tick.label,
                tick_font.clone(),
                text_color(),
            );
        }
    }

    for tick in &mag_ticks {
        let point = FftPoint::new(0.0, magnitude_to_linear(tick.value, state), 0.0);
        let y = mag_to_y(&point, plot_rect, state);
        if !y.is_finite() || y < plot_rect.min.y || y > plot_rect.max.y {
            continue;
        }
        let stroke = if tick.major {
            Stroke::new(1.0, grid_major_color())
        } else {
            Stroke::new(0.5, grid_minor_color())
        };
        painter.line_segment(
            [Pos2::new(plot_rect.min.x, y), Pos2::new(plot_rect.max.x, y)],
            stroke,
        );
        if tick.major {
            let width = measure_text_width(painter, &tick.label, tick_font.clone(), text_color());
            metrics.max_y_tick_label_width = metrics.max_y_tick_label_width.max(width);
            painter.text(
                y_tick_label_position(y, plot_rect),
                egui::Align2::RIGHT_CENTER,
                &tick.label,
                tick_font.clone(),
                text_color(),
            );
        }
    }

    metrics
}

fn frequency_ticks(state: &FftState, approx: usize) -> Vec<AxisTick> {
    match state.freq_scale {
        FrequencyScale::Linear => linear_ticks(state.freq_min, state.freq_max, approx, |v| {
            format_freq_tick(v)
        }),
        FrequencyScale::Log => log_ticks(state.freq_min.max(1e-12), state.freq_max),
    }
}

fn magnitude_ticks(state: &FftState, approx: usize) -> Vec<AxisTick> {
    linear_ticks(state.mag_min, state.mag_max, approx, |v| {
        match state.mag_scale {
            MagnitudeScale::Linear => {
                let value = v.max(0.0);
                if value >= 1e3 || (value > 0.0 && value < 1e-3) {
                    format!("{:.2e}", value)
                } else {
                    format!("{:.3}", value)
                }
            }
            _ => format!("{:.0}", v),
        }
    })
}

fn linear_ticks<F>(min: f64, max: f64, approx: usize, mut labeler: F) -> Vec<AxisTick>
where
    F: FnMut(f64) -> String,
{
    if !min.is_finite() || !max.is_finite() || max <= min {
        return Vec::new();
    }
    let target = approx.max(2) as f64;
    let raw_step = (max - min) / target;
    let step = nice_step(raw_step);
    if !step.is_finite() || step <= 0.0 {
        return Vec::new();
    }

    let epsilon = step * 1e-9;
    let start = (min / step).floor() * step;
    let end = (max / step).ceil() * step;

    let mut major_ticks = Vec::with_capacity(approx + 4);
    let mut major = start;
    while major <= end + epsilon && major_ticks.len() < MAX_LINEAR_MAJOR_TICKS {
        if major >= min - epsilon && major <= max + epsilon {
            major_ticks.push(major);
        }
        major += step;
    }

    let mut ticks = Vec::with_capacity(
        major_ticks
            .len()
            .saturating_mul(LINEAR_MINOR_SUBDIVISIONS)
            .min(MAX_LINEAR_MINOR_TICKS),
    );

    for &major_value in &major_ticks {
        ticks.push(AxisTick {
            value: major_value,
            label: labeler(major_value),
            major: true,
        });
    }

    let minor_step = step / LINEAR_MINOR_SUBDIVISIONS as f64;
    if !minor_step.is_finite() || minor_step <= 0.0 {
        ticks.sort_by(|a, b| a.value.total_cmp(&b.value));
        return ticks;
    }

    let mut minor_count = 0usize;
    let mut base = start;
    while base <= end + epsilon && minor_count < MAX_LINEAR_MINOR_TICKS {
        for i in 1..LINEAR_MINOR_SUBDIVISIONS {
            if minor_count >= MAX_LINEAR_MINOR_TICKS {
                break;
            }
            let value = base + i as f64 * minor_step;
            if value <= min + epsilon || value >= max - epsilon {
                continue;
            }
            let coincides_with_major = major_ticks.iter().any(|&m| (m - value).abs() < epsilon);
            if coincides_with_major {
                continue;
            }
            ticks.push(AxisTick {
                value,
                label: String::new(),
                major: false,
            });
            minor_count += 1;
        }
        base += step;
    }

    ticks.sort_by(|a, b| a.value.total_cmp(&b.value));
    ticks
}

fn log_ticks(min: f64, max: f64) -> Vec<AxisTick> {
    if !min.is_finite() || !max.is_finite() || max <= min || min <= 0.0 {
        return Vec::new();
    }
    let min_dec = min.log10().floor() as i32;
    let max_dec = max.log10().ceil() as i32;
    let mut ticks = Vec::new();
    for dec in min_dec..=max_dec {
        let decade = 10.0_f64.powi(dec);
        for mult in 1..=9 {
            let major = mult == 1;
            let value = decade * mult as f64;
            if value < min || value > max {
                continue;
            }
            ticks.push(AxisTick {
                value,
                label: if major {
                    format_freq_tick(value)
                } else {
                    String::new()
                },
                major,
            });
        }
    }
    ticks
}

fn nice_step(raw_step: f64) -> f64 {
    if !raw_step.is_finite() || raw_step <= 0.0 {
        return 1.0;
    }
    let exponent = raw_step.log10().floor();
    let base = 10.0_f64.powf(exponent);
    let fraction = raw_step / base;
    let nice_fraction = if fraction <= 1.0 {
        1.0
    } else if fraction <= 2.0 {
        2.0
    } else if fraction <= 5.0 {
        5.0
    } else {
        10.0
    };
    nice_fraction * base
}

fn format_freq_tick(freq: f64) -> String {
    if freq >= 1e9 {
        format!("{:.2}G", freq / 1e9)
    } else if freq >= 1e6 {
        format!("{:.2}M", freq / 1e6)
    } else if freq >= 1e3 {
        format!("{:.2}k", freq / 1e3)
    } else if freq >= 1.0 {
        format!("{:.2}", freq)
    } else if freq > 0.0 {
        format!("{:.2e}", freq)
    } else {
        "0".to_string()
    }
}

fn magnitude_to_linear(value: f64, state: &FftState) -> f64 {
    let z0 = if state.z0.is_finite() && state.z0 > 0.0 {
        state.z0
    } else {
        50.0
    };
    match state.mag_scale {
        MagnitudeScale::Linear => value,
        MagnitudeScale::DB => 10.0_f64.powf(value / 20.0),
        MagnitudeScale::DBc => {
            let fundamental_db = state
                .analysis
                .as_ref()
                .and_then(|analysis| analysis.fundamental_db)
                .unwrap_or(0.0);
            10.0_f64.powf((value + fundamental_db) / 20.0)
        }
        MagnitudeScale::DBm => {
            let power_w = 1e-3 * 10.0_f64.powf(value / 10.0);
            (power_w * z0).sqrt()
        }
    }
}
