//! Nothing selected: the sheet is the object, and the panel edits it.
//!
//! The rows are the sheet's own properties, editable where they stand: its
//! paper size and orientation, its title block, and the title-block values
//! this sheet authors. Each edit is one Page Setup transaction, with the
//! dialog's validation and undo entry, so the two cannot disagree. Margins,
//! frames, zones and custom sizes stay in Page Setup. Project-owned values and
//! the fields the product fills in (cell, page, revision, date) are not
//! offered here: they are set for the whole project, or derived.
//!
//! What the document tab, the sheet strip, the status bar and the navigator
//! already say about the sheet is not said again, and the toolbar's commands
//! are not repeated as buttons.

use egui::Ui;

use super::{
    action_stack, checks_status, count_label, findings_problem, sheet_floating_net_findings,
    sheet_open_pin_findings,
};
use crate::schematic::view::drawing_sheet::{
    ActiveDrawingSheet, DrawingSheetOverflowSummary, drawing_sheet_overflow_summary,
    resolve_active_drawing_sheet, show_first_drawing_sheet_overflow,
};
use crate::schematic::view::frame_symbol_context;
use crate::state::{
    AuthoredDrawingSheetSize, DrawingSheetStandard, DrawingSheetTitleBlockTemplate,
    DrawingSheetTitleFieldId, DrawingSheetTitleFieldValueAuthority, SchematicPageOrientation,
    SheetId,
};
use crate::ui::icons::Icon;
use crate::ui::widgets::Button;
use crate::workbench::AppState;
use crate::workbench::app::{
    DrawingSheetEdit, apply_drawing_sheet_edit, current_size_choice_label,
    drawing_sheet_setup_available_for_state, open_drawing_sheet_overflow_review,
    open_drawing_sheet_setup_for_custom_size, standard_size_choice_label,
};
use crate::workbench::design_system::{
    Problem, problem_line, property_row_combo, property_row_input_with_hint,
    schematic_property_row as property_row, schematic_property_row_problem as property_row_problem,
};
use rspice_ui_kit::panels::section_header;

/// The Size menu's entry for a size the standard list does not hold.
const CUSTOM_SIZE: &str = "custom";

/// The title-block templates a sheet can choose for itself. The organization
/// block is assigned by the organization, never chosen here.
const TEMPLATES: [DrawingSheetTitleBlockTemplate; 4] = [
    DrawingSheetTitleBlockTemplate::Compact,
    DrawingSheetTitleBlockTemplate::Standard,
    DrawingSheetTitleBlockTemplate::Wide,
    DrawingSheetTitleBlockTemplate::None,
];

pub(super) fn sheet_panel(ui: &mut Ui, state: &mut AppState) {
    let sheet = resolve_active_drawing_sheet(state);
    let identity = sheet_identity(state);
    let editable = drawing_sheet_setup_available_for_state(state);
    let mut edit = None;
    sheet_section(ui, state, &sheet, editable, &mut edit);
    title_block_section(ui, &sheet, &identity, editable, &mut edit);
    checks_section(ui, state);
    if let Some(edit) = edit
        && let Err(error) = apply_drawing_sheet_edit(state, edit)
    {
        state.prompt(error);
    }
}

/// Paper size and orientation, and what does not fit on the sheet they make.
fn sheet_section(
    ui: &mut Ui,
    state: &mut AppState,
    sheet: &ActiveDrawingSheet,
    editable: bool,
    edit: &mut Option<DrawingSheetEdit>,
) {
    section_header(ui, "Sheet", None);
    let format = &sheet.format;
    if editable {
        let mut size = match &format.authored_size {
            AuthoredDrawingSheetSize::Standard { standard } => standard.label().to_owned(),
            AuthoredDrawingSheetSize::Custom { .. } => current_size_choice_label(format),
        };
        let mut sizes = DrawingSheetStandard::ALL
            .into_iter()
            .map(|standard| {
                (
                    standard.label().to_owned(),
                    standard_size_choice_label(standard, format),
                )
            })
            .collect::<Vec<_>>();
        sizes.push((CUSTOM_SIZE.to_owned(), "Custom size\u{2026}".to_owned()));
        if property_row_combo(ui, "Size", "sheet-inspector-size", &mut size, &sizes, true) {
            if size == CUSTOM_SIZE {
                open_drawing_sheet_setup_for_custom_size(state);
            } else if let Some(standard) = DrawingSheetStandard::ALL
                .into_iter()
                .find(|standard| standard.label() == size)
            {
                *edit = Some(DrawingSheetEdit::Size(standard));
            }
        }

        let mut orientation = orientation_label(format.orientation).to_owned();
        let orientations = [
            SchematicPageOrientation::Landscape,
            SchematicPageOrientation::Portrait,
        ];
        let choices = orientations.map(|choice| {
            (
                orientation_label(choice).to_owned(),
                orientation_label(choice).to_owned(),
            )
        });
        if property_row_combo(
            ui,
            "Orientation",
            "sheet-inspector-orientation",
            &mut orientation,
            &choices,
            true,
        ) && let Some(choice) = orientations
            .into_iter()
            .find(|choice| orientation_label(*choice) == orientation)
        {
            *edit = Some(DrawingSheetEdit::Orientation(choice));
        }
    } else {
        property_row(ui, "Size", &current_size_choice_label(format));
        property_row(ui, "Orientation", orientation_label(format.orientation));
    }

    // The canvas's own cached context: the inspector resolves nothing.
    let symbol_context = frame_symbol_context(state);
    let overflow = drawing_sheet_overflow_summary(state, &symbol_context, sheet);
    if !overflow.is_clear() {
        problem_line(
            ui,
            Problem::Warning,
            &drawing_sheet_overflow_text(&overflow),
        );
        action_stack(ui, |ui| {
            if Button::new("Show first")
                .icon(Icon::ZoomFit)
                .show(ui)
                .clicked()
            {
                show_first_drawing_sheet_overflow(state);
            }
            if Button::new("Review all\u{2026}").show(ui).clicked() {
                open_drawing_sheet_overflow_review(state);
            }
        });
    }
}

