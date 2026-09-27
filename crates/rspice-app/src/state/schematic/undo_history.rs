//! Pending schematic edit scopes and editor-only cancellation state.

use std::sync::atomic::{AtomicU64, Ordering};

use super::committed_history::SchematicHistory;
pub use super::committed_history::{SchematicSnapshot, UndoSequence, next_undo_sequence};

static NEXT_OPERATION_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone)]
pub(super) struct OperationCancelState {
    selection: super::selection::Selection,
    was_dirty: bool,
}

impl OperationCancelState {
    pub(super) fn capture(state: &super::state::SchematicState) -> Self {
        Self {
            selection: state.selection.clone(),
            was_dirty: state.is_dirty,
        }
    }
}

/// Tracks an in-progress operation for transaction-based undo
#[derive(Debug, Clone)]
pub(super) struct PendingOperation {
    id: u64,
    cancel_state: Option<OperationCancelState>,
    /// Snapshot captured at begin_operation
    before_snapshot: SchematicSnapshot,
    /// Description of the operation
    description: String,
    /// Balanced nested transaction scopes inside the owning operation.
    ///
    /// Nested helpers are common in editor code. They must extend the outer
    /// atomic operation instead of overwriting its before-snapshot.
    nesting_depth: usize,
}

impl PendingOperation {
    pub(super) fn restore_cancelled(&self, state: &mut super::state::SchematicState) {
        if !self.before_snapshot.is_equal_document(&state.document) {
            state.apply_snapshot(&self.before_snapshot);
            state.reconcile_grid_pitch_runtime();
            state.recalculate_runtime_state();
        }
        if let Some(cancel) = &self.cancel_state {
            state.selection.clone_from(&cancel.selection);
            state.is_dirty = cancel.was_dirty;
        }
    }
}

/// Committed document history beside the one pending editor operation.
#[derive(Debug, Clone, Default)]
pub struct UndoHistory {
    pub committed: SchematicHistory,
    pending: Option<PendingOperation>,
}

impl UndoHistory {
    pub fn initialize(&mut self) {
        self.committed.initialize();
        self.pending = None;
    }

    pub fn clear(&mut self) {
        self.committed.clear();
        self.pending = None;
    }

    /// Begin an undoable operation
    ///
    /// Call this BEFORE modifying state. Captures current state as the
    /// "before" snapshot for the undo entry.
    ///
    /// # Arguments
    /// * `before_snapshot` - Current state snapshot
    /// * `description` - Human-readable description of the upcoming operation
    ///
    pub(super) fn begin_operation(
        &mut self,
        mut before_snapshot: SchematicSnapshot,
        description: impl Into<String>,
        cancel_state: Option<OperationCancelState>,
    ) {
        self.committed
            .capture_sheet_assignments(&mut before_snapshot);
        if let Some(pending) = self.pending.as_mut() {
            pending.nesting_depth = pending.nesting_depth.saturating_add(1);
            log::debug!(
                "nested undo operation joined outer transaction {:?}",
                pending.description
            );
            return;
        }

        self.pending = Some(PendingOperation {
            id: NEXT_OPERATION_ID.fetch_add(1, Ordering::Relaxed),
            before_snapshot,
            cancel_state,
            description: description.into(),
            nesting_depth: 0,
        });
    }

    /// End an undoable operation
    ///
    /// Call this AFTER modifying state. Compares before/after snapshots and
    /// creates an undo entry only if state actually changed.
    ///
    /// # Arguments
    /// * `after_snapshot` - Current state snapshot after the operation
    ///
    /// # Returns
    /// `true` if an undo entry was created (state changed), `false` otherwise
    pub fn end_operation(&mut self, after_snapshot: SchematicSnapshot) -> bool {
        if let Some(pending) = self.pending.as_mut()
            && pending.nesting_depth > 0
        {
            pending.nesting_depth -= 1;
            return false;
        }

        let pending = match self.pending.take() {
            Some(p) => p,
            None => {
                log::warn!("end_operation called without begin_operation");
                return false;
            }
        };

        self.committed.commit(
            pending.before_snapshot,
            &after_snapshot,
            pending.description,
        )
    }

