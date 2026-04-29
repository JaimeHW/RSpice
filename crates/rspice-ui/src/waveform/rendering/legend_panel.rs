use super::formatting::{
    format_optional_freq, format_optional_time, format_optional_value, truncate_legend_trace_name,
};
use super::*;

// =============================================================================
// Legend Rendering
// =============================================================================

pub(super) fn render_legend(
    ui: &mut Ui,
    layout: &ViewerLayout,
    viewer_state: &mut WaveformViewerState,
) {
    let painter = ui.painter();

    // Background
    painter.rect_filled(layout.legend, Rounding::ZERO, Color32::from_rgb(30, 33, 40));

    // Match plot chrome so panel/graph top and edges align visually.
    painter.rect_stroke(
        layout.legend,
        Rounding::ZERO,
        Stroke::new(1.0, plot_border_color()),
    );

    // Create UI area for legend items
    let legend_inner = legend_inner_rect(layout.legend);
    ui.allocate_new_ui(UiBuilder::new().max_rect(legend_inner), |ui| {
        ui.set_min_width(legend_inner.width());
        ui.set_max_width(legend_inner.width());
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .id_salt("waveform_legend_scroll")
            .show(ui, |ui| {
                let content_width = ui.available_width().max(0.0);
                ui.set_min_width(content_width);
                ui.set_max_width(content_width);
                render_trace_list_section(ui, viewer_state);
                ui.add_space(LEGEND_SECTION_SPACING);
                render_markers_panel(ui, viewer_state);
                if viewer_state.show_measurements {
                    ui.add_space(LEGEND_SECTION_SPACING);
                    render_measurements_panel(ui, viewer_state);
                }
                if viewer_state.show_export {
                    ui.add_space(LEGEND_SECTION_SPACING);
                    render_export_panel(ui, viewer_state);
                }
            });
    });
}

