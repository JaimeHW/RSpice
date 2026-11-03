//! Measures a Xyce `.PRINT` card names, printed as they stand at each point.
use super::*;
use crate::analysis::measure::EventOccurrence;
use std::borrow::Cow;

/// The measures of one analysis that a Xyce `.PRINT` card of that analysis
/// names, bare (`maxvr4`) or inside an expression (`{vv1-vv2}`), by
/// canonical name, each with the first request and operand that names it, in
/// card order. A continuous `_CONT` measure of the analysis counts.
///
/// Xyce builds a `MeasureOp` for a print operand that names a measure
/// (N_IO_OpBuilders.C:1590-1601); it reads the measure's current result
/// (N_IO_Op.C:1793-1796), which every point updates before its row is
/// written. No other dialect has such an operand, so there the name stays an
/// unknown signal.
fn printed_measure_names<'a>(
    netlist: &'a Netlist,
    analysis: &str,
) -> Vec<(String, &'a OutputRequest, usize)> {
    let mut printed = Vec::new();
    let Some(kind) = OutputAnalysisKind::from_keyword(analysis) else {
        return printed;
    };
    if netlist.parse_dialect() != crate::config::SpiceDialect::Xyce {
        return printed;
    }
    let continuous = format!("{analysis}_CONT");
    let measures = netlist
        .measurements
        .iter()
        .filter(|statement| {
            statement.analysis.eq_ignore_ascii_case(analysis)
                || statement.analysis.eq_ignore_ascii_case(&continuous)
        })
        .map(|statement| statement.name.to_ascii_uppercase())
        .collect::<HashSet<_>>();
    if measures.is_empty() {
        return printed;
    }
    for request in matching_print_requests(netlist, kind) {
        for (operand_index, operand) in request.operand_kinds().iter().enumerate() {
            let text = match operand {
                OutputOperandKind::Probe(SaveSignal::Raw(name)) => name.as_str(),
                OutputOperandKind::Expression { body } => body.as_str(),
                OutputOperandKind::Probe(_) => continue,
            };
            for identifier in identifiers(text) {
                let canonical = identifier.to_ascii_uppercase();
                if measures.contains(&canonical)
                    && !printed.iter().any(|(name, _, _)| *name == canonical)
                {
                    printed.push((canonical, request, operand_index));
                }
            }
        }
    }
    printed
}

/// The identifiers of an operand's text, function and accessor names
/// included: a spare one only costs a measure trace nobody prints.
fn identifiers(text: &str) -> impl Iterator<Item = &str> {
    text.split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .filter(|token| {
            token
                .chars()
                .next()
                .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        })
}

/// The netlist the continuous `_CONT` measures of `analysis` a print card
/// names evaluate over, `None` when it names none.
///
/// A row prints what such a measure has found by then, so one that counts
/// its event from the end (`CROSS=LAST`, `RISE=-2`) is evaluated counting
/// from the start: its records are then every occurrence, and each row picks
/// the one its count selects among those it has reached.
fn printed_continuous_netlist<'a>(
    netlist: &'a Netlist,
    analysis: &str,
    printed: &HashSet<String>,
) -> Option<Cow<'a, Netlist>> {
    let continuous = format!("{analysis}_CONT");
    let selected = |statement: &MeasureStatement| {
        statement.analysis.eq_ignore_ascii_case(&continuous)
            && printed.contains(&statement.name.to_ascii_uppercase())
    };
    if !netlist.measurements.iter().any(selected) {
        return None;
    }
    if !netlist
        .measurements
        .iter()
        .any(|statement| selected(statement) && from_end_count(statement).is_some())
    {
        return Some(Cow::Borrowed(netlist));
    }
    let mut forward = netlist.clone();
    for statement in &mut forward.measurements {
        if selected(statement)
            && let Some(occurrence) = counted_event(&mut statement.measure_type)
            && occurrence.number < 0
        {
            occurrence.number = 1;
        }
    }
    Some(Cow::Owned(forward))
}

