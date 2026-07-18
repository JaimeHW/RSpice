//! SPICE Compatibility Module
//!
//! Parsers for standard SPICE file formats.

pub(crate) mod ground;

mod ltspice_raw;

pub use ltspice_raw::{
    RawFileHeader, RawParseError, RawVariable, RawWaveform, RawWaveformData, parse_raw_file,
    parse_raw_file_with_limits, parse_raw_reader, parse_raw_reader_with_limits,
};
