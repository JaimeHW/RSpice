//! The pin table: the engineering table over a symbol's pins, edited in
//! place.
//!
//! Every cell edits the field the inspector edits on one pin, by the
//! inspector's own rules. Select rows, edit a cell, and every selected row
//! takes the value as one undo step named for what it did. The rows the
//! table selects are the pins the canvas selects: the two are one
//! selection.

use std::collections::BTreeSet;

use egui::{Grid, Key, Modifiers, RichText, ScrollArea, Shape, TextEdit, Ui, Vec2};
use rspice_design::symbol::SymbolDocument;
use rspice_schematic_editor::symbol_editor::pin_table::{
    PIN_TABLE_GRID_ID, PinTableColumn, PinTableEdit, move_pins, set_pin_cells,
};
use rspice_schematic_editor::symbol_editor::session::SymbolSelection;

use super::{
    BodyAction, EngineeringTable, EngineeringTableDialogState, TableCell, content_column_widths,
    table_cell,
};
use crate::state::ViewType;
use crate::state::engineering_table::{
    EngineeringDataset, EngineeringProjection, active_schematic_dataset, symbol_pin_dataset,
    symbol_pin_row_index,
};
use crate::ui::theme;
use crate::ui::tokens::Tokens;
use crate::ui::widgets::select_with_response;
use crate::workbench::SymbolCommitIntent;
use crate::workbench::app::actions::workspace::read_only_master_message;
use crate::workbench::app::actions::workspace::symbol_document::{
    commit_active_symbol_edit, load_active_symbol_editor_metadata, record_symbol_edit,
};
use crate::workbench::app_state::Model;

/// Rows the pin table shows before its rows scroll: a symbol's pins are
/// what the table is for, so it shows twice the rows a schematic's objects
/// get under the manager's controls.
const SHOWN_ROWS: f32 = 12.0;

/// A cell of the pin table being edited: the rows the edit sets, the
/// column, and the text typed or picked so far.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinCellEdit {
    pub(crate) rows: Vec<String>,
    pub(crate) row: String,
    pub(crate) column: PinTableColumn,
    pub(crate) text: String,
    /// The editor has taken the keyboard, or opened its list.
    pub(crate) begun: bool,
}

/// The dataset the engineering table shows: a symbol's pins where a symbol
/// is in front, the schematic's objects otherwise.
pub(super) fn active_dataset(model: &Model) -> EngineeringDataset {
    if model.workspace.content.active_view_type() == Some(ViewType::Symbol) {
        let document = model.load_active_symbol_document().unwrap_or_default();
        return symbol_pin_dataset(&document, model.library_manager.revision());
    }
    active_schematic_dataset(&model.schematic)
}

/// Whether `dataset` is a symbol's pins.
pub(super) fn is_pin_table(dataset: &EngineeringDataset) -> bool {
    dataset.id == PIN_TABLE_GRID_ID
}

/// The row ids of the pins the canvas selects.
pub(super) fn selected_pin_rows(
    dataset: &EngineeringDataset,
    selection: &SymbolSelection,
) -> BTreeSet<String> {
    dataset
        .rows
        .iter()
        .filter(|row| {
            row.cells
                .get(PinTableColumn::Name.id())
                .is_some_and(|name| {
                    selection
                        .pins
                        .iter()
                        .any(|pin| pin.eq_ignore_ascii_case(&name.display))
                })
        })
        .map(|row| row.stable_id.clone())
        .collect()
}

/// The pins the rows `ids` stand for, in netlist order.
fn pin_names(document: &SymbolDocument, ids: &[String]) -> Vec<String> {
    let mut indices = ids
        .iter()
        .filter_map(|id| symbol_pin_row_index(document.pins.len(), id))
        .collect::<Vec<_>>();
    indices.sort_unstable();
    indices.dedup();
    indices
        .into_iter()
        .map(|index| document.pins[index].name.clone())
        .collect()
}

