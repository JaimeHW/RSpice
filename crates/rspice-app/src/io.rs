//! I/O Module
//!
//! File input/output for SPICE libraries, netlists, waveforms, and sessions.
//!
//! # Submodules
//!
//! - `session_io` - Session state serialization
//! - `file_exchange` - Desktop and browser file pickers, and bounded reads
//! - `schematic_io` - Schematic file save/load
//! - `netlist_export` - Netlist generation from schematic
//! - `generated_bundle` - Portable generated-netlist archive writing
//! - `waveform_io` - Waveform export (CSV, TSV, Touchstone)

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use rspice_output::durable_file;
pub(crate) mod file_exchange;
pub(crate) mod generated_bundle;
pub(crate) mod netlist_export;
mod project_execution;
pub(crate) mod project_io;
mod project_results;
pub(crate) mod schematic_io;

pub(crate) mod waveform_io;

// Re-exports
pub use generated_bundle::build_generated_bundle;
pub use netlist_export::NetlistFormat;
pub(crate) use project_execution::{capture_execution_context, restore_execution_context};
pub use project_io::{ProjectSimulationResults, ProjectSnapshot, load_project_file};
#[cfg(test)]
pub(crate) use project_results::simulation_state_from_results;
pub(crate) use project_results::{capture_simulation_results, restore_simulation_results};
pub use rspice_project::{PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION, ProjectExecutionContext};
// The run's own corner-expansion entry point, so the Corners page's tests can
// assert that the page and the run reach the same verdict rather than that
// they read alike. Nothing outside a test may reach past the context type.
#[cfg(test)]
pub(crate) use rspice_project::{ProjectModelLibrary, persisted_active_model_section_names};
pub use schematic_io::{SchematicIoError, load_schematic, save_schematic, show_save_dialog};
// Native file pickers. The browser reaches its own import path instead.
#[cfg(not(target_arch = "wasm32"))]
pub use project_io::{show_open_project_dialog, show_save_project_dialog};
#[cfg(not(target_arch = "wasm32"))]
pub use schematic_io::{SchematicFile, show_open_dialog};

pub use waveform_io::{WaveformDataset, WaveformFormat, WaveformWriter};
