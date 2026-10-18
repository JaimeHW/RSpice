//! One edit to the active sheet, staged and published as one transaction.

use std::collections::BTreeMap;

use crate::schematic::view::drawing_sheet::resolve_active_drawing_sheet;
use crate::state::{
    AuthoredDrawingSheetSize, DRAWING_SHEET_MIXED_SIZES_ADVISORY, DrawingSheetBorderTemplate,
    DrawingSheetDisplayUnit, DrawingSheetDocumentControl, DrawingSheetFrameConvention,
    DrawingSheetInheritance, DrawingSheetPreset, DrawingSheetPresetScope,
    DrawingSheetReleaseStatus, DrawingSheetStandard, DrawingSheetTitleBlockTemplate,
    DrawingSheetTitleFieldId, DrawingSheetTransactionKind, DrawingSheetTransactionReceipt,
    DrawingSheetZoneMode, SchematicPageOrientation, SchematicSheetFormat, SheetId, ViewType,
    drawing_sheet_sizes_mixed,
};
use crate::workbench::SurfaceId;
use crate::workbench::app_state::{AppState, DesignManagementHistoryEntry};
use crate::workbench::state::Workspace;

use super::draft::{DrawingSheetDraft, SheetSizeChoice};

/// Whether the active sheet can be edited: a writable schematic or testbench
/// on the design surface. Print and Export own output media and never stand
/// in for the sheet.
pub(crate) fn drawing_sheet_editable(state: &AppState) -> bool {
    state.workbench.workspace == Workspace::Design
        && state.workbench.current_route().surface_id() == SurfaceId::Design
        && !state.schematic_edit_read_only()
        && matches!(
            state.workspace.content.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        )
}

/// One property of the active sheet, as the sheet panel edits it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DrawingSheetEdit {
    Size(DrawingSheetStandard),
    /// A one-off size the standard list does not hold, its edges as the sheet
    /// stands in its orientation. It keeps the frame of the size it started
    /// from.
    CustomSize {
        width_um: u64,
        height_um: u64,
    },
    /// The name a one-off custom size is known and saved by.
    CustomSizeName(String),
    /// A saved custom size, the project's or the person's own. A personal
    /// one is copied into the project when the sheet takes it.
    Preset(Box<DrawingSheetPreset>),
    /// Save the sheet's one-off custom size to the project's custom sizes or
    /// the person's own, and put the sheet on the saved size.
    SaveCustomSize(DrawingSheetPresetScope),
    Orientation(SchematicPageOrientation),
    /// Take the project default's format and follow it, or keep the format
    /// the sheet has as its own.
    FollowProjectDefault(bool),
    /// The frame of the sheet's standard, or none.
    Frame(DrawingSheetBorderTemplate),
    /// The standard's zones along the frame, or none.
    Zones(bool),
    /// ISO 5457's centring marks, or ASME Y14.1's microfilm arrows.
    CentringMarks(bool),
    /// ASME Y14.1 4.1.3's binding margin.
    BindingMargin(bool),
    /// The unit the sheet's sizes are stated in.
    Units(DrawingSheetDisplayUnit),
    TitleBlock(DrawingSheetTitleBlockTemplate),
    /// Whether the title block shows a box its form leaves optional.
    BoxShown(DrawingSheetTitleFieldId, bool),
    /// A typed title-block value, kept where its field keeps it: on this
    /// sheet, on the project for the values every sheet shares, or in the
    /// document control for the revision and its date.
    TitleField(DrawingSheetTitleFieldId, String),
    /// Where the document stands in its release.
    Status(DrawingSheetReleaseStatus),
    /// The change order, ticket or release record the document's revision
    /// answers to.
    ChangeReference(String),
}

/// Apply one property of the active sheet as one transaction.
///
/// The sheet's draft is taken from what it draws, the edit is staged into
/// it, the draft is validated whole, and the result is published with one
/// undo entry. Nothing is left behind when the edit is refused.
pub(crate) fn apply_drawing_sheet_edit(
    state: &mut AppState,
    edit: DrawingSheetEdit,
) -> Result<String, String> {
    if !drawing_sheet_editable(state) {
        return Err("The sheet can only be edited on a writable schematic.".to_owned());
    }
    let mut transaction = SheetTransaction::open(state);
    transaction.stage(state, edit)?;
    transaction.commit(state)
}

