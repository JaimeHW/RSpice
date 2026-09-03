//! Analysis families this executor runs, and how each authored card becomes
//! one shared [`AnalysisResultDocument`].
//!
//! The request names one family. The canonical [`DeckPlan`] names the authored
//! cards and their identities, and this module is the single place that turns
//! one materialized card into an engine call and a typed result projection.
//! Nothing here decides what a directive *means*: the configuration types, the
//! runners, and the result projections all belong to `rspice-core`.
//!
//! Families the shared result contract cannot express yet are listed in
//! [`unmapped_deck_card`] with the exact core API that is missing. They are
//! refused before any solver work rather than executed into a lossy artifact.

use rspice_core::abort_signal::AbortSignal;
use rspice_core::analysis::{
    AcSensitivityOutput, Distribution, HbConfig, HbTone, PacConfig, PssConfig, StbConfig,
    StbSweepType,
};
use rspice_core::execution::result_document::AxisValues;
use rspice_core::execution::{
    AnalysisInstanceId, AnalysisKind as PlannedAnalysisKind, AnalysisResultDocument,
    AnalysisResultDocumentBuilder, MaterializedAnalysis, ResultAxis, ResultAxisKind, SignalUnit,
};
use rspice_core::netlist::{
    AnalysisCommand, DcSweepSpec, ElementKind, FftFormat, FreqVariation, MonteCarloDistribution,
    PoleZeroAnalysisType, PoleZeroTransferType,
};
use rspice_core::{Engine, Netlist, SimulationError};

use crate::failure::{DirectiveFailure, check_abort, map_result_document_error};
use crate::fft_result_document::{FftResultDocumentError, TransientFftResultDocument};

/// Harmonics per tone when neither the `.HB` card nor `.OPTIONS HBINT NUMFREQ`
/// says. This mirrors the shared default the other frontends resolve to.
const DEFAULT_HB_HARMONICS: usize = 9;

/// Accepted samples one analysis may retain before the bounded resource
/// outcome replaces an unbounded waveform.
pub(crate) const MAX_SERIES_SAMPLES: usize = 2_000_000;

//=============================================================================
// Requested families
//=============================================================================

/// One analysis family the request may select, in its wire spelling.
///
/// The wire name selects a planned analysis kind; the plan, not this table,
/// decides which authored cards carry that kind and what identity each gets.
pub const REQUEST_KINDS: &[(&str, PlannedAnalysisKind)] = &[
    ("operating_point", PlannedAnalysisKind::Op),
    ("dc_sweep", PlannedAnalysisKind::Dc),
    ("ac_small_signal", PlannedAnalysisKind::Ac),
    ("transient", PlannedAnalysisKind::Tran),
    ("noise", PlannedAnalysisKind::Noise),
    ("distortion", PlannedAnalysisKind::Distortion),
    ("transfer_function", PlannedAnalysisKind::TransferFunction),
    ("stability", PlannedAnalysisKind::Stb),
    ("sensitivity", PlannedAnalysisKind::Sensitivity),
    ("pole_zero", PlannedAnalysisKind::PoleZero),
    ("monte_carlo", PlannedAnalysisKind::MonteCarlo),
    ("harmonic_balance", PlannedAnalysisKind::HarmonicBalance),
    ("pss", PlannedAnalysisKind::Pss),
    ("pac", PlannedAnalysisKind::Pac),
    ("envelope", PlannedAnalysisKind::Envelope),
];

/// Wire spellings this build recognizes but deliberately does not run, each
/// with the reason an operator needs to act on it.
pub const REFUSED_REQUEST_KINDS: &[(&str, &str)] = &[
    ("mixed_signal", MIXED_SIGNAL_IS_TRANSIENT),
    ("s_parameters", S_PARAMETER_GAP),
    ("port_noise", S_PARAMETER_GAP),
    ("pnoise", PNOISE_GAP),
    ("fourier", POST_PROCESS_IDENTITY_GAP),
    ("fft", POST_PROCESS_IDENTITY_GAP),
];

