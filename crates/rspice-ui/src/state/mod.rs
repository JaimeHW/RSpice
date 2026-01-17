//! State Management Module
//!
//! Application state for simulation, project, and UI state.

pub mod fft;
pub mod history;
mod schematic;
mod simulation;
mod simulation_runner;
pub mod waveform_math;

#[allow(unused_imports)]
pub use fft::{compute_fft, FftResult, WindowFunction};
pub use history::SchematicHistory;
pub use schematic::*;
pub use simulation::{ConsoleMessage, MessageSeverity, SimulationState, WaveformData};
pub use simulation_runner::run_simulation;
#[allow(unused_imports)]
pub use waveform_math::eval_expression;
