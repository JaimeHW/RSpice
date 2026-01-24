//! State Types for Schematic Editor
//!
//! Contains view-local state types for drag operations, selections,
//! context menus, editing dialogs, and label repositioning.

use crate::state::Point;

// =============================================================================
// Drag State
// =============================================================================

/// Drag operation state for moving components, wires, or junctions
#[derive(Clone, Copy, PartialEq, Default)]
pub struct DragState {
    /// Whether a drag is currently active
    pub active: bool,
    /// ID of the component being dragged (if any)
    pub component_id: Option<u64>,
    /// ID of the wire being dragged (if any)
    pub wire_id: Option<u64>,
    /// Junction point being dragged (moves all wires at that point)
    pub junction_point: Option<Point>,
    /// Starting grid position of the drag
    pub start_grid: Point,
    /// Current grid position during drag
    pub current_grid: Point,
    /// When true, move ALL selected items (not just the one clicked)
    pub multi_selection: bool,
}

impl DragState {
    /// Create a new drag state for a component
    pub fn component(id: u64, start: Point) -> Self {
        Self {
            active: true,
            component_id: Some(id),
            start_grid: start,
            current_grid: start,
            ..Default::default()
        }
    }

    /// Create a new drag state for a wire
    pub fn wire(id: u64, start: Point) -> Self {
        Self {
            active: true,
            wire_id: Some(id),
            start_grid: start,
            current_grid: start,
            ..Default::default()
        }
    }

    /// Create a new drag state for a junction
    pub fn junction(point: Point) -> Self {
        Self {
            active: true,
            junction_point: Some(point),
            start_grid: point,
            current_grid: point,
            ..Default::default()
        }
    }

    /// Create a multi-selection drag
    pub fn multi_select(start: Point) -> Self {
        Self {
            active: true,
            multi_selection: true,
            start_grid: start,
            current_grid: start,
            ..Default::default()
        }
    }

    /// Calculate the delta from start to current position
    pub fn delta(&self) -> Point {
        Point::new(
            self.current_grid.x - self.start_grid.x,
            self.current_grid.y - self.start_grid.y,
        )
    }

    /// Check if this is dragging a specific component
    pub fn is_dragging_component(&self, id: u64) -> bool {
        self.active && self.component_id == Some(id)
    }

    /// Check if this is dragging a specific wire
    pub fn is_dragging_wire(&self, id: u64) -> bool {
        self.active && self.wire_id == Some(id)
    }
}

// =============================================================================
// Context Menu State
// =============================================================================

/// Context menu state for right-click menus
#[derive(Clone, Default)]
pub struct ContextMenuState {
    /// Whether the context menu is visible
    pub visible: bool,
    /// Screen position of the menu (x, y in pixels)
    pub position: (f64, f64),
    /// Component targeted by the menu (if any)
    pub target_component: Option<u64>,
    /// Wire targeted by the menu (if any)
    pub target_wire: Option<u64>,
}

impl ContextMenuState {
    /// Show context menu for a component
    pub fn for_component(id: u64, x: f64, y: f64) -> Self {
        Self {
            visible: true,
            position: (x, y),
            target_component: Some(id),
            target_wire: None,
        }
    }

    /// Show context menu for a wire
    pub fn for_wire(id: u64, x: f64, y: f64) -> Self {
        Self {
            visible: true,
            position: (x, y),
            target_component: None,
            target_wire: Some(id),
        }
    }

    /// Show context menu for canvas (no specific target)
    pub fn for_canvas(x: f64, y: f64) -> Self {
        Self {
            visible: true,
            position: (x, y),
            target_component: None,
            target_wire: None,
        }
    }

    /// Hide the context menu
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Check if menu is for a component
    pub fn has_component_target(&self) -> bool {
        self.target_component.is_some()
    }

    /// Check if menu is for a wire
    pub fn has_wire_target(&self) -> bool {
        self.target_wire.is_some()
    }
}

// =============================================================================
// Box Selection State
// =============================================================================

/// Rubber-band box selection state for selecting multiple items by dragging
#[derive(Clone, Copy, PartialEq, Default)]
pub struct BoxSelectionState {
    /// Whether we're currently drawing a selection box
    pub active: bool,
    /// Start point in grid coordinates
    pub start_grid: Point,
    /// Current end point in grid coordinates
    pub end_grid: Point,
    /// Start point in pixel coordinates (for rendering)
    pub start_px: (f64, f64),
    /// Current end point in pixel coordinates
    pub end_px: (f64, f64),
    /// Set to true when box selection just completed (prevents onclick from clearing selection)
    pub just_completed: bool,
}

impl BoxSelectionState {
    /// Start a new box selection
    pub fn start(grid: Point, px: (f64, f64)) -> Self {
        Self {
            active: true,
            start_grid: grid,
            end_grid: grid,
            start_px: px,
            end_px: px,
            just_completed: false,
        }
    }

    /// Update the current end position
    pub fn update(&mut self, grid: Point, px: (f64, f64)) {
        self.end_grid = grid;
        self.end_px = px;
    }

    /// Complete the box selection
    pub fn complete(&mut self) {
        self.active = false;
        self.just_completed = true;
    }

    /// Get the bounding box in grid coordinates (min_x, min_y, max_x, max_y)
    pub fn bounds(&self) -> (i32, i32, i32, i32) {
        let min_x = self.start_grid.x.min(self.end_grid.x);
        let max_x = self.start_grid.x.max(self.end_grid.x);
        let min_y = self.start_grid.y.min(self.end_grid.y);
        let max_y = self.start_grid.y.max(self.end_grid.y);
        (min_x, min_y, max_x, max_y)
    }

