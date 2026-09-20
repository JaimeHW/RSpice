//! Checked frequency-grid construction shared by physical analyses.

use crate::Value;
use crate::abort_signal::AbortSignal;

/// Sweep scale used to construct a frequency grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FrequencyGridScale {
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
    /// The requested grid cannot retain distinct finite values within its endpoints.
    UnrepresentableSpacing,
    /// No frequency points were requested.
    EmptySweep,
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
            Self::UnrepresentableSpacing => {
                formatter.write_str("frequency spacing cannot be represented as distinct finite values within the requested range")
            }
            Self::EmptySweep => {
                formatter.write_str("frequency sweep must contain at least one point")
            }
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
pub(crate) fn validate_generated_sweep(
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
/// The caller must first validate the same endpoints, point parameter and scale.
/// Logarithmic grids use the authored density anchored at start; the stop is
/// retained only when it falls on that grid. A sub-step span has one point.
pub(crate) fn frequency_point_count(
    start: Value,
    stop: Value,
    point_parameter: usize,
    scale: FrequencyGridScale,
) -> Result<usize, FrequencyGridError> {
    match scale {
        FrequencyGridScale::Linear => Ok(point_parameter),
        _ => logarithmic_layout(start, stop, point_parameter, scale).map(|layout| layout.count),
    }
}

struct LogarithmicLayout {
    count: usize,
    span_steps: Value,
    aligned_stop: bool,
}

fn logarithmic_layout(
    start: Value,
    stop: Value,
    density: usize,
    scale: FrequencyGridScale,
) -> Result<LogarithmicLayout, FrequencyGridError> {
    let base_log = match scale {
        FrequencyGridScale::Decade => std::f64::consts::LN_10,
        FrequencyGridScale::Octave => std::f64::consts::LN_2,
        FrequencyGridScale::Linear => unreachable!("linear grid has no logarithmic layout"),
    };
    let ratio = stop / start;
    let span = if ratio.is_finite() {
        ratio.ln()
    } else {
        stop.ln() - start.ln()
    };
    let span_steps = span / base_log * density as Value;
    // Leave room for the starting point. usize::MAX may round upward as f64.
    if !span_steps.is_finite() || span_steps >= (usize::MAX - 1) as Value {
        return Err(FrequencyGridError::PointCountOverflow);
    }
    let rounded = span_steps.round();
    let aligned_stop =
        (span_steps - rounded).abs() <= 8.0 * Value::EPSILON * span_steps.abs().max(1.0);
    let intervals = if aligned_stop {
        rounded
    } else {
        span_steps.floor()
    } as usize;
    Ok(LogarithmicLayout {
        count: intervals + 1,
        span_steps,
        aligned_stop,
    })
}

/// Construct a validated generated grid with fallible retention and cancellation.
/// LIN is a total point count; DEC/OCT are fixed densities per logarithmic unit.
/// No off-grid stop is appended or used to stretch the authored density.
pub(crate) fn generate_frequency_grid(
    start: Value,
    stop: Value,
    point_parameter: usize,
    scale: FrequencyGridScale,
    linear_start_may_be_zero: bool,
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
    let layout = if scale == FrequencyGridScale::Linear {
        None
    } else {
        Some(logarithmic_layout(start, stop, point_parameter, scale)?)
    };
    let count = layout
        .as_ref()
        .map_or(point_parameter, |layout| layout.count);
    let mut frequencies = Vec::new();
    frequencies
        .try_reserve_exact(count)
        .map_err(|_| FrequencyGridError::Allocation { requested: count })?;
    let low = start.ln();
    let high = stop.ln();
    for index in 0..count {
        poll_abort(abort, index)?;
        let value = if index == 0 {
            start
        } else if let Some(layout) = &layout {
            if layout.aligned_stop && index == count - 1 {
                stop
            } else {
                // Convex interpolation in log space avoids both ratio overflow
                // and the overflow of base^(index/density) for extreme spans.
                let fraction = index as Value / layout.span_steps;
                ((1.0 - fraction) * low + fraction * high).exp()
            }
        } else if index == count - 1 {
            stop
        } else {
            start + index as Value * ((stop - start) / (count - 1) as Value)
        };
        if !value.is_finite()
            || value < start
            || value > stop
            || (start < stop
                && frequencies
                    .last()
                    .is_some_and(|previous| value <= *previous))
        {
            return Err(FrequencyGridError::UnrepresentableSpacing);
        }
        frequencies.push(value);
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
    if index.is_multiple_of(ABORT_POLL_STRIDE) {
        ensure_not_aborted(abort)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::{CountingAbort, ImmediateAbort, NoAbort};

    #[test]
    fn periodic_frequency_density_reaches_pac_pxf_stb_and_transfer_configs() {
        use crate::analysis::{
            pac::{PacConfig, PacSweepType},
            pxf::{PxfConfig, PxfSweepType},
            stb::{StbConfig, StbSweepType},
            transfer::{AcSweepType, AcTransferConfig},
        };
        for (octave, start, stop, density, expected) in [
            (
                false,
                10.0,
                1000.0,
                2,
                vec![
                    10.0,
                    10.0 * 10_f64.sqrt(),
                    100.0,
                    100.0 * 10_f64.sqrt(),
                    1000.0,
                ],
            ),
            (
                false,
                10.0,
                800.0,
                2,
                vec![10.0, 10.0 * 10_f64.sqrt(), 100.0, 100.0 * 10_f64.sqrt()],
            ),
            (true, 8.0, 20.0, 2, vec![8.0, 8.0 * 2_f64.sqrt(), 16.0]),
            (
                true,
                8.0,
                32.0,
                2,
                vec![8.0, 8.0 * 2_f64.sqrt(), 16.0, 16.0 * 2_f64.sqrt(), 32.0],
            ),
            (false, 100.0, 110.0, 10, vec![100.0]),
            (true, 8.0, 8.0, 20, vec![8.0]),
        ] {
            let pac = PacConfig::new()
                .with_sweep(start, stop, density)
                .with_sweep_type(if octave {
                    PacSweepType::Octave
                } else {
                    PacSweepType::Decade
                });
            let pxf = PxfConfig::new()
                .with_sweep(start, stop, density)
                .with_sweep_type(if octave {
                    PxfSweepType::Octave
                } else {
                    PxfSweepType::Decade
                });
            let stb = StbConfig::new()
                .with_sweep(start, stop, density)
                .with_sweep_type(if octave {
                    StbSweepType::Octave
                } else {
                    StbSweepType::Decade
                });
            let mut transfer = AcTransferConfig::decade("out", "V1", start, stop, density);
            if octave {
                transfer.sweep_type = AcSweepType::Octave;
            }
            for count in [
                pac.frequency_point_count().unwrap(),
                pxf.frequency_point_count().unwrap(),
                stb.frequency_point_count().unwrap(),
            ] {
                assert_eq!(count, expected.len());
            }
            for grid in [
                pac.frequency_points().unwrap(),
                pxf.frequency_points().unwrap(),
                stb.frequency_points().unwrap(),
                transfer.frequency_points().unwrap(),
            ] {
                assert_eq!(grid.len(), expected.len());
                for (&actual, &expected) in grid.iter().zip(&expected) {
                    assert!((actual / expected - 1.0).abs() < 1e-14, "{grid:?}");
                }
            }
        }
    }

    #[test]
    fn generated_grid_preserves_endpoints_and_scale() {
        let logarithmic =
            generate_frequency_grid(1.0, 1.0e3, 2, FrequencyGridScale::Decade, false, &NoAbort)
                .unwrap();
        assert_eq!(logarithmic.len(), 7);
        for (index, frequency) in logarithmic.iter().enumerate() {
            let expected = 10.0_f64.powf(index as f64 / 2.0);
            assert!((frequency / expected - 1.0).abs() < 1e-14);
        }
        assert_eq!(
            generate_frequency_grid(
                0.0,
                Value::MAX,
                3,
                FrequencyGridScale::Linear,
                true,
                &NoAbort,
            )
            .expect("extreme finite linear grid"),
            vec![0.0, Value::MAX / 2.0, Value::MAX]
        );
    }

    #[test]
    fn generated_grid_preserves_extreme_finite_endpoints_exactly() {
        for scale in [
            FrequencyGridScale::Linear,
            FrequencyGridScale::Decade,
            FrequencyGridScale::Octave,
        ] {
            let start = Value::MAX
                / if scale == FrequencyGridScale::Decade {
                    1000.0
                } else {
                    1024.0
                };
            let grid = generate_frequency_grid(start, Value::MAX, 32, scale, false, &NoAbort)
                .expect("representable extreme sweep");
            assert_eq!(grid.first(), Some(&start), "{scale:?}");
            assert_eq!(grid.last(), Some(&Value::MAX), "{scale:?}");
            assert!(grid.iter().all(|value| value.is_finite()), "{scale:?}");
            assert!(grid.windows(2).all(|pair| pair[0] < pair[1]), "{scale:?}");
        }
    }

    #[test]
    fn generated_grid_refuses_unrepresentable_spacing() {
        assert!(
            generate_frequency_grid(
                1.0,
                1.0_f64.next_up(),
                3,
                FrequencyGridScale::Linear,
                false,
                &NoAbort
            )
            .is_err()
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
                &NoAbort,
            ),
            Err(FrequencyGridError::Allocation {
                requested: usize::MAX
            })
        );
    }

    #[test]
    fn generated_grids_are_cancellable_before_and_during_construction() {
        assert_eq!(
            generate_frequency_grid(
                1.0,
                2.0,
                2,
                FrequencyGridScale::Linear,
                false,
                &ImmediateAbort,
            ),
            Err(FrequencyGridError::Aborted)
        );
        let abort = CountingAbort::new(1);
        assert_eq!(
            generate_frequency_grid(1.0, 2.0, 300, FrequencyGridScale::Linear, false, &abort),
            Err(FrequencyGridError::Aborted)
        );
    }
}
