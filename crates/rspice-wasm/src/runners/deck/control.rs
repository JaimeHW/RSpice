//! Publish the shared control host's completed runs through core documents.

use super::*;
use rspice_core::engine::{
    ControlAnalysisResult, ControlCircuit, ControlCommandEffect, ControlExecutionError,
    ControlPresentationKind,
};
use rspice_core::execution::control::{
    ControlError, ControlErrorKind, ControlLimits, ControlProgram,
};
use rspice_core::execution::topology_fingerprint_with_abort;
use rspice_core::netlist::ControlScriptSource;

pub(super) fn run(
    netlist: Netlist,
    options: &WasmExecutionOptions,
    abort: &dyn AbortSignal,
) -> DetailedWasmResult<DeckExecution> {
    let limits = options.resource_limits.to_core();
    let initial_plan = DeckPlan::from_netlist_with_abort(&netlist, &limits, abort)
        .map_err(deck_plan_wasm_error)?;
    if !initial_plan.axes().is_empty()
        || !netlist.fft_analyses.is_empty()
        || netlist
            .analyses
            .iter()
            .any(|command| matches!(command, AnalysisCommand::Four { .. }))
        || netlist.options.restart.is_some()
        || options.transient_compression.is_some()
    {
        return Err(unsupported_deck_analysis(
            "control-script execution with declarative run axes, Fourier/FFT, restart or compression is not yet implemented".into(),
        ));
    }
    let script = netlist
        .control_script
        .clone()
        .ok_or_else(|| document_error("control execution requires retained source".into()))?;
    let program = ControlProgram::parse_deck_with_abort(
        script.text(),
        ControlLimits {
            max_source_bytes: limits.max_expanded_source_bytes,
            max_source_lines: limits.max_netlist_lines,
            max_loop_values: limits.max_batch_runs,
            ..ControlLimits::default()
        },
        abort,
    )
    .map_err(|error| command_error(error, &script))?;
    let engine = engine_with_resource_limits(limits)?;
    let mut session = program.start(netlist.params.clone());
    let mut circuit =
        ControlCircuit::new(netlist).map_err(|error| execution_error(error, &script))?;
    let mut presentations = Vec::new();
    let mut presentation_values = 0usize;
    let mut topologies = Vec::new();

    while let Some(command) = session
        .next_command(&mut circuit, abort)
        .map_err(|error| command_error(error, &script))?
    {
        // The core accounts for all retained raw datasets. Charge previously
        // retained presentation samples as well before the next allocation.
        let mut config = engine.config().clone();
        config.resource_limits.max_result_values =
            limits.max_result_values.saturating_sub(presentation_values);
        let bounded = Engine::try_new(config).map_err(|error| {
            simulation_error(rspice_core::SimulationError::Configuration(error))
        })?;
        let previous = circuit.datasets().len();
        match circuit
            .execute(&bounded, &command, session.variables(), abort)
            .map_err(|error| execution_error(error, &script))?
        {
            ControlCommandEffect::CircuitChanged => {}
            ControlCommandEffect::Analyses(_) => {
                let topology = topology_fingerprint_with_abort(&bounded, circuit.netlist(), abort)
                    .map_err(simulation_error)?;
                for _ in previous..circuit.datasets().len() {
                    topologies.push(topology);
                }
            }
            ControlCommandEffect::Presentation(presentation) => {
                // Even metadata-only scripts cannot retain unbounded output.
                let requested = presentations.len().saturating_add(1);
                if requested > limits.max_batch_runs {
                    return Err(resource_limit_error(
                        ResourceKind::BatchRuns,
                        requested,
                        limits.max_batch_runs,
                    ));
                }
                let traces = match &presentation.kind {
                    ControlPresentationKind::Plot { traces, .. }
                    | ControlPresentationKind::Print(traces) => traces.as_slice(),
                    ControlPresentationKind::UnitsChanged { .. } => &[],
                };
                let count = traces.iter().fold(0usize, |count, trace| {
                    count.saturating_add(
                        trace
                            .x
                            .samples
                            .len()
                            .saturating_add(trace.y.samples.len())
                            .saturating_mul(2),
                    )
                });
                presentation_values = presentation_values.saturating_add(count);
                if presentation_values > limits.max_result_values {
                    return Err(resource_limit_error(
                        ResourceKind::ResultValues,
                        presentation_values,
                        limits.max_result_values,
                    ));
                }
                presentations.push(presentation);
            }
        }
    }
    ensure_not_aborted(abort)?;
    if circuit.datasets().is_empty() {
        return Err(unsupported_deck_analysis(
            "control script completed without an analysis result".into(),
        ));
    }

    // The ordered script determines its analysis list at runtime. Describe
    // that realized list; do not publish the declarative implicit-OP placeholder.
    let mut realized = circuit.netlist().clone();
    realized.analyses = circuit
        .datasets()
        .iter()
        .map(|dataset| dataset.command.clone())
        .collect();
    realized.control_script = None;
    let plan = DeckPlan::from_netlist_with_abort(&realized, &limits, abort)
        .map_err(deck_plan_wasm_error)?;
    let coordinates = plan
        .coordinates_with_abort(&limits, abort)
        .map_err(deck_plan_wasm_error)?;
    let coordinate = coordinates
        .first()
        .ok_or_else(|| document_error("control run has no coordinate".into()))?;
    let result_coordinate = ResultCoordinate::from_run_coordinate(coordinate);
    let mut results = Vec::new();
    let mut dataset_names = Vec::new();
    let mut retained = presentation_values;
    for (dataset, topology) in circuit.into_datasets().into_iter().zip(topologies) {
        ensure_not_aborted(abort)?;
        let builder = match &dataset.result {
            ControlAnalysisResult::OperatingPoint(result) => {
                AnalysisResultDocument::from_operating_point(
                    dataset.analysis_id,
                    result,
                    dataset.device_op_report.as_deref(),
                )
            }
            ControlAnalysisResult::Ac(points) => {
                AnalysisResultDocument::from_ac(dataset.analysis_id, points)
            }
            ControlAnalysisResult::Transient(result) => AnalysisResultDocument::from_transient(
                dataset.analysis_id,
                result,
                None,
                Vec::new(),
            ),
        }
        .map_err(document_projection_error)?;
        let namespace = format!("control/{}", dataset.analysis_id);
        let document = builder
            .coordinate(result_coordinate.clone())
            .topology_fingerprint(topology)
            .namespaces(ResultNamespaces {
                output: namespace.clone(),
                checkpoint: namespace,
            })
            .build_with_abort(abort)
            .map_err(document_projection_error)?;
        retained = retained.saturating_add(document.total_value_count());
        if retained > limits.max_result_values {
            return Err(resource_limit_error(
                ResourceKind::ResultValues,
                retained,
                limits.max_result_values,
            ));
        }
        dataset_names.push(dataset.name);
        results.push(document);
    }
    Ok(DeckExecution {
        plan,
        coordinates,
        results,
        control_presentations: presentations,
        control_datasets: dataset_names,
    })
}

