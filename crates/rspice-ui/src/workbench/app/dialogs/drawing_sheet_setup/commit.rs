//! Authority capture and atomic publication for drawing-sheet setup.

use crate::state::{
    DrawingSheetTransactionKind, DrawingSheetTransactionReceipt, DrawingSheetTransactionSkip,
    SchematicSheetFormat, SheetId, ViewType,
};
use crate::workbench::SurfaceId;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::{AppState, DesignManagementHistoryEntry};
use crate::workbench::state::Workspace;

use super::state::{
    DrawingSheetAuthority, DrawingSheetDraft, DrawingSheetSetupState,
    GovernedDrawingSheetAuthority, PageSetupScope, ValidatedDrawingSheetDraft,
};

pub(crate) fn drawing_sheet_setup_available(app: &RSpiceApp) -> bool {
    drawing_sheet_setup_available_for_state(&app.state)
}

fn drawing_sheet_setup_available_for_state(state: &AppState) -> bool {
    state.workbench.workspace == Workspace::Design
        && state.workbench.current_route().surface_id() == SurfaceId::Design
        && !state.schematic_edit_read_only()
        && matches!(
            state.workspace.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        )
}

pub(crate) fn open_drawing_sheet_setup(app: &mut RSpiceApp) {
    let _ = open_drawing_sheet_setup_for_state(&mut app.state);
}

pub(crate) fn open_drawing_sheet_setup_with_preset(
    app: &mut RSpiceApp,
    preset: crate::state::DrawingSheetPreset,
) -> Result<(), String> {
    if app.state.dialogs.drawing_sheet_setup.support_suspended {
        resume_drawing_sheet_setup_after_support(&mut app.state)?;
    } else if !open_drawing_sheet_setup_for_state(&mut app.state) {
        return Err(
            "Custom sheet sizes can only be used from a schematic or testbench drawing sheet."
                .to_owned(),
        );
    }
    let scope = app.state.dialogs.drawing_sheet_setup.draft.scope;
    let title_fields = app
        .state
        .dialogs
        .drawing_sheet_setup
        .draft
        .title_fields
        .clone();
    let mut draft = DrawingSheetDraft::from_format(&preset.format);
    draft.scope = scope;
    draft.title_fields = title_fields;
    draft.size = super::state::SheetSizeChoice::CapturedPreset {
        id: preset.id,
        name: preset.name,
        scope: preset.scope,
    };
    if let Some(title) = draft
        .title_fields
        .get_mut(&crate::state::DrawingSheetTitleFieldId::SheetTitle)
        && title.value.trim().is_empty()
    {
        title
            .value
            .clone_from(&app.state.dialogs.drawing_sheet_setup.sheet_name);
    }
    let managed_authority = app.state.dialogs.drawing_sheet_setup.baseline.clone();
    draft.enforce_managed_authority(&managed_authority);
    app.state.dialogs.drawing_sheet_setup.restore_margins = preset.format.margins;
    app.state.dialogs.drawing_sheet_setup.draft = draft;
    Ok(())
}

