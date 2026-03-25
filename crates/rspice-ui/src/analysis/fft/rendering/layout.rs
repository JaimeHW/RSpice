use super::{
    AXIS_BOTTOM_GUTTER, AXIS_LEFT_GUTTER, AXIS_RIGHT_GUTTER, AXIS_TICK_X_OFFSET,
    AXIS_TICK_Y_OFFSET, AXIS_TITLE_BOTTOM_INSET, AXIS_TITLE_MIN_LEFT_INSET,
    AXIS_TITLE_TO_VALUE_LABEL_GAP, AXIS_TOP_GUTTER, CHART_BOTTOM_PADDING, CHART_LEFT_PADDING,
    CHART_RIGHT_PADDING, CHART_TOP_GAP, FFT_MIN_PLOT_WIDTH, HEADER_DROPDOWN_TEXT_PADDING,
    HEADER_MAIN_HEIGHT, HEADER_TOP_HEIGHT, INFO_PANEL_PADDING, INFO_SCROLLBAR_ALLOWANCE,
    INFO_WIDTH_FRACTION, INFO_WIDTH_MAX, INFO_WIDTH_MAX_FRACTION, INFO_WIDTH_MIN, text_color,
};
use crate::analysis::fft::state::FftState;
use egui::{Color32, FontId, Pos2, Rect, Ui, Vec2};

#[derive(Debug, Clone)]
pub(super) struct FftLayout {
    pub total: Rect,
    pub header_top: Rect,
    pub header_main: Rect,
    pub spectrum: Rect,
    pub info: Rect,
}

pub(super) fn calculate_layout(available: Rect) -> FftLayout {
    let info_width = clamp_fft_info_pane_width(
        available,
        (available.width() * INFO_WIDTH_FRACTION).clamp(INFO_WIDTH_MIN, INFO_WIDTH_MAX),
    );
    calculate_layout_with_info_width(available, info_width)
}

pub(super) fn calculate_layout_with_info_width(available: Rect, info_width: f32) -> FftLayout {
    let total = available;
    let info_width = clamp_fft_info_pane_width(total, info_width);
    let total_header_height = (HEADER_TOP_HEIGHT + HEADER_MAIN_HEIGHT).min(total.height());
    let top_height = HEADER_TOP_HEIGHT.min(total_header_height);
    let main_height = (total_header_height - top_height).max(0.0);

    let header_top = Rect::from_min_size(total.min, Vec2::new(total.width(), top_height));
    let header_main = Rect::from_min_size(
        Pos2::new(total.min.x, header_top.max.y),
        Vec2::new(total.width(), main_height),
    );
    let content_top = header_main.max.y;

    let info = Rect::from_min_size(
        Pos2::new(total.max.x - info_width, content_top),
        Vec2::new(info_width, (total.max.y - content_top).max(0.0)),
    );

    let spectrum = Rect::from_min_max(
        Pos2::new(
            total.min.x + CHART_LEFT_PADDING,
            content_top + CHART_TOP_GAP,
        ),
        Pos2::new(
            info.min.x - CHART_RIGHT_PADDING,
            total.max.y - CHART_BOTTOM_PADDING,
        ),
    );

    FftLayout {
        total,
        header_top,
        header_main,
        spectrum,
        info,
    }
}

pub(super) fn fft_info_pane_width_bounds(total: Rect) -> (f32, f32) {
    let min = INFO_WIDTH_MIN;
    let max_by_fraction = (total.width() * INFO_WIDTH_MAX_FRACTION).max(min);
    let max_by_plot = (total.width() - CHART_LEFT_PADDING - FFT_MIN_PLOT_WIDTH).max(min);
    let max = max_by_fraction
        .min(max_by_plot)
        .min(INFO_WIDTH_MAX)
        .max(min);
    (min, max)
}

pub(super) fn clamp_fft_info_pane_width(total: Rect, width: f32) -> f32 {
    let (min, max) = fft_info_pane_width_bounds(total);
    width.clamp(min, max)
}

pub(super) fn resolve_fft_info_pane_width(
    total: Rect,
    manual_width: Option<f32>,
    auto_width: f32,
) -> f32 {
    let base = (total.width() * INFO_WIDTH_FRACTION).clamp(INFO_WIDTH_MIN, INFO_WIDTH_MAX);
    let desired = manual_width.unwrap_or_else(|| base.max(auto_width));
    clamp_fft_info_pane_width(total, desired)
}

