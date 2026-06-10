//! Waveform Measurements
//!
//! Commercial-grade waveform measurement calculations matching
//! the capabilities of Cadence ViVA and similar tools.
//!
//! # Supported Measurements
//!
//! - **Amplitude**: min, max, pk-pk, mean, RMS
//! - **Timing**: rise time, fall time, period, frequency, duty cycle
//! - **Statistics**: standard deviation, variance
//! - **Area**: integral under curve

// =============================================================================

mod aggregate;
mod annotations;
mod basic;
mod cache;
mod integral;
mod timing;
mod types;

pub use aggregate::{calculate_all_measurements, calculate_measurements_in_range};
pub use annotations::{AnnotationSet, AnnotationType, CalloutPosition, MeasurementAnnotation};
pub use basic::{
    calculate_max, calculate_mean, calculate_min, calculate_min_max_rms, calculate_pk_pk,
    calculate_rms, calculate_std_dev,
};
pub use cache::MeasurementCache;
pub use integral::calculate_integral;
pub use timing::{
    Crossing, calculate_duty_cycle, calculate_fall_time, calculate_frequency, calculate_period,
    calculate_rise_time, find_crossings,
};
pub use types::{Measurement, TraceMeasurements};
