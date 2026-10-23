//! Editor scopes around design-owned sheet topology.
//!
//! All sheets of a cell view live in one document and share its coordinates.
//! An operation that scans every wire -- splitting at a T, maintaining
//! junctions, cleaning up, rubber-banding, stretching -- would reach into the
//! sheets the user cannot see. So each of those operations demands an
//! [`ActiveSheetTopology`], and the only way to hold one is inside
//! [`SchematicState::with_active_wire_topology`], with the other sheets'
//! conductors already out of reach.
use crate::state::SchematicState;

/// Proof that the cell view's other sheets are out of reach.
///
/// Only [`SchematicState::with_active_wire_topology`] makes one, and it lends
/// it to its closure, which cannot keep it.
#[derive(Debug)]
pub struct ActiveSheetTopology(());

impl SchematicState {
    /// Run `operation` with the conductors `hidden` names taken out of the
    /// document, then put them back exactly where they were.
    pub(crate) fn with_active_wire_topology<R>(
        &mut self,
        hidden: impl Fn(u64) -> bool,
        operation: impl FnOnce(&mut SchematicState, &ActiveSheetTopology) -> R,
    ) -> R {
        let hidden = self.design.take_hidden_wire_topology(hidden);
        let result = operation(self, &ActiveSheetTopology(()));
        hidden.restore(&mut self.design);
        result
    }

    /// A document with one sheet hides nothing; tests build those.
    #[cfg(test)]
    pub(crate) fn with_single_sheet_topology<R>(
        &mut self,
        operation: impl FnOnce(&mut SchematicState, &ActiveSheetTopology) -> R,
    ) -> R {
        self.with_active_wire_topology(|_| false, operation)
    }
}
