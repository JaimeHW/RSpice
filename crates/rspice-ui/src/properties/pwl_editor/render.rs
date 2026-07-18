//! The PWL point table in design-system style: mono column headers, inset
//! mono cells, accent row selection, and a faint summary footer.

use egui::Ui;

use crate::quantity::{
    QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale, parse_ui_quantity,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, mono_input};

use super::data::{format_engineering_for_spice, format_spice_number_lossless};
use super::state::PwlEditorState;

/// Result of PWL editor interaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PwlEditorResult {
    /// No change.
    None,
    /// Data was modified.
    Modified,
}

/// Index column width.
const INDEX_COL: f32 = 30.0;
const RAW_SOURCE_ACCESSIBLE_LABEL: &str = "PWL source text";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PwlInputField {
    Time,
    Value,
}

/// Render the PWL editor widget.
///
/// Returns `PwlEditorResult::Modified` if the data changed.
pub fn render_pwl_editor(
    ui: &mut Ui,
    state: &mut PwlEditorState,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> PwlEditorResult {
    if state.raw_source_draft().is_some() {
        return render_raw_source_draft(ui, state);
    }

    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let mut result = PwlEditorResult::None;

    let compact = ui.available_width() < 330.0;
    // Action strip: the narrow mockup aside stacks its command row so every
    // action remains reachable instead of clipping past the pane edge.
    let render_kicker = |ui: &mut Ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        let mut kicker = egui::text::LayoutJob::default();
        kicker.append(
            "WAVEFORM POINTS",
            0.0,
            egui::TextFormat {
                font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
                color: c.text_faint,
                extra_letter_spacing: 0.08 * tokens::FS_0,
                ..Default::default()
            },
        );
        ui.label(kicker);
    };
    let mut render_actions = |ui: &mut Ui| {
        ui.horizontal(|ui| {
            if Button::new("+ Add point").ghost().show(ui).clicked() {
                state.adding_point = true;
            }
            if state.selected_row.is_some() && Button::new("Remove").ghost().show(ui).clicked() {
                state.delete_selected();
                result = PwlEditorResult::Modified;
            }
        });
    };
    if compact {
        render_kicker(ui);
        ui.add_space(4.0);
        render_actions(ui);
    } else {
        ui.horizontal(|ui| {
            render_kicker(ui);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                render_actions(ui);
            });
        });
    }

    if state.adding_point {
        render_new_point_row(ui, state, &mut result, quantity_policy, number_locale);
    }

    ui.add_space(4.0);
    render_table_header(ui, state);
    render_points_table(ui, state, &mut result, quantity_policy, number_locale);

    if let Some(error) = &state.validation_error {
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(error.as_str())
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(c.err),
        );
    }

    render_summary(ui, state);

    result
}

fn render_new_point_row(
    ui: &mut Ui,
    state: &mut PwlEditorState,
    result: &mut PwlEditorResult,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) {
    ui.add_space(4.0);
    let compact = ui.available_width() < 330.0;
    let mut actions = |ui: &mut Ui, state: &mut PwlEditorState| {
        if Button::new("Add").accent().show(ui).clicked() {
            match normalize_pwl_pair(
                &state.new_time,
                &state.new_value,
                quantity_policy,
                number_locale,
            ) {
                Ok((time, value)) => {
                    state.new_time = time;
                    state.new_value = value;
                    state.add_point();
                    *result = PwlEditorResult::Modified;
                }
                Err(error) => state.validation_error = Some(error),
            }
        }
        if Button::new("Cancel").ghost().show(ui).clicked() {
            state.adding_point = false;
            state.new_time.clear();
            state.new_value.clear();
        }
    };
    if compact {
        ui.label("Time (s)");
        let time_response = mono_input(ui, &mut state.new_time, ui.available_width());
        ui.label(format!("Value ({})", state.value_unit));
        let value_response = mono_input(ui, &mut state.new_value, ui.available_width());
        ui.horizontal(|ui| actions(ui, state));
        annotate_pwl_input(
            ui,
            &time_response,
            &pwl_input_label(None, PwlInputField::Time, &state.value_unit),
            pwl_input_error(state.validation_error.as_deref(), None, PwlInputField::Time),
        );
        annotate_pwl_input(
            ui,
            &value_response,
            &pwl_input_label(None, PwlInputField::Value, &state.value_unit),
            pwl_input_error(
                state.validation_error.as_deref(),
                None,
                PwlInputField::Value,
            ),
        );
    } else {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            let cell = ((ui.available_width() - INDEX_COL - 170.0) * 0.5).max(70.0);
            ui.add_space(INDEX_COL);
            let time_response = mono_input(ui, &mut state.new_time, cell);
            let value_response = mono_input(ui, &mut state.new_value, cell);
            actions(ui, state);
            annotate_pwl_input(
                ui,
                &time_response,
                &pwl_input_label(None, PwlInputField::Time, &state.value_unit),
                pwl_input_error(state.validation_error.as_deref(), None, PwlInputField::Time),
            );
            annotate_pwl_input(
                ui,
                &value_response,
                &pwl_input_label(None, PwlInputField::Value, &state.value_unit),
                pwl_input_error(
                    state.validation_error.as_deref(),
                    None,
                    PwlInputField::Value,
                ),
            );
        });
    }
}

