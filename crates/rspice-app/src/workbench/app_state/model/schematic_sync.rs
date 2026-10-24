//! The sync of the open schematic into the project: the drawing the editor
//! holds is written back to its cell view's buffer, its sheet catalog and its
//! generated symbol view. Every frame runs it, and so does every operation
//! that must read the project as the open schematic edits it, whichever
//! feature asks.

use super::Model;
use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{CellViewRef, View, ViewType};
use crate::ui::accessibility::counted;
use crate::workbench::app_state::DesignManagementHistoryEntry;
use rspice_design::symbol::generated::{
    GENERATED_SYMBOL_VIEW, is_generated_symbol, store_generated_symbol,
};
use std::collections::BTreeMap;

/// Failures of the per-frame drawing sync that the Console already holds.
///
/// The drawing is synced every frame, so a failure that stands is recorded
/// when it first appears or changes, not sixty times a second.
#[derive(Debug, Clone, Default)]
pub(crate) struct SyncFailures {
    pub(crate) sheet_membership: Option<String>,
    pub(crate) generated_symbol: Option<String>,
}

impl SyncFailures {
    /// Note `failure` in `slot`, answering whether it is new and so is to be
    /// recorded. `None` clears the slot, so the same failure returning later
    /// is recorded again.
    pub(crate) fn observe(slot: &mut Option<String>, failure: Option<&str>) -> bool {
        if slot.as_deref() == failure {
            return false;
        }
        *slot = failure.map(str::to_owned);
        failure.is_some()
    }
}

/// Whether a view of this type is edited as a schematic drawing.
fn is_schematic_like(view_type: ViewType) -> bool {
    matches!(view_type, ViewType::Schematic | ViewType::Testbench)
}

impl Model {
    /// Write the open schematic back to the project.
    ///
    /// Answers the prompt the sync has for the reader, if it has one: a
    /// cross-sheet connection dropped with the objects it anchored to is
    /// stated on the status bar, which the caller owns.
    #[must_use]
    pub(crate) fn sync_active_schematic_to_workspace(&mut self) -> Option<String> {
        let mut prompt = None;
        if is_schematic_like(self.workspace.content.active_view_type()) {
            let active = self.workspace.content.active_schematic_reference();
            if self.schematic.has_pending_operation() {
                // Window projection may retain a live gesture in a runtime
                // buffer; it must not publish symbols or sheet transactions.
                self.workspace
                    .insert_schematic_editor(active.key(), self.schematic.clone());
                return None;
            }
            prompt = self.reconcile_active_sheet_membership(&active);
            // Nothing may be written back to the workspace still claiming a
            // master's netlist identity after that master is gone — including
            // a placement an undo has just restored, which is otherwise the
            // one way a deleted cell comes back looking resolved.
            self.schematic
                .revalidate_instance_bindings(&self.library_manager);
            self.workspace.save_active_schematic(&self.schematic);
        }
        self.retry_annotation_restoration();
        if is_schematic_like(self.workspace.content.active_view_type()) {
            self.sync_generated_symbol_view();
        }
        prompt
    }