pub(super) fn spectrum_plot_rect(spectrum_rect: Rect) -> Rect {
    let min_x = (spectrum_rect.min.x + AXIS_LEFT_GUTTER).min(spectrum_rect.max.x - 1.0);
    let max_x = (spectrum_rect.max.x - AXIS_RIGHT_GUTTER).max(min_x + 1.0);
    let min_y = (spectrum_rect.min.y + AXIS_TOP_GUTTER).min(spectrum_rect.max.y - 1.0);
    let max_y = (spectrum_rect.max.y - AXIS_BOTTOM_GUTTER).max(min_y + 1.0);
    Rect::from_min_max(Pos2::new(min_x, min_y), Pos2::new(max_x, max_y))
}

pub(super) fn x_axis_title_position(spectrum_rect: Rect, plot_rect: Rect) -> Pos2 {
    Pos2::new(
        plot_rect.center().x,
        spectrum_rect.max.y - AXIS_TITLE_BOTTOM_INSET,
    )
}

pub(super) fn y_axis_title_position(
    spectrum_rect: Rect,
    plot_rect: Rect,
    max_y_tick_label_width: f32,
    y_title_width: f32,
) -> Pos2 {
    let y_tick_anchor_x = y_tick_label_position(plot_rect.center().y, plot_rect).x;
    let y_tick_left_edge = y_tick_anchor_x - max_y_tick_label_width.max(0.0);
    let title_left = (y_tick_left_edge - AXIS_TITLE_TO_VALUE_LABEL_GAP - y_title_width)
        .max(spectrum_rect.min.x + AXIS_TITLE_MIN_LEFT_INSET);
    Pos2::new(title_left, plot_rect.center().y)
}

pub(super) fn info_content_rect(layout: &FftLayout) -> Rect {
    let lane = Rect::from_min_max(
        Pos2::new(layout.spectrum.max.x, layout.info.min.y),
        layout.info.max,
    );
    lane.shrink(INFO_PANEL_PADDING)
}

pub(super) fn info_outline_rect(layout: &FftLayout) -> Option<Rect> {
    let top = (layout.info.min.y + AXIS_TOP_GUTTER).min(layout.info.max.y);
    if top >= layout.info.max.y {
        return None;
    }
    Some(Rect::from_min_max(
        Pos2::new(layout.info.min.x, top),
        layout.info.max,
    ))
}

pub(super) fn x_tick_label_position(x: f32, plot_rect: Rect) -> Pos2 {
    Pos2::new(x, plot_rect.max.y + AXIS_TICK_Y_OFFSET)
}

pub(super) fn y_tick_label_position(y: f32, plot_rect: Rect) -> Pos2 {
    Pos2::new(plot_rect.min.x - AXIS_TICK_X_OFFSET, y)
}

pub(super) fn measure_text_width(
    painter: &egui::Painter,
    text: &str,
    font: FontId,
    color: Color32,
) -> f32 {
    painter
        .layout_no_wrap(text.to_owned(), font, color)
        .size()
        .x
}

pub(super) fn measure_text_size(
    painter: &egui::Painter,
    text: &str,
    font: FontId,
    color: Color32,
) -> Vec2 {
    painter.layout_no_wrap(text.to_owned(), font, color).size()
}

pub(super) fn combo_width_from_texts<'a, I>(
    ui: &Ui,
    selected_text: &str,
    options: I,
    min_width: f32,
    max_width: f32,
) -> f32
where
    I: IntoIterator<Item = &'a str>,
{
    let painter = ui.painter();
    let font = FontId::proportional(12.0);
    let color = text_color();

    let mut max_text_width = measure_text_width(painter, selected_text, font.clone(), color);
    for option in options {
        let width = measure_text_width(painter, option, font.clone(), color);
        max_text_width = max_text_width.max(width);
    }

    (max_text_width + HEADER_DROPDOWN_TEXT_PADDING).clamp(min_width, max_width)
}

pub(super) fn fft_button_width_for_text(
    painter: &egui::Painter,
    text: &str,
    font: FontId,
    color: Color32,
) -> f32 {
    measure_text_width(painter, text, font, color) + 16.0
}

