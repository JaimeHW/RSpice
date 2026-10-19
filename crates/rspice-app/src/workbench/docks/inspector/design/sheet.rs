//! Nothing selected: the sheet is the object, and the panel edits it.
//!
//! The rows are the sheet's own properties, editable where they stand: its
//! paper, frame and units, its title block and the boxes it shows, and every
//! value its title block shows that someone types — this sheet's own, then
//! the document's, which every sheet in the project letters alike. Each edit
//! is one sheet transaction, with its validation and undo entry. Below them
//! sit the grid and net-name characters every sheet of the schematic shares,
//! which Preferences only chooses for new schematics. The fields RSpice fills
//! in itself (project, cell, page, paper size) are not offered: nothing types
//! them. A value its cell cannot hold says so under its row, because the
//! block prints it cut short rather than shrinking it.
//!
//! What the document tab, the sheet strip, the status bar and the navigator
//! already say about the sheet is not said again, and the toolbar's commands
//! are not repeated as buttons.

mod format;

use egui::Ui;

use super::{
    checks_status, count_label, findings_problem, sheet_floating_net_findings,
    sheet_open_pin_findings,
};
use crate::schematic::view::drawing_sheet::{
    ActiveDrawingSheet, grid_pitch_label, resolve_active_drawing_sheet, resolved_title_block_fields,
};
use crate::state::{
    AuthoredDrawingSheetSize, DRAWING_SHEET_TITLE_VALUE_CUT_ADVISORY, DrawingSheetReleaseStatus,
    DrawingSheetTitleBlockTemplate, DrawingSheetTitleFieldId, DrawingSheetTitleFieldValueAuthority,
    NetNamingPolicy, SchematicGridPitch, SchematicSheetFormat, SheetId, drawing_sheet_title_layout,
};
use crate::workbench::AppState;
use crate::workbench::app::drawing_sheet_edit::{
    DrawingSheetEdit, apply_drawing_sheet_edit, drawing_sheet_editable, format_sheet_length,
    parse_sheet_length,
};
use crate::workbench::app_state::schematic_preferences::NET_NAME_CHARACTERS;
use crate::workbench::design_system::{
    Problem, problem_line, property_row_combo, property_row_input_with_hint,
    schematic_property_row as property_row, schematic_property_row_problem as property_row_problem,
};
use rspice_ui_kit::panels::{section_header, section_header_explained};

/// An edit the panel asks for this frame, or why what was typed is refused.
type PanelEdit = Result<DrawingSheetEdit, String>;

/// The title-block templates `format`'s sheet can choose for itself: ISO
/// 7200's form, ASME Y14.1's on an ANSI or ARCH sheet or one already on it,
/// or none. The organization block is assigned by the organization, never
/// chosen here.
fn template_choices(format: &SchematicSheetFormat) -> Vec<DrawingSheetTitleBlockTemplate> {
    let current = format.title_block.template;
    let asme_format =
        format.frame_convention() == crate::state::DrawingSheetFrameConvention::AsmeY141;
    [
        DrawingSheetTitleBlockTemplate::Standard,
        DrawingSheetTitleBlockTemplate::AsmeY141,
        DrawingSheetTitleBlockTemplate::None,
    ]
    .into_iter()
    .filter(|choice| {
        *choice != DrawingSheetTitleBlockTemplate::AsmeY141 || asme_format || current == *choice
    })
    .collect()
}

pub(super) fn sheet_panel(ui: &mut Ui, state: &mut AppState) {
    let sheet = resolve_active_drawing_sheet(state);
    let identity = sheet_identity(state);
    let editable = drawing_sheet_editable(state);
    let cut = cut_fields(state, &sheet);
    let mut edit = None;
    let mut schematic_edit = None;
    format::sheet_section(ui, state, &sheet, &identity, editable, &mut edit);
    title_block_section(ui, &sheet, &identity, editable, &cut, &mut edit);
    document_section(ui, state, &sheet, &identity, editable, &cut, &mut edit);
    schematic_section(ui, state, editable, &mut schematic_edit);
    checks_section(ui, state);
    if let Some(edit) = edit
        && let Err(error) = edit.and_then(|edit| apply_drawing_sheet_edit(state, edit))
    {
        state.prompt(error);
    }
    if let Some(edit) = schematic_edit {
        apply_schematic_edit(state, edit);
    }
}

/// The title fields the sheet's block cuts short, as the sheet prints them:
/// lettering never shrinks to fit, so each prints with an ellipsis.
fn cut_fields(state: &AppState, sheet: &ActiveDrawingSheet) -> Vec<DrawingSheetTitleFieldId> {
    drawing_sheet_title_layout(&sheet.format, &sheet.geometry.physical)
        .map_or_else(Vec::new, |layout| {
            layout.cut_fields(&resolved_title_block_fields(state, sheet))
        })
}

