//! Interactive wire drawing over design-owned conductor geometry.

mod drawing;
mod routing;

pub use drawing::WireDrawing;
pub use routing::WireRoutingMode;
#[cfg(test)]
pub use rspice_design::schematic::wire::WireConnection;
pub use rspice_design::schematic::wire::{Wire, WireSegment};
