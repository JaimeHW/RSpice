//! I/O Module
//!
//! File input/output for SPICE libraries, netlists, waveforms, and sessions.
//!
//! # Submodules
//!
//! - `lib_parser` - SPICE library file parser
//! - `session_io` - Session state serialization
//! - `schematic_io` - Schematic file save/load
//! - `netlist_export` - Netlist generation from schematic
//! - `waveform_io` - Waveform file formats (PSF, NUTMEG, CSV)

pub mod binary_io;
mod cadence_psf;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod durable_file;
pub mod lib_parser;
pub mod netlist_export;
mod project_execution;
pub mod project_io;
pub mod schematic_io;

pub mod waveform_io;

// Re-exports
pub use binary_io::{PsfHeader, PsfReader, PsfWriter};
pub use cadence_psf::{CadencePsfError, ParsedCadencePsfBinary, parse_cadence_psf_binary};
pub use lib_parser::{
    IncludeDirective, IncludeType, LibrarySection, ModelDef, ParamValue, ParsedLibrary,
    SubcircuitDef,
};
pub use netlist_export::{ExportOptions, NetlistExporter, NetlistFormat};
pub use project_execution::{
    PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION, ProjectExecutionContext, ProjectModelLibrary,
};
#[allow(deprecated)]
pub use project_io::{
    ProjectFile, ProjectIoError, ProjectSimulationResults, ProjectVersion, load_project_file,
    save_project_file, show_open_project_dialog, show_save_project_dialog,
};
pub use schematic_io::{
    SchematicFile, SchematicIoError, SchematicVersion, load_schematic, save_schematic,
    show_open_dialog, show_save_dialog,
};

pub use waveform_io::{
    SignalType, WaveformDataset, WaveformFormat, WaveformReader, WaveformSignal, WaveformWriter,
};
