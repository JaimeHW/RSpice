use super::super::formatting::truncate_legend_trace_name;
use super::super::*;

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

pub(super) fn render_trace_list_section(ui: &mut Ui, viewer_state: &mut WaveformViewerState) {
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
