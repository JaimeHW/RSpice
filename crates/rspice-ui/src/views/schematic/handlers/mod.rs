//! Event Handlers for Schematic Editor
//!
//! This module provides testable event handling logic for the schematic editor.
//!
//! ## Design Philosophy
//!
//! Commercial-grade CAD tools separate event handling into layers:
//! 1. **Key/Action Mapping** - Pure functions mapping inputs to actions (fully testable)
//! 2. **Coordinate Conversion** - Screen to grid coordinate transformation
//! 3. **Hit Detection** - Find elements at cursor position
//! 4. **Net Tracing** - Flood-fill connected wire nets
//! 5. **Zoom Calculation** - Cursor-centered zoom with proper pan adjustment
//!
//! This module implements all testable logic, keeping the logic pure while
//! the actual signal mutations remain in the Dioxus components.

pub mod keyboard;
pub mod mouse;
pub mod wheel;

// Re-export keyboard types
pub use keyboard::{get_all_bindings, map_key_to_action, map_special_key_to_action};
pub use keyboard::{KeyBinding, KeyboardAction, SpecialKey};

// Re-export mouse types and functions
pub use mouse::{calculate_drag_delta, grid_to_screen, screen_to_grid, HitTestResult};
pub use mouse::{cursor_for_context, find_all_junctions, trace_connected_net};

// Re-export wheel types and functions
pub use wheel::{calculate_cursor_centered_zoom, calculate_fit_to_view, ZoomConfig};
pub use wheel::{next_zoom_level, ZOOM_LEVELS};
