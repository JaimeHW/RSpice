//! Nothing selected: the sheet is the object, and the panel edits it.
//!
//! The rows are the sheet's own properties, editable where they stand: its
//! paper size and orientation, its title block, and every value its title
//! block shows that someone types — this sheet's own, then the document's,
//! which every sheet in the project letters alike. Each edit is one Page
//! Setup transaction, with the dialog's validation and undo entry, so the two
//! cannot disagree. Below them sit the grid and net-name characters every
//! sheet of the schematic shares, which Preferences only chooses for new
//! schematics. The frame and its zones, custom sizes and which fields a block
//! shows stay in Page Setup. The fields RSpice fills in itself (project,
//! cell, page, paper size) are not offered: nothing types them.
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
    grid_pitch_label, resolve_active_drawing_sheet, show_first_drawing_sheet_overflow,
};
use crate::schematic::view::frame_symbol_context;
use crate::state::{
    AuthoredDrawingSheetSize, DrawingSheetFrameConvention, DrawingSheetReleaseStatus,
    DrawingSheetStandard, DrawingSheetTitleBlockTemplate, DrawingSheetTitleFieldId,
    DrawingSheetTitleFieldValueAuthority, NetNamingPolicy, SchematicGridPitch,
    SchematicPageOrientation, SchematicSheetFormat, SheetId,
};
use crate::ui::icons::Icon;
use crate::ui::widgets::Button;
use crate::workbench::AppState;
use crate::workbench::app::{
    DrawingSheetEdit, apply_drawing_sheet_edit, current_size_choice_label,
    drawing_sheet_setup_available_for_state, open_drawing_sheet_overflow_review,
    open_drawing_sheet_setup_for_custom_size, standard_size_choice_label,
};
use crate::workbench::app_state::schematic_preferences::NET_NAME_CHARACTERS;
use crate::workbench::design_system::{
    Problem, problem_line, property_row_combo, property_row_input_with_hint,
    schematic_property_row as property_row, schematic_property_row_problem as property_row_problem,
};
use rspice_ui_kit::panels::{section_header, section_header_explained};

/// The Size menu's entry for a size the standard list does not hold.
const CUSTOM_SIZE: &str = "custom";

/// The title-block templates `format`'s sheet can choose for itself: ISO
/// 7200's form, ASME Y14.1's on an ANSI or ARCH sheet, or none. A block the
/// sheet is already on stays offered beside them, and the organization block
/// is assigned by the organization, never chosen here.
fn template_choices(format: &SchematicSheetFormat) -> Vec<DrawingSheetTitleBlockTemplate> {
    let current = format.title_block.template;
    let asme_format = format.frame_convention() == DrawingSheetFrameConvention::AsmeY141;
    [
        DrawingSheetTitleBlockTemplate::Compact,
        DrawingSheetTitleBlockTemplate::Standard,
        DrawingSheetTitleBlockTemplate::AsmeY141,
        DrawingSheetTitleBlockTemplate::None,
    ]
    .into_iter()
    .filter(|choice| match choice {
        DrawingSheetTitleBlockTemplate::Compact => current == *choice,
        DrawingSheetTitleBlockTemplate::AsmeY141 => asme_format || current == *choice,
        _ => true,
    })
    .collect()
}

pub(super) fn sheet_panel(ui: &mut Ui, state: &mut AppState) {
    let sheet = resolve_active_drawing_sheet(state);
    let identity = sheet_identity(state);
    let editable = drawing_sheet_setup_available_for_state(state);
    let mut edit = None;
    let mut schematic_edit = None;
    sheet_section(ui, state, &sheet, editable, &mut edit);
    title_block_section(ui, &sheet, &identity, editable, &mut edit);
    document_section(ui, state, &sheet, &identity, editable, &mut edit);
    schematic_section(ui, state, editable, &mut schematic_edit);
    checks_section(ui, state);
    if let Some(edit) = edit
        && let Err(error) = apply_drawing_sheet_edit(state, edit)
    {
        state.prompt(error);
    }
    if let Some(edit) = schematic_edit {
        apply_schematic_edit(state, edit);
    }
}

/// A change to a property the whole schematic shares, every sheet of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SchematicEdit {
    Grid(SchematicGridPitch),
    NetNameCharacters(NetNamingPolicy),
}

