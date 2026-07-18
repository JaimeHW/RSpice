use rspice_core::abort_signal::AbortSignal;
use rspice_core::netlist::{AnalysisCommand, StepSweep};
use rspice_core::{ResourceKind, ResourceLimits, Value};
use std::fmt;

use super::super::error::{ServiceRunError, ServiceRunResult, poll_periodically};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StepSweepExpandError {
    NonFiniteLinearSweep,
    ZeroLinearStep,
    LinearDirectionMismatch,
    NoLinearPoints,
    TooManyPoints { requested: usize, limit: usize },
    AllocationFailed { requested: usize },
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
            Self::TooManyPoints { requested, limit } => write!(
                f,
                "Parametric sweep requests {requested} runs, exceeding the configured limit of {limit}"
            ),
            Self::AllocationFailed { requested } => write!(
                f,
                "Parametric sweep could not allocate storage for {requested} points"
            ),
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

fn ensure_point_limit(requested: usize, limit: usize) -> Result<(), StepSweepExpandError> {
    if requested <= limit {
        Ok(())
    } else {
        Err(StepSweepExpandError::TooManyPoints { requested, limit })
    }
}

fn checked_total_points(
    span: Value,
    density: usize,
    limit: usize,
) -> Result<usize, StepSweepExpandError> {
    let intervals = span * density as Value;
    let total_points = if !intervals.is_finite() || intervals >= usize::MAX as Value {
        usize::MAX
    } else {
        (intervals.ceil() as usize).saturating_add(1)
    };
    ensure_point_limit(total_points, limit)?;
    Ok(total_points)
}

fn point_buffer(capacity: usize) -> Result<Vec<Value>, StepSweepExpandError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(capacity)
        .map_err(|_| StepSweepExpandError::AllocationFailed {
            requested: capacity,
        })?;
    Ok(values)
}

pub(crate) fn expand_step_sweep_values(
    sweep: &StepSweep,
) -> Result<Vec<Value>, StepSweepExpandError> {
    match expand_step_sweep_values_impl(sweep, ResourceLimits::default().max_batch_runs, None) {
        Ok(values) => Ok(values),
        Err(StepSweepExpansionFailure::Invalid(error)) => Err(error),
        Err(StepSweepExpansionFailure::Aborted) => {
            unreachable!("NoAbort expansion cannot report cancellation")
        }
    }
}

pub(super) fn expand_step_sweep_values_with_abort(
    sweep: &StepSweep,
    max_batch_runs: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Value>> {
    expand_step_sweep_values_impl(sweep, max_batch_runs, Some(abort)).map_err(|error| match error {
        StepSweepExpansionFailure::Invalid(StepSweepExpandError::TooManyPoints {
            requested,
            limit,
        }) => ServiceRunError::resource_limit(ResourceKind::BatchRuns, requested, limit),
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
    max_batch_runs: usize,
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
                ensure_point_limit(1, max_batch_runs)?;
                return Ok(vec![*start]);
            }

            let raw_intervals = (stop - start).abs() / step.abs();
            let estimated_points =
                if !raw_intervals.is_finite() || raw_intervals >= usize::MAX as Value {
                    usize::MAX
                } else {
                    (raw_intervals.floor() as usize).saturating_add(1)
                };
            ensure_point_limit(estimated_points, max_batch_runs)?;
            let capacity = estimated_points.saturating_add(1).min(max_batch_runs);
            let mut values = point_buffer(capacity)?;
            let mut current = *start;
            let tolerance = (step.abs() * 1e-12).max((start.abs().max(stop.abs())) * 1e-12);

            if *step > 0.0 {
                while current <= *stop + tolerance {
                    poll_expansion(abort, values.len())?;
                    if values.len() == max_batch_runs {
                        return Err(StepSweepExpandError::TooManyPoints {
                            requested: max_batch_runs.saturating_add(1),
                            limit: max_batch_runs,
                        }
                        .into());
                    }
                    values.push(current);
                    current += *step;
                }
            } else {
                while current >= *stop - tolerance {
                    poll_expansion(abort, values.len())?;
                    if values.len() == max_batch_runs {
                        return Err(StepSweepExpandError::TooManyPoints {
                            requested: max_batch_runs.saturating_add(1),
                            limit: max_batch_runs,
                        }
                        .into());
                    }
                    values.push(current);
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
            let total_points = checked_total_points(span, *points_per_decade, max_batch_runs)?;
            let denom = (total_points - 1).max(1) as f64;
            let mut values = point_buffer(total_points)?;
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
            let total_points = checked_total_points(span, *points_per_octave, max_batch_runs)?;
            let denom = (total_points - 1).max(1) as f64;
            let mut values = point_buffer(total_points)?;
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
            ensure_point_limit(values.len(), max_batch_runs)?;
            if values.iter().any(|value| !value.is_finite()) {
                return Err(StepSweepExpandError::NonFiniteListValues.into());
            }
            let mut copied = point_buffer(values.len())?;
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
    max_batch_runs: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Value>> {
    let mut temperatures: Vec<Value> = Vec::new();
    let mut sorted_temperatures: Vec<Value> = Vec::new();
    for (analysis_index, analysis) in netlist.analyses.iter().enumerate() {
        poll_periodically(abort, analysis_index)?;
        if let AnalysisCommand::Temp {
            temperatures: temps,
        } = analysis
        {
            for (temp_index, &temp) in temps.iter().enumerate() {
                poll_periodically(abort, temp_index)?;
                let insertion = sorted_temperatures
                    .partition_point(|existing| existing.total_cmp(&temp).is_lt());
                let already_present = insertion
                    .checked_sub(1)
                    .and_then(|index| sorted_temperatures.get(index))
                    .is_some_and(|existing| (*existing - temp).abs() < 1e-15)
                    || sorted_temperatures
                        .get(insertion)
                        .is_some_and(|existing| (*existing - temp).abs() < 1e-15);
                if !already_present {
                    let requested = temperatures.len().saturating_add(1);
                    if requested > max_batch_runs {
                        return Err(ServiceRunError::resource_limit(
                            ResourceKind::BatchRuns,
                            requested,
                            max_batch_runs,
                        ));
                    }
                    temperatures.push(temp);
                    sorted_temperatures.insert(insertion, temp);
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

        let result = expand_step_sweep_values_with_abort(&sweep, 512, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert!(abort.polls.load(Ordering::Relaxed) >= 3);
    }

    #[test]
    fn list_sweep_rejects_configured_batch_limit_before_copying() {
        let sweep = StepSweep::List(vec![1.0, 2.0, 3.0]);

        let result = expand_step_sweep_values_with_abort(&sweep, 2, &rspice_core::NoAbort);

        assert!(matches!(
            result,
            Err(ServiceRunError::ResourceLimit(
                rspice_core::ResourceLimitError {
                    resource: ResourceKind::BatchRuns,
                    requested: 3,
                    limit: 2,
                }
            ))
        ));
    }

    #[test]
    fn linear_sweep_rejects_configured_batch_limit_before_expansion() {
        let sweep = StepSweep::Linear {
            start: 0.0,
            stop: 10.0,
            step: 1.0,
        };

        let result = expand_step_sweep_values_with_abort(&sweep, 4, &rspice_core::NoAbort);

        assert!(matches!(
            result,
            Err(ServiceRunError::ResourceLimit(
                rspice_core::ResourceLimitError {
                    resource: ResourceKind::BatchRuns,
                    requested: 11,
                    limit: 4,
                }
            ))
        ));
    }
}