const MIXED_SIGNAL_IS_TRANSIENT: &str = "A mixed-signal deck is a transient: request the transient kind. This engine build has no \
     separate mixed-signal analysis, and running one under its own name would report an \
     analysis the deck never authored.";

const S_PARAMETER_GAP: &str = "S-parameter and port-noise results have no shared engine entry point: rspice-core exposes \
     no Engine::run_* method that produces an SParameterResult, so the adapter cannot publish \
     the shared sp/port-noise result document without deciding the .SP projection itself.";

const PNOISE_GAP: &str = "Periodic-noise results have no shared projection: the driven .PNOISE runners return \
     PnoiseAnalysisResult and the autonomous one returns OscPnoiseResult, while \
     AnalysisResultDocument::from_pnoise accepts only analysis::pnoise::PnoiseResult, which no \
     Engine::run_* method produces.";

const POST_PROCESS_IDENTITY_GAP: &str = "Fourier and FFT results have no canonical analysis identity: DeckPlan mints no \
     AnalysisInstanceId for .FOUR or .FFT and AnalysisInstanceId has no public constructor, so \
     the shared fourier/fft result document cannot be named.";

const PAC_HB_UPSTREAM_GAP: &str = "A .PAC or .PNOISE card that linearizes around a .HB carrier cannot record that provenance: \
     the shared result document accepts only a .PSS parent for the pac and pnoise families. \
     Author the periodic operating point as .PSS.";

/// Resolve one wire analysis kind.
pub fn planned_kind_for_request(kind: &str) -> Option<PlannedAnalysisKind> {
    REQUEST_KINDS
        .iter()
        .find(|(name, _)| *name == kind)
        .map(|(_, planned)| *planned)
}

/// The reason a recognized wire kind is refused, when it is.
pub fn refusal_for_request(kind: &str) -> Option<&'static str> {
    REFUSED_REQUEST_KINDS
        .iter()
        .find(|(name, _)| *name == kind)
        .map(|(_, reason)| *reason)
}

/// Wire spelling of one planned analysis kind, for response labelling.
pub fn request_kind_name(kind: PlannedAnalysisKind) -> Option<&'static str> {
    let kind = if kind == PlannedAnalysisKind::ImplicitOp {
        PlannedAnalysisKind::Op
    } else {
        kind
    };
    REQUEST_KINDS
        .iter()
        .find(|(_, planned)| *planned == kind)
        .map(|(name, _)| *name)
}

/// Whether one planned analysis identity answers the requested family.
///
/// An implicit operating point is the deck's own OP when it authored none, so
/// an `operating_point` request selects it exactly as it selects `.OP`.
pub fn matches_request(requested: PlannedAnalysisKind, planned: PlannedAnalysisKind) -> bool {
    planned == requested
        || (requested == PlannedAnalysisKind::Op && planned == PlannedAnalysisKind::ImplicitOp)
}

/// Dot-command spelling and refusal reason for an authored card this build has
/// no lossless result mapping for.
pub fn unmapped_deck_card(command: &AnalysisCommand) -> Option<(&'static str, &'static str)> {
    match command {
        AnalysisCommand::Sp { .. } => Some((".SP", S_PARAMETER_GAP)),
        AnalysisCommand::Pnoise(_) => Some((".PNOISE", PNOISE_GAP)),
        AnalysisCommand::Four { .. } => Some((".FOUR", POST_PROCESS_IDENTITY_GAP)),
        _ => None,
    }
}

//=============================================================================
// Directive execution
//=============================================================================

/// One executed card's typed projection.
pub(crate) struct DirectiveProjection {
    /// The shared result document, staged with its family payload and series.
    pub(crate) builder: AnalysisResultDocumentBuilder,
    /// Transient `.FFT` spectra, when the parent transient authored any.
    pub(crate) fft: Option<TransientFftResultDocument>,
}

impl From<AnalysisResultDocumentBuilder> for DirectiveProjection {
    fn from(builder: AnalysisResultDocumentBuilder) -> Self {
        Self { builder, fft: None }
    }
}

