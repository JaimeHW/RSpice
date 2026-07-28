//! Waveform I/O
//!
//! Read and write waveform data in various formats.
//! Supports interchange formats used by commercial simulators.
//!
//! # Supported Formats
//!
//! - NUTMEG (SPICE3/ngspice raw format) for import
//! - CSV and TSV for import/export
//! - PSF-Lite binary waveform format (`PSFL`) for import
//! - PSF ASCII waveform exports (`psfascii`) for import
//! - Cadence PSF native binary waveform databases for import

#![allow(clippy::needless_range_loop, clippy::type_complexity)]
//! - Touchstone S-parameter format (`.sNp`) for import/export

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod types;
mod writer;

pub use types::{SignalType, WaveformDataset, WaveformFormat, WaveformSignal};
pub use writer::WaveformWriter;

// =============================================================================
// Tests
// =============================================================================