fn legend_inner_rect(legend_rect: Rect) -> Rect {
    legend_rect.shrink2(Vec2::new(LEGEND_INSET_X, LEGEND_INSET_Y))
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LegendTraceRowLayout {
    show_swatch: bool,
    show_solo: bool,
    name_width: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LegendFindRowLayout {
    show_clear: bool,
    edit_width: f32,
}

fn legend_row_rect(row_left: f32, row_top: f32, row_width: f32) -> Rect {
    Rect::from_min_size(
        Pos2::new(row_left, row_top),
        Vec2::new(row_width.max(0.0), LEGEND_ROW_HEIGHT),
    )
}

fn calculate_legend_trace_row_layout(row_width: f32, item_spacing_x: f32) -> LegendTraceRowLayout {
    let available = row_width.max(0.0);
    let show_swatch = available >= LEGEND_TRACE_SHOW_SWATCH_MIN_WIDTH;
    let show_solo = available >= LEGEND_TRACE_SHOW_SOLO_MIN_WIDTH;

    let swatch_width = if show_swatch {
        LEGEND_TRACE_SWATCH_WIDTH + item_spacing_x
    } else {
        0.0
    };
    let solo_width = if show_solo {
        LEGEND_TRACE_SOLO_WIDTH + item_spacing_x
    } else {
        0.0
    };
    let fixed_width = swatch_width + LEGEND_TRACE_CONTROL_WIDTH + item_spacing_x + solo_width;
    let name_width = (available - fixed_width).max(LEGEND_TRACE_LABEL_MIN_WIDTH);

    LegendTraceRowLayout {
        show_swatch,
        show_solo,
        name_width,
    }
}

fn calculate_legend_find_row_layout(row_width: f32, item_spacing_x: f32) -> LegendFindRowLayout {
    let available = row_width.max(0.0);
    let required_for_clear = LEGEND_FIND_EDIT_MIN_WIDTH + LEGEND_TRACE_SOLO_WIDTH + item_spacing_x;
    let show_clear = available >= required_for_clear;
    let edit_width = if show_clear {
        (available - LEGEND_TRACE_SOLO_WIDTH - item_spacing_x - LEGEND_FIND_RIGHT_GUARD).max(0.0)
    } else {
        available
    };
    LegendFindRowLayout {
        show_clear,
        edit_width,
    }
}

fn active_solo_trace_index(traces: &[TraceData]) -> Option<usize> {
    let mut solo: Option<usize> = None;
    for (idx, trace) in traces.iter().enumerate() {
        if !trace.visible {
            continue;
        }
        if solo.is_some() {
            return None;
        }
        solo = Some(idx);
    }
    solo
}

fn render_solo_control(ui: &mut Ui, rect: Rect, is_active: bool) -> Response {
    ui.put(rect, egui::RadioButton::new(is_active, ""))
}

fn render_legend_control_label(ui: &mut Ui, text: &str) {
    ui.allocate_ui_with_layout(
        Vec2::new(LEGEND_CONTROL_LABEL_WIDTH, LEGEND_ROW_HEIGHT),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.label(
                egui::RichText::new(text)
                    .size(9.0)
                    .color(Color32::from_rgb(120, 125, 135)),
            );
        },
    );
}

fn render_trace_list_section(ui: &mut Ui, viewer_state: &mut WaveformViewerState) {
    ui.spacing_mut().item_spacing = Vec2::new(4.0, 4.0);
    ui.spacing_mut().interact_size.y = LEGEND_ROW_HEIGHT;

    ui.label(
        egui::RichText::new("Traces")
            .size(11.0)
            .strong()
            .color(Color32::from_rgb(160, 165, 175)),
    );
    ui.horizontal(|ui| {
        render_legend_control_label(ui, "Show");
        if ui.small_button("All").clicked() {
            legend::show_all_traces(&mut viewer_state.traces);
        }
        if ui.small_button("Clear").clicked() {
            legend::hide_all_traces(&mut viewer_state.traces);
        }
    });

    ui.horizontal(|ui| {
        render_legend_control_label(ui, "Sort");
        let combo_width = ui.available_width().clamp(60.0, 140.0);
        egui::ComboBox::from_id_salt("waveform_legend_sort")
            .selected_text(match viewer_state.legend_state.sort_by {
                LegendSortOrder::Index => "Index",
                LegendSortOrder::Name => "Name",
                LegendSortOrder::Visibility => "Visible",
            })
            .width(combo_width)
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut viewer_state.legend_state.sort_by,
                    LegendSortOrder::Index,
                    "Index",
                );
                ui.selectable_value(
                    &mut viewer_state.legend_state.sort_by,
                    LegendSortOrder::Name,
                    "Name",
                );
                ui.selectable_value(
                    &mut viewer_state.legend_state.sort_by,
                    LegendSortOrder::Visibility,
                    "Visible",
                );
            });
    });

    ui.horizontal(|ui| {
        render_legend_control_label(ui, "Find");
        let find_layout =
            calculate_legend_find_row_layout(ui.available_width(), ui.spacing().item_spacing.x);
        let edit_rect = ui
            .allocate_space(Vec2::new(find_layout.edit_width, LEGEND_ROW_HEIGHT))
            .1;
        ui.put(
            edit_rect,
            egui::TextEdit::singleline(&mut viewer_state.legend_state.filter).hint_text("trace"),
        );
        if find_layout.show_clear {
            let clear_rect = ui
                .allocate_space(Vec2::new(LEGEND_TRACE_SOLO_WIDTH, edit_rect.height()))
                .1;
            let clear_clicked = ui
                .scope(|ui| {
                    ui.spacing_mut().button_padding = Vec2::ZERO;
                    ui.put(clear_rect, egui::Button::new("x")).clicked()
                })
                .inner;
            if clear_clicked {
                viewer_state.legend_state.clear_filter();
            }
            ui.add_space(LEGEND_FIND_RIGHT_GUARD);
        }
    });
    ui.add_space(4.0);

    let items = legend::build_legend_items(&viewer_state.traces, &viewer_state.legend_state);
    let mut visibility_updates: Vec<(usize, bool)> = Vec::new();
    let mut solo_trace_idx: Option<usize> = None;
    let mut selected_trace_name: Option<String> = None;
    let solo_active_idx = active_solo_trace_index(&viewer_state.traces);
    let trace_rows_width = ui.max_rect().width().max(0.0);
    let trace_rows_left = ui.max_rect().min.x;
    let item_spacing_x = ui.spacing().item_spacing.x;

    for item in &items {
        let color = Color32::from_rgba_unmultiplied(
            item.color[0],
            item.color[1],
            item.color[2],
            item.color[3],
        );
        let selected = viewer_state
            .selected_trace
            .as_deref()
            .is_some_and(|name| name == item.name);

        let row_top = ui.cursor().min.y;
        let row_rect = legend_row_rect(trace_rows_left, row_top, trace_rows_width);
        ui.allocate_rect(row_rect, Sense::hover());
        let row_layout = calculate_legend_trace_row_layout(trace_rows_width, item_spacing_x);
        let mut left = row_rect.min.x;

        if row_layout.show_swatch {
            let swatch_rect = Rect::from_center_size(
                Pos2::new(
                    left + (LEGEND_TRACE_SWATCH_WIDTH * 0.5),
                    row_rect.center().y,
                ),
                Vec2::new(LEGEND_TRACE_SWATCH_WIDTH, LEGEND_TRACE_SWATCH_WIDTH),
            );
            if item.visible {
                ui.painter()
                    .rect_filled(swatch_rect, Rounding::same(2.0), color);
            } else {
                ui.painter()
                    .rect_stroke(swatch_rect, Rounding::same(2.0), Stroke::new(1.0, color));
            }
            left += LEGEND_TRACE_SWATCH_WIDTH + item_spacing_x;
        }

        let checkbox_rect = Rect::from_min_size(
            Pos2::new(left, row_rect.min.y),
            Vec2::new(LEGEND_TRACE_CONTROL_WIDTH, LEGEND_ROW_HEIGHT),
        );
        let mut visible = item.visible;
        if ui
            .put(checkbox_rect, egui::Checkbox::without_text(&mut visible))
            .changed()
        {
            visibility_updates.push((item.index, visible));
        }
        left += LEGEND_TRACE_CONTROL_WIDTH + item_spacing_x;

        if row_layout.show_solo {
            let solo_rect = Rect::from_min_size(
                Pos2::new(left, row_rect.min.y),
                Vec2::new(LEGEND_TRACE_SOLO_WIDTH, LEGEND_ROW_HEIGHT),
            );
            let is_active = solo_active_idx == Some(item.index);
            if render_solo_control(ui, solo_rect, is_active)
                .on_hover_text("Solo trace")
                .clicked()
            {
                solo_trace_idx = Some(item.index);
                selected_trace_name = Some(item.name.clone());
            }
            left += LEGEND_TRACE_SOLO_WIDTH + item_spacing_x;
        }

        let name_slot_width = (row_rect.max.x - left).max(LEGEND_TRACE_LABEL_MIN_WIDTH);
        let name_rect = Rect::from_min_size(
            Pos2::new(left, row_rect.min.y),
            Vec2::new(name_slot_width, LEGEND_ROW_HEIGHT),
        );
        let text_color = if item.visible {
            Color32::from_rgb(200, 205, 215)
        } else {
            Color32::from_rgb(110, 115, 125)
        };
        let display_name = truncate_legend_trace_name(
            ui.painter(),
            &item.name,
            FontId::proportional(10.0),
            (name_slot_width - LEGEND_TEXT_TRUNCATION_PADDING).max(0.0),
        );
        let label = egui::RichText::new(&display_name)
            .size(10.0)
            .color(text_color);
        let label_response = ui
            .allocate_new_ui(UiBuilder::new().max_rect(name_rect), |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.selectable_label(selected, label)
                })
                .inner
            })
            .inner;
        let label_response = if display_name != item.name {
            label_response.on_hover_text(&item.name)
        } else {
            label_response
        };
        if label_response.clicked() {
            selected_trace_name = Some(item.name.clone());
        }
    }

    for (idx, visible) in visibility_updates {
        if let Some(trace) = viewer_state.traces.get_mut(idx) {
            trace.visible = visible;
        }
    }
    if let Some(idx) = solo_trace_idx {
        legend::solo_trace(&mut viewer_state.traces, idx);
    }
    apply_legend_selection(viewer_state, selected_trace_name);

    if items.is_empty() {
        ui.label(
            egui::RichText::new("No traces in filter")
                .size(10.0)
                .color(Color32::from_rgb(100, 105, 115)),
        );
    }
}

