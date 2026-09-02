//! Checked frequency-grid construction shared by physical analyses.

use crate::Value;
use crate::abort_signal::AbortSignal;

/// Sweep scale used to construct a frequency grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrequencyGridScale {
    /// Uniformly spaced frequencies.
    Linear,
    /// Uniform spacing in base-10 logarithmic frequency.
    Decade,
    /// Uniform spacing in base-2 logarithmic frequency.
    Octave,
}

/// Failure while validating or retaining a physical-analysis frequency grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FrequencyGridError {
    /// The start frequency was not finite or did not satisfy the sweep's sign rule.
    InvalidStartFrequency,
    /// The stop frequency was not finite or preceded the start frequency.
    InvalidStopFrequency,
    /// The analysis requires a non-degenerate range but stop equaled start.
    NonIncreasingSweep,
    /// No frequency points were requested.
    EmptySweep,
    /// An explicitly authored frequency was not finite and strictly positive.
    InvalidExplicitFrequency {
        /// Zero-based position in the authored list.
        index: usize,
    },
    /// A logarithmic span and point density implied more points than `usize` can hold.
    PointCountOverflow,
    /// The grid's backing allocation could not be reserved.
    Allocation {
        /// Number of `Value` elements requested.
        requested: usize,
    },
    /// A caller-provided retained-point ceiling was exceeded before allocation.
    LimitExceeded {
        /// Exact retained count when known, otherwise the first proven count above the limit.
        requested: usize,
        /// Caller-provided maximum retained point count.
        limit: usize,
    },
    /// The caller cancelled grid construction.
    Aborted,
}

impl std::fmt::Display for FrequencyGridError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidStartFrequency => {
                formatter.write_str("start frequency is invalid for this sweep")
            }
            Self::InvalidStopFrequency => {
                formatter.write_str("stop frequency must be finite and >= start frequency")
            }
            Self::NonIncreasingSweep => {
                formatter.write_str("stop frequency must be greater than start frequency")
            }
            Self::EmptySweep => {
                formatter.write_str("frequency sweep must contain at least one point")
            }
            Self::InvalidExplicitFrequency { index } => write!(
                formatter,
                "explicit frequency at index {index} must be positive and finite"
            ),
            Self::PointCountOverflow => {
                formatter.write_str("frequency-grid point count exceeds addressable limits")
            }
            Self::Allocation { requested } => write!(
                formatter,
                "unable to allocate {requested} values for the frequency grid"
            ),
            Self::LimitExceeded { requested, limit } => write!(
                formatter,
                "frequency grid requires at least {requested} points, exceeding the limit of {limit}"
            ),
            Self::Aborted => formatter.write_str("frequency-grid construction was aborted"),
        }
    }
}

impl std::error::Error for FrequencyGridError {}

/// Validate the common endpoints of a generated sweep without allocating it.
///
/// `point_parameter` is a total count for [`FrequencyGridScale::Linear`] and
/// a density per decade or octave for logarithmic scales.
pub fn validate_generated_sweep(
    start: Value,
    stop: Value,
    point_parameter: usize,
    scale: FrequencyGridScale,
    linear_start_may_be_zero: bool,
) -> Result<(), FrequencyGridError> {
    let valid_start = start.is_finite()
        && if scale == FrequencyGridScale::Linear && linear_start_may_be_zero {
            start >= 0.0
        } else {
            start > 0.0
        };
    if !valid_start {
        return Err(FrequencyGridError::InvalidStartFrequency);
    }
    if !stop.is_finite() || stop < start {
        return Err(FrequencyGridError::InvalidStopFrequency);
    }
    if point_parameter == 0 {
        return Err(FrequencyGridError::EmptySweep);
    }
    Ok(())
}

/// Compute the exact retained point count without allocating the grid.
///
/// The caller must first apply [`validate_generated_sweep`] with the same
/// endpoints, point parameter, and scale. `minimum_log_points` permits a
/// logarithmic consumer to require two interpolated endpoints; use `1` for
/// PAC/PXF-compatible single-point degenerate spans.
pub fn frequency_point_count(
    start: Value,
    stop: Value,
    point_parameter: usize,
    scale: FrequencyGridScale,
    minimum_log_points: usize,
) -> Result<usize, FrequencyGridError> {
    let logarithmic_span = match scale {
        FrequencyGridScale::Linear => return Ok(point_parameter),
        FrequencyGridScale::Decade => stop.log10() - start.log10(),
        FrequencyGridScale::Octave => stop.log2() - start.log2(),
    };
    let rounded_count = (logarithmic_span * point_parameter as Value).ceil();
    // `usize::MAX as f64` can round upward. Reject the equality boundary so
    // the subsequent float-to-integer cast cannot silently saturate.
    if !rounded_count.is_finite() || rounded_count >= usize::MAX as Value {
        return Err(FrequencyGridError::PointCountOverflow);
    }
    Ok((rounded_count as usize).max(minimum_log_points.max(1)))
}