/// The governed sheet an edit lands on, as it stood when the edit began.
#[derive(Debug, Clone)]
struct GovernedSheet {
    cell_view_key: String,
    sheet_id: SheetId,
    sheet_revision: u64,
}

/// One edit to the active sheet, staged and not yet published.
#[derive(Debug, Clone)]
struct SheetTransaction {
    /// None for a schematic whose first edit gives it its governed sheets.
    governed: Option<GovernedSheet>,
    sheet_name: String,
    project_default: SchematicSheetFormat,
    /// The sheet as it was, whose organization-managed block no edit
    /// replaces.
    baseline: DrawingSheetDraft,
    draft: DrawingSheetDraft,
    /// Project-owned title values, staged whole once one of them is edited.
    staged_project_title_values: Option<BTreeMap<DrawingSheetTitleFieldId, String>>,
    /// The project's document control, staged once part of it is edited.
    staged_document_control: Option<DrawingSheetDocumentControl>,
}

impl SheetTransaction {
    /// The active sheet's transaction, drafted from what the sheet draws.
    fn open(state: &AppState) -> Self {
        let cell_view_key = state.workspace.content.active_key();
        let resolved = resolve_active_drawing_sheet(state);
        let designs = &state.workspace.content.design_management;
        let governed = designs
            .sheet_catalog(&cell_view_key)
            .and_then(|catalog| catalog.active())
            .map(|sheet| GovernedSheet {
                cell_view_key: cell_view_key.clone(),
                sheet_id: sheet.id(),
                sheet_revision: sheet.revision(),
            });
        let mut draft = DrawingSheetDraft::from_format(&resolved.format);
        let sheet_title = draft
            .title_fields
            .entry(DrawingSheetTitleFieldId::SheetTitle)
            .or_default();
        if sheet_title.value.trim().is_empty() {
            sheet_title.value.clone_from(&resolved.sheet_name);
        }
        Self {
            governed,
            sheet_name: resolved.sheet_name,
            project_default: designs.drawing_sheet_settings().default_format.clone(),
            baseline: draft.clone(),
            draft,
            staged_project_title_values: None,
            staged_document_control: None,
        }
    }

    /// Stage `edit` in the draft, or refuse it in words the panel shows.
    fn stage(&mut self, state: &AppState, edit: DrawingSheetEdit) -> Result<(), String> {
        let draft = &mut self.draft;
        match edit {
            DrawingSheetEdit::Size(standard) => {
                draft.apply_size_choice(SheetSizeChoice::Standard(standard));
            }
            DrawingSheetEdit::CustomSize {
                width_um,
                height_um,
            } => {
                if draft.size != SheetSizeChoice::Custom {
                    draft.apply_size_choice(SheetSizeChoice::Custom);
                }
                (draft.width_um, draft.height_um) = (width_um, height_um);
                // The edges are typed as the sheet is drawn, so a width under
                // its height stands the sheet upright, and one over it lays
                // the sheet down.
                if width_um != height_um {
                    draft.orientation = if width_um > height_um {
                        SchematicPageOrientation::Landscape
                    } else {
                        SchematicPageOrientation::Portrait
                    };
                }
            }
            DrawingSheetEdit::CustomSizeName(name) => {
                if draft.size != SheetSizeChoice::Custom {
                    return Err("Only a one-off custom size is named here.".to_owned());
                }
                draft.custom_name = name;
            }
            DrawingSheetEdit::Preset(preset) => {
                let preset = *preset;
                let mut replacement = DrawingSheetDraft::from_format(&preset.format);
                replacement.title_fields = std::mem::take(&mut draft.title_fields);
                replacement.size = SheetSizeChoice::CapturedPreset {
                    id: preset.id,
                    name: preset.name,
                    scope: preset.scope,
                };
                replacement.enforce_managed_authority(&self.baseline);
                *draft = replacement;
            }
            DrawingSheetEdit::SaveCustomSize(scope) => {
                if draft.size != SheetSizeChoice::Custom {
                    return Err(
                        "Only a one-off custom size can be saved to custom sizes.".to_owned()
                    );
                }
                draft.save_custom_preset = true;
                draft.custom_preset_scope = scope;
            }
            DrawingSheetEdit::Orientation(orientation) => draft.set_orientation(orientation),
            DrawingSheetEdit::FollowProjectDefault(true) => {
                let mut replacement = DrawingSheetDraft::from_format(&self.project_default);
                replacement.inheritance = DrawingSheetInheritance::ProjectDefault;
                replacement.title_fields = std::mem::take(&mut draft.title_fields);
                replacement.enforce_managed_authority(&self.baseline);
                *draft = replacement;
            }
            DrawingSheetEdit::FollowProjectDefault(false) => {
                draft.inheritance = DrawingSheetInheritance::Explicit;
            }
            DrawingSheetEdit::Frame(border) => draft.set_border_template(border),
            DrawingSheetEdit::Zones(on) => {
                if on && !draft.standard_frame() {
                    return Err("A sheet without a frame has no zones.".to_owned());
                }
                draft.zone_mode = if on {
                    DrawingSheetZoneMode::Automatic
                } else {
                    DrawingSheetZoneMode::None
                };
            }
            DrawingSheetEdit::CentringMarks(on) => {
                if on && !draft.standard_frame() {
                    return Err("A sheet without a frame has no centring marks.".to_owned());
                }
                draft.centring_marks = on;
            }
            DrawingSheetEdit::BindingMargin(on) => {
                if on
                    && !(draft.standard_frame()
                        && draft.frame_convention() == DrawingSheetFrameConvention::AsmeY141)
                {
                    return Err(
                        "Only a framed ASME Y14.1 sheet takes ASME Y14.1's binding margin."
                            .to_owned(),
                    );
                }
                draft.binding_margin = on;
            }
            DrawingSheetEdit::Units(unit) => draft.display_unit = unit,
            DrawingSheetEdit::TitleBlock(template) => draft.title_block = template,
            DrawingSheetEdit::BoxShown(field, shown) => {
                refuse_organization_locked(draft, field)?;
                if !shown && draft.title_block.requires_field(field) {
                    return Err(draft.title_block.required_field_note().to_owned());
                }
                draft.title_fields.entry(field).or_default().visible = shown;
            }
            DrawingSheetEdit::TitleField(field, value) => {
                refuse_organization_locked(draft, field)?;
                self.stage_title_value(state, field, value);
            }
            DrawingSheetEdit::Status(status) => {
                self.stage_document_control(state, |control| control.status = status);
            }
            DrawingSheetEdit::ChangeReference(reference) => {
                self.stage_document_control(state, |control| {
                    control.change_reference = reference;
                });
            }
        }
        Ok(())
    }

