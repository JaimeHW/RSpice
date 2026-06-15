//! The PWL point table in design-system style: mono column headers, inset
//! mono cells, accent row selection, and a faint summary footer.

use egui::Ui;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, mono_input};

use super::data::format_engineering_for_spice;
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

/// Render the PWL editor widget.
///
/// Returns `PwlEditorResult::Modified` if the data changed.
pub fn render_pwl_editor(ui: &mut Ui, state: &mut PwlEditorState) -> PwlEditorResult {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let mut result = PwlEditorResult::None;

    // Action strip: kicker left, add/remove right.
    ui.horizontal(|ui| {
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
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if Button::new("+ Add point").ghost().show(ui).clicked() {
                state.adding_point = true;
            }
            if state.selected_row.is_some() && Button::new("Remove").ghost().show(ui).clicked() {
                state.delete_selected();
                result = PwlEditorResult::Modified;
            }
        });
    });

    if state.adding_point {
        render_new_point_row(ui, state, &mut result);
    }

    ui.add_space(4.0);
    render_table_header(ui, state);
    render_points_table(ui, state, &mut result);

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

fn render_new_point_row(ui: &mut Ui, state: &mut PwlEditorState, result: &mut PwlEditorResult) {
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        let cell = ((ui.available_width() - INDEX_COL - 170.0) * 0.5).max(70.0);
        ui.add_space(INDEX_COL);
        mono_input(ui, &mut state.new_time, cell);
        mono_input(ui, &mut state.new_value, cell);
        if Button::new("Add").accent().show(ui).clicked() {
            state.add_point();
            *result = PwlEditorResult::Modified;
        }
        if Button::new("Cancel").ghost().show(ui).clicked() {
            state.adding_point = false;
            state.new_time.clear();
            state.new_value.clear();
        }
    });
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

fn render_points_table(ui: &mut Ui, state: &mut PwlEditorState, result: &mut PwlEditorResult) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    egui::ScrollArea::vertical()
        .id_salt("pwl_points_table")
        .max_height(220.0)
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            let mut data_changed = false;

            for i in 0..state.edit_buffers.len() {
                let is_selected = state.selected_row == Some(i);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 6.0;

                    // Index cell selects the row.
                    let (index_rect, index_response) = ui.allocate_exact_size(
                        egui::vec2(INDEX_COL, t.metrics.ctl_h),
                        egui::Sense::click(),
                    );
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
                    if index_response.clicked() {
                        state.selected_row = Some(i);
                    }

                    let cell = ((ui.available_width() - 6.0) * 0.5).max(70.0);
                    let time_response = mono_input(ui, &mut state.edit_buffers[i].0, cell);
                    let value_response =
                        mono_input(ui, &mut state.edit_buffers[i].1, ui.available_width());
                    if time_response.changed() || value_response.changed() {
                        data_changed = true;
                    }
                    if time_response.gained_focus() || value_response.gained_focus() {
                        state.selected_row = Some(i);
                    }
                });
            }

            if data_changed {
                match state.sync_data_from_buffers() {
                    Ok(()) => {
                        state.validation_error = None;
                        *result = PwlEditorResult::Modified;
                    }
                    Err(e) => {
                        state.validation_error = Some(e.to_string());
                    }
                }
            }
        });
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
