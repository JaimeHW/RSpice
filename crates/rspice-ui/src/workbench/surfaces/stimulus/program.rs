//! What the definition is made of, in the sheet's own words.
//!
//! Two columns. On the left the operating point and the small-signal layer,
//! which every source has and which no waveform family owns; on the right the
//! shape's own fields. Both come from the `PropertySheet` the registry already
//! holds for the component type, in that sheet's declared order, with that
//! sheet's labels, units, defaults and `required` marks. There is no second
//! table of parameter names here, and there must never be one: the sheet is
//! what the property dialog edits, what the netlister reads and what the
//! engine contract audits, so a field this band invented would be a field
//! nothing else in the product knows about.
//!
//! An empty field is not an unset field. It means "leave it to the engine",
//! and what the engine does then is the sheet's `default_value` for the ones
//! the netlister fills in and a substitution the audit strip states for the
//! ones the parser resolves. That is why the hint is the default rather than
//! the word "optional".
//!
//! The band is as tall as what it holds. The shape's fields are laid into a
//! grid of as many columns as the width gives them, so a seven-field pulse is
//! three rows on a laptop rather than a seven-row list in a slot that shows
//! three and a half of them; [`content_height`] states that height before a
//! pixel is painted, from the same arithmetic the painter then follows, and
//! the stage sizes the band to it. The band scrolls only when the stage is
//! too short to give it that, which is the one case a list cannot avoid.

use egui::{Rect, Ui, UiBuilder, Vec2};

use crate::state::stimulus_library::definition::{StimulusFamily, StimulusKind};
use crate::state::{PropertyDefinition, PropertySheet, PropertyValue};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::state::read_field;
use crate::workbench::{AppState, MessageId};

use super::super::super::design_system::{
    PANEL_SECTION_H, property_row_combo_labelled, property_row_height,
    property_row_input_with_hint_labelled, property_row_width_for_label, section_header,
};
use super::pwl;
use super::{Stage, StageAction};

/// Fields the bias column owns, whichever family's sheet declares them.
const BIAS_FIELDS: [&str; 3] = ["dc", "ac", "acphase"];
/// The column split. The shape has more fields than the bias, so it takes the
/// larger share; below this width the two stack instead.
const BIAS_FRACTION: f32 = 0.3;
const STACK_BELOW: f32 = 620.0;
/// The narrowest a property column can be and still show a label beside a
/// value. The two-column split is only taken well above twice this, so the
/// clamp is a statement of the contract rather than a fallback.
const MINIMUM_COLUMN: f32 = 200.0;
/// The narrowest one cell of the field grid is drawn, and the most columns a
/// grid is ever given. Below the first a value has no room beside its label;
/// past the second a row of fields stops reading as the card's own order.
const MINIMUM_CELL: f32 = 232.0;
const MAXIMUM_GRID_COLUMNS: usize = 3;
/// Air between two rows of the grid, and under the last one.
const ROW_GAP: f32 = 4.0;
/// Air between a column's head and its first row. With [`ROW_GAP`] under the
/// last row and [`BAND_FOOT`] under that, the fields sit in the band with the
/// same margin above as below; flush under the head they read as hung from it.
const HEAD_GAP: f32 = 6.0;
const BAND_FOOT: f32 = 2.0;

/// How tall the band is when nothing in it has to scroll.
///
/// The same walk [`show`] makes, adding heights instead of painting: the two
/// must agree to the pixel, because the stage gives the band exactly this and
/// a row the painter added that this did not count is a row under the seam.
pub(super) fn content_height(ui: &Ui, state: &AppState, stage: &Stage, width: f32) -> f32 {
    let Some(sheet) = state.property_registry.get(stage.working.component_type()) else {
        return 0.0;
    };
    let bias = bias_height(ui, state, sheet, bias_width(width).unwrap_or(width));
    let shape = shape_height(
        ui,
        state,
        stage,
        sheet,
        bias_width(width).map_or(width, |bias| width - bias),
    );
    let columns = if bias_width(width).is_some() {
        bias.max(shape)
    } else {
        bias + shape
    };
    columns + BAND_FOOT
}

/// The bias column's width, or `None` when the band is too narrow for two
/// columns and stacks them.
fn bias_width(width: f32) -> Option<f32> {
    (width >= STACK_BELOW)
        .then(|| (width * BIAS_FRACTION).clamp(MINIMUM_COLUMN, width - MINIMUM_COLUMN))
}

