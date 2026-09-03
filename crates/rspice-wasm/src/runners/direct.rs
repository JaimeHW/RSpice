//! Direct execution entry points.
//!
//! These run one explicitly configured analysis from JavaScript arguments
//! instead of from an authored card. They still produce the same core
//! `AnalysisResultDocument` and the same handle as the deck route: a direct
//! request is planned as a one-analysis [`DeckPlan`], which is where its
//! canonical `AnalysisInstanceId` and its single run coordinate come from.
//! Nothing here mints an identity of its own.

use rspice_core::execution::{
    AnalysisInstanceId, AnalysisKind, AnalysisResultDocument, AnalysisResultDocumentBuilder,
    DeckPlan, RunCoordinate, SignalUnit, topology_fingerprint_with_abort,
};
use rspice_core::{AbortSignal, Engine, NoAbort, ResourceKind, ResourceLimits, Value};

use crate::DetailedWasmResult;
use crate::abort::ensure_not_aborted;
use crate::dto::{NetlistSummary, WasmHealthReport};
use crate::errors::{
    WasmError, diagnostic_summary, resource_limit_error, startup_diagnostic_summary,
};
use crate::options::WasmExecutionOptions;
use crate::runners::deck::{
    DeckExecution, deck_plan_wasm_error, document_projection_error, preflight_transient_points,
    simulation_error,
};
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
pub fn summarize_netlist(source: &str) -> crate::WasmResult<NetlistSummary> {
    summarize_netlist_detailed(source).map_err(|error| error.message)
}

/// Plan one direct request and finish its documents at its one coordinate.
///
/// A `DeckPlan` with no axes has exactly one coordinate. Consuming that
/// instead of inventing a "scalar, no coordinate" special case is what keeps
/// a direct result and the same analysis inside a deck describable by one
/// reader.
fn direct_execution(
    kind: AnalysisKind,
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    resource_limits: ResourceLimits,
    abort: &dyn AbortSignal,
    project: impl FnOnce(AnalysisInstanceId) -> DetailedWasmResult<Vec<AnalysisResultDocumentBuilder>>,
) -> DetailedWasmResult<DeckExecution> {
    let plan = DeckPlan::for_direct_analyses(kind, 1).map_err(deck_plan_wasm_error)?;
    let coordinates: Vec<RunCoordinate> = plan
        .coordinates_with_abort(&resource_limits, abort)
        .map_err(deck_plan_wasm_error)?;
    let coordinate = coordinates.first().cloned().ok_or_else(|| {
        Box::new(WasmError::new(
            "a one-analysis plan produced no run coordinate".to_owned(),
            "invalid_deck_plan",
            "result_validation",
        ))
    })?;
    let id = plan
        .analyses()
        .first()
        .ok_or_else(|| {
            Box::new(WasmError::new(
                "a one-analysis plan produced no planned analysis".to_owned(),
                "invalid_deck_plan",
                "result_validation",
            ))
        })?
        .id();
    let topology =
        topology_fingerprint_with_abort(engine, netlist, abort).map_err(simulation_error)?;
    let result_coordinate =
        rspice_core::execution::result_document::ResultCoordinate::from_run_coordinate(&coordinate);

    let builders = project(id)?;
    let mut results = Vec::new();
    results
        .try_reserve_exact(builders.len())
        .map_err(|_| allocation_error("direct analysis results"))?;
    let mut retained = 0usize;
    for builder in builders {
        let document = builder
            .coordinate(result_coordinate.clone())
            .topology_fingerprint(topology)
            .build_with_abort(abort)
            .map_err(document_projection_error)?;
        retained = retained
            .checked_add(document.total_value_count())
            .ok_or_else(|| allocation_error("direct retained-value accounting"))?;
        if retained > resource_limits.max_result_values {
            return Err(resource_limit_error(
                ResourceKind::ResultValues,
                retained,
                resource_limits.max_result_values,
            ));
        }
        results.push(document);
    }
    Ok(DeckExecution {
        plan,
        coordinates,
        results,
    })
}