/// How far from the end a measure counts its WHEN event (`CROSS=LAST` is 1,
/// `RISE=-2` is 2); `None` for one counted from the start or without one.
fn from_end_count(statement: &MeasureStatement) -> Option<usize> {
    let condition = match &statement.measure_type {
        MeasureType::When { condition, .. } => condition,
        MeasureType::Find {
            when: Some(condition),
            ..
        }
        | MeasureType::Derivative {
            when: Some(condition),
            ..
        } => condition,
        _ => return None,
    };
    let number = condition.occurrence.number;
    (number < 0).then(|| number.unsigned_abs())
}

/// The occurrence of a measure's WHEN event, for a WHEN, FIND or DERIV one.
fn counted_event(measure: &mut MeasureType) -> Option<&mut EventOccurrence> {
    match measure {
        MeasureType::When { condition, .. }
        | MeasureType::Find {
            when: Some(condition),
            ..
        }
        | MeasureType::Derivative {
            when: Some(condition),
            ..
        } => Some(&mut condition.occurrence),
        _ => None,
    }
}

/// The printed measures' traces: the plain ones from the same live
/// evaluation an `EQN` measure reads its operands through, so a row holds the
/// value the measure has after taking in that row's point and its default
/// before then; the continuous ones from their records.
#[allow(clippy::too_many_arguments)]
fn printed_traces(
    netlist: &Netlist,
    analysis: &str,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    implicit_default: Value,
    dc_sweep_ascending: Option<bool>,
    current_result: Option<&TransientResult>,
    continuous: &[ContinuousMeasureResult],
    printed: &HashSet<String>,
    abort: &dyn AbortSignal,
) -> Result<Vec<EquationMeasureTrace>, EquationMeasurementEvaluationError> {
    let evaluation = evaluate_equation_measurements_with_observations(
        netlist,
        analysis,
        axis,
        signals,
        implicit_default,
        dc_sweep_ascending,
        current_result,
        None,
        printed,
        abort,
    )?;
    let mut traces = evaluation
        .traces
        .into_iter()
        .filter(|trace| printed.contains(&trace.name.to_ascii_uppercase()))
        .collect::<Vec<_>>();
    let label = format!("{analysis}_CONT");
    for result in continuous {
        let statement = netlist.measurements.iter().find(|statement| {
            statement.analysis.eq_ignore_ascii_case(&label)
                && statement.name.eq_ignore_ascii_case(&result.name)
        });
        if let Some(statement) = statement
            && printed.contains(&statement.name.to_ascii_uppercase())
            && let Some(trace) = continuous_trace(netlist, statement, result, axis)
        {
            traces.push(trace);
        }
    }
    Ok(traces)
}

/// The value a continuous measure prints at each row: its latest record found
/// by that row, its default before the first.
///
/// A Xyce `_CONT` measure re-arms after every occurrence and its result is
/// the latest one: MEASURE_CONT/FindWhenTestTran.cir.prn steps
/// WHENCROSSCONTTEST2 through 6.25e-4, 4e-3, 6.25e-3 and 8.75e-3 on the row
/// after each crossing. A record is found at the first row whose point
/// reaches its axis (the target of a TRIG/TARG pair), in the direction the
/// sweep moves there. A record without an axis leaves the measure unprinted.
/// A measure counting from the end prints the record its count selects
/// among those found: WHENCROSSCONTNEG2 (`CROSS=-2`) trails one crossing
/// behind, printing 6.25e-4 once the 4e-3 crossing is found.
fn continuous_trace(
    netlist: &Netlist,
    statement: &MeasureStatement,
    result: &ContinuousMeasureResult,
    axis: &[Value],
) -> Option<EquationMeasureTrace> {
    let found = result
        .records
        .iter()
        .map(|record| Some((record.target_axis.or(record.event_axis)?, record.value)))
        .collect::<Option<Vec<_>>>()?;
    let default = netlist
        .options
        .measure_default_value
        .or(statement.default_value)
        .unwrap_or(0.0);
    let from_end = from_end_count(statement).unwrap_or(1);
    let mut reached = 0_usize;
    let mut records = found.iter().peekable();
    let mut previous = axis.first().copied().unwrap_or_default();
    let mut values = Vec::with_capacity(axis.len());
    for &point in axis {
        let tolerance = 1.0e-12 * point.abs().max(previous.abs());
        while let Some(&&(at, _)) = records.peek() {
            let found_by_now = if point >= previous {
                at <= point + tolerance
            } else {
                at >= point - tolerance
            };
            if !found_by_now {
                break;
            }
            reached += 1;
            records.next();
        }
        values.push(
            reached
                .checked_sub(from_end)
                .and_then(|index| found.get(index))
                .map_or(default, |&(_, value)| value),
        );
        previous = point;
    }
    Some(EquationMeasureTrace {
        name: statement.name.clone(),
        valid: values.iter().map(|value| !value.is_nan()).collect(),
        values,
        initialized: !found.is_empty(),
    })
}

