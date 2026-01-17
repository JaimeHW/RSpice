//! State Management Module
//!
//! Application state for simulation, project, and UI state.

mod simulation;
mod simulation_runner;

pub use simulation::{ConsoleMessage, MessageSeverity, SimulationState, WaveformData};
pub use simulation_runner::run_simulation;
