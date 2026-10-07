//! Bounded continuous event evaluation. Candidates stay on the stack; only
//! selected, budgeted records are retained.

use super::*;
use crate::SimulationError;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};

pub(crate) fn poll(abort: &dyn AbortSignal, work: usize) -> Result<(), SimulationError> {
    if work.is_multiple_of(1024) && abort.is_aborted() {
        Err(SimulationError::Aborted)
    } else {
        Ok(())
    }
}

#[derive(Debug)]
enum EvaluationError {
    Measurement(String),
    Simulation(SimulationError),
}

impl From<String> for EvaluationError {
    fn from(error: String) -> Self {
        Self::Measurement(error)
    }
}

impl From<&str> for EvaluationError {
    fn from(error: &str) -> Self {
        Self::Measurement(error.to_owned())
    }
}

impl From<SimulationError> for EvaluationError {
    fn from(error: SimulationError) -> Self {
        Self::Simulation(error)
    }
}

struct RecordBudget {
    retained: usize,
    limit: usize,
}

impl RecordBudget {
    fn admit(&mut self, count: usize) -> Result<(), SimulationError> {
        let requested = self.retained.saturating_add(count);
        ResourceLimitError::ensure(ResourceKind::ResultValues, requested, self.limit)?;
        self.retained = requested;
        Ok(())
    }

    fn push(
        &mut self,
        records: &mut Vec<ContinuousMeasureRecord>,
        record: ContinuousMeasureRecord,
        fail_value: Option<Value>,
    ) -> Result<(), SimulationError> {
        let record = record.check_fail_value(fail_value);
        self.admit(
            2 + usize::from(record.event_axis.is_some())
                + usize::from(record.trigger_axis.is_some())
                + usize::from(record.target_axis.is_some())
                + usize::from(record.failure_limit.is_some()),
        )?;
        records
            .try_reserve(1)
            .map_err(|source| SimulationError::Allocation {
                object: "continuous measurement records",
                source,
            })?;
        records.push(record);
        Ok(())
    }
}

