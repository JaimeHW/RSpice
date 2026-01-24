//! Keyboard Handlers for Schematic Editor
//!
//! This module provides key binding mappings and keyboard event handling logic
//! for the schematic editor. The design separates the pure key mapping logic
//! (which can be unit tested) from the actual event handler closures (which
//! must be defined inline in the component due to signal capture requirements).
//!
//! ## Architecture
//!
//! Commercial CAD tools like Cadence Virtuoso use a layered approach:
//! 1. Key bindings defined as a testable data structure
//! 2. Key-to-action mapping logic that's pure and testable
//! 3. Action execution in the UI layer with access to state
//!
//! This module implements layers 1 and 2.

use crate::state::{ComponentType, Tool};

/// Keyboard action that can be triggered by a key press
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardAction {
    /// Switch to select tool
    SelectTool,
    /// Switch to wire drawing mode
    WireTool,
    /// Toggle wire routing mode (horizontal-first vs vertical-first)
    ToggleWireRouting,
    /// Delete selected items
    Delete,
    /// Rotate selection or preview rotation
    Rotate,
    /// Cancel current operation (return to select mode)
    Cancel,
    /// Place a specific component type
    PlaceComponent(ComponentType),
    /// Undo last action (Ctrl+Z)
    Undo,
    /// Redo last undone action (Ctrl+Y)
    Redo,
    /// Copy selection (Ctrl+C)
    Copy,
    /// Paste clipboard at cursor (Ctrl+V)
    Paste,
}

/// Maps a key character to a keyboard action
///
/// This is the core key binding logic, designed to be testable
/// without any UI dependencies.
///
/// # Arguments
/// * `key` - The key character (lowercase or uppercase)
/// * `has_ctrl` - Whether Ctrl modifier is pressed
/// * `is_wire_drawing` - Whether currently drawing a wire
///
/// # Returns
/// The corresponding action, or None if no binding exists
pub fn map_key_to_action(
    key: &str,
    has_ctrl: bool,
    is_wire_drawing: bool,
) -> Option<KeyboardAction> {
    // Ctrl+key shortcuts take priority
    if has_ctrl {
        return match key.to_lowercase().as_str() {
            "z" => Some(KeyboardAction::Undo),
            "y" => Some(KeyboardAction::Redo),
            "c" => Some(KeyboardAction::Copy),
            "v" => Some(KeyboardAction::Paste),
            _ => None,
        };
    }

    // Handle character keys without modifiers
    match key.to_lowercase().as_str() {
        // Tool switching
        "s" => Some(KeyboardAction::SelectTool),
        "w" => Some(KeyboardAction::WireTool),

        // Actions
        " " if is_wire_drawing => Some(KeyboardAction::ToggleWireRouting),
        "r" => Some(KeyboardAction::Rotate),

        // Component placement shortcuts
        "g" => Some(KeyboardAction::PlaceComponent(ComponentType::Ground)),
        "v" => Some(KeyboardAction::PlaceComponent(ComponentType::VoltageSource)),
        "i" => Some(KeyboardAction::PlaceComponent(ComponentType::CurrentSource)),
        "c" => Some(KeyboardAction::PlaceComponent(ComponentType::Capacitor)),
        "l" => Some(KeyboardAction::PlaceComponent(ComponentType::Inductor)),
        "d" => Some(KeyboardAction::PlaceComponent(ComponentType::Diode)),
        "q" => Some(KeyboardAction::PlaceComponent(ComponentType::NpnBjt)),
        "m" => Some(KeyboardAction::PlaceComponent(ComponentType::Nmos)),

        _ => None,
    }
}

/// Maps a special key (Escape, Delete, etc.) to an action
pub fn map_special_key_to_action(key: SpecialKey) -> Option<KeyboardAction> {
    match key {
        SpecialKey::Escape => Some(KeyboardAction::Cancel),
        SpecialKey::Delete => Some(KeyboardAction::Delete),
        SpecialKey::Backspace => Some(KeyboardAction::Delete),
    }
}

/// Special keys that trigger actions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialKey {
    Escape,
    Delete,
    Backspace,
}

/// Keyboard binding definition for documentation and configuration
#[derive(Debug, Clone)]
pub struct KeyBinding {
    /// The key or key combination (e.g., "Ctrl+Z", "R", "Escape")
    pub key: &'static str,
    /// Human-readable description of the action
    pub description: &'static str,
    /// The action this binding triggers
    pub action: KeyboardAction,
}

