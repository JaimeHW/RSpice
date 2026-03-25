//! Schematic State
//!
//! Main state container for the schematic editor.

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::clipboard::ClipboardData;
use super::component::{Component, LibraryCellInstance};
use super::component_type::ComponentType;
use super::net_label::{Junction, NetLabel};
use super::point::Point;
use super::rotation::Rotation;
use super::selection::Selection;
use super::snap::SnapEngine;
use super::tool::Tool;
use super::wire::{Wire, WireConnection, WireDrawing, WireSegment};

mod editor_ops;
mod junction_ops;

// =============================================================================
// Constants
// =============================================================================

/// Default zoom level for serde deserialization (prevents black screen on file load)
fn default_zoom() -> f64 {
    1.0
}

/// Snap distance in grid units for terminal connections
const SNAP_DISTANCE: i32 = 1;

// =============================================================================
// SchematicState
// =============================================================================

/// Main schematic state
///
/// Contains all components, wires, selection state, and interaction state
/// for a single schematic document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchematicState {
    /// All placed components
    pub components: Vec<Component>,

    /// All wires
    pub wires: Vec<Wire>,

    /// Current selection
    pub selection: Selection,

    /// Current tool (runtime state, not persisted - always starts as Select)
    #[serde(skip)]
    pub tool: Tool,

    /// Wire drawing state
    pub wire_drawing: WireDrawing,

    /// Grid size in pixels
    pub grid_size: i32,

    /// Zoom level (1.0 = 100%) - not part of undo history or saved files
    /// Uses default of 1.0 when deserializing to prevent black screen
    #[serde(skip, default = "default_zoom")]
    pub zoom: f64,

    /// Pan offset in pixels - not part of undo history or saved files
    #[serde(skip, default)]
    pub pan: (f64, f64),

    /// Current schematic file path (for save without dialog)
    #[serde(skip)]
    pub current_file: Option<PathBuf>,

    /// Next component ID (runtime state, not persisted)
    #[serde(skip)]
    next_id: u64,

    /// Component counters for auto-naming (runtime state, not persisted)
    #[serde(skip)]
    component_counters: HashMap<&'static str, u32>,

    /// Clipboard for copy/paste operations
    pub clipboard: ClipboardData,

    /// Net labels for naming nodes
    pub net_labels: Vec<NetLabel>,

    /// Explicit wire junctions for connecting crossing wires
    /// Only wires sharing an endpoint OR joined by an explicit junction are connected
    pub junctions: Vec<Junction>,

    /// Preview rotation for component placement
    pub preview_rotation: Rotation,

    /// Pending library/cell/view placement payload used with `Tool::Place(CellInstance)`.
    ///
    /// Runtime interaction state only and never persisted to schematic files.
    #[serde(skip)]
    pub pending_library_cell: Option<LibraryCellInstance>,

    /// Wire-to-terminal connections (for rubber-banding)
    pub connections: Vec<WireConnection>,

    /// Cached point-to-net mapping from last netlist generation (for probe lookup)
    /// Updated after each simulation run
    #[serde(skip)]
    pub net_mapping: HashMap<Point, String>,

    /// Flag indicating unsaved changes (runtime state, not persisted)
    #[serde(skip)]
    pub is_dirty: bool,

    /// Flag indicating zoom_to_fit should be called after next render with actual viewport dimensions.
    /// Set to true when loading a file, cleared after the fit is performed.
    #[serde(skip)]
    pub needs_fit: bool,

    /// Flag indicating the undo history should be reset (e.g., after loading a file).
    /// Set to true when a file is loaded, cleared after history is reset.
    #[serde(skip)]
    pub needs_history_reset: bool,

    /// Topology version counter for cache invalidation (runtime state, not persisted)
    /// Incremented on any structural change (add/remove/move component/wire/junction)
    /// Used by LabelPositionCache and JunctionCache to detect stale data
    #[serde(skip)]
    topology_version: u64,

    /// Snap engine configuration (runtime state, not persisted)
    /// Controls cursor snapping behavior during wire drawing
    #[serde(skip)]
    pub snap_engine: SnapEngine,

    /// Rubber-band box selection rectangle (runtime state, not persisted)
    /// Used for drag-to-select operations
    #[serde(skip)]
    pub selection_rect: super::selection::SelectionRect,

    /// Net highlighting state (runtime state, not persisted)
    /// Tracks which wires are part of the highlighted net
    #[serde(skip)]
    pub net_highlight: super::net_highlight::NetHighlightState,

    /// Undo/redo history (runtime state, not persisted)
    /// Manages snapshots for undo/redo operations
    #[serde(skip)]
    pub undo_history: super::undo_history::UndoHistory,
}

