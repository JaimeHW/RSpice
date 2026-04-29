use super::basic::BasicStats;
use super::integral::calculate_integral;
use super::timing::{
    calculate_duty_cycle, calculate_fall_time, calculate_frequency, calculate_period,
    calculate_rise_time,
};
use super::types::TraceMeasurements;
use crate::waveform::state::TraceData;

// Complete Measurement Calculation
// =============================================================================

/// Calculate all measurements for a trace
pub fn calculate_all_measurements(trace: &TraceData) -> TraceMeasurements {
    let x = &trace.x;
    let y = &trace.y;
    let basic = BasicStats::from_samples(y);

    TraceMeasurements {
        trace_name: trace.name.clone(),
        min: basic.map(|stats| stats.min),
        max: basic.map(|stats| stats.max),
        pk_pk: basic.map(BasicStats::pk_pk),
        mean: basic.map(|stats| stats.mean),
        rms: basic.map(BasicStats::rms),
        std_dev: basic.and_then(BasicStats::std_dev),
        rise_time: calculate_rise_time(x, y),
        fall_time: calculate_fall_time(x, y),
        period: calculate_period(x, y),
        frequency: calculate_frequency(x, y),
        duty_cycle: calculate_duty_cycle(x, y),
        integral: calculate_integral(x, y),
    }
}

/// Calculate measurements over a specified X range
pub fn calculate_measurements_in_range(
    trace: &TraceData,
    x_start: f64,
    x_end: f64,
) -> TraceMeasurements {
    let n = trace.len();
    let start_idx = trace.x.partition_point(|x| *x < x_start).min(n);
    let mut end_idx = (trace.x.partition_point(|x| *x <= x_end) + 1).min(n);
    if end_idx <= start_idx {
        end_idx = (start_idx + 1).min(n);
    }

    // Create sliced trace
    let x_slice = &trace.x[start_idx..end_idx.min(n)];
    let y_slice = &trace.y[start_idx..end_idx.min(n)];
    let basic = BasicStats::from_samples(y_slice);

    TraceMeasurements {
        trace_name: trace.name.clone(),
        min: basic.map(|stats| stats.min),
        max: basic.map(|stats| stats.max),
        pk_pk: basic.map(BasicStats::pk_pk),
        mean: basic.map(|stats| stats.mean),
        rms: basic.map(BasicStats::rms),
        std_dev: basic.and_then(BasicStats::std_dev),
        rise_time: calculate_rise_time(x_slice, y_slice),
        fall_time: calculate_fall_time(x_slice, y_slice),
        period: calculate_period(x_slice, y_slice),
        frequency: calculate_frequency(x_slice, y_slice),
        duty_cycle: calculate_duty_cycle(x_slice, y_slice),
        integral: calculate_integral(x_slice, y_slice),
    }
}

// =============================================================================
