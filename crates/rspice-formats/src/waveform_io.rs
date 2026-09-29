//! Waveform I/O
//!
//! Read and write waveform data in explicitly qualified formats.
//!
//! # Supported Formats
//!
//! - CSV and TSV export
//! - Touchstone v1/v2 S-parameter import and export

#![allow(clippy::needless_range_loop, clippy::type_complexity)]
//! - Touchstone S-parameter format (`.sNp`) for import/export

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const MAX_TOUCHSTONE_PORTS: usize = 64;

#[cfg(feature = "result-waveform")]
pub mod result;
mod touchstone_noise;
mod touchstone_reader;
mod types;
mod writer;

pub use touchstone_reader::read_touchstone_bytes;
pub use types::{
    SignalType, UnsupportedWaveformDomain, WaveformDataset, WaveformDomain, WaveformFormat,
    WaveformSignal,
};
pub use writer::{WaveformWriteError, WaveformWriter};

/// A Touchstone validation or decoding failure, with parser causes retained.
#[derive(Debug)]
pub enum TouchstoneError {
    InvalidData(String),
    Encoding(std::str::Utf8Error),
    InvalidFloat {
        detail: String,
        source: std::num::ParseFloatError,
    },
    InvalidInteger {
        detail: String,
        source: std::num::ParseIntError,
    },
}

impl std::fmt::Display for TouchstoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidData(detail)
            | Self::InvalidFloat { detail, .. }
            | Self::InvalidInteger { detail, .. } => f.write_str(detail),
            Self::Encoding(source) => write!(f, "Touchstone source is not valid UTF-8: {source}"),
        }
    }
}

impl std::error::Error for TouchstoneError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Encoding(source) => Some(source),
            Self::InvalidFloat { source, .. } => Some(source),
            Self::InvalidInteger { source, .. } => Some(source),
            Self::InvalidData(_) => None,
        }
    }
}

impl From<String> for TouchstoneError {
    fn from(detail: String) -> Self {
        Self::InvalidData(detail)
    }
}

impl From<&str> for TouchstoneError {
    fn from(detail: &str) -> Self {
        Self::InvalidData(detail.to_owned())
    }
}