/// Execute one materialized card and project its result.
///
/// `peers` are the other analyses materialized at the same coordinate; a
/// `.PAC` or `.ENVELOPE` card reads its upstream periodic card from them so
/// the large-signal operating point comes from the deck's own configuration
/// rather than from a re-derived default.
pub(crate) fn run_directive(
    engine: &Engine,
    netlist: &Netlist,
    analysis: &MaterializedAnalysis,
    peers: &[MaterializedAnalysis],
    abort: &dyn AbortSignal,
) -> Result<DirectiveProjection, DirectiveFailure> {
    check_abort(abort)?;
    let id = analysis.id();
    let Some(command) = analysis.command() else {
        if analysis.id().kind() != PlannedAnalysisKind::ImplicitOp {
            return Err(DirectiveFailure::ResultDocument(
                "the canonical materializer omitted a requested authored directive".to_owned(),
            ));
        }
        return operating_point(engine, netlist, id, abort).map(DirectiveProjection::from);
    };
    if let Some((card, reason)) = unmapped_deck_card(command) {
        return Err(DirectiveFailure::UnsupportedForm(format!(
            "The deck authors a {card} card ({id}). {reason}"
        )));
    }
    let projection = match command {
        AnalysisCommand::Op => operating_point(engine, netlist, id, abort),
        AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            mode,
            sweep2,
        } => {
            let primary = DcSweepSpec {
                start: *start,
                stop: *stop,
                step: *step,
                mode: mode.clone(),
            };
            let points = engine.run_dc_sweep2_spec_with_report_and_abort(
                netlist,
                source,
                &primary,
                sweep2.as_ref(),
                abort,
            )?;
            if points.len() > MAX_SERIES_SAMPLES {
                return Err(DirectiveFailure::SeriesBudget);
            }
            let mut builder = AnalysisResultDocument::from_dc_sweep(
                id,
                source,
                sweep_axis_unit(netlist, source),
                &points,
            )
            .map_err(map_result_document_error)?;
            if let Some(outer) = sweep2 {
                let inner_count = primary.points().len();
                let values: Vec<f64> = outer
                    .spec()
                    .points()
                    .into_iter()
                    .flat_map(|value| std::iter::repeat_n(value, inner_count))
                    .collect();
                if values.len() != points.len() {
                    return Err(DirectiveFailure::ResultDocument(
                        "nested DC result shape does not match its declared sweep grid".to_owned(),
                    ));
                }
                let axis = ResultAxis::new(
                    format!("sweep:{}", outer.source.trim().to_ascii_lowercase()),
                    outer.source.trim(),
                    ResultAxisKind::SweepValue,
                    sweep_axis_unit(netlist, &outer.source),
                    AxisValues::Real { values },
                )
                .map_err(map_result_document_error)?;
                builder = builder.axis(axis);
            }
            Ok(builder)
        }
        AnalysisCommand::Ac {
            variation,
            points,
            start_freq,
            stop_freq,
        } => {
            let frequencies =
                sweep_frequencies(*variation, *points, *start_freq, *stop_freq, abort)?;
            let results = engine.run_ac_with_abort(netlist, &frequencies, abort)?;
            AnalysisResultDocument::from_ac(id, &results).map_err(map_result_document_error)
        }
        AnalysisCommand::Tran {
            step,
            stop,
            start,
            max_step,
            uic,
        } => {
            let ceiling = rspice_core::execution::resolve_transient_maximum_step(
                *step, *stop, *start, *max_step,
            )
            .map_err(|error| DirectiveFailure::InvalidAnalysis(error.to_string()))?;
            let result = engine.run_tran_with_startup_mode_and_abort(
                netlist,
                *stop,
                ceiling,
                rspice_core::engine::TransientStartupMode::from_uic(*uic),
                abort,
            )?;
            if result.time.len() > MAX_SERIES_SAMPLES {
                return Err(DirectiveFailure::SeriesBudget);
            }
            validate_fft_result_sequence(netlist, &result.fft_results, *stop, abort)?;
            let fft = if result.fft_results.is_empty() {
                None
            } else {
                Some(
                    TransientFftResultDocument::from_engine_results_with_abort(
                        id.tag(),
                        &result.fft_results,
                        &netlist.fft_analyses,
                        netlist.options.fft_mode.unwrap_or_default(),
                        abort,
                    )
                    .map_err(map_fft_document_error)?,
                )
            };
            return AnalysisResultDocument::from_transient(id, &result, None, Vec::new())
                .map_err(map_result_document_error)
                .map(|builder| DirectiveProjection { builder, fft });
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
            let frequencies =
                sweep_frequencies(*variation, *points, *start_freq, *stop_freq, abort)?;
            let results = engine.run_noise_named_with_input_source_and_abort(
                netlist,
                output_node,
                reference_node.as_deref(),
                input_source,
                &frequencies,
                netlist
                    .options
                    .temp
                    .map_or(engine.config().temperature, |temp| {
                        rspice_core::constants::celsius_to_kelvin(temp)
                    }),
                abort,
            )?;
            AnalysisResultDocument::from_noise(id, &results).map_err(map_result_document_error)
        }
        AnalysisCommand::Disto {
            variation,
            points,
            start_freq,
            stop_freq,
            f2_over_f1,
        } => {
            let frequencies =
                sweep_frequencies(*variation, *points, *start_freq, *stop_freq, abort)?;
            let result =
                engine.run_distortion_with_abort(netlist, &frequencies, *f2_over_f1, abort)?;
            AnalysisResultDocument::from_distortion(id, &result).map_err(map_result_document_error)
        }
        AnalysisCommand::Tf {
            output_node,
            reference_node,
            output_is_current,
            input_source,
        } => {
            let result = engine.run_transfer_function_with_abort(
                netlist,
                output_node,
                reference_node.as_deref(),
                *output_is_current,
                input_source,
                abort,
            )?;
            AnalysisResultDocument::from_transfer_function(id, &result)
                .map_err(map_result_document_error)
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
                .with_sweep_type(match variation {
                    FreqVariation::Lin => StbSweepType::Linear,
                    FreqVariation::Dec => StbSweepType::Decade,
                    FreqVariation::Oct => StbSweepType::Octave,
                })
                .with_probe(probe);
            let result = engine.run_stb_with_abort(netlist, config, abort)?;
            AnalysisResultDocument::from_stability(id, &result.result)
                .map_err(map_result_document_error)
        }
        AnalysisCommand::Sensitivity {
            output_node,
            reference_node,
            output_is_current,
            filters,
            ac_sweep,
        } => {
            if ac_sweep.is_some() {
                return Err(DirectiveFailure::UnsupportedForm(format!(
                    "The .SENS card ({id}) requests an AC sweep. The engine returns an \
                     AcSensitivityResult for that form, and the shared sensitivity result \
                     document accepts only the DC SensitivityResult."
                )));
            }
            let output = if *output_is_current {
                AcSensitivityOutput::BranchCurrent(output_node.clone())
            } else {
                let resolver = NodeResolver::build(engine, netlist, abort)?;
                let positive = resolver.resolve(output_node, ".SENS output")?;
                let negative = match reference_node {
                    Some(node) => {
                        let index = resolver.resolve(node, ".SENS reference")?;
                        (index != 0).then_some(index)
                    }
                    None => None,
                };
                AcSensitivityOutput::Voltage { positive, negative }
            };
            let result =
                engine.run_sensitivity_dc_complete_with_abort(netlist, output, filters, abort)?;
            AnalysisResultDocument::from_sensitivity(id, &result).map_err(map_result_document_error)
        }
        AnalysisCommand::PoleZero {
            input_pos,
            input_neg,
            output_pos,
            output_neg,
            transfer_type,
            analysis_type,
        } => {
            let resolver = NodeResolver::build(engine, netlist, abort)?;
            let (compute_poles, compute_zeros) = match analysis_type {
                PoleZeroAnalysisType::PoleZero => (true, true),
                PoleZeroAnalysisType::PolesOnly => (true, false),
                PoleZeroAnalysisType::ZerosOnly => (false, true),
            };
            let result = engine.run_pz_ports_with_abort(
                netlist,
                resolver.resolve(input_pos, ".PZ input")?,
                Some(resolver.resolve(input_neg, ".PZ input reference")?),
                resolver.resolve(output_pos, ".PZ output")?,
                Some(resolver.resolve(output_neg, ".PZ output reference")?),
                matches!(transfer_type, PoleZeroTransferType::Current),
                compute_poles,
                compute_zeros,
                abort,
            )?;
            AnalysisResultDocument::from_pole_zero(id, &result).map_err(map_result_document_error)
        }
        AnalysisCommand::MonteCarlo(card) => {
            let distribution = match card.distribution {
                MonteCarloDistribution::Gaussian => Distribution::Gaussian {
                    sigma: card.relative_spread,
                },
                MonteCarloDistribution::Uniform => Distribution::Uniform {
                    tolerance: card.relative_spread,
                },
                MonteCarloDistribution::WorstCase => Distribution::WorstCase {
                    tolerance: card.relative_spread,
                },
            };
            let result = engine.run_monte_carlo_with_options_and_abort(
                netlist,
                card.runs,
                // A deck that does not seed its own Monte Carlo is still
                // required to reproduce byte for byte, so the seed is the
                // documented constant rather than an entropy source.
                card.seed.unwrap_or(1),
                distribution,
                (!card.params.is_empty()).then_some(card.params.as_slice()),
                abort,
            )?;
            AnalysisResultDocument::from_monte_carlo(id, &result).map_err(map_result_document_error)
        }
        AnalysisCommand::Hb { frequencies } => {
            let config = hb_config(netlist, frequencies)?;
            let result = engine.run_hb_with_abort(netlist, config, abort)?;
            AnalysisResultDocument::from_harmonic_balance(id, &result.result)
                .map_err(map_result_document_error)
        }
        AnalysisCommand::Pss(card) => {
            let result =
                engine.run_pss_with_abort(netlist, PssConfig::from(card.as_ref()), abort)?;
            AnalysisResultDocument::from_pss(id, &result.result).map_err(map_result_document_error)
        }
        AnalysisCommand::Pac(card) => {
            let (upstream_id, upstream) = upstream_card(analysis, peers)?;
            let config = PacConfig::from(card.as_ref());
            let AnalysisCommand::Pss(pss) = upstream else {
                return Err(DirectiveFailure::UnsupportedForm(format!(
                    "The .PAC card ({id}) linearizes around {upstream_id}. {PAC_HB_UPSTREAM_GAP}"
                )));
            };
            let operating_point = engine.run_pss_operating_point_with_abort(
                netlist,
                PssConfig::from(pss.as_ref()),
                abort,
            )?;
            let result =
                engine.run_pac_from_pss_with_abort(netlist, config, &operating_point, abort)?;
            AnalysisResultDocument::from_pac(id, &result.result)
                .map(|builder| builder.parent_analysis(upstream_id))
                .map_err(map_result_document_error)
        }
        AnalysisCommand::Envelope(card) => {
            let (upstream_id, upstream) = upstream_card(analysis, peers)?;
            let AnalysisCommand::Hb { frequencies } = upstream else {
                return Err(DirectiveFailure::ResultDocument(format!(
                    "the canonical plan bound {id} to {upstream_id}, which is not a .HB carrier"
                )));
            };
            let config = hb_config(netlist, frequencies)?;
            let result = engine.run_envelope_with_abort(
                netlist,
                config,
                &card.frozen_sources,
                card.duration,
                card.max_step,
                abort,
            )?;
            AnalysisResultDocument::from_envelope(id, &result)
                .map(|builder| builder.parent_analysis(upstream_id))
                .map_err(map_result_document_error)
        }
        // Run axes and post-processing cards occupy no planned analysis slot,
        // and every unmapped card was refused above, so reaching this arm is
        // an executor logic error rather than deck content.
        other => Err(DirectiveFailure::Engine(SimulationError::Circuit(format!(
            "directive class {other:?} reached the executor without a planned identity"
        )))),
    };
    projection.map(DirectiveProjection::from)
}