    /// A recovered reference transition can be retried after its owners are
    /// repaired. Never publish it over an unfinished edit in any document.
    fn retry_annotation_restoration(&mut self) {
        let Some(previous_error) = self
            .workspace
            .annotation_restoration_error()
            .map(str::to_owned)
        else {
            return;
        };
        if self
            .workspace
            .content
            .schematic_buffers
            .values()
            .any(|schematic| schematic.pending_operation_id().is_some())
        {
            return;
        }
        match self
            .workspace
            .restore_pending_annotation(&self.library_manager)
        {
            Ok(count) => {
                if let Some(schematic) = self.workspace.active_context_schematic() {
                    self.schematic = schematic.clone_editor();
                }
                self.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    format!(
                        "Reference annotation recovery completed: {}.",
                        counted(count, "component updated", "components updated")
                    ),
                );
            }
            Err(error) if error != previous_error => {
                self.record(
                    LogSeverity::Error,
                    LogSource::Studio,
                    format!(
                        "Reference annotation recovery still needs correction: {error}. \
                         Recovered documents were preserved."
                    ),
                );
            }
            Err(_) => {}
        }
    }

    /// Bring the active cell view's sheet catalog back into step with its
    /// drawing, and state in the project history anything the drawing cannot
    /// put back on its own.
    ///
    /// Membership travels through the schematic step that is being synced: an
    /// object an undo has just restored returns to the sheet it was drawn on.
    /// A cross-sheet port contract cannot travel that way — nothing in the
    /// drawing holds it — so losing one is a project transaction of its own,
    /// which restores both the contract and the anchors it names. Answers the
    /// prompt that says so.
    fn reconcile_active_sheet_membership(&mut self, active: &CellViewRef) -> Option<String> {
        #[cfg(test)]
        rspice_design_model::work_tally::record(|work| work.sheet_reconciliations += 1);
        let recorded = self.schematic.take_restored_sheet_assignments();
        // Only a cell view that already holds contracts can lose one, and only
        // that case needs the retained before-state. Every other document
        // synchronizes without copying its design authority.
        let before = self
            .workspace
            .content
            .design_management
            .sheet_catalog(&active.key())
            .is_some_and(|catalog| !catalog.cross_sheet_ports().is_empty())
            .then(|| {
                (
                    (*self.workspace.content.design_management).clone(),
                    self.workspace
                        .clone_schematic_editor(&active.key())
                        .unwrap_or_else(|| self.schematic.clone()),
                )
            });
        let revision_before = self.workspace.content.project.revision();
        let assigned = self
            .workspace
            .content
            .assign_unowned_objects_to_active_sheet(active, &self.schematic, &recorded);
        // Recorded once while it stands, the way annotation recovery is: this
        // runs on every frame's sync.
        let failure = assigned
            .as_ref()
            .err()
            .map(|error| format!("Sheet membership could not be updated: {error}"));
        if SyncFailures::observe(
            &mut self.recorded_sync_failures.sheet_membership,
            failure.as_deref(),
        ) && let Some(failure) = failure
        {
            self.record(LogSeverity::Warning, LogSource::Studio, failure);
        }
        let Ok(receipt) = assigned else {
            return None;
        };
        let committed_revision = self.workspace.content.project.revision();
        let assignments = self
            .workspace
            .content
            .design_management
            .sheet_catalog(&active.key())
            .map(|catalog| catalog.object_assignments().clone())
            .unwrap_or_default();
        self.schematic.set_live_sheet_assignments(assignments);
        let published = receipt.is_some();
        let removed_ports = receipt.map_or(0, |receipt| receipt.removed_cross_sheet_ports);
        // A dropped contract is a step of its own, recorded below; anything
        // else membership published is bookkeeping the earlier steps survive.
        if published && removed_ports == 0 {
            self.carry_project_history_over_sheet_membership(revision_before, committed_revision);
        }
        let (before, before_schematic) = before.filter(|_| removed_ports > 0)?;
        let after = (*self.workspace.content.design_management).clone();
        self.record_design_management_transaction(DesignManagementHistoryEntry {
            description: "drop cross-sheet connections whose anchors were deleted".to_owned(),
            owner: active.clone(),
            before,
            after,
            before_schematics: BTreeMap::from([(active.key(), before_schematic)]),
            after_schematics: BTreeMap::from([(active.key(), self.schematic.clone())]),
            references: Default::default(),
            committed_revision,
        });
        let subject = if removed_ports == 1 {
            "connection"
        } else {
            "connections"
        };
        Some(format!(
            "Undo restores {removed_ports} cross-sheet {subject} dropped with the objects they \
             anchor to."
        ))
    }

    /// Keep the active cell's generated "symbol" view in step with its
    /// schematic interface — created when ports exist, refreshed when they
    /// change, removed when the last port goes. A hand-authored symbol
    /// view (no `generated` marker) is never touched: the user owns it, and
    /// neither is anything in a read-only library.
    fn sync_generated_symbol_view(&mut self) {
        let reference = self.workspace.content.active_view.clone();
        if !reference.view.eq_ignore_ascii_case("schematic") {
            return;
        }
        let Some(library) = self.library_manager.get_library(&reference.library) else {
            return;
        };
        let Some(cell) = library.get_cell(&reference.cell) else {
            return;
        };
        if library.read_only {
            return;
        }
        let existing_symbol = cell.get_view(GENERATED_SYMBOL_VIEW).cloned();
        let ports = self.schematic.interface_ports();

        if ports.is_empty() {
            if !existing_symbol.as_ref().is_some_and(is_generated_symbol) {
                return;
            }
            if let Some(mut library) = self.library_manager.edit_library(&reference.library) {
                library.remove_view(&reference.cell, GENERATED_SYMBOL_VIEW);
            }
            return;
        }

        let symbol_error = match existing_symbol {
            Some(mut view) if is_generated_symbol(&view) => {
                let before = view.metadata.clone();
                match store_generated_symbol(&mut view, &ports, |_| None) {
                    Err(error) => Some(format!("Generated symbol could not be refreshed: {error}")),
                    Ok(()) => {
                        if view.metadata != before
                            && let Some(mut library) =
                                self.library_manager.edit_library(&reference.library)
                        {
                            library.replace_view_metadata(
                                &reference.cell,
                                GENERATED_SYMBOL_VIEW,
                                view.metadata,
                            );
                        }
                        None
                    }
                }
            }
            Some(_) => None, // hand-authored symbol: leave it alone
            None => {
                let mut view = View::new(GENERATED_SYMBOL_VIEW, ViewType::Symbol);
                match store_generated_symbol(&mut view, &ports, |_| None) {
                    Err(error) => Some(format!("Generated symbol could not be created: {error}")),
                    Ok(()) => {
                        if let Some(mut library) =
                            self.library_manager.edit_library(&reference.library)
                        {
                            library.add_view(&reference.cell, view);
                        }
                        None
                    }
                }
            }
        };
        if SyncFailures::observe(
            &mut self.recorded_sync_failures.generated_symbol,
            symbol_error.as_deref(),
        ) && let Some(error) = symbol_error
        {
            self.record(LogSeverity::Warning, LogSource::Studio, error);
        }
    }
}

#[cfg(test)]
mod tests;
