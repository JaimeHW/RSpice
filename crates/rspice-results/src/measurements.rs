//! Interval statistics shared by calculator expressions and result readouts.
//!
//! Measurements use the piecewise-linear retained waveform, independently of
//! plot smoothing or display decimation. Missing coverage is an explicit error;
//! sample density must not change the measured mean or RMS.

mod interval;

pub use interval::{IntervalStatistics, MeasurementError, measure_interval};

/// Finite (minimum, maximum) of retained samples, or `None` without finite values.
pub fn finite_extremes(values: &[f64]) -> Option<(f64, f64)> {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for &v in values {
        if v.is_finite() {
            lo = lo.min(v);
            hi = hi.max(v);
        }
    }
    (lo <= hi).then_some((lo, hi))
}
