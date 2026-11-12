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

use crate::diagnostics::LogAnchor;
use crate::schematic::view::violations::location_anchor;
use crate::state::{CellViewRef, Point, RetainedSimulationState};
use crate::workbench::app::actions::hierarchy_net_highlight::NetCarry;
use crate::workbench::app_state::Model;
use crate::workbench::state::{WorkbenchSession, Workspace, WorkspaceDocumentId};
use rspice_app_runs::workbench::slots::runs::RunsState;
use rspice_design::drc::DrcSeverity;

use super::symbol_document::cycle_active_symbol_pin_finding;
use super::{
    SchematicRestoreParts, bump_active_schematic_epoch, commit_pending_inspector_edit,
    hierarchical_edit_in_place_enabled, highlight_failed_run_sites, schematic_for_workspace,
    selected_hierarchy_master, sync_active_schematic, veriloga_source_for_component,
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

    /// Follow `anchor` to what it points at: the object or pin in its
    /// drawing, the objects a failed run named, or the run a row names.
    pub(crate) fn navigate_to_log_anchor(&mut self, anchor: LogAnchor) {
        match anchor {
            LogAnchor::Schematic {
                cell_view,
                x,
                y,
                component,
                wire,
                sheet,
            } => {
                // The point and the object are the drawing's own, so the
                // drawing opens first. A view that will not open — an edit
                // in the inspector that cannot commit — leaves the reader
                // where they were rather than selecting in the wrong view.
                if !cell_view.key().eq_ignore_ascii_case(
                    &self
                        .model
                        .workspace
                        .content
                        .active_schematic_reference()
                        .key(),
                ) {
                    self.open_workspace_view(cell_view.clone());
                    if !cell_view.key().eq_ignore_ascii_case(
                        &self
                            .model
                            .workspace
                            .content
                            .active_schematic_reference()
                            .key(),
                    ) {
                        self.wb.prompt(format!(
                            "{} could not be opened, so the finding was not followed.",
                            cell_view.display_path()
                        ));
                        return;
                    }
                }
                // An object on another sheet is drawn nowhere until its sheet
                // is in view, and selecting it there would select something
                // the reader cannot see. A sheet that will not come forward
                // has said why, and the reader stays where they were.
                if let Some(sheet) = sheet
                    && self.model.active_sheet_id() != Some(sheet)
                {
                    let outcome = self.model.activate_sheet(sheet, self.restore.drag.let_go());
                    if self.wb.report(outcome).is_err() {
                        return;
                    }
                }
                self.wb.activate(Workspace::Design);
                self.model.schematic.session.editor.center_request = Some(Point::new(x, y));
                self.model.schematic.session.editor.net_highlight.clear();
                self.model.schematic.session.editor.selection.clear();
                if let Some(id) = component {
                    self.model
                        .schematic
                        .session
                        .editor
                        .selection
                        .select_component(id);
                }
                if let Some(id) = wire {
                    self.model
                        .schematic
                        .session
                        .editor
                        .selection
                        .select_wire(id);
                }
            }
            LogAnchor::Symbol {
                reference,
                pin_name,
                point,
            } => {
                self.open_workspace_view(reference);
                self.wb.activate(Workspace::Design);
                self.restore.symbol.editor.select_pin(pin_name);
                if let Some(point) = point {
                    let zoom = self.restore.symbol.editor.zoom.max(1.0);
                    self.restore.symbol.editor.pan =
                        (-(point.x as f32) * zoom, -(point.y as f32) * zoom);
                }
                self.restore.symbol.editor.needs_fit = false;
            }
            LogAnchor::Simulation { nets, devices } => {
                highlight_failed_run_sites(self.model, self.wb, &nets, &devices);
            }
            LogAnchor::ResultRun { run_sequence } => {
                // Select first, activate second: landing in Results on
                // whichever run happened to be selected would be a different
                // dataset than the row named.
                if self.model.simulation.select_run_by_sequence(run_sequence) {
                    self.wb.activate(Workspace::Results);
                } else {
                    self.wb
                        .prompt(RetainedSimulationState::retired_run_message(run_sequence));
                }
            }
        }
    }

    /// Jump the view to the next (`step` = 1) or previous (−1) finding,
    /// ordered worst severity first. Selects the offending object where one
    /// exists and centers the canvas on the anchor; node/global findings have
    /// no spot and stay in the console and pill.
    ///
    /// The finding it lands on becomes *the* current finding — the same selection
    /// the Problems grid draws its current row from, held by canonical identity in
    /// the diagnostic store. It used to be an index into the list this function
    /// sorts for itself, which no other surface could read and which a
    /// republished batch silently repointed.
    pub(crate) fn cycle_violation(&mut self, step: isize) {
        // A symbol view's findings are the pin checks', which address a pin on a
        // symbol rather than a spot in a drawing — and they are published as
        // records, so the tour reads them from the store and names each one by
        // the identity the grid names it by.
        //
        // With no findings, or none current, the command is disabled, so a press
        // that reaches here anyway says nothing.
        if self.model.workspace.content.active_view_type() == crate::state::ViewType::Symbol {
            cycle_active_symbol_pin_finding(self, step);
            return;
        }
        let Some(result) = &self.model.design_checks.drc_results else {
            return;
        };
        if self.model.design_checks.drc_checked_version != self.model.schematic.topology_version() {
            return;
        }

        // What the published findings are called, in the order the result holds
        // them. Read rather than derived here: the store is where a finding's
        // identity is minted, and a second derivation of it is a second identity.
        let identities = self.model.canvas_finding_identities();
        // Owned snapshot of the anchored findings so the borrows drop before
        // the selection/viewport mutations below.
        let mut anchored: Vec<AnchoredFinding> = result
            .violations()
            .iter()
            .enumerate()
            .filter_map(|(ordinal, violation)| {
                location_anchor(
                    self.model,
                    self.restore.visibility.review_markers,
                    &violation.location,
                )
                .map(|anchor| AnchoredFinding {
                    severity: violation.severity,
                    identity: identities.get(ordinal).copied(),
                    anchor,
                    message: violation.message.clone(),
                })
            })
            .collect();
        if anchored.is_empty() {
            self.wb
                .prompt("No finding has a place on the canvas; Problems lists them.");
            return;
        }
        // Worst first; stable within a severity.
        anchored.sort_by(|a, b| b.severity.cmp(&a.severity));

        let len = anchored.len() as isize;
        // Where the tour stands is wherever the current finding sits in it. A
        // selection made on the grid — or on a finding this tour cannot visit,
        // such as a netlist error — therefore does not strand the tour: it starts
        // from the worst finding, which is where it starts from a cold session.
        let standing = self
            .model
            .diagnostics
            .current_finding()
            .and_then(|current| {
                anchored
                    .iter()
                    .position(|found| found.identity == Some(current))
            });
        let index = match standing {
            // First jump lands on the worst finding, not the second one.
            None if step >= 0 => 0,
            None => (len - 1) as usize,
            Some(current) => ((current as isize + step).rem_euclid(len)) as usize,
        };

        let found = anchored.swap_remove(index);
        // A finding with no published record leaves nothing current rather than
        // leaving the previous finding standing, which the tour has just left.
        self.model.diagnostics.set_current_finding(found.identity);
        self.navigate_to_log_anchor(found.anchor);
        self.wb.prompt(format!(
            "Finding {}/{} — {} · {}",
            index + 1,
            len,
            found.severity.display_name(),
            found.message,
        ));
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

/// One finding the tour can visit: what it is called, how bad it is, where it
/// is, and what it says.
struct AnchoredFinding {
    severity: DrcSeverity,
    /// What the store calls it, where the checks published a record for it.
    identity: Option<uuid::Uuid>,
    anchor: crate::diagnostics::LogAnchor,
    message: String,
}