    /// Cancel a pending operation without creating an undo entry
    pub(super) fn cancel_operation(&mut self) -> Option<PendingOperation> {
        let pending = self.pending.take()?;
        self.committed
            .adopt_restored_sheet_assignments(&pending.before_snapshot.sheet_assignments);
        Some(pending)
    }

    pub(crate) fn pending_operation_id(&self) -> Option<u64> {
        self.pending.as_ref().map(|pending| pending.id)
    }

    pub(crate) fn pending_was_dirty(&self) -> Option<bool> {
        self.pending
            .as_ref()?
            .cancel_state
            .as_ref()
            .map(|cancel| cancel.was_dirty)
    }

    /// Save acceptance can change whether the cancellation baseline is dirty
    /// without committing or replacing the live operation.
    pub(crate) fn set_pending_was_dirty(&mut self, was_dirty: bool) {
        if let Some(pending) = &mut self.pending
            && let Some(cancel) = &mut pending.cancel_state
        {
            cancel.was_dirty = was_dirty;
        }
    }

    /// Check if an operation is currently pending
    pub fn has_pending_operation(&self) -> bool {
        self.pending.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::super::component_type::ComponentType;
    use super::super::document::SchematicDocument;
    use super::super::point::Point;
    use super::super::state::SchematicState;
    use super::*;

    /// Snapshot containing `n` distinct resistors (and nothing else).
    fn snapshot_with(n: usize) -> SchematicSnapshot {
        SchematicSnapshot::capture(&SchematicDocument {
            grid_size: 10,
            components: (0..n)
                .map(|i| {
                    super::super::component::Component::new(
                        i as u64,
                        ComponentType::Resistor,
                        Point::new(i as i32, 0),
                    )
                })
                .collect(),
            ..SchematicDocument::default()
        })
    }

    fn capture(state: &SchematicState) -> SchematicSnapshot {
        SchematicSnapshot::capture(&state.document)
    }

    // -------------------------------------------------------------------------
    // UndoHistory (transaction core)
    // -------------------------------------------------------------------------

    #[test]
    fn begin_edit_end_creates_exactly_one_entry() {
        let mut history = UndoHistory::default();
        history.initialize();

        history.begin_operation(snapshot_with(0), "Add R1", None);
        let created = history.end_operation(snapshot_with(1));

        assert!(created);
        assert_eq!(history.committed.undo_count(), 1);
        assert_eq!(history.committed.redo_count(), 0);
        assert!(history.committed.can_undo());
        assert_eq!(history.committed.undo_description(), Some("Add R1"));
        assert!(!history.has_pending_operation());
    }

    #[test]
    fn begin_end_without_change_creates_no_entry() {
        let mut history = UndoHistory::default();
        history.initialize();

        history.begin_operation(snapshot_with(2), "No-op move", None);
        let created = history.end_operation(snapshot_with(2));

        assert!(!created);
        assert_eq!(history.committed.undo_count(), 0);
        assert!(!history.committed.can_undo());
    }

    #[test]
    fn end_without_begin_is_a_noop() {
        let mut history = UndoHistory::default();
        history.initialize();

        assert!(!history.end_operation(snapshot_with(1)));
        assert_eq!(history.committed.undo_count(), 0);
    }

    #[test]
    fn cancel_discards_pending_operation() {
        let mut history = UndoHistory::default();
        history.initialize();

        history.begin_operation(snapshot_with(0), "Cancelled drag", None);
        assert!(history.has_pending_operation());
        assert!(history.cancel_operation().is_some());

        assert!(!history.has_pending_operation());
        // A later end_operation has no pending transaction to commit.
        assert!(!history.end_operation(snapshot_with(1)));
        assert_eq!(history.committed.undo_count(), 0);
    }

    #[test]
    fn nested_begin_joins_the_outer_atomic_transaction() {
        // A helper can open an undo scope inside an already-atomic caller.
        // The inner end only closes its nesting level; the outer end commits
        // one entry from the original before snapshot.
        let mut history = UndoHistory::default();
        history.initialize();

        history.begin_operation(snapshot_with(0), "First", None);
        history.begin_operation(snapshot_with(1), "Second", None);
        assert!(!history.end_operation(snapshot_with(2)));
        let created = history.end_operation(snapshot_with(2));

        assert!(created);
        assert_eq!(history.committed.undo_count(), 1);
        assert_eq!(history.committed.undo_description(), Some("First"));
        let (restored, _) = history.committed.undo(snapshot_with(2)).unwrap();
        assert!(restored.is_equal(&snapshot_with(0)));
    }

    #[test]
    fn clear_resets_everything_including_initialized() {
        let mut history = UndoHistory::default();
        history.initialize();
        history.begin_operation(snapshot_with(0), "Add R1", None);
        history.end_operation(snapshot_with(1));
        history.committed.undo(snapshot_with(1)).unwrap();
        history.begin_operation(snapshot_with(0), "Pending", None);

        history.clear();

        assert_eq!(history.committed.undo_count(), 0);
        assert_eq!(history.committed.redo_count(), 0);
        assert!(!history.has_pending_operation());
        assert!(!history.committed.is_initialized());
    }

    // -------------------------------------------------------------------------
    // SchematicState wrappers (with_undo / undo / redo / dirty flag)
    // -------------------------------------------------------------------------

    #[test]
    fn with_undo_records_entry_and_undo_restores_state_exactly() {
        let mut state = SchematicState::default();
        state.init_undo_history();
        let baseline = capture(&state);

        let changed = state.with_undo("Add resistor", |s| {
            s.add_component(ComponentType::Resistor, Point::new(10, 20));
        });
        assert!(changed);
        assert_eq!(state.document.components.len(), 1);
        assert!(state.can_undo());

        assert!(state.undo());
        assert!(state.document.components.is_empty());
        assert!(capture(&state).is_equal(&baseline));
        assert!(!state.can_undo());
        assert!(state.can_redo());
    }

    #[test]
    fn state_redo_reapplies_the_operation() {
        let mut state = SchematicState::default();
        state.init_undo_history();

        state.with_undo("Add resistor", |s| {
            s.add_component(ComponentType::Resistor, Point::new(10, 20));
        });
        let after = capture(&state);

        assert!(state.undo());
        assert!(state.redo());
        assert_eq!(state.document.components.len(), 1);
        assert!(capture(&state).is_equal(&after));
        assert!(state.can_undo());
        assert!(!state.can_redo());
    }

    #[test]
    fn with_undo_returns_false_when_nothing_changed() {
        let mut state = SchematicState::default();
        state.init_undo_history();

        let changed = state.with_undo("Nothing", |_| {});
        assert!(!changed);
        assert!(!state.can_undo());
    }

    #[test]
    fn complete_component_properties_and_connections_participate_in_undo_equality() {
        let mut state = SchematicState::default();
        let id = state.add_component(ComponentType::Resistor, Point::new(10, 20));
        state.clear_undo_history();

        assert!(state.with_undo("Edit component properties", |schematic| {
            let component = schematic
                .document
                .components
                .iter_mut()
                .find(|component| component.id == id)
                .unwrap();
            component.name = "R99".to_owned();
            component.params = "tc1=0.001".to_owned();
        }));
        assert!(state.can_undo());
        assert!(state.undo());
        let component = state
            .document
            .components
            .iter()
            .find(|component| component.id == id)
            .unwrap();
        assert_ne!(component.name, "R99");
        assert!(component.params.is_empty());
    }

    #[test]
    fn project_portable_grid_policy_participates_in_undo_and_redo() {
        let mut state = SchematicState::default();
        state.init_undo_history();
        let original_policy = state.document.document_policy;
        let original_grid_size = state.document.grid_size;

        assert!(state.with_undo("change schematic grid pitch", |schematic| {
            schematic.document.document_policy.grid_pitch =
                super::super::document_policy::SchematicGridPitch::Metric;
            schematic.document.grid_size = schematic
                .document
                .document_policy
                .grid_pitch
                .canvas_grid_size();
        }));
        assert_ne!(state.document.document_policy, original_policy);
        assert_ne!(state.document.grid_size, original_grid_size);

        assert!(state.undo());
        assert_eq!(state.document.document_policy, original_policy);
        assert_eq!(state.document.grid_size, original_grid_size);
        assert!(state.redo());
        assert_eq!(
            state.document.document_policy.grid_pitch,
            super::super::document_policy::SchematicGridPitch::Metric
        );
        assert_eq!(
            state.document.grid_size,
            super::super::document_policy::SchematicGridPitch::Metric.canvas_grid_size()
        );
    }

    #[test]
    fn undo_and_redo_preserve_exact_runtime_wire_and_bus_routing() {
        let mut state = SchematicState::default();
        state.wire_drawing.routing_mode = super::super::wire::WireRoutingMode::VerticalFirst;
        state.bus_drawing.routing_mode = super::super::wire::WireRoutingMode::VerticalFirst;
        state.init_undo_history();

        assert!(state.with_undo("Add resistor", |schematic| {
            schematic.add_component(ComponentType::Resistor, Point::new(0, 0));
        }));
        assert!(state.undo());
        assert_eq!(
            state.wire_drawing.routing_mode,
            super::super::wire::WireRoutingMode::VerticalFirst
        );
        assert_eq!(
            state.bus_drawing.routing_mode,
            super::super::wire::WireRoutingMode::VerticalFirst
        );

        assert!(state.redo());
        assert_eq!(
            state.wire_drawing.routing_mode,
            super::super::wire::WireRoutingMode::VerticalFirst
        );
        assert_eq!(
            state.bus_drawing.routing_mode,
            super::super::wire::WireRoutingMode::VerticalFirst
        );
    }

    #[test]
    fn with_undo_is_skipped_on_read_only_state() {
        let mut state = SchematicState::default();
        state.init_undo_history();
        state.read_only = true;

        let changed = state.with_undo("Add resistor", |s| {
            s.add_component(ComponentType::Resistor, Point::new(0, 0));
        });

        // The closure must not run at all on a read-only view.
        assert!(!changed);
        assert!(state.document.components.is_empty());
        assert!(!state.can_undo());
    }

    #[test]
    fn state_undo_on_empty_history_returns_false() {
        let mut state = SchematicState::default();
        state.init_undo_history();

        assert!(!state.undo());
        assert!(!state.redo());
    }

    #[test]
    fn undo_marks_state_dirty_and_clears_selection() {
        let mut state = SchematicState::default();
        state.init_undo_history();

        let id = {
            let mut id = 0;
            state.with_undo("Add resistor", |s| {
                id = s.add_component(ComponentType::Resistor, Point::new(10, 20));
            });
            id
        };
        state.is_dirty = false;
        state.selection.select_component(id);
        assert!(!state.selection.is_empty());

        assert!(state.undo());
        assert!(state.is_dirty);
        assert!(state.selection.is_empty());
    }

    #[test]
    fn begin_operation_auto_initializes_history() {
        let mut state = SchematicState::default();
        assert!(!state.undo_history.committed.is_initialized());

        state.begin_operation("Add resistor");
        state.add_component(ComponentType::Resistor, Point::new(0, 0));
        assert!(state.end_operation());

        assert!(state.undo_history.committed.is_initialized());
        assert_eq!(state.undo_history.committed.undo_count(), 1);
    }

    #[test]
    fn nested_helpers_extend_one_outer_atomic_undo_transaction() {
        let mut state = SchematicState::default();
        state.init_undo_history();

        state.begin_operation("outer edit");
        state.add_component(ComponentType::Resistor, Point::new(0, 0));
        assert!(!state.with_undo("nested helper", |schematic| {
            schematic.add_component(ComponentType::Capacitor, Point::new(20, 0));
        }));
        assert!(state.has_pending_operation());
        assert!(state.end_operation());

        assert_eq!(state.undo_history.committed.undo_count(), 1);
        assert_eq!(state.undo_description(), Some("outer edit"));
        assert!(state.undo());
        assert!(state.document.components.is_empty());
    }

    #[test]
    fn cancelled_state_operation_leaves_no_entry() {
        let mut state = SchematicState::default();
        state.init_undo_history();

        state.begin_operation("Aborted drag");
        state.add_component(ComponentType::Resistor, Point::new(0, 0));
        state.cancel_operation();

        assert!(!state.has_pending_operation());
        assert!(!state.can_undo());
        assert!(state.document.components.is_empty());
    }

    #[test]
    fn cancelled_operation_restores_selection_dirty_state_and_redo_without_rewinding_caches() {
        for was_dirty in [false, true] {
            let mut state = SchematicState::default();
            let resistor = state.add_component(ComponentType::Resistor, Point::new(10, 20));
            state.with_undo("add capacitor", |state| {
                state.add_component(ComponentType::Capacitor, Point::new(100, 20));
            });
            assert!(state.undo());
            state.selection.select_only_component(resistor);
            state.is_dirty = was_dirty;
            let before = SchematicSnapshot::capture(&state.document);
            let selection = state.selection.clone();
            let content_version = state.content_version();
            state.begin_operation("drag selection");
            state.document.components[0].pos = Point::new(80, 90);
            state.is_dirty = true;
            state.bump_topology_version();
            let dragged_topology = state.topology_version();
            state.pan = (123.0, 0.0);

            assert!(state.cancel_operation());
            assert!(before.is_equal_document(&state.document));
            assert_eq!(state.selection, selection);
            assert_eq!(state.is_dirty, was_dirty);
            assert_eq!(state.pan, (123.0, 0.0));
            assert_eq!(state.content_version(), content_version);
            assert_ne!(state.topology_version(), dragged_topology);
            assert_eq!(state.redo_description(), Some("add capacitor"));
            assert!(!state.can_undo());
            let cancelled_topology = state.topology_version();
            assert!(!state.cancel_operation());
            assert_eq!(state.topology_version(), cancelled_topology);
            assert!(state.redo());
            assert_eq!(state.document.components.len(), 2);
            assert_eq!(state.document.components[0].pos, Point::new(10, 20));
        }
    }

    #[test]
    fn cancelled_nested_operation_restores_the_outer_baseline() {
        let mut state = SchematicState::default();
        state.is_dirty = false;
        state.begin_operation("outer gesture");
        state.add_component(ComponentType::Resistor, Point::origin());
        state.begin_operation("nested helper");
        state.add_component(ComponentType::Capacitor, Point::new(50, 0));
        assert!(state.cancel_operation());
        assert!(state.document.components.is_empty());
        assert!(!state.is_dirty);
        assert!(!state.has_pending_operation());
        assert!(!state.end_operation());
        assert!(!state.can_undo());
    }

    #[test]
    fn cancelled_no_op_preserves_selection_and_does_not_dirty_or_invalidate_the_document() {
        let mut state = SchematicState::default();
        let resistor = state.add_component(ComponentType::Resistor, Point::origin());
        state.selection.select_only_component(resistor);
        state.is_dirty = false;
        let selection = state.selection.clone();
        let topology = state.topology_version();
        state.begin_operation("stationary drag");
        assert!(state.cancel_operation());
        assert_eq!(state.selection, selection);
        assert!(!state.is_dirty);
        assert_eq!(state.topology_version(), topology);
        assert!(!state.can_undo());
    }

    #[test]
    fn committed_history_does_not_retain_gesture_cancellation_state() {
        let mut state = SchematicState::default();
        state.with_undo("add component", |state| {
            state.add_component(ComponentType::Resistor, Point::origin());
        });
        assert!(state.undo_history.pending.is_none());
        assert!(state.can_undo());
    }
}
