//! Selection-drag presentation types; transaction ownership stays with the host.

use serde::{Deserialize, Serialize};

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
