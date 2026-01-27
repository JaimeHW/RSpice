//! Interaction State
//!
//! Runtime interaction state for UI operations like dragging,
//! selection, and other transient user interactions.

use serde::{Deserialize, Serialize};

// =============================================================================
// Interaction State
// =============================================================================

/// Runtime interaction state for UI operations
///
/// This captures transient UI state that doesn't belong in persistent
/// dialog configuration (like analysis parameters) but is needed
/// for smooth user interactions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InteractionState {
    /// Current drag operation state
    pub drag: DragState,

    /// Component being hovered
    pub hover_component_id: Option<u64>,

    /// Wire being hovered
    pub hover_wire_id: Option<u64>,

    /// Last click position in grid coordinates
    pub last_click_pos: Option<(i32, i32)>,
}

impl InteractionState {
    /// Create a new interaction state
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any drag operation is in progress
    pub fn is_dragging(&self) -> bool {
        self.drag.is_active()
    }

    /// Clear all transient state
    pub fn clear(&mut self) {
        self.drag.cancel();
        self.hover_component_id = None;
        self.hover_wire_id = None;
        self.last_click_pos = None;
    }
}

// =============================================================================
// Drag State
// =============================================================================

/// State for drag operations (moving selection, rubber-banding)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DragState {
    /// Starting position of the drag (grid coordinates)
    pub start_pos: Option<(i32, i32)>,

    /// Last position during drag for computing delta (grid coordinates)
    pub last_pos: Option<(i32, i32)>,

    /// Type of drag operation
    pub drag_type: DragType,
}

impl DragState {
    /// Start a new drag operation
    pub fn start(&mut self, pos: (i32, i32), drag_type: DragType) {
        self.start_pos = Some(pos);
        self.last_pos = Some(pos);
        self.drag_type = drag_type;
    }

    /// Update the drag position
    pub fn update(&mut self, pos: (i32, i32)) {
        self.last_pos = Some(pos);
    }

    /// Complete the drag operation
    pub fn finish(&mut self) -> Option<DragResult> {
        if let (Some(start), Some(end)) = (self.start_pos, self.last_pos) {
            let result = DragResult {
                start,
                end,
                delta: (end.0 - start.0, end.1 - start.1),
                drag_type: self.drag_type,
            };
            self.cancel();
            Some(result)
        } else {
            self.cancel();
            None
        }
    }

    /// Cancel the drag operation
    pub fn cancel(&mut self) {
        self.start_pos = None;
        self.last_pos = None;
        self.drag_type = DragType::None;
    }

    /// Check if a drag operation is in progress
    pub fn is_active(&self) -> bool {
        self.start_pos.is_some() && self.drag_type != DragType::None
    }

    /// Get the current delta from start position
    pub fn current_delta(&self) -> Option<(i32, i32)> {
        match (self.start_pos, self.last_pos) {
            (Some(start), Some(last)) => Some((last.0 - start.0, last.1 - start.1)),
            _ => None,
        }
    }

    /// Get the delta from the last position (for incremental moves)
    pub fn delta_from_last(&self, new_pos: (i32, i32)) -> Option<(i32, i32)> {
        self.last_pos
            .map(|last| (new_pos.0 - last.0, new_pos.1 - last.1))
    }
}

// =============================================================================
// Drag Type Enum
// =============================================================================

/// Type of drag operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DragType {
    /// No drag operation
    #[default]
    None,
    /// Moving selected components/wires
    MoveSelection,
    /// Drawing a box selection rectangle
    BoxSelect,
    /// Panning the viewport
    Pan,
    /// Dragging a wire endpoint
    WireEndpoint,
    /// Dragging a wire vertex
    WireVertex,
}

impl DragType {
    /// Check if this drag type affects schematic content
    pub fn modifies_schematic(self) -> bool {
        matches!(
            self,
            Self::MoveSelection | Self::WireEndpoint | Self::WireVertex
        )
    }
}

// =============================================================================
// Drag Result
// =============================================================================

/// Result of a completed drag operation
#[derive(Debug, Clone, Copy)]
pub struct DragResult {
    /// Starting position (grid coordinates)
    pub start: (i32, i32),
    /// Ending position (grid coordinates)
    pub end: (i32, i32),
    /// Total delta from start to end
    pub delta: (i32, i32),
    /// Type of drag that was performed
    pub drag_type: DragType,
}

impl DragResult {
    /// Check if the drag resulted in any movement
    pub fn has_movement(&self) -> bool {
        self.delta.0 != 0 || self.delta.1 != 0
    }

