//! State Management Module
//!
//! Application state for simulation, project, and UI state.

pub mod canvas_focus;
pub mod cross_probing;
pub mod display_settings;
pub mod document;
pub mod fft;
pub mod history;
pub mod netlist_generator;
mod schematic;
pub mod schematic_file;
mod simulation;
pub mod simulation_command;
mod simulation_runner;
pub mod waveform_math;

pub use canvas_focus::{use_canvas_focus, CanvasFocus, CanvasFocusState};
pub use display_settings::{PinNameVisibility, SchematicDisplaySettings};
pub use document::{Document, DocumentManager};
#[allow(unused_imports)]
pub use fft::{compute_fft, FftResult, WindowFunction};
pub use history::SchematicHistory;
pub use netlist_generator::{generate_netlist, NetlistResult};
pub use schematic::*;
pub use simulation::{ConsoleMessage, MessageSeverity, SimulationState, WaveformData};
pub use simulation_runner::{run_simulation, SimulationResult, SimulationStats};
#[allow(unused_imports)]
pub use waveform_math::eval_expression;