pub(super) fn preferred_fft_info_pane_width(ui: &Ui, state: &FftState) -> f32 {
    let painter = ui.painter();
    let label_font = FontId::proportional(10.0);
    let value_font = FontId::proportional(11.0);
    let small_font = FontId::proportional(9.0);
    let label_color = Color32::from_rgb(120, 125, 135);
    let value_color = Color32::from_rgb(200, 205, 215);

    const INFO_LABELS: &[&str] = &[
        "Fund.",
        "Level",
        "THD",
        "SFDR",
        "SNR",
        "SINAD",
        "Noise",
        "Harmonics",
        "Type",
        "Norm",
        "Fidelity",
        "Tstart",
        "Tstop",
        "N set",
        "Sidelobe",
        "ENBW",
        "Trace",
        "Input N",
        "Samples",
        "Decim",
        "Fs",
    ];

    let mut max_label_width = 0.0f32;
    for label in INFO_LABELS {
        let width = measure_text_width(
            painter,
            &format!("{}:", label),
            label_font.clone(),
            label_color,
        );
        max_label_width = max_label_width.max(width);
    }

    let mut value_samples: Vec<String> = vec![
        state.window.display_name().to_owned(),
        state.normalization.display_name().to_owned(),
        state.input_fidelity.display_name().to_owned(),
        format!("{:.0} dB", state.window.sidelobe_level()),
        format!("{:.2} bins", state.window.noise_bandwidth()),
        format!("{}", state.num_harmonics),
        "--".to_string(),
    ];

    if !state.time_window_auto {
        value_samples.push(crate::waveform::axis::format_time(state.time_window_start));
        value_samples.push(crate::waveform::axis::format_time(state.time_window_end));
    }
    if !state.sample_count_auto {
        value_samples.push(format!("{}", state.sample_count));
    }
    if let Some(ref analysis) = state.analysis {
        if let Some(v) = analysis.fundamental_frequency {
            value_samples.push(super::format_freq(v));
        }
        if let Some(v) = analysis.fundamental_db {
            value_samples.push(format!("{:.1} dB", v));
        }
        if let Some(v) = analysis.thd_percent {
            value_samples.push(format!("{:.3}%", v));
        }
        if let Some(v) = analysis.sfdr_db {
            value_samples.push(format!("{:.1} dB", v));
        }
        if let Some(v) = analysis.snr_db {
            value_samples.push(format!("{:.1} dB", v));
        }
        if let Some(v) = analysis.sinad_db {
            value_samples.push(format!("{:.1} dB", v));
        }
        if let Some(v) = analysis.noise_floor_db {
            value_samples.push(format!("{:.1} dB", v));
        }
        value_samples.push(format!("{}", analysis.harmonics.len()));
    }
    if let Some(ref source) = state.source_cache {
        value_samples.push(source.name.clone());
        value_samples.push(format!("{}", source.original_count));
        value_samples.push(format!("{}", source.samples.len()));
        if source.decimation_factor > 1 {
            value_samples.push(format!("x{}", source.decimation_factor));
        }
        value_samples.push(super::format_freq(source.sample_rate));
    }

    let mut max_value_width = 0.0f32;
    for value in &value_samples {
        let width = measure_text_width(painter, value, value_font.clone(), value_color);
        max_value_width = max_value_width.max(width);
    }
    let info_rows_width = max_label_width + 8.0 + max_value_width;

    let markers_hint = measure_text_width(
        painter,
        "Alt+LMB add, Alt+RMB remove",
        small_font.clone(),
        label_color,
    );
    let marker_freq_text = state
        .marker_frequencies
        .iter()
        .copied()
        .last()
        .map(super::format_freq)
        .unwrap_or_else(|| "1.00 kHz".to_string());
    let marker_row_width = measure_text_width(painter, "M16", value_font.clone(), value_color)
        + 4.0
        + fft_button_width_for_text(painter, &marker_freq_text, value_font.clone(), value_color)
        + 4.0
        + fft_button_width_for_text(painter, "x", value_font, value_color);

    let content_width = info_rows_width.max(markers_hint).max(marker_row_width);
    content_width + INFO_PANEL_PADDING * 2.0 + INFO_SCROLLBAR_ALLOWANCE
}

pub(super) fn next_fft_info_pane_width(
    current_width: Option<f32>,
    fallback_layout_width: f32,
    drag_delta_x: f32,
    total: Rect,
) -> f32 {
    let base = current_width.unwrap_or(fallback_layout_width);
    clamp_fft_info_pane_width(total, base - drag_delta_x)
}
