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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_net_label_new() {
        let label = NetLabel::new(1, Point::new(10, 20), "VCC");
        assert_eq!(label.id, 1);
        assert_eq!(label.pos, Point::new(10, 20));
        assert_eq!(label.name, "VCC");
    }

    #[test]
    fn test_net_label_is_power_net() {
        assert!(NetLabel::new(1, Point::origin(), "VCC").is_power_net());
        assert!(NetLabel::new(1, Point::origin(), "VDD").is_power_net());
        assert!(NetLabel::new(1, Point::origin(), "GND").is_power_net());
        assert!(NetLabel::new(1, Point::origin(), "vcc").is_power_net()); // Case insensitive
        assert!(!NetLabel::new(1, Point::origin(), "OUT").is_power_net());
        assert!(!NetLabel::new(1, Point::origin(), "CLK").is_power_net());
    }

    #[test]
    fn test_net_label_is_ground() {
        assert!(NetLabel::new(1, Point::origin(), "GND").is_ground());
        assert!(NetLabel::new(1, Point::origin(), "GROUND").is_ground());
        assert!(NetLabel::new(1, Point::origin(), "0").is_ground());
        assert!(NetLabel::new(1, Point::origin(), "VSS").is_ground());
        assert!(NetLabel::new(1, Point::origin(), "gnd").is_ground()); // Case insensitive
        assert!(!NetLabel::new(1, Point::origin(), "VCC").is_ground());
        assert!(!NetLabel::new(1, Point::origin(), "VDD").is_ground());
    }

    #[test]
    fn test_junction_new() {
        let junc = Junction::new(5, Point::new(15, 25));
        assert_eq!(junc.id, 5);
        assert_eq!(junc.pos, Point::new(15, 25));
    }

    #[test]
    fn test_junction_equality() {
        let j1 = Junction::new(1, Point::new(10, 20));
        let j2 = Junction::new(1, Point::new(10, 20));
        let j3 = Junction::new(2, Point::new(10, 20));

        assert_eq!(j1, j2);
        assert_ne!(j1, j3); // Different IDs
    }
}
