//! Shared bounded signed frequency grids for translated native analyses.
use super::*;
use crate::analysis::frequency_grid::{
    FrequencyGridScale, frequency_point_count, generate_frequency_grid, validate_generated_sweep,
};
use crate::netlist::{FreqVariation, QpacSweep};
use crate::{ResourceKind, ResourceLimitError, ResourceLimits};
fn error(message: impl Into<String>) -> SimulationError {
    SimulationError::Circuit(format!(
        "quasi-periodic frequency sweep: {}",
        message.into()
    ))
}
fn grid_error(e: crate::analysis::FrequencyGridError) -> SimulationError {
    match e {
        crate::analysis::FrequencyGridError::Aborted => SimulationError::Aborted,
        other => error(other.to_string()),
    }
}
fn scale(v: FreqVariation) -> FrequencyGridScale {
    match v {
        FreqVariation::Lin => FrequencyGridScale::Linear,
        FreqVariation::Dec => FrequencyGridScale::Decade,
        FreqVariation::Oct => FrequencyGridScale::Octave,
    }
}
pub(super) fn validate(
    sweep: &QpacSweep,
    limits: &ResourceLimits,
) -> Result<usize, SimulationError> {
    let count = match sweep {
        QpacSweep::Explicit(values) => {
            if values.is_empty()
                || values.iter().any(|v| !v.is_finite())
                || values.windows(2).any(|v| v[0] >= v[1])
            {
                return Err(error(
                    "frequencies must be finite, nonempty and strictly increasing",
                ));
            }
            values.len()
        }
        QpacSweep::Generated(s) => {
            if s.variation == FreqVariation::Lin {
                if !s.start_freq.is_finite()
                    || !s.stop_freq.is_finite()
                    || s.stop_freq < s.start_freq
                    || s.points == 0
                {
                    return Err(error(
                        "LIN requires finite increasing endpoints and a positive point count",
                    ));
                }
            } else {
                validate_generated_sweep(
                    s.start_freq,
                    s.stop_freq,
                    s.points,
                    scale(s.variation),
                    true,
                )
                .map_err(grid_error)?;
            }
            let count = if s.variation == FreqVariation::Lin {
                s.points
            } else {
                frequency_point_count(s.start_freq, s.stop_freq, s.points, scale(s.variation))
                    .map_err(grid_error)?
            };
            if count > 1 && s.start_freq == s.stop_freq {
                return Err(error(
                    "multiple frequency points require distinct endpoints",
                ));
            }
            count
        }
    };
    ResourceLimitError::ensure(
        ResourceKind::AnalysisPoints,
        count,
        limits.max_analysis_points,
    )?;
    ResourceLimitError::ensure(ResourceKind::ResultValues, count, limits.max_result_values)?;
    Ok(count)
}
pub(super) fn resolve(
    sweep: &QpacSweep,
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<Value>, SimulationError> {
    check_abort(abort)?;
    validate(sweep, limits)?;
    let values = match sweep {
        QpacSweep::Explicit(values) => values.clone(),
        QpacSweep::Generated(s) if s.variation == FreqVariation::Lin && s.start_freq < 0.0 => {
            let mut values = generate_frequency_grid(
                0.0,
                1.0,
                s.points,
                FrequencyGridScale::Linear,
                true,
                abort,
            )
            .map_err(grid_error)?;
            let last = values.len() - 1;
            for (i, fraction) in values.iter_mut().enumerate() {
                if i.is_multiple_of(256) {
                    check_abort(abort)?;
                }
                *fraction = if i == 0 {
                    s.start_freq
                } else if i == last {
                    s.stop_freq
                } else {
                    (1.0 - *fraction) * s.start_freq + *fraction * s.stop_freq
                };
            }
            values
        }
        QpacSweep::Generated(s) => generate_frequency_grid(
            s.start_freq,
            s.stop_freq,
            s.points,
            scale(s.variation),
            true,
            abort,
        )
        .map_err(grid_error)?,
    };
    check_abort(abort)?;
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quasi_periodic_log_sweeps_preserve_density_and_off_grid_stops() {
        for (variation, points, start, stop, expected) in [
            (
                FreqVariation::Dec,
                2,
                10.0,
                1000.0,
                vec![
                    10.0,
                    10.0 * 10_f64.sqrt(),
                    100.0,
                    100.0 * 10_f64.sqrt(),
                    1000.0,
                ],
            ),
            (
                FreqVariation::Dec,
                2,
                10.0,
                800.0,
                vec![10.0, 10.0 * 10_f64.sqrt(), 100.0, 100.0 * 10_f64.sqrt()],
            ),
            (FreqVariation::Dec, 10, 100.0, 110.0, vec![100.0]),
            (
                FreqVariation::Oct,
                2,
                8.0,
                20.0,
                vec![8.0, 8.0 * 2_f64.sqrt(), 16.0],
            ),
            (
                FreqVariation::Oct,
                2,
                8.0,
                32.0,
                vec![8.0, 8.0 * 2_f64.sqrt(), 16.0, 16.0 * 2_f64.sqrt(), 32.0],
            ),
        ] {
            let sweep = QpacSweep::Generated(crate::netlist::PeriodicSweep {
                variation,
                points,
                start_freq: start,
                stop_freq: stop,
            });
            let actual = resolve(&sweep, &ResourceLimits::default(), &NoAbort).unwrap();
            assert_eq!(
                validate(&sweep, &ResourceLimits::default()).unwrap(),
                expected.len()
            );
            assert_eq!(actual.len(), expected.len());
            for (a, b) in actual.iter().zip(expected) {
                assert!((a / b - 1.0).abs() < 1e-14, "{actual:?}");
            }
        }
        let sweep = QpacSweep::Generated(crate::netlist::PeriodicSweep {
            variation: FreqVariation::Dec,
            points: 1,
            start_freq: 1e-300,
            stop_freq: 1e300,
        });
        let values = resolve(&sweep, &ResourceLimits::default(), &NoAbort).unwrap();
        assert_eq!(values.len(), 601);
        assert_eq!(values[0], 1e-300);
        assert_eq!(values[600], 1e300);
    }
}
