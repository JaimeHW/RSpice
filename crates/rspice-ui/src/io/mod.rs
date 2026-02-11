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
pub mod lib_parser;
pub mod netlist_export;
pub mod schematic_io;
pub mod session_io;
pub mod waveform_io;

// Re-exports
pub use binary_io::{PsfHeader, PsfReader, PsfWriter};
pub use cadence_psf::{parse_cadence_psf_binary, CadencePsfError, ParsedCadencePsfBinary};
pub use lib_parser::{
    IncludeDirective, IncludeType, LibrarySection, ModelDef, ParamValue, ParsedLibrary,
    SubcircuitDef,
};
pub use netlist_export::{ExportOptions, NetlistExporter, NetlistFormat};
pub use schematic_io::{
    load_schematic, save_schematic, show_open_dialog, show_save_dialog, SchematicFile,
    SchematicIoError, SchematicVersion,
};
pub use session_io::{load_session, save_session, SessionFile, SessionVersion};
pub use waveform_io::{
    SignalType, WaveformDataset, WaveformFormat, WaveformReader, WaveformSignal, WaveformWriter,
};