    /// Stage one title-block value where its field keeps it: the revision
    /// and its date in the document control, the values every sheet shares
    /// on the project, and the rest on this sheet.
    fn stage_title_value(
        &mut self,
        state: &AppState,
        field: DrawingSheetTitleFieldId,
        value: String,
    ) {
        match field {
            DrawingSheetTitleFieldId::Revision => {
                self.stage_document_control(state, |control| control.revision = value);
            }
            DrawingSheetTitleFieldId::Date => {
                self.stage_document_control(state, |control| control.revision_date_utc = value);
            }
            _ if field.is_project_owned() => {
                // Every project-owned value staged is published, so the ones
                // not being edited are staged as they stand, including a
                // value an older build left on the sheet itself.
                let mut values = state
                    .workspace
                    .content
                    .design_management
                    .drawing_sheet_settings()
                    .title_block_field_values
                    .clone();
                values.insert(field, value);
                for id in DrawingSheetTitleFieldId::PROJECT_OWNED {
                    let staged = values.entry(id).or_default();
                    if id != field
                        && staged.is_empty()
                        && let Some(legacy) = self.draft.title_fields.get(&id)
                    {
                        staged.clone_from(&legacy.value);
                    }
                    self.draft
                        .title_fields
                        .entry(id)
                        .or_default()
                        .value
                        .clone_from(staged);
                }
                self.staged_project_title_values = Some(values);
            }
            _ => {
                if let Some(authored) = self.draft.title_fields.get_mut(&field) {
                    authored.value = value;
                }
            }
        }
    }

    /// Stage the project's document control with one change made to it.
    fn stage_document_control(
        &mut self,
        state: &AppState,
        change: impl FnOnce(&mut DrawingSheetDocumentControl),
    ) {
        let mut control = state
            .workspace
            .content
            .design_management
            .drawing_sheet_settings()
            .document_control
            .clone();
        change(&mut control);
        self.staged_document_control = Some(control);
    }

    /// Validate the draft whole and publish it as one undo entry.
    fn commit(self, state: &mut AppState) -> Result<String, String> {
        let mut draft = self.draft.clone();
        draft.enforce_managed_authority(&self.baseline);
        let format = draft.validate()?;
        match &self.governed {
            Some(governed) => apply_governed_sheet_edit(state, &self, governed, format),
            None => apply_first_sheet_edit(state, &self, format),
        }
    }
}