pub(super) fn show(ui: &mut Ui, state: &AppState, stage: &Stage, actions: &mut Vec<StageAction>) {
    let Some(sheet) = state.property_registry.get(stage.working.component_type()) else {
        return;
    };
    let band = ui.available_rect_before_wrap();
    let Some(bias_width) = bias_width(band.width()) else {
        egui::ScrollArea::vertical()
            .id_salt("workbench.stimulus.program.stacked")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                bias_column(ui, state, stage, sheet, actions);
                shape_column(ui, state, stage, sheet, actions);
            });
        return;
    };
    // Two measured rectangles rather than two allocations in a row. A column
    // allocated at a fixed width inside a row can be handed less than it asked
    // for, and what is left for the column beside it is then nothing at all —
    // which is not a narrow property row, it is a property row of negative
    // width. Splitting the band makes both columns' widths arithmetic.
    let bias_rect = Rect::from_min_max(
        band.min,
        egui::pos2(band.left() + bias_width, band.bottom()),
    );
    let shape_rect = Rect::from_min_max(egui::pos2(bias_rect.right(), band.top()), band.max);
    ui.painter().vline(
        bias_rect.right(),
        band.y_range(),
        egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
    );
    column(ui, bias_rect, "bias", |ui| {
        bias_column(ui, state, stage, sheet, actions);
    });
    column(ui, shape_rect, "shape", |ui| {
        shape_column(ui, state, stage, sheet, actions);
    });
}

/// One column of the program, scrolling inside its own rectangle.
fn column(ui: &mut Ui, rect: Rect, salt: &'static str, content: impl FnOnce(&mut Ui)) {
    let clip = ui.clip_rect();
    let mut column = ui.new_child(UiBuilder::new().max_rect(rect));
    column.set_clip_rect(clip.intersect(rect));
    column.set_max_size(rect.size());
    egui::ScrollArea::vertical()
        .id_salt(("workbench.stimulus.program", salt))
        .auto_shrink([false, false])
        .show(&mut column, |ui| {
            // Every height in this band is stated, so the spacing between two
            // items is part of those statements rather than added to them.
            ui.spacing_mut().item_spacing.y = 0.0;
            content(ui);
        });
}

/// The pitch of one grid row: the design system's property row, and the air
/// under it.
pub(super) fn row_pitch(ui: &Ui) -> f32 {
    property_row_height(ui) + ROW_GAP
}

/// How a grid of fields is laid into a width: how many columns, which
/// spelling of the labels, and how wide the label column is in every cell.
#[derive(Debug, Clone, Copy, PartialEq)]
struct GridShape {
    columns: usize,
    compact: bool,
    label_width: f32,
}

/// Air between the widest label and the value column.
const LABEL_AIR: f32 = 6.0;

/// The most columns whose cells still hold every label whole.
///
/// The label column is as wide as the widest label in the grid rather than a
/// share of the cell, so a short label does not strand its value half a cell
/// away and a long one is not cut while its value has room to spare. Column
/// counts are tried from the widest down, and at each the full names before
/// the card's words: a pulse's seven fields spell themselves out in two
/// columns on the smallest window and three on a laptop, and fall back to
/// `V1`, `TD`, `TR` only where the names would not fit. One column holds any
/// label the sheet declares, so the walk always ends on a shape that cuts
/// nothing.
fn grid_shape(
    ui: &Ui,
    fields: &[&PropertyDefinition],
    width: f32,
    most_columns: usize,
) -> GridShape {
    let widest = ((width / MINIMUM_CELL).floor() as usize)
        .clamp(1, most_columns.max(1))
        .min(fields.len().max(1));
    for columns in (1..=widest).rev() {
        let cell_width = width / columns as f32;
        for compact in [false, true] {
            let label_width = widest_label(ui, fields, compact);
            if property_row_width_for_label(label_width) <= cell_width {
                return GridShape {
                    columns,
                    compact,
                    label_width,
                };
            }
        }
    }
    GridShape {
        columns: 1,
        compact: true,
        label_width: widest_label(ui, fields, true),
    }
}

/// The width of the widest label among the fields that share a cell's width,
/// in one spelling, with the air it keeps from its value.
///
/// One spelling for the whole grid: rows that spelled some names out and
/// abbreviated others would read as two kinds of field. A field that takes the
/// whole row is not asked, because its row is wider than any cell.
fn widest_label(ui: &Ui, fields: &[&PropertyDefinition], compact: bool) -> f32 {
    fields
        .iter()
        .filter(|definition| !spans_the_row(definition))
        .map(|definition| label_width(ui, &row_label(definition, compact)))
        .fold(0.0_f32, f32::max)
        + LABEL_AIR
}

