//! The two shapes whose parameters are a table rather than a row of fields.
//!
//! An authored `PWL` is a list of breakpoints, and a list is not eight text
//! boxes: it is a table you add to, delete from and select a row of, with the
//! plot above it marking the row you are on. A `PWL FILE=` is the same shape
//! read from a measurement, so its editor is the retained file, the import
//! transform the engine's loader applies, and the statement that the project
//! keeps the bytes.
//!
//! The authored text is edited token by token rather than reformatted. A time
//! someone wrote as `1m` stays `1m`: the table is a view of the text, and a
//! view that rewrote `1m` as `0.001` on every keystroke would be editing the
//! card behind the reader's back. [`PwlData`] stays the parser and the
//! validator — the slope column and the row count are read through it, and its
//! refusals are what the audit strip states.

use egui::{Rect, Ui};

use crate::properties::pwl_editor::PwlData;
use crate::state::stimulus_library::definition::StimulusKind;
use crate::state::{PropertySheet, format_engineering_display};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::state::read_field;
use crate::workbench::{AppState, MessageId};

use super::super::super::design_system::{
    WorkbenchIcon, labeled_icon_button_sized, property_row_height, property_row_input_with_hint,
};
use super::program::{field_grid, grid_height, note, note_height, quantity_unit, shape_fields};
use super::{Stage, StageAction};

/// The field an authored PWL table is stored in.
const POINTS_FIELD: &str = "pwl_data";
/// Height of one editable table row, and of the toolbar above the table.
const ROW_HEIGHT: f32 = 24.0;
/// Height of the column heads.
const HEADER_HEIGHT: f32 = 20.0;
/// How many points the table shows before it scrolls. The plot above marks the
/// selected one, so the table is for editing a neighbourhood, not for reading
/// a hundred rows at once.
const VISIBLE_ROWS: usize = 6;
/// Air between the table and the scalar fields under it.
const TABLE_FOOT: f32 = 4.0;
/// Inset of the table from its column, matching a property row's label.
const INSET: f32 = 10.0;
const INDEX_WIDTH: f32 = 26.0;
const SLOPE_WIDTH: f32 = 96.0;
const CELL_GAP: f32 = 6.0;

/// The authored point table.
pub(super) fn points_editor(
    ui: &mut Ui,
    state: &AppState,
    stage: &Stage,
    sheet: &PropertySheet,
    actions: &mut Vec<StageAction>,
) {
    let messages = state.ui.messages();
    let text = read_field(&stage.working, POINTS_FIELD);
    let cells = text
        .split_whitespace()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let rows = cells.len() / 2;
    let mut trailing = Vec::new();
    super::split_row(
        ui,
        ROW_HEIGHT,
        186.0,
        |ui| {
            ui.add_space(INSET);
            ui.add(
                egui::Label::new(
                    egui::RichText::new(messages.format(
                        MessageId::StimulusPwlPoints,
                        &[("count", &rows.to_string())],
                    ))
                    .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                    .color(Tokens::get(ui.ctx()).color.text_dim),
                )
                .truncate(),
            );
        },
        |ui| {
            ui.add_space(INSET);
            let deletable = rows > 1;
            let delete = ui
                .add_enabled_ui(deletable, |ui| {
                    labeled_icon_button_sized(
                        ui,
                        WorkbenchIcon::Trash,
                        &messages.text(MessageId::StimulusPwlDelete),
                        false,
                        84.0,
                        22.0,
                    )
                })
                .inner;
            if deletable {
                if delete.clicked()
                    && let Some(index) = stage.selected_point.filter(|index| *index < rows)
                {
                    let mut next = cells.clone();
                    next.drain(index * 2..index * 2 + 2);
                    trailing.push(StageAction::SetPoints(next.join(" ")));
                    trailing.push(StageAction::SelectPoint(Some(index.saturating_sub(1))));
                }
            } else {
                delete.on_disabled_hover_text(messages.text(MessageId::StimulusPwlKeepOnePoint));
            }
            if labeled_icon_button_sized(
                ui,
                WorkbenchIcon::Add,
                &messages.text(MessageId::StimulusPwlAdd),
                false,
                72.0,
                22.0,
            )
            .clicked()
            {
                let after = stage.selected_point.unwrap_or(rows.saturating_sub(1));
                let mut next = cells.clone();
                let at = ((after + 1) * 2).min(next.len());
                let seed = inserted_point(&cells, after);
                next.splice(at..at, seed);
                trailing.push(StageAction::SetPoints(next.join(" ")));
                trailing.push(StageAction::SelectPoint(Some(after + 1)));
            }
        },
    );
    actions.append(&mut trailing);

    if is_table(&cells) {
        table_header(ui, state, stage);
        egui::ScrollArea::vertical()
            .id_salt("workbench.stimulus.pwl.points")
            .auto_shrink([false, false])
            .max_height(visible_rows(rows) as f32 * ROW_HEIGHT)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                point_rows(ui, state, stage, &cells, actions);
            });
    } else {
        raw_table(ui, state, stage, &text, actions);
    }
    ui.add_space(TABLE_FOOT);
    field_grid(ui, state, stage, &scalar_fields(sheet), actions);
}

