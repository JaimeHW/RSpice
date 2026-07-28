//! I/O Module
//!
//! File input/output for SPICE libraries, netlists, waveforms, and sessions.
//!
//! # Submodules
//!
//! - `session_io` - Session state serialization
//! - `schematic_io` - Schematic file save/load
//! - `netlist_export` - Netlist generation from schematic
//! - `waveform_io` - Waveform export (CSV, TSV, Touchstone)

#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod durable_file;
pub(crate) mod netlist_export;
mod project_execution;
pub(crate) mod project_io;
pub(crate) mod schematic_io;

pub(crate) mod waveform_io;

// Re-exports
pub use netlist_export::NetlistFormat;
pub use project_execution::{
    PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION, ProjectExecutionContext,
};
#[allow(deprecated)]
pub use project_io::{
    ProjectFile, ProjectIoError, ProjectSimulationResults, load_project_file,
    show_open_project_dialog, show_save_project_dialog,
};
pub use schematic_io::{
    SchematicFile, SchematicIoError, load_schematic, save_schematic, show_open_dialog,
    show_save_dialog,
};

pub use waveform_io::{
    SignalType, WaveformDataset, WaveformFormat, WaveformSignal, WaveformWriter,
};