fn operating_point(
    engine: &Engine,
    netlist: &Netlist,
    id: AnalysisInstanceId,
    abort: &dyn AbortSignal,
) -> Result<AnalysisResultDocumentBuilder, DirectiveFailure> {
    let (result, report) = engine.run_dc_op_with_report_and_abort(netlist, abort)?;
    AnalysisResultDocument::from_operating_point(id, &result, Some(&report))
        .map_err(map_result_document_error)
}

/// The authored card whose periodic operating point this analysis consumes.
fn upstream_card<'a>(
    analysis: &MaterializedAnalysis,
    peers: &'a [MaterializedAnalysis],
) -> Result<(AnalysisInstanceId, &'a AnalysisCommand), DirectiveFailure> {
    let upstream = analysis.planned().request().upstream().ok_or_else(|| {
        DirectiveFailure::ResultDocument(format!(
            "{} requires an upstream periodic analysis and the canonical plan bound none",
            analysis.id()
        ))
    })?;
    let command = peers
        .iter()
        .find(|peer| peer.id() == upstream)
        .and_then(MaterializedAnalysis::command)
        .ok_or_else(|| {
            DirectiveFailure::ResultDocument(format!(
                "the materialized coordinate omitted upstream analysis {upstream}"
            ))
        })?;
    Ok((upstream, command))
}

