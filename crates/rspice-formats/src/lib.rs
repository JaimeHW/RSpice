//! Portable engineering-data codecs.

#[cfg(feature = "columnar")]
pub mod columnar;
pub mod csv_sheet;
pub mod delimited;
#[cfg(feature = "result-digital")]
pub mod digital;
pub mod fst;
#[cfg(feature = "hdf5")]
pub mod hdf5;
pub mod matlab;
#[cfg(feature = "monte-carlo-checkpoint")]
pub mod monte_carlo_checkpoint;
#[cfg(feature = "native-bundle")]
pub mod native_bundle;
pub mod numeric;
pub mod numpy;
pub mod psf;
#[cfg(feature = "result-csv")]
pub mod result_csv;
pub mod table;
#[cfg(feature = "table-schema")]
pub mod table_schema;
#[cfg(feature = "result-vcd")]
pub mod vcd;
pub mod waveform_io;
#[cfg(feature = "xlsx")]
pub mod xlsx;
pub mod zip;

pub use waveform_io::{
    SignalType, WaveformDataset, WaveformDomain, WaveformFormat, WaveformSignal, WaveformWriter,
    read_touchstone_bytes,
};
