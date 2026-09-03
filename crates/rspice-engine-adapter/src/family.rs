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
    Distribution, HbConfig, PacConfig, PssConfig, StbConfig, StbSweepType,
};
use rspice_core::engine::SensitivityCardResult;
use rspice_core::execution::result_document::{AxisValues, DcSweepAxisDocument};
use rspice_core::execution::{
    AnalysisInstanceId, AnalysisKind as PlannedAnalysisKind, AnalysisResultDocument,
    AnalysisResultDocumentBuilder, MaterializedAnalysis, ResultAxis, ResultAxisKind,
    RunCoordinateId, SignalUnit,
};
use rspice_core::netlist::{
    AnalysisCommand, DcSweepSpec, ElementKind, FftFormat, FreqVariation, MonteCarloDistribution,
};
use rspice_core::{Engine, Netlist, SimulationError};

use crate::failure::{DirectiveFailure, check_abort, map_result_document_error};
use crate::fft_result_document::{FftResultDocumentError, TransientFftResultDocument};

/// Accepted samples one analysis may retain before the bounded resource
/// outcome replaces an unbounded waveform.
const MAX_SERIES_SAMPLES: usize = 2_000_000;

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
    ("pnoise", PlannedAnalysisKind::PNoise),
    ("s_parameters", PlannedAnalysisKind::Sp),
    ("envelope", PlannedAnalysisKind::Envelope),
];

/// Wire spellings this build recognizes but deliberately does not run, each
/// with the reason an operator needs to act on it.
pub const REFUSED_REQUEST_KINDS: &[(&str, &str)] = &[
    ("mixed_signal", MIXED_SIGNAL_IS_TRANSIENT),
    ("port_noise", SECOND_DOCUMENT_GAP),
    ("fourier", SECOND_DOCUMENT_GAP),
    ("fft", SECOND_DOCUMENT_GAP),
];

const MIXED_SIGNAL_IS_TRANSIENT: &str = "A mixed-signal deck is a transient: request the transient kind. This engine build has no \
     separate mixed-signal analysis, and running one under its own name would report an \
     analysis the deck never authored.";

/// Families whose result is a *second* document produced by another card.
///
/// The executor stages exactly one shared result document per materialized
/// card. Port noise is the `.SP` card's optional second result, and `.FOUR`
/// and `.FFT` are the transient's post-processing products; `rspice-core` now
/// names and projects all three, but this executor has no slot to publish them
/// in beside their parent, so a deck that authors one is refused rather than
/// executed with the second result silently dropped.
const SECOND_DOCUMENT_GAP: &str = "This executor publishes one shared result document per authored card, and this family is a \
     second result produced beside another card's: port noise beside .SP, and .FOUR and .FFT \
     beside their parent transient. rspice-core names and projects all three; the adapter has no \
     slot to publish them in, and dropping them silently is not an option.";

