//! The sync of the open schematic into the project: the drawing the editor
//! holds is written back to its cell view's buffer, its sheet catalog and its
//! generated symbol view. Every frame runs it, and so does every operation
//! that must read the project as the open schematic edits it, whichever
//! feature asks.

use super::Model;
use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{CellViewRef, SchematicSnapshot, SymbolDocument, View, ViewType};
use crate::workbench::app_state::design_history::DesignManagementHistoryEntry;
use rspice_design::schematic::document::SchematicDocument;
use rspice_design::symbol::edit::{
    places_symbol, resolve_symbol_instance_wiring, symbol_pin_position_remaps,
};
use rspice_design::symbol::generated::{
    GENERATED_SYMBOL_VIEW, is_generated_symbol, store_generated_symbol,
};
use rspice_ui_kit::accessibility::counted;
use std::collections::BTreeMap;

/// Failures of the per-frame drawing sync that the Console already holds.
///
/// The drawing is synced every frame, so a failure that stands is recorded
/// when it first appears or changes, not sixty times a second.
#[derive(Debug, Clone, Default)]
pub struct SyncFailures {
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
    pub fn sync_active_schematic_to_workspace(&mut self) -> Option<String> {
        let mut prompt = None;
        if self
            .workspace
            .content
            .active_view_type()
            .is_some_and(is_schematic_like)
        {
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
        if self
            .workspace
            .content
            .active_view_type()
            .is_some_and(is_schematic_like)
        {
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
    /// schematic interface — created when ports exist and refreshed when they
    /// change. A hand-authored symbol view (no `generated` marker) is never
    /// touched: the user owns it, and neither is anything in a read-only
    /// library.
    ///
    /// No view goes by itself. When the last port goes the symbol stays as it
    /// was drawn: its pins are then the ones the pin checks report as not in
    /// the schematic, and a port that comes back is drawn into it again.
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
            return;
        }

        let key = reference.key();
        let symbol_error = match existing_symbol {
            Some(mut view) if is_generated_symbol(&view) => {
                let before = view.metadata.clone();
                let drawn = SymbolDocument::load_from_view(&view).ok();
                match store_generated_symbol(&mut view, &ports, |_| None) {
                    Err(error) => Some(format!("Generated symbol could not be refreshed: {error}")),
                    Ok(()) => {
                        let redrawn = SymbolDocument::load_from_view(&view).ok();
                        if view.metadata != before
                            && let Some(mut library) =
                                self.library_manager.edit_library(&reference.library)
                        {
                            library.replace_view_metadata(
                                &reference.cell,
                                GENERATED_SYMBOL_VIEW,
                                view.metadata,
                            );
                            drop(library);
                            let named = named_ports(self.schematic.document());
                            let renames = self
                                .generated_symbol_ports
                                .get(&key)
                                .map(|drawn_for| renamed_ports(drawn_for, &named))
                                .unwrap_or_default();
                            self.generated_symbol_ports.insert(key, named);
                            if let (Some(drawn), Some(redrawn)) = (drawn, redrawn) {
                                let symbol = CellViewRef::new(
                                    &reference.library,
                                    &reference.cell,
                                    GENERATED_SYMBOL_VIEW,
                                );
                                self.carry_placed_instances(&symbol, &drawn, &redrawn, &renames);
                            }
                        } else if !self.generated_symbol_ports.contains_key(&key) {
                            let named = named_ports(self.schematic.document());
                            self.generated_symbol_ports.insert(key, named);
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
                        let named = named_ports(self.schematic.document());
                        self.generated_symbol_ports.insert(key, named);
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

    /// Carry every placed instance of `symbol` from `drawn` to `redrawn`:
    /// each wire end a terminal held follows it to where the redrawn symbol
    /// puts it, and a terminal whose port was renamed takes the new name, so
    /// its wire stays on the pin. A pin that went holds its wire nowhere: the
    /// wire is left where it was, for the checks to report. Each sheet that
    /// changes records the change as one step of its own.
    fn carry_placed_instances(
        &mut self,
        symbol: &CellViewRef,
        drawn: &SymbolDocument,
        redrawn: &SymbolDocument,
        renames: &BTreeMap<String, String>,
    ) {
        const STEP: &str = "move wires with symbol pins";
        let remaps = symbol_pin_position_remaps(drawn, redrawn, renames);
        let renamed: BTreeMap<String, String> = renames
            .iter()
            .map(|(from, to)| (from.to_ascii_lowercase(), to.clone()))
            .collect();
        if remaps.is_empty() && renamed.is_empty() {
            return;
        }
        // The open schematic is the cell's own, which places no instance of
        // the cell it draws.
        let active = self.workspace.content.active_schematic_reference().key();
        let content = &self.workspace.content;
        let held = content
            .schematic_buffers
            .iter()
            .filter(|(key, design)| **key != active && places_symbol(design.document(), symbol))
            .map(|(key, design)| {
                let catalog = content.design_management.sheet_catalog(key);
                let sheet_of = |id: u64| {
                    catalog.and_then(|catalog| {
                        catalog
                            .sheet_for_object(id)
                            .or_else(|| catalog.active_sheet_id())
                    })
                };
                let wiring = resolve_symbol_instance_wiring(
                    design.document(),
                    symbol,
                    drawn,
                    |part, wire| sheet_of(part) == sheet_of(wire),
                );
                (
                    key.clone(),
                    wiring,
                    SchematicSnapshot::capture(design.document()),
                )
            })
            .collect::<Vec<_>>();
        for (key, wiring, before) in held {
            let mut before = Some(before);
            self.workspace
                .for_each_schematic_editor_mut(|buffer, schematic| {
                    if buffer != key {
                        return;
                    }
                    let moved = schematic.apply_symbol_instance_wiring(&wiring, &remaps);
                    let relabelled = schematic.rename_instance_terminals(symbol, &renamed) > 0;
                    if (moved || relabelled)
                        && let Some(before) = before.take()
                    {
                        schematic.commit_undo_from(before, STEP);
                    }
                });
        }
    }
}

/// The named ports of `document`, each by the object that declares it.
fn named_ports(document: &SchematicDocument) -> Vec<(u64, String)> {
    document
        .components
        .iter()
        .filter_map(|component| component.port_spec().map(|port| (component.id, port.name)))
        .collect()
}

/// The ports renamed between `drawn_for` and `named`, old name to new: the
/// same port object, under another name. A port added or deleted renames
/// nothing, so a wire on a port that went is never handed to one that came.
fn renamed_ports(drawn_for: &[(u64, String)], named: &[(u64, String)]) -> BTreeMap<String, String> {
    drawn_for
        .iter()
        .filter_map(|(id, old)| {
            named
                .iter()
                .find(|(other, new)| other == id && !new.eq_ignore_ascii_case(old))
                .map(|(_, new)| (old.clone(), new.clone()))
        })
        .collect()
}

#[cfg(test)]
mod tests;
