//! Schematic Undo/Redo System
//!
//! Commercial-grade snapshot-based undo/redo following Cadence Virtuoso patterns.
//!
//! # Architecture Overview
//!
//! This system uses a **transaction-based approach** where:
//! 1. `begin_operation()` captures the current state as a "before" snapshot
//! 2. The operation modifies state
//! 3. `end_operation()` compares before/after and creates undo entry if changed
//!
//! This architecture prevents common bugs like:
//! - Creating undo entries when nothing changed
//! - Missing undo entries when checkpoint order is wrong
//! - State drift between history and actual state
//!
//! # What is Undoable (Design Data)
//!
//! - Components (placement, values, rotation, mirror)
//! - Wires (routing, endpoints)
//! - Junctions (explicit connection points)
//! - Net labels (node names)
//! - Wire connections (for rubber-banding)
//!
//! # What is NOT Undoable (View/Runtime State)
//!
//! - Zoom and pan (view preferences)
//! - Selection (transient interaction state)
//! - Tool mode (transient interaction state)
//! - Clipboard (separate from document state)
//! - Caches (net_mapping, topology_version)

use std::collections::VecDeque;
use std::sync::Arc;

use super::component::Component;
use super::net_label::{Junction, NetLabel};
use super::wire::{Wire, WireConnection};

// =============================================================================
// Constants
// =============================================================================

/// Maximum number of undo steps to keep
/// Matches commercial tool defaults (Cadence Virtuoso uses 50-100)
pub const MAX_UNDO_STEPS: usize = 100;

// =============================================================================
// SchematicSnapshot
// =============================================================================

/// A snapshot of the undoable portion of schematic state
///
/// Captures only design data that participates in undo/redo.
/// View state (zoom, pan, selection) is intentionally excluded.
#[derive(Debug, Clone)]
pub struct SchematicSnapshot {
    /// All placed components
    pub components: Vec<Component>,
    /// All wires
    pub wires: Vec<Wire>,
    /// Explicit wire junctions
    pub junctions: Vec<Junction>,
    /// Net labels for naming nodes
    pub net_labels: Vec<NetLabel>,
    /// Wire-to-terminal connections (for rubber-banding)
    pub connections: Vec<WireConnection>,
}

impl SchematicSnapshot {
    /// Create a snapshot from the current schematic state
    pub fn capture(state: &super::state::SchematicState) -> Self {
        Self {
            components: state.components.clone(),
            wires: state.wires.clone(),
            junctions: state.junctions.clone(),
            net_labels: state.net_labels.clone(),
            connections: state.connections.clone(),
        }
    }

    /// Apply this snapshot to a schematic state
    ///
    /// Restores undoable fields without touching view state.
    pub fn apply(&self, state: &mut super::state::SchematicState) {
        state.components = self.components.clone();
        state.wires = self.wires.clone();
        state.junctions = self.junctions.clone();
        state.net_labels = self.net_labels.clone();
        state.connections = self.connections.clone();

        // Invalidate caches since topology changed
        state.bump_topology_version();

        // Mark as dirty since we're restoring to a different state
        state.is_dirty = true;

        // Clear selection since entities may have changed
        state.selection.clear();
    }

    /// Check if two snapshots have the same content
    ///
    /// Used to prevent creating undo entries when nothing changed.
    pub fn is_equal(&self, other: &Self) -> bool {
        // Quick length checks first
        if self.components.len() != other.components.len()
            || self.wires.len() != other.wires.len()
            || self.junctions.len() != other.junctions.len()
            || self.net_labels.len() != other.net_labels.len()
        {
            return false;
        }

        // Deep comparison - component IDs, positions, rotations, and values
        for (a, b) in self.components.iter().zip(other.components.iter()) {
            if a.id != b.id
                || a.pos != b.pos
                || a.rotation != b.rotation
                || a.mirror_h != b.mirror_h
                || a.mirror_v != b.mirror_v
                || a.value != b.value
            {
                return false;
            }
        }

        // Wire comparison
        for (a, b) in self.wires.iter().zip(other.wires.iter()) {
            if a.id != b.id || a.points != b.points {
                return false;
            }
        }

        // Junction comparison
        for (a, b) in self.junctions.iter().zip(other.junctions.iter()) {
            if a.id != b.id || a.pos != b.pos {
                return false;
            }
        }

        // Net label comparison
        for (a, b) in self.net_labels.iter().zip(other.net_labels.iter()) {
            if a.id != b.id || a.pos != b.pos || a.name != b.name {
                return false;
            }
        }

        true
    }
}