/// The title block's template and the values this sheet letters in it.
fn title_block_section(
    ui: &mut Ui,
    sheet: &ActiveDrawingSheet,
    identity: &SheetIdentity,
    editable: bool,
    edit: &mut Option<DrawingSheetEdit>,
) {
    section_header(ui, "Title block", None);
    let format = &sheet.format;
    let template = format.title_block.template;
    let managed = template == DrawingSheetTitleBlockTemplate::OrganizationManaged;
    if editable && !managed {
        let mut selected = template_name(template).to_owned();
        let choices = TEMPLATES.map(|choice| {
            (
                template_name(choice).to_owned(),
                template_name(choice).to_owned(),
            )
        });
        if property_row_combo(
            ui,
            "Template",
            "sheet-inspector-title-template",
            &mut selected,
            &choices,
            true,
        ) && let Some(choice) = TEMPLATES
            .into_iter()
            .find(|choice| template_name(*choice) == selected)
        {
            *edit = Some(DrawingSheetEdit::TitleBlock(choice));
        }
    } else {
        property_row(ui, "Template", template_name(template));
    }
    if format.title_block_substituted() {
        let printed = template_name(sheet.geometry.physical.effective_title_block_template);
        problem_line(
            ui,
            Problem::Warning,
            &format!(
                "{} doesn't fit this size; {printed} is printed.",
                template_name(template)
            ),
        );
    }
    if template == DrawingSheetTitleBlockTemplate::None {
        return;
    }

    let managed_snapshot = format.title_block.managed_template.as_ref();
    for field in format.title_block_field_order() {
        let Some(authored) = format.title_block.fields.get(&field) else {
            continue;
        };
        if !authored.visible
            || field.is_project_owned()
            || field.policy().value_authority != DrawingSheetTitleFieldValueAuthority::Authored
        {
            continue;
        }
        // What the block letters while the sheet leaves the value empty.
        let fallback = match field {
            DrawingSheetTitleFieldId::SheetTitle => sheet.sheet_name.as_str(),
            _ => field.default_value(),
        };
        let locked =
            managed && managed_snapshot.is_some_and(|snapshot| snapshot.locks_field(field));
        if editable && !locked {
            title_field_row(ui, identity, field, &authored.value, fallback, edit);
        } else {
            let shown = [authored.value.trim(), fallback]
                .into_iter()
                .find(|value| !value.is_empty())
                .unwrap_or("\u{2014}");
            property_row(ui, field.display_label(), shown);
        }
    }
}

/// A title-block value being typed, held until the field is left. It names
/// the sheet it was typed on, so a value can only ever land on that sheet.
#[derive(Clone)]
struct TitleFieldDraft {
    sheet: SheetIdentity,
    text: String,
}

/// The cell view and governed sheet the inspector is editing.
type SheetIdentity = (String, Option<SheetId>);

fn sheet_identity(state: &AppState) -> SheetIdentity {
    let key = state.workspace.content.active_key();
    let sheet = state
        .workspace
        .content
        .design_management
        .sheet_catalog(&key)
        .and_then(|catalog| catalog.active_sheet_id());
    (key, sheet)
}

fn title_field_draft_id(field: DrawingSheetTitleFieldId) -> egui::Id {
    egui::Id::new(("design-inspector-sheet-title-field", field.display_label()))
}

/// Remove and return the draft held for `field`.
fn take_title_field_draft(
    data: &mut egui::util::IdTypeMap,
    field: DrawingSheetTitleFieldId,
) -> Option<TitleFieldDraft> {
    let id = title_field_draft_id(field);
    let draft = data.get_temp::<TitleFieldDraft>(id);
    data.remove::<TitleFieldDraft>(id);
    draft
}

