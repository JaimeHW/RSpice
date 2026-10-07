//! Continuous measurement adapters share typed execution limits and cancellation.
use super::*;
use crate::analysis::measure::continuous::poll;

fn statements<'a>(
    netlist: &'a Netlist,
    analysis: &str,
    points: usize,
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<&'a MeasureStatement>, SimulationError> {
    poll(abort, 0)?;
    let mut selected = Vec::new();
    for (index, statement) in netlist.measurements.iter().enumerate() {
        poll(abort, index)?;
        if statement.analysis.eq_ignore_ascii_case(analysis) {
            ResourceLimitError::ensure(
                ResourceKind::ResultValues,
                selected.len().saturating_add(1),
                limits.max_result_values,
            )?;
            selected.push(statement);
        }
    }
    if !selected.is_empty() {
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            points,
            limits.max_analysis_points,
        )?;
    }
    Ok(selected)
}

fn failed(
    statements: &[&MeasureStatement],
    reason: &str,
    abort: &dyn AbortSignal,
) -> Result<Vec<ContinuousMeasureResult>, SimulationError> {
    let mut results = Vec::new();
    for (index, statement) in statements.iter().enumerate() {
        poll(abort, index)?;
        results.push(ContinuousMeasureResult {
            name: statement.name.clone(),
            records: Vec::new(),
            failure: Some(reason.to_owned()),
            failure_metadata: None,
        });
    }
    Ok(results)
}

fn legacy_result(
    netlist: &Netlist,
    analysis: &str,
    result: Result<Vec<ContinuousMeasureResult>, SimulationError>,
) -> Vec<ContinuousMeasureResult> {
    result.unwrap_or_else(|error| {
        failed_continuous_measurements(
            &measurements_for_analysis(netlist, analysis),
            &error.to_string(),
        )
    })
}

fn ensure_series(
    points: usize,
    columns: usize,
    limits: &ResourceLimits,
) -> Result<(), SimulationError> {
    ResourceLimitError::ensure(
        ResourceKind::ResultValues,
        points.saturating_mul(columns),
        limits.max_result_values,
    )?;
    Ok(())
}