// =============================================================================
// UndoEntry
// =============================================================================

/// A single entry in the undo/redo stack.
///
/// Snapshots are `Arc`-shared: cloning the history (workspace buffering,
/// autosave) bumps refcounts instead of deep-copying up to `MAX_UNDO_STEPS`
/// full copies of the design.
#[derive(Debug, Clone)]
pub struct UndoEntry {
    /// Snapshot of state BEFORE the operation
    pub before: Arc<SchematicSnapshot>,
    /// Human-readable description of the operation
    pub description: String,
}

// =============================================================================
// PendingOperation
// =============================================================================

/// Tracks an in-progress operation for transaction-based undo
#[derive(Debug, Clone)]
struct PendingOperation {
    /// Snapshot captured at begin_operation
    before_snapshot: SchematicSnapshot,
    /// Description of the operation
    description: String,
}

// =============================================================================
// UndoHistory
// =============================================================================

/// Commercial-grade undo/redo history manager
///
/// Uses a transaction-based model:
/// 1. Call `begin_operation()` before modifying state
/// 2. Modify state
/// 3. Call `end_operation()` to finalize the undo entry
///
/// The system automatically handles:
/// - Deduplication (no undo entry if nothing changed)
/// - Maximum history size
/// - Redo stack clearing on new operations
#[derive(Debug, Clone)]
pub struct UndoHistory {
    /// Undo stack (past operations)
    undo_stack: VecDeque<UndoEntry>,
    /// Redo stack (undone operations available for redo)
    redo_stack: Vec<UndoEntry>,
    /// Currently pending operation (between begin/end)
    pending: Option<PendingOperation>,
    /// Maximum undo steps to keep
    max_size: usize,
    /// Whether the history has been initialized
    initialized: bool,
}

impl Default for UndoHistory {
    fn default() -> Self {
        Self::new(MAX_UNDO_STEPS)
    }
}

