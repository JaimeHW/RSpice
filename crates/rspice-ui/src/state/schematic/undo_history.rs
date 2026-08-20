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
//! - Caches (topology_version)

use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::state::SheetId;

use super::bus::{Bus, BusTap};
use super::component::Component;
use super::design_note::DesignNote;
use super::documentation_shape::DocumentationShape;
use super::net_label::{Junction, NetLabel};
use super::probe::SchematicProbe;
use super::wire::{Wire, WireConnection};

// =============================================================================
// Constants
// =============================================================================

/// Maximum number of undo steps to keep
/// Matches commercial tool defaults (Cadence Virtuoso uses 50-100)
pub const MAX_UNDO_STEPS: usize = 100;

// =============================================================================
// Global undo sequence
// =============================================================================

/// One process-global order over every undo record the session holds.
///
/// Each document owns a history and the project owns another, so no per-stack
/// index can say which of two records the user made last — and Undo has to
/// answer exactly that. A single monotonic counter answers it, and only if
/// every commit boundary draws from it.
///
/// The counter is drawn at every stack transition, not once at authoring
/// time: Undo asks which record moved most recently, so a record that is
/// undone and lands on the redo stack takes a fresh sequence, and so does one
/// that is redone. Comparing the newest sequence across the stacks is then
/// correct in both directions.
pub type UndoSequence = u64;

/// The one value the counter never hands out, so a record carrying it was
/// stamped by nobody.
pub const UNSTAMPED_UNDO_SEQUENCE: UndoSequence = 0;

static NEXT_UNDO_SEQUENCE: AtomicU64 = AtomicU64::new(UNSTAMPED_UNDO_SEQUENCE + 1);

/// Draw the next global sequence. Every commit boundary — this module's
/// `end_operation`, `undo` and `redo`, and every project-level transaction
/// record — must call this and nothing else.
pub fn next_undo_sequence() -> UndoSequence {
    NEXT_UNDO_SEQUENCE.fetch_add(1, Ordering::Relaxed)
}

// =============================================================================
// SchematicSnapshot
// =============================================================================

/// A snapshot of the undoable portion of schematic state
///
/// Captures only design data that participates in undo/redo.
/// View state (zoom, pan, selection) is intentionally excluded.
#[derive(Debug, Clone)]
pub struct SchematicSnapshot {
    /// Project-portable editor and connectivity semantics.
    pub document_policy: super::document_policy::SchematicDocumentPolicy,
    /// Canvas spacing derived from the document grid pitch.
    pub grid_size: i32,
    /// All placed components
    pub components: Vec<Component>,
    /// All wires
    pub wires: Vec<Wire>,
    /// Durable bus polylines.
    pub buses: Vec<Bus>,
    /// Typed bus taps.
    pub bus_taps: Vec<BusTap>,
    /// Explicit wire junctions
    pub junctions: Vec<Junction>,
    /// Net labels for naming nodes
    pub net_labels: Vec<NetLabel>,
    /// Durable non-electrical design notes.
    pub design_notes: Vec<DesignNote>,
    /// Durable non-electrical documentation geometry.
    pub documentation_shapes: Vec<DocumentationShape>,
    /// Durable schematic probe flags and their exact output bindings.
    pub probes: Vec<SchematicProbe>,
    /// Wire-to-terminal connections (for rubber-banding)
    pub connections: Vec<WireConnection>,
    /// Which sheet each object sat on while this snapshot was the live design.
    ///
    /// Sheet membership belongs to the project's sheet catalog, not to the
    /// drawing, so it is never restored by [`SchematicSnapshot::apply`] and
    /// never participates in equality — a catalog-only change must not
    /// manufacture an undo step. It is retained here because the catalog is
    /// the one authority that forgets: reconciliation drops the membership of
    /// an object the moment it leaves the drawing, and only this snapshot
    /// still knows where an object undone back into existence used to live.
    ///
    /// Empty until the boundary that owns both authorities states it.
    pub sheet_assignments: BTreeMap<u64, SheetId>,
}

