use rspice_core::netlist::{AnalysisCommand, StepSweep};
use rspice_core::Value;
use std::fmt;

const MAX_SWEEP_POINTS: usize = 1_000_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum StepSweepExpandError {
    NonFiniteLinearSweep,
    ZeroLinearStep,
    LinearDirectionMismatch,
    NoLinearPoints,
    TooManyPoints,
    ZeroDecadeDensity,
    NonPositiveDecadeBounds,
    ZeroOctaveDensity,
    NonPositiveOctaveBounds,
    EmptyList,
    NonFiniteListValues,
}

impl fmt::Display for StepSweepExpandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteLinearSweep => {
                f.write_str("Parametric linear sweep requires finite start/stop/step")
            }
            Self::ZeroLinearStep => f.write_str("Parametric linear sweep step cannot be zero"),
            Self::LinearDirectionMismatch => {
                f.write_str("Parametric linear sweep step direction must match start/stop")
            }
            Self::NoLinearPoints => f.write_str("Parametric linear sweep produced no points"),
            Self::TooManyPoints => {
                f.write_str("Parametric sweep exceeds maximum supported point count")
            }
            Self::ZeroDecadeDensity => {
                f.write_str("Parametric decade sweep points_per_decade must be > 0")
            }
            Self::NonPositiveDecadeBounds => {
                f.write_str("Parametric decade sweep requires positive finite start/stop")
            }
            Self::ZeroOctaveDensity => {
                f.write_str("Parametric octave sweep points_per_octave must be > 0")
            }
            Self::NonPositiveOctaveBounds => {
                f.write_str("Parametric octave sweep requires positive finite start/stop")
            }
            Self::EmptyList => f.write_str("Parametric LIST sweep requires at least one value"),
            Self::NonFiniteListValues => {
                f.write_str("Parametric LIST sweep requires finite values")
            }
        }
    }
}

fn checked_total_points(span: Value, density: usize) -> Result<usize, StepSweepExpandError> {
    let total_points = (span * density as Value).ceil() as usize + 1;
    if total_points > MAX_SWEEP_POINTS {
        return Err(StepSweepExpandError::TooManyPoints);
    }
    Ok(total_points)
}

pub(super) fn expand_step_sweep_values(
    sweep: &StepSweep,
) -> Result<Vec<Value>, StepSweepExpandError> {
    match sweep {
        StepSweep::Linear { start, stop, step } => {
            if !start.is_finite() || !stop.is_finite() || !step.is_finite() {
                return Err(StepSweepExpandError::NonFiniteLinearSweep);
            }
            if *step == 0.0 {
                return Err(StepSweepExpandError::ZeroLinearStep);
            }
            if (stop - start).signum() != step.signum() && (stop - start).abs() > 0.0 {
                return Err(StepSweepExpandError::LinearDirectionMismatch);
            }

            if (stop - start).abs() == 0.0 {
                return Ok(vec![*start]);
            }

            let est_points =
                (((stop - start).abs() / step.abs()).ceil() as usize + 2).min(MAX_SWEEP_POINTS);
            let mut values = Vec::with_capacity(est_points);
            let mut current = *start;
            let tolerance = (step.abs() * 1e-12).max((start.abs().max(stop.abs())) * 1e-12);

            if *step > 0.0 {
                while current <= *stop + tolerance {
                    values.push(current);
                    if values.len() > MAX_SWEEP_POINTS {
                        return Err(StepSweepExpandError::TooManyPoints);
                    }
                    current += *step;
                }
            } else {
                while current >= *stop - tolerance {
                    values.push(current);
                    if values.len() > MAX_SWEEP_POINTS {
                        return Err(StepSweepExpandError::TooManyPoints);
                    }
                    current += *step;
                }
            }

            if values.is_empty() {
                return Err(StepSweepExpandError::NoLinearPoints);
            }

            Ok(values)
        }
        StepSweep::Decade {
            points_per_decade,
            start,
            stop,
        } => {
            if *points_per_decade == 0 {
                return Err(StepSweepExpandError::ZeroDecadeDensity);
            }
            if !start.is_finite() || !stop.is_finite() || *start <= 0.0 || *stop <= 0.0 {
                return Err(StepSweepExpandError::NonPositiveDecadeBounds);
            }
            let start_log = start.log10();
            let stop_log = stop.log10();
            let span = (stop_log - start_log).abs();
            let total_points = checked_total_points(span, *points_per_decade)?;
            let denom = (total_points - 1).max(1) as f64;
            Ok((0..total_points)
                .map(|i| {
                    let t = i as f64 / denom;
                    let log_value = start_log + (stop_log - start_log) * t;
                    10.0_f64.powf(log_value)
                })
                .collect())
        }
        StepSweep::Octave {
            points_per_octave,
            start,
            stop,
        } => {
            if *points_per_octave == 0 {
                return Err(StepSweepExpandError::ZeroOctaveDensity);
            }
            if !start.is_finite() || !stop.is_finite() || *start <= 0.0 || *stop <= 0.0 {
                return Err(StepSweepExpandError::NonPositiveOctaveBounds);
            }
            let start_log = start.log2();
            let stop_log = stop.log2();
            let span = (stop_log - start_log).abs();
            let total_points = checked_total_points(span, *points_per_octave)?;
            let denom = (total_points - 1).max(1) as f64;
            Ok((0..total_points)
                .map(|i| {
                    let t = i as f64 / denom;
                    let log_value = start_log + (stop_log - start_log) * t;
                    2.0_f64.powf(log_value)
                })
                .collect())
        }
        StepSweep::List(values) => {
            if values.is_empty() {
                return Err(StepSweepExpandError::EmptyList);
            }
            if values.iter().any(|value| !value.is_finite()) {
                return Err(StepSweepExpandError::NonFiniteListValues);
            }
            Ok(values.clone())
        }
    }
}

