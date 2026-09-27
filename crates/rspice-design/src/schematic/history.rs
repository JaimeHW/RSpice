//! Undoable schematic snapshots, bounded committed history, and shared transaction order.

use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use rspice_design_model::design_management::SheetId;

use super::bus::{Bus, BusTap};
use super::component::Component;
use super::design_note::DesignNote;
use super::documentation_shape::DocumentationShape;
use super::net_label::{Junction, NetLabel};
use super::probe::SchematicProbe;
use super::wire::{Wire, WireConnection};

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
/// `commit`, `undo` and `redo`, and every project-level transaction
/// record — must call this and nothing else.
pub fn next_undo_sequence() -> UndoSequence {
    NEXT_UNDO_SEQUENCE.fetch_add(1, Ordering::Relaxed)
}

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
    pub fn capture(state: &super::document::SchematicDocument) -> Self {
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

    /// Restore undoable document fields and report whether electrical content changed.
    /// The validated-save journal and project sheet catalog are not restored.
    pub fn apply(&self, state: &mut super::document::SchematicDocument) -> bool {
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

        electrical_changed
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
    pub fn is_equal_document(&self, state: &super::document::SchematicDocument) -> bool {
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

/// A single entry in the undo/redo stack.
///
/// Snapshots are `Arc`-shared: cloning the history (workspace buffering,
/// autosave) bumps refcounts instead of deep-copying up to `MAX_UNDO_STEPS`
/// full copies of the design.
#[derive(Debug, Clone)]
struct UndoEntry {
    /// Snapshot of state BEFORE the operation
    before: Arc<SchematicSnapshot>,
    /// Human-readable description of the operation
    description: String,
    /// Where this entry sits in the one order Undo arbitrates over. Only
    /// [`SchematicHistory::stamped`] produces an entry, so the value is never
    /// [`UNSTAMPED_UNDO_SEQUENCE`].
    sequence: UndoSequence,
}

/// Bounded committed schematic history with shared immutable snapshots.
/// Pending gestures and their cancellation state belong to the editor.
#[derive(Debug, Clone)]
pub struct SchematicHistory {
    /// Undo stack (past operations)
    undo_stack: VecDeque<UndoEntry>,
    /// Redo stack (undone operations available for redo)
    redo_stack: Vec<UndoEntry>,
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

impl Default for SchematicHistory {
    fn default() -> Self {
        Self::new(MAX_UNDO_STEPS)
    }
}

impl SchematicHistory {
    /// Capture membership at the start of an edit, before reconciliation can remove it.
    pub fn capture_sheet_assignments(&self, snapshot: &mut SchematicSnapshot) {
        snapshot
            .sheet_assignments
            .clone_from(&self.live_sheet_assignments);
    }

    /// Create a new undo history with specified max size
    pub fn new(max_size: usize) -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: Vec::new(),
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
        self.initialized = true;
        self.restored_sheet_assignments.clear();
    }

    /// Check if history has been initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Retain an actual document change and start a new undo branch.
    pub fn commit(
        &mut self,
        before_snapshot: SchematicSnapshot,
        after_snapshot: &SchematicSnapshot,
        description: String,
    ) -> bool {
        // Only create undo entry if state actually changed
        if before_snapshot.is_equal(after_snapshot) {
            return false;
        }

        // Create undo entry with the "before" snapshot
        let entry = Self::stamped(before_snapshot, description);

        self.undo_stack.push_back(entry);

        // Enforce max size
        while self.undo_stack.len() > self.max_size {
            self.undo_stack.pop_front();
        }

        // Clear redo stack - new operation invalidates redo
        self.redo_stack.clear();

        true
    }

    /// A step that has just been applied makes its own membership the live
    /// one, and hands it to the boundary that owns the sheet catalog.
    pub fn adopt_restored_sheet_assignments(&mut self, assignments: &BTreeMap<u64, SheetId>) {
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
        self.initialized = false;
        self.restored_sheet_assignments.clear();
    }

    /// A committed project transaction starts a new branch of this document's
    /// history without discarding the preceding local undo steps.
    pub fn clear_redo(&mut self) {
        self.redo_stack.clear();
    }
}

/// Take the snapshot out of its `Arc` — zero-copy when this history holds
/// the only reference (the common case; clones exist only in workspace
/// buffers).
fn unwrap_snapshot(snapshot: Arc<SchematicSnapshot>) -> SchematicSnapshot {
    Arc::try_unwrap(snapshot).unwrap_or_else(|shared| (*shared).clone())
}

#[cfg(test)]
mod tests {
    use super::super::component_type::ComponentType;
    use super::super::document::SchematicDocument;
    use super::*;
    use rspice_design_model::Point;

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

    #[test]
    fn undo_returns_before_snapshot_and_arms_redo() {
        let mut history = SchematicHistory::default();
        history.initialize();

        history.commit(snapshot_with(0), &snapshot_with(1), "Add R1".into());

        let (restored, desc) = history.undo(snapshot_with(1)).expect("undo entry");
        assert!(restored.is_equal(&snapshot_with(0)));
        assert_eq!(desc, "Add R1");
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 1);
        assert_eq!(history.redo_description(), Some("Add R1"));
    }

    #[test]
    fn redo_returns_after_snapshot_and_rearms_undo() {
        let mut history = SchematicHistory::default();
        history.initialize();

        history.commit(snapshot_with(0), &snapshot_with(1), "Add R1".into());
        let (restored, _) = history.undo(snapshot_with(1)).unwrap();

        let (redone, desc) = history.redo(restored).expect("redo entry");
        assert!(redone.is_equal(&snapshot_with(1)));
        assert_eq!(desc, "Add R1");
        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.redo_count(), 0);
    }

    #[test]
    fn new_operation_after_undo_clears_redo_stack() {
        let mut history = SchematicHistory::default();
        history.initialize();

        history.commit(snapshot_with(0), &snapshot_with(1), "Add R1".into());
        history.undo(snapshot_with(1)).unwrap();
        assert!(history.can_redo());

        history.commit(snapshot_with(0), &snapshot_with(2), "Add C1".into());

        assert!(!history.can_redo());
        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.undo_description(), Some("Add C1"));
    }

    #[test]
    fn undo_on_empty_history_is_a_noop() {
        let mut history = SchematicHistory::default();
        history.initialize();

        assert!(history.undo(snapshot_with(0)).is_none());
        assert!(history.redo(snapshot_with(0)).is_none());
        // A failed undo must not pollute the redo stack.
        assert_eq!(history.redo_count(), 0);
        assert_eq!(history.undo_count(), 0);
    }

    #[test]
    fn history_is_capped_at_max_undo_steps() {
        let mut history = SchematicHistory::default();
        history.initialize();

        // Push 105 distinct operations; the 5 oldest must be evicted.
        for i in 0..105 {
            assert!(history.commit(snapshot_with(i), &snapshot_with(i + 1), format!("Op {i}")));
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
    fn every_commit_boundary_draws_a_live_global_sequence() {
        let mut history = SchematicHistory::default();
        history.initialize();

        assert!(history.commit(snapshot_with(0), &snapshot_with(1), "Add R1".into()));
        let commit = history.undo_sequence().expect("the commit is stamped");
        assert_ne!(commit, UNSTAMPED_UNDO_SEQUENCE);

        assert!(history.commit(snapshot_with(1), &snapshot_with(2), "Add C1".into()));
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
        let mut history = SchematicHistory::default();
        history.initialize();

        history.commit(snapshot_with(0), &snapshot_with(1), "Add R1".into());
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
}