impl Default for SchematicState {
    fn default() -> Self {
        Self {
            components: Vec::new(),
            wires: Vec::new(),
            selection: Selection::default(),
            tool: Tool::default(),
            wire_drawing: WireDrawing::default(),
            grid_size: 10,
            zoom: 1.0,
            pan: (0.0, 0.0),
            current_file: None,
            next_id: 1,
            component_counters: HashMap::new(),
            clipboard: ClipboardData::default(),
            net_labels: Vec::new(),
            junctions: Vec::new(),
            preview_rotation: Rotation::default(),
            pending_library_cell: None,
            connections: Vec::new(),
            net_mapping: HashMap::new(),
            is_dirty: false,
            needs_fit: false,
            needs_history_reset: false,
            topology_version: 0,
            snap_engine: SnapEngine::default(),
            selection_rect: super::selection::SelectionRect::default(),
            net_highlight: super::net_highlight::NetHighlightState::default(),
            undo_history: super::undo_history::UndoHistory::default(),
        }
    }
}

impl SchematicState {
    // =========================================================================
    // ID and Name Generation
    // =========================================================================

    /// Generate a unique ID
    pub fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Get the current topology version
    ///
    /// This version is incremented whenever the schematic topology changes
    /// (components, wires, junctions added/removed/moved). Used by caches
    /// to detect when they need to be rebuilt.
    pub fn topology_version(&self) -> u64 {
        self.topology_version
    }

    /// Increment the topology version
    ///
    /// Call this after any structural change to invalidate caches.
    /// This is automatically called by mutation methods like add_component,
    /// add_wire, move_component_with_wires, etc.
    pub fn bump_topology_version(&mut self) {
        self.topology_version = self.topology_version.wrapping_add(1);
    }

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

        let snapshot = super::undo_history::SchematicSnapshot::capture(self);
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
        let snapshot = super::undo_history::SchematicSnapshot::capture(self);
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
        self.begin_operation(description);
        operation(self);
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
        let current = super::undo_history::SchematicSnapshot::capture(self);

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
        let current = super::undo_history::SchematicSnapshot::capture(self);

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

    /// Recalculate runtime state after loading from file
    /// This MUST be called after deserialization to prevent ID collisions
    pub fn recalculate_runtime_state(&mut self) {
        // Find the maximum ID currently in use (components, wires, and junctions)
        let max_component_id = self.components.iter().map(|c| c.id).max().unwrap_or(0);
        let max_wire_id = self.wires.iter().map(|w| w.id).max().unwrap_or(0);
        let max_junction_id = self.junctions.iter().map(|j| j.id).max().unwrap_or(0);
        self.next_id = max_component_id.max(max_wire_id).max(max_junction_id) + 1;

        // Rebuild component counters from existing component names
        self.component_counters.clear();
        for comp in &self.components {
            let prefix = comp.kind.spice_prefix();
            if !prefix.is_empty() {
                // Extract number from name like "R1", "C5", etc.
                if let Some(num_str) = comp.name.strip_prefix(prefix) {
                    if let Ok(num) = num_str.parse::<u32>() {
                        let counter = self.component_counters.entry(prefix).or_insert(0);
                        *counter = (*counter).max(num);
                    }
                }
            }
        }
    }

    /// Generate a unique component name
    pub fn generate_name(&mut self, kind: ComponentType) -> String {
        let prefix = kind.spice_prefix();
        if prefix.is_empty() {
            return String::new();
        }
        let counter = self.component_counters.entry(prefix).or_insert(0);
        *counter += 1;
        format!("{}{}", prefix, counter)
    }

    // =========================================================================
    // Viewport Management
    // =========================================================================