pub(super) fn extract_temp_points(netlist: &rspice_core::Netlist) -> Vec<Value> {
    let mut temperatures: Vec<Value> = Vec::new();
    for analysis in &netlist.analyses {
        if let AnalysisCommand::Temp {
            temperatures: temps,
        } = analysis
        {
            for &temp in temps {
                if !temperatures
                    .iter()
                    .any(|existing| (*existing - temp).abs() < 1e-15)
                {
                    temperatures.push(temp);
                }
            }
        }
    }
    temperatures
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_linear_sweep_with_expected_points() {
        let sweep = StepSweep::Linear {
            start: 0.0,
            stop: 1.0,
            step: 0.5,
        };
        let values = expand_step_sweep_values(&sweep).expect("linear sweep should expand");
        assert_eq!(values, vec![0.0, 0.5, 1.0]);
    }

    #[test]
    fn test_expand_linear_sweep_rejects_zero_step() {
        let sweep = StepSweep::Linear {
            start: 0.0,
            stop: 1.0,
            step: 0.0,
        };
        assert_eq!(
            expand_step_sweep_values(&sweep),
            Err(StepSweepExpandError::ZeroLinearStep)
        );
    }

    #[test]
    fn test_expand_decade_sweep_generates_requested_density() {
        let sweep = StepSweep::Decade {
            points_per_decade: 10,
            start: 1.0,
            stop: 10.0,
        };
        let values = expand_step_sweep_values(&sweep).expect("decade sweep should expand");
        assert_eq!(values.len(), 11);
        assert!((values[0] - 1.0).abs() < 1e-12);
        assert!((values[10] - 10.0).abs() < 1e-9);
    }

    #[test]
    fn test_expand_list_sweep_rejects_non_finite_values() {
        let sweep = StepSweep::List(vec![1.0, f64::NAN]);
        assert_eq!(
            expand_step_sweep_values(&sweep),
            Err(StepSweepExpandError::NonFiniteListValues)
        );
    }

    #[test]
    fn test_extract_temp_points_deduplicates_values() {
        let netlist = rspice_core::netlist::parse_netlist(
            r#"Temp extraction
.TEMP 25 25 85
.TEMP 85 -40
.END
"#,
        )
        .expect("temperature netlist should parse");
        let temps = extract_temp_points(&netlist);
        assert_eq!(temps, vec![25.0, 85.0, -40.0]);
    }
}
