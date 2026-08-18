//! Net Labels and Junctions
//!
//! Types for naming nets and handling wire junctions.

use super::point::Point;
use rspice_design_model::design_management::CrossSheetPortDirection;
use serde::{Deserialize, Serialize};

// =============================================================================
// Net Label
// =============================================================================

/// What a net label claims about where its name lives.
///
/// Netlisting joins labels by name and nothing else, so the kind never changes
/// connectivity. It records the author's declaration that a name is expected to
/// reappear on another sheet of the same cellview, which is what lets the canvas
/// draw a connector and lets checks demand a partner.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum NetLabelKind {
    /// A name assigned on the sheet that carries it.
    #[default]
    Local,

    /// A name declared to continue on another sheet of the same cellview.
    OffSheet { direction: CrossSheetPortDirection },
}

impl NetLabelKind {
    /// The declarable off-sheet directions, in the order every surface offers
    /// them. The enum itself belongs to the design-management model, so the
    /// product's ordering and wording for it live beside the label kind.
    pub const DIRECTIONS: [CrossSheetPortDirection; 4] = [
        CrossSheetPortDirection::Input,
        CrossSheetPortDirection::Output,
        CrossSheetPortDirection::InOut,
        CrossSheetPortDirection::Supply,
    ];

    /// Human-readable direction copy shared by the placement dialog and the
    /// canvas accessibility description.
    pub const fn direction_label(direction: CrossSheetPortDirection) -> &'static str {
        match direction {
            CrossSheetPortDirection::Input => "input",
            CrossSheetPortDirection::Output => "output",
            CrossSheetPortDirection::InOut => "bidirectional",
            CrossSheetPortDirection::Supply => "supply",
        }
    }

    /// The declared direction, or `None` for a label that makes no cross-sheet
    /// claim.
    pub const fn off_sheet_direction(self) -> Option<CrossSheetPortDirection> {
        match self {
            Self::Local => None,
            Self::OffSheet { direction } => Some(direction),
        }
    }
}

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

    /// Cross-sheet declaration. Projects written before off-sheet connectors
    /// existed carry no such field and load as ordinary local labels.
    #[serde(default)]
    pub kind: NetLabelKind,
}

impl NetLabel {
    /// Create a new net label
    pub fn new(id: u64, pos: Point, name: impl Into<String>) -> Self {
        Self {
            id,
            pos,
            name: name.into(),
            kind: NetLabelKind::Local,
        }
    }

    /// Create a net label that declares its name continues on another sheet.
    pub fn off_sheet(
        id: u64,
        pos: Point,
        name: impl Into<String>,
        direction: CrossSheetPortDirection,
    ) -> Self {
        Self {
            id,
            pos,
            name: name.into(),
            kind: NetLabelKind::OffSheet { direction },
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

    /// Check if this label names the simulation reference node.
    ///
    /// The alias set is the product's one ground authority, so the canvas, the
    /// checker and the engine cannot disagree about which labels are node `0`.
    /// `VSS` and `VEE` are supply rails and answer `false`.
    pub fn is_ground(&self) -> bool {
        super::ground_names::is_ground_reference(&self.name)
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

    #[test]
    fn a_label_written_before_off_sheet_connectors_loads_local_and_regains_the_field() {
        let legacy = r#"{"id":7,"pos":{"x":40,"y":-20},"name":"BIAS"}"#;
        let loaded: NetLabel = serde_json::from_str(legacy).expect("legacy label loads");
        assert_eq!(loaded, NetLabel::new(7, Point::new(40, -20), "BIAS"));
        assert_eq!(loaded.kind, NetLabelKind::Local);

        let round_tripped = serde_json::to_string(&loaded).expect("label serializes");
        assert!(
            round_tripped.contains("\"kind\":\"local\""),
            "re-serialized label must state its kind: {round_tripped}"
        );
        assert_eq!(
            serde_json::from_str::<NetLabel>(&round_tripped).expect("round trip"),
            loaded
        );
    }

    #[test]
    fn an_off_sheet_connector_carries_its_direction_through_serde() {
        let connector =
            NetLabel::off_sheet(9, Point::new(0, 0), "BIAS", CrossSheetPortDirection::Output);
        assert_eq!(
            connector.kind.off_sheet_direction(),
            Some(CrossSheetPortDirection::Output)
        );
        assert_eq!(
            NetLabel::new(9, Point::new(0, 0), "BIAS").kind,
            NetLabelKind::Local
        );
        assert_eq!(
            NetLabel::new(9, Point::new(0, 0), "BIAS")
                .kind
                .off_sheet_direction(),
            None
        );

        let encoded = serde_json::to_string(&connector).expect("connector serializes");
        assert_eq!(
            serde_json::from_str::<NetLabel>(&encoded).expect("connector round trips"),
            connector
        );
    }

    #[test]
    fn every_declarable_direction_has_distinct_copy() {
        let mut labels: Vec<&'static str> = NetLabelKind::DIRECTIONS
            .iter()
            .map(|direction| NetLabelKind::direction_label(*direction))
            .collect();
        assert_eq!(labels.len(), 4);
        labels.sort_unstable();
        labels.dedup();
        assert_eq!(labels.len(), 4, "direction copy must be unambiguous");
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
