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
use super::snap::{SnapEngine, SnapResult};
use super::tool::Tool;
use super::wire::{Wire, WireConnection, WireDrawing, WireSegment};

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
        let bounds = self.content_bounds();

        if bounds.is_none() {
            // No content - reset to default view
            self.zoom = 1.0;
            self.pan = (0.0, 0.0);
            return;
        }

        let (min_x, min_y, max_x, max_y) = bounds.unwrap();

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
            let half_w = 30;
            let half_h = 20;
            min_x = min_x.min(comp.pos.x - half_w);
            min_y = min_y.min(comp.pos.y - half_h);
            max_x = max_x.max(comp.pos.x + half_w);
            max_y = max_y.max(comp.pos.y + half_h);
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
            {
                if !self.selection.has_component(comp.id) {
                    self.selection.select_component(comp.id);
                    count += 1;
                }
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
            {
                if !self.selection.has_junction(junction.pos) {
                    self.selection.select_junction(junction.pos);
                    count += 1;
                }
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

    // =========================================================================
    // Wire Management
    // =========================================================================

    /// Add a wire
    pub fn add_wire(&mut self, points: Vec<Point>) -> Option<u64> {
        if points.len() < 2 {
            return None;
        }
        let id = self.next_id();
        self.wires.push(Wire::new(id, points));
        self.is_dirty = true;
        self.bump_topology_version();
        Some(id)
    }

    /// Find wire at grid position
    pub fn wire_at(&self, pos: Point) -> Option<u64> {
        for wire in &self.wires {
            if wire.contains_point(pos) {
                return Some(wire.id);
            }
        }
        None
    }

    /// Find all wire points at a grid position
    /// Returns (wire_id, point_index) pairs for junction detection
    pub fn wire_points_at(&self, pos: Point) -> Vec<(u64, usize)> {
        let mut result = Vec::new();
        for wire in &self.wires {
            for (idx, point) in wire.points.iter().enumerate() {
                if *point == pos {
                    result.push((wire.id, idx));
                }
            }
        }
        result
    }

    /// Find all wire ENDPOINTS at a grid position
    /// Unlike wire_points_at, this only returns first/last points of wires
    pub fn wire_endpoints_at(&self, pos: Point) -> Vec<(u64, usize)> {
        let mut result = Vec::new();
        for wire in &self.wires {
            if let Some(first) = wire.points.first() {
                if *first == pos {
                    result.push((wire.id, 0));
                }
            }
            if wire.points.len() > 1 {
                if let Some(last) = wire.points.last() {
                    if *last == pos {
                        result.push((wire.id, wire.points.len() - 1));
                    }
                }
            }
        }
        result
    }

    /// Find wire vertex at a grid position for dragging
    ///
    /// Returns (wire_id, vertex_index) if there's a wire vertex at this position.
    /// This is used for wire corner dragging - a professional EDA feature.
    pub fn wire_vertex_at(&self, pos: Point) -> Option<(u64, usize)> {
        for wire in &self.wires {
            for (idx, point) in wire.points.iter().enumerate() {
                if *point == pos {
                    return Some((wire.id, idx));
                }
            }
        }
        None
    }

    /// Check if a position is a draggable wire point
    ///
    /// Returns true if there's either:
    /// - A wire vertex at this position
    /// - A junction marker at this position
    /// - A wire segment that passes through this position
    ///
    /// This is used for detecting draggable points to enable T-junction dragging.
    pub fn is_draggable_wire_point(&self, pos: Point) -> bool {
        // Check for wire vertices
        if self.wire_vertex_at(pos).is_some() {
            return true;
        }

        // Check for junction markers
        if self.junctions.iter().any(|j| j.pos == pos) {
            return true;
        }

        false
    }

    /// Start drawing a wire at position
    pub fn start_wire(&mut self, pos: Point) {
        log::info!("[Wire] start_wire at {:?}", pos);
        self.wire_drawing.clear();
        self.wire_drawing.points.push(pos);
        self.wire_drawing.active = true;
    }

    /// Update the wire preview position (called on mouse move)
    pub fn update_wire_preview(&mut self, pos: Point) {
        if self.wire_drawing.active {
            self.wire_drawing.preview_pos = Some(pos);
        }
    }

    /// Toggle wire routing mode (horizontal-first vs vertical-first)
    pub fn toggle_wire_routing(&mut self) {
        self.wire_drawing.routing_mode = self.wire_drawing.routing_mode.toggle();
    }

    /// Add a point to the current wire using orthogonal routing
    pub fn extend_wire(&mut self, pos: Point) {
        if !self.wire_drawing.active {
            return;
        }

        if let Some(last) = self.wire_drawing.points.last().copied() {
            if last == pos {
                return; // Same point, skip
            }

            // Add corner point for orthogonal routing if needed
            if let Some(corner) = self.wire_drawing.get_route_corner(pos) {
                if corner != last && corner != pos {
                    self.wire_drawing.points.push(corner);
                }
            }

            self.wire_drawing.points.push(pos);
        }
    }

    /// Simplify wire path by removing intermediate points on straight segments
    pub(crate) fn simplify_wire_path(points: Vec<Point>) -> Vec<Point> {
        if points.len() <= 2 {
            return points;
        }

        let mut result = Vec::with_capacity(points.len());
        result.push(points[0]);

        for i in 1..points.len() - 1 {
            let prev = &points[i - 1];
            let curr = &points[i];
            let next = &points[i + 1];

            let all_same_x = prev.x == curr.x && curr.x == next.x;
            let all_same_y = prev.y == curr.y && curr.y == next.y;

            if !all_same_x && !all_same_y {
                result.push(*curr);
            }
        }

        result.push(*points.last().unwrap());
        result
    }

    /// Finish drawing the current wire
    ///
    /// Implements professional EDA behavior:
    /// - When a wire endpoint lands on another wire mid-segment, the other wire
    ///   is automatically split at that point (creating a proper vertex)
    /// - This ensures correct rubber-banding: all wires at a T-junction share
    ///   a common endpoint vertex, so moving any wire keeps the junction intact
    pub fn finish_wire(&mut self) -> Option<u64> {
        if !self.wire_drawing.active {
            return None;
        }

        self.wire_drawing.active = false;
        self.wire_drawing.preview_pos = None;

        let points = std::mem::take(&mut self.wire_drawing.points);
        let simplified = Self::simplify_wire_path(points);

        if simplified.len() < 2 {
            return None;
        }

        // Split the path into individual 2-point wire segments
        let mut last_wire_id = None;
        let mut endpoints_to_check = Vec::new();

        for i in 0..simplified.len() - 1 {
            let segment = vec![simplified[i], simplified[i + 1]];
            endpoints_to_check.push(simplified[i]);
            if i == simplified.len() - 2 {
                endpoints_to_check.push(simplified[i + 1]);
            }
            if let Some(wire_id) = self.add_wire(segment) {
                last_wire_id = Some(wire_id);
            }
        }

        // Professional EDA behavior: split existing wires at T-junction points
        // This ensures all wires at a junction share a common endpoint vertex
        for pt in &endpoints_to_check {
            self.split_wires_at_t_junction(*pt);
        }

        // Add junction markers where 3+ wire endpoints meet
        self.update_wire_junctions();

        last_wire_id
    }

    /// Split all wires that pass through a point mid-segment (T-junction creation)
    ///
    /// When a wire endpoint lands on another wire's mid-segment, we split the
    /// through wire at that point. This implements professional EDA behavior
    /// where T-junctions are formed by splitting wires, not just by visual overlap.
    ///
    /// This ensures correct rubber-banding: since all wires at the junction
    /// share the same endpoint vertex, moving any attached wire keeps the
    /// junction topology intact.
    pub fn split_wires_at_t_junction(&mut self, point: Point) {
        // Find wires that pass through this point but don't have it as a vertex
        let wires_to_split: Vec<u64> = self
            .wires
            .iter()
            .filter(|w| {
                // Wire passes through point mid-segment (not at a vertex)
                w.contains_point(point) && !w.points.contains(&point)
            })
            .map(|w| w.id)
            .collect();

        // Split each wire at the junction point
        for wire_id in wires_to_split {
            let _ = self.split_wire(wire_id, point);
        }
    }

    /// Cancel wire drawing
    pub fn cancel_wire(&mut self) {
        self.wire_drawing.clear();
    }

    // =========================================================================
    // Wire Operations (Commercial-Grade)
    // =========================================================================

    /// Split a wire into two wires at the given point
    ///
    /// If the point is exactly on the wire (either at a vertex or on a segment),
    /// this will create two new wires: one from the original start to the split point,
    /// and one from the split point to the original end.
    ///
    /// Returns `Some((wire_before_id, wire_after_id))` if successful, `None` otherwise.
    ///
    /// # Arguments
    /// * `wire_id` - The ID of the wire to split
    /// * `at_point` - The point at which to split (must be on the wire)
    pub fn split_wire(&mut self, wire_id: u64, at_point: Point) -> Option<(u64, u64)> {
        // Find the wire
        let wire_idx = self.wires.iter().position(|w| w.id == wire_id)?;
        let wire = &self.wires[wire_idx];

        // Validate that the point is on the wire
        if !wire.contains_point(at_point) {
            return None;
        }

        // Don't split at endpoints - nothing to split
        if wire.start() == Some(at_point) || wire.end() == Some(at_point) {
            return None;
        }

        // Find where to split
        let points = wire.points.clone();

        // Check if split point is at an existing vertex
        let vertex_idx = points.iter().position(|p| *p == at_point);

        let (before_points, after_points) = if let Some(v_idx) = vertex_idx {
            // Split at vertex - both wires share this point
            let before: Vec<Point> = points[..=v_idx].to_vec();
            let after: Vec<Point> = points[v_idx..].to_vec();
            (before, after)
        } else {
            // Point is on a segment, need to find which one and insert it
            let mut before_points = Vec::new();
            let mut after_points = Vec::new();
            let mut found_segment = false;

            for i in 0..points.len() - 1 {
                let seg = WireSegment::new(points[i], points[i + 1]);
                if !found_segment {
                    before_points.push(points[i]);
                    if seg.contains_point(at_point) && points[i] != at_point {
                        before_points.push(at_point);
                        after_points.push(at_point);
                        found_segment = true;
                    }
                }
                if found_segment {
                    after_points.push(points[i + 1]);
                }
            }

            if !found_segment {
                return None;
            }

            (before_points, after_points)
        };

        // Validate both parts are valid wires
        if before_points.len() < 2 || after_points.len() < 2 {
            return None;
        }

        // Remove original wire
        self.wires.remove(wire_idx);

        // Create two new wires
        let id1 = self.next_id();
        let id2 = self.next_id();

        self.wires.push(Wire::new(id1, before_points));
        self.wires.push(Wire::new(id2, after_points));

        self.is_dirty = true;
        self.bump_topology_version();

        Some((id1, id2))
    }

    /// Split a wire at a specific segment, inserting a corner point at the midpoint
    ///
    /// This is useful for creating corners in straight wire runs.
    ///
    /// # Arguments
    /// * `wire_id` - The wire to modify
    /// * `segment_index` - Which segment to split (0 = first segment)
    ///
    /// Returns the modified wire ID if successful
    pub fn split_wire_at_segment(&mut self, wire_id: u64, segment_index: usize) -> Option<u64> {
        let wire = self.wires.iter_mut().find(|w| w.id == wire_id)?;

        if segment_index >= wire.segment_count() {
            return None;
        }

        let segment = wire.segment_at(segment_index)?;
        let midpoint = segment.midpoint();

        // Don't insert if midpoint equals an endpoint (zero-length segment)
        if midpoint == segment.start || midpoint == segment.end {
            return None;
        }

        // Insert the midpoint as a new vertex
        wire.points.insert(segment_index + 1, midpoint);

        self.is_dirty = true;
        self.bump_topology_version();

        Some(wire_id)
    }

    /// Merge two wires that share an endpoint
    ///
    /// The wires must be connected at exactly one endpoint. After merging,
    /// the first wire is removed and the second wire is modified to include
    /// all points from both.
    ///
    /// # Arguments
    /// * `wire_a` - First wire ID
    /// * `wire_b` - Second wire ID
    ///
    /// Returns the ID of the merged wire if successful
    pub fn merge_wires(&mut self, wire_a: u64, wire_b: u64) -> Option<u64> {
        if wire_a == wire_b {
            return None;
        }

        // Find both wires
        let idx_a = self.wires.iter().position(|w| w.id == wire_a)?;
        let idx_b = self.wires.iter().position(|w| w.id == wire_b)?;

        // Check if they share an endpoint
        let (a_start, a_end) = (self.wires[idx_a].start()?, self.wires[idx_a].end()?);
        let (b_start, b_end) = (self.wires[idx_b].start()?, self.wires[idx_b].end()?);

        // Determine connection type and build merged points
        let merged_points: Vec<Point> = if a_end == b_start {
            // A's end connects to B's start: A.start → A.end/B.start → B.end
            let mut pts = self.wires[idx_a].points.clone();
            pts.extend(self.wires[idx_b].points.iter().skip(1));
            pts
        } else if a_end == b_end {
            // A's end connects to B's end: A.start → A.end/B.end → B.start
            let mut pts = self.wires[idx_a].points.clone();
            pts.extend(self.wires[idx_b].points.iter().rev().skip(1));
            pts
        } else if a_start == b_end {
            // B's end connects to A's start: B.start → B.end/A.start → A.end
            let mut pts = self.wires[idx_b].points.clone();
            pts.extend(self.wires[idx_a].points.iter().skip(1));
            pts
        } else if a_start == b_start {
            // A's start connects to B's start: A.end ← A.start/B.start → B.end
            let mut pts: Vec<Point> = self.wires[idx_a].points.iter().rev().cloned().collect();
            pts.extend(self.wires[idx_b].points.iter().skip(1));
            pts
        } else {
            // Wires don't share an endpoint
            return None;
        };

        // Remove both wires (higher index first to avoid shifting)
        let (remove_first, remove_second) = if idx_a > idx_b {
            (idx_a, idx_b)
        } else {
            (idx_b, idx_a)
        };
        self.wires.remove(remove_first);
        self.wires.remove(remove_second);

        // Create merged wire
        let merged_id = self.next_id();
        self.wires.push(Wire::new(merged_id, merged_points));

        self.is_dirty = true;
        self.bump_topology_version();

        Some(merged_id)
    }

    /// Remove unnecessary intermediate vertices from a wire
    ///
    /// This removes collinear points (points that lie on a straight line
    /// between their neighbors) to simplify the wire path.
    ///
    /// # Arguments
    /// * `wire_id` - The wire to straighten
    pub fn straighten_wire(&mut self, wire_id: u64) {
        if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
            let simplified = Self::simplify_wire_path(wire.points.clone());
            if simplified != wire.points {
                wire.points = simplified;
                self.is_dirty = true;
                self.bump_topology_version();
            }
        }
    }

    /// Optimize all wires by removing collinear intermediate points
    pub fn optimize_all_wires(&mut self) {
        let mut changed = false;
        for wire in &mut self.wires {
            let simplified = Self::simplify_wire_path(wire.points.clone());
            if simplified != wire.points {
                wire.points = simplified;
                changed = true;
            }
        }
        if changed {
            self.is_dirty = true;
            self.bump_topology_version();
        }
    }

    /// Remove degenerate segments from all wires
    ///
    /// This is a cleanup operation that removes:
    /// 1. Zero-length segments (consecutive identical points)
    /// 2. Wires that become invalid after cleanup (< 2 points)
    ///
    /// This is called automatically after wire editing operations to ensure
    /// the schematic maintains valid topology. Matches Cadence Virtuoso behavior.
    ///
    /// # Returns
    /// A tuple of (wires_modified, wires_removed) counts
    pub fn remove_degenerate_segments(&mut self) -> (usize, usize) {
        let mut wires_modified = 0;
        let initial_wire_count = self.wires.len();

        // Phase 1: Remove zero-length segments from each wire
        for wire in &mut self.wires {
            let original_len = wire.points.len();

            // Remove consecutive duplicate points (zero-length segments)
            let mut cleaned = Vec::with_capacity(wire.points.len());
            for point in &wire.points {
                if cleaned.last() != Some(point) {
                    cleaned.push(*point);
                }
            }

            if cleaned.len() != original_len {
                wire.points = cleaned;
                wires_modified += 1;
            }
        }

        // Phase 2: Remove wires that are now invalid (< 2 points)
        let wires_to_remove: Vec<u64> = self
            .wires
            .iter()
            .filter(|w| w.points.len() < 2)
            .map(|w| w.id)
            .collect();

        for wire_id in &wires_to_remove {
            log::info!("Removing zero-length wire id={}", wire_id);
        }

        self.wires.retain(|w| w.points.len() >= 2);

        let wires_removed = initial_wire_count - self.wires.len();

        if wires_modified > 0 || wires_removed > 0 {
            self.is_dirty = true;
            self.bump_topology_version();
        }

        (wires_modified, wires_removed)
    }

    /// Remove degenerate segments from a specific wire
    ///
    /// Returns true if the wire was modified, false if unchanged or not found.
    /// If the wire becomes invalid (< 2 points), it is removed entirely.
    pub fn remove_degenerate_segments_for_wire(&mut self, wire_id: u64) -> bool {
        let wire_idx = match self.wires.iter().position(|w| w.id == wire_id) {
            Some(idx) => idx,
            None => return false,
        };

        let wire = &mut self.wires[wire_idx];
        let original_len = wire.points.len();

        // Remove consecutive duplicate points
        let mut cleaned = Vec::with_capacity(wire.points.len());
        for point in &wire.points {
            if cleaned.last() != Some(point) {
                cleaned.push(*point);
            }
        }

        let was_modified = cleaned.len() != original_len;

        if cleaned.len() < 2 {
            // Wire is now invalid, remove it
            self.wires.remove(wire_idx);
            self.is_dirty = true;
            self.bump_topology_version();
            return true;
        }

        if was_modified {
            self.wires[wire_idx].points = cleaned;
            self.is_dirty = true;
            self.bump_topology_version();
        }

        was_modified
    }

    /// Clean up wire topology after editing operations
    ///
    /// This comprehensive cleanup method should be called after bulk editing:
    /// 1. Removes degenerate (zero-length) segments
    /// 2. Optimizes wire paths (removes collinear points)
    /// 3. Updates junction markers
    ///
    /// This matches commercial EDA tool behavior for maintaining clean topology.
    pub fn cleanup_wire_topology(&mut self) {
        self.remove_degenerate_segments();
        self.optimize_all_wires();
        self.update_wire_junctions();
    }

    /// Delete a wire by ID
    ///
    /// Returns true if a wire was deleted
    pub fn delete_wire(&mut self, wire_id: u64) -> bool {
        let len_before = self.wires.len();
        self.wires.retain(|w| w.id != wire_id);
        let deleted = self.wires.len() < len_before;
        if deleted {
            self.is_dirty = true;
            self.bump_topology_version();
        }
        deleted
    }

    /// Insert a corner vertex into an existing wire at a specific location
    ///
    /// # Arguments
    /// * `wire_id` - The wire to modify
    /// * `at_point` - The point on the wire where to insert the corner
    /// * `corner_offset` - The offset to move the new corner point
    ///
    /// Returns true if successful
    pub fn insert_wire_corner(
        &mut self,
        wire_id: u64,
        at_point: Point,
        corner_offset: Point,
    ) -> bool {
        let wire = match self.wires.iter_mut().find(|w| w.id == wire_id) {
            Some(w) => w,
            None => return false,
        };

        // Find the segment containing the point
        if let Some((seg_idx, _)) = wire.segment_containing_point(at_point) {
            // Insert two new vertices: the original point and the offset corner
            let new_corner = Point::new(at_point.x + corner_offset.x, at_point.y + corner_offset.y);

            // Insert after the segment start
            wire.points.insert(seg_idx + 1, at_point);
            wire.points.insert(seg_idx + 2, new_corner);

            self.is_dirty = true;
            self.bump_topology_version();
            return true;
        }

        false
    }

    /// Move a specific vertex of a wire
    ///
    /// # Arguments
    /// * `wire_id` - The wire to modify
    /// * `vertex_index` - Which vertex to move
    /// * `new_pos` - The new position
    ///
    /// Returns true if successful
    pub fn move_wire_vertex(&mut self, wire_id: u64, vertex_index: usize, new_pos: Point) -> bool {
        if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
            if wire.move_vertex(vertex_index, new_pos) {
                self.is_dirty = true;
                self.bump_topology_version();
                return true;
            }
        }
        false
    }

    /// Move ALL wire vertices at a given position to a new position
    ///
    /// This is the professional EDA behavior for junction/corner dragging:
    /// when you drag a point where multiple wires meet, all of them move together.
    ///
    /// For T-junctions where a wire passes through without a vertex, we first
    /// split that wire at the junction point so it can move with the others.
    ///
    /// # Arguments
    /// * `old_pos` - The current position of the vertices to move
    /// * `new_pos` - The new position
    ///
    /// Returns true if any vertices were moved
    pub fn move_all_vertices_at(&mut self, old_pos: Point, new_pos: Point) -> bool {
        if old_pos == new_pos {
            return false;
        }

        // First, check if this is a junction point where wires might pass through
        // without having a vertex. If so, split those wires first.
        let is_junction = self.junctions.iter().any(|j| j.pos == old_pos);
        if is_junction {
            // Split any wires that pass through this junction point but don't have a vertex there
            self.split_wires_at_t_junction(old_pos);
        }

        let mut moved = false;

        // Move all wire vertices at this position
        for wire in &mut self.wires {
            for point in &mut wire.points {
                if *point == old_pos {
                    *point = new_pos;
                    moved = true;
                }
            }
        }

        // Also move any junction at this position
        for junction in &mut self.junctions {
            if junction.pos == old_pos {
                junction.pos = new_pos;
            }
        }

        if moved {
            self.is_dirty = true;
            self.bump_topology_version();
        }

        moved
    }

    // =========================================================================
    // Junction Management
    // =========================================================================

    /// Add an explicit junction at a position
    pub fn add_junction(&mut self, pos: Point) -> u64 {
        // Check if junction already exists at this position
        if let Some(existing) = self.junctions.iter().find(|j| j.pos == pos) {
            return existing.id;
        }

        let id = self.next_id();
        self.junctions.push(Junction::new(id, pos));
        self.is_dirty = true;
        self.bump_topology_version();
        id
    }

    /// Remove a junction by ID
    pub fn remove_junction(&mut self, id: u64) -> bool {
        let len_before = self.junctions.len();
        self.junctions.retain(|j| j.id != id);
        let removed = self.junctions.len() < len_before;
        if removed {
            self.is_dirty = true;
            self.bump_topology_version();
        }
        removed
    }

    /// Find junction at a position
    pub fn junction_at(&self, pos: Point) -> Option<u64> {
        self.junctions.iter().find(|j| j.pos == pos).map(|j| j.id)
    }

    /// Check if a junction exists at a position
    pub fn has_junction(&self, pos: Point) -> bool {
        self.junctions.iter().any(|j| j.pos == pos)
    }

    /// Add a net label at the given position
    pub fn add_net_label(&mut self, pos: Point, name: String) -> u64 {
        let id = self.next_id();
        self.net_labels.push(NetLabel::new(id, pos, name));
        self.is_dirty = true;
        id
    }

    // =========================================================================
    // Clipboard Operations
    // =========================================================================

    /// Copy selected components and wires to clipboard
    ///
    /// In addition to explicitly selected wires, automatically includes
    /// any wires that have both endpoints connected to selected components.
    /// This preserves circuit connectivity when copying/pasting.
    pub fn copy_selection(&mut self) {
        if self.selection.is_empty() {
            return;
        }

        let selected_comps: Vec<Component> = self
            .components
            .iter()
            .filter(|c| self.selection.has_component(c.id))
            .cloned()
            .collect();

        // Get all terminal positions for selected components
        let selected_terminals: Vec<Point> = selected_comps
            .iter()
            .flat_map(|c| c.terminal_positions().into_iter().map(|(_, pos)| pos))
            .collect();

        // Find wires that have both endpoints at selected component terminals
        let mut wires_to_copy: Vec<Wire> = Vec::new();

        for wire in &self.wires {
            // Check if explicitly selected
            if self.selection.has_wire(wire.id) {
                wires_to_copy.push(wire.clone());
                continue;
            }

            // Check if both endpoints connect to selected components
            if wire.points.len() >= 2 {
                let start = wire.points[0];
                let end = *wire.points.last().unwrap();

                let start_connected = selected_terminals.contains(&start);
                let end_connected = selected_terminals.contains(&end);

                if start_connected && end_connected {
                    wires_to_copy.push(wire.clone());
                }
            }
        }

        self.clipboard = ClipboardData::from_selection(selected_comps, wires_to_copy);
    }

    /// Check if clipboard has content
    pub fn can_paste(&self) -> bool {
        self.clipboard.has_content()
    }

    /// Paste clipboard contents at the given position
    pub fn paste_at(&mut self, pos: Point) {
        if !self.can_paste() {
            return;
        }

        let clipboard_components = self.clipboard.components.clone();
        let clipboard_wires = self.clipboard.wires.clone();
        let origin = self.clipboard.origin;

        let offset_x = pos.x - origin.x;
        let offset_y = pos.y - origin.y;

        self.selection.clear();

        // Paste components with new IDs
        for comp in clipboard_components {
            let new_id = self.next_id();
            let mut new_comp = comp;
            new_comp.id = new_id;
            new_comp.pos.x += offset_x;
            new_comp.pos.y += offset_y;
            new_comp.name = self.generate_name(new_comp.kind);
            self.components.push(new_comp);
            self.selection.select_component(new_id);
        }

        // Paste wires with new IDs
        for wire in clipboard_wires {
            let new_id = self.next_id();
            let new_points: Vec<Point> = wire
                .points
                .iter()
                .map(|p| Point::new(p.x + offset_x, p.y + offset_y))
                .collect();
            self.wires.push(Wire::new(new_id, new_points));
            self.selection.select_wire(new_id);
        }

        self.is_dirty = true;
        self.bump_topology_version();
    }

    // =========================================================================
    // Wire Connection Management (for rubber-banding)
    // =========================================================================

    /// Find a component terminal at or near a grid position
    pub fn find_terminal_at(&self, pos: Point) -> Option<(u64, String, Point)> {
        for comp in &self.components {
            for (term_name, term_pos) in comp.terminal_positions() {
                let dx = (pos.x - term_pos.x).abs();
                let dy = (pos.y - term_pos.y).abs();
                if dx <= SNAP_DISTANCE && dy <= SNAP_DISTANCE {
                    return Some((comp.id, term_name.to_string(), term_pos));
                }
            }
        }
        None
    }

    /// Rebuild all wire connections based on current positions
    pub fn rebuild_connections(&mut self) {
        self.connections.clear();

        let wire_endpoints: Vec<(u64, Point, usize)> = self
            .wires
            .iter()
            .filter(|w| !w.points.is_empty())
            .flat_map(|w| {
                let mut endpoints = vec![(w.id, w.points[0], 0usize)];
                let end_idx = w.points.len() - 1;
                if end_idx > 0 {
                    endpoints.push((w.id, w.points[end_idx], end_idx));
                }
                endpoints
            })
            .collect();

        for (wire_id, pos, point_index) in wire_endpoints {
            if let Some((comp_id, term_name, _)) = self.find_terminal_at(pos) {
                self.connections.push(WireConnection::new(
                    wire_id,
                    point_index,
                    comp_id,
                    term_name,
                ));
            }
        }
    }

    /// Find all connections for a specific component
    pub fn connections_for_component(&self, component_id: u64) -> Vec<&WireConnection> {
        self.connections
            .iter()
            .filter(|c| c.component_id == component_id)
            .collect()
    }

    /// Move a component and update all attached wire endpoints (rubber-banding)
    pub fn move_component_with_wires(&mut self, component_id: u64, delta: Point) {
        // Get the component's terminal positions BEFORE moving
        let terminals: Vec<Point> = {
            if let Some(comp) = self.components.iter().find(|c| c.id == component_id) {
                comp.terminal_positions()
                    .into_iter()
                    .map(|(_, pos)| pos)
                    .collect()
            } else {
                return;
            }
        };

        // Find ALL wire points that are at ANY terminal position
        let mut wire_updates: Vec<(u64, usize, Point)> = Vec::new();

        for wire in &self.wires {
            for (point_idx, point) in wire.points.iter().enumerate() {
                for term_pos in &terminals {
                    if *point == *term_pos {
                        let new_pos = Point::new(term_pos.x + delta.x, term_pos.y + delta.y);
                        wire_updates.push((wire.id, point_idx, new_pos));
                        break;
                    }
                }
            }
        }

        // Move the component
        if let Some(comp) = self.components.iter_mut().find(|c| c.id == component_id) {
            comp.pos.x += delta.x;
            comp.pos.y += delta.y;
        }

        // Apply wire updates
        for (wire_id, point_idx, new_pos) in wire_updates {
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
                if point_idx < wire.points.len() {
                    wire.points[point_idx] = new_pos;
                }
            }
        }

        self.is_dirty = true;
        self.bump_topology_version();
    }

    /// Move all selected components and rubber-band connected wires
    ///
    /// This is the multi-component version of move_component_with_wires.
    /// Wires connected to selected components are stretched to maintain
    /// the connection. Wires that connect two selected components are
    /// moved entirely (not stretched).
    pub fn move_selection_with_rubber_band(&mut self, delta: Point) {
        let selected_components: Vec<u64> = self.selection.components.iter().copied().collect();
        if selected_components.is_empty() && self.selection.wires.is_empty() {
            return;
        }

        // Collect all terminal positions for selected components BEFORE moving
        let mut all_terminals: Vec<(u64, Point)> = Vec::new();
        for comp_id in &selected_components {
            if let Some(comp) = self.components.iter().find(|c| c.id == *comp_id) {
                for (_, pos) in comp.terminal_positions() {
                    all_terminals.push((*comp_id, pos));
                }
            }
        }

        // Find wires that should be stretched (one end connected to selection)
        // vs moved entirely (both ends connected to selection)
        let mut wire_updates: Vec<(u64, usize, Point)> = Vec::new();
        let mut wires_to_move: Vec<u64> = Vec::new();

        for wire in &self.wires {
            let start = wire.points.first().copied();
            let end = wire.points.last().copied();

            // Check if endpoints connect to selected components
            let start_connected = start.map_or(false, |p| {
                all_terminals.iter().any(|(_, term_pos)| *term_pos == p)
            });
            let end_connected = end.map_or(false, |p| {
                all_terminals.iter().any(|(_, term_pos)| *term_pos == p)
            });

            if start_connected && end_connected {
                // Both ends connected to selection - move entire wire
                wires_to_move.push(wire.id);
            } else {
                // Stretch endpoints that are connected
                for (point_idx, point) in wire.points.iter().enumerate() {
                    for (_, term_pos) in &all_terminals {
                        if *point == *term_pos {
                            let new_pos = Point::new(point.x + delta.x, point.y + delta.y);
                            wire_updates.push((wire.id, point_idx, new_pos));
                            break;
                        }
                    }
                }
            }
        }

        // Move selected components
        for comp_id in &selected_components {
            if let Some(comp) = self.components.iter_mut().find(|c| c.id == *comp_id) {
                comp.pos.x += delta.x;
                comp.pos.y += delta.y;
            }
        }

        // Move selected wires (from selection, not from rubber-banding)
        for wire_id in self.selection.wires.iter() {
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == *wire_id) {
                for point in &mut wire.points {
                    point.x += delta.x;
                    point.y += delta.y;
                }
            }
        }

        // Move wires that have both ends connected to selection
        for wire_id in wires_to_move {
            // Skip if already in selection (already moved above)
            if self.selection.wires.contains(&wire_id) {
                continue;
            }
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
                for point in &mut wire.points {
                    point.x += delta.x;
                    point.y += delta.y;
                }
            }
        }

        // Apply stretch updates for partially connected wires
        for (wire_id, point_idx, new_pos) in wire_updates {
            // Skip if wire was already moved entirely
            if self.selection.wires.contains(&wire_id) {
                continue;
            }
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
                if point_idx < wire.points.len() {
                    wire.points[point_idx] = new_pos;
                }
            }
        }

        self.is_dirty = true;
        self.bump_topology_version();
    }

    /// Move all points of a wire by a delta
    pub fn move_wire(&mut self, wire_id: u64, delta: Point) {
        let old_endpoints: Vec<Point> = self
            .wires
            .iter()
            .find(|w| w.id == wire_id)
            .map(|w| {
                let mut eps = Vec::new();
                if let Some(first) = w.points.first() {
                    eps.push(*first);
                }
                if let Some(last) = w.points.last() {
                    eps.push(*last);
                }
                eps
            })
            .unwrap_or_default();

        if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
            for point in &mut wire.points {
                point.x += delta.x;
                point.y += delta.y;
            }
        }

        for old_pt in old_endpoints {
            if let Some(junction) = self.junctions.iter_mut().find(|j| j.pos == old_pt) {
                junction.pos.x += delta.x;
                junction.pos.y += delta.y;
            }
        }

        self.is_dirty = true;
        self.bump_topology_version();
    }

    /// Move all selected components and wires by a delta
    pub fn move_selection(&mut self, delta: Point) {
        let selection = self.selection.clone();

        let selected_wire_ids: std::collections::HashSet<u64> =
            selection.wires.iter().copied().collect();

        // Move all selected components with rubber-banding
        for comp_id in &selection.components {
            let terminals: Vec<Point> = {
                if let Some(comp) = self.components.iter().find(|c| c.id == *comp_id) {
                    comp.terminal_positions()
                        .into_iter()
                        .map(|(_, pos)| pos)
                        .collect()
                } else {
                    continue;
                }
            };

            let mut wire_updates: Vec<(u64, usize, Point)> = Vec::new();
            for wire in &self.wires {
                if selected_wire_ids.contains(&wire.id) {
                    continue;
                }
                for (point_idx, point) in wire.points.iter().enumerate() {
                    for term_pos in &terminals {
                        if *point == *term_pos {
                            let new_pos = Point::new(term_pos.x + delta.x, term_pos.y + delta.y);
                            wire_updates.push((wire.id, point_idx, new_pos));
                            break;
                        }
                    }
                }
            }

            if let Some(comp) = self.components.iter_mut().find(|c| c.id == *comp_id) {
                comp.pos.x += delta.x;
                comp.pos.y += delta.y;
            }

            for (wire_id, point_idx, new_pos) in wire_updates {
                if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
                    if point_idx < wire.points.len() {
                        wire.points[point_idx] = new_pos;
                    }
                }
            }
        }

        // Move all selected wires entirely
        let mut wire_endpoints: Vec<Point> = Vec::new();
        for wire_id in &selection.wires {
            if let Some(wire) = self.wires.iter().find(|w| w.id == *wire_id) {
                if let Some(first) = wire.points.first() {
                    wire_endpoints.push(*first);
                }
                if let Some(last) = wire.points.last() {
                    wire_endpoints.push(*last);
                }
            }
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == *wire_id) {
                for point in &mut wire.points {
                    point.x += delta.x;
                    point.y += delta.y;
                }
            }
        }

        // Move junctions at selected wire endpoints
        for old_pt in wire_endpoints {
            if let Some(junction) = self.junctions.iter_mut().find(|j| j.pos == old_pt) {
                junction.pos.x += delta.x;
                junction.pos.y += delta.y;
            }
        }

        self.is_dirty = true;
        self.bump_topology_version();
    }

    /// Move all wire points at a junction to a new position
    pub fn move_junction(&mut self, old_pos: Point, new_pos: Point) {
        for wire in &mut self.wires {
            for point in &mut wire.points {
                if *point == old_pos {
                    *point = new_pos;
                }
            }
        }

        if let Some(junction) = self.junctions.iter_mut().find(|j| j.pos == old_pos) {
            junction.pos = new_pos;
        }

        self.is_dirty = true;
    }

    /// Get wire points adjusted for a component drag preview
    pub fn get_wire_preview_points(
        &self,
        wire: &Wire,
        dragging_component_id: Option<u64>,
        delta: Point,
    ) -> Vec<Point> {
        let mut points = wire.points.clone();

        let comp_id = match dragging_component_id {
            Some(id) => id,
            None => return points,
        };

        let terminals: Vec<Point> =
            if let Some(comp) = self.components.iter().find(|c| c.id == comp_id) {
                comp.terminal_positions()
                    .into_iter()
                    .map(|(_, pos)| pos)
                    .collect()
            } else {
                return points;
            };

        for point in points.iter_mut() {
            for term_pos in &terminals {
                if *point == *term_pos {
                    *point = Point::new(term_pos.x + delta.x, term_pos.y + delta.y);
                    break;
                }
            }
        }

        points
    }

    /// Snap wire endpoints to nearby terminals and rebuild connections
    pub fn snap_wire_to_terminals(&mut self, wire_id: u64) {
        let (start_pos, end_pos, end_idx) = {
            if let Some(wire) = self.wires.iter().find(|w| w.id == wire_id) {
                if wire.points.is_empty() {
                    return;
                }
                let end_idx = wire.points.len().saturating_sub(1);
                (
                    Some(wire.points[0]),
                    if end_idx > 0 {
                        Some(wire.points[end_idx])
                    } else {
                        None
                    },
                    end_idx,
                )
            } else {
                return;
            }
        };

        let snap_start = start_pos.and_then(|p| self.find_terminal_at(p).map(|(_, _, pos)| pos));
        let snap_end = end_pos.and_then(|p| self.find_terminal_at(p).map(|(_, _, pos)| pos));

        if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
            if let Some(term_pos) = snap_start {
                wire.points[0] = term_pos;
            }
            if let Some(term_pos) = snap_end {
                if end_idx > 0 {
                    wire.points[end_idx] = term_pos;
                }
            }
        }

        self.rebuild_connections();
    }

    // =========================================================================
    // Automatic Junction Detection System (Commercial-Grade)
    // =========================================================================

    /// Find all intersection points between wires
    ///
    /// This detects where wires cross or connect, which is essential for
    /// automatic junction placement. Like Cadence, we check both:
    /// 1. Wire-to-wire segment intersections
    /// 2. Wire endpoints touching other wires
    pub fn find_wire_intersections(&self) -> Vec<(Point, Vec<u64>)> {
        use std::collections::HashMap;

        let mut intersection_map: HashMap<Point, Vec<u64>> = HashMap::new();

        // Phase 1: Find all wire endpoint connections
        for wire in &self.wires {
            if let Some(start) = wire.start() {
                intersection_map.entry(start).or_default().push(wire.id);
            }
            if let Some(end) = wire.end() {
                if wire.points.len() > 1 {
                    intersection_map.entry(end).or_default().push(wire.id);
                }
            }
        }

        // Phase 2: Find wire-to-wire segment intersections
        let wires: Vec<_> = self.wires.iter().collect();
        for i in 0..wires.len() {
            for j in (i + 1)..wires.len() {
                let wire_a = wires[i];
                let wire_b = wires[j];

                // Get all intersection points between these two wires
                let intersections = wire_a.intersections_with_wire(wire_b);
                for point in intersections {
                    let entry = intersection_map.entry(point).or_default();
                    if !entry.contains(&wire_a.id) {
                        entry.push(wire_a.id);
                    }
                    if !entry.contains(&wire_b.id) {
                        entry.push(wire_b.id);
                    }
                }
            }
        }

        // Phase 3: Check if any wire passes through another wire's vertex
        for wire in &self.wires {
            for other_wire in &self.wires {
                if wire.id == other_wire.id {
                    continue;
                }
                // Check if other_wire passes through any vertex of wire
                for vertex in &wire.points {
                    if other_wire.contains_point(*vertex) {
                        let entry = intersection_map.entry(*vertex).or_default();
                        if !entry.contains(&wire.id) {
                            entry.push(wire.id);
                        }
                        if !entry.contains(&other_wire.id) {
                            entry.push(other_wire.id);
                        }
                    }
                }
            }
        }

        // Convert to sorted vector - only include points with 2+ wires (actual connections)
        let mut result: Vec<_> = intersection_map
            .into_iter()
            .filter(|(_, wire_ids)| wire_ids.len() >= 2)
            .collect();
        result.sort_by(|a, b| a.0.x.cmp(&b.0.x).then_with(|| a.0.y.cmp(&b.0.y)));
        result
    }

    /// Detect junction points that need visual markers
    ///
    /// A junction needs a marker (dot) when:
    /// - 3+ wire segments meet at a point (T-junction or cross)
    /// - A wire endpoint meets another wire mid-segment (T-junction)
    ///
    /// This counts SEGMENTS meeting at each point, not wire IDs:
    /// - A wire endpoint contributes 1 segment
    /// - A wire passing through mid-segment contributes 2 segments
    pub fn detect_junction_points(&self) -> Vec<Point> {
        use std::collections::HashMap;

        let mut segment_counts: HashMap<Point, usize> = HashMap::new();

        for wire in &self.wires {
            // Check each point on the wire
            for (i, point) in wire.points.iter().enumerate() {
                let is_endpoint = i == 0 || i == wire.points.len() - 1;
                let count = if is_endpoint { 1 } else { 2 }; // Mid-point = 2 segments
                *segment_counts.entry(*point).or_insert(0) += count;
            }

            // Also check if wire passes through any point on another wire
            for other_wire in &self.wires {
                if wire.id == other_wire.id {
                    continue;
                }
                // Check if other_wire vertices lie on this wire's segments
                for vertex in &other_wire.points {
                    if wire.contains_point(*vertex) {
                        // Check if this is already counted as a wire vertex
                        let is_vertex_of_wire = wire.points.contains(vertex);
                        if !is_vertex_of_wire {
                            // Wire passes through this point mid-segment = 2 segments
                            *segment_counts.entry(*vertex).or_insert(0) += 2;
                        }
                    }
                }
            }
        }

        // Return points where 3+ segments meet (T-junction or more)
        segment_counts
            .into_iter()
            .filter(|(_, count)| *count >= 3)
            .map(|(point, _)| point)
            .collect()
    }

    /// Classify the type of junction at a given point
    pub fn classify_junction_type(&self, pos: Point) -> super::wire::JunctionType {
        let wire_count = self.wires.iter().filter(|w| w.contains_point(pos)).count();
        super::wire::JunctionType::from_wire_count(wire_count)
    }

    /// Automatically place junctions at all detected intersection points
    ///
    /// This is the main entry point for automatic junction management.
    /// Call this after wire operations to maintain junction consistency.
    pub fn auto_place_junctions(&mut self) {
        let junction_points = self.detect_junction_points();
        let mut changes = false;

        // Add junctions at detected points that don't have one
        for point in &junction_points {
            let has_junction = self.junctions.iter().any(|j| j.pos == *point);
            if !has_junction {
                self.add_junction(*point);
                changes = true;
            }
        }

        // Remove junctions that are no longer at intersection points
        let len_before = self.junctions.len();
        self.junctions.retain(|j| junction_points.contains(&j.pos));
        if self.junctions.len() != len_before {
            changes = true;
        }

        if changes {
            self.is_dirty = true;
            self.bump_topology_version();
        }
    }

    /// Remove orphaned junctions that no longer have wire connections
    pub fn remove_orphan_junctions(&mut self) -> usize {
        let initial_count = self.junctions.len();

        self.junctions.retain(|junction| {
            // Keep junction if any wire passes through it
            self.wires.iter().any(|w| w.contains_point(junction.pos))
        });

        let removed = initial_count - self.junctions.len();
        if removed > 0 {
            self.is_dirty = true;
            self.bump_topology_version();
        }
        removed
    }

    /// Update junction markers based on current wire topology
    ///
    /// This is a more comprehensive update that:
    /// 1. Removes orphan junctions
    /// 2. Places new junctions where needed
    /// 3. Updates junction types
    pub fn update_wire_junctions(&mut self) {
        self.remove_orphan_junctions();
        self.auto_place_junctions();
    }

    /// Find all points where wires could potentially be split
    /// (where they cross other wires without connecting)
    pub fn find_potential_splits(&self) -> Vec<(u64, Point)> {
        let mut splits = Vec::new();

        for wire in &self.wires {
            for other_wire in &self.wires {
                if wire.id == other_wire.id {
                    continue;
                }

                let intersections = wire.intersections_with_wire(other_wire);
                for point in intersections {
                    // Check if this intersection point is already a vertex on wire
                    let is_vertex = wire.points.iter().any(|p| *p == point);
                    if !is_vertex {
                        splits.push((wire.id, point));
                    }
                }
            }
        }

        // Deduplicate
        splits.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.x.cmp(&b.1.x)));
        splits.dedup();
        splits
    }

    /// Create T-junctions by splitting wires at intersection points
    ///
    /// When a wire endpoint touches another wire mid-segment,
    /// this will split the second wire and create proper junction.
    pub fn create_t_junctions_from_endpoints(&mut self) {
        // Find all points where one wire ends on another wire's segment
        let mut splits_needed: Vec<(u64, Point)> = Vec::new();

        for wire in &self.wires {
            let endpoints = [wire.start(), wire.end()];
            for endpoint in endpoints.into_iter().flatten() {
                for other_wire in &self.wires {
                    if other_wire.id == wire.id {
                        continue;
                    }

                    // Check if endpoint is on other_wire but not at a vertex
                    if other_wire.contains_point(endpoint) {
                        let is_at_vertex = other_wire.points.iter().any(|p| *p == endpoint);
                        if !is_at_vertex {
                            splits_needed.push((other_wire.id, endpoint));
                        }
                    }
                }
            }
        }

        // Perform splits and add junctions
        let has_splits = !splits_needed.is_empty();
        for (wire_id, point) in splits_needed {
            // Insert vertex at the intersection point
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
                if let Some((segment_idx, _segment)) = wire.segment_containing_point(point) {
                    wire.insert_vertex(segment_idx + 1, point);
                }
            }

            // Ensure junction exists at this point
            let has_junction = self.junctions.iter().any(|j| j.pos == point);
            if !has_junction {
                self.add_junction(point);
            }
        }

        if has_splits {
            self.is_dirty = true;
            self.bump_topology_version();
        }
    }

    /// Count how many wire segments connect at a given point
    pub fn count_connections_at(&self, pos: Point) -> usize {
        let mut count = 0;
        for wire in &self.wires {
            // Count endpoints
            if wire.start() == Some(pos) {
                count += 1;
            }
            if wire.end() == Some(pos) && wire.points.len() > 1 {
                count += 1;
            }

            // Count mid-wire vertices (each vertex connects 2 segments)
            for (i, point) in wire.points.iter().enumerate() {
                if *point == pos && i > 0 && i < wire.points.len() - 1 {
                    count += 2; // This vertex connects two segments
                }
            }
        }
        count
    }
}