const SP_DONOISE_GAP: &str = "The .SP card requests DONOISE. Port noise is that card's second result document and this \
     executor publishes one document per card, so the noise evidence would be dropped. Author \
     the .SP card without its noise flag.";

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
        AnalysisCommand::Sp { do_noise: true, .. } => Some((".SP", SP_DONOISE_GAP)),
        AnalysisCommand::Four { .. } => Some((".FOUR", SECOND_DOCUMENT_GAP)),
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
    coordinate: RunCoordinateId,
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
            // Both authored sweep variables are declared, so the shared
            // document keeps the outer source instead of losing it, and the
            // core constructor checks the flattened grid against the card.
            let mut axes = Vec::new();
            if let Some(outer) = sweep2 {
                axes.push(DcSweepAxisDocument {
                    name: outer.source.trim().to_ascii_lowercase(),
                    unit: sweep_axis_unit(netlist, &outer.source),
                    value_count: outer.spec().points().len(),
                });
            }
            axes.push(DcSweepAxisDocument {
                name: source.trim().to_ascii_lowercase(),
                unit: sweep_axis_unit(netlist, source),
                value_count: primary.points().len(),
            });
            let mut builder = AnalysisResultDocument::from_nested_dc_sweep(id, &axes, &points)
                .map_err(map_result_document_error)?;
            if let Some(outer) = sweep2 {
                let inner_count = primary.points().len();
                let values: Vec<f64> = outer
                    .spec()
                    .points()
                    .into_iter()
                    .flat_map(|value| std::iter::repeat_n(value, inner_count))
                    .collect();
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
        AnalysisCommand::AcData { table_name } => {
            // The table supplies the frequency grid and may override circuit
            // parameters per row. The per-row netlists the engine returns are
            // the deck's own materialization, not a second interpretation, so
            // the result projection is the ordinary AC one.
            let (_rows, results) = engine.run_ac_data_with_abort(netlist, table_name, abort)?;
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
        AnalysisCommand::NoiseData {
            output_node,
            reference_node,
            input_source,
            table_name,
        } => {
            let (_rows, results) = engine.run_noise_data_named_with_input_source_and_abort(
                netlist,
                output_node,
                reference_node.as_deref(),
                input_source,
                table_name,
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
        AnalysisCommand::Sensitivity { .. } => {
            // The card names its own output probe; resolving it against the
            // elaborated circuit and choosing the DC or AC driver are core's
            // decisions, taken once in the card runner.
            match engine.run_sensitivity_from_card_with_abort(netlist, command, abort)? {
                SensitivityCardResult::Dc(result) => {
                    AnalysisResultDocument::from_sensitivity(id, &result)
                }
                SensitivityCardResult::Ac(result) => {
                    AnalysisResultDocument::from_ac_sensitivity(id, &result)
                }
            }
            .map_err(map_result_document_error)
        }
        AnalysisCommand::PoleZero { .. } => {
            let result = engine.run_pz_from_card_with_abort(netlist, command, abort)?;
            AnalysisResultDocument::from_pole_zero(id, &result).map_err(map_result_document_error)
        }
        AnalysisCommand::Sp { .. } => {
            let run = engine.run_sp_with_abort(netlist, command, abort)?;
            AnalysisResultDocument::from_s_parameters(id, &run.scattering)
                .map_err(map_result_document_error)
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
                // documented constant rather than an entropy source. Inside a
                // `.STEP` or `.TEMP` sweep the card runs once per coordinate,
                // and the core derivation gives each coordinate its own
                // reproducible stream instead of repeating one sample.
                rspice_core::execution::monte_carlo_seed_at_coordinate(
                    card.seed.unwrap_or(1),
                    coordinate,
                ),
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
            // Shooting `.PSS` and harmonic balance both produce the periodic
            // operating point a `.PAC` card linearizes around, and the shared
            // document records either as the parent.
            let result = match upstream {
                AnalysisCommand::Pss(pss) => {
                    let operating_point = engine.run_pss_operating_point_with_abort(
                        netlist,
                        PssConfig::from(pss.as_ref()),
                        abort,
                    )?;
                    engine.run_pac_from_pss_with_abort(netlist, config, &operating_point, abort)?
                }
                AnalysisCommand::Hb { frequencies } => {
                    let carrier = engine.run_hb_with_abort(
                        netlist,
                        hb_config(netlist, frequencies)?,
                        abort,
                    )?;
                    engine.run_pac_from_hb_with_abort(
                        netlist,
                        config,
                        &carrier.operating_point,
                        abort,
                    )?
                }
                _ => {
                    return Err(DirectiveFailure::ResultDocument(format!(
                        "the canonical plan bound {id} to {upstream_id}, which is not a periodic carrier"
                    )));
                }
            };
            AnalysisResultDocument::from_pac(id, &result.result)
                .map(|builder| builder.parent_analysis(upstream_id))
                .map_err(map_result_document_error)
        }
        AnalysisCommand::Pnoise(card) => {
            let (upstream_id, upstream) = upstream_card(analysis, peers)?;
            let result = match upstream {
                AnalysisCommand::Pss(pss) => {
                    let operating_point = engine.run_pss_operating_point_with_abort(
                        netlist,
                        PssConfig::from(pss.as_ref()),
                        abort,
                    )?;
                    engine.run_pnoise_card_from_pss_with_abort(
                        netlist,
                        card,
                        &operating_point,
                        abort,
                    )?
                }
                AnalysisCommand::Hb { frequencies } => {
                    let carrier = engine.run_hb_with_abort(
                        netlist,
                        hb_config(netlist, frequencies)?,
                        abort,
                    )?;
                    engine.run_pnoise_card_from_hb_with_abort(
                        netlist,
                        card,
                        &carrier.operating_point,
                        abort,
                    )?
                }
                _ => {
                    return Err(DirectiveFailure::ResultDocument(format!(
                        "the canonical plan bound {id} to {upstream_id}, which is not a periodic carrier"
                    )));
                }
            };
            AnalysisResultDocument::from_pnoise(id, &result)
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
/// The default harmonic order, the multi-tone common basis and the
/// `.OPTIONS HBINT NUMFREQ` collocation rule all belong to `rspice-core`; this
/// only hands it the card's tones and the deck's authored order list.
fn hb_config(netlist: &Netlist, frequencies: &[f64]) -> Result<HbConfig, DirectiveFailure> {
    HbConfig::from_hb_card(frequencies, &netlist.options.hb_num_frequencies)
        .map_err(|error| DirectiveFailure::InvalidAnalysis(format!("invalid .HB card: {error}")))
}

/// Declared unit of one `.DC` sweep axis.
///
/// A source has the unit its excitation is measured in and temperature is
/// degrees Celsius. A swept parameter or device parameter has no unit the
/// simulator knows, which is the shared document's `Unspecified` rather than
/// the pure ratio `Dimensionless` would claim.
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
        .unwrap_or(SignalUnit::Unspecified)
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