fn label_width(ui: &Ui, label: &str) -> f32 {
    ui.painter()
        .layout_no_wrap(
            label.to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            egui::Color32::PLACEHOLDER,
        )
        .size()
        .x
}

/// How tall a grid of `fields` is in `width`.
pub(super) fn grid_height(ui: &Ui, fields: &[&PropertyDefinition], width: f32) -> f32 {
    if fields.is_empty() {
        return 0.0;
    }
    let columns = grid_shape(ui, fields, width, MAXIMUM_GRID_COLUMNS).columns;
    grid_cells(fields, columns).1 as f32 * row_pitch(ui)
}

/// Whether a field takes a whole row of the grid.
///
/// Free text does: a data file's path or a bit pattern is as long as its author
/// made it, and a third of a row shows the first fourteen characters of
/// `meas/bridge_step_c07.csv` and hides the part that tells two files apart.
/// A number is a few characters whatever it is.
fn spans_the_row(definition: &PropertyDefinition) -> bool {
    matches!(definition.default_value, PropertyValue::String(_))
}

/// Where each field sits, as `(row, column, columns spanned)`, and how many
/// rows the grid has.
///
/// One walk shared by [`grid_height`] and [`field_grid`], so the height the
/// band was given and the rows the painter fills cannot come apart.
fn grid_cells(
    fields: &[&PropertyDefinition],
    columns: usize,
) -> (Vec<(usize, usize, usize)>, usize) {
    let (mut row, mut column) = (0, 0);
    let mut cells = Vec::with_capacity(fields.len());
    for definition in fields {
        if spans_the_row(definition) {
            if column > 0 {
                row += 1;
            }
            cells.push((row, 0, columns));
            row += 1;
            column = 0;
        } else {
            cells.push((row, column, 1));
            column += 1;
            if column == columns {
                row += 1;
                column = 0;
            }
        }
    }
    (cells, row + usize::from(column > 0))
}

/// Lay `fields` into a grid, in the sheet's order, left to right and then down.
///
/// Every cell is a rectangle this function computed, so the grid is as tall as
/// [`grid_height`] said it would be whatever a row inside it decides to draw.
pub(super) fn field_grid(
    ui: &mut Ui,
    state: &AppState,
    stage: &Stage,
    fields: &[&PropertyDefinition],
    actions: &mut Vec<StageAction>,
) {
    field_grid_of(ui, state, stage, fields, MAXIMUM_GRID_COLUMNS, actions);
}

/// [`field_grid`] held to `most_columns`.
fn field_grid_of(
    ui: &mut Ui,
    state: &AppState,
    stage: &Stage,
    fields: &[&PropertyDefinition],
    most_columns: usize,
    actions: &mut Vec<StageAction>,
) {
    if fields.is_empty() {
        return;
    }
    let width = ui.available_width();
    let shape = grid_shape(ui, fields, width, most_columns);
    let (cells, rows) = grid_cells(fields, shape.columns);
    let pitch = row_pitch(ui);
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(width, rows as f32 * pitch), egui::Sense::hover());
    let cell_width = rect.width() / shape.columns as f32;
    for (definition, (row, column, span)) in fields.iter().zip(cells) {
        let cell = Rect::from_min_size(
            egui::pos2(
                rect.left() + column as f32 * cell_width,
                rect.top() + row as f32 * pitch,
            ),
            Vec2::new(cell_width * span as f32, pitch),
        );
        let mut child = ui.new_child(
            UiBuilder::new()
                .id_salt(("workbench.stimulus.program.cell", definition.name.as_str()))
                .max_rect(cell)
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        child.set_clip_rect(ui.clip_rect().intersect(cell));
        // A field with the whole row to itself has room for its name, and its
        // value starts where the column under it starts when the name allows.
        let spelled_out = span > 1 || !shape.compact;
        let label = row_label(definition, !spelled_out);
        let label_column = shape
            .label_width
            .max(label_width(&child, &label) + LABEL_AIR);
        field_row(
            &mut child,
            state,
            stage,
            definition,
            &label,
            label_column,
            actions,
        );
    }
}