/// The grid and net-name characters of the open schematic. Preferences only
/// seeds them for new schematics, so this is where an open one changes.
fn schematic_section(
    ui: &mut Ui,
    state: &AppState,
    editable: bool,
    edit: &mut Option<SchematicEdit>,
) {
    section_header(ui, "Schematic", None);
    let policy = &state.schematic.document().document_policy;
    let naming = NET_NAME_CHARACTERS
        .into_iter()
        .find(|(choice, _)| *choice == policy.net_naming)
        .map_or("\u{2014}", |(_, label)| label);
    if !editable {
        property_row(ui, "Grid", &grid_pitch_label(policy.grid_pitch));
        property_row(ui, "Net names", naming);
        return;
    }

    let mut grid = grid_pitch_label(policy.grid_pitch);
    let grids =
        SchematicGridPitch::ALL.map(|pitch| (grid_pitch_label(pitch), grid_pitch_label(pitch)));
    if property_row_combo(ui, "Grid", "sheet-inspector-grid", &mut grid, &grids, true)
        && let Some(pitch) = SchematicGridPitch::ALL
            .into_iter()
            .find(|pitch| grid_pitch_label(*pitch) == grid)
        && pitch != policy.grid_pitch
    {
        *edit = Some(SchematicEdit::Grid(pitch));
    }

    let mut selected = naming.to_owned();
    let choices = NET_NAME_CHARACTERS.map(|(_, label)| (label.to_owned(), label.to_owned()));
    if property_row_combo(
        ui,
        "Net names",
        "sheet-inspector-net-names",
        &mut selected,
        &choices,
        true,
    ) && let Some((choice, _)) = NET_NAME_CHARACTERS
        .into_iter()
        .find(|(_, label)| *label == selected)
        && choice != policy.net_naming
    {
        *edit = Some(SchematicEdit::NetNameCharacters(choice));
    }
}

/// Apply a schematic-wide change as one undo step. A new grid moves the
/// editor's snap with it, as Grid, snap and wire routing does.
fn apply_schematic_edit(state: &mut AppState, edit: SchematicEdit) {
    let changed = match edit {
        SchematicEdit::Grid(pitch) => {
            let changed = state.schematic.change_grid_pitch(pitch);
            if changed {
                state.schematic.reconcile_grid_pitch_runtime();
                state.ui.schematic_snap.grid_size =
                    state.schematic.session.editor.snap_engine.grid_size;
            }
            changed
        }
        SchematicEdit::NetNameCharacters(policy) => state.schematic.change_net_naming(policy),
    };
    if changed {
        state.sync_active_schematic_to_workspace();
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
        let templates = template_choices(format);
        let mut selected = template_name(format, template).to_owned();
        let choices = templates
            .iter()
            .map(|choice| {
                (
                    template_name(format, *choice).to_owned(),
                    template_name(format, *choice).to_owned(),
                )
            })
            .collect::<Vec<_>>();
        if property_row_combo(
            ui,
            "Template",
            "sheet-inspector-title-template",
            &mut selected,
            &choices,
            true,
        ) && let Some(choice) = templates
            .into_iter()
            .find(|choice| template_name(format, *choice) == selected)
        {
            *edit = Some(DrawingSheetEdit::TitleBlock(choice));
        }
    } else {
        property_row(ui, "Template", template_name(format, template));
    }
    if format.title_block_substituted() {
        problem_line(
            ui,
            Problem::Warning,
            &match sheet.geometry.physical.effective_title_block_template {
                DrawingSheetTitleBlockTemplate::None => format!(
                    "{} doesn't fit this sheet's drawing area; no title block is printed.",
                    template_name(format, template)
                ),
                printed => format!(
                    "{} doesn't fit this size; {} is printed.",
                    template_name(format, template),
                    template_name(format, printed)
                ),
            },
        );
    }
    if template == DrawingSheetTitleBlockTemplate::None {
        return;
    }

    // The form's fields in the order it is read, named as it names them.
    for field in format.title_block_field_order() {
        let Some(authored) = format.title_block.fields.get(&field) else {
            continue;
        };
        if !format.title_field_shown(field)
            || is_document_field(field)
            || field.policy().value_authority != DrawingSheetTitleFieldValueAuthority::Authored
        {
            continue;
        }
        // What the block letters while the sheet leaves the value empty.
        let fallback = match field {
            DrawingSheetTitleFieldId::SheetTitle => sheet.sheet_name.as_str(),
            _ => field.default_value(),
        };
        if editable && !locked_by_organization(format, field) {
            // An empty date shows the form it is written in.
            let hint = if field.is_signature_date() {
                field.entry_hint()
            } else {
                fallback
            };
            title_field_row(ui, identity, field, &authored.value, hint, edit);
        } else {
            let shown = [authored.value.trim(), fallback]
                .into_iter()
                .find(|value| !value.is_empty())
                .unwrap_or("\u{2014}");
            property_row(ui, field.display_label(), shown);
        }
    }
}

