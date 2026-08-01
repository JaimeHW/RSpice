//! Design management dialog flow.

use super::manager::design_management_manager_body;
use super::subflows::design_management_subflow_body;
use super::widgets::{parse_reserved_ranges, reorder_sheet_ids};
use super::*;
use crate::diagnostics::ConsoleMessage;

impl RSpiceApp {
    pub(in crate::workbench) fn render_design_management_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.design_management.open {
            return;
        }
        let page = self.state.dialogs.design_management.page;
        let manager_dirty = self.state.dialogs.design_management.dirty();
        let subflow_dirty = self.state.dialogs.design_management.subflow_dirty();
        let dirty = if page == DesignManagementPage::Manager {
            manager_dirty
        } else {
            subflow_dirty
        };
        let discard = self.state.dialogs.design_management.discard_confirmation;
        let validation = validate_design_management_page(&self.state, page);
        let write_allowed = !self.state.workbench.safe_mode.project_read_only();
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
            "Review the exact source, proposed transaction, resulting revision, and invariant before applying this operation to the manager draft."
        };
        let initial_height = if page == DesignManagementPage::Manager {
            MANAGER_INITIAL_HEIGHT
        } else {
            SUBFLOW_INITIAL_HEIGHT
        };
        let size = if page == DesignManagementPage::Manager {
            DialogSize::SimulationWorkflow
        } else {
            DialogSize::WideWorkflow
        };
        let ghost = if discard {
            "Discard changes"
        } else if page == DesignManagementPage::Manager {
            "Close"
        } else {
            "Cancel"
        };
        let error = self.state.dialogs.design_management.error.clone();
        let mut body_scroll_offset = self.state.dialogs.design_management.body_scroll_offset;
        let mut dialog = Dialog::new(page.eyebrow(), page.title(), page.primary())
            .description(description)
            .size(size)
            .initial_height(initial_height)
            .flush_body()
            .ghost(ghost)
            .primary_enabled(primary_enabled)
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::BodyControl)
            .body_scroll_offset(&mut body_scroll_offset);
        if dirty && !discard {
            dialog = dialog.retain_on_cancel_focus(DialogInitialFocus::Ghost);
        }
        let drawing_sheet_personal = self
            .state
            .ui
            .preferences
            .drawing_sheet_personal_preferences();
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
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            let response = if page == DesignManagementPage::Manager {
                body_action = design_management_manager_body(
                    ui,
                    &mut self.state.dialogs.design_management,
                    &self.state.workspace,
                    &self.state.library_manager,
                    &self.state.schematic,
                    write_allowed,
                );
                None
            } else {
                Some(design_management_subflow_body(
                    ui,
                    &mut self.state.dialogs.design_management,
                    &self.state.workspace,
                    &self.state.library_manager,
                    &self.state.schematic,
                    &drawing_sheet_personal,
                    write_allowed,
                ))
            };
            response.map(|response| response.id)
        });
        self.state.dialogs.design_management.body_scroll_offset = body_scroll_offset;
        self.handle_design_management_body_action(body_action);
        match choice {
            DialogChoice::Primary => {
                let result = if page == DesignManagementPage::Manager {
                    self.publish_design_management_draft()
                } else {
                    self.commit_design_management_subflow(page)
                };
                if let Err(error) = result {
                    self.state.dialogs.design_management.error = Some(error);
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                if dirty && !discard {
                    self.state.dialogs.design_management.discard_confirmation = true;
                } else if page == DesignManagementPage::Manager {
                    self.state.dialogs.design_management.close_and_discard();
                    crate::workbench::app::close_design_management_dialog_route(&mut self.state);
                } else {
                    let active_sheet = self
                        .state
                        .dialogs
                        .design_management
                        .draft
                        .as_ref()
                        .and_then(|catalog| {
                            catalog.sheet_catalog(&self.state.dialogs.design_management.owner_key)
                        })
                        .and_then(|catalog| catalog.active_sheet_id());
                    let configurations = self.state.workspace.configuration_sets.clone();
                    self.state.dialogs.design_management.reset_inputs_for_page(
                        DesignManagementPage::Manager,
                        Some(&configurations),
                        active_sheet,
                    );
                }
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }

    fn handle_design_management_body_action(&mut self, action: DesignManagementBodyAction) {
        match action {
            DesignManagementBodyAction::None => {}
            DesignManagementBodyAction::Open(page) => {
                let active_sheet = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_ref()
                    .and_then(|catalog| {
                        catalog.sheet_catalog(&self.state.dialogs.design_management.owner_key)
                    })
                    .and_then(|catalog| catalog.active_sheet_id());
                let configurations = self.state.workspace.configuration_sets.clone();
                self.state.dialogs.design_management.reset_inputs_for_page(
                    page,
                    Some(&configurations),
                    active_sheet,
                );
            }
            DesignManagementBodyAction::OpenConfigurationSets => {
                if self.state.dialogs.design_management.dirty() {
                    self.state.dialogs.design_management.error = Some(
                        "Apply or discard this Design Management draft before opening configuration bindings."
                            .to_owned(),
                    );
                    return;
                }
                self.state.dialogs.design_management.close_and_discard();
                crate::workbench::app::close_design_management_dialog_route(&mut self.state);
                crate::workbench::app::open_configuration_sets_dialog(&mut self.state);
            }
            DesignManagementBodyAction::OpenConfigurationBinding => {
                if self.state.dialogs.design_management.dirty() {
                    self.state.dialogs.design_management.error = Some(
                        "Apply or discard this Design Management draft before opening configuration bindings."
                            .to_owned(),
                    );
                    return;
                }
                self.state.dialogs.design_management.close_and_discard();
                crate::workbench::app::close_design_management_dialog_route(&mut self.state);
                crate::workbench::app::open_configuration_binding_dialog(&mut self.state);
            }
        }
    }

    fn publish_design_management_draft(&mut self) -> Result<(), String> {
        let before = self.state.workspace.design_management.clone();
        let draft = self
            .state
            .dialogs
            .design_management
            .draft
            .clone()
            .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
        draft.validate().map_err(|error| error.to_string())?;
        let schematic_tx = self
            .state
            .prepare_design_management_schematic_transaction(&draft)?;
        let owner = self.state.workspace.active_schematic_reference();
        let committed_revision = self
            .state
            .workspace
            .replace_design_management(draft)
            .map_err(|error| error.to_string())?;
        self.state
            .apply_design_management_schematic_transaction(&schematic_tx);
        let candidate = self.state.workspace.design_management.clone();
        self.state
            .record_design_management_transaction(DesignManagementHistoryEntry {
                description: "apply reviewed design-management changes".to_owned(),
                owner,
                before,
                after: candidate.clone(),
                before_schematics: schematic_tx.before,
                after_schematics: schematic_tx.after,
                committed_revision,
            });
        self.state.design_execution_epoch = self.state.design_execution_epoch.wrapping_add(1);
        self.invalidate_simulation_preflight();
        self.state.ui.netlist.current_generation_input_digest = None;
        self.state.push_user_message(ConsoleMessage::info(format!(
            "Design Management revision {} was published as project revision {:?}.",
            candidate.revision(),
            committed_revision
        )));
        let owner_key = self.state.dialogs.design_management.owner_key.clone();
        let selection_object_ids = self
            .state
            .dialogs
            .design_management
            .selection_object_ids
            .clone();
        let selection_summary = self
            .state
            .dialogs
            .design_management
            .selection_summary
            .clone();
        let all_object_ids = self.state.dialogs.design_management.all_object_ids.clone();
        let default_sheet_name = self
            .state
            .dialogs
            .design_management
            .default_sheet_name
            .clone();
        self.state.dialogs.design_management.open(
            &candidate,
            owner_key,
            selection_object_ids,
            all_object_ids,
            selection_summary,
            default_sheet_name,
        )?;
        self.state.dialogs.design_management.receipt = Some(format!(
            "Published design-management revision {} \u{00b7} one project transaction",
            candidate.revision()
        ));
        Ok(())
    }
}

pub(super) fn validate_design_management_page(
    state: &AppState,
    page: DesignManagementPage,
) -> Result<(), String> {
    let dialog = &state.dialogs.design_management;
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
        DesignManagementPage::ReorderSheets => {
            let count = draft
                .sheet_catalog(&dialog.owner_key)
                .map_or(0, |catalog| catalog.sheets().len());
            if count < 2 {
                return Err(
                    "At least two complete sheet identities are required to reorder.".to_owned(),
                );
            }
            reorder_sheet_ids(dialog)?;
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
            let selected_components = state
                .schematic
                .components
                .iter()
                .filter(|component| dialog.selection_object_ids.contains(&component.id))
                .count();
            if selected_components != dialog.selection_object_ids.len()
                || selected_components == 0
                || state.schematic.selection.count() != state.schematic.selection.components.len()
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
            if state.schematic.components.is_empty() {
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
