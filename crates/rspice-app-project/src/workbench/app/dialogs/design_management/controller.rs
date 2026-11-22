//! Design management dialog flow.

use super::manager::design_management_manager_body;
use super::operations::commit_design_management_subflow;
use super::subflows::design_management_subflow_body;
use super::widgets::parse_reserved_ranges;
use super::*;
use crate::diagnostics::{LogSeverity, LogSource};

/// Draw Design management while it is open, and carry out what its body
/// and its primary and ghost actions asked for.
pub fn render_design_management_dialog(parts: &mut DesignManagementParts<'_>, ctx: &Context) {
    if !parts.dialog.open {
        return;
    }
    let page = parts.dialog.page;
    if page == DesignManagementPage::RenumberPreview {
        super::operations::refresh_renumber_table(
            parts.dialog,
            &parts.model.workspace,
            &parts.model.schematic,
        );
    }
    super::variant_readout::refresh_variant_readout(parts.dialog, parts.model);
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
    let primary_enabled = page.read_out()
        || write_allowed
            && validation.is_ok()
            && if page == DesignManagementPage::Manager {
                manager_dirty
            } else {
                true
            };
    let description = page.description();
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
    // A manager with nothing pending is at rest, not failing: Apply waits
    // for a reviewed change, and the footer says so without a failure mark.
    // A new sheet or variant waits for its name the same way.
    let at_rest = validation
        .as_ref()
        .err()
        .is_some_and(|message| AT_REST.contains(&message.as_str()));
    let mut body_scroll_offset = parts.dialog.body_scroll_offset;
    let mut dialog = Dialog::new(page.title(), page.primary())
        .description(description)
        .size(size)
        .initial_height(initial_height)
        .flush_body()
        .overlay_scrollbar()
        .primary_enabled(primary_enabled)
        .primary_on_enter(false)
        .initial_focus(DialogInitialFocus::BodyControl)
        .body_scroll_offset(&mut body_scroll_offset);
    // A read-out's Close is its only action: there is nothing to cancel.
    if !page.read_out() {
        dialog = dialog.ghost(ghost);
    }
    let drawing_sheet_personal = parts.model.preferences.drawing_sheet_personal_preferences();
    if discard {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            "Discard uncommitted design-management changes?",
            "The live project remains unchanged. Subordinate operations already added to this manager draft are discarded together.",
        );
    } else if let Some(error) = error.as_deref().or_else(|| {
        validation
            .as_ref()
            .err()
            .filter(|_| !at_rest)
            .map(String::as_str)
    }) {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            "Design transaction cannot continue",
            error,
        );
    } else if let Some(waiting) = validation.as_ref().err().filter(|_| at_rest) {
        dialog = dialog.hint(waiting);
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
                &mut body_action,
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
                !page.read_out() && parts.dialog.subflow_dirty()
            };
            if dirty && !parts.dialog.discard_confirmation {
                parts.dialog.discard_confirmation = true;
                response.retain_cancel_focus(DialogInitialFocus::Ghost);
            } else if page == DesignManagementPage::Manager {
                parts.dialog.close_and_discard();
                super::close_design_management_dialog_route(parts.wb);
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
            super::close_design_management_dialog_route(parts.wb);
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
            super::close_design_management_dialog_route(parts.wb);
            crate::workbench::app::dialogs::project::configuration_dialogs::show_configuration_page_in(
                parts.platform,
                parts.wb,
                parts.project_page,
            );
            *parts.configuration_tab =
                crate::workbench::project::configuration_state::ConfigurationTab::Instances;
        }
        DesignManagementBodyAction::ShowPart(object) => {
            if parts.dialog.dirty() {
                parts.dialog.error = Some(
                    "Apply or discard this Design Management draft before going to a part."
                        .to_owned(),
                );
                return;
            }
            let Some(place) = super::variant_readout::part_place(parts.model, &object) else {
                parts.dialog.error = Some("That part is no longer in the design.".to_owned());
                return;
            };
            // The draft is the project's own, so the Comparison it names is
            // a variant the project has, or the base design.
            let comparison = parts.dialog.inputs.compare_target;
            let chosen = parts
                .model
                .workspace
                .content
                .design_management
                .variants()
                .active_variant_id();
            parts.dialog.close_and_discard();
            super::close_design_management_dialog_route(parts.wb);
            match parts.model.choose_assembly_variant(comparison) {
                Ok(message) => {
                    if chosen != comparison {
                        rspice_app_runs::workbench::preflight::invalidate(
                            &mut parts.runs.preflight,
                            parts.run_controller,
                        );
                    }
                    parts.wb.prompt(message);
                }
                Err(error) => parts.wb.prompt(error),
            }
            parts.wb.outbox.push(place);
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
    let committed_revision = parts.model.publish_design_management_candidate(
        "apply reviewed design-management changes",
        before,
        draft,
    )?;
    let candidate = (*parts.model.workspace.content.design_management).clone();
    rspice_app_runs::workbench::preflight::invalidate(
        &mut parts.runs.preflight,
        parts.run_controller,
    );
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

/// What Apply waits for while the manager holds no reviewed change.
const NOTHING_PENDING: &str = "No reviewed design changes are pending.";
/// What Create waits for on a new sheet: a name in a field that opens empty.
const SHEET_NAME_NEEDED: &str = "Type a name for the new sheet.";
/// What Create waits for on a new variant.
const VARIANT_NAME_NEEDED: &str = "Type a name for the new variant.";
/// What Renumber waits for while every part in scope keeps its name.
const NOTHING_TO_RENUMBER: &str = "No part in this scope would change its name.";
/// What a page waits for rather than fails on: the footer states it as a
/// hint, and the primary stays disabled until it is given.
const AT_REST: [&str; 4] = [
    NOTHING_PENDING,
    SHEET_NAME_NEEDED,
    VARIANT_NAME_NEEDED,
    NOTHING_TO_RENUMBER,
];

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
                return Err(NOTHING_PENDING.to_owned());
            }
        }
        DesignManagementPage::NewSheet => {
            if dialog.inputs.sheet_name.trim().is_empty() {
                return Err(SHEET_NAME_NEEDED.to_owned());
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
                return Err(VARIANT_NAME_NEEDED.to_owned());
            }
        }
        // A read-out says in its list why it shows nothing.
        DesignManagementPage::CompareVariants | DesignManagementPage::VariantMatrix => {}
        DesignManagementPage::RenumberPreview => {
            let table = dialog
                .renumber_table
                .as_ref()
                .ok_or_else(|| "The list of new names is unavailable.".to_owned())?;
            if let Some(problem) = &table.problem {
                return Err(problem.clone());
            }
            if table.changes() == 0 {
                return Err(NOTHING_TO_RENUMBER.to_owned());
            }
        }
        DesignManagementPage::AnnotationPolicy => {
            parse_reserved_ranges(&dialog.inputs.reserved_ranges)?;
        }
        DesignManagementPage::HierarchyAudit => {}
    }
    Ok(())
}
