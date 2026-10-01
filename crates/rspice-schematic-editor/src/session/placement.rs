//! Model-card state carried by one armed device placement.

use super::tool::Tool;

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