impl EngineeringTable<'_> {
    /// Before the table draws: its rows select the pins the canvas selects.
    pub(super) fn take_canvas_pin_selection(&mut self, dataset: &EngineeringDataset) {
        self.dialog.selected_row_ids =
            selected_pin_rows(dataset, &self.symbol.editor.effective_selection());
    }

    /// After the table draws: the canvas selects the pins its rows select,
    /// where the reader changed them.
    pub(super) fn give_canvas_pin_selection(
        &mut self,
        dataset: &EngineeringDataset,
        before: &BTreeSet<String>,
    ) {
        if self.dialog.selected_row_ids == *before {
            return;
        }
        let pins = dataset
            .rows
            .iter()
            .filter(|row| self.dialog.selected_row_ids.contains(&row.stable_id))
            .filter_map(|row| row.cells.get(PinTableColumn::Name.id()))
            .map(|name| name.display.clone())
            .collect();
        self.symbol.editor.set_selection(SymbolSelection {
            pins,
            ..SymbolSelection::default()
        });
    }

    /// Set `column` of the pins `rows` stand for to `text`.
    pub(super) fn edit_symbol_pins(&mut self, rows: &[String], column: PinTableColumn, text: &str) {
        self.apply_pin_table_edit(rows, |document, pins| {
            set_pin_cells(document, pins, column, text)
        });
    }

    /// Move the pins the selected rows stand for one netlist place.
    pub(super) fn move_symbol_pins(&mut self, later: bool) {
        let rows = self
            .dialog
            .selected_row_ids
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        self.apply_pin_table_edit(&rows, |document, pins| Ok(move_pins(document, pins, later)));
    }

    /// One table edit of the active symbol: recorded as one undo step,
    /// published as the inspector publishes, and left selected.
    fn apply_pin_table_edit(
        &mut self,
        rows: &[String],
        edit: impl FnOnce(&mut SymbolDocument, &[String]) -> Result<Option<PinTableEdit>, String>,
    ) {
        if self.model.active_view_read_only() {
            self.dialog.error = Some(read_only_master_message(self.model));
            return;
        }
        let loaded = self.model.load_active_symbol_document().and_then(|before| {
            load_active_symbol_editor_metadata(self.model, &before)
                .map(|metadata| (before, metadata))
        });
        let (before, metadata) = match loaded {
            Ok(loaded) => loaded,
            Err(error) => {
                self.dialog.error = Some(error);
                return;
            }
        };
        let pins = pin_names(&before, rows);
        let mut document = before.clone();
        let outcome = match edit(&mut document, &pins) {
            Ok(Some(outcome)) => outcome,
            Ok(None) => {
                self.dialog.error = None;
                return;
            }
            Err(reason) => {
                self.dialog.error = Some(reason);
                return;
            }
        };
        record_symbol_edit(self.symbol, self.model, &before, &outcome.step);
        let intent = SymbolCommitIntent {
            renames: outcome.renames,
        };
        if let Err(error) = commit_active_symbol_edit(
            self.symbol,
            self.model,
            self.workbench,
            &document,
            &metadata,
            &intent,
        ) {
            self.dialog.error = Some(error);
            return;
        }
        self.dialog.error = None;
        self.symbol.editor.set_selection(SymbolSelection {
            pins: outcome.pins.into_iter().collect(),
            ..SymbolSelection::default()
        });
        self.workbench.prompt(outcome.step);
    }
}

/// The pin table's verbs beside Export and Copy: the selected pins one
/// netlist place earlier or later.
pub(super) fn reorder_verbs(ui: &mut Ui, selected: bool) -> Option<BodyAction> {
    use crate::ui::widgets::Button;
    let later = Button::new("Move later")
        .enabled(selected)
        .show(ui)
        .on_hover_text("Move the selected pins one netlist place later (Alt+Down)")
        .clicked();
    let earlier = Button::new("Move earlier")
        .enabled(selected)
        .show(ui)
        .on_hover_text("Move the selected pins one netlist place earlier (Alt+Up)")
        .clicked();
    if earlier {
        Some(BodyAction::MovePins { later: false })
    } else if later {
        Some(BodyAction::MovePins { later: true })
    } else {
        None
    }
}