/// What [`points_editor`] will take in a column `width` wide.
pub(super) fn points_height(ui: &Ui, stage: &Stage, sheet: &PropertySheet, width: f32) -> f32 {
    let text = read_field(&stage.working, POINTS_FIELD);
    let cells = text.split_whitespace().collect::<Vec<_>>();
    let table = if cells.is_empty() || !cells.len().is_multiple_of(2) {
        property_row_height(ui)
    } else {
        HEADER_HEIGHT + visible_rows(cells.len() / 2) as f32 * ROW_HEIGHT
    };
    ROW_HEIGHT + table + TABLE_FOOT + grid_height(ui, &scalar_fields(sheet), width)
}

/// Whether the authored text reads as whole `(time, level)` pairs.
fn is_table(cells: &[String]) -> bool {
    !cells.is_empty() && cells.len().is_multiple_of(2)
}

/// How many rows the table shows before it scrolls.
fn visible_rows(rows: usize) -> usize {
    rows.min(VISIBLE_ROWS)
}

/// The fields a PWL card carries beside its table, in the sheet's order.
fn scalar_fields(sheet: &PropertySheet) -> Vec<&crate::state::PropertyDefinition> {
    ["td", "repeat", "r"]
        .iter()
        .filter_map(|name| sheet.get(name))
        .collect()
}

/// The table as text, for a list the parser cannot read.
///
/// An odd token count or an unparsable number is authored input, not a bug:
/// the reader is mid-edit. Showing the raw text keeps every character they
/// typed reachable, and the audit strip says what is wrong with it.
fn raw_table(
    ui: &mut Ui,
    state: &AppState,
    stage: &Stage,
    text: &str,
    actions: &mut Vec<StageAction>,
) {
    let messages = state.ui.messages();
    let editing = stage.editing_field.as_deref() == Some(POINTS_FIELD);
    let mut buffer = if editing {
        state
            .workbench
            .stimulus_editor
            .field_text(stage.saved.name(), POINTS_FIELD)
            .unwrap_or_default()
            .to_owned()
    } else {
        text.to_owned()
    };
    let response = property_row_input_with_hint(
        ui,
        &messages.text(MessageId::StimulusPwlTable),
        &mut buffer,
        "0 0 1u 1 2u 0",
        true,
    );
    if response.lost_focus() {
        actions.push(StageAction::CommitField {
            field: POINTS_FIELD.to_owned(),
            value: buffer,
        });
    } else if response.changed() || response.gained_focus() {
        actions.push(StageAction::TypeField {
            field: POINTS_FIELD.to_owned(),
            value: buffer,
        });
    }
}

/// One editable row per breakpoint, plus the slope into it.
fn point_rows(
    ui: &mut Ui,
    state: &AppState,
    stage: &Stage,
    cells: &[String],
    actions: &mut Vec<StageAction>,
) {
    let palette = Tokens::get(ui.ctx()).color;
    let unit = quantity_unit(stage.working.kind());
    let parsed = PwlData::parse(&cells.join(" ")).ok();
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);

    for index in 0..cells.len() / 2 {
        let selected = stage.selected_point == Some(index);
        // The row is allocated first and the cells are put on top of it, so a
        // press on a cell edits the cell and a press anywhere else on the row
        // selects it; the other order gives the row every press.
        let (row, response) = ui.allocate_exact_size(
            egui::Vec2::new(ui.available_width(), ROW_HEIGHT),
            egui::Sense::click(),
        );
        let columns = TableColumns::of(row);
        if selected {
            // The product's list selection: the active fill, and the accent
            // bar at the row's leading edge.
            ui.painter().rect_filled(row, 0.0, palette.bg_active);
            ui.painter().rect_filled(
                Rect::from_min_max(row.min, egui::pos2(row.left() + 2.0, row.bottom())),
                0.0,
                palette.accent,
            );
        } else if response.hovered() {
            ui.painter().rect_filled(row, 0.0, palette.bg_hover);
        }
        ui.painter().text(
            egui::pos2(columns.index.right(), row.center().y),
            egui::Align2::RIGHT_CENTER,
            (index + 1).to_string(),
            font.clone(),
            palette.text_faint,
        );
        let mut focused = false;
        for (slot, cell) in [(index * 2, columns.time), (index * 2 + 1, columns.level)] {
            focused |= cell_edit(ui, state, stage, cells, slot, cell, actions);
        }
        ui.painter().text(
            egui::pos2(columns.slope.right(), row.center().y),
            egui::Align2::RIGHT_CENTER,
            slope(parsed.as_ref(), index, unit),
            font.clone(),
            palette.text_dim,
        );
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                ui.is_enabled(),
                format!(
                    "PWL breakpoint {}: time {}, level {}",
                    index + 1,
                    cells[index * 2],
                    cells[index * 2 + 1]
                ),
            )
        });
        theme::paint_focus_ring(ui, &response, row);
        if (response.clicked() || focused) && !selected {
            actions.push(StageAction::SelectPoint(Some(index)));
        }
    }
}

