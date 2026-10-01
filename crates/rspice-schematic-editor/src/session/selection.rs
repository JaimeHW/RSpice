//! Schematic selection gestures and editor filter preferences.

use rspice_design_model::Point;
use serde::{Deserialize, Serialize};

/// Session-owned schematic selection classes.
///
/// This is deliberately separate from [`Selection`]: the filter is an editor
/// preference, never design data, and therefore belongs to the serialized UI
/// session rather than a schematic file or undo snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SchematicSelectionFilter {
    pub instances: bool,
    pub wires: bool,
    pub labels: bool,
    pub annotations: bool,
}

impl Default for SchematicSelectionFilter {
    fn default() -> Self {
        Self {
            instances: true,
            wires: true,
            labels: true,
            annotations: true,
        }
    }
}

/// What Duplicate does with a terminal whose net keeps existing outside the
/// copied set.
///
/// This is a sticky editor preference rather than a per-command decision: a
/// user who works on one style of schematic wants the same answer every time,
/// and the alternative was a modal in front of every Ctrl+D.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum DuplicateExternalNets {
    /// The copy's terminals come out unconnected.
    #[default]
    LeaveUnconnected,
    /// The copy keeps the named connection the original terminal had.
    PreserveNamedNetAttachment,
}

impl SchematicSelectionFilter {
    /// Remove objects belonging to disabled classes from a live selection.
    ///
    /// Wire handles, junctions, buses and taps all belong to the electrical
    /// conductor class. Design notes and documentation geometry are
    /// annotations; net labels remain their own class.
    pub fn retain_matching(self, selection: &mut Selection) {
        if !self.instances {
            selection.components.clear();
        }
        if !self.wires {
            selection.wires.clear();
            selection.wire_segments.clear();
            selection.wire_vertices.clear();
            selection.junctions.clear();
            selection.buses.clear();
            selection.bus_taps.clear();
        }
        if !self.labels {
            selection.net_labels.clear();
        }
        if !self.annotations {
            selection.design_notes.clear();
            selection.documentation_shapes.clear();
            selection.probes.clear();
        }
    }
}

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

pub use rspice_design::schematic::selection::{JunctionSelection, Selection};

// =============================================================================
// Tests
// =============================================================================

/// Stable canvas object currently owned by schematic keyboard traversal.
///
/// The authored document selection remains the command authority for objects
/// that already participate in editing. Probe flags deliberately remain
/// output-intent markers rather than pretending to support edit operations;
/// this transient identity gives them the same visible keyboard focus without
/// inventing probe clipboard/delete semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchematicKeyboardFocus {
    Component(u64),
    Wire(u64),
    Bus(u64),
    BusTap(u64),
    Junction(u64),
    NetLabel(u64),
    Probe(u64),
    DesignNote(u64),
    DocumentationShape(u64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schematic_selection_filter_defaults_to_every_mockup_class() {
        let filter = SchematicSelectionFilter::default();

        assert_eq!(
            filter,
            SchematicSelectionFilter {
                instances: true,
                wires: true,
                labels: true,
                annotations: true,
            }
        );
    }

    #[test]
    fn schematic_selection_filter_prunes_every_disabled_object_taxonomy() {
        let mut selection = Selection::new();
        selection.select_component(1);
        selection.select_wire(2);
        selection.select_wire_segment(2, 0);
        selection.select_wire_vertex(2, 0);
        selection.select_junction(Point::new(3, 4));
        selection.select_bus(5);
        selection.select_bus_tap(6);
        selection.select_net_label(7);
        selection.select_design_note(8);
        selection.select_documentation_shape(9);

        SchematicSelectionFilter {
            instances: false,
            wires: false,
            labels: true,
            annotations: false,
        }
        .retain_matching(&mut selection);

        assert_eq!(selection.count(), 1);
        assert!(selection.has_net_label(7));
    }

    #[test]
    fn probe_selection_is_exclusive_and_participates_in_annotation_filtering() {
        let mut selection = Selection::new();
        selection.select_only_probe(73);

        assert!(selection.has_probe(73));
        assert_eq!(selection.single_probe(), Some(73));
        assert_eq!(selection.count(), 1);

        selection.select_component(9);
        assert_eq!(selection.single_probe(), None);

        SchematicSelectionFilter {
            instances: true,
            wires: true,
            labels: true,
            annotations: false,
        }
        .retain_matching(&mut selection);

        assert!(!selection.has_probe(73));
        assert!(selection.has_component(9));
    }
}