/// Whether a Xyce `.PRINT` card of `analysis` names one of its measures.
pub(super) fn prints_measures(netlist: &Netlist, analysis: &str) -> bool {
    !printed_measure_names(netlist, analysis).is_empty()
}

/// Evaluate the measures a Xyce `.PRINT TRAN` or `.PRINT DC` card names over
/// the measure signal table of its result, failing at the first operand that
/// names one. `continuous` evaluates the analysis's `_CONT` measures; it runs
/// only when the card names one.
pub(super) fn printed_real_measure_traces(
    netlist: &Netlist,
    analysis: OutputAnalysisKind,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    current_result: Option<&TransientResult>,
    continuous: impl FnOnce(&Netlist) -> Result<Vec<ContinuousMeasureResult>, SimulationError>,
    abort: &dyn AbortSignal,
) -> Result<Vec<EquationMeasureTrace>, OutputProjectionError> {
    let (label, implicit_default, dc_sweep_ascending) = match analysis {
        OutputAnalysisKind::Dc => (
            "DC",
            0.0,
            Some(dc_primary_sweep_is_ascending(netlist, axis)),
        ),
        _ => ("TRAN", -1.0, None),
    };
    let printed = printed_measure_names(netlist, label);
    let Some(&(_, request, operand_index)) = printed.first() else {
        return Ok(Vec::new());
    };
    let failed = |detail| output_request_error(request, analysis, operand_index, None, detail);
    let names = printed
        .iter()
        .map(|(name, _, _)| name.clone())
        .collect::<HashSet<_>>();
    let continuous = match printed_continuous_netlist(netlist, label, &names) {
        Some(measured) => match continuous(&measured) {
            Ok(results) => results,
            Err(SimulationError::Aborted) => return Err(OutputProjectionError::Aborted),
            Err(error) => return Err(failed(error.to_string())),
        },
        None => Vec::new(),
    };
    printed_traces(
        netlist,
        label,
        axis,
        signals,
        implicit_default,
        dc_sweep_ascending,
        current_result,
        &continuous,
        &names,
        abort,
    )
    .map_err(|error| match error {
        EquationMeasurementEvaluationError::Aborted => OutputProjectionError::Aborted,
        EquationMeasurementEvaluationError::Detail(detail) => failed(detail),
    })
}

