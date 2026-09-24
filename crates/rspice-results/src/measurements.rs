//! Interval statistics shared by calculator expressions and result readouts.
//!
//! Measurements use the piecewise-linear retained waveform, independently of
//! plot smoothing or display decimation. Missing coverage is an explicit error;
//! sample density must not change the measured mean or RMS.

mod interval;

pub use interval::{IntervalStatistics, MeasurementError, measure_interval};
