//! Selection Management
//!
//! Tracks selected components, wires, and wire sub-elements in the schematic.
//! Supports commercial-grade selection including:
//! - Component selection
//! - Whole wire selection
//! - Individual segment selection (for dragging)
//! - Vertex selection (for corner manipulation)
//! - Rubber-band box selection (drag to select multiple items)

use super::Point;
use serde::{Deserialize, Serialize};

// =============================================================================
// Selection Rectangle (Rubber-band Box Selection)
// =============================================================================

/// Rubber-band selection rectangle state
///
/// Used during drag-to-select operations. The user clicks and drags to create
/// a rectangular region, and all items within the region are selected.
/// Matches Cadence Virtuoso and other professional EDA tool behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SelectionRect {
    /// Starting point of the selection (where mouse was pressed)
    pub start: Point,

    /// Current point of the selection (current mouse position)
    pub current: Point,

    /// Whether a selection drag is currently active
    pub active: bool,
}

impl SelectionRect {
    /// Create a new inactive selection rect
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a new selection rectangle at the given position
    pub fn start_at(&mut self, pos: Point) {
        self.start = pos;
        self.current = pos;
        self.active = true;
    }

    /// Update the current position during drag
    pub fn update(&mut self, pos: Point) {
        if self.active {
            self.current = pos;
        }
    }

    /// Finish the selection and return the bounds
    ///
    /// Returns `Some((min_x, min_y, max_x, max_y))` if a valid selection was made,
    /// or `None` if the selection is empty (same start and end point).
    pub fn finish(&mut self) -> Option<(i32, i32, i32, i32)> {
        if !self.active {
            return None;
        }
        self.active = false;

        // Compute normalized bounds (min to max)
        let (min_x, max_x) = if self.start.x <= self.current.x {
            (self.start.x, self.current.x)
        } else {
            (self.current.x, self.start.x)
        };

        let (min_y, max_y) = if self.start.y <= self.current.y {
            (self.start.y, self.current.y)
        } else {
            (self.current.y, self.start.y)
        };

        // Return None for zero-size selections (just a click)
        if min_x == max_x && min_y == max_y {
            return None;
        }

        Some((min_x, min_y, max_x, max_y))
    }

    /// Cancel the current selection
    pub fn cancel(&mut self) {
        self.active = false;
    }

    /// Get the current bounds of the selection rectangle (normalized)
    ///
    /// Returns `(min_x, min_y, max_x, max_y)` regardless of drag direction.
    pub fn bounds(&self) -> (i32, i32, i32, i32) {
        let (min_x, max_x) = if self.start.x <= self.current.x {
            (self.start.x, self.current.x)
        } else {
            (self.current.x, self.start.x)
        };

        let (min_y, max_y) = if self.start.y <= self.current.y {
            (self.start.y, self.current.y)
        } else {
            (self.current.y, self.start.y)
        };

        (min_x, min_y, max_x, max_y)
    }

    /// Check if the selection rectangle is active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Check if a point is within the selection rectangle
    pub fn contains(&self, point: Point) -> bool {
        let (min_x, min_y, max_x, max_y) = self.bounds();
        point.x >= min_x && point.x <= max_x && point.y >= min_y && point.y <= max_y
    }

    /// Check if a rectangle (min_x, min_y, max_x, max_y) intersects the selection
    pub fn intersects_rect(&self, rect: (i32, i32, i32, i32)) -> bool {
        let (sel_min_x, sel_min_y, sel_max_x, sel_max_y) = self.bounds();
        let (rect_min_x, rect_min_y, rect_max_x, rect_max_y) = rect;

        // Check for non-intersection
        !(rect_max_x < sel_min_x
            || rect_min_x > sel_max_x
            || rect_max_y < sel_min_y
            || rect_min_y > sel_max_y)
    }
}

// =============================================================================
// Wire Segment Selection
// =============================================================================

/// Selection of a specific wire segment
///
/// Used for fine-grained wire manipulation where the user wants to
/// move a single segment (e.g., the middle horizontal part of a wire).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WireSegmentSelection {
    /// Wire ID
    pub wire_id: u64,

    /// Segment index (between points[index] and points[index+1])
    pub segment_index: usize,
}

impl WireSegmentSelection {
    /// Create a new segment selection
    pub fn new(wire_id: u64, segment_index: usize) -> Self {
        Self {
            wire_id,
            segment_index,
        }
    }
}

// =============================================================================
// Wire Vertex Selection
// =============================================================================

/// Selection of a specific wire vertex (corner point)
///
/// Used for manipulating individual corners of a wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WireVertexSelection {
    /// Wire ID
    pub wire_id: u64,

    /// Vertex index (into points array)
    pub vertex_index: usize,
}

impl WireVertexSelection {
    /// Create a new vertex selection
    pub fn new(wire_id: u64, vertex_index: usize) -> Self {
        Self {
            wire_id,
            vertex_index,
        }
    }

