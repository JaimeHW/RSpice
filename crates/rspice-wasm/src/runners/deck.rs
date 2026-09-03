//! Authored-deck execution over the canonical `DeckPlan` coordinate product.

use rspice_core::{AbortSignal, Engine, Netlist, NoAbort, ResourceKind, ResourceLimits};

use crate::DetailedWasmResult;
use crate::abort::{aborted_error, ensure_not_aborted};
use crate::deck_result_document::{self, DeckResultDocument};
use crate::dto::{
    AcPointSnapshot, TransientFftSnapshot, complex_series_from_slice,
    transient_snapshot_from_result,
};
use crate::errors::WasmError;
use crate::errors::resource_limit_error;
use crate::options::WasmExecutionOptions;
use crate::result_document::{self, AnalogResultDocument};
use crate::runners::direct::validate_transient_request;
use crate::support::{engine_with_resource_limits, parse_netlist_detailed};

/// Execute every supported physical analysis in an authored analog deck over
/// its canonical STEP/TEMP coordinate product. No request selection or
/// implicit scalar fallback is applied: an unsupported card rejects the whole
/// request before a result handle is published.
pub fn run_authored_deck_document_with_options_and_abort_detailed(
    source: &str,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<DeckResultDocument> {
    use rspice_core::execution::{AnalysisKind, DeckPlan};
    ensure_not_aborted(external_abort)?;
    let resource_limits = options.resource_limits.to_core();
    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    preflight_authored_deck(&netlist)?;
    let plan = DeckPlan::from_netlist_with_abort(&netlist, &resource_limits, external_abort)
        .map_err(deck_plan_wasm_error)?;
    refuse_unsupported_deck_analyses(&netlist, &plan)?;
    for axis in plan.axes() {
        match axis.kind() {
            rspice_core::execution::AxisKind::Data
            | rspice_core::execution::AxisKind::Step
            | rspice_core::execution::AxisKind::Temperature => {}
            rspice_core::execution::AxisKind::Alter => {
                return Err(unsupported_deck_axis(
                    "ALTER axes must be expanded before browser deck execution".to_owned(),
                ));
            }
            _ => {
                return Err(unsupported_deck_axis(
                    "unknown canonical run-axis kind is not supported".to_owned(),
                ));
            }
        }
    }
    for planned in plan.analyses() {
        if !matches!(
            planned.id().kind(),
            AnalysisKind::ImplicitOp
                | AnalysisKind::Op
                | AnalysisKind::Dc
                | AnalysisKind::Ac
                | AnalysisKind::Tran
                | AnalysisKind::Noise
        ) {
            return Err(unsupported_deck_analysis(format!(
                "canonical analysis {} is not mapped by the browser deck API",
                planned.id()
            )));
        }
    }

    let planning_engine = engine_with_resource_limits(resource_limits)?;
    let materializer = planning_engine
        .prepare_deck_plan_materializer_with_abort(&netlist, &plan, external_abort)
        .map_err(materialized_run_wasm_error)?;
    let mut document = DeckResultDocument::new(&plan).map_err(deck_document_error)?;
    document
        .coordinates
        .try_reserve_exact(materializer.len())
        .map_err(|_| deck_allocation_error("deck coordinates"))?;
    let result_capacity = materializer
        .len()
        .checked_mul(plan.analyses().len())
        .ok_or_else(|| deck_allocation_error("coordinate-local analysis results"))?;
    document
        .results
        .try_reserve_exact(result_capacity)
        .map_err(|_| deck_allocation_error("coordinate-local analysis results"))?;
    let mut retained_values = 0usize;

    for run_index in 0..materializer.len() {
        ensure_not_aborted(external_abort)?;
        let materialized = materializer
            .materialize_run_with_abort(run_index, external_abort)
            .map_err(materialized_run_wasm_error)?;
        let (coordinate, coordinate_netlist, _topology, analyses) = materialized.into_parts();
        let coordinate_index = document
            .push_coordinate(&coordinate)
            .map_err(deck_document_error)?;
        if analyses.len() != plan.analyses().len() {
            return Err(deck_document_error(format!(
                "coordinate {coordinate_index} materialized {} analyses; expected {}",
                analyses.len(),
                plan.analyses().len()
            )));
        }
        let coordinate_config = rspice_core::resolve_simulation_config(
            planning_engine.config(),
            Some(&coordinate_netlist.options),
            &rspice_core::SimulationConfigOverrides::default(),
        );
        let coordinate_engine = Engine::try_new(coordinate_config).map_err(|error| {
            Box::new(WasmError::from_simulation_error(
                rspice_core::engine::SimulationError::Configuration(error),
            ))
        })?;

        for analysis in analyses {
            ensure_not_aborted(external_abort)?;
            let analysis_id = analysis.id();
            let ordinal = analysis_id.ordinal() as usize + 1;
            let (mut analog, fft_results) = match analysis.command() {
                None if analysis_id.kind() == AnalysisKind::ImplicitOp => {
                    execute_authored_operating_point(
                        &coordinate_engine,
                        &coordinate_netlist,
                        ordinal,
                        external_abort,
                    )?
                }
                None => {
                    return Err(deck_document_error(format!(
                        "canonical materializer omitted the authored command for {analysis_id}"
                    )));
                }
                Some(command) => execute_authored_analysis(
                    &coordinate_engine,
                    &coordinate_netlist,
                    command,
                    ordinal,
                    resource_limits,
                    external_abort,
                )?,
            };
            deck_result_document::set_execution_identity(&mut analog, &coordinate, analysis_id);
            let analog_values = analog.retained_numeric_value_count();
            let fft_values = fft_results.iter().try_fold(0usize, |total, fft| {
                total
                    .checked_add(
                        deck_result_document::fft_retained_numeric_value_count(fft)
                            .map_err(deck_document_error)?,
                    )
                    .ok_or_else(|| {
                        deck_document_error("deck retained-value count overflowed usize".to_owned())
                    })
            })?;
            let next_retained_values = retained_values
                .checked_add(analog_values)
                .and_then(|value| value.checked_add(fft_values))
                .ok_or_else(|| {
                    deck_document_error("deck retained-value count overflowed usize".to_owned())
                })?;
            if next_retained_values > resource_limits.max_result_values {
                return Err(resource_limit_error(
                    ResourceKind::ResultValues,
                    next_retained_values,
                    resource_limits.max_result_values,
                ));
            }
            retained_values = next_retained_values;
            let output_namespace = analysis.output_namespace().components().join("/");
            let checkpoint_namespace = analysis.checkpoint_namespace().components().join("/");
            let parent_result_index = document.results.len();
            document
                .results
                .push(deck_result_document::DeckAnalogResult {
                    coordinate_index,
                    analysis_instance_id: analysis_id.tag(),
                    output_namespace,
                    checkpoint_namespace,
                    document: analog,
                });
            if !fft_results.is_empty() {
                document
                    .fft_results
                    .try_reserve(fft_results.len())
                    .map_err(|_| deck_allocation_error("attached FFT results"))?;
            }
            for mut fft in fft_results {
                fft.parent_analysis_id = analysis_id.tag();
                let output_namespace = format!(
                    "{}/{}/{}",
                    coordinate.stable_tag(),
                    analysis_id,
                    fft.analysis_id
                );
                document
                    .fft_results
                    .push(deck_result_document::DeckFftResult {
                        coordinate_index,
                        parent_result_index,
                        output_namespace,
                        snapshot: fft,
                    });
            }
        }
    }
    ensure_not_aborted(external_abort)?;
    document
        .validate_with_abort(external_abort)
        .map_err(|message| {
            if external_abort.is_aborted() {
                aborted_error()
            } else {
                deck_document_error(message)
            }
        })?;
    Ok(document)
}

pub fn run_authored_deck_document_detailed(source: &str) -> DetailedWasmResult<DeckResultDocument> {
    run_authored_deck_document_with_options_and_abort_detailed(
        source,
        &WasmExecutionOptions::default(),
        &NoAbort,
    )
}

pub(crate) fn preflight_authored_deck(netlist: &Netlist) -> DetailedWasmResult<()> {
    use rspice_core::netlist::AnalysisCommand;

    if netlist.source_text.as_deref().is_some_and(|source| {
        source.lines().any(|line| {
            line.split_whitespace()
                .next()
                .is_some_and(|token| token.eq_ignore_ascii_case(".alter"))
        })
    }) {
        return Err(unsupported_deck_axis(
            "ALTER variants must be expanded before browser deck execution".to_owned(),
        ));
    }
    for command in &netlist.analyses {
        match command {
            AnalysisCommand::Op
            | AnalysisCommand::Dc { sweep2: None, .. }
            | AnalysisCommand::Ac { .. }
            | AnalysisCommand::Tran { start: None, .. }
            | AnalysisCommand::Tran {
                start: Some(0.0), ..
            }
            | AnalysisCommand::Noise { .. }
            | AnalysisCommand::Step(_)
            | AnalysisCommand::Temp { .. } => {}
            AnalysisCommand::Dc {
                sweep2: Some(_), ..
            } => {
                return Err(unsupported_deck_analysis(
                    "nested two-source DC sweeps are not represented by analog result schema v1"
                        .to_owned(),
                ));
            }
            AnalysisCommand::Tran { start: Some(_), .. } => {
                return Err(unsupported_deck_analysis(
                    "nonzero transient output start times are not represented by this browser deck executor"
                        .to_owned(),
                ));
            }
            AnalysisCommand::Four { .. } => {
                return Err(unsupported_deck_analysis(
                    "authored FOUR post-processing is not represented by the browser deck API"
                        .to_owned(),
                ));
            }
            AnalysisCommand::Pss(_)
            | AnalysisCommand::Pac(_)
            | AnalysisCommand::Pnoise(_)
            | AnalysisCommand::Envelope(_) => {
                // Refused by `refuse_unsupported_deck_analyses` once the
                // canonical plan has assigned each card its instance identity,
                // so the refusal can name it.
            }
            other => {
                return Err(unsupported_deck_analysis(format!(
                    "authored analysis {other:?} is not mapped by the browser deck API"
                )));
            }
        }
    }
    Ok(())
}

/// Dot-command spelling of a card the browser deck API cannot execute.
pub(crate) fn unsupported_deck_analysis_card(
    analysis: &rspice_core::netlist::AnalysisCommand,
) -> Option<&'static str> {
    use rspice_core::netlist::AnalysisCommand;

    match analysis {
        AnalysisCommand::Pss(_) => Some(".PSS"),
        AnalysisCommand::Pac(_) => Some(".PAC"),
        AnalysisCommand::Pnoise(_) => Some(".PNOISE"),
        AnalysisCommand::Envelope(_) => Some(".ENVELOPE"),
        _ => None,
    }
}