    /// Zoom to fit all schematic content in the viewport.
    ///
    /// Sets zoom and pan so all components and wires are visible with comfortable margins.
    ///
    /// Parameters:
    /// - `viewport_width`: Width of the schematic canvas in pixels
    /// - `viewport_height`: Height of the schematic canvas in pixels
    pub fn zoom_to_fit(&mut self, viewport_width: f64, viewport_height: f64) {
        // Calculate bounding box of all content (in schematic pixel coordinates)
        let Some((min_x, min_y, max_x, max_y)) = self.content_bounds() else {
            // No content - reset to default view
            self.zoom = 1.0;
            self.pan = (0.0, 0.0);
            return;
        };

        // content_bounds returns schematic pixel coordinates, not grid cell indices
        // So we use them directly without multiplying by grid_size
        let min_px = min_x as f64;
        let min_py = min_y as f64;
        let max_px = max_x as f64;
        let max_py = max_y as f64;

        // Content size in schematic pixels
        let content_width = (max_px - min_px).max(1.0);
        let content_height = (max_py - min_py).max(1.0);

        // Add margin (10% of content size, minimum 50 pixels) for a comfortable fit
        let margin = (content_width.max(content_height) * 0.10).max(50.0);

        let total_width = content_width + margin * 2.0;
        let total_height = content_height + margin * 2.0;

        // Calculate zoom to fit (use the smaller scale to ensure everything fits)
        let zoom_x = viewport_width / total_width;
        let zoom_y = viewport_height / total_height;
        let fit_zoom = zoom_x.min(zoom_y);

        // Clamp zoom to reasonable bounds (0.25x to 4x)
        self.zoom = fit_zoom.clamp(0.25, 4.0);

        // Calculate pan to center the content in the viewport
        // Screen position formula: screen = bounds.min + pan + schematic * zoom
        // We want the center of content to appear at center of viewport:
        // viewport_width/2 = pan + center_schematic * zoom
        // pan = viewport_width/2 - center_schematic * zoom
        let center_schematic_x = (min_px + max_px) / 2.0;
        let center_schematic_y = (min_py + max_py) / 2.0;

        self.pan = (
            viewport_width / 2.0 - center_schematic_x * self.zoom,
            viewport_height / 2.0 - center_schematic_y * self.zoom,
        );

        log::debug!(
            "zoom_to_fit: content=[{:.0},{:.0}]-[{:.0},{:.0}], viewport={:.0}x{:.0}, zoom={:.2}, pan=({:.0},{:.0})",
            min_px, min_py, max_px, max_py, viewport_width, viewport_height, self.zoom, self.pan.0, self.pan.1
        );
    }

    /// Calculate the bounding box of all schematic content.
    /// Returns (min_x, min_y, max_x, max_y) in schematic pixel coordinates, or None if empty.
    /// Note: These are pixel coordinates snapped to grid, not grid cell indices.
    pub fn content_bounds(&self) -> Option<(i32, i32, i32, i32)> {
        if self.components.is_empty() && self.wires.is_empty() && self.junctions.is_empty() {
            return None;
        }

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        // Include component bounds (with approximate size for the symbol)
        for comp in &self.components {
            let (comp_min_x, comp_min_y, comp_max_x, comp_max_y) = comp.bounding_box();
            min_x = min_x.min(comp_min_x);
            min_y = min_y.min(comp_min_y);
            max_x = max_x.max(comp_max_x);
            max_y = max_y.max(comp_max_y);
        }

        // Include wire endpoints
        for wire in &self.wires {
            for point in &wire.points {
                min_x = min_x.min(point.x);
                min_y = min_y.min(point.y);
                max_x = max_x.max(point.x);
                max_y = max_y.max(point.y);
            }
        }

        // Include junctions
        for junction in &self.junctions {
            min_x = min_x.min(junction.pos.x);
            min_y = min_y.min(junction.pos.y);
            max_x = max_x.max(junction.pos.x);
            max_y = max_y.max(junction.pos.y);
        }

        Some((min_x, min_y, max_x, max_y))
    }

    // =========================================================================
    // Component Management
    // =========================================================================

    /// Add a component at the given position
    pub fn add_component(&mut self, kind: ComponentType, pos: Point) -> u64 {
        let id = self.next_id();
        let name = self.generate_name(kind);
        let mut component = Component::new(id, kind, pos);
        component.name = name;
        component.rotation = self.preview_rotation;

        // Set default values
        component.value = kind.default_value().to_string();

        self.components.push(component);
        self.is_dirty = true;
        self.bump_topology_version();
        id
    }

    /// Add a generic library/cell/view instance at the given position.
    pub fn add_library_cell_component(
        &mut self,
        pos: Point,
        library_cell: LibraryCellInstance,
    ) -> u64 {
        let id = self.next_id();
        let name = self.generate_name(ComponentType::CellInstance);
        let mut component = Component::new(id, ComponentType::CellInstance, pos);
        component.name = name;
        component.rotation = self.preview_rotation;
        component.value = library_cell.cell.clone();
        component.library_cell = Some(library_cell);

        self.components.push(component);
        self.is_dirty = true;
        self.bump_topology_version();
        id
    }

    /// Find component at grid position
    pub fn component_at(&self, pos: Point) -> Option<u64> {
        // Check terminals first (precise connection points)
        for comp in &self.components {
            let terminals = comp.terminal_positions();
            for (_, term_pos) in terminals {
                if term_pos == pos {
                    return Some(comp.id);
                }
            }
        }
        // Then check component bounding boxes (uses symbol_dimensions for accurate hit detection)
        for comp in &self.components {
            if comp.contains_point(pos) {
                return Some(comp.id);
            }
        }
        None
    }