impl SchematicSnapshot {
    /// Create a snapshot from the current schematic state
    pub fn capture(state: &super::state::SchematicState) -> Self {
        Self {
            document_policy: state.document_policy,
            grid_size: state.grid_size,
            components: state.components.clone(),
            wires: state.wires.clone(),
            buses: state.buses.clone(),
            bus_taps: state.bus_taps.clone(),
            junctions: state.junctions.clone(),
            net_labels: state.net_labels.clone(),
            design_notes: state.design_notes.clone(),
            documentation_shapes: state.documentation_shapes.clone(),
            probes: state.probes.clone(),
            connections: state.connections.clone(),
            sheet_assignments: BTreeMap::new(),
        }
    }

    /// Apply this snapshot to a schematic state
    ///
    /// Restores undoable fields without touching view state.
    pub fn apply(&self, state: &mut super::state::SchematicState) {
        let electrical_changed = self.document_policy != state.document_policy
            || self.grid_size != state.grid_size
            || self.components != state.components
            || self.wires != state.wires
            || self.buses != state.buses
            || self.bus_taps != state.bus_taps
            || self.junctions != state.junctions
            || self.net_labels != state.net_labels
            || self.connections != state.connections;
        state.document_policy = self.document_policy;
        state.grid_size = self.grid_size;
        state.components = self.components.clone();
        state.wires = self.wires.clone();
        state.buses = self.buses.clone();
        state.bus_taps = self.bus_taps.clone();
        state.junctions = self.junctions.clone();
        state.net_labels = self.net_labels.clone();
        state.design_notes = self.design_notes.clone();
        state.documentation_shapes = self.documentation_shapes.clone();
        state.probes = self.probes.clone();
        state.connections = self.connections.clone();

        if electrical_changed {
            state.bump_topology_version();
        }

        // Mark as dirty since we're restoring to a different state
        state.is_dirty = true;

        // Clear selection since entities may have changed
        state.selection.clear();
    }

    /// Check if two snapshots have the same content
    ///
    /// Used to prevent creating undo entries when nothing changed.
    pub fn is_equal(&self, other: &Self) -> bool {
        self.document_policy == other.document_policy
            && self.grid_size == other.grid_size
            && self.components == other.components
            && self.wires == other.wires
            && self.buses == other.buses
            && self.bus_taps == other.bus_taps
            && self.junctions == other.junctions
            && self.net_labels == other.net_labels
            && self.design_notes == other.design_notes
            && self.documentation_shapes == other.documentation_shapes
            && self.probes == other.probes
            && self.connections == other.connections
    }