/// The operating point and the small-signal layer.
fn bias_column(
    ui: &mut Ui,
    state: &AppState,
    stage: &Stage,
    sheet: &PropertySheet,
    actions: &mut Vec<StageAction>,
) {
    let messages = state.ui.messages();
    section_header(ui, &messages.text(MessageId::StimulusBiasColumn), None);
    ui.add_space(HEAD_GAP);
    let fields = bias_fields(sheet);
    if fields.is_empty() {
        note(ui, &messages.text(MessageId::StimulusNoBiasFields));
    } else {
        // One column whatever the width: three fields side by side would be
        // read as one triple, and they are three separate layers of a source.
        field_grid_of(ui, state, stage, &fields, 1, actions);
    }
}

/// The bias fields this sheet declares, in the column's order.
fn bias_fields(sheet: &PropertySheet) -> Vec<&PropertyDefinition> {
    BIAS_FIELDS
        .iter()
        .filter_map(|name| sheet.get(name))
        .collect()
}

/// What [`bias_column`] will take.
fn bias_height(ui: &Ui, state: &AppState, sheet: &PropertySheet, width: f32) -> f32 {
    let fields = bias_fields(sheet);
    PANEL_SECTION_H
        + HEAD_GAP
        + if fields.is_empty() {
            note_height(
                ui,
                &state.ui.messages().text(MessageId::StimulusNoBiasFields),
                width,
            )
        } else {
            fields.len() as f32 * row_pitch(ui)
        }
}

/// What [`shape_column`] will take.
fn shape_height(
    ui: &Ui,
    state: &AppState,
    stage: &Stage,
    sheet: &PropertySheet,
    width: f32,
) -> f32 {
    PANEL_SECTION_H
        + HEAD_GAP
        + match stage.working.family() {
            StimulusFamily::Dc | StimulusFamily::Ac => note_height(
                ui,
                &state.ui.messages().text(MessageId::StimulusDcShapeNote),
                width,
            ),
            StimulusFamily::Pwl => pwl::points_height(ui, stage, sheet, width),
            StimulusFamily::PwlFile => pwl::file_height(ui, state, stage, sheet, width),
            _ => grid_height(ui, &shape_fields(sheet, stage), width),
        }
}

/// The waveform's own fields, or the editor that replaces them.
fn shape_column(
    ui: &mut Ui,
    state: &AppState,
    stage: &Stage,
    sheet: &PropertySheet,
    actions: &mut Vec<StageAction>,
) {
    let messages = state.ui.messages();
    let family = stage.working.family();
    section_header(
        ui,
        &messages.format(
            MessageId::StimulusShapeColumn,
            &[("family", family.label())],
        ),
        None,
    );
    ui.add_space(HEAD_GAP);
    match family {
        StimulusFamily::Dc | StimulusFamily::Ac => {
            note(ui, &messages.text(MessageId::StimulusDcShapeNote));
        }
        StimulusFamily::Pwl => pwl::points_editor(ui, state, stage, sheet, actions),
        StimulusFamily::PwlFile => pwl::file_editor(ui, state, stage, sheet, actions),
        _ => field_grid(ui, state, stage, &shape_fields(sheet, stage), actions),
    }
}

/// The sheet's own fields for this family's waveform, in its declared order.
///
/// Which categories those are is derived from the sheet rather than listed:
/// the family's primary field is the first positional argument of its card, so
/// the category that field sits in is the card's own group. Two families put
/// the positional tail of the same card in a second group — `PWL FILE`'s
/// import transform and `TRNOISE`'s random-telegraph triple — and those are
/// named, because nothing in the sheet says they belong to the card above
/// them.
pub(super) fn shape_fields<'a>(
    sheet: &'a PropertySheet,
    stage: &Stage,
) -> Vec<&'a PropertyDefinition> {
    let primary = crate::properties::property_bridge::get_primary_property_name(
        stage.working.component_type(),
    );
    let mut categories = sheet
        .get(primary)
        .map(|definition| definition.category.clone())
        .into_iter()
        .collect::<Vec<_>>();
    match stage.working.family() {
        StimulusFamily::PwlFile => categories.push("Scaling".to_owned()),
        StimulusFamily::Trnoise => categories.push("Random telegraph".to_owned()),
        _ => {}
    }
    sheet
        .iter()
        .filter(|definition| categories.contains(&definition.category))
        .filter(|definition| !BIAS_FIELDS.contains(&definition.name.as_str()))
        .filter(|definition| definition.name != "name" && definition.name != "symbol")
        .collect()
}