impl UndoHistory {
    /// Create a new undo history with specified max size
    pub fn new(max_size: usize) -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: Vec::new(),
            pending: None,
            max_size,
            initialized: false,
        }
    }

    /// Initialize the history (call once at startup or after file load)
    pub fn initialize(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.pending = None;
        self.initialized = true;
    }

    /// Check if history has been initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
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
    /// # Panics
    /// Panics if called while another operation is pending (missing end_operation)
    pub fn begin_operation(
        &mut self,
        before_snapshot: SchematicSnapshot,
        description: impl Into<String>,
    ) {
        if self.pending.is_some() {
            // In a production tool, we'd log a warning and auto-commit
            // For now, just overwrite (the previous op probably did nothing)
            log::warn!("begin_operation called while another operation pending - auto-committing");
        }

        self.pending = Some(PendingOperation {
            before_snapshot,
            description: description.into(),
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
        let pending = match self.pending.take() {
            Some(p) => p,
            None => {
                log::warn!("end_operation called without begin_operation");
                return false;
            }
        };

        // Only create undo entry if state actually changed
        if pending.before_snapshot.is_equal(&after_snapshot) {
            return false;
        }

        // Create undo entry with the "before" snapshot
        let entry = UndoEntry {
            before: Arc::new(pending.before_snapshot),
            description: pending.description,
        };

        self.undo_stack.push_back(entry);

        // Enforce max size
        while self.undo_stack.len() > self.max_size {
            self.undo_stack.pop_front();
        }

        // Clear redo stack - new operation invalidates redo
        self.redo_stack.clear();

        true
    }

    /// Cancel a pending operation without creating an undo entry
    pub fn cancel_operation(&mut self) {
        self.pending = None;
    }

    /// Undo the last operation
    ///
    /// # Arguments
    /// * `current_snapshot` - Current state snapshot (for redo)
    ///
    /// # Returns
    /// The snapshot to restore to, and the description of what was undone
    pub fn undo(
        &mut self,
        current_snapshot: SchematicSnapshot,
    ) -> Option<(SchematicSnapshot, String)> {
        let entry = self.undo_stack.pop_back()?;

        // Save current state for redo
        self.redo_stack.push(UndoEntry {
            before: Arc::new(current_snapshot),
            description: entry.description.clone(),
        });

        Some((unwrap_snapshot(entry.before), entry.description))
    }

    /// Redo the last undone operation
    ///
    /// # Arguments
    /// * `current_snapshot` - Current state snapshot (for undo)
    ///
    /// # Returns
    /// The snapshot to restore to, and the description of what was redone
    pub fn redo(
        &mut self,
        current_snapshot: SchematicSnapshot,
    ) -> Option<(SchematicSnapshot, String)> {
        let entry = self.redo_stack.pop()?;

        // Save current state for undo
        self.undo_stack.push_back(UndoEntry {
            before: Arc::new(current_snapshot),
            description: entry.description.clone(),
        });

        Some((unwrap_snapshot(entry.before), entry.description))
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get description of the next undo operation
    pub fn undo_description(&self) -> Option<&str> {
        self.undo_stack.back().map(|e| e.description.as_str())
    }

    /// Get description of the next redo operation
    pub fn redo_description(&self) -> Option<&str> {
        self.redo_stack.last().map(|e| e.description.as_str())
    }

    /// Get the number of available undo steps
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of available redo steps
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.pending = None;
        self.initialized = false;
    }

    /// Check if an operation is currently pending
    pub fn has_pending_operation(&self) -> bool {
        self.pending.is_some()
    }
}

/// Take the snapshot out of its `Arc` — zero-copy when this history holds
/// the only reference (the common case; clones exist only in workspace
/// buffers).
fn unwrap_snapshot(snapshot: Arc<SchematicSnapshot>) -> SchematicSnapshot {
    Arc::try_unwrap(snapshot).unwrap_or_else(|shared| (*shared).clone())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::super::component_type::ComponentType;
    use super::super::point::Point;
    use super::super::state::SchematicState;
    use super::*;

    /// Snapshot containing `n` distinct resistors (and nothing else).
    fn snapshot_with(n: usize) -> SchematicSnapshot {
        SchematicSnapshot {
            components: (0..n)
                .map(|i| Component::new(i as u64, ComponentType::Resistor, Point::new(i as i32, 0)))
                .collect(),
            wires: Vec::new(),
            junctions: Vec::new(),
            net_labels: Vec::new(),
            connections: Vec::new(),
        }
    }

    fn capture(state: &SchematicState) -> SchematicSnapshot {
        SchematicSnapshot::capture(state)
    }

    // -------------------------------------------------------------------------
    // UndoHistory (transaction core)
    // -------------------------------------------------------------------------

    #[test]
    fn begin_edit_end_creates_exactly_one_entry() {
        let mut history = UndoHistory::default();
        history.initialize();

        history.begin_operation(snapshot_with(0), "Add R1");
        let created = history.end_operation(snapshot_with(1));

        assert!(created);
        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.redo_count(), 0);
        assert!(history.can_undo());
        assert_eq!(history.undo_description(), Some("Add R1"));
        assert!(!history.has_pending_operation());
    }

    #[test]
    fn begin_end_without_change_creates_no_entry() {
        let mut history = UndoHistory::default();
        history.initialize();

        history.begin_operation(snapshot_with(2), "No-op move");
        let created = history.end_operation(snapshot_with(2));

        assert!(!created);
        assert_eq!(history.undo_count(), 0);
        assert!(!history.can_undo());
    }

    #[test]
    fn end_without_begin_is_a_noop() {
        let mut history = UndoHistory::default();
        history.initialize();

        assert!(!history.end_operation(snapshot_with(1)));
        assert_eq!(history.undo_count(), 0);
    }

    #[test]
    fn cancel_discards_pending_operation() {
        let mut history = UndoHistory::default();
        history.initialize();

        history.begin_operation(snapshot_with(0), "Cancelled drag");
        assert!(history.has_pending_operation());
        history.cancel_operation();

        assert!(!history.has_pending_operation());
        // A later end_operation has no pending transaction to commit.
        assert!(!history.end_operation(snapshot_with(1)));
        assert_eq!(history.undo_count(), 0);
    }

    #[test]
    fn undo_returns_before_snapshot_and_arms_redo() {
        let mut history = UndoHistory::default();
        history.initialize();

        history.begin_operation(snapshot_with(0), "Add R1");
        history.end_operation(snapshot_with(1));

        let (restored, desc) = history.undo(snapshot_with(1)).expect("undo entry");
        assert!(restored.is_equal(&snapshot_with(0)));
        assert_eq!(desc, "Add R1");
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 1);
        assert_eq!(history.redo_description(), Some("Add R1"));
    }

    #[test]
    fn redo_returns_after_snapshot_and_rearms_undo() {
        let mut history = UndoHistory::default();
        history.initialize();

        history.begin_operation(snapshot_with(0), "Add R1");
        history.end_operation(snapshot_with(1));
        let (restored, _) = history.undo(snapshot_with(1)).unwrap();

        let (redone, desc) = history.redo(restored).expect("redo entry");
        assert!(redone.is_equal(&snapshot_with(1)));
        assert_eq!(desc, "Add R1");
        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.redo_count(), 0);
    }

    #[test]
    fn new_operation_after_undo_clears_redo_stack() {
        let mut history = UndoHistory::default();
        history.initialize();

        history.begin_operation(snapshot_with(0), "Add R1");
        history.end_operation(snapshot_with(1));
        history.undo(snapshot_with(1)).unwrap();
        assert!(history.can_redo());

        history.begin_operation(snapshot_with(0), "Add C1");
        history.end_operation(snapshot_with(2));

        assert!(!history.can_redo());
        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.undo_description(), Some("Add C1"));
    }

    #[test]
    fn undo_on_empty_history_is_a_noop() {
        let mut history = UndoHistory::default();
        history.initialize();

        assert!(history.undo(snapshot_with(0)).is_none());
        assert!(history.redo(snapshot_with(0)).is_none());
        // A failed undo must not pollute the redo stack.
        assert_eq!(history.redo_count(), 0);
        assert_eq!(history.undo_count(), 0);
    }

    #[test]
    fn double_begin_replaces_pending_snapshot() {
        // Current behavior: a second begin_operation overwrites the pending
        // transaction (with a logged warning). The committed entry uses the
        // second "before" snapshot and only one entry is created.
        let mut history = UndoHistory::default();
        history.initialize();

        history.begin_operation(snapshot_with(0), "First");
        history.begin_operation(snapshot_with(1), "Second");
        let created = history.end_operation(snapshot_with(2));

        assert!(created);
        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.undo_description(), Some("Second"));
        let (restored, _) = history.undo(snapshot_with(2)).unwrap();
        assert!(restored.is_equal(&snapshot_with(1)));
    }

    #[test]
    fn history_is_capped_at_max_undo_steps() {
        let mut history = UndoHistory::default();
        history.initialize();

        // Push 105 distinct operations; the 5 oldest must be evicted.
        for i in 0..105 {
            history.begin_operation(snapshot_with(i), format!("Op {i}"));
            assert!(history.end_operation(snapshot_with(i + 1)));
        }
        assert_eq!(history.undo_count(), MAX_UNDO_STEPS);
        assert_eq!(history.undo_description(), Some("Op 104"));

        // Unwind the full depth; the oldest surviving entry is op 5.
        let mut last = None;
        for _ in 0..MAX_UNDO_STEPS {
            last = history.undo(snapshot_with(0));
            assert!(last.is_some());
        }
        let (oldest, desc) = last.unwrap();
        assert_eq!(desc, "Op 5");
        assert!(oldest.is_equal(&snapshot_with(5)));
        assert!(!history.can_undo());
        assert_eq!(history.redo_count(), MAX_UNDO_STEPS);
    }

    #[test]
    fn clear_resets_everything_including_initialized() {
        let mut history = UndoHistory::default();
        history.initialize();
        history.begin_operation(snapshot_with(0), "Add R1");
        history.end_operation(snapshot_with(1));
        history.undo(snapshot_with(1)).unwrap();
        history.begin_operation(snapshot_with(0), "Pending");

        history.clear();

        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 0);
        assert!(!history.has_pending_operation());
        assert!(!history.is_initialized());
    }

    #[test]
    fn snapshot_equality_detects_component_value_edit() {
        let mut a = snapshot_with(1);
        let b = snapshot_with(1);
        assert!(a.is_equal(&b));

        a.components[0].value = "10k".to_string();
        assert!(!a.is_equal(&b));
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
        assert_eq!(state.components.len(), 1);
        assert!(state.can_undo());

        assert!(state.undo());
        assert!(state.components.is_empty());
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
        assert_eq!(state.components.len(), 1);
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
    fn with_undo_is_skipped_on_read_only_state() {
        let mut state = SchematicState::default();
        state.init_undo_history();
        state.read_only = true;

        let changed = state.with_undo("Add resistor", |s| {
            s.add_component(ComponentType::Resistor, Point::new(0, 0));
        });

        // The closure must not run at all on a read-only view.
        assert!(!changed);
        assert!(state.components.is_empty());
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
        assert!(!state.undo_history.is_initialized());

        state.begin_operation("Add resistor");
        state.add_component(ComponentType::Resistor, Point::new(0, 0));
        assert!(state.end_operation());

        assert!(state.undo_history.is_initialized());
        assert_eq!(state.undo_history.undo_count(), 1);
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
        // The (uncommitted) edit itself remains; only the undo entry is dropped.
        assert_eq!(state.components.len(), 1);
    }
}
