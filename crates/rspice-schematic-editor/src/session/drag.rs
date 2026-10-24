//! Selection-drag presentation types; transaction ownership stays with the host.

/// Type of drag operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DragType {
    /// No drag operation
    #[default]
    None,
    /// Moving selected components/wires
    MoveSelection,
    /// Dragging a wire vertex
    WireVertex,
}