/// The value of every measure a Xyce `.PRINT AC` card names, at each
/// frequency of the sweep.
pub(super) fn printed_ac_measure_traces(
    netlist: &Netlist,
    sweep: &[AcResult],
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<EquationMeasureTrace>, SimulationError> {
    let printed = printed_measure_names(netlist, "AC")
        .into_iter()
        .map(|(name, _, _)| name)
        .collect::<HashSet<_>>();
    if printed.is_empty() {
        return Ok(Vec::new());
    }
    let Some(series) = AcSweepSeries::from_sweep_with_abort(sweep, PhaseUnit::of(netlist), abort)?
    else {
        return Ok(Vec::new());
    };
    let continuous = match printed_continuous_netlist(netlist, "AC", &printed) {
        Some(measured) => evaluate_ac_continuous_measurements_with_limits_and_abort(
            &measured, sweep, limits, abort,
        )?,
        None => Vec::new(),
    };
    let alias_projection = InterfaceNodeAliasProjection::new_with_abort(
        netlist,
        OutputAnalysisKind::Ac,
        series.axis().len(),
        abort,
    )
    .map_err(alias_error)?;
    let mut signals = series.equation_signal_map();
    alias_projection
        .augment_with_abort(&mut signals, abort)
        .map_err(alias_error)?;
    printed_traces(
        netlist,
        "AC",
        series.axis(),
        &signals,
        -1.0,
        None,
        None,
        &continuous,
        &printed,
        abort,
    )
    .map_err(evaluation_error)
}

/// The value of every measure a Xyce `.PRINT NOISE` card names, at each
/// frequency of the sweep.
pub(crate) fn printed_noise_measure_traces(
    netlist: &Netlist,
    sweep: &[crate::analysis::NoiseResult],
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<EquationMeasureTrace>, SimulationError> {
    let printed = printed_measure_names(netlist, "NOISE")
        .into_iter()
        .map(|(name, _, _)| name)
        .collect::<HashSet<_>>();
    if printed.is_empty() {
        return Ok(Vec::new());
    }
    let Some(series) =
        NoiseSweepSeries::from_sweep_with_abort(sweep, PhaseUnit::of(netlist), abort)?
    else {
        return Ok(Vec::new());
    };
    let continuous = match printed_continuous_netlist(netlist, "NOISE", &printed) {
        Some(measured) => evaluate_noise_continuous_measurements_with_limits_and_abort(
            &measured, sweep, limits, abort,
        )?,
        None => Vec::new(),
    };
    let alias_projection = InterfaceNodeAliasProjection::new_with_abort(
        netlist,
        OutputAnalysisKind::Noise,
        series.axis().len(),
        abort,
    )
    .map_err(alias_error)?;
    let mut signals = series.equation_signal_map();
    alias_projection
        .augment_with_abort(&mut signals, abort)
        .map_err(alias_error)?;
    printed_traces(
        netlist,
        "NOISE",
        series.axis(),
        &signals,
        -1.0,
        None,
        None,
        &continuous,
        &printed,
        abort,
    )
    .map_err(evaluation_error)
}

fn alias_error(error: InterfaceNodeAliasProjectionError) -> SimulationError {
    match error {
        InterfaceNodeAliasProjectionError::Aborted => SimulationError::Aborted,
        InterfaceNodeAliasProjectionError::Detail(detail) => SimulationError::Netlist(detail),
    }
}

fn evaluation_error(error: EquationMeasurementEvaluationError) -> SimulationError {
    match error {
        EquationMeasurementEvaluationError::Aborted => SimulationError::Aborted,
        EquationMeasurementEvaluationError::Detail(detail) => {
            SimulationError::Netlist(format!("a printed measure cannot be evaluated: {detail}"))
        }
    }
}

/// Add each printed measure's trace to an output signal table under its
/// name, unless a signal already answers that name.
pub(crate) fn insert_printed_measure_traces<'a>(
    signals: &mut HashMap<String, &'a [Value]>,
    traces: &'a [EquationMeasureTrace],
) {
    if traces.is_empty() {
        return;
    }
    let taken = signals
        .keys()
        .map(|name| canonical_measure_signal_name(name))
        .collect::<HashSet<_>>();
    for trace in traces {
        if !taken.contains(&canonical_measure_signal_name(&trace.name)) {
            insert_case_variants(signals, &trace.name, &trace.values);
        }
    }
}
