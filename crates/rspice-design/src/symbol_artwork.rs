//! Immutable built-in SVG artwork, component bindings and terminal geometry.

mod error;
mod library;
mod parser;
mod placement;
mod types;

pub use error::SymbolError;
pub use library::SymbolLibrary;
pub use placement::PlacedRun;
pub use types::{PathCommand, Symbol, SymbolPath};