impl MeasureEngine {
    /// Evaluate continuous measurements using the default resource limits.
    /// Use [`Self::evaluate_continuous_with_limits_and_abort`] to preserve typed
    /// resource/cancellation errors or supply a different execution budget.
    pub fn evaluate_continuous(
        &self,
        axis: &[Value],
        signals: &HashMap<String, &[Value]>,
        segment_starts: &[usize],
    ) -> Vec<ContinuousMeasureResult> {
        self.evaluate_continuous_with_limits_and_abort(
            axis,
            signals,
            segment_starts,
            &ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap_or_else(|error| {
            self.measurements
                .iter()
                .map(|statement| {
                    ContinuousMeasureResult::failed(&statement.name, error.to_string())
                })
                .collect()
        })
    }

    /// Evaluate Xyce `*_CONT` WHEN, FIND, DERIV and TRIG/TARG statements.
    /// Positive occurrences emit a suffix; negative occurrences select one
    /// event from the end. ResultValues counts every retained numeric record
    /// field, with at least one value per failed stream. Admission precedes
    /// allocation, and every waveform scan polls cancellation at most 1024
    /// samples apart, including scans that produce no events.
    pub fn evaluate_continuous_with_limits_and_abort(
        &self,
        axis: &[Value],
        signals: &HashMap<String, &[Value]>,
        segment_starts: &[usize],
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<ContinuousMeasureResult>, SimulationError> {
        poll(abort, 0)?;
        if self.measurements.is_empty() {
            return Ok(Vec::new());
        }
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            axis.len(),
            limits.max_analysis_points,
        )?;
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            self.measurements.len(),
            limits.max_result_values,
        )?;
        let mut invalid = axis
            .is_empty()
            .then(|| "measurement axis is empty".to_owned());
        for (index, value) in axis.iter().enumerate() {
            poll(abort, index)?;
            if !value.is_finite() {
                invalid = Some(format!(
                    "measurement axis contains non-finite sample at index {index}"
                ));
                break;
            }
        }
        for (index, (name, signal)) in signals.iter().enumerate() {
            poll(abort, index)?;
            if invalid.is_none() && signal.len() != axis.len() {
                invalid = Some(format!(
                    "signal '{name}' has {} samples but measurement axis has {}",
                    signal.len(),
                    axis.len()
                ));
            }
        }
        for (index, start) in segment_starts.iter().enumerate() {
            poll(abort, index)?;
            if invalid.is_none()
                && (*start == 0
                    || *start >= axis.len()
                    || index > 0 && *start <= segment_starts[index - 1])
            {
                invalid = Some("measurement segment starts are invalid or unordered".to_owned());
            }
        }
        let indexed_signals = index_measure_signals_with_abort(signals, abort)?;
        let data = MeasureData {
            axis,
            signals: &indexed_signals,
            segment_starts,
        };
        let mut budget = RecordBudget {
            retained: 0,
            limit: limits.max_result_values,
        };
        let mut results = Vec::new();
        for statement in &self.measurements {
            poll(abort, 0)?;
            let before = budget.retained;
            let evaluated = if let Some(error) = &invalid {
                Err(EvaluationError::Measurement(error.clone()))
            } else if statement.name.trim().is_empty() {
                Err("continuous measurement name is empty".into())
            } else if !matches!(
                statement.analysis.to_ascii_uppercase().as_str(),
                "TRAN_CONT" | "DC_CONT" | "AC_CONT" | "NOISE_CONT"
            ) {
                Err(format!("continuous evaluation requires TRAN_CONT, DC_CONT, AC_CONT, or NOISE_CONT, got {}", statement.analysis).into())
            } else {
                evaluate_one(statement, data, &mut budget, abort)
            };
            let result = match evaluated {
                Ok(result) => result,
                Err(EvaluationError::Measurement(error)) => {
                    ContinuousMeasureResult::failed(&statement.name, error)
                }
                Err(EvaluationError::Simulation(error)) => return Err(error),
            };
            if result.failure.is_some() {
                // A domain failure drops the statement's provisional records.
                budget.retained = before;
                let endpoints = result.failure_metadata.map_or(0, |metadata| {
                    usize::from(metadata.trigger_axis.is_some())
                        + usize::from(metadata.target_axis.is_some())
                });
                budget.admit((endpoints + usize::from(statement.fail_value.is_some())).max(1))?;
            }
            results
                .try_reserve(1)
                .map_err(|source| SimulationError::Allocation {
                    object: "continuous measurement streams",
                    source,
                })?;
            results.push(result);
        }
        poll(abort, 0)?;
        Ok(results)
    }
}

fn window_bounds(
    axis: &[Value],
    analysis: &str,
    window: MeasureWindow,
    abort: &dyn AbortSignal,
) -> Result<(Value, Value), SimulationError> {
    let mut ascending = true;
    for (index, pair) in axis.windows(2).enumerate() {
        poll(abort, index)?;
        if pair[0] != pair[1] {
            ascending = pair[1] > pair[0];
            break;
        }
    }
    let (mut lower, upper) = match (window.from, window.to) {
        (Some(from), Some(to)) => (from, to),
        (Some(from), None) if ascending => (from, Value::INFINITY),
        (Some(from), None) => (Value::NEG_INFINITY, from),
        (None, Some(to)) if ascending => (Value::NEG_INFINITY, to),
        (None, Some(to)) => (to, Value::INFINITY),
        (None, None) => (Value::NEG_INFINITY, Value::INFINITY),
    };
    if analysis.eq_ignore_ascii_case("TRAN_CONT")
        && let Some(td) = window.td
    {
        lower = lower.max(td);
    }
    Ok((lower, upper))
}