/// Open the authored-sheet workflow from state-owned schematic surfaces such
/// as the canvas context menu. Returns `false` outside schematic/testbench
/// design routes. Print and export media are owned by their separate hardcopy
/// workflows and must never be substituted for authored drawing-sheet setup.
pub(crate) fn open_drawing_sheet_setup_for_state(state: &mut AppState) -> bool {
    if !drawing_sheet_setup_available_for_state(state) {
        return false;
    }
    let cell_view_key = state.workspace.active_key();
    let resolved_active = crate::schematic::view::drawing_sheet::ActiveDrawingSheet::resolve(state);
    let governed = state
        .workspace
        .design_management
        .sheet_catalog(&cell_view_key)
        .and_then(|catalog| catalog.active().map(|sheet| (catalog, sheet)));
    let (governed, format, sheet_name, sheet_count, sheet_number) =
        if let Some((catalog, sheet)) = governed {
            let sheet_number = catalog
                .sheets()
                .iter()
                .position(|candidate| candidate.id() == sheet.id())
                .map_or(1, |index| index + 1);
            (
                Some(GovernedDrawingSheetAuthority {
                    cell_view_key: cell_view_key.clone(),
                    catalog_revision: catalog.revision(),
                    sheet_id: sheet.id(),
                    sheet_revision: sheet.revision(),
                }),
                resolved_active.format,
                sheet.name().to_owned(),
                catalog.sheets().len(),
                sheet_number,
            )
        } else {
            (
                None,
                resolved_active.format,
                state.workspace.active_view.cell.clone(),
                1,
                1,
            )
        };
    let mut draft = DrawingSheetDraft::from_format(&format);
    let sheet_title = draft
        .title_fields
        .get_mut(&crate::state::DrawingSheetTitleFieldId::SheetTitle)
        .expect("the canonical title-field set contains Sheet title");
    if sheet_title.value.trim().is_empty() {
        sheet_title.value.clone_from(&sheet_name);
    }
    let mut available_presets = state
        .workspace
        .design_management
        .drawing_sheet_settings()
        .presets
        .clone();
    available_presets.extend(
        state
            .ui
            .preferences
            .drawing_sheet_personal_preferences()
            .presets,
    );
    let document = state.workspace.active_display_path().to_uppercase();
    let managed_sheet_names = state
        .workspace
        .design_management
        .sheet_catalog(&cell_view_key)
        .map(|catalog| {
            catalog
                .sheets()
                .iter()
                .filter(|sheet| format_is_organization_managed(sheet.page_format()))
                .map(|sheet| sheet.name().to_owned())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let writable_sheet_count = sheet_count.saturating_sub(managed_sheet_names.len());
    state.dialogs.drawing_sheet_setup = DrawingSheetSetupState {
        open: true,
        eyebrow: format!(
            "DRAWING SHEET · {document} · {} · SHEET {sheet_number} OF {sheet_count}",
            sheet_name.to_uppercase()
        ),
        document_name: state.workspace.active_display_path().to_owned(),
        sheet_name,
        sheet_count,
        writable_sheet_count,
        managed_sheet_names,
        available_presets,
        project_default: state
            .workspace
            .design_management
            .drawing_sheet_settings()
            .default_format
            .clone(),
        restore_margins: format.margins,
        authority: Some(DrawingSheetAuthority {
            edit: crate::workbench::app::SchematicEditAuthority::capture(state),
            cell_view_key,
            design_management_revision: state.workspace.design_management.revision(),
            personal_preferences_digest: Some(
                state
                    .ui
                    .preferences
                    .drawing_sheet_personal_preferences()
                    .semantic_digest(),
            ),
            governed,
        }),
        baseline: draft.clone(),
        last_valid_format: format,
        draft,
        ..DrawingSheetSetupState::default()
    };
    true
}

pub(crate) fn validate_drawing_sheet_authority(
    state: &AppState,
    authority: &DrawingSheetAuthority,
) -> Result<(), String> {
    if state.workspace.active_key() != authority.cell_view_key {
        return Err("The active cell/view changed. Close and reopen Page Setup.".to_owned());
    }
    authority.edit.validate(state, "Page Setup")?;
    if state.workspace.design_management.revision() != authority.design_management_revision {
        return Err(
            "Drawing-sheet project policy changed. Close and reopen this workflow.".to_owned(),
        );
    }
    if authority
        .personal_preferences_digest
        .is_some_and(|expected| {
            state
                .ui
                .preferences
                .drawing_sheet_personal_preferences()
                .semantic_digest()
                != expected
        })
    {
        return Err(
            "Personal custom sheet sizes changed. Close and reopen Page Setup before applying."
                .to_owned(),
        );
    }
    let Some(governed) = &authority.governed else {
        let active_governed_sheet = state
            .workspace
            .design_management
            .sheet_catalog(&state.workspace.active_key())
            .and_then(|catalog| catalog.active_sheet_id());
        return if active_governed_sheet.is_none() {
            Ok(())
        } else {
            Err(
                "The active schematic acquired governed sheet authority. Close and reopen Page Setup."
                    .to_owned(),
            )
        };
    };
    if state.workspace.active_key() != governed.cell_view_key {
        return Err("The active cell/view changed. Close and reopen Page Setup.".to_owned());
    }
    let catalog = state
        .workspace
        .design_management
        .sheet_catalog(&governed.cell_view_key)
        .ok_or_else(|| {
            "The governed sheet catalog changed. Close and reopen Page Setup.".to_owned()
        })?;
    if catalog.revision() != governed.catalog_revision
        || catalog.active_sheet_id() != Some(governed.sheet_id)
        || catalog
            .find(governed.sheet_id)
            .is_none_or(|sheet| sheet.revision() != governed.sheet_revision)
    {
        return Err("The governed active sheet changed. Close and reopen Page Setup.".to_owned());
    }
    Ok(())
}

/// Rebase only the authority captured around an intentionally nested support
/// workflow. The Page Setup working draft and its opened-value baseline stay
/// byte-for-byte intact; current target title fields are still preserved by
/// the eventual atomic apply.
pub(crate) fn resume_drawing_sheet_setup_after_support(state: &mut AppState) -> Result<(), String> {
    let previous = state
        .dialogs
        .drawing_sheet_setup
        .authority
        .clone()
        .ok_or_else(|| "Page Setup lost its suspended authority.".to_owned())?;
    if state.workspace.active_key() != previous.cell_view_key {
        return Err(
            "The active cell/view changed while Page Setup was suspended. Close and reopen it."
                .to_owned(),
        );
    }
    let governed = if let Some(previous_governed) = previous.governed {
        let catalog = state
            .workspace
            .design_management
            .sheet_catalog(&previous_governed.cell_view_key)
            .ok_or_else(|| {
                "The governed sheet catalog changed while Page Setup was suspended.".to_owned()
            })?;
        if catalog.active_sheet_id() != Some(previous_governed.sheet_id) {
            return Err(
                "The active sheet changed while Page Setup was suspended. Close and reopen it."
                    .to_owned(),
            );
        }
        let sheet = catalog.find(previous_governed.sheet_id).ok_or_else(|| {
            "The active sheet was removed while Page Setup was suspended.".to_owned()
        })?;
        Some(GovernedDrawingSheetAuthority {
            cell_view_key: previous_governed.cell_view_key,
            catalog_revision: catalog.revision(),
            sheet_id: sheet.id(),
            sheet_revision: sheet.revision(),
        })
    } else {
        None
    };
    let personal = state.ui.preferences.drawing_sheet_personal_preferences();
    let mut available_presets = state
        .workspace
        .design_management
        .drawing_sheet_settings()
        .presets
        .clone();
    available_presets.extend(personal.presets.clone());
    let authority = DrawingSheetAuthority {
        edit: crate::workbench::app::SchematicEditAuthority::capture(state),
        cell_view_key: previous.cell_view_key,
        design_management_revision: state.workspace.design_management.revision(),
        personal_preferences_digest: Some(personal.semantic_digest()),
        governed,
    };
    let setup = &mut state.dialogs.drawing_sheet_setup;
    setup.authority = Some(authority);
    setup.available_presets = available_presets;
    setup.project_default = state
        .workspace
        .design_management
        .drawing_sheet_settings()
        .default_format
        .clone();
    setup.authority_error = None;
    setup.commit_error = None;
    setup.support_suspended = false;
    setup.open = true;
    Ok(())
}

pub(crate) fn apply_drawing_sheet_setup(app: &mut RSpiceApp) -> Result<String, String> {
    let authority = app
        .state
        .dialogs
        .drawing_sheet_setup
        .authority
        .clone()
        .ok_or_else(|| "Page Setup has no active sheet authority.".to_owned())?;
    validate_drawing_sheet_authority(&app.state, &authority)?;
    let mut draft = app.state.dialogs.drawing_sheet_setup.draft.clone();
    let managed_authority = app.state.dialogs.drawing_sheet_setup.baseline.clone();
    draft.enforce_managed_authority(&managed_authority);
    let validated = draft.validate().map_err(|problems| {
        problems
            .into_iter()
            .map(|problem| problem.message)
            .collect::<Vec<_>>()
            .join(" ")
    })?;

    let outcome = if let Some(governed) = &authority.governed {
        apply_governed_sheet_setup(app, governed, validated)?
    } else {
        apply_legacy_sheet_setup(app, validated)?
    };
    app.state.dialogs.drawing_sheet_setup.close();
    Ok(outcome)
}

fn apply_governed_sheet_setup(
    app: &mut RSpiceApp,
    authority: &GovernedDrawingSheetAuthority,
    validated: ValidatedDrawingSheetDraft,
) -> Result<String, String> {
    let transaction = app.state.dialogs.drawing_sheet_setup.clone();
    let scope = transaction.draft.scope;
    let before = app.state.workspace.design_management.clone();
    let source_catalog = before
        .sheet_catalog(&authority.cell_view_key)
        .ok_or_else(|| "The governed sheet catalog is unavailable.".to_owned())?;
    let targets = match scope {
        PageSetupScope::CurrentSheet | PageSetupScope::CurrentSheetAndDefault => {
            vec![(authority.sheet_id, authority.sheet_revision)]
        }
        PageSetupScope::Document => source_catalog
            .sheets()
            .iter()
            .map(|sheet| (sheet.id(), sheet.revision()))
            .collect(),
    };
    let selected_sheet_ids = targets
        .iter()
        .map(|(sheet_id, _)| *sheet_id)
        .collect::<Vec<_>>();
    let mut candidate = before.clone();
    let personal_before = app
        .state
        .ui
        .preferences
        .drawing_sheet_personal_preferences();
    let mut personal = personal_before.clone();
    let mut sheet_format = validated.page_format;
    if scope != PageSetupScope::Document {
        let target_authority = source_catalog
            .find(authority.sheet_id)
            .map(|sheet| sheet.page_format())
            .ok_or_else(|| "The governed active sheet is unavailable.".to_owned())?;
        sheet_format = enforce_managed_format_authority(sheet_format, target_authority)?;
    }
    migrate_and_canonicalize_project_title_values(&mut candidate, &mut sheet_format, false)?;
    if scope != PageSetupScope::CurrentSheet
        || (sheet_format.inheritance != crate::state::DrawingSheetInheritance::Explicit
            && !same_setup_ignoring_inheritance(
                &sheet_format,
                candidate.drawing_sheet_settings().default_format.clone(),
            )?)
    {
        sheet_format = sheet_format
            .try_update(|draft| {
                draft.inheritance = crate::state::DrawingSheetInheritance::Explicit;
            })
            .map_err(|error| error.to_string())?;
    }

    let mut preset_saved = false;
    let mut personal_saved = false;
    capture_selected_personal_preset(
        &mut candidate,
        &personal,
        &transaction.draft.size,
        &mut sheet_format,
    )?;
    if transaction.draft.save_custom_preset {
        let result = save_one_off_custom_preset(
            &mut candidate,
            &mut personal,
            transaction.draft.custom_preset_scope,
            transaction.draft.custom_name.trim(),
            &mut sheet_format,
        )?;
        preset_saved = true;
        personal_saved = result;
    }
    remember_explicit_format(&mut candidate, &sheet_format)?;
    let mut personal_candidate = app.state.ui.preferences.clone();
    if personal != personal_before {
        personal_candidate.set_drawing_sheet_personal_preferences(personal)?;
    }

    let mut default_changed = false;
    if scope == PageSetupScope::CurrentSheetAndDefault {
        let default_authority = candidate.drawing_sheet_settings().default_format.clone();
        let default_format = enforce_managed_format_authority(
            sheet_format
                .as_drawing_sheet_default()
                .try_update(|draft| {
                    draft.inheritance = crate::state::DrawingSheetInheritance::ProjectDefault;
                })
                .map_err(|error| error.to_string())?,
            &default_authority,
        )?;
        if candidate.drawing_sheet_settings().default_format != default_format {
            let revision = candidate.revision();
            candidate
                .update_drawing_sheet_default(revision, default_format)
                .map_err(|error| error.to_string())?;
            default_changed = true;
        }
    }

    let mut changed = 0_usize;
    let mut applied_sheet_ids = Vec::new();
    let mut unchanged_sheet_ids = Vec::new();
    let mut skipped = Vec::new();
    {
        let catalog = candidate
            .sheet_catalog_mut(&authority.cell_view_key)
            .ok_or_else(|| "The governed sheet catalog is unavailable.".to_owned())?;
        for (sheet_id, sheet_revision) in targets {
            let target_format = catalog
                .find(sheet_id)
                .map(|sheet| sheet.page_format().clone())
                .ok_or_else(|| "A selected drawing sheet no longer exists.".to_owned())?;
            if scope == PageSetupScope::Document && format_is_organization_managed(&target_format) {
                let name = catalog
                    .find(sheet_id)
                    .map_or_else(|| sheet_id.to_string(), |sheet| sheet.name().to_owned());
                skipped.push(DrawingSheetTransactionSkip {
                    sheet_id,
                    sheet_name: name,
                    reason: "Organization-managed drawing-sheet format".to_owned(),
                });
                continue;
            }
            let applied_format = if scope == PageSetupScope::Document {
                // A document-wide format operation must retain the authored
                // identity and responsibility fields of every target sheet.
                sheet_format.with_target_sheet_title_fields(&target_format)
            } else {
                // Current-sheet Page Setup owns the visible title-field edits
                // in its draft as well as the physical format.
                sheet_format.clone()
            };
            let already_matches = catalog.find(sheet_id).is_some_and(|sheet| {
                sheet.page_format() == &applied_format
                    || (sheet.page_format().inheritance
                        == crate::state::DrawingSheetInheritance::ProjectDefault
                        && applied_format.inheritance
                            == crate::state::DrawingSheetInheritance::ProjectDefault)
            });
            if already_matches {
                unchanged_sheet_ids.push(sheet_id);
                continue;
            }
            catalog
                .update_sheet_page_format(sheet_id, sheet_revision, applied_format)
                .map_err(|error| error.to_string())?;
            changed += 1;
            applied_sheet_ids.push(sheet_id);
        }
    }
    let project_settings_changed =
        candidate.drawing_sheet_settings() != before.drawing_sheet_settings();
    if changed == 0 && !project_settings_changed {
        if skipped.is_empty() {
            return Ok("The drawing-sheet setup already matched the authored sheet.".to_owned());
        }
        let names = skipped
            .iter()
            .map(|entry| entry.sheet_name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        record_drawing_sheet_transaction(
            &mut candidate,
            DrawingSheetTransactionKind::PageSetup,
            &authority.cell_view_key,
            &sheet_format,
            selected_sheet_ids,
            applied_sheet_ids,
            unchanged_sheet_ids,
            skipped,
            false,
            false,
            false,
        )?;
        commit_governed_page_setup_candidate(app, before, candidate, personal_candidate)?;
        return Ok(format!(
            "No sheets changed. Organization-managed sheets were preserved and recorded: {names}."
        ));
    }

    record_drawing_sheet_transaction(
        &mut candidate,
        DrawingSheetTransactionKind::PageSetup,
        &authority.cell_view_key,
        &sheet_format,
        selected_sheet_ids,
        applied_sheet_ids,
        unchanged_sheet_ids,
        skipped.clone(),
        default_changed,
        preset_saved,
        project_settings_changed,
    )?;
    candidate.validate().map_err(|error| error.to_string())?;
    commit_governed_page_setup_candidate(app, before, candidate, personal_candidate)?;
    let mut effects = vec![format!(
        "{changed} {}",
        if changed == 1 { "sheet" } else { "sheets" }
    )];
    if default_changed {
        effects.push("project default".to_owned());
    }
    if preset_saved {
        effects.push(if personal_saved {
            "personal custom size plus exact project snapshot".to_owned()
        } else {
            "project custom-size preset".to_owned()
        });
    } else if project_settings_changed && !default_changed {
        effects.push("project drawing-sheet policy".to_owned());
    }
    if !skipped.is_empty() {
        effects.push(format!(
            "organization-managed sheets preserved: {}",
            skipped
                .iter()
                .map(|entry| entry.sheet_name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    Ok(format!(
        "Drawing-sheet setup applied to {}.",
        effects.join(", ")
    ))
}

#[allow(clippy::too_many_arguments)]
fn record_drawing_sheet_transaction(
    candidate: &mut crate::state::DesignManagementCatalog,
    kind: DrawingSheetTransactionKind,
    owner_cell_view_key: &str,
    source_format: &SchematicSheetFormat,
    selected_sheet_ids: Vec<SheetId>,
    applied_sheet_ids: Vec<SheetId>,
    unchanged_sheet_ids: Vec<SheetId>,
    skipped: Vec<DrawingSheetTransactionSkip>,
    project_default_changed: bool,
    project_preset_saved: bool,
    project_settings_changed: bool,
) -> Result<(), String> {
    let catalog_revision = candidate
        .revision()
        .checked_add(1)
        .ok_or_else(|| "Drawing-sheet receipt revision space is exhausted.".to_owned())?;
    let receipt = DrawingSheetTransactionReceipt {
        catalog_revision,
        kind,
        owner_cell_view_key: owner_cell_view_key.to_owned(),
        source_format_digest: source_format
            .content_digest()
            .map_err(|error| error.to_string())?,
        selected_sheet_ids,
        applied_sheet_ids,
        unchanged_sheet_ids,
        skipped,
        project_default_changed,
        project_preset_saved,
        project_settings_changed,
    };
    candidate
        .record_drawing_sheet_transaction(candidate.revision(), receipt)
        .map_err(|error| error.to_string())?;
    Ok(())
}

fn commit_governed_page_setup_candidate(
    app: &mut RSpiceApp,
    before: crate::state::DesignManagementCatalog,
    candidate: crate::state::DesignManagementCatalog,
    personal_candidate: crate::workbench::UserPreferences,
) -> Result<(), String> {
    candidate.validate().map_err(|error| error.to_string())?;
    let schematic_tx = app
        .state
        .prepare_design_management_schematic_transaction(&candidate)?;
    let owner = app.state.workspace.active_schematic_reference();
    let committed_revision = app
        .state
        .workspace
        .replace_design_management(candidate)
        .map_err(|error| error.to_string())?;
    app.state
        .apply_design_management_schematic_transaction(&schematic_tx);
    let after = app.state.workspace.design_management.clone();
    app.state
        .record_design_management_transaction(DesignManagementHistoryEntry {
            description: "Sheet format".to_owned(),
            owner,
            before,
            after,
            before_schematics: schematic_tx.before,
            after_schematics: schematic_tx.after,
            committed_revision,
        });
    app.state.ui.preferences = personal_candidate;
    Ok(())
}

fn format_is_organization_managed(format: &SchematicSheetFormat) -> bool {
    format.border == crate::state::DrawingSheetBorderTemplate::OrganizationManaged
        || format.title_block.template
            == crate::state::DrawingSheetTitleBlockTemplate::OrganizationManaged
}

fn enforce_managed_format_authority(
    candidate: SchematicSheetFormat,
    authority: &SchematicSheetFormat,
) -> Result<SchematicSheetFormat, String> {
    candidate
        .try_update(|draft| {
            if authority.border == crate::state::DrawingSheetBorderTemplate::OrganizationManaged {
                draft.border = authority.border;
                draft.zones.mode = authority.zones.mode;
            }
            if authority.title_block.template
                == crate::state::DrawingSheetTitleBlockTemplate::OrganizationManaged
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
        .try_update(|draft| {
            draft.inheritance = crate::state::DrawingSheetInheritance::Explicit;
        })
        .map_err(|error| error.to_string())?;
    right = right
        .try_update(|draft| {
            draft.inheritance = crate::state::DrawingSheetInheritance::Explicit;
        })
        .map_err(|error| error.to_string())?;
    Ok(left == right)
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
    choice: &super::state::SheetSizeChoice,
    sheet_format: &mut SchematicSheetFormat,
) -> Result<(), String> {
    let super::state::SheetSizeChoice::CapturedPreset {
        id,
        scope: crate::state::DrawingSheetPresetScope::User,
        ..
    } = choice
    else {
        return Ok(());
    };
    let source = personal.find_preset(id).ok_or_else(|| {
        format!("Personal custom size '{id}' is no longer available. Reopen Page Setup.")
    })?;
    if matches!(
        &source.format.authored_size,
        crate::state::AuthoredDrawingSheetSize::Custom {
            snapshot: crate::state::CustomDrawingSheetSnapshot {
                source_preset_unavailable: true,
                ..
            }
        }
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
    scope: crate::state::DrawingSheetPresetScope,
    name: &str,
    sheet_format: &mut SchematicSheetFormat,
) -> Result<bool, String> {
    if !matches!(
        sheet_format.authored_size,
        crate::state::AuthoredDrawingSheetSize::Custom { .. }
    ) {
        return Err("Only a one-off custom size can be saved as a reusable preset.".to_owned());
    }
    match scope {
        crate::state::DrawingSheetPresetScope::Project => {
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
            let preset_format = sheet_format
                .try_update(|draft| {
                    draft.inheritance = crate::state::DrawingSheetInheritance::Explicit;
                    if let crate::state::AuthoredDrawingSheetSize::Custom { snapshot } =
                        &mut draft.authored_size
                    {
                        snapshot.preset_id = Some(id.clone());
                        snapshot.name = name.to_owned();
                        snapshot.source_preset_unavailable = false;
                    }
                })
                .map_err(|error| error.to_string())?;
            let preset = crate::state::DrawingSheetPreset {
                id,
                name: name.to_owned(),
                scope: crate::state::DrawingSheetPresetScope::Project,
                format: preset_format.as_reusable_drawing_sheet_preset(),
            };
            candidate
                .publish_drawing_sheet_preset(candidate.revision(), preset.clone())
                .map_err(|error| error.to_string())?;
            *sheet_format = attach_project_preset(sheet_format, &preset)?;
            Ok(false)
        }
        crate::state::DrawingSheetPresetScope::User => {
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
            let format = sheet_format
                .try_update(|draft| {
                    draft.inheritance = crate::state::DrawingSheetInheritance::Explicit;
                    if let crate::state::AuthoredDrawingSheetSize::Custom { snapshot } =
                        &mut draft.authored_size
                    {
                        snapshot.preset_id = Some(id.clone());
                        snapshot.name = name.to_owned();
                        snapshot.source_preset_unavailable = false;
                    }
                })
                .map_err(|error| error.to_string())?;
            let source = crate::state::DrawingSheetPreset {
                id,
                name: name.to_owned(),
                scope: crate::state::DrawingSheetPresetScope::User,
                format: format.as_reusable_drawing_sheet_preset(),
            };
            personal.presets.push(source.clone());
            personal.validate()?;
            let captured =
                crate::workbench::app::dialogs::drawing_sheet_presets::capture_personal_preset_into_project(
                    candidate, &source,
                )?;
            *sheet_format = attach_project_preset(sheet_format, &captured)?;
            Ok(true)
        }
        crate::state::DrawingSheetPresetScope::Organization => {
            Err("Organization custom sizes are managed and cannot be created here.".to_owned())
        }
    }
}

fn attach_project_preset(
    sheet_format: &SchematicSheetFormat,
    preset: &crate::state::DrawingSheetPreset,
) -> Result<SchematicSheetFormat, String> {
    let crate::state::AuthoredDrawingSheetSize::Custom {
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
            draft.inheritance = crate::state::DrawingSheetInheritance::Explicit;
            let mut snapshot = source_snapshot.clone();
            snapshot.preset_id = Some(preset.id.clone());
            snapshot.name.clone_from(&preset.name);
            snapshot.source_preset_unavailable = false;
            draft.authored_size = crate::state::AuthoredDrawingSheetSize::Custom { snapshot };
        })
        .map_err(|error| error.to_string())
}

fn apply_legacy_sheet_setup(
    app: &mut RSpiceApp,
    validated: ValidatedDrawingSheetDraft,
) -> Result<String, String> {
    let transaction = app.state.dialogs.drawing_sheet_setup.clone();
    let before = app.state.workspace.design_management.clone();
    let mut candidate = before.clone();
    let personal_before = app
        .state
        .ui
        .preferences
        .drawing_sheet_personal_preferences();
    let mut personal = personal_before.clone();
    let owner_key = app.state.workspace.active_key();
    let sheet_id = candidate
        .bootstrap_for_cell_view(
            &owner_key,
            transaction.sheet_name.clone(),
            all_stable_object_ids(&app.state.schematic),
        )
        .map_err(|error| error.to_string())?;
    let mut format = validated.page_format;
    migrate_and_canonicalize_project_title_values(&mut candidate, &mut format, true)?;
    if transaction.draft.scope != PageSetupScope::CurrentSheet {
        format = format
            .try_update(|draft| {
                draft.inheritance = crate::state::DrawingSheetInheritance::Explicit;
            })
            .map_err(|error| error.to_string())?;
    }
    capture_selected_personal_preset(
        &mut candidate,
        &personal,
        &transaction.draft.size,
        &mut format,
    )?;
    if transaction.draft.save_custom_preset {
        save_one_off_custom_preset(
            &mut candidate,
            &mut personal,
            transaction.draft.custom_preset_scope,
            transaction.draft.custom_name.trim(),
            &mut format,
        )?;
    }
    remember_explicit_format(&mut candidate, &format)?;
    let mut personal_candidate = app.state.ui.preferences.clone();
    if personal != personal_before {
        personal_candidate.set_drawing_sheet_personal_preferences(personal)?;
    }
    if transaction.draft.scope == PageSetupScope::CurrentSheetAndDefault {
        let default_authority = candidate.drawing_sheet_settings().default_format.clone();
        let default_format = enforce_managed_format_authority(
            format
                .as_drawing_sheet_default()
                .try_update(|draft| {
                    draft.inheritance = crate::state::DrawingSheetInheritance::ProjectDefault;
                })
                .map_err(|error| error.to_string())?,
            &default_authority,
        )?;
        let revision = candidate.revision();
        candidate
            .update_drawing_sheet_default(revision, default_format)
            .map_err(|error| error.to_string())?;
    }
    let (sheet_revision, current_page_format) = candidate
        .sheet_catalog(&owner_key)
        .and_then(|catalog| catalog.find(sheet_id))
        .map(|sheet| (sheet.revision(), sheet.page_format().clone()))
        .ok_or_else(|| "The newly governed drawing sheet is unavailable.".to_owned())?;
    let receipt_format = format.clone();
    if current_page_format != format {
        candidate
            .sheet_catalog_mut(&owner_key)
            .ok_or_else(|| "The newly governed drawing-sheet catalog is unavailable.".to_owned())?
            .update_sheet_page_format(sheet_id, sheet_revision, format)
            .map_err(|error| error.to_string())?;
    }
    let project_settings_changed =
        candidate.drawing_sheet_settings() != before.drawing_sheet_settings();
    record_drawing_sheet_transaction(
        &mut candidate,
        DrawingSheetTransactionKind::PageSetup,
        &owner_key,
        &receipt_format,
        vec![sheet_id],
        vec![sheet_id],
        Vec::new(),
        Vec::new(),
        transaction.draft.scope == PageSetupScope::CurrentSheetAndDefault,
        transaction.draft.save_custom_preset,
        project_settings_changed,
    )?;
    candidate.validate().map_err(|error| error.to_string())?;
    let schematic_tx = app
        .state
        .prepare_design_management_schematic_transaction(&candidate)?;
    let owner = app.state.workspace.active_schematic_reference();
    let committed_revision = app
        .state
        .workspace
        .replace_design_management(candidate)
        .map_err(|error| error.to_string())?;
    app.state
        .apply_design_management_schematic_transaction(&schematic_tx);
    let after = app.state.workspace.design_management.clone();
    app.state
        .record_design_management_transaction(DesignManagementHistoryEntry {
            description: "Sheet format".to_owned(),
            owner,
            before,
            after,
            before_schematics: schematic_tx.before,
            after_schematics: schematic_tx.after,
            committed_revision,
        });
    app.state.ui.preferences = personal_candidate;
    Ok("The active schematic now owns a governed authored drawing sheet.".to_owned())
}

fn migrate_and_canonicalize_project_title_values(
    candidate: &mut crate::state::DesignManagementCatalog,
    format: &mut SchematicSheetFormat,
    replace_existing: bool,
) -> Result<(), String> {
    let mut settings = candidate.drawing_sheet_settings().clone();
    let mut changed = false;
    for id in crate::state::DrawingSheetTitleFieldId::PROJECT_OWNED {
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

fn remember_explicit_format(
    candidate: &mut crate::state::DesignManagementCatalog,
    format: &SchematicSheetFormat,
) -> Result<(), String> {
    let settings = candidate.drawing_sheet_settings();
    if !settings.remember_last_explicit_format
        || format.inheritance != crate::state::DrawingSheetInheritance::Explicit
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
    let mut ids = std::collections::BTreeSet::new();
    ids.extend(schematic.components.iter().map(|object| object.id));
    ids.extend(schematic.wires.iter().map(|object| object.id));
    ids.extend(schematic.buses.iter().map(|object| object.id));
    ids.extend(schematic.bus_taps.iter().map(|object| object.id));
    ids.extend(schematic.junctions.iter().map(|object| object.id));
    ids.extend(schematic.net_labels.iter().map(|object| object.id));
    ids.extend(schematic.design_notes.iter().map(|object| object.id));
    ids.extend(
        schematic
            .documentation_shapes
            .iter()
            .map(|object| object.id),
    );
    ids.extend(schematic.probes.iter().map(|object| object.id));
    ids.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{DrawingSheetStandard, SheetDefinition, SheetPortPolicy, SheetTemplate};

    fn governed_app_with_sheets(count: usize) -> (RSpiceApp, String, Vec<crate::state::SheetId>) {
        let mut app = RSpiceApp::test_instance();
        let key = app.state.workspace.active_key();
        let first = app
            .state
            .workspace
            .design_management
            .bootstrap_for_cell_view(&key, "Sheet 1", [])
            .unwrap();
        let mut ids = vec![first];
        for index in 1..count {
            let catalog = app
                .state
                .workspace
                .design_management
                .sheet_catalog_mut(&key)
                .unwrap();
            let id = catalog
                .create_sheet(
                    SheetDefinition {
                        name: format!("Sheet {}", index + 1),
                        template: SheetTemplate::AnalogSchematic,
                        port_policy: SheetPortPolicy::TypedOffSheetPorts,
                        explicit_page_number: Some((index + 1) as u32),
                    },
                    ids.last().copied(),
                )
                .unwrap();
            ids.push(id);
        }
        (app, key, ids)
    }

    #[test]
    fn page_setup_is_owned_only_by_schematic_like_design_routes() {
        let app = RSpiceApp::test_instance();
        assert!(drawing_sheet_setup_available(&app));
    }

    #[test]
    fn page_setup_availability_fails_closed_when_live_schematic_is_read_only() {
        let mut app = RSpiceApp::test_instance();
        assert!(drawing_sheet_setup_available(&app));

        app.state.workbench.safe_mode.activate(
            crate::workbench::state::LocalSafeModeOptions {
                open_project_read_only: true,
                ..Default::default()
            },
            String::new(),
        );

        assert!(app.state.schematic_edit_read_only());
        assert!(!drawing_sheet_setup_available(&app));
        assert!(!open_drawing_sheet_setup_for_state(&mut app.state));
        assert!(!app.state.dialogs.drawing_sheet_setup.open);
        assert_eq!(
            crate::workbench::commands::vocabulary::Command::PageSetup.availability(&app),
            crate::workbench::commands::CommandAvailability::Disabled(
                "the active schematic is read-only"
            )
        );
    }

    #[test]
    fn stale_authority_rejects_active_document_drift() {
        let mut state = AppState::default();
        let authority = DrawingSheetAuthority {
            edit: crate::workbench::app::SchematicEditAuthority::capture(&state),
            cell_view_key: state.workspace.active_key(),
            design_management_revision: state.workspace.design_management.revision(),
            personal_preferences_digest: Some(
                state
                    .ui
                    .preferences
                    .drawing_sheet_personal_preferences()
                    .semantic_digest(),
            ),
            governed: None,
        };
        state.schematic.grid_size += 1;
        assert!(validate_drawing_sheet_authority(&state, &authority).is_err());
    }

    #[test]
    fn stale_authority_rejects_project_drawing_sheet_policy_drift() {
        let mut app = RSpiceApp::test_instance();
        open_drawing_sheet_setup(&mut app);
        let authority = app
            .state
            .dialogs
            .drawing_sheet_setup
            .authority
            .clone()
            .unwrap();
        let mut settings = app
            .state
            .workspace
            .design_management
            .drawing_sheet_settings()
            .clone();
        settings.new_sheet_policy = crate::state::DrawingSheetNewSheetPolicy::Ask;
        let revision = app.state.workspace.design_management.revision();
        app.state
            .workspace
            .design_management
            .update_drawing_sheet_settings(revision, settings)
            .unwrap();
        assert!(validate_drawing_sheet_authority(&app.state, &authority).is_err());
    }

    #[test]
    fn governed_apply_is_one_exact_undoable_presentation_transaction() {
        let (mut app, key, ids) = governed_app_with_sheets(1);
        let topology = app.state.schematic.topology_version();
        let drc_version = app.state.dialogs.drc_checked_version;
        let retained_runs = app.state.simulation.runs.len();
        open_drawing_sheet_setup(&mut app);
        app.state.dialogs.drawing_sheet_setup.draft.margin_top = "12".to_owned();
        let expected = app
            .state
            .dialogs
            .drawing_sheet_setup
            .draft
            .validate()
            .unwrap()
            .page_format
            .try_update(|draft| {
                draft.inheritance = crate::state::DrawingSheetInheritance::Explicit;
            })
            .unwrap();

        apply_drawing_sheet_setup(&mut app).unwrap();

        assert_eq!(
            app.state
                .workspace
                .design_management
                .sheet_catalog(&key)
                .unwrap()
                .find(ids[0])
                .unwrap()
                .page_format(),
            &expected
        );
        assert!(app.state.can_undo_project_design());
        assert!(app.state.workspace.project_metadata_dirty);
        assert_eq!(app.state.schematic.topology_version(), topology);
        assert_eq!(app.state.dialogs.drc_checked_version, drc_version);
        assert_eq!(app.state.simulation.runs.len(), retained_runs);
        let receipt = app
            .state
            .workspace
            .design_management
            .drawing_sheet_settings()
            .transaction_receipts
            .last()
            .unwrap();
        assert_eq!(receipt.kind, DrawingSheetTransactionKind::PageSetup);
        assert_eq!(receipt.applied_sheet_ids, vec![ids[0]]);
        assert!(app.state.undo_project_design().unwrap().is_some());
        assert!(!app.state.can_undo_project_design());
    }

    #[test]
    fn inherited_sheet_opens_with_the_effective_project_format_and_keeps_its_source() {
        let (mut app, key, ids) = governed_app_with_sheets(1);
        let catalog = app
            .state
            .workspace
            .design_management
            .sheet_catalog_mut(&key)
            .unwrap();
        let revision = catalog.find(ids[0]).unwrap().revision();
        let inherited = SchematicSheetFormat::from_standard(
            DrawingSheetStandard::AnsiD,
            crate::state::SchematicPageOrientation::Portrait,
        )
        .try_update(|draft| {
            draft.inheritance = crate::state::DrawingSheetInheritance::ProjectDefault;
            draft
                .title_block
                .fields
                .get_mut(&crate::state::DrawingSheetTitleFieldId::SheetTitle)
                .unwrap()
                .value = "Sheet 1".to_owned();
        })
        .unwrap();
        catalog
            .update_sheet_page_format(ids[0], revision, inherited)
            .unwrap();

        open_drawing_sheet_setup(&mut app);

        let setup = &app.state.dialogs.drawing_sheet_setup;
        assert_eq!(
            setup.draft.inheritance,
            crate::state::DrawingSheetInheritance::ProjectDefault
        );
        assert_eq!(
            setup.draft.size,
            super::super::state::SheetSizeChoice::Standard(DrawingSheetStandard::IsoA4)
        );
        assert!(!setup.is_dirty());
    }

    #[test]
    fn document_scope_updates_every_sheet_in_one_transaction() {
        let (mut app, key, ids) = governed_app_with_sheets(3);
        open_drawing_sheet_setup(&mut app);
        let setup = &mut app.state.dialogs.drawing_sheet_setup;
        setup.draft.scope = PageSetupScope::Document;
        setup
            .draft
            .apply_size_choice(super::super::state::SheetSizeChoice::Standard(
                DrawingSheetStandard::IsoA3,
            ));
        let expected = setup.draft.validate().unwrap().page_format;

        apply_drawing_sheet_setup(&mut app).unwrap();

        let catalog = app
            .state
            .workspace
            .design_management
            .sheet_catalog(&key)
            .unwrap();
        assert!(ids.iter().all(|id| {
            let format = catalog.find(*id).unwrap().page_format();
            format.authored_size == expected.authored_size
                && format.orientation == expected.orientation
        }));
        assert_eq!(
            catalog
                .find(ids[1])
                .unwrap()
                .page_format()
                .title_block
                .fields[&crate::state::DrawingSheetTitleFieldId::SheetTitle]
                .value,
            "Sheet 2"
        );
        assert!(app.state.can_undo_project_design());
    }

    #[test]
    fn document_scope_preserves_organization_managed_sheets_and_names_them() {
        let (mut app, key, ids) = governed_app_with_sheets(2);
        let managed_format = SchematicSheetFormat::default()
            .try_update(|draft| {
                draft.border = crate::state::DrawingSheetBorderTemplate::OrganizationManaged;
                draft.marks =
                    crate::state::DrawingSheetBorderTemplate::OrganizationManaged.default_marks();
                draft
                    .title_block
                    .fields
                    .get_mut(&crate::state::DrawingSheetTitleFieldId::SheetTitle)
                    .unwrap()
                    .value = "Sheet 2".to_owned();
            })
            .unwrap();
        {
            let catalog = app
                .state
                .workspace
                .design_management
                .sheet_catalog_mut(&key)
                .unwrap();
            let revision = catalog.find(ids[1]).unwrap().revision();
            catalog
                .update_sheet_page_format(ids[1], revision, managed_format.clone())
                .unwrap();
            catalog.set_active(ids[0]).unwrap();
        }
        open_drawing_sheet_setup(&mut app);
        let setup = &mut app.state.dialogs.drawing_sheet_setup;
        setup.draft.scope = PageSetupScope::Document;
        setup
            .draft
            .apply_size_choice(super::super::state::SheetSizeChoice::Standard(
                DrawingSheetStandard::IsoA3,
            ));

        let message = apply_drawing_sheet_setup(&mut app).unwrap();

        let catalog = app
            .state
            .workspace
            .design_management
            .sheet_catalog(&key)
            .unwrap();
        assert_eq!(
            catalog
                .find(ids[0])
                .unwrap()
                .page_format()
                .authored_size
                .label(),
            "ISO A3"
        );
        assert_eq!(catalog.find(ids[1]).unwrap().page_format(), &managed_format);
        assert!(message.contains("Sheet 2"));
        assert!(message.contains("organization-managed"));
    }

    #[test]
    fn managed_border_and_title_authorities_are_reapplied_at_commit() {
        let (mut app, key, ids) = governed_app_with_sheets(1);
        let managed = SchematicSheetFormat::default()
            .try_update(|draft| {
                draft.apply_border_template(
                    crate::state::DrawingSheetBorderTemplate::OrganizationManaged,
                );
                draft.zones.mode = crate::state::DrawingSheetZoneMode::Custom;
                draft.zones.custom_columns = Some(6);
                draft.zones.custom_rows = Some(4);
                draft.title_block.template =
                    crate::state::DrawingSheetTitleBlockTemplate::OrganizationManaged;
                draft
                    .title_block
                    .fields
                    .get_mut(&crate::state::DrawingSheetTitleFieldId::SheetTitle)
                    .unwrap()
                    .value = "Managed sheet".to_owned();
            })
            .unwrap();
        {
            let catalog = app
                .state
                .workspace
                .design_management
                .sheet_catalog_mut(&key)
                .unwrap();
            let revision = catalog.find(ids[0]).unwrap().revision();
            catalog
                .update_sheet_page_format(ids[0], revision, managed.clone())
                .unwrap();
        }
        open_drawing_sheet_setup(&mut app);
        let draft = &mut app.state.dialogs.drawing_sheet_setup.draft;
        draft.border = super::super::state::BorderTemplateChoice::Plain;
        draft.zone_columns = 9;
        draft.zone_rows = 7;
        draft.zone_labels = super::super::state::ZoneLabelsChoice::Coordinates;
        draft.registration_marks = false;
        draft.title_block = super::super::state::TitleBlockTemplateChoice::Compact;
        draft.title_block_anchor = super::super::state::TitleBlockAnchorChoice::TopRight;
        draft
            .title_fields
            .get_mut(&crate::state::DrawingSheetTitleFieldId::SheetTitle)
            .unwrap()
            .value = "Edited managed sheet".to_owned();
        draft.apply_size_choice(super::super::state::SheetSizeChoice::Standard(
            DrawingSheetStandard::IsoA3,
        ));

        apply_drawing_sheet_setup(&mut app).unwrap();

        let saved = app
            .state
            .workspace
            .design_management
            .sheet_catalog(&key)
            .unwrap()
            .find(ids[0])
            .unwrap()
            .page_format();
        assert_eq!(
            saved.border,
            crate::state::DrawingSheetBorderTemplate::OrganizationManaged
        );
        assert_eq!(saved.zones.mode, managed.zones.mode);
        assert_eq!(saved.zones.custom_columns, Some(9));
        assert_eq!(saved.zones.custom_rows, Some(7));
        assert_eq!(
            saved.zones.labels,
            crate::state::DrawingSheetZoneLabels::Coordinates
        );
        assert!(!saved.marks.registration);
        assert_eq!(
            saved.title_block.template,
            crate::state::DrawingSheetTitleBlockTemplate::OrganizationManaged
        );
        assert_eq!(
            saved.title_block.fields[&crate::state::DrawingSheetTitleFieldId::SheetTitle].value,
            "Edited managed sheet"
        );
        assert_eq!(
            saved.title_block.anchor,
            crate::state::DrawingSheetTitleBlockAnchor::TopRight
        );
        assert_eq!(saved.authored_size.label(), "ISO A3");
    }

    #[test]
    fn page_setup_preview_counts_only_document_sheets_that_can_be_written() {
        let (mut app, key, ids) = governed_app_with_sheets(3);
        {
            let catalog = app
                .state
                .workspace
                .design_management
                .sheet_catalog_mut(&key)
                .unwrap();
            let revision = catalog.find(ids[1]).unwrap().revision();
            let managed = catalog
                .find(ids[1])
                .unwrap()
                .page_format()
                .try_update(|draft| {
                    draft.apply_border_template(
                        crate::state::DrawingSheetBorderTemplate::OrganizationManaged,
                    );
                })
                .unwrap();
            catalog
                .update_sheet_page_format(ids[1], revision, managed)
                .unwrap();
            catalog.set_active(ids[0]).unwrap();
        }

        open_drawing_sheet_setup(&mut app);

        let setup = &app.state.dialogs.drawing_sheet_setup;
        assert_eq!(setup.sheet_count, 3);
        assert_eq!(setup.writable_sheet_count, 2);
        assert_eq!(setup.managed_sheet_names, vec!["Sheet 2"]);
    }

    #[test]
    fn default_and_custom_preset_scope_publish_real_catalog_state() {
        let (mut app, key, ids) = governed_app_with_sheets(1);
        open_drawing_sheet_setup(&mut app);
        let setup = &mut app.state.dialogs.drawing_sheet_setup;
        setup.draft.size = super::super::state::SheetSizeChoice::Custom;
        setup.draft.custom_name = "Lab panel".to_owned();
        setup.draft.width = "250".to_owned();
        setup.draft.height = "400".to_owned();
        setup.draft.save_custom_preset = true;
        setup.draft.scope = PageSetupScope::CurrentSheetAndDefault;

        apply_drawing_sheet_setup(&mut app).unwrap();

        let management = &app.state.workspace.design_management;
        let format = management
            .sheet_catalog(&key)
            .unwrap()
            .find(ids[0])
            .unwrap()
            .page_format();
        assert_eq!(format.authored_size.label(), "Lab panel",);
        assert_eq!(management.drawing_sheet_settings().presets.len(), 1);
        assert_eq!(
            management
                .drawing_sheet_settings()
                .default_format
                .inheritance,
            crate::state::DrawingSheetInheritance::ProjectDefault
        );
    }

    #[test]
    fn personal_preset_use_captures_an_exact_project_snapshot_on_apply() {
        let (mut app, key, ids) = governed_app_with_sheets(1);
        let personal = crate::state::DrawingSheetPreset {
            id: "personal-review-strip".to_owned(),
            name: "Review strip".to_owned(),
            scope: crate::state::DrawingSheetPresetScope::User,
            format: crate::state::SchematicSheetFormat::try_custom(
                "Review strip",
                210_001,
                594_002,
                crate::state::SchematicPageOrientation::Portrait,
            )
            .unwrap()
            .try_update(|draft| {
                if let crate::state::AuthoredDrawingSheetSize::Custom { snapshot } =
                    &mut draft.authored_size
                {
                    snapshot.preset_id = Some("personal-review-strip".to_owned());
                }
            })
            .unwrap(),
        };
        let mut personal_settings = app
            .state
            .ui
            .preferences
            .drawing_sheet_personal_preferences();
        personal_settings.presets.push(personal.clone());
        app.state
            .ui
            .preferences
            .set_drawing_sheet_personal_preferences(personal_settings)
            .unwrap();

        open_drawing_sheet_setup_with_preset(&mut app, personal).unwrap();
        apply_drawing_sheet_setup(&mut app).unwrap();

        let project_preset = app
            .state
            .workspace
            .design_management
            .drawing_sheet_settings()
            .presets
            .first()
            .unwrap();
        assert_eq!(
            project_preset.scope,
            crate::state::DrawingSheetPresetScope::Project
        );
        assert_eq!(
            project_preset.format.portrait_dimensions_um(),
            (210_001, 594_002)
        );
        let sheet = app
            .state
            .workspace
            .design_management
            .sheet_catalog(&key)
            .unwrap()
            .find(ids[0])
            .unwrap();
        let crate::state::AuthoredDrawingSheetSize::Custom { snapshot } =
            &sheet.page_format().authored_size
        else {
            panic!("captured personal size must remain a custom snapshot");
        };
        assert_eq!(
            snapshot.preset_id.as_deref(),
            Some(project_preset.id.as_str())
        );
    }

    #[test]
    fn personal_preset_authority_change_fails_closed_while_page_setup_is_open() {
        let (mut app, _, _) = governed_app_with_sheets(1);
        open_drawing_sheet_setup(&mut app);
        let mut personal = app
            .state
            .ui
            .preferences
            .drawing_sheet_personal_preferences();
        personal.presets.push(
            crate::state::DrawingSheetPreset {
                id: "late-personal-size".to_owned(),
                name: "Late personal size".to_owned(),
                scope: crate::state::DrawingSheetPresetScope::User,
                format: crate::state::SchematicSheetFormat::try_custom(
                    "Late personal size",
                    250_000,
                    400_000,
                    crate::state::SchematicPageOrientation::Portrait,
                )
                .unwrap(),
            }
            .normalized_for_storage()
            .unwrap(),
        );
        app.state
            .ui
            .preferences
            .set_drawing_sheet_personal_preferences(personal)
            .unwrap();

        let error = apply_drawing_sheet_setup(&mut app).unwrap_err();

        assert!(error.contains("Personal custom sheet sizes changed"));
    }

    #[test]
    fn nested_support_resume_retains_the_isolated_page_setup_draft() {
        let (mut app, _, _) = governed_app_with_sheets(1);
        open_drawing_sheet_setup(&mut app);
        app.state.dialogs.drawing_sheet_setup.draft.width = "333.125".to_owned();
        app.state.dialogs.drawing_sheet_setup.draft.custom_name =
            "Uncommitted working size".to_owned();
        let draft = app.state.dialogs.drawing_sheet_setup.draft.clone();
        app.state.dialogs.drawing_sheet_setup.open = false;
        app.state.dialogs.drawing_sheet_setup.support_suspended = true;

        resume_drawing_sheet_setup_after_support(&mut app.state).unwrap();

        assert!(app.state.dialogs.drawing_sheet_setup.open);
        assert!(!app.state.dialogs.drawing_sheet_setup.support_suspended);
        assert_eq!(app.state.dialogs.drawing_sheet_setup.draft, draft);
    }

    #[test]
    fn bootstrap_resume_fails_closed_after_the_active_view_changes() {
        let mut app = RSpiceApp::test_instance();
        open_drawing_sheet_setup(&mut app);
        app.state.dialogs.drawing_sheet_setup.open = false;
        app.state.dialogs.drawing_sheet_setup.support_suspended = true;
        app.state.workspace.active_view.cell = "different_cell".to_owned();

        let error = resume_drawing_sheet_setup_after_support(&mut app.state).unwrap_err();

        assert!(error.contains("active cell/view changed"));
        assert!(!app.state.dialogs.drawing_sheet_setup.open);
        assert!(app.state.dialogs.drawing_sheet_setup.support_suspended);
    }

    #[test]
    fn unchanged_apply_creates_no_history_and_read_only_authority_fails_closed() {
        let (mut app, _, _) = governed_app_with_sheets(1);
        open_drawing_sheet_setup(&mut app);
        assert!(apply_drawing_sheet_setup(&mut app).is_ok());
        assert!(!app.state.can_undo_project_design());

        open_drawing_sheet_setup(&mut app);
        app.state.workbench.safe_mode.activate(
            crate::workbench::state::LocalSafeModeOptions {
                open_project_read_only: true,
                ..Default::default()
            },
            String::new(),
        );
        assert!(apply_drawing_sheet_setup(&mut app).is_err());
        assert!(!app.state.can_undo_project_design());
    }

    #[test]
    fn bootstrap_apply_saves_staged_sheet_and_project_title_fields_consistently() {
        let mut app = RSpiceApp::test_instance();
        let key = app.state.workspace.active_key();
        let mut settings = app
            .state
            .workspace
            .design_management
            .drawing_sheet_settings()
            .clone();
        settings.title_block_field_values.insert(
            crate::state::DrawingSheetTitleFieldId::Organization,
            "Previous organization".to_owned(),
        );
        let revision = app.state.workspace.design_management.revision();
        app.state
            .workspace
            .design_management
            .update_drawing_sheet_settings(revision, settings)
            .unwrap();
        open_drawing_sheet_setup(&mut app);
        let fields = &mut app.state.dialogs.drawing_sheet_setup.draft.title_fields;
        fields
            .get_mut(&crate::state::DrawingSheetTitleFieldId::SheetTitle)
            .unwrap()
            .value = "Bootstrap title".to_owned();
        fields
            .get_mut(&crate::state::DrawingSheetTitleFieldId::Organization)
            .unwrap()
            .value = "Example Labs".to_owned();

        apply_drawing_sheet_setup(&mut app).unwrap();

        let catalog = app
            .state
            .workspace
            .design_management
            .sheet_catalog(&key)
            .unwrap();
        let sheet = catalog.active().unwrap();
        assert_eq!(
            sheet.page_format().title_block.fields
                [&crate::state::DrawingSheetTitleFieldId::SheetTitle]
                .value,
            "Bootstrap title"
        );
        assert_eq!(
            sheet.page_format().title_block.fields
                [&crate::state::DrawingSheetTitleFieldId::Organization]
                .value,
            ""
        );
        assert_eq!(
            app.state
                .workspace
                .design_management
                .drawing_sheet_settings()
                .title_block_field_values[&crate::state::DrawingSheetTitleFieldId::Organization],
            "Example Labs"
        );
    }

    #[test]
    fn bootstrap_apply_publishes_an_unchanged_inherited_format() {
        let mut app = RSpiceApp::test_instance();
        let key = app.state.workspace.active_key();
        open_drawing_sheet_setup(&mut app);
        assert!(!app.state.dialogs.drawing_sheet_setup.is_dirty());

        apply_drawing_sheet_setup(&mut app).unwrap();

        let catalog = app
            .state
            .workspace
            .design_management
            .sheet_catalog(&key)
            .expect("bootstrap publishes the first governed sheet");
        assert_eq!(catalog.sheets().len(), 1);
        assert!(app.state.can_undo_project_design());
    }
}
