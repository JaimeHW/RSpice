//! Moving the design canvas to another cell view or occurrence: opening a
//! view, descending into an instance, focusing a breadcrumb or ascending,
//! and opening what the selected instance is an instance of.
//!
//! Each move commits the inspector's pending edit, lets go of a canvas drag,
//! files the schematic on screen back into the workspace, and carries the
//! net highlight across the step where the policy says so. It reads and
//! writes the parts of the design slot that hold those, the model, the
//! session, and the runs whose facts the check projection is stated under.

use std::collections::BTreeMap;

use crate::state::CellViewRef;
use crate::workbench::app::actions::hierarchy_net_highlight::NetCarry;
use crate::workbench::app_state::Model;
use crate::workbench::state::{WorkbenchSession, WorkspaceDocumentId};
use rspice_app_runs::workbench::slots::runs::RunsState;

use super::{
    SchematicRestoreParts, bump_active_schematic_epoch, commit_pending_inspector_edit,
    hierarchical_edit_in_place_enabled, schematic_for_workspace, selected_hierarchy_master,
    sync_active_schematic, veriloga_source_for_component,
};

/// What moving the canvas to another cell view or occurrence reads and
/// writes: the design slot's restore parts, its canvas hover and view
/// centre, and the highlight each occurrence held when it was left; the
/// model; the session; and the runs.
pub(crate) struct NavigationParts<'a> {
    pub(crate) restore: SchematicRestoreParts<'a>,
    pub(crate) canvas_hover: &'a mut Option<(f64, f64)>,
    pub(crate) canvas_view_center: &'a mut Option<(f64, f64)>,
    pub(crate) occurrence_net_highlights: &'a mut BTreeMap<String, String>,
    pub(crate) model: &'a mut Model,
    pub(crate) wb: &'a mut WorkbenchSession,
    pub(crate) runs: &'a RunsState,
}

