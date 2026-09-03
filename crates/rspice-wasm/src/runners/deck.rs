//! Authored-deck execution over the canonical `DeckPlan` coordinate product.
//!
//! There is one execution route in this crate and this is it. Every planned
//! analysis identity the core assigns is dispatched here, every result becomes
//! a `rspice_core::execution::AnalysisResultDocument`, and every long call
//! receives the caller's abort source.
//!
//! A family this build cannot execute is refused by name, with the core API
//! that is missing spelled out, before any solver work starts. It is never
//! quietly turned into an operating point.

use rspice_core::analysis::{
    Distribution, FrequencyGridError, PacConfig, PssConfig, StbConfig, StbSweepType,
};
use rspice_core::engine::{
    CompressionConfig, HbOperatingPoint, PssOperatingPoint, TransientStartupMode,
};
use rspice_core::execution::result_document::FftChildReference;
use rspice_core::execution::{
    AnalysisInstanceId, AnalysisKind, AnalysisRequest, AnalysisResultDocument,
    AnalysisResultDocumentBuilder, DeckPlan, DeckPlanError, MaterializedRunError, ResultCoordinate,
    ResultDocumentError, ResultNamespaces, RunCoordinate, SignalUnit,
};
use rspice_core::netlist::{
    AnalysisCommand, DcSweepSpec, FftFormat, FreqVariation, MonteCarloDistribution,
    PeriodicSourceSelector, PeriodicSweep,
};
use rspice_core::{AbortSignal, Engine, Netlist, NoAbort, ResourceKind, ResourceLimits, Value};

use crate::DetailedWasmResult;
use crate::abort::{aborted_error, ensure_not_aborted};
use crate::errors::{WasmError, resource_limit_error};
use crate::hb_request::{HbRequest, hb_request_for_tones};
use crate::options::WasmExecutionOptions;
use crate::support::{engine_with_resource_limits, parse_netlist_detailed};

/// Everything one authored-deck run produced: the plan it executed, the
/// coordinates it materialized, and one core document per result.
#[derive(Debug)]
pub struct DeckExecution {
    pub plan: DeckPlan,
    pub coordinates: Vec<RunCoordinate>,
    pub results: Vec<AnalysisResultDocument>,
}