/// The pin table's rows, each cell editable: double-click a cell, or press
/// Enter or F2 on it, to edit it; Enter or a click away keeps the value and
/// Escape drops it. A click selects its row, Ctrl+click adds or removes
/// one, Shift+click selects the run from the last row clicked; a click on a
/// selected row keeps the selection, so the cell edits every selected pin.
pub(super) fn pin_grid(
    ui: &mut Ui,
    projection: &EngineeringProjection,
    state: &mut EngineeringTableDialogState,
) -> Option<BodyAction> {
    let tokens = Tokens::get(ui.ctx());
    if projection.rows.is_empty() {
        let empty = if projection.logical_rows == 0 {
            "This symbol has no pins."
        } else {
            "No pin matches the filters."
        };
        ui.label(RichText::new(empty).color(tokens.color.text_dim));
        return None;
    }

    const CHECK_WIDTH: f32 = 28.0;
    const GAP: f32 = 8.0;
    const CELL_HEIGHT: f32 = 24.0;
    const ROW_STRIDE: f32 = 28.0;
    let columns = projection
        .columns
        .iter()
        .map(|column| PinTableColumn::from_id(&column.id))
        .collect::<Vec<_>>();
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
    let rows_scroll = !tokens.metrics.is_touch();
    let gutter = if rows_scroll {
        ui.spacing().scroll.allocated_width() + GAP
    } else {
        0.0
    };
    let room = (ui.available_width() - CHECK_WIDTH - GAP * headings.len() as f32 - gutter).max(0.0);
    let column_widths = content_column_widths(ui, projection, &headings, room);
    let mut action = None;
    Grid::new("symbol-pin-table-headings")
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
    let mut rows = |ui: &mut Ui, visible: std::ops::Range<usize>| {
        Grid::new("symbol-pin-table-rows")
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
                    let check = check_cell.inner.on_hover_text("Select the pin");
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
                    for (column_index, ((column, width), kind)) in projection
                        .columns
                        .iter()
                        .zip(&column_widths)
                        .zip(&columns)
                        .enumerate()
                    {
                        let size = Vec2::new(*width, CELL_HEIGHT);
                        let text = row
                            .cells
                            .get(&column.id)
                            .map(|cell| cell.display.as_str())
                            .unwrap_or_default();
                        let editing = state.pin_edit.as_ref().is_some_and(|edit| {
                            edit.row == row.stable_id && Some(edit.column) == *kind
                        });
                        if editing {
                            if let Some(edited) = cell_editor(ui, size, state) {
                                action = Some(edited);
                            }
                            continue;
                        }
                        let active = state.active_cell == Some((row_index, column_index));
                        let response = table_cell(ui, size, text, TableCell::Value { active })
                            .on_hover_text(text);
                        if state.focus_cell == Some((row_index, column_index)) {
                            response.request_focus();
                            state.focus_cell = None;
                        }
                        if response.clicked() {
                            click_row(ui, projection, state, row_index);
                            state.active_cell = Some((row_index, column_index));
                            response.request_focus();
                        }
                        let begin = response.double_clicked()
                            || (response.has_focus()
                                && ui.input(|input| {
                                    input.key_pressed(Key::F2)
                                        || (input.key_pressed(Key::Enter)
                                            && input.modifiers.is_none())
                                }));
                        if begin && let Some(column) = *kind {
                            if !state.selected_row_ids.contains(&row.stable_id) {
                                state.selected_row_ids = BTreeSet::from([row.stable_id.clone()]);
                                state.pin_anchor = Some(row_index);
                            }
                            state.active_cell = Some((row_index, column_index));
                            state.pin_edit = Some(PinCellEdit {
                                rows: state.selected_row_ids.iter().cloned().collect(),
                                row: row.stable_id.clone(),
                                column,
                                text: text.to_owned(),
                                begun: false,
                            });
                            continue;
                        }
                        if response.has_focus()
                            && let Some(moved) =
                                cell_keys(ui, projection, state, row_index, column_index)
                        {
                            action = Some(moved);
                        }
                    }
                    ui.end_row();
                }
            });
    };
    if rows_scroll {
        ScrollArea::vertical()
            .id_salt("symbol-pin-table-rows-scroll")
            .max_height(ROW_STRIDE * SHOWN_ROWS)
            .auto_shrink([false, true])
            .show_rows(ui, ROW_STRIDE, projection.rows.len(), |ui, visible| {
                rows(ui, visible);
            });
    } else {
        rows(ui, 0..projection.rows.len());
    }
    action
}