/// Returns all keyboard bindings for the schematic editor
///
/// This is useful for:
/// - Generating help text
/// - Displaying keyboard shortcuts in tooltips
/// - Configuration dialogs
pub fn get_all_bindings() -> Vec<KeyBinding> {
    vec![
        // Ctrl+key shortcuts
        KeyBinding {
            key: "Ctrl+Z",
            description: "Undo last action",
            action: KeyboardAction::Undo,
        },
        KeyBinding {
            key: "Ctrl+Y",
            description: "Redo undone action",
            action: KeyboardAction::Redo,
        },
        KeyBinding {
            key: "Ctrl+C",
            description: "Copy selection",
            action: KeyboardAction::Copy,
        },
        KeyBinding {
            key: "Ctrl+V",
            description: "Paste at cursor",
            action: KeyboardAction::Paste,
        },
        // Tool shortcuts
        KeyBinding {
            key: "S",
            description: "Select tool",
            action: KeyboardAction::SelectTool,
        },
        KeyBinding {
            key: "W",
            description: "Wire tool",
            action: KeyboardAction::WireTool,
        },
        KeyBinding {
            key: "Space",
            description: "Toggle wire routing",
            action: KeyboardAction::ToggleWireRouting,
        },
        // Actions
        KeyBinding {
            key: "R",
            description: "Rotate selection/preview",
            action: KeyboardAction::Rotate,
        },
        KeyBinding {
            key: "Delete",
            description: "Delete selection",
            action: KeyboardAction::Delete,
        },
        KeyBinding {
            key: "Escape",
            description: "Cancel operation",
            action: KeyboardAction::Cancel,
        },
        // Component placement
        KeyBinding {
            key: "G",
            description: "Place ground",
            action: KeyboardAction::PlaceComponent(ComponentType::Ground),
        },
        KeyBinding {
            key: "V",
            description: "Place voltage source",
            action: KeyboardAction::PlaceComponent(ComponentType::VoltageSource),
        },
        KeyBinding {
            key: "I",
            description: "Place current source",
            action: KeyboardAction::PlaceComponent(ComponentType::CurrentSource),
        },
        KeyBinding {
            key: "C",
            description: "Place capacitor",
            action: KeyboardAction::PlaceComponent(ComponentType::Capacitor),
        },
        KeyBinding {
            key: "L",
            description: "Place inductor",
            action: KeyboardAction::PlaceComponent(ComponentType::Inductor),
        },
        KeyBinding {
            key: "D",
            description: "Place diode",
            action: KeyboardAction::PlaceComponent(ComponentType::Diode),
        },
        KeyBinding {
            key: "Q",
            description: "Place NPN BJT",
            action: KeyboardAction::PlaceComponent(ComponentType::NpnBjt),
        },
        KeyBinding {
            key: "M",
            description: "Place NMOS",
            action: KeyboardAction::PlaceComponent(ComponentType::Nmos),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // =============================================================================
    // Ctrl+Key Shortcuts
    // =============================================================================

    #[test]
    fn test_ctrl_z_undo() {
        let action = map_key_to_action("z", true, false);
        assert_eq!(action, Some(KeyboardAction::Undo));
    }

    #[test]
    fn test_ctrl_z_uppercase_undo() {
        let action = map_key_to_action("Z", true, false);
        assert_eq!(action, Some(KeyboardAction::Undo));
    }

    #[test]
    fn test_ctrl_y_redo() {
        let action = map_key_to_action("y", true, false);
        assert_eq!(action, Some(KeyboardAction::Redo));
    }

    #[test]
    fn test_ctrl_c_copy() {
        let action = map_key_to_action("c", true, false);
        assert_eq!(action, Some(KeyboardAction::Copy));
    }

    #[test]
    fn test_ctrl_v_paste() {
        let action = map_key_to_action("v", true, false);
        assert_eq!(action, Some(KeyboardAction::Paste));
    }

    #[test]
    fn test_ctrl_unknown_key() {
        let action = map_key_to_action("x", true, false);
        assert_eq!(action, None);
    }

    // =============================================================================
    // Tool Switching
    // =============================================================================

    #[test]
    fn test_s_select_tool() {
        let action = map_key_to_action("s", false, false);
        assert_eq!(action, Some(KeyboardAction::SelectTool));
    }

    #[test]
    fn test_s_uppercase_select_tool() {
        let action = map_key_to_action("S", false, false);
        assert_eq!(action, Some(KeyboardAction::SelectTool));
    }

    #[test]
    fn test_w_wire_tool() {
        let action = map_key_to_action("w", false, false);
        assert_eq!(action, Some(KeyboardAction::WireTool));
    }

    #[test]
    fn test_w_uppercase_wire_tool() {
        let action = map_key_to_action("W", false, false);
        assert_eq!(action, Some(KeyboardAction::WireTool));
    }

    // =============================================================================
    // Wire Routing
    // =============================================================================

    #[test]
    fn test_space_toggle_routing_when_drawing() {
        let action = map_key_to_action(" ", false, true);
        assert_eq!(action, Some(KeyboardAction::ToggleWireRouting));
    }

    #[test]
    fn test_space_no_action_when_not_drawing() {
        let action = map_key_to_action(" ", false, false);
        assert_eq!(action, None);
    }

    // =============================================================================
    // Rotation
    // =============================================================================

    #[test]
    fn test_r_rotate() {
        let action = map_key_to_action("r", false, false);
        assert_eq!(action, Some(KeyboardAction::Rotate));
    }

    #[test]
    fn test_r_uppercase_rotate() {
        let action = map_key_to_action("R", false, false);
        assert_eq!(action, Some(KeyboardAction::Rotate));
    }

    // =============================================================================
    // Component Placement
    // =============================================================================

    #[test]
    fn test_g_ground() {
        let action = map_key_to_action("g", false, false);
        assert_eq!(
            action,
            Some(KeyboardAction::PlaceComponent(ComponentType::Ground))
        );
    }

    #[test]
    fn test_v_voltage_source() {
        let action = map_key_to_action("v", false, false);
        assert_eq!(
            action,
            Some(KeyboardAction::PlaceComponent(ComponentType::VoltageSource))
        );
    }

    #[test]
    fn test_i_current_source() {
        let action = map_key_to_action("i", false, false);
        assert_eq!(
            action,
            Some(KeyboardAction::PlaceComponent(ComponentType::CurrentSource))
        );
    }

    #[test]
    fn test_c_capacitor() {
        let action = map_key_to_action("c", false, false);
        assert_eq!(
            action,
            Some(KeyboardAction::PlaceComponent(ComponentType::Capacitor))
        );
    }

    #[test]
    fn test_l_inductor() {
        let action = map_key_to_action("l", false, false);
        assert_eq!(
            action,
            Some(KeyboardAction::PlaceComponent(ComponentType::Inductor))
        );
    }

    #[test]
    fn test_d_diode() {
        let action = map_key_to_action("d", false, false);
        assert_eq!(
            action,
            Some(KeyboardAction::PlaceComponent(ComponentType::Diode))
        );
    }

    #[test]
    fn test_q_npn_bjt() {
        let action = map_key_to_action("q", false, false);
        assert_eq!(
            action,
            Some(KeyboardAction::PlaceComponent(ComponentType::NpnBjt))
        );
    }

    #[test]
    fn test_m_nmos() {
        let action = map_key_to_action("m", false, false);
        assert_eq!(
            action,
            Some(KeyboardAction::PlaceComponent(ComponentType::Nmos))
        );
    }

    // =============================================================================
    // Special Keys
    // =============================================================================

    #[test]
    fn test_escape_cancel() {
        let action = map_special_key_to_action(SpecialKey::Escape);
        assert_eq!(action, Some(KeyboardAction::Cancel));
    }

    #[test]
    fn test_delete_key() {
        let action = map_special_key_to_action(SpecialKey::Delete);
        assert_eq!(action, Some(KeyboardAction::Delete));
    }

    #[test]
    fn test_backspace_delete() {
        let action = map_special_key_to_action(SpecialKey::Backspace);
        assert_eq!(action, Some(KeyboardAction::Delete));
    }

    // =============================================================================
    // Unknown Keys
    // =============================================================================

    #[test]
    fn test_unknown_key_returns_none() {
        let action = map_key_to_action("x", false, false);
        assert_eq!(action, None);
    }

    #[test]
    fn test_number_key_returns_none() {
        let action = map_key_to_action("1", false, false);
        assert_eq!(action, None);
    }

    // =============================================================================
    // Binding List
    // =============================================================================

    #[test]
    fn test_all_bindings_exist() {
        let bindings = get_all_bindings();
        assert!(bindings.len() >= 15, "Should have at least 15 bindings");
    }

    #[test]
    fn test_bindings_have_descriptions() {
        let bindings = get_all_bindings();
        for binding in bindings {
            assert!(
                !binding.description.is_empty(),
                "Binding for {} should have description",
                binding.key
            );
        }
    }

    #[test]
    fn test_bindings_have_unique_keys() {
        let bindings = get_all_bindings();
        let mut keys: Vec<&str> = bindings.iter().map(|b| b.key).collect();
        keys.sort();
        let original_len = keys.len();
        keys.dedup();
        assert_eq!(
            keys.len(),
            original_len,
            "All binding keys should be unique"
        );
    }
}
