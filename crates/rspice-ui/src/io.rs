//! I/O Module
//!
//! File input/output for SPICE libraries, netlists, waveforms, and sessions.
//!
//! # Submodules
//!
//! - `session_io` - Session state serialization
//! - `schematic_io` - Schematic file save/load
//! - `netlist_export` - Netlist generation from schematic
//! - `generated_bundle` - Portable generated-netlist archive writing
//! - `waveform_io` - Waveform export (CSV, TSV, Touchstone)

#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod durable_file;
pub(crate) mod generated_bundle;
pub(crate) mod netlist_export;
mod project_execution;
pub(crate) mod project_io;
pub(crate) mod schematic_io;

pub(crate) mod waveform_io;

// Re-exports
pub use generated_bundle::build_generated_bundle;
pub use netlist_export::NetlistFormat;
pub use project_execution::{PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION, ProjectExecutionContext};
pub(crate) use project_io::ProjectResultExpressionGroup;
#[allow(deprecated)]
pub use project_io::{ProjectFile, ProjectIoError, ProjectSimulationResults, load_project_file};
pub use schematic_io::{SchematicIoError, load_schematic, save_schematic, show_save_dialog};
// Native file pickers. The browser reaches its own import path instead.
#[cfg(not(target_arch = "wasm32"))]
pub use project_io::{show_open_project_dialog, show_save_project_dialog};
#[cfg(not(target_arch = "wasm32"))]
pub use schematic_io::{SchematicFile, show_open_dialog};

pub use waveform_io::{
    SignalType, WaveformDataset, WaveformFormat, WaveformSignal, WaveformWriter,
};
