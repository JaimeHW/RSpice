//! Shared electrical port identities and their declared conductor widths.

use serde::{Deserialize, Serialize};

/// Electrical direction of an interface port.
///
/// Direction drives generated-symbol pin placement (inputs left, outputs
/// right, supplies top/bottom) and is advisory for netlisting — SPICE port
/// lists are positional and direction-free.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PortDirection {
    /// Signal input — generated symbols place these on the left edge.
    In,
    /// Signal output — right edge.
    Out,
    /// Bidirectional or unclassified — right edge, after outputs.
    #[default]
    InOut,
    /// Power/ground rail — top edge for the first, bottom for the second.
    Supply,
}

impl PortDirection {
    /// Parse the `dir=` instance parameter, tolerant of common synonyms.
    pub fn parse(raw: &str) -> Self {
        match raw.trim().to_ascii_lowercase().as_str() {
            "in" | "input" => PortDirection::In,
            "out" | "output" => PortDirection::Out,
            "supply" | "power" | "rail" | "global" => PortDirection::Supply,
            _ => PortDirection::InOut,
        }
    }

    /// Canonical keyword for the `dir=` parameter.
    pub fn keyword(&self) -> &'static str {
        match self {
            PortDirection::In => "in",
            PortDirection::Out => "out",
            PortDirection::InOut => "inout",
            PortDirection::Supply => "supply",
        }
    }
}

/// Electrical role of one interface pin: its direction plus what a symbol
/// needs to draw it in the conventional place. A SPICE port list is
/// positional, so roles come from pin names and the vendor's pin-order
/// comment, never from the netlist itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PinRole {
    NonInvertingInput,
    InvertingInput,
    /// Any other input: an enable, a reference, a single-ended input.
    Input,
    Output,
    PositiveSupply,
    NegativeSupply,
    Ground,
    /// Bidirectional or not recognized.
    Bidirectional,
}

impl PinRole {
    /// The direction a generated symbol places this role by.
    pub const fn direction(self) -> PortDirection {
        match self {
            Self::NonInvertingInput | Self::InvertingInput | Self::Input => PortDirection::In,
            Self::Output => PortDirection::Out,
            Self::PositiveSupply | Self::NegativeSupply | Self::Ground => PortDirection::Supply,
            Self::Bidirectional => PortDirection::InOut,
        }
    }
}

/// One pin of a cell's interface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PortSpec {
    /// Port (and net) name.
    pub name: String,
    /// Declared direction.
    pub direction: PortDirection,
}

impl PortSpec {
    /// The vector this pin declares, when its name declares one.
    ///
    /// The name is the declaration — see [`super::bus::declared_vector`] — so a
    /// port drawn `DATA[7:0]` is one interface pin carrying eight conductors,
    /// and no second field can drift away from the name the drawing shows.
    pub fn vector(&self) -> Option<super::bus::BusDeclaration> {
        super::bus::declared_vector(&self.name)
    }

    /// Conductors this pin carries: the declared width, or one.
    pub fn width(&self) -> usize {
        super::bus::declared_width(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_parsing_tolerates_synonyms() {
        assert_eq!(PortDirection::parse("Input"), PortDirection::In);
        assert_eq!(PortDirection::parse("OUTPUT"), PortDirection::Out);
        assert_eq!(PortDirection::parse("power"), PortDirection::Supply);
        assert_eq!(PortDirection::parse("weird"), PortDirection::InOut);
    }
}
