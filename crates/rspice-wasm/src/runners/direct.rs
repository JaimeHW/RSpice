//! Direct execution entry points.
//!
//! Each function takes an explicitly configured analysis request, runs it
//! through an abort-aware `rspice-core` entrypoint, and projects the result
//! into a versioned document.

use rspice_core::analysis::StbConfig;
use rspice_core::{AbortSignal, NoAbort, ResourceKind, ResourceLimits};

use crate::DetailedWasmResult;
use crate::WasmResult;
use crate::abort::ensure_not_aborted;
use crate::dto::{
    AcPointSnapshot, DcOperatingPoint, NetlistSummary, TransientSnapshot, WasmHealthReport,
    complex_series_from_slice, transient_snapshot_from_compressed_result,
    transient_snapshot_from_result,
};
use crate::errors::resource_limit_error;
use crate::errors::{WasmError, diagnostic_summary, startup_diagnostic_summary};
use crate::options::{WasmCompressionOptions, WasmExecutionOptions, WasmStbSweep};
use crate::result_document::{self, AnalogResultDocument};
use crate::stb_result_document::{self, StbResultDocument};
use crate::support::{engine_with_resource_limits, parse_netlist_detailed};

/// Summarize and semantically validate a netlist, returning typed diagnostics.
pub fn summarize_netlist_detailed(source: &str) -> DetailedWasmResult<NetlistSummary> {
    summarize_netlist_with_options_detailed(source, &WasmExecutionOptions::default())
}

/// Summarize a netlist under an explicit browser execution policy.
pub fn summarize_netlist_with_options_detailed(
    source: &str,
    options: &WasmExecutionOptions,
) -> DetailedWasmResult<NetlistSummary> {
    summarize_netlist_with_options_and_abort_detailed(source, options, &NoAbort)
}

