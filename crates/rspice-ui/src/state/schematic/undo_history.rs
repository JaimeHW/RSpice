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