fn sweep_frequencies(
    variation: FreqVariation,
    points: usize,
    start: f64,
    stop: f64,
    abort: &dyn AbortSignal,
) -> Result<Vec<f64>, DirectiveFailure> {
    rspice_core::analysis::ac::try_ac_sweep_frequencies_with_abort(
        variation, points, start, stop, abort,
    )
    .map_err(DirectiveFailure::FrequencyGrid)
}

/// Harmonic-balance configuration for one authored `.HB` card.
///
/// `.OPTIONS HBINT NUMFREQ` supplies the harmonic order per tone, broadcasting
/// a single order across every tone. An explicit single-tone order also pins
/// the minimal bilateral `2N+1` collocation grid, which is the Xyce contract
/// the parser records the option for.
fn hb_config(netlist: &Netlist, frequencies: &[f64]) -> Result<HbConfig, DirectiveFailure> {
    if frequencies.is_empty() {
        return Err(DirectiveFailure::InvalidAnalysis(
            ".HB requires at least one positive tone frequency".to_owned(),
        ));
    }
    let requested = &netlist.options.hb_num_frequencies;
    let orders: Vec<usize> = if requested.is_empty() {
        vec![DEFAULT_HB_HARMONICS; frequencies.len()]
    } else if requested.contains(&0) {
        return Err(DirectiveFailure::InvalidAnalysis(
            ".OPTIONS HBINT NUMFREQ harmonic orders must all be at least 1".to_owned(),
        ));
    } else if requested.len() == 1 {
        vec![requested[0]; frequencies.len()]
    } else if requested.len() == frequencies.len() {
        requested.clone()
    } else {
        return Err(DirectiveFailure::InvalidAnalysis(format!(
            ".HB has {} tones but .OPTIONS HBINT NUMFREQ lists {} harmonic orders; provide one \
             order to broadcast or one per tone",
            frequencies.len(),
            requested.len()
        )));
    };

    if frequencies.len() == 1 {
        let config = HbConfig::new(frequencies[0]).with_harmonics(orders[0]);
        if requested.is_empty() {
            return Ok(config);
        }
        let points = config.minimum_collocation_points().ok_or_else(|| {
            DirectiveFailure::InvalidAnalysis(format!(
                ".OPTIONS HBINT NUMFREQ harmonic count {} exceeds the addressable collocation grid",
                orders[0]
            ))
        })?;
        return Ok(config.with_collocation_points(points));
    }

    let mut seen = std::collections::BTreeSet::new();
    let mut tones = Vec::with_capacity(frequencies.len());
    for (index, (frequency, order)) in frequencies.iter().zip(&orders).enumerate() {
        if !frequency.is_finite() || *frequency <= 0.0 {
            return Err(DirectiveFailure::InvalidAnalysis(format!(
                ".HB tone {} must be a positive finite frequency, not {frequency}",
                index + 1
            )));
        }
        if !seen.insert(frequency.to_bits()) {
            return Err(DirectiveFailure::InvalidAnalysis(format!(
                ".HB lists the tone frequency {frequency} more than once"
            )));
        }
        tones.push(HbTone::new(*frequency, *order).with_name(format!("tone{}", index + 1)));
    }
    Ok(HbConfig::multi_tone(tones))
}

