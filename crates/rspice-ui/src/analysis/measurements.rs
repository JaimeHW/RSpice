//! Basic amplitude statistics over waveform sample slices.
//!
//! Single-pass Welford accumulation over a bare `&[f64]`, shared by the
//! Results workspace's cursor and measurement tables.
//!
//! # Why this is not `rspice_core::analysis::measurements::Waveform`
//!
//! Core's `Waveform` is the measurement suite for analysis results: it pairs a
//! time vector with values, validates them, and offers overshoot, undershoot,
//! crossings, and AC RMS. This module is deliberately narrower, and it differs
//! in one way that matters for display:
//!
//! - It takes values alone. A cursor readout measures a windowed slice of a
//!   trace, and the caller has already indexed the time vector.
//! - **It skips non-finite samples.** A diverged transient, a log of zero in a
//!   dB trace, or a phase discontinuity puts a NaN or an infinity in the
//!   series. Core's `rms` and `average` sum straight through those, so one bad
//!   sample makes the whole reported figure NaN. A measurement table must
//!   still report the finite part of the trace, so this accumulator ignores
//!   non-finite samples and returns `None` only when nothing finite remains.
//! - It computes min, max, mean, RMS, and variance in one pass, because the
//!   Results workspace recomputes them on every cursor move.
//!
//! Do not fold this into core's suite without preserving the finite-only
//! contract: that would regress every measurement readout on a trace holding
//! a single non-finite sample.

mod basic;

pub use basic::calculate_min_max_rms;