/// Where the four columns of the point table sit inside one row.
///
/// One owner for the head and every row under it, so a column head is over
/// its column by construction rather than by two sums agreeing.
struct TableColumns {
    index: Rect,
    time: Rect,
    level: Rect,
    slope: Rect,
}

impl TableColumns {
    fn of(row: Rect) -> Self {
        let left = row.left() + INSET;
        let right = row.right() - INSET;
        let cell = ((right - left - INDEX_WIDTH - SLOPE_WIDTH - 3.0 * CELL_GAP) * 0.5).max(48.0);
        let span = |from: f32, width: f32| {
            Rect::from_min_max(
                egui::pos2(from, row.top() + 1.0),
                egui::pos2(from + width, row.bottom() - 1.0),
            )
        };
        let index = span(left, INDEX_WIDTH);
        let time = span(index.right() + CELL_GAP, cell);
        let level = span(time.right() + CELL_GAP, cell);
        let slope = span(level.right() + CELL_GAP, SLOPE_WIDTH);
        Self {
            index,
            time,
            level,
            slope,
        }
    }
}

/// The column heads, over the columns [`TableColumns`] gives every row.
fn table_header(ui: &mut Ui, state: &AppState, stage: &Stage) {
    let messages = state.ui.messages();
    let palette = Tokens::get(ui.ctx()).color;
    let font = theme::sans(tokens::FS_MICRO, FontWeight::Medium);
    let (row, _) = ui.allocate_exact_size(
        egui::Vec2::new(ui.available_width(), HEADER_HEIGHT),
        egui::Sense::hover(),
    );
    let columns = TableColumns::of(row);
    let level = match stage.working.kind() {
        StimulusKind::Voltage => messages.text(MessageId::StimulusPwlLevel),
        StimulusKind::Current => messages.text(MessageId::StimulusPwlCurrent),
    };
    for (rect, align, text) in [
        (columns.index, egui::Align2::RIGHT_CENTER, "#".to_owned()),
        (
            columns.time,
            egui::Align2::LEFT_CENTER,
            messages.text(MessageId::StimulusPwlTime),
        ),
        (columns.level, egui::Align2::LEFT_CENTER, level),
        (
            columns.slope,
            egui::Align2::RIGHT_CENTER,
            messages.text(MessageId::StimulusPwlSlope),
        ),
    ] {
        let x = if align == egui::Align2::RIGHT_CENTER {
            rect.right()
        } else {
            // Over the text inside the cell, not over the cell's frame.
            rect.left() + 6.0
        };
        ui.painter().text(
            egui::pos2(x, row.center().y),
            align,
            text,
            font.clone(),
            palette.text_faint,
        );
    }
    ui.painter().hline(
        (row.left() + INSET)..=(row.right() - INSET),
        row.bottom() - 0.5,
        egui::Stroke::new(1.0, palette.border),
    );
}

/// One cell of the table, editing the token at `slot` inside `cell`.
///
/// Reports whether the caret has just arrived, which selects the row: a reader
/// who tabs into a point is on that point, and the marker above should say so.
fn cell_edit(
    ui: &mut Ui,
    state: &AppState,
    stage: &Stage,
    tokens: &[String],
    slot: usize,
    cell: Rect,
    actions: &mut Vec<StageAction>,
) -> bool {
    let field = format!("{POINTS_FIELD}.{slot}");
    let editing = stage.editing_field.as_deref() == Some(field.as_str());
    let mut text = if editing {
        state
            .workbench
            .stimulus_editor
            .field_text(stage.saved.name(), &field)
            .unwrap_or_default()
            .to_owned()
    } else {
        tokens.get(slot).cloned().unwrap_or_default()
    };
    let messages = state.ui.messages();
    let label = messages.format(
        if slot.is_multiple_of(2) {
            MessageId::StimulusPwlTimeOf
        } else {
            MessageId::StimulusPwlLevelOf
        },
        &[("index", &(slot / 2 + 1).to_string())],
    );
    let response = ui.put(
        cell,
        egui::TextEdit::singleline(&mut text)
            .font(egui::TextStyle::Monospace)
            .desired_width(cell.width())
            .margin(egui::Margin::symmetric(6, 3))
            .id_salt(("workbench.stimulus.pwl.cell", slot)),
    );
    let arrived = response.gained_focus();
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(label.as_str());
    });
    if response.lost_focus() {
        let mut next = tokens.to_vec();
        if let Some(slot) = next.get_mut(slot) {
            *slot = if text.trim().is_empty() {
                "0".to_owned()
            } else {
                text.trim().to_owned()
            };
        }
        actions.push(StageAction::SetPoints(next.join(" ")));
    } else if response.changed() || response.gained_focus() {
        actions.push(StageAction::TypeField { field, value: text });
    }
    arrived
}

