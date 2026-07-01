use rspice_core::Value;
use rspice_core::netlist::{AnalysisCommand, StepSweep};
use std::fmt;

const MAX_SWEEP_POINTS: usize = 1_000_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StepSweepExpandError {
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

pub(crate) fn expand_step_sweep_values(
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
        StepSweep::Data { .. } => Ok(Vec::new()),
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