    /// Get the rendering bounds in pixels (left, top, width, height)
    pub fn render_bounds(&self) -> (f64, f64, f64, f64) {
        let left = self.start_px.0.min(self.end_px.0);
        let top = self.start_px.1.min(self.end_px.1);
        let width = (self.end_px.0 - self.start_px.0).abs();
        let height = (self.end_px.1 - self.start_px.1).abs();
        (left, top, width, height)
    }

    /// Check if a point is inside the selection box
    pub fn contains(&self, point: Point) -> bool {
        let (min_x, min_y, max_x, max_y) = self.bounds();
        point.x >= min_x && point.x <= max_x && point.y >= min_y && point.y <= max_y
    }
}

// =============================================================================
// Editing State
// =============================================================================

/// Component editing state for the properties dialog
#[derive(Clone, Default)]
pub struct EditingState {
    /// Component being edited
    pub component_id: Option<u64>,
    /// Screen position for popup
    pub position: (f64, f64),
}

impl EditingState {
    /// Start editing a component
    pub fn start(id: u64, x: f64, y: f64) -> Self {
        Self {
            component_id: Some(id),
            position: (x, y),
        }
    }

    /// Check if editing is active
    pub fn is_active(&self) -> bool {
        self.component_id.is_some()
    }

    /// Close the editor
    pub fn close(&mut self) {
        self.component_id = None;
    }
}

// =============================================================================
// Label Drag State
// =============================================================================

/// Label drag state for interactive repositioning of component labels
#[derive(Clone, Copy, PartialEq, Default)]
pub struct LabelDragState {
    /// Whether a label drag is in progress
    pub active: bool,
    /// ID of the component whose label is being dragged
    pub component_id: u64,
    /// True = dragging name label, False = dragging value label
    pub is_name_label: bool,
    /// Starting offset (in pixels, relative to component center)
    pub start_offset: (f64, f64),
    /// Current offset during drag (in pixels)
    pub current_offset: (f64, f64),
}

impl LabelDragState {
    /// Start dragging a name label
    pub fn name(component_id: u64, offset: (f64, f64)) -> Self {
        Self {
            active: true,
            component_id,
            is_name_label: true,
            start_offset: offset,
            current_offset: offset,
        }
    }

    /// Start dragging a value label
    pub fn value(component_id: u64, offset: (f64, f64)) -> Self {
        Self {
            active: true,
            component_id,
            is_name_label: false,
            start_offset: offset,
            current_offset: offset,
        }
    }

    /// Update the current offset during drag
    pub fn update(&mut self, offset: (f64, f64)) {
        self.current_offset = offset;
    }

    /// Calculate the delta from start to current
    pub fn delta(&self) -> (f64, f64) {
        (
            self.current_offset.0 - self.start_offset.0,
            self.current_offset.1 - self.start_offset.1,
        )
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drag_state_component() {
        let drag = DragState::component(42, Point::new(10, 20));
        assert!(drag.active);
        assert_eq!(drag.component_id, Some(42));
        assert!(drag.is_dragging_component(42));
        assert!(!drag.is_dragging_component(99));
    }

    #[test]
    fn test_drag_state_delta() {
        let mut drag = DragState::component(1, Point::new(0, 0));
        drag.current_grid = Point::new(5, -3);
        let delta = drag.delta();
        assert_eq!(delta.x, 5);
        assert_eq!(delta.y, -3);
    }

    #[test]
    fn test_context_menu_for_component() {
        let menu = ContextMenuState::for_component(123, 100.0, 200.0);
        assert!(menu.visible);
        assert_eq!(menu.position, (100.0, 200.0));
        assert!(menu.has_component_target());
        assert!(!menu.has_wire_target());
    }

    #[test]
    fn test_box_selection_bounds() {
        let sel = BoxSelectionState {
            active: true,
            start_grid: Point::new(10, 5),
            end_grid: Point::new(2, 15),
            start_px: (0.0, 0.0),
            end_px: (0.0, 0.0),
            just_completed: false,
        };
        let (min_x, min_y, max_x, max_y) = sel.bounds();
        assert_eq!(min_x, 2);
        assert_eq!(min_y, 5);
        assert_eq!(max_x, 10);
        assert_eq!(max_y, 15);
    }

    #[test]
    fn test_box_selection_contains() {
        let sel = BoxSelectionState {
            active: true,
            start_grid: Point::new(0, 0),
            end_grid: Point::new(10, 10),
            start_px: (0.0, 0.0),
            end_px: (100.0, 100.0),
            just_completed: false,
        };
        assert!(sel.contains(Point::new(5, 5)));
        assert!(sel.contains(Point::new(0, 0)));
        assert!(sel.contains(Point::new(10, 10)));
        assert!(!sel.contains(Point::new(11, 5)));
        assert!(!sel.contains(Point::new(-1, 5)));
    }

    #[test]
    fn test_editing_state() {
        let edit = EditingState::start(42, 300.0, 400.0);
        assert!(edit.is_active());
        assert_eq!(edit.component_id, Some(42));
    }

    #[test]
    fn test_label_drag_delta() {
        let mut drag = LabelDragState::name(1, (10.0, 20.0));
        drag.update((25.0, 15.0));
        let (dx, dy) = drag.delta();
        assert_eq!(dx, 15.0);
        assert_eq!(dy, -5.0);
    }
}