    /// Rotate selected components
    pub fn rotate_selection(&mut self) {
        for id in &self.selection.components {
            if let Some(c) = self.components.iter_mut().find(|c| c.id == *id) {
                c.rotation = c.rotation.rotate_cw();
            }
        }
        self.is_dirty = true;
        self.bump_topology_version();
    }

    /// Mirror selected components horizontally (flip about Y-axis)
    ///
    /// This flips components left-to-right, swapping terminal positions.
    /// Essential for proper transistor orientation in circuit design.
    /// Matches Cadence Virtuoso 'H' key behavior.
    pub fn mirror_selection_h(&mut self) {
        for id in &self.selection.components {
            if let Some(c) = self.components.iter_mut().find(|c| c.id == *id) {
                c.toggle_mirror_h();
            }
        }
        self.is_dirty = true;
        self.bump_topology_version();
    }

    /// Mirror selected components vertically (flip about X-axis)
    ///
    /// This flips components up-to-down, swapping terminal positions.
    /// Matches Cadence Virtuoso 'V' key behavior.
    pub fn mirror_selection_v(&mut self) {
        for id in &self.selection.components {
            if let Some(c) = self.components.iter_mut().find(|c| c.id == *id) {
                c.toggle_mirror_v();
            }
        }
        self.is_dirty = true;
        self.bump_topology_version();
    }

    /// Select all components and wires within a rectangular region
    ///
    /// This is used for rubber-band box selection. The user drags to create
    /// a selection rectangle, and all items within the rectangle are selected.
    ///
    /// # Arguments
    /// * `min_x`, `min_y`, `max_x`, `max_y` - The selection rectangle bounds (in grid coordinates)
    /// * `add_to_selection` - If true, add to existing selection; if false, replace selection
    ///
    /// # Returns
    /// The number of items selected
    pub fn select_in_rect(
        &mut self,
        min_x: i32,
        min_y: i32,
        max_x: i32,
        max_y: i32,
        add_to_selection: bool,
    ) -> usize {
        if !add_to_selection {
            self.selection.clear();
        }

        let mut count = 0;

        // Select components whose center is within the rectangle
        for comp in &self.components {
            if comp.pos.x >= min_x
                && comp.pos.x <= max_x
                && comp.pos.y >= min_y
                && comp.pos.y <= max_y
                && !self.selection.has_component(comp.id)
            {
                self.selection.select_component(comp.id);
                count += 1;
            }
        }

        // Select wires that have at least one point within the rectangle
        for wire in &self.wires {
            let wire_in_rect = wire
                .points
                .iter()
                .any(|p| p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y);
            if wire_in_rect && !self.selection.has_wire(wire.id) {
                self.selection.select_wire(wire.id);
                count += 1;
            }
        }

        // Select junctions within the rectangle
        for junction in &self.junctions {
            if junction.pos.x >= min_x
                && junction.pos.x <= max_x
                && junction.pos.y >= min_y
                && junction.pos.y <= max_y
                && !self.selection.has_junction(junction.pos)
            {
                self.selection.select_junction(junction.pos);
                count += 1;
            }
        }

        count
    }

    /// Preview selection in rectangle during drag (live highlight feedback)
    ///
    /// This updates the selection to show what would be selected when the drag
    /// is released. It replaces the current selection with items in the rect.
    pub fn preview_selection_in_rect(&mut self, min_x: i32, min_y: i32, max_x: i32, max_y: i32) {
        // Clear and rebuild selection based on current rect
        self.selection.clear();

        // Select components whose center is within the rectangle
        for comp in &self.components {
            if comp.pos.x >= min_x
                && comp.pos.x <= max_x
                && comp.pos.y >= min_y
                && comp.pos.y <= max_y
            {
                self.selection.select_component(comp.id);
            }
        }

        // Select wires that have at least one point within the rectangle
        for wire in &self.wires {
            let wire_in_rect = wire
                .points
                .iter()
                .any(|p| p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y);
            if wire_in_rect {
                self.selection.select_wire(wire.id);
            }
        }

        // Select junctions within the rectangle
        for junction in &self.junctions {
            if junction.pos.x >= min_x
                && junction.pos.x <= max_x
                && junction.pos.y >= min_y
                && junction.pos.y <= max_y
            {
                self.selection.select_junction(junction.pos);
            }
        }
    }

    /// Remove selected components and wires
    pub fn delete_selection(&mut self) {
        self.components
            .retain(|c| !self.selection.has_component(c.id));
        self.wires.retain(|w| !self.selection.has_wire(w.id));
        self.selection.clear();
        self.is_dirty = true;
        self.bump_topology_version();
    }
}