pub(super) fn handle_waveform_right_pane_splitter(
    ui: &mut Ui,
    layout: &ViewerLayout,
    viewer_state: &mut WaveformViewerState,
) {
    let half_hit = LEGEND_SPLITTER_HIT_WIDTH * 0.5;
    let splitter_rect = Rect::from_min_max(
        Pos2::new(layout.legend.min.x - half_hit, layout.legend.min.y),
        Pos2::new(layout.legend.min.x + half_hit, layout.legend.max.y),
    );

    let splitter_id = ui.id().with("waveform_right_pane_splitter");
    let mut response = ui.interact(splitter_rect, splitter_id, Sense::click_and_drag());
    response = response.on_hover_cursor(CursorIcon::ResizeHorizontal);

    if response.double_clicked() {
        viewer_state.right_pane_width = None;
    }

    if response.dragged() {
        let delta_x = ui.ctx().input(|i| i.pointer.delta().x);
        let next = next_waveform_right_pane_width(
            viewer_state.right_pane_width,
            layout.legend.width(),
            delta_x,
            layout.total,
        );
        viewer_state.right_pane_width = Some(next);
    }

    let stroke_color = if response.dragged() {
        Color32::from_rgb(115, 150, 220)
    } else if response.hovered() {
        Color32::from_rgb(90, 115, 165)
    } else {
        plot_border_color()
    };
    ui.painter().line_segment(
        [
            Pos2::new(layout.legend.min.x, layout.legend.min.y),
            Pos2::new(layout.legend.min.x, layout.legend.max.y),
        ],
        Stroke::new(LEGEND_SPLITTER_STROKE_WIDTH, stroke_color),
    );
}

