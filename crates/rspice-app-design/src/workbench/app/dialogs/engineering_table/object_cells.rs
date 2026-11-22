//! The Object table's cells, edited in place.
//!
//! A part's name, value, parameters and purchasing fields are edited where
//! the table shows them, by the inspector's own rules: a value the inspector
//! refuses is refused here, for the same reason, and nothing is written.
//! Select rows, then edit a cell, and every selected part takes the value as
//! one undo step, the step Bulk edit takes; a name is the one part's alone.
//! Rows of wires, labels, buses and junctions are read here and edited on the
//! canvas.

use std::collections::BTreeSet;

use egui::{Grid, Key, RichText, ScrollArea, Shape, TextEdit, Ui, Vec2};
use rspice_design::schematic::bom::BomField;

use super::symbol_pins::click_row;
use super::{
    BodyAction, EngineeringTable, EngineeringTableDialogState, TableCell, content_column_widths,
    table_cell,
};
use crate::state::Component;
use crate::state::engineering_table::EngineeringProjection;
use crate::ui::theme;
use crate::ui::tokens::Tokens;
use crate::workbench::app::actions::workspace::{read_only_master_message, sync_active_schematic};
use crate::workbench::design::inspector::inline_instance::{
    edit_description, field_value, prepare_field_edit_with,
};
use crate::workbench::state::InlineEditField;

/// A cell of the Object table being edited: the rows the edit sets, the row
/// and column of the cell, and the text typed so far.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectCellEdit {
    pub(crate) rows: Vec<String>,
    pub(crate) row: String,
    pub(crate) column: String,
    pub(crate) text: String,
    /// The editor has taken the keyboard.
    pub(crate) begun: bool,
}

/// The inspector field a column of the Object table edits, if it edits one.
fn column_field(column: &str) -> Option<InlineEditField> {
    match column {
        "identifier" => Some(InlineEditField::Instance),
        "value" => Some(InlineEditField::Value),
        "parameters" => Some(InlineEditField::Parameters),
        key => BomField::from_key(key).map(|_| InlineEditField::Parameter(key.to_owned())),
    }
}

/// The part a row stands for; `None` for a wire, label, bus or junction.
fn component_id(row: &str) -> Option<u64> {
    row.strip_prefix("component-")?.parse().ok()
}

/// Whether the cell at `row`, `column` is edited here.
fn editable(row: &str, column: &str) -> bool {
    component_id(row).is_some() && column_field(column).is_some()
}

/// The part a Parameters cell leaves: the line as the inspector's Parameters
/// row takes it, then each parameter the line changes as the inspector's own
/// row for that parameter takes it, so a typed parameter is held to its
/// property sheet here too. A key the inspector reads from elsewhere (a
/// purchasing field, a multiplicity, the symbol) is left as the line's text.
fn parameters_candidate(
    registry: &crate::properties::PropertyEditorSchema,
    model: &crate::workbench::app_state::Model,
    wb: &crate::workbench::state::WorkbenchSession,
    component: &Component,
    text: &str,
) -> Result<Component, String> {
    let line = prepare_field_edit_with(
        registry,
        model,
        wb,
        component,
        &InlineEditField::Parameters,
        text,
    )?;
    let before = crate::state::parse_params_string(&component.params);
    let mut changed = crate::state::parse_params_string(&line.params)
        .into_iter()
        .filter(|(key, value)| before.get(key) != Some(value))
        .collect::<Vec<_>>();
    changed.sort();
    let mut candidate = line.clone();
    for (key, value) in changed {
        let field = InlineEditField::Parameter(key.clone());
        if field_value(&line, &field) != value {
            continue;
        }
        candidate = prepare_field_edit_with(registry, model, wb, &candidate, &field, &value)
            .map_err(|reason| format!("{key}: {reason}"))?;
    }
    Ok(candidate)
}

