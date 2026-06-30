//! SPICE Compatibility Module
//!
//! Parsers for standard SPICE file formats.

pub(crate) mod ground;

mod ltspice_raw;

pub use ltspice_raw::{RawFileHeader, RawVariable, RawWaveform, RawWaveformData, parse_raw_file};