/// The slope into one point, as the engine's own ramp between two samples.
///
/// Spelled the way the strip above this table spells the steepest of them
/// (`5 kV/s`): the two numbers are on one screen, and the same quantity
/// written two ways there reads as two quantities.
fn slope(parsed: Option<&PwlData>, index: usize, unit: &str) -> String {
    let Some(points) = parsed.map(PwlData::points) else {
        return "\u{2014}".to_owned();
    };
    if index == 0 {
        return "\u{2014}".to_owned();
    }
    let (Some(previous), Some(point)) = (points.get(index - 1), points.get(index)) else {
        return "\u{2014}".to_owned();
    };
    let step = point.time - previous.time;
    if step <= 0.0 {
        return "step".to_owned();
    }
    crate::ui::plot::tick_with_unit(
        &format_engineering_display((point.value - previous.value) / step),
        &format!("{unit}/s"),
    )
}

/// The point an Add inserts: halfway past the selected row, at its level.
///
/// Halfway rather than at the same time, because two points at one time is a
/// vertical step and a reader who pressed Add wanted a breakpoint, not an
/// edge. The level is copied so the inserted point changes nothing until it
/// is given one.
fn inserted_point(tokens: &[String], after: usize) -> Vec<String> {
    let parse = |slot: usize| {
        tokens
            .get(slot)
            .and_then(|text| crate::quantity::parse_engineering_value(text).ok())
    };
    let time = parse(after * 2).unwrap_or(0.0);
    let next = parse((after + 1) * 2);
    let level = tokens
        .get(after * 2 + 1)
        .cloned()
        .unwrap_or_else(|| "0".to_owned());
    let inserted = match next {
        Some(next) if next > time => (time + next) * 0.5,
        _ => {
            if time > 0.0 {
                time * 2.0
            } else {
                1e-6
            }
        }
    };
    // Shortest round-trip decimal rather than engineering notation: the
    // inserted time is a number this editor made up, and writing it the way
    // the value actually is keeps it exactly what a reparse will read.
    vec![format!("{inserted}"), level]
}

/// The measured-file shape.
pub(super) fn file_editor(
    ui: &mut Ui,
    state: &AppState,
    stage: &Stage,
    sheet: &PropertySheet,
    actions: &mut Vec<StageAction>,
) {
    let messages = state.ui.messages();
    let palette = Tokens::get(ui.ctx()).color;
    let retained = match stage.working.pwl_file.as_ref() {
        Some(file) => messages.format(
            MessageId::StimulusRetainedFileDetail,
            &[
                ("name", file.file_name.as_str()),
                ("bytes", &file.contents.len().to_string()),
                (
                    "digest",
                    &crate::state::model_library::short_digest(&file.digest.to_string()),
                ),
            ],
        ),
        None => messages.text(MessageId::StimulusNoFileRetained),
    };
    let mut trailing = Vec::new();
    super::split_row(
        ui,
        ROW_HEIGHT,
        134.0,
        |ui| {
            ui.add_space(INSET);
            ui.add(
                egui::Label::new(
                    egui::RichText::new(retained)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(palette.text_dim),
                )
                .truncate(),
            );
        },
        |ui| {
            ui.add_space(INSET);
            if labeled_icon_button_sized(
                ui,
                WorkbenchIcon::Folder,
                &messages.text(MessageId::StimulusImportFile),
                false,
                112.0,
                22.0,
            )
            .clicked()
            {
                trailing.push(StageAction::ImportDataFile);
            }
        },
    );
    actions.append(&mut trailing);
    field_grid(ui, state, stage, &shape_fields(sheet, stage), actions);
    note(ui, &messages.text(MessageId::StimulusRetainedNote));
}

/// What [`file_editor`] will take in a column `width` wide.
pub(super) fn file_height(
    ui: &Ui,
    state: &AppState,
    stage: &Stage,
    sheet: &PropertySheet,
    width: f32,
) -> f32 {
    ROW_HEIGHT
        + grid_height(ui, &shape_fields(sheet, stage), width)
        + note_height(
            ui,
            &state.ui.messages().text(MessageId::StimulusRetainedNote),
            width,
        )
}
