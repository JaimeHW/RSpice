//! SPICE Compatibility Module
//!
//! Readers for foreign simulator file formats.
//!
//! The SPICE ground-name rule used to live here too, which put this module
//! underneath the solver, the device models and the parser. It is a naming
//! rule rather than a compatibility shim, and now lives in [`crate::naming`].

mod ltspice_raw;

pub use ltspice_raw::{
    RawFileHeader, RawParseError, RawVariable, RawWaveform, RawWaveformData, parse_raw_file,
    parse_raw_file_with_limits, parse_raw_reader, parse_raw_reader_with_limits,
};