/// Refuse an edit to `field` when the organization's block on the sheet
/// fixes it.
fn refuse_organization_locked(
    draft: &DrawingSheetDraft,
    field: DrawingSheetTitleFieldId,
) -> Result<(), String> {
    let locked = draft.title_block == DrawingSheetTitleBlockTemplate::OrganizationManaged
        && draft
            .managed_title_template
            .as_ref()
            .is_some_and(|snapshot| snapshot.locks_field(field));
    if locked {
        return Err(format!(
            "The organization-managed {} value cannot be edited.",
            field.display_label().to_lowercase()
        ));
    }
    Ok(())
}

/// The paper every other sheet of the active schematic is drawn on: a sheet
/// that follows the project default is drawn on the default's.
fn other_sheet_sizes(state: &AppState) -> Vec<AuthoredDrawingSheetSize> {
    let designs = &state.workspace.content.design_management;
    let default_size = &designs
        .drawing_sheet_settings()
        .default_format
        .authored_size;
    designs
        .sheet_catalog(&state.workspace.content.active_key())
        .map(|catalog| {
            let active = catalog.active_sheet_id();
            catalog
                .sheets()
                .iter()
                .filter(|sheet| Some(sheet.id()) != active)
                .map(|sheet| match sheet.page_format().inheritance {
                    DrawingSheetInheritance::ProjectDefault => default_size.clone(),
                    _ => sheet.page_format().authored_size.clone(),
                })
                .collect()
        })
        .unwrap_or_default()
}

/// ASME Y14.1's advice when the active sheet, drawn as `format`, leaves an
/// ANSI or ARCH schematic's sheets on more than one size. ISO 5457 makes no
/// such recommendation.
pub(crate) fn mixed_size_advisory(
    state: &AppState,
    format: &SchematicSheetFormat,
) -> Option<&'static str> {
    (format.frame_convention() == DrawingSheetFrameConvention::AsmeY141
        && drawing_sheet_sizes_mixed(
            std::iter::once(&format.authored_size).chain(&other_sheet_sizes(state)),
        ))
    .then_some(DRAWING_SHEET_MIXED_SIZES_ADVISORY)
}

/// The format `transaction` writes to its sheet: the organization's block
/// where the sheet carries one, the project's values moved to the project,
/// following the project default only while it still matches it, and on a
/// saved custom size where it takes or saves one.
fn edited_sheet_format(
    candidate: &mut crate::state::DesignManagementCatalog,
    personal: &mut crate::workbench::DrawingSheetPersonalPreferences,
    transaction: &SheetTransaction,
    format: SchematicSheetFormat,
    target_authority: Option<&SchematicSheetFormat>,
) -> Result<SchematicSheetFormat, String> {
    let mut sheet_format = match target_authority {
        Some(authority) => enforce_managed_format_authority(format, authority)?,
        None => format,
    };
    publish_staged_document_control(candidate, transaction.staged_document_control.as_ref())?;
    migrate_and_canonicalize_project_title_values(
        candidate,
        &mut sheet_format,
        target_authority.is_none() || transaction.staged_project_title_values.is_some(),
    )?;
    if sheet_format.inheritance != DrawingSheetInheritance::Explicit
        && !same_setup_ignoring_inheritance(
            &sheet_format,
            candidate.drawing_sheet_settings().default_format.clone(),
        )?
    {
        sheet_format = sheet_format
            .try_update(|draft| draft.inheritance = DrawingSheetInheritance::Explicit)
            .map_err(|error| error.to_string())?;
    }
    capture_selected_personal_preset(
        candidate,
        personal,
        &transaction.draft.size,
        &mut sheet_format,
    )?;
    if transaction.draft.save_custom_preset {
        save_one_off_custom_preset(
            candidate,
            personal,
            transaction.draft.custom_preset_scope,
            transaction.draft.custom_name.trim(),
            &mut sheet_format,
        )?;
    }
    remember_explicit_format(candidate, &sheet_format)?;
    Ok(sheet_format)
}