impl EngineeringTable<'_> {
    /// Write a cell's text to the parts its edit sets. A refusal writes
    /// nothing, and the cell opens again on the refused text with the
    /// reason under the table, so it can be corrected or dropped.
    pub(super) fn edit_object_cells(&mut self, edit: ObjectCellEdit) {
        match self.apply_object_cell_edit(&edit) {
            Ok(()) => self.dialog.error = None,
            Err(reason) => {
                self.dialog.error = Some(reason);
                self.dialog.object_edit = Some(ObjectCellEdit {
                    begun: false,
                    ..edit
                });
            }
        }
    }

    fn apply_object_cell_edit(&mut self, edit: &ObjectCellEdit) -> Result<(), String> {
        let field = column_field(&edit.column).ok_or("This column is not edited here.")?;
        if self.model.schematic_edit_read_only() || self.model.safe_mode.project_read_only() {
            return Err(read_only_master_message(self.model));
        }
        // A name is one part's: the edit sets the cell's own row alone.
        let rows = if field == InlineEditField::Instance {
            std::slice::from_ref(&edit.row)
        } else {
            edit.rows.as_slice()
        };
        let ids = rows
            .iter()
            .filter_map(|row| component_id(row))
            .collect::<BTreeSet<_>>();
        let parts = self
            .model
            .schematic
            .document()
            .components
            .iter()
            .filter(|component| ids.contains(&component.id))
            .cloned()
            .collect::<Vec<_>>();
        if parts.len() != ids.len() {
            return Err("A part in the edit is no longer in the schematic.".to_owned());
        }
        let several = parts.len() > 1;
        let mut edits = Vec::with_capacity(parts.len());
        for part in parts {
            let candidate = match &field {
                InlineEditField::Parameters => parameters_candidate(
                    self.registry,
                    self.model,
                    self.workbench,
                    &part,
                    &edit.text,
                ),
                _ => prepare_field_edit_with(
                    self.registry,
                    self.model,
                    self.workbench,
                    &part,
                    &field,
                    &edit.text,
                ),
            }
            .map_err(|reason| {
                if several {
                    format!("{}: {reason}", part.name)
                } else {
                    reason
                }
            })?;
            edits.push((part, candidate));
        }
        let description = edit_description(&field);
        let edited = edits.iter().map(|(part, _)| part.id).collect::<Vec<_>>();
        if field == InlineEditField::Instance {
            // A rename carries the probes, saved outputs and other
            // references that name the part, as the inspector's does.
            let Some((expected, candidate)) = edits.pop() else {
                return Ok(());
            };
            self.model
                .edit_component_transaction(&expected, candidate, &description)?;
        } else {
            crate::state::bulk_edit::replace_components(
                &mut self.model.schematic,
                edits,
                &description,
            )?;
        }
        // What was edited is selected, as after Bulk edit, so the inspector
        // reads it back.
        let selection = &mut self.model.schematic.session.editor.selection;
        selection.clear();
        for id in edited {
            selection.select_component(id);
        }
        sync_active_schematic(self.model, self.workbench);
        Ok(())
    }
}

