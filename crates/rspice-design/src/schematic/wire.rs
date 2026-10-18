//! Committed conductors, terminal connections, and exact segment geometry.

mod connection;
mod polyline;
mod segment;
mod through;

pub use connection::WireConnection;
pub use polyline::Wire;
pub use segment::{WireHitResult, WireSegment};
pub(crate) use through::WiresThrough;
