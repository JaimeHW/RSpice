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

use egui::Ui;

use crate::properties::pwl_editor::PwlData;
use crate::state::stimulus_library::definition::StimulusKind;
use crate::state::{PropertySheet, format_engineering};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::state::read_field;
use crate::workbench::{AppState, MessageId};

use super::super::super::design_system::{
    WorkbenchIcon, labeled_icon_button_sized, property_row_input_with_hint,
};
use super::program::{field_row, note, quantity_unit, shape_fields};
use super::{Stage, StageAction};

/// The field an authored PWL table is stored in.
const POINTS_FIELD: &str = "pwl_data";
/// Height of one editable table row.
const ROW_HEIGHT: f32 = 22.0;

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
        176.0,
        |ui| {
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

    if rows == 0 || !cells.len().is_multiple_of(2) {
        raw_table(ui, state, stage, &text, actions);
    } else {
        egui::ScrollArea::vertical()
            .id_salt("workbench.stimulus.pwl.points")
            .auto_shrink([false, false])
            .max_height((ui.available_height() - 30.0).max(30.0))
            .show(ui, |ui| {
                point_rows(ui, state, stage, &cells, actions);
            });
    }
    for name in ["td", "repeat", "r"] {
        if let Some(definition) = sheet.get(name) {
            field_row(ui, state, stage, definition, actions);
        }
    }
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
    tokens: &[String],
    actions: &mut Vec<StageAction>,
) {
    let messages = state.ui.messages();
    let palette = Tokens::get(ui.ctx()).color;
    let unit = quantity_unit(stage.working.kind());
    let parsed = PwlData::parse(&tokens.join(" ")).ok();
    let level_label = match stage.working.kind() {
        StimulusKind::Voltage => messages.text(MessageId::StimulusPwlLevel),
        StimulusKind::Current => messages.text(MessageId::StimulusPwlCurrent),
    };
    let width = ui.available_width();
    let index_width = 28.0;
    let slope_width = 84.0;
    let cell = ((width - index_width - slope_width - 24.0) * 0.5).max(48.0);

    ui.horizontal(|ui| {
        header(ui, "#", index_width, palette.text_faint);
        header(
            ui,
            &messages.text(MessageId::StimulusPwlTime),
            cell,
            palette.text_faint,
        );
        header(ui, &level_label, cell, palette.text_faint);
        header(
            ui,
            &messages.text(MessageId::StimulusPwlSlope),
            slope_width,
            palette.text_faint,
        );
    });

    for index in 0..tokens.len() / 2 {
        let selected = stage.selected_point == Some(index);
        let row = ui.horizontal(|ui| {
            ui.set_min_height(ROW_HEIGHT);
            header(
                ui,
                &(index + 1).to_string(),
                index_width,
                palette.text_faint,
            );
            cell_edit(ui, state, stage, tokens, index * 2, cell, actions);
            cell_edit(ui, state, stage, tokens, index * 2 + 1, cell, actions);
            header(
                ui,
                &slope(parsed.as_ref(), index, unit),
                slope_width,
                palette.text_dim,
            );
        });
        if selected {
            ui.painter().rect_stroke(
                row.response.rect,
                2.0,
                egui::Stroke::new(1.0, palette.accent),
                egui::StrokeKind::Inside,
            );
        }
        if row.response.interact(egui::Sense::click()).clicked() {
            actions.push(StageAction::SelectPoint(Some(index)));
        }
    }
}

/// One cell of the table, editing the token at `slot`.
fn cell_edit(
    ui: &mut Ui,
    state: &AppState,
    stage: &Stage,
    tokens: &[String],
    slot: usize,
    width: f32,
    actions: &mut Vec<StageAction>,
) {
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
    let response = ui.add(
        egui::TextEdit::singleline(&mut text)
            .font(egui::TextStyle::Monospace)
            .desired_width(width)
            .margin(egui::Margin::symmetric(5, 2))
            .id_salt(("workbench.stimulus.pwl.cell", slot)),
    );
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
}

/// The slope into one point, as the engine's own ramp between two samples.
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
    format!(
        "{}{unit}/s",
        format_engineering((point.value - previous.value) / step)
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

/// One fixed-width, non-interactive cell.
fn header(ui: &mut Ui, text: &str, width: f32, color: egui::Color32) {
    ui.add_sized(
        [width, ROW_HEIGHT],
        egui::Label::new(
            egui::RichText::new(text)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(color),
        )
        .truncate(),
    );
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
        124.0,
        |ui| {
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
    for definition in shape_fields(sheet, stage) {
        field_row(ui, state, stage, definition, actions);
    }
    note(ui, &messages.text(MessageId::StimulusRetainedNote));
}