/// The document's values that this sheet's title block shows: its revision,
/// date of issue and release status, and the project's owner, document number
/// and classification. Every sheet in the project letters the same values, so
/// the header says a change here changes them all.
fn document_section(
    ui: &mut Ui,
    state: &AppState,
    sheet: &ActiveDrawingSheet,
    identity: &SheetIdentity,
    editable: bool,
    edit: &mut Option<DrawingSheetEdit>,
) {
    let format = &sheet.format;
    if format.title_block.template == DrawingSheetTitleBlockTemplate::None {
        return;
    }
    let fields = format
        .title_block_field_order()
        .into_iter()
        .filter(|field| is_document_field(*field) && format.title_field_shown(*field))
        .collect::<Vec<_>>();
    if fields.is_empty() {
        return;
    }
    section_header_explained(
        ui,
        "Document",
        "All sheets",
        "Every sheet in the project letters these values, so a change here changes them all.",
    );
    let status = state
        .workspace
        .content
        .design_management
        .drawing_sheet_settings()
        .document_control
        .status;
    for field in fields {
        let writable = editable && !locked_by_organization(format, field);
        if field == DrawingSheetTitleFieldId::DocumentStatus {
            if writable {
                let mut selected = status.label().to_owned();
                let choices = DrawingSheetReleaseStatus::ALL
                    .map(|choice| (choice.label().to_owned(), choice.label().to_owned()));
                if property_row_combo(
                    ui,
                    field.display_label(),
                    "sheet-inspector-document-status",
                    &mut selected,
                    &choices,
                    true,
                ) && let Some(choice) = DrawingSheetReleaseStatus::ALL
                    .into_iter()
                    .find(|choice| choice.label() == selected)
                {
                    *edit = Some(DrawingSheetEdit::Status(choice));
                }
            } else {
                property_row(ui, field.display_label(), status.label());
            }
            continue;
        }
        let stored = stored_title_value(state, format, field);
        if writable {
            // The date is refused unless it is written the one way Page
            // Setup reads it.
            let hint = if field == DrawingSheetTitleFieldId::Date {
                "YYYY-MM-DD"
            } else {
                ""
            };
            title_field_row(ui, identity, field, &stored, hint, edit);
        } else {
            let shown = if stored.trim().is_empty() {
                "\u{2014}"
            } else {
                stored.as_str()
            };
            property_row(ui, field.display_label(), shown);
        }
        // ISO 7200's advice on a revision index, or ASME Y14.35's on the
        // ASME form.
        if field == DrawingSheetTitleFieldId::Revision
            && let Some(advisory) = format.title_block.template.revision_advisory(&stored)
        {
            problem_line(ui, Problem::Warning, advisory);
        }
    }
}

/// Whether `field` holds a value of the whole document, kept by the project
/// and lettered alike on every sheet, rather than one of this sheet's own.
const fn is_document_field(field: DrawingSheetTitleFieldId) -> bool {
    field.is_project_owned()
        || matches!(
            field,
            DrawingSheetTitleFieldId::Revision
                | DrawingSheetTitleFieldId::Date
                | DrawingSheetTitleFieldId::DocumentStatus
        )
}

/// Whether the organization's block fixes `field`, so no sheet edits it.
fn locked_by_organization(format: &SchematicSheetFormat, field: DrawingSheetTitleFieldId) -> bool {
    format.title_block.template == DrawingSheetTitleBlockTemplate::OrganizationManaged
        && format
            .title_block
            .managed_template
            .as_ref()
            .is_some_and(|snapshot| snapshot.locks_field(field))
}

/// What `field` holds where it is kept — the document control, the project,
/// or this sheet — as the panel edits it.
fn stored_title_value(
    state: &AppState,
    format: &SchematicSheetFormat,
    field: DrawingSheetTitleFieldId,
) -> String {
    let settings = state
        .workspace
        .content
        .design_management
        .drawing_sheet_settings();
    let on_sheet = || {
        format
            .title_block
            .fields
            .get(&field)
            .map(|authored| authored.value.clone())
            .unwrap_or_default()
    };
    match field {
        DrawingSheetTitleFieldId::Revision => settings.document_control.revision_index().to_owned(),
        DrawingSheetTitleFieldId::Date => settings.document_control.revision_date_utc.clone(),
        // The project's value, or one an older build left on the sheet, as
        // the title block resolves it.
        _ if field.is_project_owned() => settings
            .title_block_field_values
            .get(&field)
            .filter(|value| !value.trim().is_empty())
            .cloned()
            .unwrap_or_else(on_sheet),
        _ => on_sheet(),
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

/// One title-block value. It lands once, as one undo step, when the field is
/// left — Enter, Tab or a click elsewhere — and Escape abandons it.
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
        let format = resolve_active_drawing_sheet(state).format;
        let stored = stored_title_value(state, &format, field);
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

/// The template's one name on `format`'s sheet, as Page Setup and every
/// other surface give it.
fn template_name(
    format: &SchematicSheetFormat,
    template: DrawingSheetTitleBlockTemplate,
) -> &'static str {
    format.title_block_template_label(template)
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
