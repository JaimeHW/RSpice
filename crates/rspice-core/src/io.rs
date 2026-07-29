//! Waveform file formats: reading and writing SPICE RAW.
//!
//! Reading and writing the same format used to sit in two unrelated places —
//! the reader under `compat` (as a foreign-simulator shim) and the writers
//! under `analysis::output`. They are one concern, and the round-trip test in
//! [`waveform_stream`] needs both, so they live together here.
//!
//! This module is a leaf. [`raw_export`] has no in-crate dependencies at all
//! and [`waveform_stream`] needs only [`crate::resource`] for its read limits;
//! nothing here knows about circuits, devices or analyses. Callers pass names
//! and columns of [`crate::Value`], which is what keeps the layer honest: a
//! format module that reached for a result type would have to be rebuilt every
//! time a result type changed.
//!
//! Waveform *compression* is not here. `TransientResultCompressed` is a shape
//! of transient result rather than a file format, so it lives beside the
//! uncompressed one in [`crate::engine`].

pub mod ltspice_raw;
pub mod raw_export;
pub mod waveform_stream;

pub use ltspice_raw::{
    RawFileHeader, RawParseError, RawWaveform, RawWaveformData, parse_raw_file,
    parse_raw_file_with_limits, parse_raw_reader, parse_raw_reader_with_limits,
};
pub use raw_export::{
    RawExporter, RawFormat, RawVariable, VariableType, export_dc_sweep, export_transient,
};
pub use waveform_stream::{StreamingWaveformReader, StreamingWaveformWriter, WaveformStreamError};

// `ltspice_raw` also names a `RawVariable`: the descriptor parsed out of a
// file's header, as opposed to the one handed to the exporter on the way out.
// Only one of the two can hold the short name here, so the reader's stays at
// `io::ltspice_raw::RawVariable`. The types are not interchangeable and the
// longer path says so.