fn command_error(error: ControlError, script: &ControlScriptSource) -> Box<WasmError> {
    let (code, category) = match error.kind {
        ControlErrorKind::Syntax => ("control.syntax", "netlist"),
        ControlErrorKind::Expression => ("control.expression", "netlist"),
        ControlErrorKind::Host => ("control.command", "netlist"),
        ControlErrorKind::ResourceLimit => ("control.resource_limit", "resource_limit"),
        ControlErrorKind::Aborted => ("control.aborted", "cancellation"),
    };
    source_error(
        WasmError::new(error.to_string(), code, category),
        error.line,
        script,
    )
}

fn execution_error(error: ControlExecutionError, script: &ControlScriptSource) -> Box<WasmError> {
    match error {
        ControlExecutionError::Command(error) => command_error(error, script),
        ControlExecutionError::Simulation { line, source } => {
            source_error(WasmError::from_simulation_error(source), line, script)
        }
        ControlExecutionError::Configuration { line, source } => source_error(
            WasmError::from_simulation_error(rspice_core::SimulationError::Configuration(source)),
            line,
            script,
        ),
    }
}

fn source_error(mut error: WasmError, line: usize, script: &ControlScriptSource) -> Box<WasmError> {
    if let Some(origin) = script.origin(line) {
        error.primary_source = origin
            .path
            .as_ref()
            .map(|source| source.display().to_string());
        error.primary_line = Some(origin.line);
    } else {
        error.primary_line = Some(line);
    }
    Box::new(error)
}
