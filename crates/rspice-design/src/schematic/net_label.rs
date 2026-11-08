//! Net Labels and Junctions
//!
//! Types for naming nets and handling wire junctions.

use rspice_design_model::Point;
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

    /// A supply rail. Placing one declares its name global: it joins every
    /// sheet and every cell of the design, and the deck writes `.GLOBAL`.
    Supply { glyph: SupplyGlyph },
}

/// How a supply symbol is drawn. Ground keeps its own symbol.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum SupplyGlyph {
    /// A bar above the pin: a positive rail such as VDD.
    #[default]
    Bar,
    /// An arrow above the pin: a positive rail such as VCC.
    Arrow,
    /// A bar below the pin: a negative or return rail such as VSS or VEE.
    Down,
}

impl SupplyGlyph {
    pub const ALL: [Self; 3] = [Self::Bar, Self::Arrow, Self::Down];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Bar => "Bar",
            Self::Arrow => "Arrow",
            Self::Down => "Negative bar",
        }
    }

    /// The glyph a rail is drawn with by convention: VSS, VEE and a negative
    /// name point down, VCC takes the arrow, anything else the bar.
    pub fn for_name(name: &str) -> Self {
        let name = name.trim().trim_end_matches('!').to_ascii_uppercase();
        if name.starts_with("VSS") || name.starts_with("VEE") || name.starts_with("V-") {
            Self::Down
        } else if name.starts_with("VCC") {
            Self::Arrow
        } else {
            Self::Bar
        }
    }
}

/// Which side of its point a net label's name stands on.
///
/// A label placed here reads to the right of its point. A label imported
/// from a drawing that says which way its name runs keeps that side, so the
/// name stays clear of what the source drawing kept it clear of.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum NetLabelSide {
    /// Above the line through the point, reading away to the right.
    #[default]
    Right,
    /// Above the line through the point, ending short of it on the left.
    Left,
    /// Centred over the point.
    Above,
    /// Centred under the point.
    Below,
    /// Read upward, on the line through the point, starting past it: a
    /// drawing's flag on the top end of a vertical wire.
    RisingAbove,
    /// Read upward, on the line through the point, ending short of it: a
    /// drawing's flag on the bottom end of a vertical wire.
    RisingBelow,
    /// Read upward, centred to the left of the point: a drawing's flag on
    /// the middle of a vertical wire.
    RisingBeside,
}

impl NetLabelSide {
    fn is_right(&self) -> bool {
        *self == Self::Right
    }

    /// Whether a name on this side reads upward rather than level.
    #[must_use]
    pub const fn reads_upward(self) -> bool {
        matches!(
            self,
            Self::RisingAbove | Self::RisingBelow | Self::RisingBeside
        )
    }
}

impl NetLabelKind {
    /// A supply symbol's glyph, or `None` for any other label.
    pub const fn supply_glyph(self) -> Option<SupplyGlyph> {
        match self {
            Self::Supply { glyph } => Some(glyph),
            Self::Local | Self::OffSheet { .. } => None,
        }
    }

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
            // A supply joins by being global, not by a sheet-to-sheet claim.
            Self::Local | Self::Supply { .. } => None,
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

    /// Which side of its point the name stands on. A supply symbol's name
    /// follows its glyph instead.
    #[serde(default, skip_serializing_if = "NetLabelSide::is_right")]
    pub side: NetLabelSide,

    /// What the label draws at its point.
    #[serde(default, skip_serializing_if = "NetLabelDrawing::is_native")]
    pub drawing: NetLabelDrawing,
}

/// What a net label draws, which never touches the net it names.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum NetLabelDrawing {
    /// RSpice's own label: its attachment mark and name, and a rail's symbol.
    #[default]
    Native,
    /// A flag from another tool's drawing: the name at the wire end and
    /// nothing else, at the size that drawing letters it.
    Lettered,
    /// A pin another tool's drawing leaves open and its simulator ties to
    /// this net: the open-pin square and nothing else.
    OpenPin,
}

impl NetLabelDrawing {
    pub fn is_native(&self) -> bool {
        *self == Self::Native
    }
}

impl NetLabel {
    /// Create a new net label
    pub fn new(id: u64, pos: Point, name: impl Into<String>) -> Self {
        Self {
            id,
            pos,
            name: name.into(),
            kind: NetLabelKind::Local,
            side: NetLabelSide::Right,
            drawing: NetLabelDrawing::Native,
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
            side: NetLabelSide::Right,
            drawing: NetLabelDrawing::Native,
        }
    }

    /// Create a supply symbol: a rail declared global by placing it.
    pub fn supply(id: u64, pos: Point, name: impl Into<String>, glyph: SupplyGlyph) -> Self {
        Self {
            id,
            pos,
            name: name.into(),
            kind: NetLabelKind::Supply { glyph },
            side: NetLabelSide::Right,
            drawing: NetLabelDrawing::Native,
        }
    }

    /// The global this label declares, `!`-suffixed: a supply symbol always
    /// declares its rail; any other label only when its name ends in `!`.
    pub fn global_declaration(&self) -> Option<String> {
        let name = self.name.trim();
        match self.kind {
            NetLabelKind::Supply { .. } if !name.is_empty() => {
                Some(format!("{}!", name.trim_end_matches('!')))
            }
            _ if name.ends_with('!') => Some(name.to_owned()),
            _ => None,
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
    use crate::schematic::document_policy::NetNamingPolicy;

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
