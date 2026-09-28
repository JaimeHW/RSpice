//! Schematic undo and redo.
//!
//! Snapshot-based history over the schematic document. The stack is bounded,
//! and a snapshot is taken at the commit boundary of an operation rather
//! than per mutation, so one user action is one undo step.

use super::*;

use crate::state::LibraryManager;

impl SchematicState {
    /// Restore document history while updating editor invalidation and selection.
    pub fn apply_snapshot(&mut self, snapshot: &super::super::undo_history::SchematicSnapshot) {
        self.design.apply_snapshot(snapshot);
        self.is_dirty = true;
        self.selection.clear();
    }

    // =========================================================================
    // Undo/Redo System (Commercial-Grade Transaction-Based)
    // =========================================================================

    /// Initialize undo history
    ///
    /// This should be called once at startup or after loading a file.
    /// Establishes the baseline for the undo system.
    pub fn init_undo_history(&mut self) {
        self.design.initialize_history();
        self.operation_cancel = None;
    }

    /// Begin an undoable operation
    ///
    /// Call this BEFORE modifying state. Captures current state as the
    /// "before" snapshot. Must be followed by `end_operation()`.
    ///
    /// # Example
    /// ```ignore
    /// state.begin_operation("Add resistor R1");
    /// state.add_component(ComponentType::Resistor, Point::new(10, 20));
    /// state.end_operation();
    /// ```
    pub fn begin_operation(&mut self, description: impl Into<String>) {
        if !self.design.history().is_initialized() {
            self.init_undo_history();
        }
        let previous = self.design.pending_operation_id();
        let operation_id = self.design.begin_operation(description);
        if previous != Some(operation_id) {
            self.operation_cancel =
                Some(super::super::undo_history::OperationCancelState::capture(
                    operation_id,
                    &self.selection,
                    self.is_dirty,
                ));
        }
    }

    /// End an undoable operation
    ///
    /// Call this AFTER modifying state. Compares before/after and creates
    /// an undo entry only if state actually changed.
    ///
    /// # Returns
    /// `true` if an undo entry was created, `false` if nothing changed.
    pub fn end_operation(&mut self) -> bool {
        let operation_id = self.design.pending_operation_id();
        let committed = self.design.end_operation();
        if committed {
            self.is_dirty = true;
        }
        if self.design.pending_operation_id().is_none()
            && let Some(cancel) = self.operation_cancel.take()
            && Some(cancel.operation_id()) == operation_id
            && !committed
        {
            self.is_dirty = cancel.was_dirty();
        }
        committed
    }

    /// Reconcile a synchronous design edit with the editor's dirty state.
    pub(in crate::state::schematic) fn finish_document_edit(&mut self, committed: bool) {
        if committed || self.design.pending_operation_id().is_some() {
            self.is_dirty = true;
        }
        if self.design.pending_operation_id().is_none() {
            self.operation_cancel = None;
        }
    }

    /// Monotonic count of committed content changes: every `end_operation`
    /// that created an entry, plus every applied undo or redo. The cheap
    /// "did this document change since I last looked" signal.
    pub fn content_version(&self) -> u64 {
        self.design.content_version()
    }

    /// Restore a pending operation's baseline without creating an undo entry.
    ///
    /// Use this if an operation was started but then cancelled (e.g., user
    /// pressed Escape during drag).
    pub fn cancel_operation(&mut self) -> bool {
        let Some(cancelled) = self.design.cancel_operation() else {
            return false;
        };
        if let Some(repaired) = cancelled.repaired {
            self.is_dirty = true;
            self.selection.clear();
            self.snap_engine.grid_size = self.design.document().grid_size;
            self.repair_clipboard_after_load();
            self.remove_stale_runtime_references(&repaired);
        }
        if let Some(cancel) = self.operation_cancel.take()
            && cancel.operation_id() == cancelled.operation_id
        {
            cancel.restore(self);
        }
        true
    }

    /// Convenience method for simple undoable operations
    ///
    /// Wraps begin_operation/operation/end_operation in a single call.
    ///
    /// # Example
    /// ```ignore
    /// state.with_undo("Add component", |s| {
    ///     s.add_component(ComponentType::Resistor, Point::new(10, 20));
    /// });
    /// ```
    pub fn with_undo<F>(&mut self, description: impl Into<String>, operation: F) -> bool
    where
        F: FnOnce(&mut Self),
    {
        // Backstop for read-only views: the UI layers refuse with a console
        // line; anything that slips through is silently skipped here.
        if self.read_only {
            return false;
        }
        self.begin_operation(description);
        operation(self);
        self.end_operation()
    }

    /// Commit an undo entry whose "before" snapshot was captured earlier —
    /// used by editing sessions (inspector text fields) that apply changes
    /// live but record one entry when the session ends, instead of two
    /// full-design snapshots per keystroke.
    ///
    /// Creates an entry only if state changed since the snapshot.
    pub fn commit_undo_from(
        &mut self,
        before: super::super::undo_history::SchematicSnapshot,
        description: impl Into<String>,
    ) -> bool {
        if !self.design.history().is_initialized() {
            self.init_undo_history();
        }
        self.design.begin_operation_from(before, description);
        self.end_operation()
    }