fn visit_condition_events(
    condition: &WhenCondition,
    window: MeasureWindow,
    bounds: (Value, Value),
    data: MeasureData<'_, '_>,
    abort: &dyn AbortSignal,
    mut emit: impl FnMut(MeasureEvent) -> Result<(), SimulationError>,
) -> Result<(), EvaluationError> {
    let left = lookup_signal(data.signals, &condition.left)
        .ok_or_else(|| format!("When signal '{}' not found", condition.left))?;
    let right = resolve_measure_operand(&condition.right, data.signals)?;
    if !window.minval.is_finite() || window.minval < 0.0 {
        return Ok(());
    }
    let number = condition.occurrence.number;
    let Some(requested) = (if number < 0 {
        number.checked_abs()
    } else {
        Some(number.max(1))
    }) else {
        return Ok(());
    };
    let edge = condition.occurrence.edge;
    let mut count = 0;
    let segments = data.axis.len().saturating_sub(1);
    for scanned in 0..segments {
        poll(abort, scanned)?;
        let segment = if number < 0 {
            segments - 1 - scanned
        } else {
            scanned
        };
        if data.segment_starts.binary_search(&(segment + 1)).is_ok() {
            continue;
        }
        let Some(crossing) = measure_condition_crossing(
            left[segment],
            left[segment + 1],
            right.value_at(segment).unwrap_or(Value::NAN),
            right.value_at(segment + 1).unwrap_or(Value::NAN),
            window.minval,
        ) else {
            continue;
        };
        let event_axis =
            data.axis[segment] + crossing.fraction * (data.axis[segment + 1] - data.axis[segment]);
        if !point_event_axis_in_window(event_axis, bounds.0, bounds.1, window.minval) {
            continue;
        }
        let matches = edge_matches_measure_condition(edge, crossing.direction);
        count += if number < 0 {
            usize::from(matches)
        } else {
            match edge {
                EdgeType::Cross => 1,
                EdgeType::Rise => {
                    usize::from(crossing.direction == MeasureConditionDirection::Rise)
                }
                EdgeType::Fall => {
                    usize::from(crossing.direction != MeasureConditionDirection::Rise)
                }
            }
        };
        if count >= requested as usize && matches {
            emit((
                segment,
                crossing.fraction,
                event_axis,
                crossing.current_within_minval,
            ))?;
            if number < 0 {
                break;
            }
        }
    }
    Ok(())
}