fn evaluate(
    statements: &[&MeasureStatement],
    data: MeasureData<'_, '_>,
    params: &crate::netlist::ParamContext,
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<ContinuousMeasureResult>, SimulationError> {
    let derived = materialize_measure_expression_signals_with_limits_and_abort(
        statements,
        data.axis,
        data.signals,
        params,
        limits,
        abort,
    )?;
    let mut signals = HashMap::new();
    for (index, (name, waveform)) in data.signals.iter().enumerate() {
        poll(abort, index)?;
        signals.insert(name.clone(), *waveform);
    }
    for (index, (name, waveform)) in derived.iter().enumerate() {
        poll(abort, index)?;
        signals.insert(name.clone(), waveform.as_slice());
    }
    let mut engine = MeasureEngine::new();
    for (index, statement) in statements.iter().enumerate() {
        poll(abort, index)?;
        engine.add((*statement).clone());
    }
    engine.evaluate_continuous_with_limits_and_abort(
        data.axis,
        &signals,
        data.segment_starts,
        limits,
        abort,
    )
}

fn with_aliases(
    netlist: &Netlist,
    analysis: OutputAnalysisKind,
    statements: &[&MeasureStatement],
    data: MeasureData<'_, '_>,
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<ContinuousMeasureResult>, SimulationError> {
    if statements.is_empty() {
        return Ok(Vec::new());
    }
    let projection = match InterfaceNodeAliasProjection::new_with_abort(
        netlist,
        analysis,
        data.axis.len(),
        abort,
    ) {
        Ok(projection) => projection,
        Err(InterfaceNodeAliasProjectionError::Aborted) => return Err(SimulationError::Aborted),
        Err(InterfaceNodeAliasProjectionError::Detail(error)) => {
            return failed(statements, &error, abort);
        }
    };
    let mut signals = HashMap::new();
    for (index, (name, waveform)) in data.signals.iter().enumerate() {
        poll(abort, index)?;
        signals.insert(name.clone(), *waveform);
    }
    match projection.augment_with_abort(&mut signals, abort) {
        Ok(()) => {}
        Err(InterfaceNodeAliasProjectionError::Aborted) => return Err(SimulationError::Aborted),
        Err(InterfaceNodeAliasProjectionError::Detail(error)) => {
            return failed(statements, &error, abort);
        }
    }
    evaluate(
        statements,
        MeasureData {
            signals: &signals,
            ..data
        },
        &netlist.params,
        limits,
        abort,
    )
}

/// Evaluate `.MEASURE TRAN_CONT` using the default resource policy.
pub fn evaluate_tran_continuous_measurements(
    netlist: &Netlist,
    result: &TransientResult,
) -> Vec<ContinuousMeasureResult> {
    legacy_result(
        netlist,
        "TRAN_CONT",
        evaluate_tran_continuous_measurements_with_limits_and_abort(
            netlist,
            result,
            &ResourceLimits::default(),
            &NoAbort,
        ),
    )
}

/// Evaluate accepted transient point events with bounded preparation and output.
pub fn evaluate_tran_continuous_measurements_with_limits_and_abort(
    netlist: &Netlist,
    result: &TransientResult,
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<ContinuousMeasureResult>, SimulationError> {
    let statements = statements(netlist, "TRAN_CONT", result.time.len(), limits, abort)?;
    if statements.is_empty() {
        return Ok(Vec::new());
    }
    if result.time.is_empty() {
        return failed(
            &statements,
            "transient analysis produced no accepted points",
            abort,
        );
    }
    let signals = transient_signal_map(result);
    let mut failures = Vec::new();
    let mut supported = Vec::new();
    for statement in &statements {
        poll(abort, 0)?;
        let failure = if result.current_impulses.is_some() {
            match compile_live_measure_state(
                statement,
                "TRAN",
                &result.time,
                None,
                netlist.options.measure_use_lttm(),
                &netlist.params,
            ) {
                Ok(state) => {
                    match current_measure::compile(netlist, result, statement, &state, abort) {
                        Ok(_) => None,
                        Err(current_observation::CurrentObservationError::Aborted) => {
                            return Err(SimulationError::Aborted);
                        }
                        Err(error) => Some(error.to_string()),
                    }
                }
                Err(error) => Some(error),
            }
        } else {
            None
        };
        if failure.is_none() {
            supported.push(*statement);
        }
        failures.push(failure);
    }
    let mut output_limits = *limits;
    output_limits.max_result_values = limits
        .max_result_values
        .saturating_sub(statements.len() - supported.len());
    let mut evaluated = with_aliases(
        netlist,
        OutputAnalysisKind::Tran,
        &supported,
        MeasureData {
            axis: &result.time,
            signals: &signals,
            segment_starts: &[],
        },
        &output_limits,
        abort,
    )?
    .into_iter();
    let mut results = Vec::new();
    for (index, (statement, failure)) in statements.iter().zip(failures).enumerate() {
        poll(abort, index)?;
        if let Some(failure) = failure {
            results.push(ContinuousMeasureResult {
                name: statement.name.clone(),
                records: Vec::new(),
                failure: Some(failure),
                failure_metadata: None,
            });
        } else if let Some(result) = evaluated.next() {
            results.push(result);
        }
    }
    Ok(results)
}

/// Evaluate `.MEASURE DC_CONT` using the default resource policy.
pub fn evaluate_dc_continuous_measurements(
    netlist: &Netlist,
    sweep: &[(Value, SimulationResult)],
) -> Vec<ContinuousMeasureResult> {
    evaluate_dc_continuous_measurements_with_parameter_contexts(netlist, sweep, &[])
}

/// Evaluate DC continuous statements with point-local parameter contexts.
pub fn evaluate_dc_continuous_measurements_with_parameter_contexts(
    netlist: &Netlist,
    sweep: &[(Value, SimulationResult)],
    point_params: &[crate::netlist::ParamContext],
) -> Vec<ContinuousMeasureResult> {
    legacy_result(
        netlist,
        "DC_CONT",
        evaluate_dc_continuous_measurements_with_parameter_contexts_limits_and_abort(
            netlist,
            sweep,
            point_params,
            &ResourceLimits::default(),
            &NoAbort,
        ),
    )
}

/// Evaluate DC point events with resource limits and cooperative cancellation.
pub fn evaluate_dc_continuous_measurements_with_limits_and_abort(
    netlist: &Netlist,
    sweep: &[(Value, SimulationResult)],
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<ContinuousMeasureResult>, SimulationError> {
    evaluate_dc_continuous_measurements_with_parameter_contexts_limits_and_abort(
        netlist,
        sweep,
        &[],
        limits,
        abort,
    )
}

/// Evaluate DC point events while retaining `.DC DATA` parameter semantics.
pub fn evaluate_dc_continuous_measurements_with_parameter_contexts_limits_and_abort(
    netlist: &Netlist,
    sweep: &[(Value, SimulationResult)],
    point_params: &[crate::netlist::ParamContext],
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<ContinuousMeasureResult>, SimulationError> {
    let selected = statements(netlist, "DC_CONT", sweep.len(), limits, abort)?;
    if selected.is_empty() {
        return Ok(Vec::new());
    }
    let mut normalized = Vec::new();
    for (index, statement) in selected.iter().enumerate() {
        poll(abort, index)?;
        normalized.push(normalize_dc_measurement_window((*statement).clone()));
    }
    let statements = normalized.iter().collect::<Vec<_>>();
    if let Some((_, first)) = sweep.first() {
        let mut names = HashSet::new();
        let base = first
            .node_voltages
            .len()
            .saturating_add(first.branch_names.len());
        ensure_series(sweep.len(), base, limits)?;
        for (row, (_, point)) in sweep.iter().enumerate() {
            poll(abort, row)?;
            for (index, (name, _)) in point.dc_observables.iter().enumerate() {
                poll(abort, index)?;
                if names.insert(name.to_ascii_uppercase()) {
                    ensure_series(sweep.len(), base.saturating_add(names.len()), limits)?;
                }
            }
        }
    }
    let series = match DcSweepSeries::from_sweep_with_abort(sweep, abort) {
        Ok(Some(series)) => series,
        Ok(None) => return failed(&statements, "DC sweep produced no points", abort),
        Err(SimulationError::Aborted) => return Err(SimulationError::Aborted),
        Err(error) => return failed(&statements, &error.to_string(), abort),
    };
    let mut signals = series.signal_map();
    if !point_params.is_empty() && point_params.len() != series.axis().len() {
        return failed(
            &statements,
            "DC point-parameter context count does not match sweep length",
            abort,
        );
    }
    let parameter_series = parameter_series(point_params, limits, abort)?;
    for (name, values) in &parameter_series {
        insert_case_variants(&mut signals, name, values);
    }
    let segments = dc_primary_segment_starts(netlist, series.axis().len());
    with_aliases(
        netlist,
        OutputAnalysisKind::Dc,
        &statements,
        MeasureData {
            axis: series.axis(),
            signals: &signals,
            segment_starts: &segments,
        },
        limits,
        abort,
    )
}

fn parameter_series(
    contexts: &[crate::netlist::ParamContext],
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<(String, Vec<Value>)>, SimulationError> {
    let mut names = std::collections::BTreeSet::new();
    for (row, context) in contexts.iter().enumerate() {
        poll(abort, row)?;
        for (index, (name, _)) in context.numeric_parameters().into_iter().enumerate() {
            poll(abort, index)?;
            names.insert(name);
        }
    }
    let mut series = Vec::new();
    for name in names {
        poll(abort, 0)?;
        ensure_series(contexts.len(), series.len().saturating_add(1), limits)?;
        let mut values = Vec::new();
        for (row, context) in contexts.iter().enumerate() {
            poll(abort, row)?;
            let Some(value) = context.get(&name) else {
                break;
            };
            values.push(value);
        }
        if values.len() == contexts.len() {
            series.push((name, values));
        }
    }
    Ok(series)
}

/// Evaluate `.MEASURE AC_CONT` using the standard complex probe projections.
pub fn evaluate_ac_continuous_measurements(
    netlist: &Netlist,
    sweep: &[AcResult],
) -> Vec<ContinuousMeasureResult> {
    legacy_result(
        netlist,
        "AC_CONT",
        evaluate_ac_continuous_measurements_with_limits_and_abort(
            netlist,
            sweep,
            &ResourceLimits::default(),
            &NoAbort,
        ),
    )
}

/// Evaluate continuous AC events with bounded projection and output storage.
pub fn evaluate_ac_continuous_measurements_with_limits_and_abort(
    netlist: &Netlist,
    sweep: &[AcResult],
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<ContinuousMeasureResult>, SimulationError> {
    let statements = statements(netlist, "AC_CONT", sweep.len(), limits, abort)?;
    if statements.is_empty() {
        return Ok(Vec::new());
    }
    if let Some(first) = sweep.first() {
        ensure_series(
            sweep.len(),
            1usize.saturating_add(
                6usize.saturating_mul(
                    first
                        .node_names
                        .len()
                        .saturating_add(first.branch_names.len()),
                ),
            ),
            limits,
        )?;
    }
    let series = match AcSweepSeries::from_sweep_with_abort(sweep, abort) {
        Ok(Some(series)) => series,
        Ok(None) => return failed(&statements, "AC sweep produced no points", abort),
        Err(SimulationError::Aborted) => return Err(SimulationError::Aborted),
        Err(error) => return failed(&statements, &error.to_string(), abort),
    };
    with_aliases(
        netlist,
        OutputAnalysisKind::Ac,
        &statements,
        MeasureData {
            axis: series.axis(),
            signals: &series.equation_signal_map(),
            segment_starts: &[],
        },
        limits,
        abort,
    )
}

/// Evaluate `.MEASURE NOISE_CONT` while retaining every qualifying event.
pub fn evaluate_noise_continuous_measurements(
    netlist: &Netlist,
    sweep: &[crate::analysis::NoiseResult],
) -> Vec<ContinuousMeasureResult> {
    legacy_result(
        netlist,
        "NOISE_CONT",
        evaluate_noise_continuous_measurements_with_limits_and_abort(
            netlist,
            sweep,
            &ResourceLimits::default(),
            &NoAbort,
        ),
    )
}

/// Evaluate continuous noise events with bounded projection and output storage.
pub fn evaluate_noise_continuous_measurements_with_limits_and_abort(
    netlist: &Netlist,
    sweep: &[crate::analysis::NoiseResult],
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<ContinuousMeasureResult>, SimulationError> {
    let statements = statements(netlist, "NOISE_CONT", sweep.len(), limits, abort)?;
    if statements.is_empty() {
        return Ok(Vec::new());
    }
    if let Some(first) = sweep.first() {
        let mut probes = HashSet::new();
        let base = 3usize.saturating_add(
            6usize.saturating_mul(
                first
                    .node_names
                    .len()
                    .saturating_add(first.branch_names.len()),
            ),
        );
        ensure_series(sweep.len(), base, limits)?;
        for (index, identity) in first.contribution_catalog.iter().enumerate() {
            poll(abort, index)?;
            probes.insert((identity.device.to_ascii_uppercase(), None));
            if let Some(mechanism) = &identity.mechanism {
                probes.insert((
                    identity.device.to_ascii_uppercase(),
                    Some(mechanism.to_ascii_uppercase()),
                ));
            }
            ensure_series(
                sweep.len(),
                base.saturating_add(probes.len().saturating_mul(2)),
                limits,
            )?;
        }
    }
    let series = match NoiseSweepSeries::from_sweep_with_abort(sweep, abort) {
        Ok(Some(series)) => series,
        Ok(None) => return failed(&statements, "noise sweep produced no points", abort),
        Err(SimulationError::Aborted) => return Err(SimulationError::Aborted),
        Err(error) => return failed(&statements, &error.to_string(), abort),
    };
    with_aliases(
        netlist,
        OutputAnalysisKind::Noise,
        &statements,
        MeasureData {
            axis: series.axis(),
            signals: &series.equation_signal_map(),
            segment_starts: &[],
        },
        limits,
        abort,
    )
}
