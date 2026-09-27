//! Committed conductors, terminal connections, and exact segment geometry.

mod connection;
mod polyline;
mod segment;

pub use connection::WireConnection;
pub use polyline::Wire;
pub use segment::{WireHitResult, WireSegment};