    /// Compare the retained design baseline directly with live document data
    /// without allocating another full snapshot. Modal authority checks run on
    /// every preview frame, so an allocation-free comparison is essential for
    /// large schematics.
    pub fn is_equal_state(&self, state: &super::state::SchematicState) -> bool {
        self.document_policy == state.document_policy
            && self.grid_size == state.grid_size
            && self.components == state.components
            && self.wires == state.wires
            && self.buses == state.buses
            && self.bus_taps == state.bus_taps
            && self.junctions == state.junctions
            && self.net_labels == state.net_labels
            && self.design_notes == state.design_notes
            && self.documentation_shapes == state.documentation_shapes
            && self.probes == state.probes
            && self.connections == state.connections
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
    /// Where this entry sits in the one order Undo arbitrates over. Only
    /// [`UndoHistory::stamped`] produces an entry, so the value is never
    /// [`UNSTAMPED_UNDO_SEQUENCE`].
    pub sequence: UndoSequence,
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
    /// Balanced nested transaction scopes inside the owning operation.
    ///
    /// Nested helpers are common in editor code. They must extend the outer
    /// atomic operation instead of overwriting its before-snapshot.
    nesting_depth: usize,
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
    /// Sheet membership currently in force for this document, as the boundary
    /// that owns the sheet catalog last stated it. Every snapshot this history
    /// stamps carries a copy, so the membership travels with the design the
    /// snapshot holds.
    live_sheet_assignments: BTreeMap<u64, SheetId>,
    /// Sheet membership carried by the step this history applied last, held
    /// for that same boundary and cleared when it takes it.
    restored_sheet_assignments: BTreeMap<u64, SheetId>,
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
            live_sheet_assignments: BTreeMap::new(),
            restored_sheet_assignments: BTreeMap::new(),
        }
    }

    /// Initialize the history (call once at startup or after file load)
    pub fn initialize(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.pending = None;
        self.initialized = true;
        self.restored_sheet_assignments.clear();
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
    pub fn begin_operation(
        &mut self,
        mut before_snapshot: SchematicSnapshot,
        description: impl Into<String>,
    ) {
        before_snapshot
            .sheet_assignments
            .clone_from(&self.live_sheet_assignments);
        if let Some(pending) = self.pending.as_mut() {
            pending.nesting_depth = pending.nesting_depth.saturating_add(1);
            log::debug!(
                "nested undo operation joined outer transaction {:?}",
                pending.description
            );
            return;
        }

        self.pending = Some(PendingOperation {
            before_snapshot,
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

        // Only create undo entry if state actually changed
        if pending.before_snapshot.is_equal(&after_snapshot) {
            return false;
        }

        // Create undo entry with the "before" snapshot
        let entry = Self::stamped(pending.before_snapshot, pending.description);

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

    /// A step that has just been applied makes its own membership the live
    /// one, and hands it to the boundary that owns the sheet catalog.
    fn adopt_restored_sheet_assignments(&mut self, assignments: &BTreeMap<u64, SheetId>) {
        self.live_sheet_assignments.clone_from(assignments);
        self.restored_sheet_assignments.clone_from(assignments);
    }

    /// The only way an [`UndoEntry`] comes into existence, so no stack can
    /// hold one that arbitration would sort as oldest by accident.
    fn stamped(before: SchematicSnapshot, description: String) -> UndoEntry {
        let entry = UndoEntry {
            before: Arc::new(before),
            description,
            sequence: next_undo_sequence(),
        };
        debug_assert_ne!(
            entry.sequence, UNSTAMPED_UNDO_SEQUENCE,
            "every commit boundary draws a live global sequence"
        );
        entry
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
        mut current_snapshot: SchematicSnapshot,
    ) -> Option<(SchematicSnapshot, String)> {
        if self.undo_stack.is_empty() {
            return None;
        }
        current_snapshot
            .sheet_assignments
            .clone_from(&self.live_sheet_assignments);
        let entry = self
            .undo_stack
            .pop_back()
            .expect("the non-empty undo stack still holds its newest entry");

        // Save current state for redo. The redo entry takes a fresh sequence:
        // Redo has to reverse the order things were undone in, which is not
        // the order they were authored in.
        self.redo_stack
            .push(Self::stamped(current_snapshot, entry.description.clone()));

        self.adopt_restored_sheet_assignments(&entry.before.sheet_assignments);
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
        mut current_snapshot: SchematicSnapshot,
    ) -> Option<(SchematicSnapshot, String)> {
        if self.redo_stack.is_empty() {
            return None;
        }
        current_snapshot
            .sheet_assignments
            .clone_from(&self.live_sheet_assignments);
        let entry = self
            .redo_stack
            .pop()
            .expect("the non-empty redo stack still holds its newest entry");

        // Save current state for undo, freshly stamped for the same reason
        // `undo` restamps: the redone step is now the most recent one.
        self.undo_stack
            .push_back(Self::stamped(current_snapshot, entry.description.clone()));

        self.adopt_restored_sheet_assignments(&entry.before.sheet_assignments);
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

    /// Where the next undo step sits in the global order, for arbitration
    /// against the project history.
    pub fn undo_sequence(&self) -> Option<UndoSequence> {
        self.undo_stack.back().map(|entry| entry.sequence)
    }

    /// Where the next redo step sits in the global order.
    pub fn redo_sequence(&self) -> Option<UndoSequence> {
        self.redo_stack.last().map(|entry| entry.sequence)
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

    /// State the sheet membership now in force, so every later snapshot
    /// carries it. Stamping the membership as the design is captured — rather
    /// than editing a committed entry — keeps a synchronization from deep
    /// copying an already shared snapshot.
    pub fn set_live_sheet_assignments(&mut self, assignments: BTreeMap<u64, SheetId>) {
        self.live_sheet_assignments = assignments;
    }

    /// Take the sheet membership the last applied step carried. Consuming it
    /// keeps a later reconciliation from re-stating a membership that has
    /// already been honored once.
    pub fn take_restored_sheet_assignments(&mut self) -> BTreeMap<u64, SheetId> {
        std::mem::take(&mut self.restored_sheet_assignments)
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.pending = None;
        self.initialized = false;
        self.restored_sheet_assignments.clear();
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
            document_policy: super::super::document_policy::SchematicDocumentPolicy::default(),
            grid_size: 10,
            components: (0..n)
                .map(|i| Component::new(i as u64, ComponentType::Resistor, Point::new(i as i32, 0)))
                .collect(),
            wires: Vec::new(),
            buses: Vec::new(),
            bus_taps: Vec::new(),
            junctions: Vec::new(),
            net_labels: Vec::new(),
            design_notes: Vec::new(),
            documentation_shapes: Vec::new(),
            probes: Vec::new(),
            connections: Vec::new(),
            sheet_assignments: BTreeMap::new(),
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
    fn nested_begin_joins_the_outer_atomic_transaction() {
        // A helper can open an undo scope inside an already-atomic caller.
        // The inner end only closes its nesting level; the outer end commits
        // one entry from the original before snapshot.
        let mut history = UndoHistory::default();
        history.initialize();

        history.begin_operation(snapshot_with(0), "First");
        history.begin_operation(snapshot_with(1), "Second");
        assert!(!history.end_operation(snapshot_with(2)));
        let created = history.end_operation(snapshot_with(2));

        assert!(created);
        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.undo_description(), Some("First"));
        let (restored, _) = history.undo(snapshot_with(2)).unwrap();
        assert!(restored.is_equal(&snapshot_with(0)));
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
    fn every_commit_boundary_draws_a_live_global_sequence() {
        let mut history = UndoHistory::default();
        history.initialize();

        history.begin_operation(snapshot_with(0), "Add R1");
        assert!(history.end_operation(snapshot_with(1)));
        let commit = history.undo_sequence().expect("the commit is stamped");
        assert_ne!(commit, UNSTAMPED_UNDO_SEQUENCE);

        history.begin_operation(snapshot_with(1), "Add C1");
        assert!(history.end_operation(snapshot_with(2)));
        let later = history
            .undo_sequence()
            .expect("the second commit is stamped");
        assert!(
            later > commit,
            "a later commit must sort after an earlier one"
        );
    }

    #[test]
    fn undo_and_redo_restamp_so_the_step_that_moved_last_sorts_newest() {
        let mut history = UndoHistory::default();
        history.initialize();

        history.begin_operation(snapshot_with(0), "Add R1");
        history.end_operation(snapshot_with(1));
        let committed = history.undo_sequence().expect("commit");

        history.undo(snapshot_with(1)).expect("undo entry");
        let undone = history.redo_sequence().expect("the redo entry is stamped");
        assert!(
            undone > committed,
            "the record that just moved to the redo stack is the newest one"
        );

        history.redo(snapshot_with(0)).expect("redo entry");
        let redone = history.undo_sequence().expect("the undo entry is stamped");
        assert!(
            redone > undone,
            "the record that just moved back is newer than when it was undone"
        );
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
    fn complete_component_properties_and_connections_participate_in_undo_equality() {
        let mut state = SchematicState::default();
        let id = state.add_component(ComponentType::Resistor, Point::new(10, 20));
        state.clear_undo_history();

        assert!(state.with_undo("Edit component properties", |schematic| {
            let component = schematic
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
        let original_policy = state.document_policy;
        let original_grid_size = state.grid_size;

        assert!(state.with_undo("change schematic grid pitch", |schematic| {
            schematic.document_policy.grid_pitch =
                super::super::document_policy::SchematicGridPitch::Metric;
            schematic.grid_size = schematic.document_policy.grid_pitch.canvas_grid_size();
        }));
        assert_ne!(state.document_policy, original_policy);
        assert_ne!(state.grid_size, original_grid_size);

        assert!(state.undo());
        assert_eq!(state.document_policy, original_policy);
        assert_eq!(state.grid_size, original_grid_size);
        assert!(state.redo());
        assert_eq!(
            state.document_policy.grid_pitch,
            super::super::document_policy::SchematicGridPitch::Metric
        );
        assert_eq!(
            state.grid_size,
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

        assert_eq!(state.undo_history.undo_count(), 1);
        assert_eq!(state.undo_description(), Some("outer edit"));
        assert!(state.undo());
        assert!(state.components.is_empty());
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