/// One title-block value. The sheet takes it once, as one undo step, when the
/// field is left — Enter, Tab or a click elsewhere — and Escape abandons it.
fn title_field_row(
    ui: &mut Ui,
    sheet: &SheetIdentity,
    field: DrawingSheetTitleFieldId,
    stored: &str,
    fallback: &str,
    edit: &mut Option<DrawingSheetEdit>,
) {
    let draft = ui
        .data_mut(|data| take_title_field_draft(data, field))
        .filter(|draft| draft.sheet == *sheet);
    let mut text = draft
        .as_ref()
        .map_or_else(|| stored.to_owned(), |draft| draft.text.clone());
    let response = ui
        .push_id(field.display_label(), |ui| {
            property_row_input_with_hint(ui, field.display_label(), &mut text, fallback, false)
        })
        .inner;
    if response.has_focus() {
        let sheet = sheet.clone();
        ui.data_mut(|data| {
            data.insert_temp(title_field_draft_id(field), TitleFieldDraft { sheet, text });
        });
        return;
    }
    let abandoned = ui.input(|input| input.key_pressed(egui::Key::Escape));
    if draft.is_some() && !abandoned && text.trim() != stored {
        *edit = Some(DrawingSheetEdit::TitleField(field, text.trim().to_owned()));
    }
}

/// Commit a title-block value still being typed when the panel stops showing
/// the sheet. A click on the canvas can select something in the frame that
/// takes the field's focus, and leaving the field must not lose what was
/// typed. A value typed on another sheet than the active one is dropped
/// rather than written to the wrong sheet.
pub(super) fn commit_pending_title_field(ctx: &egui::Context, state: &mut AppState) {
    let sheet = sheet_identity(state);
    for field in DrawingSheetTitleFieldId::ALL {
        let Some(draft) = ctx.data_mut(|data| take_title_field_draft(data, field)) else {
            continue;
        };
        if draft.sheet != sheet {
            continue;
        }
        let stored = resolve_active_drawing_sheet(state)
            .format
            .title_block
            .fields
            .get(&field)
            .map(|authored| authored.value.clone())
            .unwrap_or_default();
        if draft.text.trim() == stored {
            continue;
        }
        let edit = DrawingSheetEdit::TitleField(field, draft.text.trim().to_owned());
        if let Err(error) = apply_drawing_sheet_edit(state, edit) {
            state.prompt(error);
        }
    }
}

/// This sheet's open pins and floating nets from the last check, when it
/// found any. The header says whether that check still describes the drawing.
fn checks_section(ui: &mut Ui, state: &AppState) {
    let open_pins = sheet_open_pin_findings(state);
    let floating_nets = sheet_floating_net_findings(state);
    if open_pins.is_empty() && floating_nets.is_empty() {
        return;
    }
    section_header(ui, "Checks", checks_status(state).as_deref());
    if !open_pins.is_empty() {
        property_row_problem(
            ui,
            "Connectivity",
            &count_label(open_pins.len(), "unconnected pin"),
            Some(findings_problem(&open_pins)),
        );
    }
    if !floating_nets.is_empty() {
        property_row_problem(
            ui,
            "Floating nets",
            &floating_nets.len().to_string(),
            Some(findings_problem(&floating_nets)),
        );
    }
}

const fn orientation_label(orientation: SchematicPageOrientation) -> &'static str {
    match orientation {
        SchematicPageOrientation::Landscape => "Landscape",
        SchematicPageOrientation::Portrait => "Portrait",
    }
}

const fn template_name(template: DrawingSheetTitleBlockTemplate) -> &'static str {
    match template {
        DrawingSheetTitleBlockTemplate::Compact => "RSpice compact",
        DrawingSheetTitleBlockTemplate::Standard => "RSpice standard",
        DrawingSheetTitleBlockTemplate::Wide => "RSpice wide",
        DrawingSheetTitleBlockTemplate::OrganizationManaged => "Organization block",
        DrawingSheetTitleBlockTemplate::None => "No title block",
    }
}

/// How many objects stand outside the drawing area, and where.
fn drawing_sheet_overflow_text(summary: &DrawingSheetOverflowSummary) -> String {
    let mut details = Vec::with_capacity(2);
    if summary.outside_border > 0 {
        let mut outside = format!("{} outside the border", summary.outside_border);
        if summary.off_paper > 0 {
            outside.push_str(&format!(" · {} beyond the paper edge", summary.off_paper));
        }
        details.push(outside);
    }
    if summary.title_block_collisions > 0 {
        details.push(format!(
            "{} overlapping the title block",
            summary.title_block_collisions
        ));
    }
    let headline = format!(
        "{} outside the drawing area",
        count_label(summary.finding_count(), "object")
    );
    if details.is_empty() {
        headline
    } else {
        format!("{headline}: {}", details.join(" · "))
    }
}

#[cfg(test)]
mod tests;
