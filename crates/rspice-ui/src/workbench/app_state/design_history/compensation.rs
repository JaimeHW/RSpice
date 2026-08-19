//! What a project transaction promises to put back, and where it sits in the
//! one undo order.
//!
//! Two facts about a project transaction cannot live inside the transaction
//! body, because they mean the same thing for every kind of transaction:
//! where the step sits in the timeline the user sees, and which documents
//! undoing it will touch. Both live here, so arbitration and navigation are
//! written once instead of once per variant.
//!
//! The sheet dimension is the part undo cannot re-derive. An object restored
//! without the sheet it was recorded on lands on whichever sheet happens to
//! be active, which is a different design from the one that was undone. A
//! transaction that moves objects between documents therefore records where
//! they lived; one that restores a whole sheet catalog records nothing here,
//! because the catalog it puts back already carries the membership.

use std::collections::BTreeMap;

use crate::state::{CellViewRef, SchematicState, SheetId, UndoSequence, next_undo_sequence};
use crate::workbench::app_state::AppState;

use super::schematic_for_reference;

/// One document a project transaction touched, with the sheet membership its
/// objects held before the transaction ran.
///
/// An empty membership map means "this transaction restores no sheet
/// assignments", never "assign everything to the default sheet".
#[derive(Debug, Clone)]
pub(super) struct DocumentCompensation {
    reference: CellViewRef,
    before_sheet_assignments: BTreeMap<u64, SheetId>,
}

impl DocumentCompensation {
    /// A document whose objects move as part of the transaction, so undo has
    /// to put them back on the sheets they were recorded on.
    pub(super) fn with_recorded_sheets(state: &AppState, reference: CellViewRef) -> Self {
        let before_sheet_assignments = state
            .workspace
            .design_management
            .sheet_catalog(&reference.key())
            .map(|catalog| catalog.object_assignments().clone())
            .unwrap_or_default();
        Self {
            reference,
            before_sheet_assignments,
        }
    }

    /// A document the transaction touches without moving objects between
    /// sheets — the whole catalog is restored with it, or it has none.
    pub(super) fn naming(reference: CellViewRef) -> Self {
        Self {
            reference,
            before_sheet_assignments: BTreeMap::new(),
        }
    }

    pub(super) fn reference(&self) -> &CellViewRef {
        &self.reference
    }

    /// Put this document's objects back on their recorded sheets.
    ///
    /// The catalog is republished only when reconciliation actually changed
    /// something, so a project without governed sheets never advances its
    /// revision here — an undo that silently bumped the revision would fail
    /// every guard beneath it on the stack.
    pub(super) fn restore_recorded_sheets(&self, state: &mut AppState) -> Result<(), String> {
        if self.before_sheet_assignments.is_empty() {
            return Ok(());
        }
        let key = self.reference.key();
        if state
            .workspace
            .design_management
            .sheet_catalog(&key)
            .is_none()
        {
            return Ok(());
        }
        let Some(live) = schematic_for_reference(state, &self.reference).map(live_object_ids)
        else {
            return Ok(());
        };
        let mut candidate = state.workspace.design_management.clone();
        let catalog = candidate
            .sheet_catalog_mut(&key)
            .expect("the cloned catalog retains the key just found on the live one");
        let default_sheet = catalog.active_sheet_id();
        let receipt = catalog
            .reconcile_object_assignments_with(
                catalog.revision(),
                live,
                &self.before_sheet_assignments,
                default_sheet,
            )
            .map_err(|error| {
                format!(
                    "Sheet membership for '{}' could not be restored: {error}",
                    self.reference.display_path()
                )
            })?;
        if receipt.added_assignments == 0
            && receipt.removed_assignments == 0
            && receipt.removed_cross_sheet_ports == 0
        {
            return Ok(());
        }
        state
            .workspace
            .replace_design_management(candidate)
            .map_err(|error| error.to_string())?;
        Ok(())
    }
}

/// The part of a project undo record that means the same thing for every kind
/// of transaction.
#[derive(Debug, Clone)]
pub(super) struct RecordHeader {
    sequence: UndoSequence,
    documents: Vec<DocumentCompensation>,
    activated_before: Option<CellViewRef>,
    activated_after: Option<CellViewRef>,
}

impl RecordHeader {
    /// Stamp a freshly committed transaction into the global order.
    ///
    /// `activated_before` and `activated_after` are the documents the design
    /// was being edited at on each side of the transaction; undo and redo
    /// return to their own side rather than mutating whatever tab is in front.
    pub(super) fn committed(
        documents: Vec<DocumentCompensation>,
        activated_before: Option<CellViewRef>,
        activated_after: Option<CellViewRef>,
    ) -> Self {
        Self {
            sequence: next_undo_sequence(),
            documents,
            activated_before,
            activated_after,
        }
    }

    /// A transaction that owns no document — a model library, a provider
    /// ledger — and therefore never navigates.
    pub(super) fn project_only() -> Self {
        Self::committed(Vec::new(), None, None)
    }

    pub(super) const fn sequence(&self) -> UndoSequence {
        self.sequence
    }

    /// Move this record to the opposite stack. Undo asks which record moved
    /// most recently, so the sequence is redrawn at every transition.
    pub(super) fn restamp(&mut self) {
        self.sequence = next_undo_sequence();
    }

    pub(super) fn documents(&self) -> &[DocumentCompensation] {
        &self.documents
    }

    pub(super) fn undo_activates(&self) -> Option<&CellViewRef> {
        self.activated_before.as_ref()
    }

    pub(super) fn redo_activates(&self) -> Option<&CellViewRef> {
        self.activated_after.as_ref()
    }
}

/// Every stable object identity a schematic currently holds — the live set
/// sheet reconciliation is measured against.
fn live_object_ids(schematic: &SchematicState) -> Vec<u64> {
    schematic
        .components
        .iter()
        .map(|object| object.id)
        .chain(schematic.wires.iter().map(|object| object.id))
        .chain(schematic.buses.iter().map(|object| object.id))
        .chain(schematic.bus_taps.iter().map(|object| object.id))
        .chain(schematic.junctions.iter().map(|object| object.id))
        .chain(schematic.net_labels.iter().map(|object| object.id))
        .chain(schematic.design_notes.iter().map(|object| object.id))
        .chain(
            schematic
                .documentation_shapes
                .iter()
                .map(|object| object.id),
        )
        .chain(schematic.probes.iter().map(|object| object.id))
        .collect()
}