    /// Undo the last operation
    ///
    /// Returns `true` if undo was successful, `false` if nothing to undo.
    pub fn undo(&mut self) -> bool {
        let Some(repaired) = self.design.undo() else {
            return false;
        };
        self.is_dirty = true;
        self.selection.clear();
        self.snap_engine.grid_size = self.design.document().grid_size;
        self.repair_clipboard_after_load();
        self.remove_stale_runtime_references(&repaired);
        true
    }

    /// Redo the last undone operation
    ///
    /// Returns `true` if redo was successful, `false` if nothing to redo.
    pub fn redo(&mut self) -> bool {
        let Some(repaired) = self.design.redo() else {
            return false;
        };
        self.is_dirty = true;
        self.selection.clear();
        self.snap_engine.grid_size = self.design.document().grid_size;
        self.repair_clipboard_after_load();
        self.remove_stale_runtime_references(&repaired);
        true
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        self.history().can_undo()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        self.history().can_redo()
    }

    /// Get description of the next undo operation
    pub fn undo_description(&self) -> Option<&str> {
        self.history().undo_description()
    }

    /// Get description of the next redo operation
    pub fn redo_description(&self) -> Option<&str> {
        self.history().redo_description()
    }

    /// Clear undo history
    pub fn clear_undo_history(&mut self) {
        self.design.clear_history();
        self.operation_cancel = None;
    }

    /// Reset undo history with current state as baseline
    ///
    /// Clears all undo/redo. Use after loading a file.
    pub fn reset_undo_history(&mut self) {
        self.design.clear_history();
        self.operation_cancel = None;
        self.init_undo_history();
    }

    /// Check if an operation is currently pending
    pub fn has_pending_operation(&self) -> bool {
        self.design.pending_operation_id().is_some()
    }

    pub(crate) fn pending_operation_id(&self) -> Option<u64> {
        self.design.pending_operation_id()
    }

    /// Re-check every hierarchical placement in this document against the
    /// project's live cell catalog, and report the masters that are gone.
    ///
    /// A placement carries a copy of its master's netlist identity so an
    /// instance card can be emitted without opening the master. That copy is
    /// what lets a placement whose cell was deleted keep behaving as though
    /// the cell were still there — including when an undo restores a placement
    /// the reader had already removed. Dropping the copy is what unresolved
    /// means here: the placement stays drawn, keeps the terminals its wires
    /// are attached to, and keeps naming the master it wants, but nothing can
    /// netlist it again until that master exists.
    ///
    /// Only a project cell can go missing this way. An executable binding
    /// resolves against the running engine, and a binding into a library this
    /// project does not hold was never answered by the cell catalog at all.
    pub fn revalidate_instance_bindings(&mut self, libraries: &LibraryManager) -> Vec<String> {
        self.design
            .revalidate_instance_bindings(libraries.catalog())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::SchematicGridPitch;

    fn assert_grid_pitch_contract(state: &SchematicState, pitch: SchematicGridPitch) {
        let expected = pitch.canvas_grid_size();
        assert_eq!(state.design.document().document_policy.grid_pitch, pitch);
        assert_eq!(state.design.document().grid_size, expected);
        assert_eq!(state.snap_engine.grid_size, expected);
    }

    #[test]
    fn content_version_advances_exactly_on_commits_undo_and_redo() {
        let mut state = SchematicState::default();
        state.init_undo_history();
        let baseline = state.content_version();

        assert!(state.with_undo("change schematic grid pitch", |schematic| {
            schematic
                .design
                .document_mut_for_test()
                .document_policy
                .grid_pitch = SchematicGridPitch::Mil25;
            schematic.design.document_mut_for_test().grid_size =
                SchematicGridPitch::Mil25.canvas_grid_size();
            schematic.snap_engine.grid_size = SchematicGridPitch::Mil25.canvas_grid_size();
        }));
        assert_eq!(state.content_version(), baseline + 1);

        // An operation that changes nothing commits nothing and moves nothing.
        assert!(!state.with_undo("no-op", |_| {}));
        assert_eq!(state.content_version(), baseline + 1);

        assert!(state.undo());
        assert_eq!(state.content_version(), baseline + 2);
        assert!(state.redo());
        assert_eq!(state.content_version(), baseline + 3);
    }

    #[test]
    fn grid_pitch_undo_and_redo_reconcile_every_schematic_runtime_owner() {
        let mut state = SchematicState::default();
        state.init_undo_history();
        assert_grid_pitch_contract(&state, SchematicGridPitch::Mil50);

        assert!(state.with_undo("change schematic grid pitch", |schematic| {
            schematic
                .design
                .document_mut_for_test()
                .document_policy
                .grid_pitch = SchematicGridPitch::Mil25;
            schematic.design.document_mut_for_test().grid_size =
                SchematicGridPitch::Mil25.canvas_grid_size();
            schematic.snap_engine.grid_size = SchematicGridPitch::Mil25.canvas_grid_size();
        }));
        assert_grid_pitch_contract(&state, SchematicGridPitch::Mil25);

        assert!(state.undo());
        assert_grid_pitch_contract(&state, SchematicGridPitch::Mil50);

        assert!(state.redo());
        assert_grid_pitch_contract(&state, SchematicGridPitch::Mil25);
    }
}
