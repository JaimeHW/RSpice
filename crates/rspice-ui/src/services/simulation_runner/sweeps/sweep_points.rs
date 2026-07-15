use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::netlist::{AnalysisCommand, StepSweep};
use std::fmt;

use super::super::error::{ServiceRunError, ServiceRunResult, poll_periodically};

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
    match expand_step_sweep_values_impl(sweep, None) {
        Ok(values) => Ok(values),
        Err(StepSweepExpansionFailure::Invalid(error)) => Err(error),
        Err(StepSweepExpansionFailure::Aborted) => {
            unreachable!("NoAbort expansion cannot report cancellation")
        }
    }
}

pub(super) fn expand_step_sweep_values_with_abort(
    sweep: &StepSweep,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Value>> {
    expand_step_sweep_values_impl(sweep, Some(abort)).map_err(|error| match error {
        StepSweepExpansionFailure::Invalid(error) => ServiceRunError::Failure(error.to_string()),
        StepSweepExpansionFailure::Aborted => ServiceRunError::Aborted,
    })
}

#[derive(Debug)]
enum StepSweepExpansionFailure {
    Invalid(StepSweepExpandError),
    Aborted,
}

impl From<StepSweepExpandError> for StepSweepExpansionFailure {
    fn from(error: StepSweepExpandError) -> Self {
        Self::Invalid(error)
    }
}

fn poll_expansion(
    abort: Option<&dyn AbortSignal>,
    index: usize,
) -> Result<(), StepSweepExpansionFailure> {
    if let Some(abort) = abort {
        poll_periodically(abort, index).map_err(|_| StepSweepExpansionFailure::Aborted)?;
    }
    Ok(())
}

fn expand_step_sweep_values_impl(
    sweep: &StepSweep,
    abort: Option<&dyn AbortSignal>,
) -> Result<Vec<Value>, StepSweepExpansionFailure> {
    poll_expansion(abort, 0)?;
    match sweep {
        StepSweep::Linear { start, stop, step } => {
            if !start.is_finite() || !stop.is_finite() || !step.is_finite() {
                return Err(StepSweepExpandError::NonFiniteLinearSweep.into());
            }
            if *step == 0.0 {
                return Err(StepSweepExpandError::ZeroLinearStep.into());
            }
            if (stop - start).signum() != step.signum() && (stop - start).abs() > 0.0 {
                return Err(StepSweepExpandError::LinearDirectionMismatch.into());
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
                    poll_expansion(abort, values.len())?;
                    values.push(current);
                    if values.len() > MAX_SWEEP_POINTS {
                        return Err(StepSweepExpandError::TooManyPoints.into());
                    }
                    current += *step;
                }
            } else {
                while current >= *stop - tolerance {
                    poll_expansion(abort, values.len())?;
                    values.push(current);
                    if values.len() > MAX_SWEEP_POINTS {
                        return Err(StepSweepExpandError::TooManyPoints.into());
                    }
                    current += *step;
                }
            }

            if values.is_empty() {
                return Err(StepSweepExpandError::NoLinearPoints.into());
            }

            poll_expansion(abort, values.len())?;
            Ok(values)
        }
        StepSweep::Decade {
            points_per_decade,
            start,
            stop,
        } => {
            if *points_per_decade == 0 {
                return Err(StepSweepExpandError::ZeroDecadeDensity.into());
            }
            if !start.is_finite() || !stop.is_finite() || *start <= 0.0 || *stop <= 0.0 {
                return Err(StepSweepExpandError::NonPositiveDecadeBounds.into());
            }
            let start_log = start.log10();
            let stop_log = stop.log10();
            let span = (stop_log - start_log).abs();
            let total_points = checked_total_points(span, *points_per_decade)?;
            let denom = (total_points - 1).max(1) as f64;
            let mut values = Vec::with_capacity(total_points);
            for index in 0..total_points {
                poll_expansion(abort, index)?;
                let t = index as f64 / denom;
                let log_value = start_log + (stop_log - start_log) * t;
                values.push(10.0_f64.powf(log_value));
            }
            poll_expansion(abort, total_points)?;
            Ok(values)
        }
        StepSweep::Octave {
            points_per_octave,
            start,
            stop,
        } => {
            if *points_per_octave == 0 {
                return Err(StepSweepExpandError::ZeroOctaveDensity.into());
            }
            if !start.is_finite() || !stop.is_finite() || *start <= 0.0 || *stop <= 0.0 {
                return Err(StepSweepExpandError::NonPositiveOctaveBounds.into());
            }
            let start_log = start.log2();
            let stop_log = stop.log2();
            let span = (stop_log - start_log).abs();
            let total_points = checked_total_points(span, *points_per_octave)?;
            let denom = (total_points - 1).max(1) as f64;
            let mut values = Vec::with_capacity(total_points);
            for index in 0..total_points {
                poll_expansion(abort, index)?;
                let t = index as f64 / denom;
                let log_value = start_log + (stop_log - start_log) * t;
                values.push(2.0_f64.powf(log_value));
            }
            poll_expansion(abort, total_points)?;
            Ok(values)
        }
        StepSweep::List(values) => {
            if values.is_empty() {
                return Err(StepSweepExpandError::EmptyList.into());
            }
            if values.iter().any(|value| !value.is_finite()) {
                return Err(StepSweepExpandError::NonFiniteListValues.into());
            }
            let mut copied = Vec::with_capacity(values.len());
            for (index, value) in values.iter().copied().enumerate() {
                poll_expansion(abort, index)?;
                copied.push(value);
            }
            poll_expansion(abort, values.len())?;
            Ok(copied)
        }
        StepSweep::Data { .. } => Ok(Vec::new()),
    }
}

pub(super) fn extract_temp_points_with_abort(
    netlist: &rspice_core::Netlist,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Value>> {
    let mut temperatures: Vec<Value> = Vec::new();
    for (analysis_index, analysis) in netlist.analyses.iter().enumerate() {
        poll_periodically(abort, analysis_index)?;
        if let AnalysisCommand::Temp {
            temperatures: temps,
        } = analysis
        {
            for (temp_index, &temp) in temps.iter().enumerate() {
                poll_periodically(abort, temp_index)?;
                let mut already_present = false;
                for (existing_index, existing) in temperatures.iter().enumerate() {
                    poll_periodically(abort, existing_index)?;
                    if (*existing - temp).abs() < 1e-15 {
                        already_present = true;
                        break;
                    }
                }
                if !already_present {
                    temperatures.push(temp);
                }
            }
        }
    }
    super::super::error::ensure_not_aborted(abort)?;
    Ok(temperatures)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    struct AbortOnPoll {
        abort_on: usize,
        polls: AtomicUsize,
    }

    impl AbortOnPoll {
        fn new(abort_on: usize) -> Self {
            Self {
                abort_on,
                polls: AtomicUsize::new(0),
            }
        }
    }

    impl AbortSignal for AbortOnPoll {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.abort_on
        }
    }

    #[test]
    fn step_expansion_honors_abort_inside_large_list_copy() {
        let sweep = StepSweep::List((0..512).map(|index| index as Value).collect());
        let abort = AbortOnPoll::new(3);

        let result = expand_step_sweep_values_with_abort(&sweep, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert!(abort.polls.load(Ordering::Relaxed) >= 3);
    }
}
