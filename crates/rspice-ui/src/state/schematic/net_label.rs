//! Net Labels and Junctions
//!
//! Types for naming nets and handling wire junctions.

use super::point::Point;
use serde::{Deserialize, Serialize};

// =============================================================================
// Net Label
// =============================================================================

/// Net label for naming nodes in the schematic
///
/// Net labels assign symbolic names to circuit nodes (e.g., "VCC", "GND", "OUT").
/// In SPICE netlists, all points with the same label are electrically connected.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetLabel {
    /// Unique identifier
    pub id: u64,

    /// Position on grid
    pub pos: Point,

    /// Net name (e.g., "VCC", "GND", "OUT")
    pub name: String,
}

impl NetLabel {
    /// Create a new net label
    pub fn new(id: u64, pos: Point, name: impl Into<String>) -> Self {
        Self {
            id,
            pos,
            name: name.into(),
        }
    }

    /// Check if this label uses a predefined power net name
    pub fn is_power_net(&self) -> bool {
        let upper = self.name.to_uppercase();
        matches!(
            upper.as_str(),
            "VCC" | "VDD" | "VEE" | "VSS" | "GND" | "GROUND" | "0" | "AVDD" | "DVDD" | "AVSS"
        )
    }

    /// Check if this label is a ground reference
    pub fn is_ground(&self) -> bool {
        let upper = self.name.to_uppercase();
        matches!(upper.as_str(), "GND" | "GROUND" | "0" | "VSS" | "VEE")
    }
}

// =============================================================================
// Junction
// =============================================================================

/// Explicit wire junction point
///
/// In standard simulators, crossing wires are NOT electrically connected
/// unless an explicit junction exists. Junctions are created by:
/// - Clicking on a wire crossing point
/// - Ending a wire on an existing wire
/// - Manually placing a junction at a point where 3+ wires should meet
///
/// Junctions visually appear as small filled dots at the connection point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Junction {
    /// Unique identifier
    pub id: u64,

    /// Position on grid
    pub pos: Point,
}

impl Junction {
    /// Create a new junction
    pub fn new(id: u64, pos: Point) -> Self {
        Self { id, pos }
    }
}

// =============================================================================
// Tests
// =============================================================================
