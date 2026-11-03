//! Measures a Xyce `.PRINT` card names, printed as they stand at each point.
use super::*;

/// The measures of one analysis that a Xyce `.PRINT` card of that analysis
/// names, bare (`maxvr4`) or inside an expression (`{vv1-vv2}`), by
/// canonical name, each with the first request and operand that names it, in
/// card order.
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
    let measures = netlist
        .measurements
        .iter()
        .filter(|statement| statement.analysis.eq_ignore_ascii_case(analysis))
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

/// The printed measures' traces, from the same live evaluation an `EQN`
/// measure reads its operands through: a row holds the value the measure has
/// after taking in that row's point, and its default before then.
#[allow(clippy::too_many_arguments)]
fn printed_traces(
    netlist: &Netlist,
    analysis: &str,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    implicit_default: Value,
    dc_sweep_ascending: Option<bool>,
    current_result: Option<&TransientResult>,
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
    Ok(evaluation
        .traces
        .into_iter()
        .filter(|trace| printed.contains(&trace.name.to_ascii_uppercase()))
        .collect())
}

/// Whether a Xyce `.PRINT` card of `analysis` names one of its measures.
pub(super) fn prints_measures(netlist: &Netlist, analysis: &str) -> bool {
    !printed_measure_names(netlist, analysis).is_empty()
}

/// Evaluate the measures a Xyce `.PRINT TRAN` or `.PRINT DC` card names over
/// the measure signal table of its result, failing at the first operand that
/// names one.
pub(super) fn printed_real_measure_traces(
    netlist: &Netlist,
    analysis: OutputAnalysisKind,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    current_result: Option<&TransientResult>,
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
    let names = printed
        .iter()
        .map(|(name, _, _)| name.clone())
        .collect::<HashSet<_>>();
    printed_traces(
        netlist,
        label,
        axis,
        signals,
        implicit_default,
        dc_sweep_ascending,
        current_result,
        &names,
        abort,
    )
    .map_err(|error| match error {
        EquationMeasurementEvaluationError::Aborted => OutputProjectionError::Aborted,
        EquationMeasurementEvaluationError::Detail(detail) => {
            output_request_error(request, analysis, operand_index, None, detail)
        }
    })
}

/// The value of every measure a Xyce `.PRINT AC` card names, at each
/// frequency of the sweep.
pub(super) fn printed_ac_measure_traces(
    netlist: &Netlist,
    sweep: &[AcResult],
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
