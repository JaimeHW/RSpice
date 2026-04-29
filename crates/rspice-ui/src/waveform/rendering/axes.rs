use super::*;

// =============================================================================
// Y-Axis Rendering
// =============================================================================

pub(super) fn render_y_axis(
    ui: &mut Ui,
    layout: &ViewerLayout,
    viewer_state: &WaveformViewerState,
) {
    let painter = ui.painter();

    // Background
    painter.rect_filled(layout.y_axis, Rounding::ZERO, Color32::from_rgb(25, 27, 33));

    // Calculate ticks
    let y_ticks = axis::calculate_ticks(viewer_state.view.y_min, viewer_state.view.y_max, 6);

    // Generate tick labels
    let labels = axis::generate_tick_labels(&y_ticks.major_ticks, y_ticks.scale, y_ticks.precision);

    // Render tick labels (numeric values) - properly aligned with plot grid
    let font = FontId::proportional(10.0);
    let text_color = Color32::from_rgb(160, 165, 175);
    let mut max_y_tick_label_width = 0.0f32;

    for (i, &tick) in y_ticks.major_ticks.iter().enumerate() {
        let y_frac = (viewer_state.view.y_max - tick) / viewer_state.view.y_range();
        let screen_y = layout.y_axis.min.y + y_frac as f32 * layout.y_axis.height();

        if let Some(label) = labels.get(i) {
            let width = measure_text_width(painter, label, font.clone(), text_color);
            max_y_tick_label_width = max_y_tick_label_width.max(width);
            let text_pos = y_tick_label_position(layout, screen_y);
            painter.text(
                text_pos,
                egui::Align2::RIGHT_CENTER,
                label,
                font.clone(),
                text_color,
            );
        }
    }

    // Y-axis title in outer lane (farther from tick values, matching FFT layout model).
    let unit_label = y_axis_title_text(viewer_state, y_ticks.prefix);
    let title_font = FontId::proportional(10.0);
    let title_color = Color32::from_rgb(130, 135, 145);
    let title_width = measure_text_width(painter, &unit_label, title_font.clone(), title_color);
    painter.text(
        y_axis_title_position(layout, max_y_tick_label_width, title_width),
        egui::Align2::LEFT_CENTER,
        &unit_label,
        title_font,
        title_color,
    );
}

// =============================================================================
// X-Axis Rendering
// =============================================================================

pub(super) fn render_x_axis(
    ui: &mut Ui,
    layout: &ViewerLayout,
    viewer_state: &WaveformViewerState,
) {
    let painter = ui.painter();

    // Background
    painter.rect_filled(layout.x_axis, Rounding::ZERO, Color32::from_rgb(25, 27, 33));

    // Calculate ticks
    let x_ticks = axis::calculate_ticks(viewer_state.view.x_min, viewer_state.view.x_max, 8);

    let labels = axis::generate_tick_labels(&x_ticks.major_ticks, x_ticks.scale, x_ticks.precision);

    // Render labels
    let font = FontId::proportional(10.0);
    let text_color = Color32::from_rgb(160, 165, 175);

    for (i, &tick) in x_ticks.major_ticks.iter().enumerate() {
        let x_frac = (tick - viewer_state.view.x_min) / viewer_state.view.x_range();
        let screen_x = layout.x_axis.min.x + x_frac as f32 * layout.x_axis.width();

        if let Some(label) = labels.get(i) {
            let text_pos = x_tick_label_position(layout, screen_x);
            painter.text(
                text_pos,
                egui::Align2::CENTER_TOP,
                label,
                font.clone(),
                text_color,
            );
        }
    }

    // Unit label (analysis-aware: Time/Frequency/Voltage)
    let label = if viewer_state.x_axis_label.is_empty() {
        "Time"
    } else {
        &viewer_state.x_axis_label
    };
    let unit = if viewer_state.x_axis_unit.is_empty() {
        "s"
    } else {
        &viewer_state.x_axis_unit
    };
    let unit_label = format!("{} ({}{})", label, x_ticks.prefix, unit);
    painter.text(
        x_axis_title_position(layout),
        egui::Align2::CENTER_BOTTOM,
        unit_label,
        FontId::proportional(9.0),
        Color32::from_rgb(120, 125, 135),
    );
}
