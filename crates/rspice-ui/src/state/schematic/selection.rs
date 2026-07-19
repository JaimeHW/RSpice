//! Selection Management
//!
//! Tracks selected components, wires, and wire sub-elements in the schematic.
//! Supports commercial-grade selection including:
//! - Component selection
//! - Whole wire selection
//! - Individual segment selection (for dragging)
//! - Vertex selection (for corner manipulation)
//! - Rubber-band box selection (drag to select multiple items)

use std::collections::HashSet;

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
/// Tracks which components, wires, buses, taps, net labels, segments, vertices,
/// and junctions are currently selected. Supports multi-selection for batch
/// operations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Selection {
    /// Selected component IDs. A set: membership is queried per element
    /// per frame by the paint loop, so lookups must be O(1).
    pub components: HashSet<u64>,

    /// Selected wire IDs (whole wire selection); a set for the same reason.
    pub wires: HashSet<u64>,

    /// Selected wire segments (for segment-level manipulation)
    pub wire_segments: Vec<WireSegmentSelection>,

    /// Selected wire vertices (for corner manipulation)
    pub wire_vertices: Vec<WireVertexSelection>,

    /// Selected junctions
    pub junctions: Vec<JunctionSelection>,

    /// Selected bus IDs.
    #[serde(default)]
    pub buses: HashSet<u64>,

    /// Selected bus-tap IDs.
    #[serde(default)]
    pub bus_taps: HashSet<u64>,

    /// Selected net-label stable IDs.
    #[serde(default)]
    pub net_labels: HashSet<u64>,
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
            && self.buses.is_empty()
            && self.bus_taps.is_empty()
            && self.net_labels.is_empty()
    }

    /// Get total number of selected items
    pub fn count(&self) -> usize {
        self.components.len()
            + self.wires.len()
            + self.wire_segments.len()
            + self.wire_vertices.len()
            + self.junctions.len()
            + self.buses.len()
            + self.bus_taps.len()
            + self.net_labels.len()
    }

    /// Clear all selections
    pub fn clear(&mut self) {
        self.components.clear();
        self.wires.clear();
        self.wire_segments.clear();
        self.wire_vertices.clear();
        self.junctions.clear();
        self.buses.clear();
        self.bus_taps.clear();
        self.net_labels.clear();
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
        self.components.insert(id);
    }

    /// Deselect a component
    pub fn deselect_component(&mut self, id: u64) {
        self.components.remove(&id);
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
        self.components.insert(id);
    }

    /// Get the single selected component ID (if exactly one is selected)
    pub fn single_component(&self) -> Option<u64> {
        if self.components.len() == 1
            && self.wires.is_empty()
            && self.wire_segments.is_empty()
            && self.wire_vertices.is_empty()
            && self.junctions.is_empty()
            && self.buses.is_empty()
            && self.bus_taps.is_empty()
            && self.net_labels.is_empty()
        {
            self.components.iter().next().copied()
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
        self.wires.insert(id);
    }

    /// Deselect a wire
    pub fn deselect_wire(&mut self, id: u64) {
        self.wires.remove(&id);
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
        self.wires.insert(id);
    }

    /// Get the single selected wire ID (if exactly one is selected)
    pub fn single_wire(&self) -> Option<u64> {
        if self.wires.len() == 1
            && self.components.is_empty()
            && self.wire_segments.is_empty()
            && self.wire_vertices.is_empty()
            && self.junctions.is_empty()
            && self.buses.is_empty()
            && self.bus_taps.is_empty()
            && self.net_labels.is_empty()
        {
            self.wires.iter().next().copied()
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
            && self.buses.is_empty()
            && self.bus_taps.is_empty()
            && self.net_labels.is_empty()
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
            && self.buses.is_empty()
            && self.bus_taps.is_empty()
            && self.net_labels.is_empty()
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

    /// Get the single selected junction position (if exactly one is selected).
    pub fn single_junction(&self) -> Option<Point> {
        if self.junctions.len() == 1
            && self.components.is_empty()
            && self.wires.is_empty()
            && self.wire_segments.is_empty()
            && self.wire_vertices.is_empty()
            && self.buses.is_empty()
            && self.bus_taps.is_empty()
            && self.net_labels.is_empty()
        {
            Some(self.junctions[0].pos)
        } else {
            None
        }
    }

    // =========================================================================
    // Bus and Bus-Tap Selection
    // =========================================================================

    pub fn has_bus(&self, id: u64) -> bool {
        self.buses.contains(&id)
    }

    pub fn select_bus(&mut self, id: u64) {
        self.buses.insert(id);
    }

    pub fn deselect_bus(&mut self, id: u64) {
        self.buses.remove(&id);
    }

    pub fn toggle_bus(&mut self, id: u64) {
        if !self.buses.remove(&id) {
            self.buses.insert(id);
        }
    }

    pub fn select_only_bus(&mut self, id: u64) {
        self.clear();
        self.buses.insert(id);
    }

    pub fn single_bus(&self) -> Option<u64> {
        (self.buses.len() == 1 && self.count() == 1)
            .then(|| self.buses.iter().next().copied())
            .flatten()
    }

    pub fn has_bus_tap(&self, id: u64) -> bool {
        self.bus_taps.contains(&id)
    }

    pub fn select_bus_tap(&mut self, id: u64) {
        self.bus_taps.insert(id);
    }

    pub fn deselect_bus_tap(&mut self, id: u64) {
        self.bus_taps.remove(&id);
    }

    pub fn toggle_bus_tap(&mut self, id: u64) {
        if !self.bus_taps.remove(&id) {
            self.bus_taps.insert(id);
        }
    }

    pub fn select_only_bus_tap(&mut self, id: u64) {
        self.clear();
        self.bus_taps.insert(id);
    }

    pub fn single_bus_tap(&self) -> Option<u64> {
        (self.bus_taps.len() == 1 && self.count() == 1)
            .then(|| self.bus_taps.iter().next().copied())
            .flatten()
    }

    // =========================================================================
    // Net-Label Selection
    // =========================================================================

    /// Check whether a net label is selected by its stable document ID.
    pub fn has_net_label(&self, id: u64) -> bool {
        self.net_labels.contains(&id)
    }

    /// Add a net label to the current selection.
    pub fn select_net_label(&mut self, id: u64) {
        self.net_labels.insert(id);
    }

    /// Remove a net label from the current selection.
    pub fn deselect_net_label(&mut self, id: u64) {
        self.net_labels.remove(&id);
    }

    /// Toggle a net label in the current selection.
    pub fn toggle_net_label(&mut self, id: u64) {
        if !self.net_labels.remove(&id) {
            self.net_labels.insert(id);
        }
    }

    /// Replace the current selection with one net label.
    pub fn select_only_net_label(&mut self, id: u64) {
        self.clear();
        self.net_labels.insert(id);
    }

    /// Return the selected label ID only when it is the sole selected item.
    pub fn single_net_label(&self) -> Option<u64> {
        (self.net_labels.len() == 1 && self.count() == 1)
            .then(|| self.net_labels.iter().next().copied())
            .flatten()
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
            && self.buses.is_empty()
            && self.bus_taps.is_empty()
            && self.net_labels.is_empty()
    }

    /// Check if only wires are selected (no components or other items)
    pub fn only_wires(&self) -> bool {
        self.components.is_empty()
            && !self.wires.is_empty()
            && self.wire_segments.is_empty()
            && self.wire_vertices.is_empty()
            && self.junctions.is_empty()
            && self.buses.is_empty()
            && self.bus_taps.is_empty()
            && self.net_labels.is_empty()
    }

    /// Check if any wire-related items are selected
    pub fn has_any_wire_selection(&self) -> bool {
        !self.wires.is_empty() || !self.wire_segments.is_empty() || !self.wire_vertices.is_empty()
    }

    /// Check if any bus or bus-tap object is selected.
    pub fn has_any_bus_selection(&self) -> bool {
        !self.buses.is_empty() || !self.bus_taps.is_empty()
    }

    /// Get all selected wire IDs (including from segments and vertices)
    pub fn all_selected_wire_ids(&self) -> Vec<u64> {
        let mut ids = self.wires.clone();
        ids.extend(self.wire_segments.iter().map(|s| s.wire_id));
        ids.extend(self.wire_vertices.iter().map(|v| v.wire_id));
        ids.into_iter().collect()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_junction_requires_an_exclusive_junction_selection() {
        let point = Point::new(4, 6);
        let mut selection = Selection::new();
        selection.select_only_junction(point);
        assert_eq!(selection.single_junction(), Some(point));

        selection.select_component(9);
        assert_eq!(selection.single_junction(), None);
    }

    #[test]
    fn bus_and_tap_selection_participate_in_exclusive_queries_and_clear() {
        let mut selection = Selection::new();
        selection.select_only_bus(9);
        assert_eq!(selection.single_bus(), Some(9));
        selection.select_bus_tap(10);
        assert_eq!(selection.single_bus(), None);
        assert!(selection.has_any_bus_selection());
        assert_eq!(selection.count(), 2);
        selection.clear();
        assert!(selection.is_empty());
    }

    #[test]
    fn net_label_selection_uses_stable_ids_and_exclusive_queries() {
        let mut selection = Selection::new();
        selection.select_only_net_label(41);
        assert!(selection.has_net_label(41));
        assert_eq!(selection.single_net_label(), Some(41));
        assert_eq!(selection.count(), 1);

        selection.toggle_net_label(42);
        assert_eq!(selection.count(), 2);
        assert_eq!(selection.single_net_label(), None);
        selection.toggle_net_label(42);
        selection.deselect_net_label(41);
        assert!(selection.is_empty());
    }

    #[test]
    fn legacy_selection_without_net_label_ids_deserializes_empty() {
        let mut value = serde_json::to_value(Selection::default()).unwrap();
        value.as_object_mut().unwrap().remove("net_labels");

        let selection: Selection = serde_json::from_value(value).unwrap();

        assert!(selection.net_labels.is_empty());
    }
}