fn apply_governed_sheet_edit(
    state: &mut AppState,
    transaction: &SheetTransaction,
    governed: &GovernedSheet,
    format: SchematicSheetFormat,
) -> Result<String, String> {
    let before = (*state.workspace.content.design_management).clone();
    let target_authority = before
        .sheet_catalog(&governed.cell_view_key)
        .and_then(|catalog| catalog.find(governed.sheet_id))
        .map(|sheet| sheet.page_format().clone())
        .ok_or_else(|| "The governed active sheet is unavailable.".to_owned())?;
    let mut candidate = before.clone();
    let personal_before = state.ui.preferences.drawing_sheet_personal_preferences();
    let mut personal = personal_before.clone();
    let sheet_format = edited_sheet_format(
        &mut candidate,
        &mut personal,
        transaction,
        format,
        Some(&target_authority),
    )?;
    let unchanged = target_authority == sheet_format
        || inherited_sheet_owned_state_matches(&target_authority, &sheet_format);
    let project_settings_changed =
        candidate.drawing_sheet_settings() != before.drawing_sheet_settings();
    if unchanged && !project_settings_changed {
        return Ok("The sheet already had that format.".to_owned());
    }
    if !unchanged {
        candidate
            .sheet_catalog_mut(&governed.cell_view_key)
            .ok_or_else(|| "The governed sheet catalog is unavailable.".to_owned())?
            .update_sheet_page_format(
                governed.sheet_id,
                governed.sheet_revision,
                sheet_format.clone(),
            )
            .map_err(|error| error.to_string())?;
    }
    let (applied, unchanged_ids) = if unchanged {
        (Vec::new(), vec![governed.sheet_id])
    } else {
        (vec![governed.sheet_id], Vec::new())
    };
    record_drawing_sheet_transaction(
        &mut candidate,
        &governed.cell_view_key,
        &sheet_format,
        vec![governed.sheet_id],
        applied,
        unchanged_ids,
        transaction.draft.save_custom_preset,
        project_settings_changed,
    )?;
    commit_sheet_candidate(state, before, candidate, personal_before, personal)?;
    Ok(format!("Sheet {} changed.", transaction.sheet_name))
}

/// The first edit to a schematic's sheet gives it its governed sheet.
fn apply_first_sheet_edit(
    state: &mut AppState,
    transaction: &SheetTransaction,
    format: SchematicSheetFormat,
) -> Result<String, String> {
    let before = (*state.workspace.content.design_management).clone();
    let mut candidate = before.clone();
    let personal_before = state.ui.preferences.drawing_sheet_personal_preferences();
    let mut personal = personal_before.clone();
    let owner_key = state.workspace.content.active_key();
    let sheet_id = candidate
        .bootstrap_for_cell_view(
            &owner_key,
            transaction.sheet_name.clone(),
            all_stable_object_ids(&state.schematic),
        )
        .map_err(|error| error.to_string())?;
    let sheet_format =
        edited_sheet_format(&mut candidate, &mut personal, transaction, format, None)?;
    let (sheet_revision, current_page_format) = candidate
        .sheet_catalog(&owner_key)
        .and_then(|catalog| catalog.find(sheet_id))
        .map(|sheet| (sheet.revision(), sheet.page_format().clone()))
        .ok_or_else(|| "The newly governed drawing sheet is unavailable.".to_owned())?;
    if current_page_format != sheet_format {
        candidate
            .sheet_catalog_mut(&owner_key)
            .ok_or_else(|| "The newly governed drawing-sheet catalog is unavailable.".to_owned())?
            .update_sheet_page_format(sheet_id, sheet_revision, sheet_format.clone())
            .map_err(|error| error.to_string())?;
    }
    let project_settings_changed =
        candidate.drawing_sheet_settings() != before.drawing_sheet_settings();
    record_drawing_sheet_transaction(
        &mut candidate,
        &owner_key,
        &sheet_format,
        vec![sheet_id],
        vec![sheet_id],
        Vec::new(),
        transaction.draft.save_custom_preset,
        project_settings_changed,
    )?;
    commit_sheet_candidate(state, before, candidate, personal_before, personal)?;
    Ok("The active schematic now has its own drawing sheet.".to_owned())
}

