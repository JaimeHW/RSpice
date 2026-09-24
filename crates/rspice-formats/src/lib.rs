//! Portable engineering-data codecs.

#[cfg(feature = "columnar")]
pub mod columnar;
#[cfg(feature = "hdf5")]
pub mod hdf5;
pub mod fst;
pub mod matlab;
pub mod numeric;
pub mod numpy;
pub mod table;
pub mod waveform_io;
#[cfg(feature = "xlsx")]
pub mod xlsx;
pub mod zip;

pub use waveform_io::{
    SignalType, WaveformDataset, WaveformFormat, WaveformSignal, WaveformWriter,
    read_touchstone_bytes,
};
