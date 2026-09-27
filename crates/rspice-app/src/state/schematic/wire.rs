//! Interactive wire drawing over design-owned conductor geometry.

mod drawing;
mod routing;

pub use drawing::WireDrawing;
pub use routing::WireRoutingMode;
pub use rspice_design::schematic::wire::{Wire, WireConnection, WireSegment};