impl NavigationParts<'_> {
    pub(crate) fn open_workspace_view(&mut self, reference: CellViewRef) {
        if self.model.workspace.content.active_view != reference {
            if !commit_pending_inspector_edit(self.model, self.wb) {
                return;
            }
            self.restore.drag.cancel_schematic_drag(self.model);
        }
        sync_active_schematic(self.model, self.wb);
        if self.model.workspace.content.active_view == reference {
            self.wb
                .documents
                .activate(WorkspaceDocumentId::CellView(reference));
            return;
        }
        *self.canvas_hover = None;
        *self.canvas_view_center = None;
        let view_type = self.model.cell_view_type(&reference);
        self.model
            .workspace
            .activate_view(reference.clone(), view_type);
        self.wb
            .documents
            .activate(WorkspaceDocumentId::CellView(reference.clone()));
        self.model.library_manager.select_view(
            &reference.library,
            &reference.cell,
            &reference.view,
        );
        let schematic_reference = self.model.workspace.content.active_schematic_reference();
        self.model.schematic =
            schematic_for_workspace(&mut self.restore, self.model, &schematic_reference);
        self.bump_active_schematic_epoch();
        self.refresh_active_design_check_projection();
        self.wb
            .prompt(format!("Opened {}.", reference.display_path()));
    }

    /// Descend into a hierarchical instance: open its master and record the
    /// instance name on the active document's occurrence. Without an instance
    /// there is no occurrence step to record, so the master opens as its own
    /// design root rather than inheriting a name it was not reached by.
    pub(crate) fn descend_into_instance(
        &mut self,
        instance: Option<String>,
        reference: CellViewRef,
    ) {
        if !commit_pending_inspector_edit(self.model, self.wb) {
            return;
        }
        self.restore.drag.cancel_schematic_drag(self.model);
        sync_active_schematic(self.model, self.wb);
        let carried = self.leave_occurrence_net_highlight(match &instance {
            Some(name) => NetCarry::Descend(name.clone()),
            None => NetCarry::Nothing,
        });
        let view_type = self.model.cell_view_type(&reference);
        match instance {
            Some(name) => self
                .model
                .workspace
                .descend_into(name, reference.clone(), view_type),
            None => self
                .model
                .workspace
                .open_as_root(reference.clone(), view_type),
        }
        self.model
            .workspace
            .content
            .set_active_read_only_reference(false);
        self.model.library_manager.select_view(
            &reference.library,
            &reference.cell,
            &reference.view,
        );
        let schematic_reference = self.model.workspace.content.active_schematic_reference();
        self.model.schematic =
            schematic_for_workspace(&mut self.restore, self.model, &schematic_reference);
        self.bump_active_schematic_epoch();
        self.refresh_active_design_check_projection();
        self.enter_occurrence_net_highlight(carried);
        self.wb
            .prompt(format!("Entered {}.", reference.display_path()));
    }

    /// Ascend one hierarchy level (the U gesture / pathbar action).
    pub(crate) fn ascend_workspace_level(&mut self) {
        let depth = self.model.workspace.content.occurrence_depth();
        if depth >= 2 {
            self.focus_workspace_breadcrumb(depth - 2);
        }
    }

    pub(crate) fn focus_workspace_breadcrumb(&mut self, index: usize) {
        if !commit_pending_inspector_edit(self.model, self.wb) {
            return;
        }
        self.restore.drag.cancel_schematic_drag(self.model);
        sync_active_schematic(self.model, self.wb);
        // One level up carries the highlight out through the port; a longer
        // jump restores what the level held.
        let carried = self.leave_occurrence_net_highlight(
            if index + 2 == self.model.workspace.content.occurrence_depth() {
                NetCarry::Ascend
            } else {
                NetCarry::Nothing
            },
        );
        if let Some(reference) = self.model.workspace.focus_breadcrumb(index) {
            self.model.library_manager.select_view(
                &reference.library,
                &reference.cell,
                &reference.view,
            );
            self.model.schematic =
                schematic_for_workspace(&mut self.restore, self.model, &reference);
            self.bump_active_schematic_epoch();
            self.refresh_active_design_check_projection();
            self.enter_occurrence_net_highlight(carried);
        }
    }

    /// Open the exact Verilog-A source owned by a bound schematic instance.
    /// Resolution happens before any workspace mutation, so stale or partial
    /// bindings fail closed and leave the active document untouched.
    pub(crate) fn open_veriloga_source_for_component(&mut self, component_id: u64) -> bool {
        let Some(reference) = veriloga_source_for_component(self.model, component_id) else {
            return false;
        };
        // The module is a view of the cell, so it opens as a Design document
        // beside the schematic it was reached from.
        self.open_workspace_view(reference);
        self.wb.activate(crate::workbench::state::Workspace::Design);
        true
    }

    /// Open what the one selected instance is an instance of: descend into
    /// its schematic master, or, for a Verilog-A instance, open its module's
    /// cell view. A module has no occurrences below it, so that is not a
    /// descent and records no occurrence step.
    pub(crate) fn open_selected_instance_master(&mut self) {
        if let Some(component_id) = self
            .model
            .schematic
            .session
            .editor
            .selection
            .single_component()
            && self.open_veriloga_source_for_component(component_id)
        {
            return;
        }
        let Some((instance_name, reference)) = selected_hierarchy_master(self.model) else {
            self.wb
                .prompt("Select one instance with a resolved schematic master first.");
            return;
        };
        if hierarchical_edit_in_place_enabled(self.model) {
            self.descend_into_instance(Some(instance_name), reference);
        } else {
            self.open_workspace_view(reference);
        }
    }

    /// [`bump_active_schematic_epoch`] over these parts.
    fn bump_active_schematic_epoch(&mut self) {
        bump_active_schematic_epoch(self.restore.symbol, self.model, self.wb);
    }

    /// Restate the active cell view's check projection under the runs' and
    /// the review markers' facts.
    fn refresh_active_design_check_projection(&mut self) {
        let facts = self.runs.slot_facts(self.restore.visibility.review_markers);
        self.model.refresh_active_design_check_projection(facts);
    }
}