fn render_raw_source_draft(ui: &mut Ui, state: &mut PwlEditorState) -> PwlEditorResult {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let mut source = state.raw_source_draft().unwrap_or_default().to_owned();

    ui.label(
        egui::RichText::new("REPAIR PWL SOURCE")
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(c.text_faint),
    );
    ui.label(
        egui::RichText::new(
            "The imported source cannot be represented as points. Correct the retained source below.",
        )
        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
        .color(c.text_dim),
    );
    ui.add_space(4.0);
    let response = ui.add_sized(
        egui::vec2(ui.available_width(), 84.0),
        egui::TextEdit::multiline(&mut source)
            .desired_width(f32::INFINITY)
            .font(theme::mono(tokens::FS_1, FontWeight::Regular)),
    );
    let result = if response.changed() {
        let _ = state.replace_raw_source_draft(source);
        PwlEditorResult::Modified
    } else {
        PwlEditorResult::None
    };
    annotate_pwl_input(
        ui,
        &response,
        RAW_SOURCE_ACCESSIBLE_LABEL,
        state.validation_error.as_deref(),
    );

    if let Some(error) = &state.validation_error {
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(error.as_str())
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(c.err),
        );
    }

    result
}

fn render_table_header(ui: &mut Ui, state: &PwlEditorState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 20.0), egui::Sense::hover());
    let painter = ui.painter();
    let header = theme::mono(tokens::FS_0, FontWeight::Regular);
    let cell = ((rect.width() - INDEX_COL - 12.0) * 0.5).max(70.0);
    painter.text(
        egui::pos2(rect.left() + 4.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        "#",
        header.clone(),
        c.text_faint,
    );
    painter.text(
        egui::pos2(rect.left() + INDEX_COL + 6.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        "TIME (S)",
        header.clone(),
        c.text_faint,
    );
    painter.text(
        egui::pos2(rect.left() + INDEX_COL + 6.0 + cell + 6.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        format!("VALUE ({})", state.value_unit.to_uppercase()),
        header,
        c.text_faint,
    );
    painter.hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border_strong),
    );
}

fn render_points_table(
    ui: &mut Ui,
    state: &mut PwlEditorState,
    result: &mut PwlEditorResult,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    egui::ScrollArea::vertical()
        .id_salt("pwl_points_table")
        .max_height(220.0)
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            for i in 0..state.edit_buffers.len() {
                let is_selected = state.selected_row == Some(i);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 6.0;

                    // Index cell selects the row.
                    let (index_rect, index_response) = ui.allocate_exact_size(
                        egui::vec2(INDEX_COL, t.metrics.ctl_h),
                        egui::Sense::click(),
                    );
                    index_response.widget_info(|| {
                        egui::WidgetInfo::selected(
                            egui::WidgetType::SelectableLabel,
                            ui.is_enabled(),
                            is_selected,
                            format!("Select PWL point {}", i + 1),
                        )
                    });
                    if is_selected {
                        ui.painter().rect_filled(index_rect, t.radius, c.accent_dim);
                    } else if index_response.hovered() {
                        ui.painter().rect_filled(index_rect, t.radius, c.bg_hover);
                    }
                    ui.painter().text(
                        index_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        format!("{}", i + 1),
                        theme::mono(tokens::FS_0, FontWeight::Regular),
                        if is_selected { c.accent } else { c.text_faint },
                    );
                    theme::paint_focus_ring(ui, &index_response, index_rect);
                    if index_response.clicked() {
                        state.selected_row = Some(i);
                    }

                    let cell = ((ui.available_width() - 6.0) * 0.5).max(70.0);
                    let time_response = mono_input(ui, &mut state.edit_buffers[i].0, cell);
                    let value_response =
                        mono_input(ui, &mut state.edit_buffers[i].1, ui.available_width());
                    if time_response.changed() || value_response.changed() {
                        *result =
                            apply_existing_pair_edit(state, i, quantity_policy, number_locale);
                    }
                    if time_response.gained_focus() || value_response.gained_focus() {
                        state.selected_row = Some(i);
                    }
                    annotate_pwl_input(
                        ui,
                        &time_response,
                        &pwl_input_label(Some(i), PwlInputField::Time, &state.value_unit),
                        pwl_input_error(
                            state.validation_error.as_deref(),
                            Some(i),
                            PwlInputField::Time,
                        ),
                    );
                    annotate_pwl_input(
                        ui,
                        &value_response,
                        &pwl_input_label(Some(i), PwlInputField::Value, &state.value_unit),
                        pwl_input_error(
                            state.validation_error.as_deref(),
                            Some(i),
                            PwlInputField::Value,
                        ),
                    );
                });
            }
        });
}