/// Execute every planned analysis in an authored analog deck over its
/// canonical DATA/STEP/TEMP coordinate product.
pub fn run_authored_deck_document_with_options_and_abort_detailed(
    source: &str,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<DeckExecution> {
    ensure_not_aborted(external_abort)?;
    let resource_limits = options.resource_limits.to_core();
    let compression = options.compression_config()?;
    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let plan = DeckPlan::from_netlist_with_abort(&netlist, &resource_limits, external_abort)
        .map_err(deck_plan_wasm_error)?;

    // Refuse every family this build cannot execute before any solve starts,
    // naming the exact authored card and its canonical instance identity.
    for (command, id) in plan.authored_analyses(&netlist) {
        if let Some(reason) = unroutable_reason(command) {
            let instance = id.map(|id| format!(" ({id})")).unwrap_or_default();
            return Err(unsupported_deck_analysis(format!(
                "authored {} card{instance} cannot be executed by the browser API: {reason}",
                card_spelling(command),
            )));
        }
    }

    let planning_engine = engine_with_resource_limits(resource_limits)?;
    // `.ALTER` and every unknown axis kind are refused by the core
    // materializer's own preparation, which owns that contract.
    let materializer = planning_engine
        .prepare_deck_plan_materializer_with_abort(&netlist, &plan, external_abort)
        .map_err(materialized_run_wasm_error)?;

    let mut coordinates = Vec::new();
    coordinates
        .try_reserve_exact(materializer.len())
        .map_err(|_| allocation_error("run coordinates"))?;
    let mut results = Vec::new();
    let result_capacity = materializer
        .len()
        .checked_mul(plan.analyses().len())
        .ok_or_else(|| allocation_error("coordinate-local analysis results"))?;
    results
        .try_reserve(result_capacity)
        .map_err(|_| allocation_error("coordinate-local analysis results"))?;
    let mut retained_values = 0usize;

    for run_index in 0..materializer.len() {
        ensure_not_aborted(external_abort)?;
        let materialized = materializer
            .materialize_run_with_abort(run_index, external_abort)
            .map_err(materialized_run_wasm_error)?;
        let (coordinate, coordinate_netlist, topology, analyses) = materialized.into_parts();
        if analyses.len() != plan.analyses().len() {
            return Err(document_error(format!(
                "coordinate {} materialized {} analyses; expected {}",
                coordinate.ordinal(),
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
        let result_coordinate = ResultCoordinate::from_run_coordinate(&coordinate);
        let mut periodic = PeriodicOperatingPoints::default();
        let mut post_process_ordinal = 0usize;

        for analysis in analyses {
            ensure_not_aborted(external_abort)?;
            let analysis_id = analysis.id();
            let name_the_failure = |error: Box<WasmError>| {
                Box::new(error.in_execution_context(
                    Some(analysis_id.tag()),
                    Some(coordinate.stable_id().to_string()),
                ))
            };
            let upstream = plan
                .analyses()
                .iter()
                .find(|planned| planned.id() == analysis_id)
                .and_then(|planned| planned.request().upstream());
            let produced = execute_analysis(
                &coordinate_engine,
                &coordinate_netlist,
                analysis.command(),
                analysis_id,
                upstream,
                &mut periodic,
                &mut post_process_ordinal,
                compression.as_ref(),
                resource_limits,
                external_abort,
            )
            .map_err(name_the_failure)?;

            let namespaces = ResultNamespaces {
                output: analysis.output_namespace().components().join("/"),
                checkpoint: analysis.checkpoint_namespace().components().join("/"),
            };
            results
                .try_reserve(produced.len())
                .map_err(|_| allocation_error("coordinate-local analysis results"))?;
            for builder in produced {
                let document = builder
                    .coordinate(result_coordinate.clone())
                    .topology_fingerprint(topology)
                    .namespaces(namespaces.clone())
                    .build_with_abort(external_abort)
                    .map_err(|error| name_the_failure(document_projection_error(error)))?;
                retained_values = retained_values
                    .checked_add(document.total_value_count())
                    .ok_or_else(|| allocation_error("deck retained-value accounting"))?;
                if retained_values > resource_limits.max_result_values {
                    return Err(resource_limit_error(
                        ResourceKind::ResultValues,
                        retained_values,
                        resource_limits.max_result_values,
                    ));
                }
                results.push(document);
            }
        }
        coordinates.push(coordinate);
    }

    ensure_not_aborted(external_abort)?;
    Ok(DeckExecution {
        plan,
        coordinates,
        results,
    })
}

/// Execute an authored deck under the browser defaults without cancellation.
pub fn run_authored_deck_document_detailed(source: &str) -> DetailedWasmResult<DeckExecution> {
    run_authored_deck_document_with_options_and_abort_detailed(
        source,
        &WasmExecutionOptions::default(),
        &NoAbort,
    )
}

/// Periodic large-signal operating points retained within one coordinate.
///
/// `.PAC` and `.ENVELOPE` linearize around an upstream `.PSS` or `.HB`. Core
/// exposes the operating point alongside that upstream analysis's own result,
/// so the deck route retains it rather than solving the large-signal problem a
/// second time.
#[derive(Default)]
struct PeriodicOperatingPoints {
    pss: Vec<(AnalysisInstanceId, PssOperatingPoint)>,
    hb: Vec<(AnalysisInstanceId, HbOperatingPoint)>,
    hb_requests: Vec<(AnalysisInstanceId, HbRequest)>,
}

impl PeriodicOperatingPoints {
    fn pss(&self, id: AnalysisInstanceId) -> Option<&PssOperatingPoint> {
        self.pss
            .iter()
            .find(|(candidate, _)| *candidate == id)
            .map(|(_, point)| point)
    }

    fn hb(&self, id: AnalysisInstanceId) -> Option<&HbOperatingPoint> {
        self.hb
            .iter()
            .find(|(candidate, _)| *candidate == id)
            .map(|(_, point)| point)
    }

    fn hb_request(&self, id: AnalysisInstanceId) -> Option<&HbRequest> {
        self.hb_requests
            .iter()
            .find(|(candidate, _)| *candidate == id)
            .map(|(_, request)| request)
    }
}

/// Run one planned analysis and project everything it produced.
///
/// A transient produces its own document plus one per attached `.FFT`
/// spectrum, so this returns a list rather than a single builder.
#[allow(clippy::too_many_arguments)]
fn execute_analysis(
    engine: &Engine,
    netlist: &Netlist,
    command: Option<&AnalysisCommand>,
    id: AnalysisInstanceId,
    upstream: Option<AnalysisInstanceId>,
    periodic: &mut PeriodicOperatingPoints,
    post_process_ordinal: &mut usize,
    compression: Option<&CompressionConfig>,
    resource_limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> DetailedWasmResult<Vec<AnalysisResultDocumentBuilder>> {
    let Some(command) = command else {
        if id.kind() != AnalysisKind::ImplicitOp {
            return Err(document_error(format!(
                "canonical materializer omitted the authored command for {id}"
            )));
        }
        return Ok(vec![operating_point_builder(engine, netlist, id, abort)?]);
    };

    match command {
        AnalysisCommand::Op => Ok(vec![operating_point_builder(engine, netlist, id, abort)?]),

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
                .map_err(simulation_error)?;
            ensure_not_aborted(abort)?;
            Ok(vec![
                AnalysisResultDocument::from_dc_sweep(id, source, dc_sweep_unit(source), &points)
                    .map_err(document_projection_error)?,
            ])
        }

        AnalysisCommand::Ac {
            variation,
            points,
            start_freq,
            stop_freq,
        } => {
            let frequencies = frequency_grid(
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                false,
                resource_limits,
                abort,
            )?;
            ac_builder(engine, netlist, id, &frequencies, abort)
        }

        AnalysisCommand::AcData { table_name } => {
            // A table-driven `.AC` may override circuit parameters per row, so
            // the core runner owns the whole sweep rather than only its grid.
            let (_, points) = engine
                .run_ac_data_with_abort(netlist, table_name, abort)
                .map_err(simulation_error)?;
            ensure_not_aborted(abort)?;
            Ok(vec![
                AnalysisResultDocument::from_ac(id, &points).map_err(document_projection_error)?,
            ])
        }

        AnalysisCommand::Tran {
            step,
            stop,
            start,
            max_step,
            uic,
        } => transient_builders(
            engine,
            netlist,
            id,
            *step,
            *stop,
            *start,
            *max_step,
            *uic,
            post_process_ordinal,
            compression.cloned(),
            resource_limits,
            abort,
        ),

        AnalysisCommand::Noise {
            output_node,
            reference_node,
            input_source,
            variation,
            points,
            start_freq,
            stop_freq,
        } => {
            let frequencies = frequency_grid(
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
                .map_err(simulation_error)?;
            ensure_not_aborted(abort)?;
            Ok(vec![
                AnalysisResultDocument::from_noise(id, &points)
                    .map_err(document_projection_error)?,
            ])
        }

        AnalysisCommand::NoiseData {
            output_node,
            reference_node,
            input_source,
            table_name,
        } => {
            let (_, points) = engine
                .run_noise_data_named_with_input_source_and_abort(
                    netlist,
                    output_node,
                    reference_node.as_deref(),
                    input_source,
                    table_name,
                    engine.config().temperature,
                    abort,
                )
                .map_err(simulation_error)?;
            ensure_not_aborted(abort)?;
            Ok(vec![
                AnalysisResultDocument::from_noise(id, &points)
                    .map_err(document_projection_error)?,
            ])
        }

        AnalysisCommand::Stb {
            variation,
            points,
            start_freq,
            stop_freq,
            probe,
        } => {
            let config = StbConfig::new()
                .with_sweep(*start_freq, *stop_freq, *points)
                .with_sweep_type(stb_sweep_type(*variation))
                .with_probe(probe)
                .with_nyquist(true);
            config.validate().map_err(|message| {
                Box::new(WasmError::invalid_argument(format!(
                    "invalid .STB card: {message}"
                )))
            })?;
            let result = engine
                .run_stb_with_abort(netlist, config, abort)
                .map_err(simulation_error)?;
            ensure_not_aborted(abort)?;
            Ok(vec![
                AnalysisResultDocument::from_stability(id, &result.result)
                    .map_err(document_projection_error)?,
            ])
        }

        AnalysisCommand::Tf {
            output_node,
            reference_node,
            output_is_current,
            input_source,
        } => {
            let result = engine
                .run_transfer_function_with_abort(
                    netlist,
                    output_node,
                    reference_node.as_deref(),
                    *output_is_current,
                    input_source,
                    abort,
                )
                .map_err(simulation_error)?;
            ensure_not_aborted(abort)?;
            Ok(vec![
                AnalysisResultDocument::from_transfer_function(id, &result)
                    .map_err(document_projection_error)?,
            ])
        }

        AnalysisCommand::Disto {
            variation,
            points,
            start_freq,
            stop_freq,
            f2_over_f1,
        } => {
            let frequencies = frequency_grid(
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                true,
                resource_limits,
                abort,
            )?;
            let result = engine
                .run_distortion_with_abort(netlist, &frequencies, *f2_over_f1, abort)
                .map_err(simulation_error)?;
            ensure_not_aborted(abort)?;
            Ok(vec![
                AnalysisResultDocument::from_distortion(id, &result)
                    .map_err(document_projection_error)?,
            ])
        }

        AnalysisCommand::MonteCarlo(request) => {
            let distribution =
                monte_carlo_distribution(request.distribution, request.relative_spread);
            let filter = (!request.params.is_empty()).then_some(request.params.as_slice());
            let result = engine
                .run_monte_carlo_with_options_and_abort(
                    netlist,
                    request.runs,
                    request.seed.unwrap_or(0),
                    distribution,
                    filter,
                    abort,
                )
                .map_err(simulation_error)?;
            ensure_not_aborted(abort)?;
            Ok(vec![
                AnalysisResultDocument::from_monte_carlo(id, &result)
                    .map_err(document_projection_error)?,
            ])
        }

        AnalysisCommand::Pss(card) => {
            let config = PssConfig::from(card.as_ref());
            let operating_point = engine
                .run_pss_operating_point_with_abort(netlist, config, abort)
                .map_err(simulation_error)?;
            ensure_not_aborted(abort)?;
            let builder = AnalysisResultDocument::from_pss(id, &operating_point.analysis().result)
                .map_err(document_projection_error)?;
            periodic
                .pss
                .try_reserve(1)
                .map_err(|_| allocation_error("retained PSS operating points"))?;
            periodic.pss.push((id, operating_point));
            Ok(vec![builder])
        }

        AnalysisCommand::Hb { frequencies } => {
            let request = hb_request_for_tones(netlist, frequencies)?;
            let result = engine
                .run_hb_with_abort(netlist, request.config.clone(), abort)
                .map_err(simulation_error)?;
            ensure_not_aborted(abort)?;
            let builder = AnalysisResultDocument::from_harmonic_balance(id, &result.result)
                .map_err(document_projection_error)?;
            periodic
                .hb
                .try_reserve(1)
                .map_err(|_| allocation_error("retained HB operating points"))?;
            periodic
                .hb_requests
                .try_reserve(1)
                .map_err(|_| allocation_error("retained HB configurations"))?;
            periodic.hb.push((id, result.operating_point));
            periodic.hb_requests.push((id, request));
            Ok(vec![builder])
        }

        AnalysisCommand::Pac(card) => {
            let mut config = PacConfig::from(card.as_ref());
            preflight_periodic_sweep(&card.sweep, resource_limits)?;
            let upstream = upstream_or_refuse(upstream, ".PAC", card.source)?;
            let result = if let Some(operating_point) = periodic.pss(upstream) {
                engine
                    .run_pac_from_pss_with_abort(netlist, config, operating_point, abort)
                    .map_err(simulation_error)?
            } else if let Some(operating_point) = periodic.hb(upstream) {
                config.fundamental_freq = 0.0;
                engine
                    .run_pac_from_hb_with_abort(netlist, config, operating_point, abort)
                    .map_err(simulation_error)?
            } else {
                return Err(unsupported_deck_analysis(format!(
                    "authored .PAC card names upstream {upstream}, which produced no retained periodic operating point"
                )));
            };
            ensure_not_aborted(abort)?;
            Ok(vec![
                AnalysisResultDocument::from_pac(id, &result.result)
                    .map_err(document_projection_error)?
                    .parent_analysis(upstream),
            ])
        }

        AnalysisCommand::Envelope(card) => {
            let upstream = upstream_or_refuse(upstream, ".ENVELOPE", PeriodicSourceSelector::Hb)?;
            let Some(request) = periodic.hb_request(upstream) else {
                return Err(unsupported_deck_analysis(format!(
                    "authored .ENVELOPE card names upstream {upstream}, which is not a retained harmonic-balance carrier"
                )));
            };
            let result = engine
                .run_envelope_with_abort(
                    netlist,
                    request.config.clone(),
                    &card.frozen_sources,
                    card.duration,
                    card.max_step,
                    abort,
                )
                .map_err(simulation_error)?;
            ensure_not_aborted(abort)?;
            Ok(vec![
                AnalysisResultDocument::from_envelope(id, &result)
                    .map_err(document_projection_error)?
                    .parent_analysis(upstream),
            ])
        }

        // Every remaining card is refused before execution starts, so
        // reaching here means the preflight and this dispatch disagree.
        other => Err(document_error(format!(
            "authored {} card reached execution without a route",
            card_spelling(other)
        ))),
    }
}

fn operating_point_builder(
    engine: &Engine,
    netlist: &Netlist,
    id: AnalysisInstanceId,
    abort: &dyn AbortSignal,
) -> DetailedWasmResult<AnalysisResultDocumentBuilder> {
    let (result, report) = engine
        .run_dc_op_with_report_and_abort(netlist, abort)
        .map_err(simulation_error)?;
    ensure_not_aborted(abort)?;
    AnalysisResultDocument::from_operating_point(id, &result, Some(&report))
        .map_err(document_projection_error)
}

fn ac_builder(
    engine: &Engine,
    netlist: &Netlist,
    id: AnalysisInstanceId,
    frequencies: &[Value],
    abort: &dyn AbortSignal,
) -> DetailedWasmResult<Vec<AnalysisResultDocumentBuilder>> {
    let points = engine
        .run_ac_with_abort(netlist, frequencies, abort)
        .map_err(simulation_error)?;
    ensure_not_aborted(abort)?;
    Ok(vec![
        AnalysisResultDocument::from_ac(id, &points).map_err(document_projection_error)?,
    ])
}

/// Run one authored `.TRAN` and project it with every attached `.FFT`.
#[allow(clippy::too_many_arguments)]
fn transient_builders(
    engine: &Engine,
    netlist: &Netlist,
    id: AnalysisInstanceId,
    step: Value,
    stop: Value,
    start: Option<Value>,
    max_step: Option<Value>,
    uic: bool,
    post_process_ordinal: &mut usize,
    compression: Option<CompressionConfig>,
    resource_limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> DetailedWasmResult<Vec<AnalysisResultDocumentBuilder>> {
    let ceiling =
        rspice_core::execution::resolve_transient_maximum_step(step, stop, start, max_step)
            .map_err(|error| Box::new(WasmError::invalid_argument(error.to_string())))?;
    preflight_transient_points(stop, ceiling, resource_limits)?;
    let startup = TransientStartupMode::from_uic(uic);
    // Compression is a browser transfer policy, not a deck semantic: the
    // solver and the authored output projection are identical either way and
    // only the published analog history is decimated. The certificate travels
    // with the result so a reader always knows which grid it is looking at.
    let (result, compression_report) = match compression {
        Some(config) => {
            let compressed = engine
                .run_tran_compressed_with_startup_mode_and_abort(
                    netlist, stop, ceiling, startup, config, abort,
                )
                .map_err(simulation_error)?;
            let report = compressed.compression_report.clone();
            let expanded = compressed.try_into_transient().map_err(document_error)?;
            (expanded, Some(report))
        }
        None => (
            engine
                .run_tran_with_startup_mode_and_abort(netlist, stop, ceiling, startup, abort)
                .map_err(simulation_error)?,
            None,
        ),
    };
    ensure_not_aborted(abort)?;

    let mut children = Vec::new();
    children
        .try_reserve_exact(result.fft_results.len())
        .map_err(|_| allocation_error("attached FFT child references"))?;
    let mut spectra = Vec::new();
    spectra
        .try_reserve_exact(result.fft_results.len())
        .map_err(|_| allocation_error("attached FFT spectra"))?;
    // `.FFT` spectra are attached to a transient rather than planned, so the
    // canonical `fft-NNN` identities are minted from a one-family plan instead
    // of being invented here.
    let fft_ids = post_process_ids(
        AnalysisKind::Fft,
        *post_process_ordinal,
        result.fft_results.len(),
    )?;
    *post_process_ordinal = post_process_ordinal
        .checked_add(result.fft_results.len())
        .ok_or_else(|| allocation_error("attached FFT identities"))?;
    for (fft_id, spectrum) in fft_ids.into_iter().zip(&result.fft_results) {
        ensure_not_aborted(abort)?;
        children.push(FftChildReference {
            analysis: fft_id,
            output_name: spectrum.output_name.clone(),
        });
        spectra.push(
            AnalysisResultDocument::from_transient_fft(
                fft_id,
                id,
                fft_output_unit(spectrum.physical_type, spectrum.format)?,
                spectrum,
            )
            .map_err(document_projection_error)?,
        );
    }

    let mut builders = Vec::new();
    builders
        .try_reserve_exact(spectra.len().saturating_add(1))
        .map_err(|_| allocation_error("transient result documents"))?;
    builders.push(
        AnalysisResultDocument::from_transient(id, &result, compression_report.as_ref(), children)
            .map_err(document_projection_error)?,
    );
    builders.extend(spectra);
    Ok(builders)
}

/// Mint canonical instance identities for a post-process family the authored
/// plan does not carry.
///
/// `AnalysisInstanceId` is deliberately not constructible outside core, so the
/// identities come from a one-family `DeckPlan` rather than from a widened
/// constructor.
fn post_process_ids(
    kind: AnalysisKind,
    already_minted: usize,
    count: usize,
) -> DetailedWasmResult<Vec<AnalysisInstanceId>> {
    if count == 0 {
        return Ok(Vec::new());
    }
    let total = already_minted
        .checked_add(count)
        .ok_or_else(|| allocation_error("post-process analysis identities"))?;
    let mut requests = Vec::new();
    requests
        .try_reserve_exact(total)
        .map_err(|_| allocation_error("post-process analysis identities"))?;
    for _ in 0..total {
        requests.push(AnalysisRequest::new(kind));
    }
    let plan = DeckPlan::new(Vec::new(), requests).map_err(deck_plan_wasm_error)?;
    let mut ids = Vec::new();
    ids.try_reserve_exact(count)
        .map_err(|_| allocation_error("post-process analysis identities"))?;
    for planned in plan.analyses().iter().skip(already_minted) {
        ids.push(planned.id());
    }
    Ok(ids)
}

/// Which authored analyses this build cannot execute, and why.
///
/// Each reason names the `rspice-core` entry point that is missing. Nothing
/// here is a policy choice: a family appears only because running it from this
/// surface would mean deciding its semantics a second time.
fn unroutable_reason(command: &AnalysisCommand) -> Option<&'static str> {
    match command {
        AnalysisCommand::Dc {
            sweep2: Some(_), ..
        } => Some(
            "the shared result document declares exactly one sweep variable, so the outer source of a nested .DC sweep would be lost",
        ),
        AnalysisCommand::Sp { .. } => Some(
            "rspice-core exposes S-parameter port collection and matrix extraction but no Engine runner that returns an SParameterResult",
        ),
        AnalysisCommand::Pnoise(_) => Some(
            "rspice-core has no Engine runner that returns the analysis::pnoise::PnoiseResult the shared result document is built from",
        ),
        AnalysisCommand::PoleZero { .. } | AnalysisCommand::Sensitivity { .. } => Some(
            "rspice-core takes node indices here and exposes no authored-name to node-index resolver for an unsolved netlist",
        ),
        AnalysisCommand::Four { .. } => Some(
            "rspice-core exposes no resolver from a .FOUR output specification to a column of a TransientResult",
        ),
        // Routed families and the meta-analysis cards the materializer owns.
        // This match is deliberately exhaustive: a new authored card must be
        // declared routable or unroutable here before it can compile.
        AnalysisCommand::Op
        | AnalysisCommand::Dc { sweep2: None, .. }
        | AnalysisCommand::Ac { .. }
        | AnalysisCommand::AcData { .. }
        | AnalysisCommand::Hb { .. }
        | AnalysisCommand::Stb { .. }
        | AnalysisCommand::Disto { .. }
        | AnalysisCommand::Tran { .. }
        | AnalysisCommand::Noise { .. }
        | AnalysisCommand::NoiseData { .. }
        | AnalysisCommand::Tf { .. }
        | AnalysisCommand::MonteCarlo(_)
        | AnalysisCommand::Step(_)
        | AnalysisCommand::Temp { .. }
        | AnalysisCommand::Pss(_)
        | AnalysisCommand::Pac(_)
        | AnalysisCommand::Envelope(_) => None,
    }
}

/// Dot-command spelling of one authored card.
fn card_spelling(command: &AnalysisCommand) -> &'static str {
    match command {
        AnalysisCommand::Op => ".OP",
        AnalysisCommand::Dc { .. } => ".DC",
        AnalysisCommand::Ac { .. } | AnalysisCommand::AcData { .. } => ".AC",
        AnalysisCommand::Hb { .. } => ".HB",
        AnalysisCommand::Sp { .. } => ".SP",
        AnalysisCommand::Stb { .. } => ".STB",
        AnalysisCommand::Disto { .. } => ".DISTO",
        AnalysisCommand::Tran { .. } => ".TRAN",
        AnalysisCommand::Noise { .. } | AnalysisCommand::NoiseData { .. } => ".NOISE",
        AnalysisCommand::PoleZero { .. } => ".PZ",
        AnalysisCommand::Sensitivity { .. } => ".SENS",
        AnalysisCommand::Tf { .. } => ".TF",
        AnalysisCommand::Four { .. } => ".FOUR",
        AnalysisCommand::MonteCarlo(_) => ".MC",
        AnalysisCommand::Step(_) => ".STEP",
        AnalysisCommand::Temp { .. } => ".TEMP",
        AnalysisCommand::Pss(_) => ".PSS",
        AnalysisCommand::Pac(_) => ".PAC",
        AnalysisCommand::Pnoise(_) => ".PNOISE",
        AnalysisCommand::Envelope(_) => ".ENVELOPE",
    }
}

fn upstream_or_refuse(
    upstream: Option<AnalysisInstanceId>,
    card: &'static str,
    selector: PeriodicSourceSelector,
) -> DetailedWasmResult<AnalysisInstanceId> {
    upstream.ok_or_else(|| {
        unsupported_deck_analysis(format!(
            "authored {card} card selects a {selector:?} periodic source, but the canonical plan bound it to no upstream analysis"
        ))
    })
}

/// Unit of one `.FFT` spectrum's coefficients.
///
/// A normalized spectrum is dimensionless by construction. An unnormalized
/// spectrum inherits the source quantity's unit, and a braced parameter
/// expression has none: the shared document has no representation for an
/// unknown unit, so that combination is refused rather than given an invented
/// one.
fn fft_output_unit(physical_type: &str, format: FftFormat) -> DetailedWasmResult<SignalUnit> {
    if matches!(format, FftFormat::Normalized) {
        return Ok(SignalUnit::Dimensionless);
    }
    match physical_type {
        "voltage" => Ok(SignalUnit::Volt),
        "current" => Ok(SignalUnit::Ampere),
        "parameter" => Err(unsupported_deck_analysis(
            "an unnormalized .FFT of a parameter expression has no declared unit; request FORMAT=NORM or probe a voltage or current"
                .to_owned(),
        )),
        other => Err(document_error(format!(
            "core reported unknown FFT physical quantity '{other}'"
        ))),
    }
}

/// Unit of a `.DC` sweep axis.
///
/// A sweep over an independent source carries that source's unit; `V`-prefixed
/// instances are voltage sources and `I`-prefixed are current sources, which
/// is the same first-letter element contract the parser itself uses. Anything
/// else is a swept parameter with no intrinsic unit.
fn dc_sweep_unit(source: &str) -> SignalUnit {
    match source
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

const fn stb_sweep_type(variation: FreqVariation) -> StbSweepType {
    match variation {
        FreqVariation::Lin => StbSweepType::Linear,
        FreqVariation::Dec => StbSweepType::Decade,
        FreqVariation::Oct => StbSweepType::Octave,
    }
}

const fn monte_carlo_distribution(
    distribution: MonteCarloDistribution,
    relative_spread: Value,
) -> Distribution {
    match distribution {
        MonteCarloDistribution::Gaussian => Distribution::Gaussian {
            sigma: relative_spread,
        },
        MonteCarloDistribution::Uniform => Distribution::Uniform {
            tolerance: relative_spread,
        },
        MonteCarloDistribution::WorstCase => Distribution::WorstCase {
            tolerance: relative_spread,
        },
    }
}

fn preflight_periodic_sweep(
    sweep: &PeriodicSweep,
    resource_limits: ResourceLimits,
) -> DetailedWasmResult<()> {
    if sweep.points > resource_limits.max_analysis_points {
        return Err(resource_limit_error(
            ResourceKind::AnalysisPoints,
            sweep.points,
            resource_limits.max_analysis_points,
        ));
    }
    Ok(())
}

/// Reject a transient whose requested grid cannot fit the analysis budget
/// before the solver allocates anything.
pub(crate) fn preflight_transient_points(
    stop: Value,
    max_step: Value,
    resource_limits: ResourceLimits,
) -> DetailedWasmResult<()> {
    if !stop.is_finite() || stop <= 0.0 {
        return Err(Box::new(WasmError::invalid_argument(format!(
            "Transient stop time must be positive and finite, got {stop}"
        ))));
    }
    if !max_step.is_finite() || max_step <= 0.0 {
        return Err(Box::new(WasmError::invalid_argument(format!(
            "Transient maximum step must be positive and finite, got {max_step}"
        ))));
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    // Both operands are checked finite and positive above, so the ceiling is a
    // non-negative count; saturation to usize::MAX still fails the budget.
    let estimated_points = (stop / max_step).ceil() as usize;
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

/// Build one authored frequency sweep through the shared core generator.
pub(crate) fn frequency_grid(
    variation: FreqVariation,
    points: usize,
    start: Value,
    stop: Value,
    strictly_positive: bool,
    resource_limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> DetailedWasmResult<Vec<Value>> {
    ensure_not_aborted(abort)?;
    if strictly_positive && (!start.is_finite() || start <= 0.0) {
        return Err(Box::new(WasmError::invalid_argument(format!(
            "authored start frequency must be positive and finite, got {start}"
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
        FrequencyGridError::Aborted => aborted_error(),
        FrequencyGridError::LimitExceeded { requested, limit } => {
            resource_limit_error(ResourceKind::AnalysisPoints, requested, limit)
        }
        FrequencyGridError::Allocation { requested } => Box::new(WasmError::new(
            format!("could not allocate the {requested}-point authored frequency grid"),
            "result_allocation_failed",
            "analysis_setup",
        )),
        other => Box::new(WasmError::invalid_argument(format!(
            "invalid authored frequency sweep {variation:?} {points} from {start} to {stop} Hz: {other}"
        ))),
    })
}

pub(crate) fn simulation_error(error: rspice_core::engine::SimulationError) -> Box<WasmError> {
    Box::new(WasmError::from_simulation_error(error))
}

pub(crate) fn unsupported_deck_analysis(message: String) -> Box<WasmError> {
    Box::new(WasmError::new(
        message,
        "unsupported_deck_analysis",
        "unsupported_feature",
    ))
}

pub(crate) fn document_error(message: String) -> Box<WasmError> {
    Box::new(WasmError::new(
        message,
        "invalid_result_document",
        "result_validation",
    ))
}

pub(crate) fn document_projection_error(error: ResultDocumentError) -> Box<WasmError> {
    match error {
        ResultDocumentError::Aborted => aborted_error(),
        other => document_error(other.to_string()),
    }
}

fn allocation_error(object: &'static str) -> Box<WasmError> {
    Box::new(WasmError::new(
        format!("could not allocate {object}"),
        "result_allocation_failed",
        "result_projection",
    ))
}

pub(crate) fn deck_plan_wasm_error(error: DeckPlanError) -> Box<WasmError> {
    match error {
        DeckPlanError::Aborted => aborted_error(),
        DeckPlanError::ResourceLimit(error) => {
            Box::new(WasmError::resource_limit(error.to_string(), error))
        }
        other => Box::new(WasmError::new(
            other.to_string(),
            "invalid_deck_plan",
            "input_validation",
        )),
    }
}

pub(crate) fn materialized_run_wasm_error(error: MaterializedRunError) -> Box<WasmError> {
    match error {
        MaterializedRunError::Aborted => aborted_error(),
        MaterializedRunError::DeckPlan(error) => deck_plan_wasm_error(error),
        MaterializedRunError::Simulation(error) => simulation_error(error),
        MaterializedRunError::AlterUnsupported => Box::new(WasmError::new(
            error.to_string(),
            "unsupported_deck_axis",
            "unsupported_feature",
        )),
        other => Box::new(WasmError::new(
            other.to_string(),
            "deck_materialization_failed",
            "execution",
        )),
    }
}

#[cfg(test)]
mod tests {
    use rspice_core::execution::{
        AnalysisResultKind, MappingStatus, NonUiSurface, analysis_result_capability,
    };

    use super::*;
    use crate::handles::WasmResultHandle;

    /// A linear RC driven by one sinusoidal source with an AC excitation.
    const LINEAR: &str = "browser capability deck\n\
V1 in 0 SIN(0 0.1 1G) AC 1\n\
R1 in out 1k\n\
C1 out 0 1p\n";

    /// The same network with the first-tone distortion excitation `.DISTO`
    /// requires.
    const DISTORTION: &str = "browser distortion deck\n\
V1 in 0 SIN(0 0.1 1G) AC 1 DISTOF1 1\n\
R1 in out 1k\n\
C1 out 0 1p\n";

    /// The same network with a series source resistance, so the `.TF` input
    /// impedance is finite. The shared result document refuses a non-finite
    /// scalar rather than encoding one, which is the correct contract: an
    /// ideal source has no finite input impedance to report.
    const TRANSFER: &str = "browser transfer-function deck\n\
V1 in 0 1 AC 1\n\
RS in mid 50\n\
R1 mid out 1k\n\
R2 out 0 10k\n";

    /// A three-pole feedback loop with a zero-volt Tian probe in series.
    ///
    /// Three poles are the point: the shared result document refuses a
    /// non-finite scalar, and a loop that never reaches -180 degrees has no
    /// finite gain margin to report. The deck therefore has a real crossover.
    const LOOP_DECK: &str = "browser stability deck\n\
V1 in 0 AC 1\n\
R1 in a 1k\n\
E1 b 0 a 0 100\n\
VP b c 0\n\
R2 c d 1k\n\
C2 d 0 160p\n\
R3 d e 1k\n\
C3 e 0 160p\n\
R4 e a 1k\n\
C4 a 0 160p\n";

    /// How the browser surface is expected to answer for one result family.
    enum Expectation {
        /// The deck executes and publishes a document of this family.
        Routed { deck: String },
        /// The deck is refused by name, quoting the missing core API.
        Refused { deck: String, names: &'static str },
    }

    fn deck(circuit: &str, cards: &str) -> String {
        format!("{circuit}{cards}.END\n")
    }

    /// The browser surface's declared answer for every core result family.
    ///
    /// This match is exhaustive on purpose: a new `AnalysisResultKind` cannot
    /// compile until this surface says, in one place, whether it executes the
    /// family or refuses it and why.
    fn expectation(kind: AnalysisResultKind) -> Expectation {
        match kind {
            AnalysisResultKind::OperatingPoint => Expectation::Routed {
                deck: deck(LINEAR, ".OP\n"),
            },
            AnalysisResultKind::DcSweep => Expectation::Routed {
                deck: deck(LINEAR, ".DC V1 0 1 0.5\n"),
            },
            AnalysisResultKind::Ac => Expectation::Routed {
                deck: deck(LINEAR, ".AC DEC 3 1k 100k\n"),
            },
            AnalysisResultKind::Transient => Expectation::Routed {
                deck: deck(LINEAR, ".TRAN 1n 20n\n"),
            },
            AnalysisResultKind::Noise => Expectation::Routed {
                deck: deck(LINEAR, ".NOISE V(out) V1 DEC 3 1k 100k\n"),
            },
            AnalysisResultKind::SParameters | AnalysisResultKind::PortNoise => {
                Expectation::Refused {
                    deck: deck(LINEAR, ".SP DEC 3 1k 100k\n"),
                    names: "SParameterResult",
                }
            }
            AnalysisResultKind::Distortion => Expectation::Routed {
                deck: deck(DISTORTION, ".DISTO DEC 3 1k 10k\n"),
            },
            AnalysisResultKind::TransferFunction => Expectation::Routed {
                deck: deck(TRANSFER, ".TF V(out) V1\n"),
            },
            AnalysisResultKind::Stability => Expectation::Routed {
                deck: deck(LOOP_DECK, ".STB DEC 8 1k 100meg PROBE=VP\n"),
            },
            AnalysisResultKind::Sensitivity => Expectation::Refused {
                deck: deck(LINEAR, ".SENS V(out)\n"),
                names: "node-index resolver",
            },
            AnalysisResultKind::PoleZero => Expectation::Refused {
                deck: deck(LINEAR, ".PZ in 0 out 0 VOL PZ\n"),
                names: "node-index resolver",
            },
            AnalysisResultKind::Fourier => Expectation::Refused {
                deck: deck(LINEAR, ".TRAN 1n 20n\n.FOUR 1G V(out)\n"),
                names: ".FOUR output specification",
            },
            AnalysisResultKind::Fft => Expectation::Routed {
                deck: deck(
                    LINEAR,
                    ".options fft\n.TRAN 0.05n 16n\n.FFT v(out) np=16 format=norm\n",
                ),
            },
            AnalysisResultKind::MonteCarlo => Expectation::Routed {
                deck: deck(LINEAR, ".MC 2 SEED 7\n"),
            },
            AnalysisResultKind::Pss => Expectation::Routed {
                deck: deck(LINEAR, ".PSS FUND=1G HARMS=3 POINTS=32 TSTABPERIODS=2\n"),
            },
            AnalysisResultKind::Pac => Expectation::Routed {
                deck: deck(
                    LINEAR,
                    ".PSS FUND=1G HARMS=3 POINTS=32 TSTABPERIODS=2\n\
.PAC LIN 2 1meg 10meg INPUT=V1 OUT=V(out)\n",
                ),
            },
            AnalysisResultKind::PNoise => Expectation::Refused {
                deck: deck(
                    LINEAR,
                    ".PSS FUND=1G HARMS=3 POINTS=32 TSTABPERIODS=2\n\
.PNOISE DEC 3 1k 100k OUT=V(out)\n",
                ),
                names: "PnoiseResult",
            },
            AnalysisResultKind::HarmonicBalance => Expectation::Routed {
                deck: deck(LINEAR, ".HB 1G\n"),
            },
            AnalysisResultKind::Envelope => Expectation::Routed {
                deck: deck(LINEAR, ".HB 1G\n.ENVELOPE TSTOP=2n MAXSTEP=0.5n\n"),
            },
        }
    }

    /// Every core result family behaves the way the shared capability registry
    /// declares it does on the WASM surface, and every routed family survives
    /// the trip through the browser handle unchanged.
    ///
    /// The registry is the input, not a comment: a cell that says `Mapped`
    /// must publish a document of that family, and a cell that says
    /// `Unsupported` must refuse by name.
    #[test]
    fn every_result_family_matches_its_wasm_capability_declaration() {
        for kind in AnalysisResultKind::ALL {
            let declared = analysis_result_capability(kind).surface(NonUiSurface::Wasm);
            match expectation(kind) {
                Expectation::Refused { deck, names } => {
                    assert!(
                        matches!(declared.scalar, MappingStatus::Unsupported(_)),
                        "{kind:?} is refused by the browser API but the registry does not say so"
                    );
                    let error = *run_authored_deck_document_detailed(&deck)
                        .expect_err("an unroutable family must be refused");
                    assert_eq!(error.code, "unsupported_deck_analysis", "{kind:?}");
                    assert_eq!(error.category, "unsupported_feature", "{kind:?}");
                    assert!(
                        error.message.contains(names),
                        "{kind:?} refusal must name the missing core API: {}",
                        error.message
                    );
                }
                Expectation::Routed { deck } => {
                    assert!(
                        !matches!(declared.scalar, MappingStatus::Unsupported(_)),
                        "{kind:?} executes on the browser API but the registry calls it unsupported"
                    );
                    let execution =
                        run_authored_deck_document_detailed(&deck).unwrap_or_else(|error| {
                            panic!("{kind:?} deck must run: {}", error.message)
                        });
                    assert_document_round_trips(kind, execution);
                }
            }
        }
    }

    /// The handle republishes exactly what the core document holds.
    fn assert_document_round_trips(kind: AnalysisResultKind, execution: DeckExecution) {
        let index = execution
            .results
            .iter()
            .position(|document| document.result_kind() == kind)
            .unwrap_or_else(|| panic!("{kind:?} deck published no document of that family"));
        let expected = execution.results[index].clone();
        let handle = WasmResultHandle::new(
            &execution.plan,
            execution.coordinates,
            execution.results,
            crate::options::browser_resource_limits(),
        )
        .expect("the browser handle retains the executed plan");

        let metadata = handle
            .result_metadata_snapshot(index)
            .expect("result metadata projects");
        assert_eq!(metadata.result_kind, kind.tag());
        assert_eq!(metadata.analysis.id, expected.analysis().tag());
        assert_eq!(metadata.point_count, expected.point_count());
        assert_eq!(metadata.axes.len(), expected.axes().len());
        assert_eq!(metadata.signals.len(), expected.signals().len());
        assert_eq!(metadata.scalars.len(), expected.scalars().len());
        assert!(
            metadata.coordinate_id.is_some(),
            "{kind:?} result must name the run coordinate it was produced at"
        );
        assert!(
            metadata.topology_fingerprint.is_some(),
            "{kind:?} result must carry the solved topology identity"
        );
        for (projected, source) in metadata.axes.iter().zip(expected.axes()) {
            assert_eq!(projected.name, source.name(), "{kind:?} axis name");
            assert_eq!(projected.kind, source.kind(), "{kind:?} axis kind");
        }
        for (projected, source) in metadata.signals.iter().zip(expected.signals()) {
            assert_eq!(
                projected.canonical_name,
                source.descriptor().canonical_name(),
                "{kind:?} signal identity"
            );
            assert_eq!(
                projected.availability,
                source.availability(),
                "{kind:?} signal availability"
            );
            assert_eq!(
                projected.has_any_sample,
                source.has_any_sample(),
                "{kind:?} signal missingness"
            );
        }

        if expected.point_count() > 0 {
            let window = handle
                .window_snapshot(index, 0, expected.point_count())
                .expect("a full-length window fits the browser transfer budget");
            let reference = expected
                .window(0, expected.point_count())
                .expect("the core document produces the same window");
            assert_eq!(window, reference, "{kind:?} window");
        }

        let json = handle
            .result_json_snapshot(index, &NoAbort)
            .expect("the lossless export encodes");
        let decoded = AnalysisResultDocument::from_json(&json)
            .expect("the lossless export decodes as the same core document");
        assert_eq!(decoded, expected, "{kind:?} lossless round trip");
    }
}

#[cfg(test)]
mod axis_tests {
    use rspice_core::abort_signal::ImmediateAbort;
    use rspice_core::execution::AnalysisResultKind;

    use super::*;
    use crate::handles::WasmResultHandle;

    const STEPPED: &str = "browser stepped deck\n\
.PARAM RLOAD=1k\n\
V1 in 0 1\n\
R1 in out {RLOAD}\n\
R2 out 0 1k\n\
.STEP PARAM RLOAD 1k 3k 1k\n\
.OP\n\
.AC DEC 2 1k 10k\n\
.END\n";

    const TEMPERATURE: &str = "browser temperature deck\n\
V1 in 0 1\n\
R1 in out 1k\n\
R2 out 0 1k\n\
.TEMP 0 27\n\
.OP\n\
.END\n";

    fn execute(source: &str) -> DeckExecution {
        run_authored_deck_document_detailed(source)
            .unwrap_or_else(|error| panic!("deck must run: {}", error.message))
    }

    /// A `.STEP` axis produces one coordinate per value, and every planned
    /// analysis is executed at every coordinate with its own identity and a
    /// collision-free namespace.
    #[test]
    fn a_step_axis_produces_one_result_per_coordinate_and_analysis() {
        let execution = execute(STEPPED);
        assert_eq!(execution.coordinates.len(), 3);
        assert_eq!(execution.results.len(), 6);

        let mut namespaces = std::collections::BTreeSet::new();
        for document in &execution.results {
            let coordinate = document
                .coordinate()
                .expect("a stepped result names its coordinate");
            assert_eq!(coordinate.assignments().len(), 1);
            assert_eq!(coordinate.assignments()[0].name(), "param:rload");
            let namespace = document
                .namespaces()
                .expect("a deck result carries its artifact namespaces");
            assert!(
                namespaces.insert(namespace.output.clone()),
                "output namespaces must not collide: {}",
                namespace.output
            );
        }
        let kinds: Vec<_> = execution
            .results
            .iter()
            .map(|document| (document.result_kind(), document.analysis().tag()))
            .collect();
        assert_eq!(
            kinds
                .iter()
                .filter(|(kind, _)| *kind == AnalysisResultKind::OperatingPoint)
                .count(),
            3
        );
        assert_eq!(
            kinds
                .iter()
                .filter(|(kind, tag)| *kind == AnalysisResultKind::Ac && tag == "ac-001")
                .count(),
            3
        );
    }

    /// A `.TEMP` axis is a run axis like any other: it produces coordinates,
    /// not a nominal-temperature run with the temperature quietly dropped.
    #[test]
    fn a_temperature_axis_produces_one_coordinate_per_temperature() {
        let execution = execute(TEMPERATURE);
        assert_eq!(execution.coordinates.len(), 2);
        let temperatures: Vec<_> = execution
            .results
            .iter()
            .filter_map(|document| document.coordinate())
            .map(|coordinate| coordinate.assignments()[0].name().to_owned())
            .collect();
        assert_eq!(
            temperatures,
            vec!["temperature".to_owned(), "temperature".to_owned()]
        );
    }

    /// The handle's own metadata describes the plan that produced it.
    #[test]
    fn handle_metadata_publishes_the_plan_coordinates_and_result_summaries() {
        let execution = execute(STEPPED);
        let handle = WasmResultHandle::new(
            &execution.plan,
            execution.coordinates,
            execution.results,
            crate::options::browser_resource_limits(),
        )
        .expect("the handle retains the executed plan");
        let metadata = handle
            .metadata_snapshot()
            .expect("handle metadata projects");

        assert_eq!(metadata.axes.len(), 1);
        assert_eq!(metadata.axes[0].kind, "step");
        assert_eq!(metadata.axes[0].value_count, 3);
        assert_eq!(metadata.planned_analyses.len(), 2);
        assert_eq!(metadata.planned_analyses[0].id, "op-001");
        assert_eq!(metadata.planned_analyses[1].id, "ac-001");
        assert_eq!(metadata.coordinates.len(), 3);
        assert_eq!(metadata.results.len(), 6);
        assert_eq!(metadata.result_count, handle.result_count());
        assert_eq!(handle.coordinate_count(), 3);
    }

    /// Textual `.ALTER` is refused by the core materializer, and the browser
    /// route reports that refusal rather than scanning the source itself.
    #[test]
    fn textual_alter_is_refused_by_the_core_materializer() {
        let error = *run_authored_deck_document_detailed(
            "browser alter deck\nV1 in 0 1\nR1 in 0 1k\n.OP\n.ALTER\nR1 in 0 2k\n.END\n",
        )
        .expect_err("textual ALTER must be refused");
        assert_eq!(error.code, "unsupported_deck_axis");
        assert_eq!(error.category, "unsupported_feature");
        assert!(
            error.message.to_ascii_lowercase().contains("alter"),
            "the refusal names ALTER: {}",
            error.message
        );
    }

    /// A pre-set abort source cancels the deck route before it publishes.
    #[test]
    fn a_pre_set_abort_cancels_the_deck_route() {
        let error = *run_authored_deck_document_with_options_and_abort_detailed(
            STEPPED,
            &WasmExecutionOptions::default(),
            &ImmediateAbort,
        )
        .expect_err("a cancelled deck must not publish results");
        assert_eq!(error.code, "aborted");
        assert_eq!(error.category, "cancellation");
        assert!(error.retryable);
    }

    /// A failure inside one coordinate-local analysis names that coordinate
    /// and that analysis instance.
    #[test]
    fn a_coordinate_local_failure_names_its_analysis_and_coordinate() {
        let mut options = WasmExecutionOptions::default();
        options.resource_limits.max_analysis_points = 1;
        let error = *run_authored_deck_document_with_options_and_abort_detailed(
            STEPPED, &options, &NoAbort,
        )
        .expect_err("an over-budget AC grid must fail");
        assert_eq!(error.code, "resource_limit");
        assert_eq!(error.analysis_id.as_deref(), Some("ac-001"));
        assert!(
            error.coordinate_id.is_some(),
            "a stepped failure names the coordinate it happened at"
        );
    }

    /// Compression is a transfer policy: the same solve is published on a
    /// decimated grid with an explicit certificate, never silently.
    #[test]
    fn transient_compression_publishes_its_certificate() {
        let source = "browser compression deck\n\
V1 in 0 PULSE(0 1 0 1n 1n 20n 40n)\n\
R1 in out 1k\n\
C1 out 0 1p\n\
.TRAN 0.1n 40n\n\
.END\n";
        let plain = execute(source);
        let plain_document = &plain.results[0];
        assert!(
            matches!(
                plain_document.payload(),
                rspice_core::execution::ResultPayload::Tran(payload) if payload.compression.is_none()
            ),
            "an uncompressed run carries no compression certificate"
        );

        let mut options = WasmExecutionOptions::default();
        options.transient_compression = Some(crate::options::WasmCompressionOptions {
            absolute_tolerance: 1e-3,
            relative_tolerance: 1e-3,
            maximum_interval: 0.0,
            enabled: true,
        });
        let compressed =
            run_authored_deck_document_with_options_and_abort_detailed(source, &options, &NoAbort)
                .expect("a compressed deck runs");
        let report = match compressed.results[0].payload() {
            rspice_core::execution::ResultPayload::Tran(payload) => payload
                .compression
                .as_ref()
                .expect("a compressed run publishes its certificate"),
            other => panic!("expected a transient payload, got {other:?}"),
        };
        assert_eq!(report.input_points, plain_document.point_count());
        assert!(report.retained_points <= report.input_points);
        assert_eq!(compressed.results[0].point_count(), report.retained_points);
    }
}
