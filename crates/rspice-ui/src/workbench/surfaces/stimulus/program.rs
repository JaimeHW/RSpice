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

use egui::{Rect, Ui};

use crate::state::stimulus_library::definition::{StimulusFamily, StimulusKind};
use crate::state::{PropertyDefinition, PropertySheet, PropertyValue};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::state::read_field;
use crate::workbench::{AppState, MessageId};

use super::super::super::design_system::{
    property_row_combo, property_row_input_with_hint, section_header,
};
use super::pwl;
use super::{Stage, StageAction};

/// Fields the bias column owns, whichever family's sheet declares them.
const BIAS_FIELDS: [&str; 3] = ["dc", "ac", "acphase"];
/// The column split. The shape has more fields than the bias, so it takes the
/// larger share; below this width the two stack instead.
const BIAS_FRACTION: f32 = 0.34;
const STACK_BELOW: f32 = 620.0;
/// The narrowest a property column can be and still show a label beside a
/// value. The two-column split is only taken well above twice this, so the
/// clamp is a statement of the contract rather than a fallback.
const MINIMUM_COLUMN: f32 = 200.0;

pub(super) fn show(ui: &mut Ui, state: &AppState, stage: &Stage, actions: &mut Vec<StageAction>) {
    let Some(sheet) = state
        .property_registry
        .get(stage.working.component_type())
    else {
        return;
    };
    let band = ui.available_rect_before_wrap();
    if band.width() < STACK_BELOW {
        egui::ScrollArea::vertical()
            .id_salt("workbench.stimulus.program.stacked")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                bias_column(ui, state, stage, sheet, actions);
                shape_column(ui, state, stage, sheet, actions);
            });
        return;
    }
    // Two measured rectangles rather than two allocations in a row. A column
    // allocated at a fixed width inside a row can be handed less than it asked
    // for, and what is left for the column beside it is then nothing at all —
    // which is not a narrow property row, it is a property row of negative
    // width. Splitting the band makes both columns' widths arithmetic.
    let bias_width = (band.width() * BIAS_FRACTION).clamp(MINIMUM_COLUMN, band.width() - MINIMUM_COLUMN);
    let bias_rect = Rect::from_min_max(
        band.min,
        egui::pos2(band.left() + bias_width, band.bottom()),
    );
    let shape_rect = Rect::from_min_max(egui::pos2(bias_rect.right(), band.top()), band.max);
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
    let mut column = ui.new_child(egui::UiBuilder::new().max_rect(rect));
    column.set_clip_rect(clip.intersect(rect));
    column.set_max_size(rect.size());
    egui::ScrollArea::vertical()
        .id_salt(("workbench.stimulus.program", salt))
        .auto_shrink([false, false])
        .show(&mut column, content);
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
    let mut painted = 0;
    for name in BIAS_FIELDS {
        let Some(definition) = sheet.get(name) else {
            continue;
        };
        field_row(ui, state, stage, definition, actions);
        painted += 1;
    }
    if painted == 0 {
        note(ui, &messages.text(MessageId::StimulusNoBiasFields));
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
    match family {
        StimulusFamily::Dc | StimulusFamily::Ac => {
            note(ui, &messages.text(MessageId::StimulusDcShapeNote));
        }
        StimulusFamily::Pwl => pwl::points_editor(ui, state, stage, sheet, actions),
        StimulusFamily::PwlFile => pwl::file_editor(ui, state, stage, sheet, actions),
        _ => {
            for definition in shape_fields(sheet, stage) {
                field_row(ui, state, stage, definition, actions);
            }
        }
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
pub(super) fn field_row(
    ui: &mut Ui,
    state: &AppState,
    stage: &Stage,
    definition: &PropertyDefinition,
    actions: &mut Vec<StageAction>,
) {
    if ui.available_width() < 96.0 {
        return;
    }
    let label = row_label(definition);
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

/// The label a row carries: the sheet's display name, marked when the field is
/// one the card cannot be written without.
fn row_label(definition: &PropertyDefinition) -> String {
    if definition.required {
        format!("{} *", definition.display_name)
    } else {
        definition.display_name.clone()
    }
}

/// A free-text field, hinted with what the engine does when it is empty.
fn text_row(
    ui: &mut Ui,
    state: &AppState,
    stage: &Stage,
    definition: &PropertyDefinition,
    label: &str,
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
    let response =
        property_row_input_with_hint(ui, label, &mut text, &default_hint(definition), invalid);
    let response = match hover_text(stage, definition) {
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
    label: &str,
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
    if property_row_combo(
        ui,
        label,
        ("workbench.stimulus.program.enum", field),
        &mut selected,
        &choices,
        true,
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
    label: &str,
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
    let mut selected = if on { "true".to_owned() } else { "false".to_owned() };
    let choices = vec![("true".to_owned(), yes), ("false".to_owned(), no)];
    if property_row_combo(
        ui,
        label,
        ("workbench.stimulus.program.switch", field),
        &mut selected,
        &choices,
        true,
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

/// The hover text a field carries: its own description, plus the engine
/// contract's line about it where there is one.
fn hover_text(stage: &Stage, definition: &PropertyDefinition) -> Option<String> {
    let mut lines = Vec::new();
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

/// One line of prose inside a column, where a field would otherwise stand.
pub(super) fn note(ui: &mut Ui, text: &str) {
    let tokens = Tokens::get(ui.ctx());
    ui.add_space(2.0);
    ui.add(
        egui::Label::new(
            egui::RichText::new(text)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(tokens.color.text_dim),
        )
        .wrap(),
    );
}

/// The unit a definition of this kind is measured in.
pub(super) const fn quantity_unit(kind: StimulusKind) -> &'static str {
    match kind {
        StimulusKind::Voltage => "V",
        StimulusKind::Current => "A",
    }
}