/// Declared unit of one `.DC` sweep axis.
///
/// A source has the unit its excitation is measured in and temperature is
/// degrees Celsius. A swept parameter or device parameter has no unit the
/// simulator knows, so it is declared as the explicit custom symbol
/// `unspecified` rather than being claimed dimensionless.
fn sweep_axis_unit(netlist: &Netlist, source: &str) -> SignalUnit {
    if source.eq_ignore_ascii_case("temp") || source.eq_ignore_ascii_case("temper") {
        return SignalUnit::Custom("degC".to_owned());
    }
    netlist
        .elements
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case(source))
        .and_then(|element| match &element.kind {
            ElementKind::VoltageSource(_) | ElementKind::VoltageSourceDeferred(_) => {
                Some(SignalUnit::Volt)
            }
            ElementKind::CurrentSource(_) | ElementKind::CurrentSourceDeferred(_) => {
                Some(SignalUnit::Ampere)
            }
            _ => None,
        })
        .unwrap_or(SignalUnit::Custom("unspecified".to_owned()))
}

/// Netlist node name to public SPICE node index.
struct NodeResolver {
    indices: std::collections::HashMap<String, usize>,
    ground: rspice_core::netlist::GroundPolicy,
}

impl NodeResolver {
    fn build(
        engine: &Engine,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<Self, DirectiveFailure> {
        let circuit = engine.build_circuit_with_abort(netlist, abort)?;
        Ok(Self {
            indices: circuit
                .node_names_sorted()
                .iter()
                .enumerate()
                .map(|(index, name)| (name.to_ascii_uppercase(), index + 1))
                .collect(),
            ground: netlist.ground_policy(),
        })
    }

    fn resolve(&self, node: &str, role: &str) -> Result<usize, DirectiveFailure> {
        let node = node.trim();
        if self.ground.is_ground(node) {
            return Ok(0);
        }
        if let Ok(index) = node.parse::<usize>() {
            return Ok(index);
        }
        self.indices
            .get(&node.to_ascii_uppercase())
            .copied()
            .ok_or_else(|| {
                DirectiveFailure::InvalidAnalysis(format!(
                    "{role} names node {node:?}, which the elaborated circuit does not contain"
                ))
            })
    }
}

/// Prove every engine FFT result answers exactly the `.FFT` card that asked
/// for it, before either is published.
pub(crate) fn validate_fft_result_sequence(
    netlist: &Netlist,
    results: &[rspice_core::engine::TransientFftResult],
    transient_stop: f64,
    abort: &dyn AbortSignal,
) -> Result<(), DirectiveFailure> {
    if results.len() != netlist.fft_analyses.len() {
        return Err(DirectiveFailure::ResultDocument(
            "transient FFT results do not match the source directive count".to_owned(),
        ));
    }
    let expected_mode = netlist.options.fft_mode.unwrap_or_default();
    let expected_accurate = netlist.options.fft_accurate.unwrap_or(true)
        && netlist.options.output_interval_schedule.is_none();
    let expected_metrics = netlist.options.fft_output_metrics.unwrap_or(false);
    for (result, authored) in results.iter().zip(&netlist.fft_analyses) {
        check_abort(abort)?;
        let expected_format = authored.format.unwrap_or(match expected_mode {
            rspice_core::netlist::XyceFftMode::HspiceCompatible => FftFormat::Normalized,
            rspice_core::netlist::XyceFftMode::SpectreCompatible => FftFormat::Unnormalized,
        });
        if result.output != authored.output
            || result.point_count != authored.points
            || !fft_float_equal(result.start_time, authored.start.unwrap_or(0.0))
            || !fft_float_equal(result.stop_time, authored.stop.unwrap_or(transient_stop))
            || result.format != expected_format
            || result.mode != expected_mode
            || result.accurate_sampling != expected_accurate
            || result.window != authored.window
            || result.window_name != authored.window_name
            || !fft_float_equal(result.alpha, authored.alpha)
            || result.metrics.is_some() != expected_metrics
        {
            return Err(DirectiveFailure::ResultDocument(
                "transient FFT result controls do not match their source directive".to_owned(),
            ));
        }
    }
    Ok(())
}

fn map_fft_document_error(error: FftResultDocumentError) -> DirectiveFailure {
    match error {
        FftResultDocumentError::Aborted => DirectiveFailure::Engine(SimulationError::Aborted),
        FftResultDocumentError::ArtifactTooLarge { .. } => DirectiveFailure::ResultArtifactBytes,
        other => DirectiveFailure::ResultDocument(other.to_string()),
    }
}

fn fft_float_equal(left: f64, right: f64) -> bool {
    let scale = left.abs().max(right.abs());
    if scale == 0.0 {
        left == right
    } else {
        (left - right).abs() <= 1.0e-12 * scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::execution::AnalysisResultKind;

    #[test]
    fn every_request_kind_names_one_planned_family_and_round_trips() {
        let mut seen = std::collections::BTreeSet::new();
        for (name, kind) in REQUEST_KINDS {
            assert!(seen.insert(*name), "duplicate request kind {name}");
            assert_eq!(planned_kind_for_request(name), Some(*kind));
            assert_eq!(request_kind_name(*kind), Some(*name));
            assert!(
                refusal_for_request(name).is_none(),
                "{name} is both runnable and refused"
            );
        }
        assert_eq!(
            request_kind_name(PlannedAnalysisKind::ImplicitOp),
            Some("operating_point"),
            "an implicit operating point answers the operating_point request"
        );
    }

    #[test]
    fn every_refused_kind_states_a_reason_and_is_not_runnable() {
        for (name, reason) in REFUSED_REQUEST_KINDS {
            assert!(
                planned_kind_for_request(name).is_none(),
                "{name} is refused and runnable"
            );
            assert!(!reason.trim().is_empty(), "{name} refuses without a reason");
        }
    }

    #[test]
    fn every_result_family_is_either_runnable_or_refused_by_name() {
        // The registry is the input: a new core result family must appear in
        // exactly one of the two request tables before this build ships.
        for kind in AnalysisResultKind::ALL {
            let runnable = REQUEST_KINDS
                .iter()
                .any(|(_, planned)| rspice_core::execution::analysis_result_kind(*planned) == kind);
            let refused = match kind {
                AnalysisResultKind::SParameters => refusal_for_request("s_parameters").is_some(),
                AnalysisResultKind::PortNoise => refusal_for_request("port_noise").is_some(),
                AnalysisResultKind::PNoise => refusal_for_request("pnoise").is_some(),
                AnalysisResultKind::Fourier => refusal_for_request("fourier").is_some(),
                AnalysisResultKind::Fft => refusal_for_request("fft").is_some(),
                AnalysisResultKind::OperatingPoint
                | AnalysisResultKind::DcSweep
                | AnalysisResultKind::Ac
                | AnalysisResultKind::Transient
                | AnalysisResultKind::Noise
                | AnalysisResultKind::Distortion
                | AnalysisResultKind::TransferFunction
                | AnalysisResultKind::Stability
                | AnalysisResultKind::Sensitivity
                | AnalysisResultKind::PoleZero
                | AnalysisResultKind::MonteCarlo
                | AnalysisResultKind::HarmonicBalance
                | AnalysisResultKind::Pss
                | AnalysisResultKind::Pac
                | AnalysisResultKind::Envelope => false,
            };
            assert!(
                runnable ^ refused,
                "{kind:?} must be exactly one of runnable and refused"
            );
        }
    }
}
