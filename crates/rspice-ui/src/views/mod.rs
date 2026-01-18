//! Views Module
//!
//! Main content views for the application.

mod console;
mod netlist;
mod schematic;
pub mod symbols;
pub mod syntax;
mod waveform;
pub mod waveform_gpu;

pub use console::Console;
pub use netlist::Netlist;
pub use schematic::{Schematic, SchematicToolbar};
pub use waveform::Waveform;
