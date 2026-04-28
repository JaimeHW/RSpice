//! Interaction Tool
//!
//! Current tool mode for schematic interaction.

use super::component_type::ComponentType;
use serde::{Deserialize, Serialize};

// =============================================================================
// Tool
// =============================================================================

/// Current interaction tool
///
/// Determines how mouse events are interpreted in the schematic editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Tool {
    /// Select and move components/wires
    ///
    /// Default mode. Click to select, drag to move, box select for multiple.
    #[default]
    Select,

    /// Draw wires
    ///
    /// Click to start/extend wire, right-click or Escape to finish.
    Wire,

    /// Place a specific component type
    ///
    /// Click to place component at cursor position. Press 'R' to rotate.
    Place(ComponentType),

    /// Probe voltage/current at nodes
    ///
    /// Click on node to add voltage probe, click on component for current.
    Probe,

    /// Place net labels
    ///
    /// Click on wire to add a net name label.
    Label,
}

impl Tool {
    /// Get human-readable name for the tool
    pub fn display_name(&self) -> &'static str {
        match self {
            Tool::Select => "Select",
            Tool::Wire => "Wire",
            Tool::Place(kind) => kind.display_name(),
            Tool::Probe => "Probe",
            Tool::Label => "Label",
        }
    }

    /// Get keyboard shortcut for this tool (if any)
    pub fn shortcut(&self) -> Option<char> {
        match self {
            Tool::Select => Some('s'),
            Tool::Wire => Some('w'),
            Tool::Place(ComponentType::Resistor) => None, // 'r' is for rotate
            Tool::Place(ComponentType::Capacitor) => Some('c'),
            Tool::Place(ComponentType::Inductor) => Some('l'),
            Tool::Place(ComponentType::Diode) => Some('d'),
            Tool::Place(ComponentType::Ground) => Some('g'),
            Tool::Place(ComponentType::VoltageSource) => Some('v'),
            Tool::Place(ComponentType::CurrentSource) => Some('i'),
            Tool::Place(ComponentType::NpnBjt) => Some('q'),
            Tool::Place(ComponentType::Nmos) => Some('m'),
            Tool::Probe => Some('p'),
            Tool::Label => None,
            _ => None,
        }
    }

    /// Check if this tool places components
    pub fn is_place_tool(&self) -> bool {
        matches!(self, Tool::Place(_))
    }

    /// Check if this is the select tool
    pub fn is_select(&self) -> bool {
        matches!(self, Tool::Select)
    }

    /// Check if this is the wire tool
    pub fn is_wire(&self) -> bool {
        matches!(self, Tool::Wire)
    }

    /// Get the component type being placed (if any)
    pub fn placing(&self) -> Option<ComponentType> {
        match self {
            Tool::Place(kind) => Some(*kind),
            _ => None,
        }
    }

    /// Get CSS cursor style for this tool
    pub fn cursor(&self) -> &'static str {
        match self {
            Tool::Select => "default",
            Tool::Wire => "crosshair",
            Tool::Place(_) => "copy",
            Tool::Probe => "crosshair",
            Tool::Label => "text",
        }
    }
}

// =============================================================================
// Tests
// =============================================================================