/// Construct a validated generated grid with fallible retention and cancellation.
///
/// This validates before computing the exact retained count, reserves all
/// storage before writing values, and never returns a partial grid.
pub fn generate_frequency_grid(
    start: Value,
    stop: Value,
    point_parameter: usize,
    scale: FrequencyGridScale,
    linear_start_may_be_zero: bool,
    minimum_log_points: usize,
    abort: &dyn AbortSignal,
) -> Result<Vec<Value>, FrequencyGridError> {
    ensure_not_aborted(abort)?;
    validate_generated_sweep(
        start,
        stop,
        point_parameter,
        scale,
        linear_start_may_be_zero,
    )?;
    let point_count =
        frequency_point_count(start, stop, point_parameter, scale, minimum_log_points)?;
    let mut frequencies = Vec::new();
    frequencies
        .try_reserve_exact(point_count)
        .map_err(|_| FrequencyGridError::Allocation {
            requested: point_count,
        })?;

    if point_count == 1 {
        frequencies.push(start);
    } else if scale == FrequencyGridScale::Linear {
        // Divide the span before multiplying by the index. `(stop - start) *
        // index / denominator` can overflow at a finite `stop == f64::MAX`.
        let step = (stop - start) / (point_count - 1) as Value;
        for index in 0..point_count {
            poll_abort(abort, index)?;
            frequencies.push(start + index as Value * step);
        }
    } else {
        let (axis_start, axis_stop, base) = match scale {
            FrequencyGridScale::Decade => (start.log10(), stop.log10(), 10.0_f64),
            FrequencyGridScale::Octave => (start.log2(), stop.log2(), 2.0_f64),
            FrequencyGridScale::Linear => {
                return Err(FrequencyGridError::PointCountOverflow);
            }
        };
        let denominator = (point_count - 1) as Value;
        for index in 0..point_count {
            poll_abort(abort, index)?;
            let axis_value = axis_start + (axis_stop - axis_start) * index as Value / denominator;
            frequencies.push(base.powf(axis_value));
        }
    }

    ensure_not_aborted(abort)?;
    Ok(frequencies)
}

/// Validate and copy an explicitly authored frequency list.
pub fn copy_explicit_frequency_grid(
    authored: &[Value],
    abort: &dyn AbortSignal,
) -> Result<Vec<Value>, FrequencyGridError> {
    ensure_not_aborted(abort)?;
    if authored.is_empty() {
        return Err(FrequencyGridError::EmptySweep);
    }
    let mut frequencies = Vec::new();
    frequencies
        .try_reserve_exact(authored.len())
        .map_err(|_| FrequencyGridError::Allocation {
            requested: authored.len(),
        })?;
    for (index, &frequency) in authored.iter().enumerate() {
        poll_abort(abort, index)?;
        if !frequency.is_finite() || frequency <= 0.0 {
            return Err(FrequencyGridError::InvalidExplicitFrequency { index });
        }
        frequencies.push(frequency);
    }
    ensure_not_aborted(abort)?;
    Ok(frequencies)
}

const ABORT_POLL_STRIDE: usize = 256;

#[inline]
fn ensure_not_aborted(abort: &dyn AbortSignal) -> Result<(), FrequencyGridError> {
    if abort.is_aborted() {
        Err(FrequencyGridError::Aborted)
    } else {
        Ok(())
    }
}

#[inline]
fn poll_abort(abort: &dyn AbortSignal, index: usize) -> Result<(), FrequencyGridError> {
    if index % ABORT_POLL_STRIDE == 0 {
        ensure_not_aborted(abort)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::{CountingAbort, ImmediateAbort, NoAbort};

    #[test]
    fn generated_grid_preserves_endpoints_and_scale() {
        assert_eq!(
            generate_frequency_grid(
                1.0,
                1.0e3,
                2,
                FrequencyGridScale::Decade,
                false,
                1,
                &NoAbort,
            )
            .expect("ordinary decade grid"),
            vec![
                1.0,
                10.0_f64.powf(0.6),
                10.0_f64.powf(1.2),
                10.0_f64.powf(1.8),
                10.0_f64.powf(2.4),
                1.0e3
            ]
        );
        assert_eq!(
            generate_frequency_grid(
                0.0,
                Value::MAX,
                3,
                FrequencyGridScale::Linear,
                true,
                1,
                &NoAbort,
            )
            .expect("extreme finite linear grid"),
            vec![0.0, Value::MAX / 2.0, Value::MAX]
        );
    }

    #[test]
    fn generated_grid_rejects_point_count_overflow_before_allocation() {
        assert_eq!(
            generate_frequency_grid(
                f64::MIN_POSITIVE,
                f64::MAX,
                usize::MAX,
                FrequencyGridScale::Decade,
                false,
                1,
                &NoAbort,
            ),
            Err(FrequencyGridError::PointCountOverflow)
        );
    }

    #[test]
    fn generated_grid_reports_allocation_failure() {
        assert_eq!(
            generate_frequency_grid(
                1.0,
                2.0,
                usize::MAX,
                FrequencyGridScale::Linear,
                false,
                1,
                &NoAbort,
            ),
            Err(FrequencyGridError::Allocation {
                requested: usize::MAX
            })
        );
    }

    #[test]
    fn generated_and_explicit_grids_are_cancellable() {
        assert_eq!(
            generate_frequency_grid(
                1.0,
                2.0,
                2,
                FrequencyGridScale::Linear,
                false,
                1,
                &ImmediateAbort,
            ),
            Err(FrequencyGridError::Aborted)
        );
        let authored = vec![1.0; 300];
        let abort = CountingAbort::new(1);
        assert_eq!(
            copy_explicit_frequency_grid(&authored, &abort),
            Err(FrequencyGridError::Aborted)
        );
    }
}