fn record_drawing_sheet_transaction(
    candidate: &mut crate::state::DesignManagementCatalog,
    owner_cell_view_key: &str,
    source_format: &SchematicSheetFormat,
    selected_sheet_ids: Vec<SheetId>,
    applied_sheet_ids: Vec<SheetId>,
    unchanged_sheet_ids: Vec<SheetId>,
    project_preset_saved: bool,
    project_settings_changed: bool,
) -> Result<(), String> {
    let catalog_revision = candidate
        .revision()
        .checked_add(1)
        .ok_or_else(|| "Drawing-sheet receipt revision space is exhausted.".to_owned())?;
    let receipt = DrawingSheetTransactionReceipt {
        catalog_revision,
        kind: DrawingSheetTransactionKind::PageSetup,
        owner_cell_view_key: owner_cell_view_key.to_owned(),
        source_format_digest: source_format
            .content_digest()
            .map_err(|error| error.to_string())?,
        selected_sheet_ids,
        applied_sheet_ids,
        unchanged_sheet_ids,
        skipped: Vec::new(),
        project_default_changed: false,
        project_preset_saved,
        project_settings_changed,
    };
    candidate
        .record_drawing_sheet_transaction(candidate.revision(), receipt)
        .map_err(|error| error.to_string())?;
    Ok(())
}

/// Publish `candidate` and the person's custom sizes as one undo entry.
fn commit_sheet_candidate(
    state: &mut AppState,
    before: crate::state::DesignManagementCatalog,
    candidate: crate::state::DesignManagementCatalog,
    personal_before: crate::workbench::DrawingSheetPersonalPreferences,
    personal: crate::workbench::DrawingSheetPersonalPreferences,
) -> Result<(), String> {
    let mut preferences = state.ui.preferences.clone();
    if personal != personal_before {
        preferences.set_drawing_sheet_personal_preferences(personal)?;
    }
    candidate.validate().map_err(|error| error.to_string())?;
    let schematic_tx = state.prepare_design_management_schematic_transaction(&candidate)?;
    let owner = state.workspace.content.active_schematic_reference();
    let committed_revision = state
        .workspace
        .content
        .replace_design_management(candidate)
        .map_err(|error| error.to_string())?;
    state.apply_design_management_schematic_transaction(&schematic_tx);
    let after = (*state.workspace.content.design_management).clone();
    state.record_design_management_transaction(DesignManagementHistoryEntry {
        description: "Sheet format".to_owned(),
        owner,
        before,
        after,
        before_schematics: schematic_tx.before,
        after_schematics: schematic_tx.after,
        references: schematic_tx.references,
        committed_revision,
    });
    state.ui.preferences = preferences;
    Ok(())
}

fn enforce_managed_format_authority(
    candidate: SchematicSheetFormat,
    authority: &SchematicSheetFormat,
) -> Result<SchematicSheetFormat, String> {
    candidate
        .try_update(|draft| {
            if authority.title_block.template == DrawingSheetTitleBlockTemplate::OrganizationManaged
            {
                draft.title_block.template = authority.title_block.template;
            }
        })
        .map_err(|error| error.to_string())
}

fn same_setup_ignoring_inheritance(
    left: &SchematicSheetFormat,
    mut right: SchematicSheetFormat,
) -> Result<bool, String> {
    let mut left = left.clone();
    // Title-field values and visibility are sheet-owned even while the
    // physical/frame setup follows the project default. They must not make a
    // support-only action (for example saving a reusable custom size) sever
    // inheritance on an otherwise unchanged sheet.
    left.title_block
        .fields
        .clone_from(&right.title_block.fields);
    let left = left
        .try_update(|draft| draft.inheritance = DrawingSheetInheritance::Explicit)
        .map_err(|error| error.to_string())?;
    right = right
        .try_update(|draft| draft.inheritance = DrawingSheetInheritance::Explicit)
        .map_err(|error| error.to_string())?;
    Ok(left == right)
}

/// Project-default sheets retain a physical-format snapshot for migration and
/// recovery, but only their title-field state is authored by the sheet. A
/// stale physical snapshot must not create history when the effective default
/// already matches; conversely, a title-field edit must never be discarded
/// merely because both formats inherit from the project default.
fn inherited_sheet_owned_state_matches(
    left: &SchematicSheetFormat,
    right: &SchematicSheetFormat,
) -> bool {
    left.inheritance == DrawingSheetInheritance::ProjectDefault
        && right.inheritance == DrawingSheetInheritance::ProjectDefault
        && left.title_block.fields == right.title_block.fields
}

fn unique_preset_id(name: &str, candidate: &crate::state::DesignManagementCatalog) -> String {
    let stem = preset_id_stem(name);
    if candidate
        .drawing_sheet_settings()
        .find_preset(&stem)
        .is_none()
    {
        return stem;
    }
    (2_u32..)
        .map(|suffix| format!("{stem}-{suffix}"))
        .find(|id| candidate.drawing_sheet_settings().find_preset(id).is_none())
        .expect("a finite preset catalog always has a free numeric suffix")
}

