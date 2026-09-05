//! Waveform file formats: reading and writing SPICE RAW and VCD.
//!
//! Reading and writing the same format used to sit in two unrelated places —
//! the reader under `compat` (as a foreign-simulator shim) and the writers
//! under `analysis::output`. They are one concern, and the round-trip test in
//! [`waveform_stream`] needs both, so they live together here.
//!
//! The RAW modules remain leaf components: [`raw_export`] has no in-crate
//! dependencies and [`waveform_stream`] needs only [`crate::resource`] for its
//! read limits. [`vcd`] carries the event-driven half — irregular four-state
//! timelines rather than a sampled table — and needs the same read limits and
//! nothing else. [`xyce_prn`] deliberately consumes the typed `.PRINT` layout
//! and simulation output policy retained by [`crate::netlist`], but no format
//! module reaches for a circuit or analysis-result type. Callers still provide
//! names and columns of [`crate::Value`], keeping serialization independent of
//! solver internals.
//!
//! Waveform *compression* is not here. `TransientResultCompressed` is a shape
//! of transient result rather than a file format, so it lives beside the
//! uncompressed one in [`crate::engine`].

pub mod ltspice_raw;
pub mod raw_export;
pub mod vcd;
pub mod waveform_stream;
pub mod xyce_prn;

pub use ltspice_raw::{
    RawFile, RawFileHeader, RawParseError, RawWaveform, RawWaveformData, parse_raw_file,
    parse_raw_file_with_limits, parse_raw_plots_file_with_limits,
    parse_raw_plots_reader_with_limits, parse_raw_reader, parse_raw_reader_with_limits,
};
pub use raw_export::{
    RawEventKind, RawEventTimeline, RawExporter, RawFormat, RawVariable, VariableType,
    export_dc_sweep, export_transient, write_event_plots,
};
pub use vcd::{
    VCD_WRITER_VERSION, VcdBit, VcdChange, VcdDocument, VcdError, VcdMagnitude, VcdSignal,
    VcdSignalKind, VcdTimeUnit, VcdTimescale, VcdValue, VcdVariable, parse_vcd_file,
    parse_vcd_file_with_limits, parse_vcd_reader, parse_vcd_reader_with_limits, write_vcd,
};
pub use waveform_stream::{StreamingWaveformWriter, WaveformStreamError};
pub use xyce_prn::{
    XycePrnError, XycePrnFooter, XycePrnLimits, XycePrnScientificStyle, XycePrnTable,
    format_xyce_prn_scientific, serialize_legacy_compact_prn_for_comparison,
    serialize_xyce_prn_sequence,
};

// `ltspice_raw` also names a `RawVariable`: the descriptor parsed out of a
// file's header, as opposed to the one handed to the exporter on the way out.
// Only one of the two can hold the short name here, so the reader's stays at
// `io::ltspice_raw::RawVariable`. The types are not interchangeable and the
// longer path says so.
