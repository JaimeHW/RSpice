//! Design management dialog flow.

use super::manager::design_management_manager_body;
use super::operations::commit_design_management_subflow;
use super::subflows::design_management_subflow_body;
use super::widgets::parse_reserved_ranges;
use super::*;
use crate::diagnostics::{LogSeverity, LogSource};

/// Draw Design management while it is open, and carry out what its body
/// and its primary and ghost actions asked for.
pub(in crate::workbench) fn render_design_management_dialog(
    parts: &mut DesignManagementParts<'_>,
    ctx: &Context,
) {
    if !parts.dialog.open {
        return;
    }
    let page = parts.dialog.page;
    let manager_dirty = parts.dialog.dirty();
    let discard = parts.dialog.discard_confirmation;
    // A new edit invalidates a previously displayed discard confirmation.
    // Capture its exact subject only while that confirmation is visible.
    let discard_baseline = discard.then(|| {
        let draft = &*parts.dialog;
        (draft.inputs.clone(), draft.draft.clone())
    });
    let validation = validate_design_management_page(parts.dialog, parts.model, page);
    let write_allowed = !parts.model.safe_mode.project_read_only();
    let primary_enabled = write_allowed
        && validation.is_ok()
        && if page == DesignManagementPage::Manager {
            manager_dirty
        } else {
            true
        };
    let description = if page == DesignManagementPage::Manager {
        MANAGER_DESCRIPTION
    } else {
        "Review the source, the proposed change, the resulting revision, and the rule it must keep before applying this operation to the draft."
    };
    let initial_height = if page == DesignManagementPage::Manager {
        MANAGER_INITIAL_HEIGHT
    } else {
        SUBFLOW_INITIAL_HEIGHT
    };
    // The manager and its subflows are one width, so moving between them
    // never resizes the dialog, and each pane's tables keep their headings.
    let size = DialogSize::WideWorkflow;
    let ghost = if discard {
        "Discard changes"
    } else if page == DesignManagementPage::Manager {
        "Close"
    } else {
        "Cancel"
    };
    let error = parts.dialog.error.clone();
    let mut body_scroll_offset = parts.dialog.body_scroll_offset;
    let mut dialog = Dialog::new(page.title(), page.primary())
        .description(description)
        .size(size)
        .initial_height(initial_height)
        .flush_body()
        .overlay_scrollbar()
        .ghost(ghost)
        .primary_enabled(primary_enabled)
        .primary_on_enter(false)
        .initial_focus(DialogInitialFocus::BodyControl)
        .body_scroll_offset(&mut body_scroll_offset);
    let drawing_sheet_personal = parts.model.preferences.drawing_sheet_personal_preferences();
    if discard {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            "Discard uncommitted design-management changes?",
            "The live project remains unchanged. Subordinate operations already added to this manager draft are discarded together.",
        );
    } else if let Some(error) = error
        .as_deref()
        .or_else(|| validation.as_ref().err().map(String::as_str))
    {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            "Design transaction cannot continue",
            error,
        );
    }

    let mut body_action = DesignManagementBodyAction::None;
    let mut response = dialog.show_transaction(ctx, |ui| {
        let response = if page == DesignManagementPage::Manager {
            body_action = design_management_manager_body(
                ui,
                &mut *parts.dialog,
                &parts.model.workspace,
                &parts.model.library_manager,
                &parts.model.schematic,
                write_allowed,
            );
            None
        } else {
            Some(design_management_subflow_body(
                ui,
                &mut *parts.dialog,
                &parts.model.workspace,
                &parts.model.library_manager,
                &parts.model.schematic,
                &drawing_sheet_personal,
                write_allowed,
            ))
        };
        response.map(|response| response.id)
    });
    parts.dialog.body_scroll_offset = body_scroll_offset;
    if let Some((inputs, catalog)) = discard_baseline {
        let draft = &mut *parts.dialog;
        if inputs != draft.inputs || catalog != draft.draft {
            draft.discard_confirmation = false;
        }
    }
    handle_design_management_body_action(parts, body_action);
    if !parts.dialog.open || parts.dialog.page != page {
        return;
    }
    match response.choice {
        DialogChoice::Primary => {
            let result = if page == DesignManagementPage::Manager {
                publish_design_management_draft(parts)
            } else {
                commit_design_management_subflow(parts, page)
            };
            if let Err(error) = result {
                parts.dialog.error = Some(error);
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            let dirty = if page == DesignManagementPage::Manager {
                parts.dialog.dirty()
            } else {
                parts.dialog.subflow_dirty()
            };
            if dirty && !parts.dialog.discard_confirmation {
                parts.dialog.discard_confirmation = true;
                response.retain_cancel_focus(DialogInitialFocus::Ghost);
            } else if page == DesignManagementPage::Manager {
                parts.dialog.close_and_discard();
                crate::workbench::app::close_design_management_dialog_route(parts.wb);
            } else {
                let active_sheet = parts
                    .dialog
                    .draft
                    .as_ref()
                    .and_then(|catalog| catalog.sheet_catalog(&parts.dialog.owner_key))
                    .and_then(|catalog| catalog.active_sheet_id());
                let configurations = parts.model.workspace.content.configuration_sets.clone();
                parts.dialog.reset_inputs_for_page(
                    DesignManagementPage::Manager,
                    Some(&configurations),
                    active_sheet,
                );
            }
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

fn handle_design_management_body_action(
    parts: &mut DesignManagementParts<'_>,
    action: DesignManagementBodyAction,
) {
    match action {
        DesignManagementBodyAction::None => {}
        DesignManagementBodyAction::Open(page) => {
            let active_sheet = parts
                .dialog
                .draft
                .as_ref()
                .and_then(|catalog| catalog.sheet_catalog(&parts.dialog.owner_key))
                .and_then(|catalog| catalog.active_sheet_id());
            let configurations = parts.model.workspace.content.configuration_sets.clone();
            parts
                .dialog
                .reset_inputs_for_page(page, Some(&configurations), active_sheet);
        }
        DesignManagementBodyAction::OpenConfigurationSets => {
            if parts.dialog.dirty() {
                parts.dialog.error = Some(
                    "Apply or discard this Design Management draft before opening configuration bindings."
                        .to_owned(),
                );
                return;
            }
            parts.dialog.close_and_discard();
            crate::workbench::app::close_design_management_dialog_route(parts.wb);
            crate::workbench::app::dialogs::project::configuration_dialogs::show_configuration_page_in(
                parts.platform,
                parts.wb,
                parts.project_page,
            );
        }
        DesignManagementBodyAction::OpenConfigurationBinding => {
            if parts.dialog.dirty() {
                parts.dialog.error = Some(
                    "Apply or discard this Design Management draft before opening configuration bindings."
                        .to_owned(),
                );
                return;
            }
            parts.dialog.close_and_discard();
            crate::workbench::app::close_design_management_dialog_route(parts.wb);
            crate::workbench::app::dialogs::project::configuration_dialogs::show_configuration_page_in(
                parts.platform,
                parts.wb,
                parts.project_page,
            );
            *parts.configuration_tab =
                crate::workbench::project::configuration_state::ConfigurationTab::Instances;
        }
    }
}

fn publish_design_management_draft(parts: &mut DesignManagementParts<'_>) -> Result<(), String> {
    let before = (*parts.model.workspace.content.design_management).clone();
    let draft = parts
        .dialog
        .draft
        .clone()
        .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
    draft.validate().map_err(|error| error.to_string())?;
    let schematic_tx = parts
        .model
        .prepare_design_management_schematic_transaction(&draft)?;
    let owner = parts.model.workspace.content.active_schematic_reference();
    let committed_revision = parts
        .model
        .workspace
        .content
        .replace_design_management(draft)
        .map_err(|error| error.to_string())?;
    parts
        .model
        .apply_design_management_schematic_transaction(&schematic_tx);
    let candidate = (*parts.model.workspace.content.design_management).clone();
    parts
        .model
        .record_design_management_transaction(DesignManagementHistoryEntry {
            description: "apply reviewed design-management changes".to_owned(),
            owner,
            before,
            after: candidate.clone(),
            before_schematics: schematic_tx.before,
            after_schematics: schematic_tx.after,
            references: schematic_tx.references,
            committed_revision,
        });
    parts.model.design_execution_epoch = parts.model.design_execution_epoch.wrapping_add(1);
    // The next two calls are what RSpiceApp::invalidate_simulation_preflight does.
    parts.runs.preflight.invalidate();
    parts.run_controller.clear_prepared_run();
    parts
        .model
        .netlist_artifacts
        .current_generation_input_digest = None;
    parts.model.record(
        LogSeverity::Info,
        LogSource::Studio,
        format!(
            "Applied design management changes (revision {}).",
            committed_revision.get()
        ),
    );
    let owner_key = parts.dialog.owner_key.clone();
    let selection_object_ids = parts.dialog.selection_object_ids.clone();
    let selection_summary = parts.dialog.selection_summary.clone();
    let all_object_ids = parts.dialog.all_object_ids.clone();
    let default_sheet_name = parts.dialog.default_sheet_name.clone();
    parts.dialog.open(
        &candidate,
        owner_key,
        selection_object_ids,
        all_object_ids,
        selection_summary,
        default_sheet_name,
    )?;
    Ok(())
}

pub(super) fn validate_design_management_page(
    dialog: &DesignManagementDialogState,
    model: &Model,
    page: DesignManagementPage,
) -> Result<(), String> {
    let draft = dialog
        .draft
        .as_ref()
        .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
    draft.validate().map_err(|error| error.to_string())?;
    match page {
        DesignManagementPage::Manager => {
            if !dialog.dirty() {
                return Err("No reviewed design changes are pending.".to_owned());
            }
        }
        DesignManagementPage::NewSheet => {
            if dialog.inputs.sheet_name.trim().is_empty() {
                return Err("Sheet name is required.".to_owned());
            }
        }
        DesignManagementPage::MoveSelection => {
            if dialog.selection_object_ids.is_empty() {
                return Err(
                    "Select one or more complete schematic objects before moving them.".to_owned(),
                );
            }
            let destination = dialog
                .inputs
                .move_destination
                .ok_or_else(|| "Destination sheet is required.".to_owned())?;
            let source = dialog
                .selection_object_ids
                .iter()
                .find_map(|id| draft.sheet_for_object_or_active(&dialog.owner_key, *id));
            if source == Some(destination) {
                return Err("Destination must differ from the source sheet.".to_owned());
            }
            if dialog.inputs.move_hierarchy_effect == MoveHierarchyEffect::SameCell
                && dialog.inputs.move_boundary_policy == MoveBoundaryPolicy::Block
            {
                return Err("Boundary-net policy blocks this move.".to_owned());
            }
            let selected_components = model
                .schematic
                .document()
                .components
                .iter()
                .filter(|component| dialog.selection_object_ids.contains(&component.id))
                .count();
            if selected_components != dialog.selection_object_ids.len()
                || selected_components == 0
                || model.schematic.session.editor.selection.count()
                    != model.schematic.session.editor.selection.components.len()
            {
                return Err(
                    "Connectivity-preserving sheet and hierarchy moves require the same complete instance-only selection captured when Design Management opened."
                        .to_owned(),
                );
            }
        }
        DesignManagementPage::NewVariant => {
            if dialog.inputs.variant_name.trim().is_empty() {
                return Err("Variant name is required.".to_owned());
            }
        }
        DesignManagementPage::CompareVariants => {
            let reference = dialog
                .inputs
                .compare_reference
                .ok_or_else(|| "Reference variant is required.".to_owned())?;
            let comparison = dialog
                .inputs
                .compare_target
                .ok_or_else(|| "Comparison variant is required.".to_owned())?;
            if reference == comparison {
                return Err("Reference and comparison variants must be different.".to_owned());
            }
        }
        DesignManagementPage::VariantMatrix => {
            if draft.variants().variants().is_empty() {
                return Err("Create at least one governed assembly variant first.".to_owned());
            }
        }
        DesignManagementPage::RenumberPreview => {
            if model.schematic.document().components.is_empty() {
                return Err("The active schematic has no instances to annotate.".to_owned());
            }
        }
        DesignManagementPage::AnnotationPolicy => {
            parse_reserved_ranges(&dialog.inputs.reserved_ranges, dialog)?;
        }
        DesignManagementPage::HierarchyAudit => {}
    }
    Ok(())
}
