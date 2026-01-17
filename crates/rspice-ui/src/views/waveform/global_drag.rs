//! Global Drag Context
//!
//! Provides a shared drag state that allows parent containers to capture
//! mouse events for smooth panel dragging.

use dioxus::prelude::*;

/// Active drag state - which panel is being dragged and its offset
#[derive(Clone, Default)]
pub struct GlobalDragState {
    /// Which panel is being dragged (by ID)
    pub active_panel: Option<String>,
    /// Mouse offset from panel position when drag started
    pub offset_x: i32,
    pub offset_y: i32,
}

/// Context for global drag coordination
/// Panels call start_drag when header is clicked
/// Parent calls handle_move and handle_up to update position
pub type GlobalDrag = Signal<GlobalDragState>;

/// Create a new global drag context
pub fn use_global_drag() -> GlobalDrag {
    use_signal(GlobalDragState::default)
}