fn render_markers_panel(ui: &mut Ui, viewer_state: &mut WaveformViewerState) {
    ui.separator();
    ui.label(
        egui::RichText::new("Markers")
            .size(11.0)
            .strong()
            .color(Color32::from_rgb(160, 165, 175)),
    );

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!("{}", viewer_state.markers.len()))
                .size(9.0)
                .color(Color32::from_rgb(120, 125, 135)),
        );
        ui.label(
            egui::RichText::new("entries")
                .size(9.0)
                .color(Color32::from_rgb(120, 125, 135)),
        );
        if ui.small_button("Clear").clicked() {
            viewer_state.clear_markers();
        }
    });
    ui.label(
        egui::RichText::new("Alt+LMB add, Alt+RMB remove")
            .size(9.0)
            .color(Color32::from_rgb(120, 125, 135)),
    );

    if viewer_state.markers.is_empty() {
        ui.label(
            egui::RichText::new("No markers")
                .size(10.0)
                .color(Color32::from_rgb(100, 105, 115)),
        );
        return;
    }

    let mut jump_to: Option<f64> = None;
    let mut remove_idx: Option<usize> = None;
    let markers: Vec<f64> = viewer_state.markers.clone();
    for (idx, marker_x) in markers.iter().copied().enumerate() {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("M{}", idx + 1))
                    .size(10.0)
                    .color(marker_color(idx)),
            );
            if ui
                .small_button(axis::format_time(marker_x))
                .on_hover_text("Center X view on marker")
                .clicked()
            {
                jump_to = Some(marker_x);
            }
            if ui
                .small_button("x")
                .on_hover_text("Delete marker")
                .clicked()
            {
                remove_idx = Some(idx);
            }
        });
    }

    if let Some(idx) = remove_idx {
        viewer_state.remove_marker_at(idx);
    }
    if let Some(marker_x) = jump_to {
        center_waveform_view_x_on_marker(
            &mut viewer_state.view,
            &viewer_state.data_bounds,
            marker_x,
        );
    }
}

fn apply_legend_selection(
    viewer_state: &mut WaveformViewerState,
    selected_trace_name: Option<String>,
) {
    if let Some(name) = selected_trace_name {
        viewer_state.selected_trace = Some(name.clone());
        viewer_state.clear_highlights();
        viewer_state.set_trace_highlight(&name, true);
    }
}

fn measurement_cursor_range(viewer_state: &WaveformViewerState) -> Option<(f64, f64)> {
    if !viewer_state.measurement_use_cursor_range {
        return None;
    }
    let (Some(c1), Some(c2)) = (
        viewer_state.cursors.cursor1_x,
        viewer_state.cursors.cursor2_x,
    ) else {
        return None;
    };
    Some((c1.min(c2), c1.max(c2)))
}

fn measurement_trace_indices(viewer_state: &WaveformViewerState) -> Vec<usize> {
    match viewer_state.measurement_scope {
        MeasurementScope::Selected => {
            let Some(selected) = viewer_state.selected_trace.as_deref() else {
                return Vec::new();
            };
            viewer_state
                .traces
                .iter()
                .enumerate()
                .find_map(|(idx, trace)| (trace.name == selected).then_some(idx))
                .into_iter()
                .collect()
        }
        MeasurementScope::Visible => viewer_state
            .traces
            .iter()
            .enumerate()
            .filter_map(|(idx, trace)| trace.visible.then_some(idx))
            .collect(),
        MeasurementScope::All => (0..viewer_state.traces.len()).collect(),
    }
}