/// A click on a row's cell: the row alone, Ctrl+click toggles it,
/// Shift+click the run from the anchor; a plain click on a selected row
/// keeps the selection, so an edit of its cell reaches every selected pin.
fn click_row(
    ui: &Ui,
    projection: &EngineeringProjection,
    state: &mut EngineeringTableDialogState,
    row_index: usize,
) {
    let modifiers = ui.input(|input| input.modifiers);
    let id = &projection.rows[row_index].stable_id;
    if modifiers.command {
        if !state.selected_row_ids.remove(id) {
            state.selected_row_ids.insert(id.clone());
        }
        state.pin_anchor = Some(row_index);
    } else if modifiers.shift {
        let anchor = state
            .pin_anchor
            .filter(|anchor| *anchor < projection.rows.len())
            .unwrap_or(row_index);
        state.selected_row_ids = projection.rows[anchor.min(row_index)..=anchor.max(row_index)]
            .iter()
            .map(|row| row.stable_id.clone())
            .collect();
    } else {
        if !state.selected_row_ids.contains(id) {
            state.selected_row_ids = BTreeSet::from([id.clone()]);
        }
        state.pin_anchor = Some(row_index);
    }
}

/// The keys a focused cell answers: arrows, Home and End move between
/// cells, Shift+Space selects the row, Alt+Up and Alt+Down move the selected
/// pins in netlist order, Ctrl+C copies.
fn cell_keys(
    ui: &Ui,
    projection: &EngineeringProjection,
    state: &mut EngineeringTableDialogState,
    row_index: usize,
    column_index: usize,
) -> Option<BodyAction> {
    let last_row = projection.rows.len().saturating_sub(1);
    let last_column = projection.columns.len().saturating_sub(1);
    if ui.input_mut(|input| input.consume_key(Modifiers::ALT, Key::ArrowUp)) {
        return Some(BodyAction::MovePins { later: false });
    }
    if ui.input_mut(|input| input.consume_key(Modifiers::ALT, Key::ArrowDown)) {
        return Some(BodyAction::MovePins { later: true });
    }
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

/// The editor in the cell being edited: a list of the column's values, or
/// a text field. Returns the edit once a value is picked or kept.
fn cell_editor(
    ui: &mut Ui,
    size: Vec2,
    state: &mut EngineeringTableDialogState,
) -> Option<BodyAction> {
    let edit = state.pin_edit.as_mut()?;
    let column = edit.column;
    let salt = format!("symbol-pin-table-{}-{}", edit.row, column.id());
    if let Some(choices) = column.choices() {
        let options = choices.into_iter().map(str::to_owned).collect::<Vec<_>>();
        let popup = ui.make_persistent_id(("rspice.select", salt.as_str()));
        let opening = !edit.begun;
        if opening {
            egui::Popup::open_id(ui.ctx(), popup);
            edit.begun = true;
        }
        let output = select_with_response(ui, &salt, column.label(), &edit.text, &options, size.x);
        if opening {
            output.response.request_focus();
        }
        if let Some(picked) = output.picked {
            let edit = state.pin_edit.take()?;
            return Some(BodyAction::EditPins {
                rows: edit.rows,
                column,
                text: options[picked].clone(),
            });
        }
        if !opening && !egui::Popup::is_id_open(ui.ctx(), popup) {
            // The list closed with nothing picked: Escape, or a click away.
            state.pin_edit = None;
            state.focus_cell = state.active_cell;
        }
        return None;
    }
    let response = ui.add_sized(
        size,
        TextEdit::singleline(&mut edit.text)
            .id_salt(&salt)
            .font(theme::field_sans()),
    );
    if !edit.begun {
        // The value is selected whole, so typing replaces it.
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
        let edit = state.pin_edit.take()?;
        state.focus_cell = state.active_cell;
        return Some(BodyAction::EditPins {
            rows: edit.rows,
            column,
            text: edit.text,
        });
    }
    None
}