/// One field, in whichever control its declared type calls for.
///
/// A row narrower than its own two columns is skipped rather than painted:
/// the property rows derive their value column by subtracting the label
/// column from the row, and below this width that subtraction is negative.
/// The instrument's own geometry never gets there — both program columns are
/// at least [`MINIMUM_COLUMN`] wide — so this is about a host that hands the
/// stage less than the workspace's smallest supported viewport.
fn field_row(
    ui: &mut Ui,
    state: &AppState,
    stage: &Stage,
    definition: &PropertyDefinition,
    label: &str,
    label_column: f32,
    actions: &mut Vec<StageAction>,
) {
    if ui.available_width() < 96.0 {
        return;
    }
    // The grid chose a shape every label fits; a host narrower than that still
    // gets an ellipsis rather than a label sliced at the column's edge.
    let label = RowLabel {
        text: crate::workbench::design_system::elide_text(
            ui,
            label,
            &theme::sans(tokens::FS_0, FontWeight::Regular),
            label_column,
        ),
        column: label_column,
    };
    match &definition.default_value {
        PropertyValue::Enum { options, .. } => {
            enum_row(ui, stage, definition, &label, options, actions);
        }
        PropertyValue::Boolean(_) => {
            boolean_row(ui, state, stage, definition, &label, actions);
        }
        _ => text_row(ui, state, stage, definition, &label, actions),
    }
}

/// What a row paints for its name, and the width of the column it paints it in.
struct RowLabel {
    text: String,
    column: f32,
}

/// The label a row carries: the sheet's display name, marked when the field is
/// one the card cannot be written without.
///
/// `compact` is the spelling for a cell too narrow for that name: the card's
/// own word for the field, which the sheet already states in parentheses
/// (`Rise Time (TR)` is `TR`). It is read out of the display name rather than
/// kept in a list here, so a field the sheet renames is renamed in both
/// spellings, and a name with no such word keeps the name it has. The full
/// name stays what the field announces and what its hover leads with.
fn row_label(definition: &PropertyDefinition, compact: bool) -> String {
    let name = if compact {
        card_word(&definition.display_name).unwrap_or(&definition.display_name)
    } else {
        &definition.display_name
    };
    if definition.required {
        format!("{name} *")
    } else {
        name.to_owned()
    }
}

/// The card's word for a field, where its display name ends with one.
fn card_word(display_name: &str) -> Option<&str> {
    let inner = display_name.trim_end().strip_suffix(')')?;
    let word = &inner[inner.rfind('(')? + 1..];
    (!word.is_empty()).then_some(word)
}

/// A free-text field, hinted with what the engine does when it is empty.
fn text_row(
    ui: &mut Ui,
    state: &AppState,
    stage: &Stage,
    definition: &PropertyDefinition,
    label: &RowLabel,
    actions: &mut Vec<StageAction>,
) {
    let field = definition.name.as_str();
    let editing = stage.editing_field.as_deref() == Some(field);
    let mut text = if editing {
        state
            .workbench
            .stimulus_editor
            .field_text(stage.saved.name(), field)
            .unwrap_or_default()
            .to_owned()
    } else {
        read_field(&stage.working, field)
    };
    let invalid = !text.trim().is_empty()
        && crate::quantity::parse_engineering_value(&text).is_err()
        && !text.trim_start().starts_with('{')
        && matches!(
            definition.default_value,
            PropertyValue::Number { .. } | PropertyValue::Expression(_)
        );
    let response = property_row_input_with_hint_labelled(
        ui,
        &label.text,
        &mut text,
        &default_hint(definition),
        invalid,
        label.column,
    );
    // Whatever the cell had room to paint, the field is announced by its name.
    let announced = row_label(definition, false);
    let abbreviated = announced != label.text;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(announced.as_str());
    });
    let response = match hover_text(stage, definition, abbreviated) {
        Some(hint) => response.on_hover_text(hint),
        None => response,
    };
    if response.lost_focus() {
        actions.push(StageAction::CommitField {
            field: field.to_owned(),
            value: text,
        });
    } else if response.changed() || response.gained_focus() {
        actions.push(StageAction::TypeField {
            field: field.to_owned(),
            value: text,
        });
    }
}