/// Summarize a netlist while observing an explicit cooperative abort source.
pub fn summarize_netlist_with_options_and_abort_detailed(
    source: &str,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<NetlistSummary> {
    let netlist =
        parse_netlist_detailed(source, options.resource_limits.to_core(), external_abort)?;
    let startup_diagnostics = netlist
        .startup_diagnostics()
        .iter()
        .map(startup_diagnostic_summary)
        .collect();
    let summary = NetlistSummary {
        title: netlist.title,
        element_count: netlist.elements.len(),
        analysis_count: netlist.analyses.len(),
        model_count: netlist.models.len(),
        subcircuit_count: netlist.subcircuits.len(),
        parameter_count: netlist.params.all_params().len(),
        diagnostics: netlist.diagnostics.iter().map(diagnostic_summary).collect(),
        startup_diagnostics,
    };
    ensure_not_aborted(external_abort)?;
    Ok(summary)
}

/// Backward-compatible string-error summary API.
pub fn summarize_netlist(source: &str) -> WasmResult<NetlistSummary> {
    summarize_netlist_detailed(source).map_err(|error| error.message)
}

/// Run an operating point after strict semantic validation.
pub fn run_dc_operating_point_detailed(source: &str) -> DetailedWasmResult<DcOperatingPoint> {
    run_dc_operating_point_with_options_detailed(source, &WasmExecutionOptions::default())
}

/// Run an operating point under an explicit browser execution policy.
pub fn run_dc_operating_point_with_options_detailed(
    source: &str,
    options: &WasmExecutionOptions,
) -> DetailedWasmResult<DcOperatingPoint> {
    run_dc_operating_point_with_options_and_abort_detailed(source, options, &NoAbort)
}

/// Run an operating point under an explicit browser policy and cooperative
/// abort source.
pub fn run_dc_operating_point_with_options_and_abort_detailed(
    source: &str,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<DcOperatingPoint> {
    let resource_limits = options.resource_limits.to_core();
    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let result = engine_with_resource_limits(resource_limits)?
        .run_dc_op_with_abort(&netlist, external_abort)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
    ensure_not_aborted(external_abort)?;
    Ok(DcOperatingPoint {
        node_names: result.node_names,
        node_voltages: result.node_voltages,
        branch_names: result.branch_names,
        branch_currents: result.branch_currents,
    })
}

/// Backward-compatible string-error operating-point API.
pub fn run_dc_operating_point(source: &str) -> WasmResult<DcOperatingPoint> {
    run_dc_operating_point_detailed(source).map_err(|error| error.message)
}

pub(crate) fn validate_analysis_ordinal(ordinal: usize) -> DetailedWasmResult<()> {
    if ordinal == 0 {
        return Err(Box::new(WasmError::invalid_argument(
            "analysis ordinal must be one-based".to_owned(),
        )));
    }
    Ok(())
}

/// Run OP into the versioned, loss-aware analog document. Unlike the legacy
/// compatibility DTO, this retains engine-owned device observables and device
/// operating regions in addition to node voltages and branch currents.
pub fn run_operating_point_document_with_options_and_abort_detailed(
    source: &str,
    ordinal: usize,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<AnalogResultDocument> {
    validate_analysis_ordinal(ordinal)?;
    ensure_not_aborted(external_abort)?;
    let resource_limits = options.resource_limits.to_core();
    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let (result, report) = engine_with_resource_limits(resource_limits)?
        .run_dc_op_with_report_and_abort(&netlist, external_abort)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
    ensure_not_aborted(external_abort)?;
    result_document::operating_point_document(result, report, ordinal).map_err(|message| {
        Box::new(WasmError::new(
            message,
            "invalid_result_document",
            "result_validation",
        ))
    })
}

pub fn run_operating_point_document_detailed(
    source: &str,
) -> DetailedWasmResult<AnalogResultDocument> {
    run_operating_point_document_with_options_and_abort_detailed(
        source,
        1,
        &WasmExecutionOptions::default(),
        &NoAbort,
    )
}

/// Run a scalar-deck DC sweep into one typed document. The adapter unions
/// device observables across points and marks coordinate-local absence
/// explicitly instead of zero-filling it.
#[allow(clippy::too_many_arguments)]
pub fn run_dc_sweep_document_with_options_and_abort_detailed(
    source: &str,
    source_name: &str,
    start: f64,
    stop: f64,
    step: f64,
    ordinal: usize,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<AnalogResultDocument> {
    validate_analysis_ordinal(ordinal)?;
    ensure_not_aborted(external_abort)?;
    if source_name.trim().is_empty() {
        return Err(Box::new(WasmError::invalid_argument(
            "DC sweep source name must not be empty".to_owned(),
        )));
    }
    if !start.is_finite() || !stop.is_finite() || !step.is_finite() || step == 0.0 {
        return Err(Box::new(WasmError::invalid_argument(
            "DC sweep start/stop must be finite and step must be finite and nonzero".to_owned(),
        )));
    }
    let resource_limits = options.resource_limits.to_core();
    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let points = engine_with_resource_limits(resource_limits)?
        .run_dc_sweep_with_report_and_abort(
            &netlist,
            source_name,
            start,
            stop,
            step,
            external_abort,
        )
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
    ensure_not_aborted(external_abort)?;
    result_document::dc_sweep_document(source_name, points, ordinal).map_err(|message| {
        Box::new(WasmError::new(
            message,
            "invalid_result_document",
            "result_validation",
        ))
    })
}

pub fn run_dc_sweep_document_detailed(
    source: &str,
    source_name: &str,
    start: f64,
    stop: f64,
    step: f64,
) -> DetailedWasmResult<AnalogResultDocument> {
    run_dc_sweep_document_with_options_and_abort_detailed(
        source,
        source_name,
        start,
        stop,
        step,
        1,
        &WasmExecutionOptions::default(),
        &NoAbort,
    )
}

/// Run AC analysis after strict semantic validation.
pub fn run_ac_analysis_detailed(
    source: &str,
    frequencies: &[f64],
) -> DetailedWasmResult<Vec<AcPointSnapshot>> {
    run_ac_analysis_with_options_detailed(source, frequencies, &WasmExecutionOptions::default())
}

/// Run AC analysis under an explicit browser execution policy.
pub fn run_ac_analysis_with_options_detailed(
    source: &str,
    frequencies: &[f64],
    options: &WasmExecutionOptions,
) -> DetailedWasmResult<Vec<AcPointSnapshot>> {
    run_ac_analysis_with_options_and_abort_detailed(source, frequencies, options, &NoAbort)
}

/// Run AC analysis under an explicit browser policy and cooperative abort
/// source.
pub fn run_ac_analysis_with_options_and_abort_detailed(
    source: &str,
    frequencies: &[f64],
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<Vec<AcPointSnapshot>> {
    ensure_not_aborted(external_abort)?;
    if frequencies.is_empty() {
        return Err(Box::new(WasmError::invalid_argument(
            "AC analysis requires at least one frequency".to_string(),
        )));
    }
    let resource_limits = options.resource_limits.to_core();
    if frequencies.len() > resource_limits.max_analysis_points {
        return Err(resource_limit_error(
            ResourceKind::AnalysisPoints,
            frequencies.len(),
            resource_limits.max_analysis_points,
        ));
    }
    if let Some((index, frequency)) = frequencies
        .iter()
        .copied()
        .enumerate()
        .find(|(_, frequency)| !frequency.is_finite() || *frequency < 0.0)
    {
        return Err(Box::new(WasmError::invalid_argument(format!(
            "AC frequency at index {index} must be finite and non-negative, got {frequency}"
        ))));
    }

    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let results = engine_with_resource_limits(resource_limits)?
        .run_ac_with_abort(&netlist, frequencies, external_abort)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;

    let snapshots = results
        .into_iter()
        .map(|point| AcPointSnapshot {
            frequency: point.frequency,
            node_names: point.node_names,
            branch_names: point.branch_names,
            voltages: complex_series_from_slice(&point.voltages),
            currents: complex_series_from_slice(&point.currents),
        })
        .collect();
    ensure_not_aborted(external_abort)?;
    Ok(snapshots)
}

/// Backward-compatible string-error AC API.
pub fn run_ac_analysis(source: &str, frequencies: &[f64]) -> WasmResult<Vec<AcPointSnapshot>> {
    run_ac_analysis_detailed(source, frequencies).map_err(|error| error.message)
}

/// Run AC into the common versioned analog document, preserving complex node
/// voltages and branch currents as aligned series.
pub fn run_ac_document_with_options_and_abort_detailed(
    source: &str,
    frequencies: &[f64],
    ordinal: usize,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<AnalogResultDocument> {
    validate_analysis_ordinal(ordinal)?;
    let points = run_ac_analysis_with_options_and_abort_detailed(
        source,
        frequencies,
        options,
        external_abort,
    )?;
    result_document::ac_document(points, ordinal).map_err(|message| {
        Box::new(WasmError::new(
            message,
            "invalid_result_document",
            "result_validation",
        ))
    })
}

pub fn run_ac_document_detailed(
    source: &str,
    frequencies: &[f64],
) -> DetailedWasmResult<AnalogResultDocument> {
    run_ac_document_with_options_and_abort_detailed(
        source,
        frequencies,
        1,
        &WasmExecutionOptions::default(),
        &NoAbort,
    )
}

/// Run transient analysis after strict semantic validation.
pub fn run_transient_analysis_detailed(
    source: &str,
    tstop: f64,
    max_step: f64,
) -> DetailedWasmResult<TransientSnapshot> {
    run_transient_analysis_with_options_detailed(
        source,
        tstop,
        max_step,
        &WasmExecutionOptions::default(),
    )
}

/// Run transient analysis under an explicit browser execution policy.
pub(crate) fn validate_transient_request(
    tstop: f64,
    max_step: f64,
    resource_limits: ResourceLimits,
) -> DetailedWasmResult<()> {
    if !tstop.is_finite() || tstop <= 0.0 {
        return Err(Box::new(WasmError::invalid_argument(format!(
            "Transient stop time must be positive and finite, got {tstop}"
        ))));
    }
    if !max_step.is_finite() || max_step <= 0.0 {
        return Err(Box::new(WasmError::invalid_argument(format!(
            "Transient maximum step must be positive and finite, got {max_step}"
        ))));
    }
    let estimated_points = (tstop / max_step).ceil() as usize;
    let estimated_points = estimated_points.saturating_add(1);
    if estimated_points > resource_limits.max_analysis_points {
        return Err(resource_limit_error(
            ResourceKind::AnalysisPoints,
            estimated_points,
            resource_limits.max_analysis_points,
        ));
    }
    Ok(())
}

pub fn run_transient_analysis_with_options_detailed(
    source: &str,
    tstop: f64,
    max_step: f64,
    options: &WasmExecutionOptions,
) -> DetailedWasmResult<TransientSnapshot> {
    run_transient_analysis_with_options_and_abort_detailed(
        source, tstop, max_step, options, &NoAbort,
    )
}

/// Run transient analysis under an explicit browser policy and cooperative
/// abort source.
pub fn run_transient_analysis_with_options_and_abort_detailed(
    source: &str,
    tstop: f64,
    max_step: f64,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<TransientSnapshot> {
    ensure_not_aborted(external_abort)?;
    let resource_limits = options.resource_limits.to_core();
    validate_transient_request(tstop, max_step, resource_limits)?;

    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let result = engine_with_resource_limits(resource_limits)?
        .run_tran_with_abort(&netlist, tstop, max_step, external_abort)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;

    let snapshot = transient_snapshot_from_result(result).map_err(|message| {
        Box::new(WasmError::new(
            message,
            "invalid_transient_result",
            "result_validation",
        ))
    })?;
    ensure_not_aborted(external_abort)?;
    Ok(snapshot)
}

/// Run transient analysis with bounded, multi-channel analog compression.
pub fn run_transient_analysis_compressed_detailed(
    source: &str,
    tstop: f64,
    max_step: f64,
    compression: &WasmCompressionOptions,
) -> DetailedWasmResult<TransientSnapshot> {
    run_transient_analysis_compressed_with_options_detailed(
        source,
        tstop,
        max_step,
        compression,
        &WasmExecutionOptions::default(),
    )
}

/// Run a compressed transient under explicit compression and browser resource
/// policies. The solver and authored output projection are identical to the
/// full-grid path; only the published analog history is decimated.
pub fn run_transient_analysis_compressed_with_options_detailed(
    source: &str,
    tstop: f64,
    max_step: f64,
    compression: &WasmCompressionOptions,
    options: &WasmExecutionOptions,
) -> DetailedWasmResult<TransientSnapshot> {
    run_transient_analysis_compressed_with_options_and_abort_detailed(
        source,
        tstop,
        max_step,
        compression,
        options,
        &NoAbort,
    )
}

/// Run compressed transient analysis under explicit browser policies and a
/// cooperative abort source. Both the solver and compression pass observe the
/// same signal through the core abort-aware entrypoint.
pub fn run_transient_analysis_compressed_with_options_and_abort_detailed(
    source: &str,
    tstop: f64,
    max_step: f64,
    compression: &WasmCompressionOptions,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<TransientSnapshot> {
    ensure_not_aborted(external_abort)?;
    let resource_limits = options.resource_limits.to_core();
    validate_transient_request(tstop, max_step, resource_limits)?;
    let compression = compression.to_core()?;
    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let result = engine_with_resource_limits(resource_limits)?
        .run_tran_compressed_with_abort(&netlist, tstop, max_step, compression, external_abort)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
    let snapshot = transient_snapshot_from_compressed_result(result).map_err(|message| {
        Box::new(WasmError::new(
            message,
            "invalid_transient_result",
            "result_validation",
        ))
    })?;
    ensure_not_aborted(external_abort)?;
    Ok(snapshot)
}

/// Backward-compatible string-error compressed transient API.
pub fn run_transient_analysis_compressed(
    source: &str,
    tstop: f64,
    max_step: f64,
    compression: &WasmCompressionOptions,
) -> WasmResult<TransientSnapshot> {
    run_transient_analysis_compressed_detailed(source, tstop, max_step, compression)
        .map_err(|error| error.message)
}

/// Backward-compatible string-error transient API.
pub fn run_transient_analysis(
    source: &str,
    tstop: f64,
    max_step: f64,
) -> WasmResult<TransientSnapshot> {
    run_transient_analysis_detailed(source, tstop, max_step).map_err(|error| error.message)
}

/// Run transient into the common result document. Projected-out solution
/// channels remain present with `None` samples, while device OP/store traces
/// retain explicit owners and unknown units.
#[allow(clippy::too_many_arguments)]
pub fn run_transient_document_with_options_and_abort_detailed(
    source: &str,
    tstop: f64,
    max_step: f64,
    ordinal: usize,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<AnalogResultDocument> {
    validate_analysis_ordinal(ordinal)?;
    let snapshot = run_transient_analysis_with_options_and_abort_detailed(
        source,
        tstop,
        max_step,
        options,
        external_abort,
    )?;
    result_document::transient_document(snapshot, ordinal).map_err(|message| {
        Box::new(WasmError::new(
            message,
            "invalid_result_document",
            "result_validation",
        ))
    })
}

pub fn run_transient_document_detailed(
    source: &str,
    tstop: f64,
    max_step: f64,
) -> DetailedWasmResult<AnalogResultDocument> {
    run_transient_document_with_options_and_abort_detailed(
        source,
        tstop,
        max_step,
        1,
        &WasmExecutionOptions::default(),
        &NoAbort,
    )
}

/// Run scalar-deck input-referred noise into the common typed result document.
/// It preserves complex small-signal voltages/currents, total densities, gain,
/// and sparse per-device contribution identities with explicit validity.
#[allow(clippy::too_many_arguments)]
pub fn run_noise_document_with_options_and_abort_detailed(
    source: &str,
    output_node: &str,
    reference_node: Option<&str>,
    input_source: &str,
    frequencies: &[f64],
    ordinal: usize,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<AnalogResultDocument> {
    validate_analysis_ordinal(ordinal)?;
    ensure_not_aborted(external_abort)?;
    if output_node.trim().is_empty() || input_source.trim().is_empty() {
        return Err(Box::new(WasmError::invalid_argument(
            "noise output node and input source must not be empty".to_owned(),
        )));
    }
    if frequencies.is_empty() {
        return Err(Box::new(WasmError::invalid_argument(
            "noise analysis requires at least one frequency".to_owned(),
        )));
    }
    let resource_limits = options.resource_limits.to_core();
    if frequencies.len() > resource_limits.max_analysis_points {
        return Err(resource_limit_error(
            ResourceKind::AnalysisPoints,
            frequencies.len(),
            resource_limits.max_analysis_points,
        ));
    }
    if let Some((index, frequency)) = frequencies
        .iter()
        .copied()
        .enumerate()
        .find(|(_, frequency)| !frequency.is_finite() || *frequency <= 0.0)
    {
        return Err(Box::new(WasmError::invalid_argument(format!(
            "noise frequency at index {index} must be finite and positive, got {frequency}"
        ))));
    }
    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let engine = engine_with_resource_limits(resource_limits)?;
    let points = engine
        .run_noise_named_with_input_source_and_abort(
            &netlist,
            output_node,
            reference_node,
            input_source,
            frequencies,
            engine.config().temperature,
            external_abort,
        )
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
    ensure_not_aborted(external_abort)?;
    result_document::noise_document(points, ordinal).map_err(|message| {
        Box::new(WasmError::new(
            message,
            "invalid_result_document",
            "result_validation",
        ))
    })
}

pub fn run_noise_document_detailed(
    source: &str,
    output_node: &str,
    reference_node: Option<&str>,
    input_source: &str,
    frequencies: &[f64],
) -> DetailedWasmResult<AnalogResultDocument> {
    run_noise_document_with_options_and_abort_detailed(
        source,
        output_node,
        reference_node,
        input_source,
        frequencies,
        1,
        &WasmExecutionOptions::default(),
        &NoAbort,
    )
}

/// Run one scalar Tian loop-stability analysis into its lossless retained
/// result document. The direct request deliberately does not consume authored
/// STEP/TEMP axes.
#[allow(clippy::too_many_arguments)]
pub fn run_stb_document_with_options_and_abort_detailed(
    source: &str,
    probe: &str,
    sweep: WasmStbSweep,
    points: usize,
    start_frequency: f64,
    stop_frequency: f64,
    compute_nyquist: bool,
    ordinal: usize,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<StbResultDocument> {
    validate_analysis_ordinal(ordinal)?;
    ensure_not_aborted(external_abort)?;
    if probe.trim().is_empty() {
        return Err(Box::new(WasmError::invalid_argument(
            "STB probe name must not be empty".to_owned(),
        )));
    }
    let config = StbConfig::new()
        .with_sweep(start_frequency, stop_frequency, points)
        .with_sweep_type(sweep.to_core())
        .with_probe(probe)
        .with_nyquist(compute_nyquist);
    config.validate().map_err(|message| {
        Box::new(WasmError::invalid_argument(format!(
            "invalid STB request: {message}"
        )))
    })?;

    let resource_limits = options.resource_limits.to_core();
    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let result = engine_with_resource_limits(resource_limits)?
        .run_stb_with_abort(&netlist, config, external_abort)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
    ensure_not_aborted(external_abort)?;
    match stb_result_document::stb_document_with_abort(result, ordinal, external_abort) {
        Ok(document) => Ok(document),
        Err(stb_result_document::StbDocumentError::Aborted) => Err(Box::new(
            WasmError::from_simulation_error(rspice_core::engine::SimulationError::Aborted),
        )),
        Err(stb_result_document::StbDocumentError::Invalid(message)) => Err(Box::new(
            WasmError::new(message, "invalid_result_document", "result_validation"),
        )),
        Err(stb_result_document::StbDocumentError::Allocation(message)) => Err(Box::new(
            WasmError::new(message, "result_allocation_failed", "result_projection"),
        )),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn run_stb_document_detailed(
    source: &str,
    probe: &str,
    sweep: WasmStbSweep,
    points: usize,
    start_frequency: f64,
    stop_frequency: f64,
    compute_nyquist: bool,
) -> DetailedWasmResult<StbResultDocument> {
    run_stb_document_with_options_and_abort_detailed(
        source,
        probe,
        sweep,
        points,
        start_frequency,
        stop_frequency,
        compute_nyquist,
        1,
        &WasmExecutionOptions::default(),
        &NoAbort,
    )
}

/// Exercise the configured browser parser-to-solver path without I/O.
pub fn health_check_with_options_detailed(
    options: &WasmExecutionOptions,
) -> DetailedWasmResult<WasmHealthReport> {
    health_check_with_options_and_abort_detailed(options, &NoAbort)
}

/// Execute the readiness probe with the same deadline and cancellation
/// contract as analysis calls.
pub fn health_check_with_options_and_abort_detailed(
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<WasmHealthReport> {
    let report = engine_with_resource_limits(options.resource_limits.to_core())?
        .health_check_with_abort(external_abort)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
    Ok(WasmHealthReport {
        status: "ready".to_string(),
        ready: true,
        duration_seconds: report.elapsed.as_secs_f64(),
        element_count: report.element_count,
        node_count: report.node_count,
        branch_count: report.branch_count,
        output_voltage: report.output_voltage,
    })
}
