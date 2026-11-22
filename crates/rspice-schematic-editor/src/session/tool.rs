//! Interaction Tool
//!
//! Current tool mode for schematic interaction.

use rspice_design::schematic::component_type::ComponentType;
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

    /// Move the frozen current selection using the transaction configured by
    /// the mockup-owned Move selection workflow.
    MoveSelection,

    /// Stretch one selected conductor segment or documentation control point
    /// while keeping every unaffected anchor fixed.
    StretchSelection,

    /// Create a deterministic repeated structure from the frozen current
    /// selection using the mockup-owned array workflow.
    ArraySelection,

    /// Draw wires
    ///
    /// Click to start/extend wire, then double-click or press Enter to finish.
    Wire,

    /// Draw a multi-conductor bus polyline.
    Bus,

    /// Place a typed scalar or slice tap on a declared bus.
    BusTap,

    /// Place an explicit electrical junction at a conductor intersection.
    Junction,

    /// Place a durable non-electrical documentation object.
    DesignNote,

    /// Draw durable non-electrical documentation geometry.
    DocumentationShape,

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

    /// Place a net label that declares its name continues on another sheet.
    ///
    /// Click on the attachment point, then name the net and its direction.
    OffSheetConnector,

    /// Mark a pin as meant to stay open, or clear its mark.
    ///
    /// Click a pin's end to toggle its no-connect mark.
    NoConnect,
}

impl Tool {
    /// Get human-readable name for the tool
    pub fn display_name(&self) -> &'static str {
        match self {
            Tool::Select => "Select",
            Tool::MoveSelection => "Move selection",
            Tool::StretchSelection => "Stretch selection",
            Tool::ArraySelection => "Create array",
            Tool::Wire => "Wire",
            Tool::Bus => "Bus",
            Tool::BusTap => "Bus tap",
            Tool::Junction => "Junction",
            Tool::DesignNote => "Text / note",
            Tool::DocumentationShape => "Documentation shape",
            Tool::Place(kind) => kind.display_name(),
            Tool::Probe => "Probe",
            Tool::Label => "Label",
            Tool::OffSheetConnector => "Off-sheet connector",
            Tool::NoConnect => "No-connect",
        }
    }

    /// Get keyboard shortcut for this tool (if any)
    pub fn shortcut(&self) -> Option<char> {
        match self {
            Tool::Select => None,
            Tool::MoveSelection => Some('m'),
            Tool::StretchSelection => Some('s'),
            Tool::ArraySelection => None,
            Tool::Wire => Some('w'),
            Tool::Bus => Some('b'),
            Tool::BusTap => Some('t'),
            Tool::Junction => Some('j'),
            Tool::DesignNote => None,
            Tool::DocumentationShape => None,
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
            // Every plausible mnemonic is already owned by another tool.
            Tool::OffSheetConnector => None,
            Tool::NoConnect => Some('x'),
            _ => None,
        }
    }

    /// Check if this tool places a durable schematic object.
    pub fn is_place_tool(&self) -> bool {
        matches!(
            self,
            Tool::Place(_)
                | Tool::Junction
                | Tool::BusTap
                | Tool::DesignNote
                | Tool::DocumentationShape
        )
    }

    /// The tool's own canvas cursor. Select keeps the arrow, since the hover
    /// pre-highlight already says what a click takes; Move and Stretch show
    /// the move cross; every tool that places or draws at a point, labels and
    /// notes included, shows the crosshair, whose centre is where the click
    /// lands (a placement ghost hangs from it).
    pub const fn cursor(&self) -> egui::CursorIcon {
        match self {
            Tool::Select => egui::CursorIcon::Default,
            Tool::MoveSelection | Tool::StretchSelection => egui::CursorIcon::Move,
            Tool::ArraySelection
            | Tool::Wire
            | Tool::Bus
            | Tool::BusTap
            | Tool::Junction
            | Tool::DesignNote
            | Tool::DocumentationShape
            | Tool::Place(_)
            | Tool::Probe
            | Tool::Label
            | Tool::OffSheetConnector
            | Tool::NoConnect => egui::CursorIcon::Crosshair,
        }
    }
}

/// What the canvas is doing this frame, for its cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanvasCursorContext {
    /// A middle drag, or a Space or Alt drag, is panning.
    pub panning: bool,
    /// Space or Alt is held with no button down: a drag would pan.
    pub pan_armed: bool,
    /// The zoom box is armed or being dragged.
    pub zoom_box: bool,
    /// A shelf drag is over the canvas: whether its drop would be accepted.
    pub shelf_drop: Option<bool>,
    /// The selection is being dragged.
    pub moving_selection: bool,
    pub tool: Tool,
}

impl CanvasCursorContext {
    /// The canvas cursor: the first that applies of panning, pan armed, zoom
    /// box, shelf drop, moving the selection, and the tool's own cursor.
    pub const fn cursor(&self) -> egui::CursorIcon {
        if self.panning {
            egui::CursorIcon::Grabbing
        } else if self.pan_armed {
            egui::CursorIcon::Grab
        } else if self.zoom_box {
            egui::CursorIcon::ZoomIn
        } else if let Some(accepted) = self.shelf_drop {
            if accepted {
                egui::CursorIcon::Crosshair
            } else {
                egui::CursorIcon::NoDrop
            }
        } else if self.moving_selection {
            egui::CursorIcon::Move
        } else {
            self.tool.cursor()
        }
    }
}

// =============================================================================
// Tests
// =============================================================================
