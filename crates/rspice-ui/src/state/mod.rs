//! State Management Module
//!
//! Application state for simulation, project, and UI state.

mod schematic;
mod simulation;
mod simulation_runner;

pub use schematic::*;
pub use simulation::{ConsoleMessage, MessageSeverity, SimulationState, WaveformData};
pub use simulation_runner::run_simulation;