/// A field whose sheet declares its choices.
fn enum_row(
    ui: &mut Ui,
    stage: &Stage,
    definition: &PropertyDefinition,
    label: &RowLabel,
    options: &[String],
    actions: &mut Vec<StageAction>,
) {
    let field = definition.name.as_str();
    let current = read_field(&stage.working, field);
    let mut selected = if current.trim().is_empty() {
        options.first().cloned().unwrap_or_default()
    } else {
        current
    };
    let choices = options
        .iter()
        .map(|option| (option.clone(), option.clone()))
        .collect::<Vec<_>>();
    if property_row_combo_labelled(
        ui,
        &label.text,
        ("workbench.stimulus.program.enum", field),
        &mut selected,
        &choices,
        true,
        label.column,
    ) {
        actions.push(StageAction::CommitField {
            field: field.to_owned(),
            value: selected,
        });
    }
}

/// A switch, offered as the two words the card is written with.
fn boolean_row(
    ui: &mut Ui,
    state: &AppState,
    stage: &Stage,
    definition: &PropertyDefinition,
    label: &RowLabel,
    actions: &mut Vec<StageAction>,
) {
    let messages = state.ui.messages();
    let field = definition.name.as_str();
    let authored = read_field(&stage.working, field);
    let default_on = matches!(definition.default_value, PropertyValue::Boolean(true));
    let on = if authored.trim().is_empty() {
        default_on
    } else {
        authored.eq_ignore_ascii_case("true") || authored.trim() == "1"
    };
    let yes = messages.text(MessageId::StimulusSwitchOn);
    let no = messages.text(MessageId::StimulusSwitchOff);
    let mut selected = if on {
        "true".to_owned()
    } else {
        "false".to_owned()
    };
    let choices = vec![("true".to_owned(), yes), ("false".to_owned(), no)];
    if property_row_combo_labelled(
        ui,
        &label.text,
        ("workbench.stimulus.program.switch", field),
        &mut selected,
        &choices,
        true,
        label.column,
    ) {
        actions.push(StageAction::CommitField {
            field: field.to_owned(),
            value: selected,
        });
    }
}

/// What the engine does with this field when it is left empty.
///
/// The sheet's own default, which is what the netlister writes into the card's
/// positional slot; the parser's own substitutions are stated by the audit
/// strip, which reads them from the engine contract.
pub(super) fn default_hint(definition: &PropertyDefinition) -> String {
    match &definition.default_value {
        PropertyValue::Expression(expression) => expression.clone(),
        other => other.display_string(),
    }
}

/// The hover text a field carries: its name when the cell abbreviated it, its
/// own description, plus the engine contract's line about it where there is
/// one.
fn hover_text(stage: &Stage, definition: &PropertyDefinition, abbreviated: bool) -> Option<String> {
    let mut lines = Vec::new();
    if abbreviated {
        lines.push(definition.display_name.clone());
    }
    if !definition.description.trim().is_empty() {
        lines.push(definition.description.clone());
    }
    if let Some(unit) = definition.unit.as_deref().filter(|unit| !unit.is_empty()) {
        lines.push(format!("unit {unit}"));
    }
    for finding in &stage.findings {
        if finding.message.contains(definition.display_name.as_str()) {
            lines.push(finding.message.clone());
        }
    }
    (!lines.is_empty()).then(|| lines.join("\n"))
}

/// Inset of a note from its column's edges, matching a property row's label.
const NOTE_INSET: f32 = 10.0;
const NOTE_PAD: f32 = 4.0;

/// One line of prose inside a column, where a field would otherwise stand.
///
/// Painted into a rectangle [`note_height`] measured, so a note that wraps
/// takes the height the band was told it would.
pub(super) fn note(ui: &mut Ui, text: &str) {
    let tokens = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let galley = note_galley(ui, text, width);
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(width, galley.size().y + 2.0 * NOTE_PAD),
        egui::Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text));
    ui.painter().galley(
        egui::pos2(rect.left() + NOTE_INSET, rect.top() + NOTE_PAD),
        galley,
        tokens.color.text_dim,
    );
}

/// What [`note`] will take in a column `width` wide.
pub(super) fn note_height(ui: &Ui, text: &str, width: f32) -> f32 {
    note_galley(ui, text, width).size().y + 2.0 * NOTE_PAD
}

fn note_galley(ui: &Ui, text: &str, width: f32) -> std::sync::Arc<egui::Galley> {
    ui.painter().layout(
        text.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        Tokens::get(ui.ctx()).color.text_dim,
        (width - 2.0 * NOTE_INSET).max(40.0),
    )
}

/// The unit a definition of this kind is measured in.
pub(super) const fn quantity_unit(kind: StimulusKind) -> &'static str {
    match kind {
        StimulusKind::Voltage => "V",
        StimulusKind::Current => "A",
    }
}