fn unique_personal_preset_id(
    name: &str,
    personal: &crate::workbench::DrawingSheetPersonalPreferences,
) -> String {
    let stem = preset_id_stem(name);
    if personal.find_preset(&stem).is_none() {
        return stem;
    }
    (2_u32..)
        .map(|suffix| format!("{stem}-{suffix}"))
        .find(|id| personal.find_preset(id).is_none())
        .expect("a finite preset catalog always has a free numeric suffix")
}

fn preset_id_stem(name: &str) -> String {
    let stem = name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    format!("custom-{}", if stem.is_empty() { "sheet" } else { &stem })
}

fn capture_selected_personal_preset(
    candidate: &mut crate::state::DesignManagementCatalog,
    personal: &crate::workbench::DrawingSheetPersonalPreferences,
    choice: &SheetSizeChoice,
    sheet_format: &mut SchematicSheetFormat,
) -> Result<(), String> {
    let SheetSizeChoice::CapturedPreset {
        id,
        scope: DrawingSheetPresetScope::User,
        ..
    } = choice
    else {
        return Ok(());
    };
    let source = personal
        .find_preset(id)
        .ok_or_else(|| format!("Personal custom size '{id}' is no longer available."))?;
    if matches!(
        &source.format.authored_size,
        AuthoredDrawingSheetSize::Custom { snapshot } if snapshot.source_preset_unavailable
    ) {
        return Err(format!(
            "Personal custom size '{}' has an unavailable dependency.",
            source.name
        ));
    }
    let captured =
        crate::workbench::app::dialogs::drawing_sheet_presets::capture_personal_preset_into_project(
            candidate, source,
        )?;
    *sheet_format = attach_project_preset(sheet_format, &captured)?;
    Ok(())
}

fn save_one_off_custom_preset(
    candidate: &mut crate::state::DesignManagementCatalog,
    personal: &mut crate::workbench::DrawingSheetPersonalPreferences,
    scope: DrawingSheetPresetScope,
    name: &str,
    sheet_format: &mut SchematicSheetFormat,
) -> Result<(), String> {
    if !matches!(
        sheet_format.authored_size,
        AuthoredDrawingSheetSize::Custom { .. }
    ) {
        return Err("Only a one-off custom size can be saved as a reusable preset.".to_owned());
    }
    let named = |id: &str| {
        sheet_format
            .try_update(|draft| {
                draft.inheritance = DrawingSheetInheritance::Explicit;
                if let AuthoredDrawingSheetSize::Custom { snapshot } = &mut draft.authored_size {
                    snapshot.preset_id = Some(id.to_owned());
                    snapshot.name = name.to_owned();
                    snapshot.source_preset_unavailable = false;
                }
            })
            .map_err(|error| error.to_string())
    };
    match scope {
        DrawingSheetPresetScope::Project => {
            if candidate
                .drawing_sheet_settings()
                .presets
                .iter()
                .any(|preset| preset.name.eq_ignore_ascii_case(name))
            {
                return Err(format!(
                    "A project custom size named '{name}' already exists."
                ));
            }
            let id = unique_preset_id(name, candidate);
            let preset = DrawingSheetPreset {
                format: named(&id)?.as_reusable_drawing_sheet_preset(),
                id,
                name: name.to_owned(),
                scope: DrawingSheetPresetScope::Project,
            };
            candidate
                .publish_drawing_sheet_preset(candidate.revision(), preset.clone())
                .map_err(|error| error.to_string())?;
            *sheet_format = attach_project_preset(sheet_format, &preset)?;
            Ok(())
        }
        DrawingSheetPresetScope::User => {
            if personal
                .presets
                .iter()
                .any(|preset| preset.name.eq_ignore_ascii_case(name))
            {
                return Err(format!(
                    "A personal custom size named '{name}' already exists."
                ));
            }
            let id = unique_personal_preset_id(name, personal);
            let source = DrawingSheetPreset {
                format: named(&id)?.as_reusable_drawing_sheet_preset(),
                id,
                name: name.to_owned(),
                scope: DrawingSheetPresetScope::User,
            };
            personal.presets.push(source.clone());
            personal.validate()?;
            let captured =
                crate::workbench::app::dialogs::drawing_sheet_presets::capture_personal_preset_into_project(
                    candidate, &source,
                )?;
            *sheet_format = attach_project_preset(sheet_format, &captured)?;
            Ok(())
        }
        DrawingSheetPresetScope::Organization => {
            Err("Organization custom sizes are managed and cannot be created here.".to_owned())
        }
    }
}

