//! Waveform Measurements
//!
//! Numeric measurements over sampled waveforms (min/max/RMS and friends),
//! used by the Results workspace. The old immediate-mode waveform viewer
//! that lived here was superseded by `crate::workbench::result_document` + the
//! `crate::ui::plot` engine.

pub mod measurements;