/// The Object table's rows, the cells of a part's name, value, parameters
/// and purchasing fields editable: double-click a cell, or press Enter or
/// F2 on it, to edit it; Enter or a click away keeps the value and Escape
/// drops it. A click selects its row, Ctrl+click adds or removes one,
/// Shift+click selects the run from the last row clicked, and an edit of a
/// selected row's cell reaches every selected part. Enter on a cell that is
/// not edited here opens its row's details. `read_only` is why nothing can
/// be edited, when nothing can.
pub(super) fn object_grid(
    ui: &mut Ui,
    projection: &EngineeringProjection,
    state: &mut EngineeringTableDialogState,
    read_only: Option<&str>,
    height: f32,
) -> Option<BodyAction> {
    let tokens = Tokens::get(ui.ctx());
    let top = ui.cursor().top();
    if projection.rows.is_empty() {
        let empty = if projection.logical_rows == 0 {
            "The schematic has no objects."
        } else {
            "No row matches the filters."
        };
        ui.label(RichText::new(empty).color(tokens.color.text_dim));
        return None;
    }

    const CHECK_WIDTH: f32 = 28.0;
    const GAP: f32 = 8.0;
    const CELL_HEIGHT: f32 = 24.0;
    const ROW_STRIDE: f32 = 28.0;
    let headings = projection
        .columns
        .iter()
        .map(|column| {
            column.unit.as_ref().map_or_else(
                || column.label.clone(),
                |unit| format!("{} [{}]", column.label, unit),
            )
        })
        .collect::<Vec<_>>();
    // The rows' scroll bar has a gutter of its own at the table's right: the
    // columns stop short of it rather than run under the bar.
    let gutter = ui.spacing().scroll.allocated_width() + GAP;
    let room = (ui.available_width() - CHECK_WIDTH - GAP * headings.len() as f32 - gutter).max(0.0);
    let column_widths = content_column_widths(ui, projection, &headings, room);
    let mut action = None;
    Grid::new("engineering-table-preview-grid")
        .min_col_width(0.0)
        .spacing(Vec2::new(GAP, 4.0))
        .show(ui, |ui| {
            ui.allocate_exact_size(Vec2::new(CHECK_WIDTH, CELL_HEIGHT), egui::Sense::hover());
            for (heading, width) in headings.iter().zip(&column_widths) {
                table_cell(
                    ui,
                    Vec2::new(*width, CELL_HEIGHT),
                    heading,
                    TableCell::Heading,
                )
                .on_hover_text(heading.as_str());
            }
            ui.end_row();
        });
    let selection_fill = tokens.color.accent.gamma_multiply(0.10);
    // A selected row is tinted across its whole width, gaps and all.
    let row_width = CHECK_WIDTH + column_widths.iter().map(|width| width + GAP).sum::<f32>();
    let rows = |ui: &mut Ui, visible: std::ops::Range<usize>| {
        Grid::new("engineering-table-preview-rows")
            .striped(true)
            .min_col_width(0.0)
            .spacing(Vec2::new(GAP, 4.0))
            .show(ui, |ui| {
                for row_index in visible {
                    let row = &projection.rows[row_index];
                    let row_selected = state.selected_row_ids.contains(&row.stable_id);
                    let row_fill = ui.painter().add(Shape::Noop);
                    let mut selected = row_selected;
                    let check_cell = ui.allocate_ui_with_layout(
                        Vec2::new(CHECK_WIDTH, CELL_HEIGHT),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            ui.set_min_size(Vec2::new(CHECK_WIDTH, CELL_HEIGHT));
                            ui.checkbox(&mut selected, "")
                        },
                    );
                    let check = check_cell.inner.on_hover_text(
                        "Select the row: an edit of a cell reaches every selected part",
                    );
                    if row_selected {
                        let row_rect = egui::Rect::from_min_size(
                            check_cell.response.rect.min,
                            Vec2::new(row_width, CELL_HEIGHT),
                        );
                        ui.painter()
                            .set(row_fill, Shape::rect_filled(row_rect, 2.0, selection_fill));
                    }
                    if check.changed() {
                        if selected {
                            state.selected_row_ids.insert(row.stable_id.clone());
                        } else {
                            state.selected_row_ids.remove(&row.stable_id);
                        }
                        state.pin_anchor = Some(row_index);
                    }
                    for (column_index, (column, width)) in
                        projection.columns.iter().zip(&column_widths).enumerate()
                    {
                        let size = Vec2::new(*width, CELL_HEIGHT);
                        let text = row
                            .cells
                            .get(&column.id)
                            .map(|cell| cell.display.as_str())
                            .unwrap_or_default();
                        let editing = state.object_edit.as_ref().is_some_and(|edit| {
                            edit.row == row.stable_id && edit.column == column.id
                        });
                        if editing {
                            if let Some(edited) = cell_editor(ui, size, state) {
                                action = Some(edited);
                            }
                            continue;
                        }
                        let can_edit = editable(&row.stable_id, &column.id);
                        let active = state.active_cell == Some((row_index, column_index));
                        let mut response = table_cell(ui, size, text, TableCell::Value { active })
                            .on_hover_text(text);
                        if can_edit && read_only.is_none() {
                            response = response.on_hover_cursor(egui::CursorIcon::Text);
                        }
                        if state.focus_cell == Some((row_index, column_index)) {
                            response.request_focus();
                            state.focus_cell = None;
                        }
                        if response.clicked() {
                            click_row(ui, projection, state, row_index);
                            state.active_cell = Some((row_index, column_index));
                            response.request_focus();
                        }
                        let keyed = response.has_focus()
                            && ui.input(|input| {
                                input.key_pressed(Key::F2)
                                    || (input.key_pressed(Key::Enter) && input.modifiers.is_none())
                            });
                        let begin = response.double_clicked() || keyed;
                        if begin && !can_edit {
                            action = Some(BodyAction::OpenDetails(row.stable_id.clone()));
                        } else if begin && let Some(reason) = read_only {
                            state.error = Some(reason.to_owned());
                        } else if begin {
                            if !state.selected_row_ids.contains(&row.stable_id) {
                                state.selected_row_ids = BTreeSet::from([row.stable_id.clone()]);
                                state.pin_anchor = Some(row_index);
                            }
                            state.active_cell = Some((row_index, column_index));
                            state.error = None;
                            state.object_edit = Some(ObjectCellEdit {
                                rows: state.selected_row_ids.iter().cloned().collect(),
                                row: row.stable_id.clone(),
                                column: column.id.clone(),
                                text: text.to_owned(),
                                begun: false,
                            });
                            continue;
                        }
                        if response.has_focus()
                            && let Some(copied) =
                                cell_keys(ui, projection, state, row_index, column_index)
                        {
                            action = Some(copied);
                        }
                    }
                    ui.end_row();
                }
            });
    };
    let headings_height = ui.cursor().top() - top;
    ScrollArea::vertical()
        .id_salt("engineering-table-preview-vertical")
        .max_height((height - headings_height).max(ROW_STRIDE))
        .auto_shrink([false, true])
        .show_rows(ui, ROW_STRIDE, projection.rows.len(), rows);
    action
}