    /// Get the bounding box of the drag (for box selection)
    pub fn bounding_box(&self) -> ((i32, i32), (i32, i32)) {
        let min_x = self.start.0.min(self.end.0);
        let min_y = self.start.1.min(self.end.1);
        let max_x = self.start.0.max(self.end.0);
        let max_y = self.start.1.max(self.end.1);
        ((min_x, min_y), (max_x, max_y))
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Drag State Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_drag_state_default_not_active() {
        let state = DragState::default();
        assert!(!state.is_active());
        assert!(state.start_pos.is_none());
        assert!(state.last_pos.is_none());
    }

    #[test]
    fn test_drag_state_start() {
        let mut state = DragState::default();
        state.start((10, 20), DragType::MoveSelection);

        assert!(state.is_active());
        assert_eq!(state.start_pos, Some((10, 20)));
        assert_eq!(state.last_pos, Some((10, 20)));
        assert_eq!(state.drag_type, DragType::MoveSelection);
    }

    #[test]
    fn test_drag_state_update() {
        let mut state = DragState::default();
        state.start((10, 20), DragType::MoveSelection);
        state.update((30, 40));

        assert_eq!(state.start_pos, Some((10, 20)));
        assert_eq!(state.last_pos, Some((30, 40)));
    }

    #[test]
    fn test_drag_state_current_delta() {
        let mut state = DragState::default();
        state.start((10, 20), DragType::MoveSelection);
        state.update((30, 50));

        let delta = state.current_delta();
        assert_eq!(delta, Some((20, 30)));
    }

    #[test]
    fn test_drag_state_delta_from_last() {
        let mut state = DragState::default();
        state.start((10, 20), DragType::MoveSelection);
        state.update((30, 40));

        let delta = state.delta_from_last((35, 45));
        assert_eq!(delta, Some((5, 5)));
    }

    #[test]
    fn test_drag_state_finish() {
        let mut state = DragState::default();
        state.start((10, 20), DragType::MoveSelection);
        state.update((50, 70));

        let result = state.finish();
        assert!(result.is_some());

        let result = result.unwrap();
        assert_eq!(result.start, (10, 20));
        assert_eq!(result.end, (50, 70));
        assert_eq!(result.delta, (40, 50));
        assert_eq!(result.drag_type, DragType::MoveSelection);

        // After finish, state should be cleared
        assert!(!state.is_active());
    }

    #[test]
    fn test_drag_state_cancel() {
        let mut state = DragState::default();
        state.start((10, 20), DragType::MoveSelection);
        state.update((30, 40));
        state.cancel();

        assert!(!state.is_active());
        assert!(state.start_pos.is_none());
        assert!(state.last_pos.is_none());
        assert_eq!(state.drag_type, DragType::None);
    }

    #[test]
    fn test_drag_state_finish_no_drag_returns_none() {
        let mut state = DragState::default();
        let result = state.finish();
        assert!(result.is_none());
    }

    // -------------------------------------------------------------------------
    // Drag Type Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_drag_type_modifies_schematic() {
        assert!(!DragType::None.modifies_schematic());
        assert!(DragType::MoveSelection.modifies_schematic());
        assert!(!DragType::BoxSelect.modifies_schematic());
        assert!(!DragType::Pan.modifies_schematic());
        assert!(DragType::WireEndpoint.modifies_schematic());
        assert!(DragType::WireVertex.modifies_schematic());
    }

    // -------------------------------------------------------------------------
    // Drag Result Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_drag_result_has_movement() {
        let result = DragResult {
            start: (0, 0),
            end: (10, 10),
            delta: (10, 10),
            drag_type: DragType::MoveSelection,
        };
        assert!(result.has_movement());

        let no_movement = DragResult {
            start: (5, 5),
            end: (5, 5),
            delta: (0, 0),
            drag_type: DragType::MoveSelection,
        };
        assert!(!no_movement.has_movement());
    }

    #[test]
    fn test_drag_result_bounding_box() {
        // Normal case: start before end
        let result = DragResult {
            start: (10, 20),
            end: (50, 60),
            delta: (40, 40),
            drag_type: DragType::BoxSelect,
        };
        let ((min_x, min_y), (max_x, max_y)) = result.bounding_box();
        assert_eq!((min_x, min_y), (10, 20));
        assert_eq!((max_x, max_y), (50, 60));

        // Reverse case: start after end
        let result2 = DragResult {
            start: (50, 60),
            end: (10, 20),
            delta: (-40, -40),
            drag_type: DragType::BoxSelect,
        };
        let ((min_x, min_y), (max_x, max_y)) = result2.bounding_box();
        assert_eq!((min_x, min_y), (10, 20));
        assert_eq!((max_x, max_y), (50, 60));
    }

    // -------------------------------------------------------------------------
    // Interaction State Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_interaction_state_default() {
        let state = InteractionState::default();
        assert!(!state.is_dragging());
        assert!(state.hover_component_id.is_none());
        assert!(state.hover_wire_id.is_none());
    }

    #[test]
    fn test_interaction_state_is_dragging() {
        let mut state = InteractionState::default();
        assert!(!state.is_dragging());

        state.drag.start((10, 10), DragType::MoveSelection);
        assert!(state.is_dragging());
    }

    #[test]
    fn test_interaction_state_clear() {
        let mut state = InteractionState::default();
        state.drag.start((10, 10), DragType::MoveSelection);
        state.hover_component_id = Some(42);
        state.hover_wire_id = Some(99);
        state.last_click_pos = Some((5, 5));

        state.clear();

        assert!(!state.is_dragging());
        assert!(state.hover_component_id.is_none());
        assert!(state.hover_wire_id.is_none());
        assert!(state.last_click_pos.is_none());
    }
}
