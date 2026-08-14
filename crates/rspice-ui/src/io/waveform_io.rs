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

mod touchstone_reader;
mod types;
mod writer;

pub(crate) use touchstone_reader::read_touchstone_bytes;
pub use types::{SignalType, WaveformDataset, WaveformFormat, WaveformSignal};
pub use writer::WaveformWriter;

// =============================================================================
// Tests
// =============================================================================