/// The keys a focused cell answers: arrows, Home and End move between
/// cells, Shift+Space selects the row, Ctrl+C copies.
fn cell_keys(
    ui: &Ui,
    projection: &EngineeringProjection,
    state: &mut EngineeringTableDialogState,
    row_index: usize,
    column_index: usize,
) -> Option<BodyAction> {
    let last_row = projection.rows.len().saturating_sub(1);
    let last_column = projection.columns.len().saturating_sub(1);
    let modifiers = ui.input(|input| input.modifiers);
    let target = ui.input(|input| {
        if input.key_pressed(Key::ArrowLeft) {
            Some((row_index, column_index.saturating_sub(1)))
        } else if input.key_pressed(Key::ArrowRight) {
            Some((row_index, (column_index + 1).min(last_column)))
        } else if input.key_pressed(Key::ArrowUp) {
            Some((row_index.saturating_sub(1), column_index))
        } else if input.key_pressed(Key::ArrowDown) {
            Some(((row_index + 1).min(last_row), column_index))
        } else if input.key_pressed(Key::Home) {
            Some((if modifiers.ctrl { 0 } else { row_index }, 0))
        } else if input.key_pressed(Key::End) {
            Some((
                if modifiers.ctrl { last_row } else { row_index },
                last_column,
            ))
        } else {
            None
        }
    });
    if let Some(target) = target {
        state.active_cell = Some(target);
        state.focus_cell = Some(target);
    }
    let id = &projection.rows[row_index].stable_id;
    if modifiers.shift
        && ui.input(|input| input.key_pressed(Key::Space))
        && !state.selected_row_ids.insert(id.clone())
    {
        state.selected_row_ids.remove(id);
    }
    (modifiers.command && ui.input(|input| input.key_pressed(Key::C))).then_some(BodyAction::Copy)
}

/// The text field in the cell being edited, the whole value selected as it
/// opens so typing replaces it. Returns the edit once a click away or
/// Enter keeps it.
fn cell_editor(
    ui: &mut Ui,
    size: Vec2,
    state: &mut EngineeringTableDialogState,
) -> Option<BodyAction> {
    let refused = state.error.is_some();
    let edit = state.object_edit.as_mut()?;
    let salt = format!("object-table-{}-{}", edit.row, edit.column);
    let response = ui.add_sized(
        size,
        TextEdit::singleline(&mut edit.text)
            .id_salt(&salt)
            .font(theme::field_sans()),
    );
    if refused {
        // The text the table turned away stays in its cell, outlined.
        ui.painter().rect_stroke(
            response.rect,
            2.0,
            egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.err),
            egui::StrokeKind::Inside,
        );
    }
    if !edit.begun {
        response.request_focus();
        let mut text_state =
            egui::text_edit::TextEditState::load(ui.ctx(), response.id).unwrap_or_default();
        text_state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(0),
                egui::text::CCursor::new(edit.text.chars().count()),
            )));
        text_state.store(ui.ctx(), response.id);
        edit.begun = true;
        return None;
    }
    if response.lost_focus() {
        let edit = state.object_edit.take()?;
        state.focus_cell = state.active_cell;
        return Some(BodyAction::EditObjects(edit));
    }
    None
}