fn evaluate_one(
    statement: &MeasureStatement,
    data: MeasureData<'_, '_>,
    budget: &mut RecordBudget,
    abort: &dyn AbortSignal,
) -> Result<ContinuousMeasureResult, EvaluationError> {
    let mut records = Vec::new();
    let mut emit = |record| budget.push(&mut records, record, statement.fail_value);
    match &statement.measure_type {
        MeasureType::When {
            condition,
            from,
            to,
            td,
            minval,
        } => {
            let window = MeasureWindow {
                from: *from,
                to: *to,
                td: *td,
                minval: *minval,
            };
            let bounds = window_bounds(data.axis, &statement.analysis, window, abort)?;
            visit_condition_events(condition, window, bounds, data, abort, |(_, _, axis, _)| {
                emit(ContinuousMeasureRecord::point(axis, axis))
            })?;
        }
        MeasureType::Find {
            signal,
            at,
            when,
            from,
            to,
            td,
            minval,
        }
        | MeasureType::Derivative {
            signal,
            at,
            when,
            from,
            to,
            td,
            minval,
        } => {
            let derivative = matches!(statement.measure_type, MeasureType::Derivative { .. });
            let signal = lookup_signal(data.signals, signal)
                .ok_or_else(|| format!("Signal '{signal}' not found"))?;
            let window = MeasureWindow {
                from: *from,
                to: *to,
                td: *td,
                minval: *minval,
            };
            let bounds = window_bounds(data.axis, &statement.analysis, window, abort)?;
            if let Some(target) = at {
                if !MeasureEngine::axis_in_measurement_window_with_minval(
                    *target, bounds.0, bounds.1, *minval,
                ) {
                    return Err("AT point is outside the measurement window".into());
                }
                let mut found = None;
                for row in usize::from(derivative)..data.axis.len() {
                    poll(abort, row - usize::from(derivative))?;
                    let starts_segment =
                        row == 0 || data.segment_starts.binary_search(&row).is_ok();
                    let previous = (!starts_segment).then(|| (data.axis[row - 1], signal[row - 1]));
                    let current = (data.axis[row], signal[row]);
                    let value = if derivative {
                        accepted_row_at_match(previous.map(|p| p.0), current.0, *target, *minval)
                            .map(|_| {
                                let previous = previous.unwrap_or(current);
                                accepted_row_secant_slope(
                                    previous.0, previous.1, current.0, current.1,
                                )
                            })
                    } else {
                        find_at_accepted_row(previous, current, *target, *minval)?
                    };
                    if let Some(value) = value {
                        found = Some(value);
                        break;
                    }
                }
                let value = found.ok_or("Time point not in simulation range")?;
                emit(ContinuousMeasureRecord::point(value, *target))?;
            } else {
                let condition = when.as_ref().ok_or(if derivative {
                    "DERIV requires AT=time or WHEN signal=value"
                } else {
                    "FIND requires AT= or WHEN condition"
                })?;
                visit_condition_events(
                    condition,
                    window,
                    bounds,
                    data,
                    abort,
                    |(segment, fraction, axis, current)| {
                        let value = if derivative {
                            accepted_row_secant_slope(
                                data.axis[segment],
                                signal[segment],
                                data.axis[segment + 1],
                                signal[segment + 1],
                            )
                        } else if current {
                            signal[segment + 1]
                        } else {
                            interpolate_extended_real(
                                signal[segment],
                                signal[segment + 1],
                                fraction,
                            )
                        };
                        emit(ContinuousMeasureRecord::point(value, axis))
                    },
                )?;
            }
        }
        MeasureType::Delay {
            trig, targ, minval, ..
        } => {
            if trig.frac_max.is_some() || targ.frac_max.is_some() {
                return Err("FRAC_MAX is supported only by scalar TRAN TRIG/TARG".into());
            }
            let mut triggers = DelayEvents::new(trig, trig.td, *minval, data, abort)?;
            let mut targets = DelayEvents::new(targ, targ.td.or(trig.td), *minval, data, abort)?;
            let trigger = triggers.next(abort)?;
            let target = targets.next(abort)?;
            let (Some(trigger_axis), Some(target_axis)) = (trigger, target) else {
                return Ok(ContinuousMeasureResult::failed_delay(
                    &statement.name,
                    trigger,
                    target,
                    "trigger/target event pair not found",
                ));
            };
            emit(ContinuousMeasureRecord::delay(trigger_axis, target_axis))?;
            while let Some(trigger) = triggers.next(abort)? {
                let Some(target) = targets.next(abort)? else {
                    break;
                };
                emit(ContinuousMeasureRecord::delay(trigger, target))?;
            }
        }
        _ => {
            return Err("continuous measures support only WHEN, FIND, DERIV, and TRIG/TARG".into());
        }
    }
    if records.is_empty() {
        return Err(
            if matches!(statement.measure_type, MeasureType::Derivative { .. }) {
                "WHEN condition never met in the measurement window"
            } else {
                "WHEN condition not found in the measurement window"
            }
            .into(),
        );
    }
    // Constructors and per-record FAILVALUE admission establish the invariants
    // incrementally; do not rescan an arbitrarily large result without polling.
    Ok(ContinuousMeasureResult {
        name: statement.name.clone(),
        records,
        failure: None,
        failure_metadata: None,
    })
}

enum DelayEvents<'a> {
    At(Option<Value>),
    When {
        axis: &'a [Value],
        left: &'a [Value],
        right: ResolvedMeasureOperand<'a>,
        segment_starts: &'a [usize],
        tracker: DelayConditionTracker,
        row: usize,
        td: Option<Value>,
    },
}

