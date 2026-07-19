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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

    /// Validate a user-authored node name against the document's retained
    /// naming policy. This is the single authority used by editors and
    /// netlist generation, so a name accepted by the UI cannot later be
    /// rejected by the simulator pipeline.
    pub fn validate_name(
        name: &str,
        policy: super::document_policy::NetNamingPolicy,
    ) -> Result<(), &'static str> {
        if name.is_empty() {
            return Err("name is empty");
        }
        if name.chars().any(char::is_whitespace) {
            return Err("whitespace is not permitted");
        }
        if name.chars().any(char::is_control) {
            return Err("control characters are not permitted");
        }
        if policy == super::document_policy::NetNamingPolicy::StrictCaseSensitive
            && name.chars().any(|character| {
                !character.is_ascii_alphanumeric() && "_.$:/![]<>-".find(character).is_none()
            })
        {
            return Err("name contains a character outside the strict project syntax");
        }
        let opens = name
            .chars()
            .filter(|character| matches!(character, '[' | '<'))
            .count();
        let closes = name
            .chars()
            .filter(|character| matches!(character, ']' | '>'))
            .count();
        if opens != closes {
            return Err("bus delimiters are unbalanced");
        }
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::NetNamingPolicy;

    #[test]
    fn name_validation_matches_document_policy_and_spice_punctuation() {
        assert!(NetLabel::validate_name("0", NetNamingPolicy::StrictCaseSensitive).is_ok());
        assert!(NetLabel::validate_name("afe.out:3", NetNamingPolicy::StrictCaseSensitive).is_ok());
        assert!(NetLabel::validate_name("DATA[7]", NetNamingPolicy::StrictCaseSensitive).is_ok());
        assert!(
            NetLabel::validate_name("unicode_δ", NetNamingPolicy::StrictCaseSensitive).is_err()
        );
        assert!(
            NetLabel::validate_name("unicode_δ", NetNamingPolicy::SpiceCompatibleRelaxed).is_ok()
        );
        assert!(
            NetLabel::validate_name("DATA[7", NetNamingPolicy::SpiceCompatibleRelaxed).is_err()
        );
        assert!(
            NetLabel::validate_name("two nodes", NetNamingPolicy::SpiceCompatibleRelaxed).is_err()
        );
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
