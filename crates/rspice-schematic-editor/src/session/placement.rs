//! Library binding and model-card state carried by armed placement tools.

use super::placement_authority::PlacementAuthority;
use super::tool::Tool;
use crate::requests::EditorRequestSource;
use rspice_design::schematic::component::LibraryCellInstance;

/// A frozen cell binding and the document context that owns its placement batch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingLibraryCellPlacement {
    pub binding: LibraryCellInstance,
    pub authority: PlacementAuthority,
}

impl PendingLibraryCellPlacement {
    pub fn new(binding: LibraryCellInstance, source: EditorRequestSource) -> Self {
        Self {
            binding,
            authority: PlacementAuthority::new(source),
        }
    }
}

/// A model card armed for the next placement of one exact device kind.
///
/// The armed tool is recorded beside the card so re-arming any other tool
/// retires it. Without that, arming a resistor after arming a zener would
/// place a resistor still carrying the diode's card — a component that reads
/// as valid everywhere and netlists as nonsense.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingPartModel {
    /// The exact placement tool this card belongs to.
    pub tool: Tool,
    /// Model card name, as the signed manifest publishes it. It becomes the
    /// placed instance's value, which is where every native emitter reads it.
    pub model: String,
    /// A named symbol skin the device family ships, when the part asked for
    /// one.
    pub variant: Option<String>,
}