    /// Check if this is the start vertex (first point)
    pub fn is_start(&self) -> bool {
        self.vertex_index == 0
    }
}

// =============================================================================
// Junction Selection
// =============================================================================

/// Selection of a junction point
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JunctionSelection {
    /// Position of the junction
    pub pos: Point,
}

impl JunctionSelection {
    /// Create a new junction selection
    pub fn new(pos: Point) -> Self {
        Self { pos }
    }
}

// =============================================================================
// Selection
// =============================================================================

/// Selection state for schematic elements
///
/// Tracks which components, wires, segments, vertices, and junctions
/// are currently selected. Supports multi-selection for batch operations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Selection {
    /// Selected component IDs
    pub components: Vec<u64>,

    /// Selected wire IDs (whole wire selection)
    pub wires: Vec<u64>,

    /// Selected wire segments (for segment-level manipulation)
    pub wire_segments: Vec<WireSegmentSelection>,

    /// Selected wire vertices (for corner manipulation)
    pub wire_vertices: Vec<WireVertexSelection>,

    /// Selected junctions
    pub junctions: Vec<JunctionSelection>,
}

impl Selection {
    /// Create an empty selection
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if selection is empty (no items selected)
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
            && self.wires.is_empty()
            && self.wire_segments.is_empty()
            && self.wire_vertices.is_empty()
            && self.junctions.is_empty()
    }

    /// Get total number of selected items
    pub fn count(&self) -> usize {
        self.components.len()
            + self.wires.len()
            + self.wire_segments.len()
            + self.wire_vertices.len()
            + self.junctions.len()
    }

    /// Clear all selections
    pub fn clear(&mut self) {
        self.components.clear();
        self.wires.clear();
        self.wire_segments.clear();
        self.wire_vertices.clear();
        self.junctions.clear();
    }

    // =========================================================================
    // Component Selection
    // =========================================================================

    /// Check if a component is selected
    pub fn has_component(&self, id: u64) -> bool {
        self.components.contains(&id)
    }

    /// Select a component (if not already selected)
    pub fn select_component(&mut self, id: u64) {
        if !self.has_component(id) {
            self.components.push(id);
        }
    }

    /// Deselect a component
    pub fn deselect_component(&mut self, id: u64) {
        self.components.retain(|&c| c != id);
    }

    /// Toggle component selection
    pub fn toggle_component(&mut self, id: u64) {
        if self.has_component(id) {
            self.deselect_component(id);
        } else {
            self.select_component(id);
        }
    }

    /// Select only a single component (clears other selections)
    pub fn select_only_component(&mut self, id: u64) {
        self.clear();
        self.components.push(id);
    }

    /// Get the single selected component ID (if exactly one is selected)
    pub fn single_component(&self) -> Option<u64> {
        if self.components.len() == 1
            && self.wires.is_empty()
            && self.wire_segments.is_empty()
            && self.wire_vertices.is_empty()
            && self.junctions.is_empty()
        {
            Some(self.components[0])
        } else {
            None
        }
    }

    // =========================================================================
    // Wire Selection (Whole Wire)
    // =========================================================================

    /// Check if a wire is selected
    pub fn has_wire(&self, id: u64) -> bool {
        self.wires.contains(&id)
    }

    /// Select a wire (if not already selected)
    pub fn select_wire(&mut self, id: u64) {
        if !self.has_wire(id) {
            self.wires.push(id);
        }
    }

    /// Deselect a wire
    pub fn deselect_wire(&mut self, id: u64) {
        self.wires.retain(|&w| w != id);
    }

    /// Toggle wire selection
    pub fn toggle_wire(&mut self, id: u64) {
        if self.has_wire(id) {
            self.deselect_wire(id);
        } else {
            self.select_wire(id);
        }
    }

    /// Select only a single wire (clears other selections)
    pub fn select_only_wire(&mut self, id: u64) {
        self.clear();
        self.wires.push(id);
    }

    /// Get the single selected wire ID (if exactly one is selected)
    pub fn single_wire(&self) -> Option<u64> {
        if self.wires.len() == 1
            && self.components.is_empty()
            && self.wire_segments.is_empty()
            && self.wire_vertices.is_empty()
            && self.junctions.is_empty()
        {
            Some(self.wires[0])
        } else {
            None
        }
    }

    // =========================================================================
    // Wire Segment Selection
    // =========================================================================

    /// Check if a wire segment is selected
    pub fn has_wire_segment(&self, wire_id: u64, segment_index: usize) -> bool {
        self.wire_segments
            .iter()
            .any(|s| s.wire_id == wire_id && s.segment_index == segment_index)
    }

    /// Select a wire segment
    pub fn select_wire_segment(&mut self, wire_id: u64, segment_index: usize) {
        if !self.has_wire_segment(wire_id, segment_index) {
            self.wire_segments
                .push(WireSegmentSelection::new(wire_id, segment_index));
        }
    }

    /// Deselect a wire segment
    pub fn deselect_wire_segment(&mut self, wire_id: u64, segment_index: usize) {
        self.wire_segments
            .retain(|s| s.wire_id != wire_id || s.segment_index != segment_index);
    }

    /// Select only a single wire segment
    pub fn select_only_wire_segment(&mut self, wire_id: u64, segment_index: usize) {
        self.clear();
        self.wire_segments
            .push(WireSegmentSelection::new(wire_id, segment_index));
    }

    /// Get single selected segment
    pub fn single_wire_segment(&self) -> Option<&WireSegmentSelection> {
        if self.wire_segments.len() == 1
            && self.components.is_empty()
            && self.wires.is_empty()
            && self.wire_vertices.is_empty()
            && self.junctions.is_empty()
        {
            Some(&self.wire_segments[0])
        } else {
            None
        }
    }

    // =========================================================================
    // Wire Vertex Selection
    // =========================================================================

    /// Check if a wire vertex is selected
    pub fn has_wire_vertex(&self, wire_id: u64, vertex_index: usize) -> bool {
        self.wire_vertices
            .iter()
            .any(|v| v.wire_id == wire_id && v.vertex_index == vertex_index)
    }

    /// Select a wire vertex
    pub fn select_wire_vertex(&mut self, wire_id: u64, vertex_index: usize) {
        if !self.has_wire_vertex(wire_id, vertex_index) {
            self.wire_vertices
                .push(WireVertexSelection::new(wire_id, vertex_index));
        }
    }

    /// Deselect a wire vertex
    pub fn deselect_wire_vertex(&mut self, wire_id: u64, vertex_index: usize) {
        self.wire_vertices
            .retain(|v| v.wire_id != wire_id || v.vertex_index != vertex_index);
    }

    /// Select only a single wire vertex
    pub fn select_only_wire_vertex(&mut self, wire_id: u64, vertex_index: usize) {
        self.clear();
        self.wire_vertices
            .push(WireVertexSelection::new(wire_id, vertex_index));
    }

    /// Get single selected vertex
    pub fn single_wire_vertex(&self) -> Option<&WireVertexSelection> {
        if self.wire_vertices.len() == 1
            && self.components.is_empty()
            && self.wires.is_empty()
            && self.wire_segments.is_empty()
            && self.junctions.is_empty()
        {
            Some(&self.wire_vertices[0])
        } else {
            None
        }
    }

    // =========================================================================
    // Junction Selection
    // =========================================================================

    /// Check if a junction is selected
    pub fn has_junction(&self, pos: Point) -> bool {
        self.junctions.iter().any(|j| j.pos == pos)
    }

    /// Select a junction
    pub fn select_junction(&mut self, pos: Point) {
        if !self.has_junction(pos) {
            self.junctions.push(JunctionSelection::new(pos));
        }
    }

    /// Deselect a junction
    pub fn deselect_junction(&mut self, pos: Point) {
        self.junctions.retain(|j| j.pos != pos);
    }

    /// Select only a single junction
    pub fn select_only_junction(&mut self, pos: Point) {
        self.clear();
        self.junctions.push(JunctionSelection::new(pos));
    }

    // =========================================================================
    // Query Methods
    // =========================================================================

    /// Check if selection contains multiple items
    pub fn is_multi_selection(&self) -> bool {
        self.count() > 1
    }

    /// Check if only components are selected (no wires or other items)
    pub fn only_components(&self) -> bool {
        !self.components.is_empty()
            && self.wires.is_empty()
            && self.wire_segments.is_empty()
            && self.wire_vertices.is_empty()
            && self.junctions.is_empty()
    }

    /// Check if only wires are selected (no components or other items)
    pub fn only_wires(&self) -> bool {
        self.components.is_empty()
            && !self.wires.is_empty()
            && self.wire_segments.is_empty()
            && self.wire_vertices.is_empty()
            && self.junctions.is_empty()
    }

    /// Check if any wire-related items are selected
    pub fn has_any_wire_selection(&self) -> bool {
        !self.wires.is_empty() || !self.wire_segments.is_empty() || !self.wire_vertices.is_empty()
    }

    /// Get all selected wire IDs (including from segments and vertices)
    pub fn all_selected_wire_ids(&self) -> Vec<u64> {
        let mut ids: Vec<u64> = self.wires.clone();
        for seg in &self.wire_segments {
            if !ids.contains(&seg.wire_id) {
                ids.push(seg.wire_id);
            }
        }
        for vtx in &self.wire_vertices {
            if !ids.contains(&vtx.wire_id) {
                ids.push(vtx.wire_id);
            }
        }
        ids
    }
}

// =============================================================================
// Tests
// =============================================================================