fn apply_existing_pair_edit(
    state: &mut PwlEditorState,
    index: usize,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) -> PwlEditorResult {
    state.is_modified = true;
    match normalize_pwl_pair(
        &state.edit_buffers[index].0,
        &state.edit_buffers[index].1,
        policy,
        locale,
    ) {
        Ok((time, value)) => {
            state.edit_buffers[index] = (time, value);
            let _ = state.apply_buffer_edits();
        }
        Err(error) => state.validation_error = Some(format!("Point {} {error}", index + 1)),
    }
    PwlEditorResult::Modified
}

fn annotate_pwl_input(
    ui: &Ui,
    response: &egui::Response,
    label: &str,
    invalid_description: Option<&str>,
) {
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(label);
        if let Some(description) = invalid_description {
            node.set_invalid(egui::accesskit::Invalid::True);
            node.set_description(description);
        }
    });
}

fn pwl_input_label(row: Option<usize>, field: PwlInputField, value_unit: &str) -> String {
    let point = row
        .map(|index| format!("PWL point {}", index + 1))
        .unwrap_or_else(|| "New PWL point".to_owned());
    match field {
        PwlInputField::Time => format!("{point} time in seconds"),
        PwlInputField::Value => format!("{point} value in {value_unit}"),
    }
}

fn pwl_input_error(error: Option<&str>, row: Option<usize>, field: PwlInputField) -> Option<&str> {
    let error = error?;
    let lower = error.to_ascii_lowercase();
    if lower.contains("point ") {
        let row_number = row? + 1;
        if !lower.contains(&format!("point {row_number}")) {
            return None;
        }
    } else if row.is_some() {
        return None;
    }
    let matches_field = match field {
        PwlInputField::Time => lower.contains("time"),
        PwlInputField::Value => lower.contains("value"),
    };
    matches_field.then_some(error)
}

fn normalize_pwl_pair(
    time: &str,
    value: &str,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) -> Result<(String, String), String> {
    // The table header supplies seconds, so these cells are engineering
    // scalars in seconds rather than free-standing time expressions that
    // require an explicit `s` under the strict unit policy.
    let time_si = parse_ui_quantity(time, QuantityInputKind::EngineeringScalar, policy, locale)
        .map_err(|error| format!("Time: {error}"))?;
    if time_si < 0.0 {
        return Err("Time cannot be negative".to_owned());
    }
    let value_si = parse_ui_quantity(value, QuantityInputKind::EngineeringScalar, policy, locale)
        .map_err(|error| format!("Value: {error}"))?;
    Ok((
        format_spice_number_lossless(time_si),
        format_spice_number_lossless(value_si),
    ))
}

fn render_summary(ui: &mut Ui, state: &PwlEditorState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let mut parts = vec![format!("{} points", state.data.len())];
    if let Some((t_min, t_max)) = state.data.time_range() {
        parts.push(format!(
            "{} -> {} s",
            format_engineering_for_spice(t_min),
            format_engineering_for_spice(t_max)
        ));
    }
    if let Some((v_min, v_max)) = state.data.value_range() {
        parts.push(format!(
            "{} -> {} {}",
            format_engineering_for_spice(v_min),
            format_engineering_for_spice(v_max),
            state.value_unit
        ));
    }
    ui.add_space(6.0);
    ui.label(
        egui::RichText::new(parts.join(" · "))
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(c.text_faint),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_cell_edit_reports_modified_and_keeps_the_raw_draft() {
        let mut state = PwlEditorState::from_string("0 0 1n 1", "V");
        state.edit_buffers[1].1 = "1e".to_owned();

        let result = apply_existing_pair_edit(
            &mut state,
            1,
            QuantityPresentationPolicy::default(),
            UiNumberLocale::default(),
        );

        assert_eq!(result, PwlEditorResult::Modified);
        assert!(state.is_modified);
        assert!(state.validation_error.is_some());
        assert_eq!(state.to_string(), "0 0 1n 1e");
    }

    #[test]
    fn accessible_labels_and_invalid_descriptions_are_field_specific() {
        assert_eq!(
            pwl_input_label(Some(1), PwlInputField::Time, "V"),
            "PWL point 2 time in seconds"
        );
        assert_eq!(
            pwl_input_label(None, PwlInputField::Value, "A"),
            "New PWL point value in A"
        );
        assert_eq!(RAW_SOURCE_ACCESSIBLE_LABEL, "PWL source text");

        let error = "Point 2 Value: invalid number";
        assert_eq!(
            pwl_input_error(Some(error), Some(1), PwlInputField::Value),
            Some(error)
        );
        assert_eq!(
            pwl_input_error(Some(error), Some(1), PwlInputField::Time),
            None
        );
        assert_eq!(
            pwl_input_error(Some(error), Some(0), PwlInputField::Value),
            None
        );
    }
}