/// Refuse the periodic large-signal family with its canonical analysis
/// identity, before any solve runs or any result handle is published.
pub(crate) fn refuse_unsupported_deck_analyses(
    netlist: &Netlist,
    plan: &rspice_core::execution::DeckPlan,
) -> DetailedWasmResult<()> {
    for (analysis, id) in plan.authored_analyses(netlist) {
        if let Some(card) = unsupported_deck_analysis_card(analysis) {
            let instance = id.map(|id| format!(" ({id})")).unwrap_or_default();
            return Err(unsupported_deck_analysis(format!(
                "authored {card} card{instance} is not mapped by the browser deck API"
            )));
        }
    }
    Ok(())
}

pub(crate) fn execute_authored_operating_point(
    engine: &Engine,
    netlist: &Netlist,
    ordinal: usize,
    abort: &dyn AbortSignal,
) -> DetailedWasmResult<(AnalogResultDocument, Vec<TransientFftSnapshot>)> {
    let (result, report) = engine
        .run_dc_op_with_report_and_abort(netlist, abort)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
    ensure_not_aborted(abort)?;
    let document = result_document::operating_point_document(result, report, ordinal)
        .map_err(deck_document_error)?;
    Ok((document, Vec::new()))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn execute_authored_analysis(
    engine: &Engine,
    netlist: &Netlist,
    command: &rspice_core::netlist::AnalysisCommand,
    ordinal: usize,
    resource_limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> DetailedWasmResult<(AnalogResultDocument, Vec<TransientFftSnapshot>)> {
    use rspice_core::netlist::{AnalysisCommand, DcSweepSpec};

    match command {
        AnalysisCommand::Op => execute_authored_operating_point(engine, netlist, ordinal, abort),
        AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            mode,
            sweep2: None,
        } => {
            let spec = DcSweepSpec {
                start: *start,
                stop: *stop,
                step: *step,
                mode: mode.clone(),
            };
            let points = engine
                .run_dc_sweep2_spec_with_report_and_abort(netlist, source, &spec, None, abort)
                .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
            ensure_not_aborted(abort)?;
            let document = result_document::dc_sweep_document(source, points, ordinal)
                .map_err(deck_document_error)?;
            Ok((document, Vec::new()))
        }
        AnalysisCommand::Ac {
            variation,
            points,
            start_freq,
            stop_freq,
        } => {
            let frequencies = authored_frequency_grid(
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                false,
                resource_limits,
                abort,
            )?;
            let results = engine
                .run_ac_with_abort(netlist, &frequencies, abort)
                .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
            let mut snapshots = Vec::new();
            snapshots
                .try_reserve_exact(results.len())
                .map_err(|_| deck_allocation_error("coordinate-local AC snapshots"))?;
            for (index, point) in results.into_iter().enumerate() {
                if index.is_multiple_of(64) {
                    ensure_not_aborted(abort)?;
                }
                snapshots.push(AcPointSnapshot {
                    frequency: point.frequency,
                    node_names: point.node_names,
                    branch_names: point.branch_names,
                    voltages: complex_series_from_slice(&point.voltages),
                    currents: complex_series_from_slice(&point.currents),
                });
            }
            ensure_not_aborted(abort)?;
            let document =
                result_document::ac_document(snapshots, ordinal).map_err(deck_document_error)?;
            Ok((document, Vec::new()))
        }
        AnalysisCommand::Tran {
            step,
            stop,
            start,
            max_step,
            uic,
        } if start.is_none_or(|start| start == 0.0) => {
            let ceiling = resolved_authored_tran_step(*step, *stop, *start, *max_step)?;
            validate_transient_request(*stop, ceiling, resource_limits)?;
            let result = engine
                .run_tran_with_startup_mode_and_abort(
                    netlist,
                    *stop,
                    ceiling,
                    rspice_core::engine::TransientStartupMode::from_uic(*uic),
                    abort,
                )
                .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
            let mut snapshot = transient_snapshot_from_result(result).map_err(|message| {
                Box::new(WasmError::new(
                    message,
                    "invalid_transient_result",
                    "result_validation",
                ))
            })?;
            ensure_not_aborted(abort)?;
            let fft_results = std::mem::take(&mut snapshot.fft_results);
            let document = result_document::transient_document(snapshot, ordinal)
                .map_err(deck_document_error)?;
            Ok((document, fft_results))
        }
        AnalysisCommand::Noise {
            output_node,
            reference_node,
            input_source,
            variation,
            points,
            start_freq,
            stop_freq,
        } => {
            let frequencies = authored_frequency_grid(
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                true,
                resource_limits,
                abort,
            )?;
            let points = engine
                .run_noise_named_with_input_source_and_abort(
                    netlist,
                    output_node,
                    reference_node.as_deref(),
                    input_source,
                    &frequencies,
                    engine.config().temperature,
                    abort,
                )
                .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
            ensure_not_aborted(abort)?;
            let document =
                result_document::noise_document(points, ordinal).map_err(deck_document_error)?;
            Ok((document, Vec::new()))
        }
        _ => Err(unsupported_deck_analysis(format!(
            "materialized analysis {command:?} is not mapped by the browser deck API"
        ))),
    }
}

pub(crate) fn authored_frequency_grid(
    variation: rspice_core::netlist::FreqVariation,
    points: usize,
    start: f64,
    stop: f64,
    strictly_positive: bool,
    resource_limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> DetailedWasmResult<Vec<f64>> {
    ensure_not_aborted(abort)?;
    if strictly_positive && (!start.is_finite() || start <= 0.0) {
        return Err(Box::new(WasmError::invalid_argument(format!(
            "authored noise start frequency must be positive and finite, got {start}"
        ))));
    }
    rspice_core::analysis::ac::try_ac_sweep_frequencies_bounded_with_abort(
        variation,
        points,
        start,
        stop,
        resource_limits.max_analysis_points,
        abort,
    )
    .map_err(|error| match error {
        rspice_core::analysis::FrequencyGridError::Aborted => aborted_error(),
        rspice_core::analysis::FrequencyGridError::LimitExceeded { requested, limit } => {
            resource_limit_error(ResourceKind::AnalysisPoints, requested, limit)
        }
        rspice_core::analysis::FrequencyGridError::Allocation { requested } => {
            Box::new(WasmError::new(
                format!("could not allocate the {requested}-point authored frequency grid"),
                "result_allocation_failed",
                "analysis_setup",
            ))
        }
        other => Box::new(WasmError::invalid_argument(format!(
            "invalid authored frequency sweep {variation:?} {points} from {start} to {stop} Hz: {other}"
        ))),
    })
}

pub(crate) fn resolved_authored_tran_step(
    step: f64,
    stop: f64,
    start: Option<f64>,
    explicit: Option<f64>,
) -> DetailedWasmResult<f64> {
    rspice_core::execution::resolve_transient_maximum_step(step, stop, start, explicit)
        .map_err(|error| Box::new(WasmError::invalid_argument(error.to_string())))
}

pub(crate) fn unsupported_deck_analysis(message: String) -> Box<WasmError> {
    Box::new(WasmError::new(
        message,
        "unsupported_deck_analysis",
        "unsupported_feature",
    ))
}

pub(crate) fn unsupported_deck_axis(message: String) -> Box<WasmError> {
    Box::new(WasmError::new(
        message,
        "unsupported_deck_axis",
        "unsupported_feature",
    ))
}

pub(crate) fn deck_document_error(message: String) -> Box<WasmError> {
    Box::new(WasmError::new(
        message,
        "invalid_deck_result_document",
        "result_validation",
    ))
}

pub(crate) fn deck_allocation_error(object: &'static str) -> Box<WasmError> {
    Box::new(WasmError::new(
        format!("could not allocate {object}"),
        "result_allocation_failed",
        "result_projection",
    ))
}

pub(crate) fn deck_plan_wasm_error(error: rspice_core::execution::DeckPlanError) -> Box<WasmError> {
    match error {
        rspice_core::execution::DeckPlanError::Aborted => aborted_error(),
        rspice_core::execution::DeckPlanError::ResourceLimit(error) => {
            Box::new(WasmError::resource_limit(error.to_string(), error))
        }
        other => Box::new(WasmError::new(
            other.to_string(),
            "invalid_deck_plan",
            "input_validation",
        )),
    }
}

pub(crate) fn materialized_run_wasm_error(
    error: rspice_core::execution::MaterializedRunError,
) -> Box<WasmError> {
    match error {
        rspice_core::execution::MaterializedRunError::Aborted => aborted_error(),
        rspice_core::execution::MaterializedRunError::DeckPlan(error) => {
            deck_plan_wasm_error(error)
        }
        rspice_core::execution::MaterializedRunError::Simulation(error) => {
            Box::new(WasmError::from_simulation_error(error))
        }
        other => Box::new(WasmError::new(
            other.to_string(),
            "deck_materialization_failed",
            "execution",
        )),
    }
}