/// Say under a field's row when its value prints cut short.
fn cut_advisory(ui: &mut Ui, cut: &[DrawingSheetTitleFieldId], field: DrawingSheetTitleFieldId) {
    if cut.contains(&field) {
        problem_line(ui, Problem::Warning, DRAWING_SHEET_TITLE_VALUE_CUT_ADVISORY);
    }
}

/// A row choosing one of `choices`, each a value and how the row names it,
/// or stating `current` where the sheet cannot be edited. Returns the value
/// chosen this frame when it is another than `current`.
fn choice_row<T: Copy + PartialEq>(
    ui: &mut Ui,
    editable: bool,
    label: &str,
    id: &str,
    current: T,
    choices: &[(T, String)],
) -> Option<T> {
    let position = choices.iter().position(|(choice, _)| *choice == current);
    if !editable {
        let shown = position.map_or("\u{2014}", |index| choices[index].1.as_str());
        property_row(ui, label, shown);
        return None;
    }
    let mut selected = position.map_or_else(String::new, |index| index.to_string());
    let options = choices
        .iter()
        .enumerate()
        .map(|(index, (_, name))| (index.to_string(), name.clone()))
        .collect::<Vec<_>>();
    if !property_row_combo(ui, label, id, &mut selected, &options, true) {
        return None;
    }
    selected
        .parse::<usize>()
        .ok()
        .and_then(|index| choices.get(index))
        .map(|(choice, _)| *choice)
        .filter(|choice| *choice != current)
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
    let grids = SchematicGridPitch::ALL.map(|pitch| (pitch, grid_pitch_label(pitch)));
    if let Some(pitch) = choice_row(
        ui,
        editable,
        "Grid",
        "sheet-inspector-grid",
        policy.grid_pitch,
        &grids,
    ) {
        *edit = Some(SchematicEdit::Grid(pitch));
    }
    let characters = NET_NAME_CHARACTERS.map(|(choice, label)| (choice, label.to_owned()));
    if let Some(choice) = choice_row(
        ui,
        editable,
        "Net names",
        "sheet-inspector-net-names",
        policy.net_naming,
        &characters,
    ) {
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

/// The title block's template, the boxes it shows that its form leaves
/// optional, and the values this sheet letters in it.
fn title_block_section(
    ui: &mut Ui,
    sheet: &ActiveDrawingSheet,
    identity: &SheetIdentity,
    editable: bool,
    cut: &[DrawingSheetTitleFieldId],
    edit: &mut Option<PanelEdit>,
) {
    section_header(ui, "Title block", None);
    let format = &sheet.format;
    let template = format.title_block.template;
    let managed = template == DrawingSheetTitleBlockTemplate::OrganizationManaged;
    let templates = template_choices(format)
        .into_iter()
        .map(|choice| (choice, template_name(format, choice).to_owned()))
        .collect::<Vec<_>>();
    if managed {
        property_row(ui, "Template", template_name(format, template));
    } else if let Some(choice) = choice_row(
        ui,
        editable,
        "Template",
        "sheet-inspector-title-template",
        template,
        &templates,
    ) {
        *edit = Some(Ok(DrawingSheetEdit::TitleBlock(choice)));
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
    if !managed {
        optional_boxes_row(ui, format, editable, edit);
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
            if let Some(typed) = text_row(
                ui,
                identity,
                SheetText::Field(field),
                field.display_label(),
                &authored.value,
                hint,
            ) {
                *edit = Some(Ok(DrawingSheetEdit::TitleField(field, typed)));
            }
        } else {
            let shown = [authored.value.trim(), fallback]
                .into_iter()
                .find(|value| !value.is_empty())
                .unwrap_or("\u{2014}");
            property_row(ui, field.display_label(), shown);
        }
        cut_advisory(ui, cut, field);
    }
}

/// The boxes on the sheet's form that the form leaves optional, which a sheet
/// shows or not. One row holds them all: its menu shows or hides one box at
/// a time, and the row says how many are shown.
fn optional_boxes_row(
    ui: &mut Ui,
    format: &SchematicSheetFormat,
    editable: bool,
    edit: &mut Option<PanelEdit>,
) {
    let template = format.title_block.template;
    let boxes = format
        .title_block_field_order()
        .into_iter()
        .filter(|field| !template.requires_field(*field))
        .map(|field| (field, format.title_field_shown(field)))
        .collect::<Vec<_>>();
    if boxes.is_empty() {
        return;
    }
    let shown = boxes.iter().filter(|(_, shown)| *shown).count();
    let mut summary = format!("{shown} of {} shown", boxes.len());
    if !editable {
        property_row(ui, "Optional boxes", &summary);
        return;
    }
    let options = boxes
        .iter()
        .enumerate()
        .map(|(index, (field, shown))| {
            let action = if *shown { "Hide" } else { "Show" };
            (
                index.to_string(),
                format!("{action} {}", field.display_label()),
            )
        })
        .collect::<Vec<_>>();
    if property_row_combo(
        ui,
        "Optional boxes",
        "sheet-inspector-optional-boxes",
        &mut summary,
        &options,
        true,
    ) && let Some((field, shown)) = summary
        .parse::<usize>()
        .ok()
        .and_then(|index| boxes.get(index))
    {
        *edit = Some(Ok(DrawingSheetEdit::BoxShown(*field, !shown)));
    }
}

/// The document's values that this sheet's title block shows: its revision,
/// date of issue and release status, the project's owner, document number,
/// classification, language, CAGE code and contract number, and the change
/// its revision answers to. Every sheet in the project letters the same
/// values, so the header says a change here changes them all.
fn document_section(
    ui: &mut Ui,
    state: &AppState,
    sheet: &ActiveDrawingSheet,
    identity: &SheetIdentity,
    editable: bool,
    cut: &[DrawingSheetTitleFieldId],
    edit: &mut Option<PanelEdit>,
) {
    let format = &sheet.format;
    if format.title_block.template == DrawingSheetTitleBlockTemplate::None {
        return;
    }
    section_header_explained(
        ui,
        "Document",
        "All sheets",
        "Every sheet in the project letters these values, so a change here changes them all.",
    );
    let control = &state
        .workspace
        .content
        .design_management
        .drawing_sheet_settings()
        .document_control;
    let fields = format
        .title_block_field_order()
        .into_iter()
        .filter(|field| is_document_field(*field) && format.title_field_shown(*field));
    for field in fields {
        let writable = editable && !locked_by_organization(format, field);
        if field == DrawingSheetTitleFieldId::DocumentStatus {
            let statuses =
                DrawingSheetReleaseStatus::ALL.map(|choice| (choice, choice.label().to_owned()));
            if let Some(status) = choice_row(
                ui,
                writable,
                field.display_label(),
                "sheet-inspector-document-status",
                control.status,
                &statuses,
            ) {
                *edit = Some(Ok(DrawingSheetEdit::Status(status)));
            }
            cut_advisory(ui, cut, field);
            continue;
        }
        let stored = stored_title_value(state, format, field);
        if writable {
            // The date is refused unless it is written the one way the
            // document control reads it.
            let hint = if field == DrawingSheetTitleFieldId::Date {
                "YYYY-MM-DD"
            } else {
                ""
            };
            if let Some(typed) = text_row(
                ui,
                identity,
                SheetText::Field(field),
                field.display_label(),
                &stored,
                hint,
            ) {
                *edit = Some(Ok(DrawingSheetEdit::TitleField(field, typed)));
            }
        } else {
            property_row(ui, field.display_label(), or_dash(&stored));
        }
        // ISO 7200's advice on a revision index, or ASME Y14.35's on the
        // ASME form.
        if field == DrawingSheetTitleFieldId::Revision
            && let Some(advisory) = format.title_block.template.revision_advisory(&stored)
        {
            problem_line(ui, Problem::Warning, advisory);
        }
        cut_advisory(ui, cut, field);
    }
    if editable {
        if let Some(typed) = text_row(
            ui,
            identity,
            SheetText::ChangeReference,
            SheetText::ChangeReference.label(),
            &control.change_reference,
            "ECO or ticket",
        ) {
            *edit = Some(Ok(DrawingSheetEdit::ChangeReference(typed)));
        }
    } else {
        property_row(
            ui,
            SheetText::ChangeReference.label(),
            or_dash(&control.change_reference),
        );
    }
}

/// `value`, or a dash where it is empty.
fn or_dash(value: &str) -> &str {
    if value.trim().is_empty() {
        "\u{2014}"
    } else {
        value
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

/// A value the panel takes as typed text: a title-block value, a one-off
/// custom size's edges and name, or the document's change reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SheetText {
    Field(DrawingSheetTitleFieldId),
    Width,
    Height,
    SizeName,
    ChangeReference,
}

impl SheetText {
    /// Every value the panel takes as typed text.
    fn all() -> impl Iterator<Item = Self> {
        DrawingSheetTitleFieldId::ALL
            .into_iter()
            .map(Self::Field)
            .chain([
                Self::Width,
                Self::Height,
                Self::SizeName,
                Self::ChangeReference,
            ])
    }

    /// The row's label. The edges' rows name the sheet's unit beside it.
    const fn label(self) -> &'static str {
        match self {
            Self::Field(field) => field.display_label(),
            Self::Width => "Width",
            Self::Height => "Height",
            Self::SizeName => "Size name",
            Self::ChangeReference => "Change reference",
        }
    }

    fn draft_id(self) -> egui::Id {
        egui::Id::new(("design-inspector-sheet-text", self.label()))
    }

    /// What the value holds now, as the panel shows it for editing.
    fn stored(self, state: &AppState, format: &SchematicSheetFormat) -> String {
        let (width_um, height_um) = format.oriented_dimensions_um();
        match self {
            Self::Field(field) => stored_title_value(state, format, field),
            Self::Width => format_sheet_length(width_um, format.display_unit),
            Self::Height => format_sheet_length(height_um, format.display_unit),
            Self::SizeName => match &format.authored_size {
                AuthoredDrawingSheetSize::Custom { snapshot } => snapshot.name.clone(),
                AuthoredDrawingSheetSize::Standard { .. } => String::new(),
            },
            Self::ChangeReference => state
                .workspace
                .content
                .design_management
                .drawing_sheet_settings()
                .document_control
                .change_reference
                .clone(),
        }
    }

    /// The edit `typed` asks of the sheet `format` describes, or why it is
    /// refused.
    fn edit(self, format: &SchematicSheetFormat, typed: String) -> PanelEdit {
        let (width_um, height_um) = format.oriented_dimensions_um();
        match self {
            Self::Field(field) => Ok(DrawingSheetEdit::TitleField(field, typed)),
            Self::Width => Ok(DrawingSheetEdit::CustomSize {
                width_um: parse_sheet_length(&typed, format.display_unit)?,
                height_um,
            }),
            Self::Height => Ok(DrawingSheetEdit::CustomSize {
                width_um,
                height_um: parse_sheet_length(&typed, format.display_unit)?,
            }),
            Self::SizeName => Ok(DrawingSheetEdit::CustomSizeName(typed)),
            Self::ChangeReference => Ok(DrawingSheetEdit::ChangeReference(typed)),
        }
    }
}

/// A value being typed, held until the field is left. It names the sheet it
/// was typed on, so a value can only ever land on that sheet.
#[derive(Clone)]
struct TextDraft {
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

/// Remove and return the draft held for `text`.
fn take_text_draft(data: &mut egui::util::IdTypeMap, text: SheetText) -> Option<TextDraft> {
    let id = text.draft_id();
    let draft = data.get_temp::<TextDraft>(id);
    data.remove::<TextDraft>(id);
    draft
}

/// One typed value. It lands once, as one undo step, when the field is left
/// — Enter, Tab or a click elsewhere — and Escape abandons it. Returns the
/// text that lands, trimmed, when it differs from `stored`.
fn text_row(
    ui: &mut Ui,
    sheet: &SheetIdentity,
    text: SheetText,
    label: &str,
    stored: &str,
    hint: &str,
) -> Option<String> {
    let draft = ui
        .data_mut(|data| take_text_draft(data, text))
        .filter(|draft| draft.sheet == *sheet);
    let mut typed = draft
        .as_ref()
        .map_or_else(|| stored.to_owned(), |draft| draft.text.clone());
    let response = ui
        .push_id(text.label(), |ui| {
            property_row_input_with_hint(ui, label, &mut typed, hint, false)
        })
        .inner;
    if response.has_focus() {
        let sheet = sheet.clone();
        ui.data_mut(|data| {
            data.insert_temp(text.draft_id(), TextDraft { sheet, text: typed });
        });
        return None;
    }
    let abandoned = ui.input(|input| input.key_pressed(egui::Key::Escape));
    (draft.is_some() && !abandoned && typed.trim() != stored).then(|| typed.trim().to_owned())
}

/// Commit a value still being typed when the panel stops showing the sheet.
/// A click on the canvas can select something in the frame that takes the
/// field's focus, and leaving the field must not lose what was typed. A value
/// typed on another sheet than the active one is dropped rather than written
/// to the wrong sheet.
pub(super) fn commit_pending_sheet_text(ctx: &egui::Context, state: &mut AppState) {
    let sheet = sheet_identity(state);
    for text in SheetText::all() {
        let Some(draft) = ctx.data_mut(|data| take_text_draft(data, text)) else {
            continue;
        };
        if draft.sheet != sheet {
            continue;
        }
        let format = resolve_active_drawing_sheet(state).format;
        if draft.text.trim() == text.stored(state, &format) {
            continue;
        }
        if let Err(error) = text
            .edit(&format, draft.text.trim().to_owned())
            .and_then(|edit| apply_drawing_sheet_edit(state, edit))
        {
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

/// The template's one name on `format`'s sheet, as every surface gives it.
fn template_name(
    format: &SchematicSheetFormat,
    template: DrawingSheetTitleBlockTemplate,
) -> &'static str {
    format.title_block_template_label(template)
}

#[cfg(test)]
mod tests;
