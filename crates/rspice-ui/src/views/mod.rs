//! Views Module
//!
//! Main content views for the application.

mod console;
mod netlist;
pub mod syntax;
mod waveform;

pub use console::Console;
pub use netlist::Netlist;
pub use waveform::Waveform;
