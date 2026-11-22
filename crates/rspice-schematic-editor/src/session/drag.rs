//! Selection-drag presentation types; transaction ownership stays with the host.

use rspice_design::schematic::label_position::InstanceText;

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
    /// Moving one part's name or value, which leaves the part where it is.
    MoveText(u64, InstanceText),
}