impl<'a> DelayEvents<'a> {
    fn new(
        clause: &TrigSpec,
        td: Option<Value>,
        minval: Value,
        data: MeasureData<'a, 'a>,
        abort: &dyn AbortSignal,
    ) -> Result<Self, EvaluationError> {
        match &clause.event {
            TriggerEvent::At(target) => {
                let mut minimum = Value::INFINITY;
                let mut maximum = Value::NEG_INFINITY;
                let mut direction = None;
                for (row, value) in data.axis.iter().copied().enumerate() {
                    poll(abort, row)?;
                    minimum = minimum.min(value);
                    maximum = maximum.max(value);
                    if direction.is_none()
                        && row > 0
                        && data.segment_starts.binary_search(&row).is_err()
                        && value != data.axis[row - 1]
                    {
                        direction = Some(value > data.axis[row - 1]);
                    }
                }
                let mut reached = false;
                if target.is_finite() && *target >= minimum && *target <= maximum {
                    for (row, value) in data.axis.iter().enumerate() {
                        poll(abort, row)?;
                        if if direction.unwrap_or(true) {
                            value - minval >= *target
                        } else {
                            value - minval <= *target
                        } {
                            reached = true;
                            break;
                        }
                    }
                }
                Ok(Self::At(reached.then_some(*target)))
            }
            TriggerEvent::When(condition) => {
                if condition.occurrence.number < 0 {
                    return Err(
                        "negative RISE/FALL/CROSS qualifiers are invalid for continuous TRIG/TARG"
                            .into(),
                    );
                }
                let left = lookup_signal(data.signals, &condition.left)
                    .ok_or_else(|| format!("When signal '{}' not found", condition.left))?;
                let right = resolve_measure_operand(&condition.right, data.signals)?;
                Ok(Self::When {
                    axis: data.axis,
                    left,
                    right,
                    segment_starts: data.segment_starts,
                    tracker: DelayConditionTracker::new_continuous(
                        condition.occurrence.edge,
                        condition.occurrence.number,
                        clause.occurrence_explicit,
                        minval,
                    ),
                    row: 0,
                    td,
                })
            }
        }
    }

    fn next(&mut self, abort: &dyn AbortSignal) -> Result<Option<Value>, SimulationError> {
        match self {
            Self::At(value) => {
                poll(abort, 0)?;
                Ok(value.take())
            }
            Self::When {
                axis,
                left,
                right,
                segment_starts,
                tracker,
                row,
                td,
            } => {
                while *row < axis.len() {
                    poll(abort, *row)?;
                    let index = *row;
                    *row += 1;
                    if segment_starts.binary_search(&index).is_ok() {
                        tracker.reset_segment();
                    }
                    if let Some(right) = right.value_at(index)
                        && let Some(event) =
                            tracker.update_with_td(axis[index], left[index], right, *td)
                    {
                        return Ok(Some(event));
                    }
                }
                Ok(None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::CountingAbort;

    #[test]
    fn every_event_scan_cancels_even_when_it_emits_nothing() {
        let axis: Vec<_> = (0..10_001).map(|row| row as Value).collect();
        let signal = vec![0.0; axis.len()];
        let signals = HashMap::from([("V(out)".to_owned(), signal.as_slice())]);
        let signals = index_measure_signals(&signals);
        let data = MeasureData {
            axis: &axis,
            signals: &signals,
            segment_starts: &[],
        };
        for card in [
            "WHEN V(out)=1 CROSS=1",
            "WHEN V(out)=1 CROSS=-1",
            "FIND V(out) AT=9999.5",
            "DERIV V(out) AT=9999.5",
            "TRIG V(out) VAL=1 CROSS=1 TARG V(out) VAL=2 CROSS=1",
            "TRIG AT=9999.5 TARG AT=9999.5",
        ] {
            let netlist =
                crate::Netlist::parse(&format!("* cancellation\n.MEAS TRAN_CONT m {card}\n.END\n"))
                    .unwrap();
            let abort = CountingAbort::new(2);
            let mut budget = RecordBudget {
                retained: 0,
                limit: usize::MAX,
            };
            assert!(
                matches!(
                    evaluate_one(&netlist.measurements[0], data, &mut budget, &abort),
                    Err(EvaluationError::Simulation(SimulationError::Aborted))
                ),
                "{card}"
            );
            assert_eq!(abort.count(), 3, "{card}");
            assert_eq!(abort.polls_after_abort(), 0, "{card}");
        }
        let abort = CountingAbort::new(2);
        assert!(matches!(
            window_bounds(&signal, "TRAN_CONT", MeasureWindow::default(), &abort),
            Err(SimulationError::Aborted)
        ));
        assert_eq!(abort.count(), 3);
    }
}