/// Run one operating point into the shared result document.
pub fn run_operating_point_document_with_options_and_abort_detailed(
    source: &str,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<DeckExecution> {
    ensure_not_aborted(external_abort)?;
    let resource_limits = options.resource_limits.to_core();
    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let engine = engine_with_resource_limits(resource_limits)?;
    direct_execution(
        AnalysisKind::Op,
        &engine,
        &netlist,
        resource_limits,
        external_abort,
        |id| {
            let (result, report) = engine
                .run_dc_op_with_report_and_abort(&netlist, external_abort)
                .map_err(simulation_error)?;
            ensure_not_aborted(external_abort)?;
            Ok(vec![
                AnalysisResultDocument::from_operating_point(id, &result, Some(&report))
                    .map_err(document_projection_error)?,
            ])
        },
    )
}

pub fn run_operating_point_document_detailed(source: &str) -> DetailedWasmResult<DeckExecution> {
    run_operating_point_document_with_options_and_abort_detailed(
        source,
        &WasmExecutionOptions::default(),
        &NoAbort,
    )
}

/// Run one linear source sweep into the shared result document.
#[allow(clippy::too_many_arguments)]
pub fn run_dc_sweep_document_with_options_and_abort_detailed(
    source: &str,
    source_name: &str,
    start: f64,
    stop: f64,
    step: f64,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<DeckExecution> {
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
    let engine = engine_with_resource_limits(resource_limits)?;
    direct_execution(
        AnalysisKind::Dc,
        &engine,
        &netlist,
        resource_limits,
        external_abort,
        |id| {
            let points = engine
                .run_dc_sweep_with_report_and_abort(
                    &netlist,
                    source_name,
                    start,
                    stop,
                    step,
                    external_abort,
                )
                .map_err(simulation_error)?;
            ensure_not_aborted(external_abort)?;
            Ok(vec![
                AnalysisResultDocument::from_dc_sweep(
                    id,
                    source_name,
                    sweep_source_unit(source_name),
                    &points,
                )
                .map_err(document_projection_error)?,
            ])
        },
    )
}

pub fn run_dc_sweep_document_detailed(
    source: &str,
    source_name: &str,
    start: f64,
    stop: f64,
    step: f64,
) -> DetailedWasmResult<DeckExecution> {
    run_dc_sweep_document_with_options_and_abort_detailed(
        source,
        source_name,
        start,
        stop,
        step,
        &WasmExecutionOptions::default(),
        &NoAbort,
    )
}

/// Run one AC sweep over an explicit frequency grid.
pub fn run_ac_document_with_options_and_abort_detailed(
    source: &str,
    frequencies: &[f64],
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<DeckExecution> {
    ensure_not_aborted(external_abort)?;
    let resource_limits = options.resource_limits.to_core();
    validate_frequency_grid("AC", frequencies, false, resource_limits)?;
    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let engine = engine_with_resource_limits(resource_limits)?;
    direct_execution(
        AnalysisKind::Ac,
        &engine,
        &netlist,
        resource_limits,
        external_abort,
        |id| {
            let points = engine
                .run_ac_with_abort(&netlist, frequencies, external_abort)
                .map_err(simulation_error)?;
            ensure_not_aborted(external_abort)?;
            Ok(vec![
                AnalysisResultDocument::from_ac(id, &points).map_err(document_projection_error)?,
            ])
        },
    )
}

pub fn run_ac_document_detailed(
    source: &str,
    frequencies: &[f64],
) -> DetailedWasmResult<DeckExecution> {
    run_ac_document_with_options_and_abort_detailed(
        source,
        frequencies,
        &WasmExecutionOptions::default(),
        &NoAbort,
    )
}

/// Run one transient over an explicit interval.
pub fn run_transient_document_with_options_and_abort_detailed(
    source: &str,
    tstop: f64,
    max_step: f64,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<DeckExecution> {
    ensure_not_aborted(external_abort)?;
    let resource_limits = options.resource_limits.to_core();
    preflight_transient_points(tstop, max_step, resource_limits)?;
    let compression = options.compression_config()?;
    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let engine = engine_with_resource_limits(resource_limits)?;
    direct_execution(
        AnalysisKind::Tran,
        &engine,
        &netlist,
        resource_limits,
        external_abort,
        |id| {
            let (result, report) = match compression {
                Some(config) => {
                    let compressed = engine
                        .run_tran_compressed_with_abort(
                            &netlist,
                            tstop,
                            max_step,
                            config,
                            external_abort,
                        )
                        .map_err(simulation_error)?;
                    let report = compressed.compression_report.clone();
                    let expanded = compressed.try_into_transient().map_err(|message| {
                        Box::new(WasmError::new(
                            message,
                            "invalid_result_document",
                            "result_validation",
                        ))
                    })?;
                    (expanded, Some(report))
                }
                None => (
                    engine
                        .run_tran_with_abort(&netlist, tstop, max_step, external_abort)
                        .map_err(simulation_error)?,
                    None,
                ),
            };
            ensure_not_aborted(external_abort)?;
            if !result.fft_results.is_empty() {
                return Err(Box::new(WasmError::new(
                    "this deck attaches .FFT post-processing to its transient; run it through runAuthoredDeckDocument so every spectrum keeps its own analysis identity".to_owned(),
                    "unsupported_deck_analysis",
                    "unsupported_feature",
                )));
            }
            Ok(vec![
                AnalysisResultDocument::from_transient(id, &result, report.as_ref(), Vec::new())
                    .map_err(document_projection_error)?,
            ])
        },
    )
}

pub fn run_transient_document_detailed(
    source: &str,
    tstop: f64,
    max_step: f64,
) -> DetailedWasmResult<DeckExecution> {
    run_transient_document_with_options_and_abort_detailed(
        source,
        tstop,
        max_step,
        &WasmExecutionOptions::default(),
        &NoAbort,
    )
}

/// Run one input-referred noise sweep over an explicit frequency grid.
#[allow(clippy::too_many_arguments)]
pub fn run_noise_document_with_options_and_abort_detailed(
    source: &str,
    output_node: &str,
    reference_node: Option<&str>,
    input_source: &str,
    frequencies: &[f64],
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<DeckExecution> {
    ensure_not_aborted(external_abort)?;
    if output_node.trim().is_empty() || input_source.trim().is_empty() {
        return Err(Box::new(WasmError::invalid_argument(
            "noise output node and input source must not be empty".to_owned(),
        )));
    }
    let resource_limits = options.resource_limits.to_core();
    validate_frequency_grid("noise", frequencies, true, resource_limits)?;
    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let engine = engine_with_resource_limits(resource_limits)?;
    direct_execution(
        AnalysisKind::Noise,
        &engine,
        &netlist,
        resource_limits,
        external_abort,
        |id| {
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
                .map_err(simulation_error)?;
            ensure_not_aborted(external_abort)?;
            Ok(vec![
                AnalysisResultDocument::from_noise(id, &points)
                    .map_err(document_projection_error)?,
            ])
        },
    )
}

pub fn run_noise_document_detailed(
    source: &str,
    output_node: &str,
    reference_node: Option<&str>,
    input_source: &str,
    frequencies: &[f64],
) -> DetailedWasmResult<DeckExecution> {
    run_noise_document_with_options_and_abort_detailed(
        source,
        output_node,
        reference_node,
        input_source,
        frequencies,
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
        .map_err(simulation_error)?;
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

fn validate_frequency_grid(
    analysis: &str,
    frequencies: &[Value],
    strictly_positive: bool,
    resource_limits: ResourceLimits,
) -> DetailedWasmResult<()> {
    if frequencies.is_empty() {
        return Err(Box::new(WasmError::invalid_argument(format!(
            "{analysis} analysis requires at least one frequency"
        ))));
    }
    if frequencies.len() > resource_limits.max_analysis_points {
        return Err(resource_limit_error(
            ResourceKind::AnalysisPoints,
            frequencies.len(),
            resource_limits.max_analysis_points,
        ));
    }
    let rejected = frequencies.iter().copied().enumerate().find(|(_, value)| {
        !value.is_finite() || *value < 0.0 || (strictly_positive && *value <= 0.0)
    });
    if let Some((index, frequency)) = rejected {
        let bound = if strictly_positive {
            "positive"
        } else {
            "non-negative"
        };
        return Err(Box::new(WasmError::invalid_argument(format!(
            "{analysis} frequency at index {index} must be finite and {bound}, got {frequency}"
        ))));
    }
    Ok(())
}

/// Unit of a directly requested sweep source.
///
/// This mirrors the first-letter element contract the parser itself uses:
/// `V`-prefixed instances are voltage sources and `I`-prefixed are current
/// sources. Anything else is a swept parameter with no intrinsic unit.
fn sweep_source_unit(source_name: &str) -> SignalUnit {
    match source_name
        .trim()
        .chars()
        .next()
        .map(|first| first.to_ascii_uppercase())
    {
        Some('V') => SignalUnit::Volt,
        Some('I') => SignalUnit::Ampere,
        _ => SignalUnit::Dimensionless,
    }
}

fn allocation_error(object: &'static str) -> Box<WasmError> {
    Box::new(WasmError::new(
        format!("could not allocate {object}"),
        "result_allocation_failed",
        "result_projection",
    ))
}

#[cfg(test)]
mod tests {
    use rspice_core::abort_signal::ImmediateAbort;
    use rspice_core::execution::AnalysisResultKind;

    use super::*;

    const DECK: &str = "browser direct deck\n\
V1 in 0 1 AC 1\n\
R1 in out 1k\n\
C1 out 0 1p\n\
.END\n";

    /// A direct request is planned, so its result carries the canonical
    /// analysis identity and the one coordinate a no-axis plan has.
    #[test]
    fn a_direct_request_is_planned_and_carries_its_canonical_identity() {
        let cases: Vec<(AnalysisResultKind, &str, DeckExecution)> = vec![
            (
                AnalysisResultKind::OperatingPoint,
                "op-001",
                run_operating_point_document_detailed(DECK).expect("OP runs"),
            ),
            (
                AnalysisResultKind::DcSweep,
                "dc-001",
                run_dc_sweep_document_detailed(DECK, "V1", 0.0, 1.0, 0.5).expect("DC runs"),
            ),
            (
                AnalysisResultKind::Ac,
                "ac-001",
                run_ac_document_detailed(DECK, &[1.0e3, 1.0e4]).expect("AC runs"),
            ),
            (
                AnalysisResultKind::Transient,
                "tran-001",
                run_transient_document_detailed(DECK, 1.0e-8, 1.0e-9).expect("TRAN runs"),
            ),
            (
                AnalysisResultKind::Noise,
                "noise-001",
                run_noise_document_detailed(DECK, "out", None, "V1", &[1.0e3, 1.0e4])
                    .expect("noise runs"),
            ),
        ];

        for (kind, tag, execution) in cases {
            assert_eq!(execution.plan.axes().len(), 0, "{kind:?}");
            assert_eq!(execution.coordinates.len(), 1, "{kind:?}");
            assert_eq!(execution.results.len(), 1, "{kind:?}");
            let document = &execution.results[0];
            assert_eq!(document.result_kind(), kind);
            assert_eq!(document.analysis().tag(), tag);
            assert!(
                document.coordinate().is_some(),
                "{kind:?} names the single coordinate of its plan"
            );
            assert!(
                document.topology_fingerprint().is_some(),
                "{kind:?} carries the solved topology identity"
            );
            assert!(
                document.namespaces().is_none(),
                "{kind:?} was not run under a deck artifact namespace"
            );
            document
                .validate()
                .expect("the published document is valid");
        }
    }

    /// Invalid direct arguments fail before any solver work, with the typed
    /// argument category.
    #[test]
    fn direct_arguments_are_validated_before_any_solve() {
        for error in [
            *run_ac_document_detailed(DECK, &[]).expect_err("an empty AC grid is invalid"),
            *run_ac_document_detailed(DECK, &[-1.0]).expect_err("a negative frequency is invalid"),
            *run_noise_document_detailed(DECK, "out", None, "V1", &[0.0])
                .expect_err("a zero noise frequency is invalid"),
            *run_noise_document_detailed(DECK, "", None, "V1", &[1.0e3])
                .expect_err("an empty noise output is invalid"),
            *run_dc_sweep_document_detailed(DECK, "V1", 0.0, 1.0, 0.0)
                .expect_err("a zero DC step is invalid"),
            *run_transient_document_detailed(DECK, -1.0, 1.0e-9)
                .expect_err("a negative stop time is invalid"),
        ] {
            assert_eq!(error.code, "invalid_argument");
            assert_eq!(error.category, "input_validation");
        }
    }

    /// Every direct entry point observes the abort source it was given.
    #[test]
    fn every_direct_entry_point_observes_its_abort_source() {
        let options = WasmExecutionOptions::default();
        let failures = [
            *summarize_netlist_with_options_and_abort_detailed(DECK, &options, &ImmediateAbort)
                .expect_err("summarize observes cancellation"),
            *run_operating_point_document_with_options_and_abort_detailed(
                DECK,
                &options,
                &ImmediateAbort,
            )
            .expect_err("OP observes cancellation"),
            *run_dc_sweep_document_with_options_and_abort_detailed(
                DECK,
                "V1",
                0.0,
                1.0,
                0.5,
                &options,
                &ImmediateAbort,
            )
            .expect_err("DC observes cancellation"),
            *run_ac_document_with_options_and_abort_detailed(
                DECK,
                &[1.0e3],
                &options,
                &ImmediateAbort,
            )
            .expect_err("AC observes cancellation"),
            *run_transient_document_with_options_and_abort_detailed(
                DECK,
                1.0e-8,
                1.0e-9,
                &options,
                &ImmediateAbort,
            )
            .expect_err("TRAN observes cancellation"),
            *run_noise_document_with_options_and_abort_detailed(
                DECK,
                "out",
                None,
                "V1",
                &[1.0e3],
                &options,
                &ImmediateAbort,
            )
            .expect_err("noise observes cancellation"),
            *health_check_with_options_and_abort_detailed(&options, &ImmediateAbort)
                .expect_err("the readiness probe observes cancellation"),
        ];
        for error in failures {
            assert_eq!(error.code, "aborted");
            assert_eq!(error.category, "cancellation");
            assert!(error.retryable);
        }
    }

    /// A transient with attached `.FFT` post-processing is refused on the
    /// direct route, because a direct request plans one analysis and each
    /// spectrum needs its own canonical identity.
    #[test]
    fn a_direct_transient_refuses_attached_fft_post_processing() {
        let source = "browser direct fft deck\n\
V1 in 0 SIN(0 1 1G)\n\
R1 in out 1k\n\
C1 out 0 1p\n\
.options fft\n\
.FFT v(out) np=16 format=norm\n\
.END\n";
        let error = *run_transient_document_detailed(source, 1.6e-8, 5.0e-11)
            .expect_err("attached FFT must be refused on the direct route");
        assert_eq!(error.code, "unsupported_deck_analysis");
        assert!(
            error.message.contains("runAuthoredDeckDocument"),
            "the refusal points at the route that does support it: {}",
            error.message
        );
    }
}
