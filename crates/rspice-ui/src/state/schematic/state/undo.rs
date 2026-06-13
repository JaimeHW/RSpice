use super::*;

impl SchematicState {
    // =========================================================================
    // Undo/Redo System (Commercial-Grade Transaction-Based)
    // =========================================================================

    /// Initialize undo history
    ///
    /// This should be called once at startup or after loading a file.
    /// Establishes the baseline for the undo system.
    pub fn init_undo_history(&mut self) {
        self.undo_history.initialize();
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
        // Auto-initialize if needed
        if !self.undo_history.is_initialized() {
            self.init_undo_history();
        }

        let snapshot = super::super::undo_history::SchematicSnapshot::capture(self);
        self.undo_history.begin_operation(snapshot, description);
    }

    /// End an undoable operation
    ///
    /// Call this AFTER modifying state. Compares before/after and creates
    /// an undo entry only if state actually changed.
    ///
    /// # Returns
    /// `true` if an undo entry was created, `false` if nothing changed.
    pub fn end_operation(&mut self) -> bool {
        let snapshot = super::super::undo_history::SchematicSnapshot::capture(self);
        self.undo_history.end_operation(snapshot)
    }

    /// Cancel a pending operation without creating an undo entry
    ///
    /// Use this if an operation was started but then cancelled (e.g., user
    /// pressed Escape during drag).
    pub fn cancel_operation(&mut self) {
        self.undo_history.cancel_operation();
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
        if !self.undo_history.is_initialized() {
            self.init_undo_history();
        }
        self.undo_history.begin_operation(before, description);
        self.end_operation()
    }

    /// Undo the last operation
    ///
    /// Returns `true` if undo was successful, `false` if nothing to undo.
    pub fn undo(&mut self) -> bool {
        if !self.undo_history.can_undo() {
            return false;
        }

        // Capture current state for redo
        let current = super::super::undo_history::SchematicSnapshot::capture(self);

        if let Some((snapshot, _desc)) = self.undo_history.undo(current) {
            snapshot.apply(self);
            self.recalculate_runtime_state();
            return true;
        }

        false
    }

    /// Redo the last undone operation
    ///
    /// Returns `true` if redo was successful, `false` if nothing to redo.
    pub fn redo(&mut self) -> bool {
        if !self.undo_history.can_redo() {
            return false;
        }

        // Capture current state for undo
        let current = super::super::undo_history::SchematicSnapshot::capture(self);

        if let Some((snapshot, _desc)) = self.undo_history.redo(current) {
            snapshot.apply(self);
            self.recalculate_runtime_state();
            return true;
        }

        false
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        self.undo_history.can_undo()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        self.undo_history.can_redo()
    }

    /// Get description of the next undo operation
    pub fn undo_description(&self) -> Option<&str> {
        self.undo_history.undo_description()
    }

    /// Get description of the next redo operation
    pub fn redo_description(&self) -> Option<&str> {
        self.undo_history.redo_description()
    }

    /// Clear undo history
    pub fn clear_undo_history(&mut self) {
        self.undo_history.clear();
    }

    /// Reset undo history with current state as baseline
    ///
    /// Clears all undo/redo. Use after loading a file.
    pub fn reset_undo_history(&mut self) {
        self.undo_history.clear();
        self.init_undo_history();
    }

    /// Check if an operation is currently pending
    pub fn has_pending_operation(&self) -> bool {
        self.undo_history.has_pending_operation()
    }
}