fn attach_project_preset(
    sheet_format: &SchematicSheetFormat,
    preset: &DrawingSheetPreset,
) -> Result<SchematicSheetFormat, String> {
    let AuthoredDrawingSheetSize::Custom {
        snapshot: source_snapshot,
    } = &preset.format.authored_size
    else {
        return Err(format!(
            "Custom size '{}' has no authoritative custom snapshot.",
            preset.name
        ));
    };
    sheet_format
        .try_update(|draft| {
            draft.inheritance = DrawingSheetInheritance::Explicit;
            let mut snapshot = source_snapshot.clone();
            snapshot.preset_id = Some(preset.id.clone());
            snapshot.name.clone_from(&preset.name);
            snapshot.source_preset_unavailable = false;
            draft.authored_size = AuthoredDrawingSheetSize::Custom { snapshot };
        })
        .map_err(|error| error.to_string())
}

fn migrate_and_canonicalize_project_title_values(
    candidate: &mut crate::state::DesignManagementCatalog,
    format: &mut SchematicSheetFormat,
    replace_existing: bool,
) -> Result<(), String> {
    let mut settings = candidate.drawing_sheet_settings().clone();
    let mut changed = false;
    for id in DrawingSheetTitleFieldId::PROJECT_OWNED {
        let staged_value = format
            .title_block
            .fields
            .get(&id)
            .map(|field| field.value.clone())
            .unwrap_or_default();
        let project_value = settings.title_block_field_values.entry(id).or_default();
        let should_publish = replace_existing || project_value.is_empty();
        if should_publish && project_value != &staged_value {
            *project_value = staged_value;
            changed = true;
        }
    }
    if changed {
        candidate
            .update_drawing_sheet_settings(candidate.revision(), settings)
            .map_err(|error| error.to_string())?;
    }
    *format = format.without_project_owned_title_values();
    Ok(())
}

fn publish_staged_document_control(
    candidate: &mut crate::state::DesignManagementCatalog,
    document_control: Option<&DrawingSheetDocumentControl>,
) -> Result<(), String> {
    let Some(document_control) = document_control else {
        return Ok(());
    };
    document_control
        .validate()
        .map_err(|error| error.to_string())?;
    if &candidate.drawing_sheet_settings().document_control == document_control {
        return Ok(());
    }
    let mut settings = candidate.drawing_sheet_settings().clone();
    settings.document_control.clone_from(document_control);
    candidate
        .update_drawing_sheet_settings(candidate.revision(), settings)
        .map_err(|error| error.to_string())?;
    Ok(())
}

fn remember_explicit_format(
    candidate: &mut crate::state::DesignManagementCatalog,
    format: &SchematicSheetFormat,
) -> Result<(), String> {
    let settings = candidate.drawing_sheet_settings();
    if !settings.remember_last_explicit_format
        || format.inheritance != DrawingSheetInheritance::Explicit
    {
        return Ok(());
    }
    let remembered = format.as_drawing_sheet_default();
    if settings.last_explicit_format.as_ref() == Some(&remembered) {
        return Ok(());
    }
    let mut settings = settings.clone();
    settings.last_explicit_format = Some(remembered);
    candidate
        .update_drawing_sheet_settings(candidate.revision(), settings)
        .map_err(|error| error.to_string())?;
    Ok(())
}

fn all_stable_object_ids(schematic: &crate::state::SchematicState) -> Vec<u64> {
    let document = schematic.document();
    let mut ids = std::collections::BTreeSet::new();
    ids.extend(document.components.iter().map(|object| object.id));
    ids.extend(document.wires.iter().map(|object| object.id));
    ids.extend(document.buses.iter().map(|object| object.id));
    ids.extend(document.bus_taps.iter().map(|object| object.id));
    ids.extend(document.junctions.iter().map(|object| object.id));
    ids.extend(document.net_labels.iter().map(|object| object.id));
    ids.extend(document.design_notes.iter().map(|object| object.id));
    ids.extend(document.documentation_shapes.iter().map(|object| object.id));
    ids.extend(document.probes.iter().map(|object| object.id));
    ids.into_iter().collect()
}

#[cfg(test)]
mod tests;