fn measurement_row(ui: &mut Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!("{}:", label))
                .size(9.0)
                .color(Color32::from_rgb(120, 125, 135)),
        );
        ui.label(
            egui::RichText::new(value)
                .size(10.0)
                .color(Color32::from_rgb(200, 205, 215)),
        );
    });
}

fn render_trace_measurements(
    ui: &mut Ui,
    trace: &TraceData,
    measurements: &TraceMeasurements,
    y_unit: &str,
    x_unit: &str,
) {
    ui.label(
        egui::RichText::new(&trace.name)
            .size(10.0)
            .strong()
            .color(trace.style.to_color32()),
    );
    measurement_row(ui, "Min", &format_optional_value(measurements.min, y_unit));
    measurement_row(ui, "Max", &format_optional_value(measurements.max, y_unit));
    measurement_row(
        ui,
        "PkPk",
        &format_optional_value(measurements.pk_pk, y_unit),
    );
    measurement_row(
        ui,
        "Mean",
        &format_optional_value(measurements.mean, y_unit),
    );
    measurement_row(ui, "RMS", &format_optional_value(measurements.rms, y_unit));
    measurement_row(
        ui,
        "Std",
        &format_optional_value(measurements.std_dev, y_unit),
    );
    measurement_row(ui, "Rise", &format_optional_time(measurements.rise_time));
    measurement_row(ui, "Fall", &format_optional_time(measurements.fall_time));
    measurement_row(ui, "Period", &format_optional_time(measurements.period));
    measurement_row(ui, "Freq", &format_optional_freq(measurements.frequency));
    measurement_row(
        ui,
        "Duty",
        &measurements
            .duty_cycle
            .map(|v| format!("{:.2}%", v))
            .unwrap_or_else(|| "--".to_string()),
    );
    measurement_row(
        ui,
        "Integral",
        &format_optional_value(measurements.integral, &format!("{}*{}", y_unit, x_unit)),
    );
}

fn render_measurements_panel(ui: &mut Ui, viewer_state: &mut WaveformViewerState) {
    ui.separator();
    ui.label(
        egui::RichText::new("Measurements")
            .size(11.0)
            .strong()
            .color(Color32::from_rgb(160, 165, 175)),
    );
    ui.add_space(4.0);

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("Scope")
                .size(9.0)
                .color(Color32::from_rgb(120, 125, 135)),
        );
        egui::ComboBox::from_id_salt("waveform_measure_scope")
            .selected_text(viewer_state.measurement_scope.display_name())
            .width(80.0)
            .show_ui(ui, |ui| {
                for scope in MeasurementScope::all() {
                    ui.selectable_value(
                        &mut viewer_state.measurement_scope,
                        *scope,
                        scope.display_name(),
                    );
                }
            });
        ui.checkbox(
            &mut viewer_state.measurement_use_cursor_range,
            "Cursor range",
        );
    });

    if let Some((start, end)) = measurement_cursor_range(viewer_state) {
        measurement_row(
            ui,
            "Range",
            &format!("{} - {}", axis::format_time(start), axis::format_time(end)),
        );
    }

    let trace_indices = measurement_trace_indices(viewer_state);
    if trace_indices.is_empty() {
        ui.label(
            egui::RichText::new("No traces in selected scope")
                .size(10.0)
                .color(Color32::from_rgb(100, 105, 115)),
        );
        return;
    }

    let y_unit = if viewer_state.y_axis_unit.is_empty() {
        "V"
    } else {
        viewer_state.y_axis_unit.as_str()
    };
    let x_unit = if viewer_state.x_axis_unit.is_empty() {
        "s"
    } else {
        viewer_state.x_axis_unit.as_str()
    };

    let cursor_range = measurement_cursor_range(viewer_state);
    let traces = &viewer_state.traces;
    let measurement_cache = &mut viewer_state.measurement_cache;
    measurement_cache.truncate_to_trace_count(traces.len());
    for (idx, trace_idx) in trace_indices.iter().enumerate() {
        if let Some(trace) = traces.get(*trace_idx) {
            if idx > 0 {
                ui.add_space(6.0);
            }
            let measurements = measurement_cache.get_or_compute(*trace_idx, trace, cursor_range);
            render_trace_measurements(ui, trace, measurements, y_unit, x_unit);
        }
    }
}

fn render_export_panel(ui: &mut Ui, viewer_state: &mut WaveformViewerState) {
    ui.separator();
    ui.label(
        egui::RichText::new("Export")
            .size(11.0)
            .strong()
            .color(Color32::from_rgb(160, 165, 175)),
    );
    ui.add_space(4.0);

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("Format")
                .size(9.0)
                .color(Color32::from_rgb(120, 125, 135)),
        );
        egui::ComboBox::from_id_salt("waveform_export_format")
            .selected_text(export_format_display_name(
                viewer_state.export_options.format,
            ))
            .width(96.0)
            .show_ui(ui, |ui| {
                for format in [ExportFormat::Csv, ExportFormat::Tsv, ExportFormat::SpiceRaw] {
                    ui.selectable_value(
                        &mut viewer_state.export_options.format,
                        format,
                        export_format_display_name(format),
                    );
                }
            });
    });

    ui.horizontal(|ui| {
        ui.checkbox(&mut viewer_state.export_options.include_header, "Header");
        ui.checkbox(&mut viewer_state.export_options.include_hidden, "Hidden");
        ui.checkbox(
            &mut viewer_state.export_options.scientific_notation,
            "Scientific",
        );
    });

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("Precision")
                .size(9.0)
                .color(Color32::from_rgb(120, 125, 135)),
        );
        ui.add(
            egui::DragValue::new(&mut viewer_state.export_options.precision)
                .range(1..=15)
                .speed(1.0),
        );
    });

    let mut use_start = viewer_state.export_options.x_start.is_some();
    let mut use_end = viewer_state.export_options.x_end.is_some();
    ui.horizontal(|ui| {
        if ui.checkbox(&mut use_start, "Start").changed() {
            viewer_state.export_options.x_start = if use_start {
                Some(viewer_state.view.x_min)
            } else {
                None
            };
        }
        if use_start {
            let mut value = viewer_state
                .export_options
                .x_start
                .unwrap_or(viewer_state.view.x_min);
            if ui
                .add(egui::DragValue::new(&mut value).speed(1e-9))
                .changed()
            {
                viewer_state.export_options.x_start = Some(value);
            }
        }
    });
    ui.horizontal(|ui| {
        if ui.checkbox(&mut use_end, "End").changed() {
            viewer_state.export_options.x_end = if use_end {
                Some(viewer_state.view.x_max)
            } else {
                None
            };
        }
        if use_end {
            let mut value = viewer_state
                .export_options
                .x_end
                .unwrap_or(viewer_state.view.x_max);
            if ui
                .add(egui::DragValue::new(&mut value).speed(1e-9))
                .changed()
            {
                viewer_state.export_options.x_end = Some(value);
            }
        }
    });

    if let (Some(start), Some(end)) = (
        viewer_state.export_options.x_start,
        viewer_state.export_options.x_end,
    ) && end < start
    {
        viewer_state.export_options.x_end = Some(start);
    }

    let stats = calculate_export_stats(&viewer_state.traces, &viewer_state.export_options);
    measurement_row(ui, "Traces", &format!("{}", stats.num_traces));
    measurement_row(ui, "Points", &format!("{}", stats.num_points));
    measurement_row(
        ui,
        "Est Size",
        &axis::format_with_si_prefix(stats.estimated_size as f64, "B", 2),
    );

    ui.horizontal(|ui| {
        if ui.button("Copy").clicked() {
            let payload = build_export_payload(&viewer_state.traces, &viewer_state.export_options);
            ui.ctx().copy_text(payload.clone());
            viewer_state.export_status = Some(format!("Copied {} bytes", payload.len()));
        }
        if ui.button("Save...").clicked() {
            let payload = build_export_payload(&viewer_state.traces, &viewer_state.export_options);
            viewer_state.export_status = match save_export_payload_with_native_dialog(
                &payload,
                viewer_state.export_options.format,
            ) {
                Ok(path) => Some(format!("Saved {}", path.display())),
                Err(err) => Some(err),
            };
        }
    });

    if let Some(status) = viewer_state.export_status.as_deref() {
        ui.label(
            egui::RichText::new(status)
                .size(9.0)
                .color(Color32::from_rgb(130, 180, 220)),
        );
    }
}

// =============================================================================
// Tests
// =============================================================================
