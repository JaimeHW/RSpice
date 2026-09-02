//! Deck execution: analysis-kind gating, engine invocation, measurement
//! extraction, and bounded result-artifact emission.

use std::collections::{BTreeMap, HashSet};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use rspice_core::abort_signal::AbortSignal;
use rspice_core::circuit::DeviceOpReport;
use rspice_core::engine::{SimulationConfig, SpiceDialect};
use rspice_core::netlist::{AnalysisCommand, DcSweepSpec, ElementKind, FftFormat};
use rspice_core::solver::SimulationResult;
use rspice_core::{Engine, Netlist, SimulationError};
#[cfg(test)]
use rspice_output::write_atomic;
use rspice_output::{
    AtomicArtifactError, AtomicArtifactFile, AtomicArtifactOptions, DestinationState, Durability,
    PreparedAtomicArtifact, remove_artifact_durably, restore_artifact_durably,
    sync_artifact_parent,
};
use serde::Deserialize;
use serde_json::Value;

use crate::document::CircuitContent;
use crate::fft_result_document::{
    FFT_RESULT_DOCUMENT_CONTENT_TYPE, FFT_RESULT_DOCUMENT_SCHEMA, FFT_RESULT_DOCUMENT_VERSION,
    FftResultDocumentError, TransientFftResultDocument,
};
use crate::measure::{Measurement, canonical_decimal, finalize_measurements, measurement_name};
use crate::result_document::{
    AnalogAnalysisKind, AnalogResultDocument, AnalogSignalKind, AxisDocument, ComplexSample,
    DeviceStateSeries, RESULT_DOCUMENT_CONTENT_TYPE, RESULT_DOCUMENT_SCHEMA,
    RESULT_DOCUMENT_VERSION, SignalDocument, SignalOwner, SignalUnit, SignalValues,
};
use crate::wire::{
    EngineResponse, EngineResultArtifactDescriptor, MAX_ENGINE_ARTIFACT_BYTES,
    MAX_ENGINE_RESULT_ARTIFACTS, valid_result_path,
};

/// Wall-clock ceiling for all engine work in one request. The worker holds
/// the authoritative external deadline; this internal one exists so a
/// pathological deck produces the bounded `engine.time_limit` outcome instead
/// of an opaque kill. Compile-time by design: the launch contract clears the
/// environment, so there is no runtime configuration channel to harden.
const SOLVE_BUDGET: Duration = Duration::from_secs(240);

/// Transient runs longer than this many accepted samples stop with the
/// bounded resource outcome before their waveforms outgrow every downstream
/// byte budget.
const MAX_SERIES_SAMPLES: usize = 2_000_000;

/// The engine-owned analysis request. Version 1 selects which deck-carried
/// directive class runs; directive parameters stay in the deck so the digest
/// over the revision covers the complete simulation definition.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AnalysisRequestV1 {
    kind: String,
}

/// Classes of deck directives this executor runs, keyed by request kind.
#[derive(Clone, Copy, PartialEq)]
enum AnalysisKind {
    OperatingPoint,
    DcSweep,
    Transient,
    AcSmallSignal,
    Noise,
    MixedSignal,
}

impl AnalysisKind {
    fn parse(kind: &str) -> Option<Self> {
        match kind {
            "operating_point" => Some(Self::OperatingPoint),
            "dc_sweep" => Some(Self::DcSweep),
            "transient" => Some(Self::Transient),
            "ac_small_signal" => Some(Self::AcSmallSignal),
            "noise" => Some(Self::Noise),
            "mixed_signal" => Some(Self::MixedSignal),
            _ => None,
        }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::OperatingPoint => "operating_point",
            Self::DcSweep => "dc_sweep",
            Self::Transient => "transient",
            Self::AcSmallSignal => "ac_small_signal",
            Self::Noise => "noise",
            Self::MixedSignal => "mixed_signal",
        }
    }

    /// Whether a parsed deck directive belongs to this request's class.
    /// Mixed-signal decks drive event sources through the transient solver,
    /// so that kind selects the same directive class as `transient`.
    fn matches(self, directive: &AnalysisCommand) -> bool {
        match self {
            Self::OperatingPoint => matches!(directive, AnalysisCommand::Op),
            Self::DcSweep => matches!(directive, AnalysisCommand::Dc { .. }),
            Self::Transient | Self::MixedSignal => {
                matches!(directive, AnalysisCommand::Tran { .. })
            }
            Self::AcSmallSignal => matches!(directive, AnalysisCommand::Ac { .. }),
            Self::Noise => matches!(directive, AnalysisCommand::Noise { .. }),
        }
    }
}

struct SolveDeadline {
    start: Instant,
}

impl AbortSignal for SolveDeadline {
    fn is_aborted(&self) -> bool {
        self.start.elapsed() >= SOLVE_BUDGET
    }
}

/// One executed request: the wire response plus the results files it
/// declared, staged in memory until `main` writes and emits them together.
pub struct Execution {
    pub response: EngineResponse,
    pub artifacts: Vec<PendingArtifact>,
}

impl Execution {
    fn failed(failure_code: &str, failure_detail: &str) -> Self {
        Self {
            response: EngineResponse::failed(failure_code, failure_detail),
            artifacts: Vec::new(),
        }
    }
}

/// Executes one validated request against the interpreted circuit content.
/// Customer-content problems become canonical failures here; this function
/// touches no filesystem, so nothing in it can raise a process fault.
pub fn execute(analysis: &Value, content: &CircuitContent, engine_build: &str) -> Execution {
    let request: AnalysisRequestV1 = match serde_json::from_value(analysis.clone()) {
        Ok(request) => request,
        Err(_) => {
            return Execution::failed(
                "analysis.invalid",
                "The analysis request is not a valid version-1 object; expected {\"kind\": ...}.",
            );
        }
    };
    let Some(kind) = AnalysisKind::parse(&request.kind) else {
        return Execution::failed(
            "analysis.unsupported_kind",
            &format!(
                "Analysis kind {:?} is not supported by this engine build; supported kinds are \
                 operating_point, dc_sweep, transient, ac_small_signal, noise, and mixed_signal.",
                request.kind
            ),
        );
    };

    let expanded_netlist = match content {
        CircuitContent::Empty => {
            // The reserved empty circuit: a well-defined trivial solution for
            // an operating point (this is the deterministic release smoke),
            // and an honest refusal for every waveform analysis.
            if kind == AnalysisKind::OperatingPoint {
                return succeeded(kind, engine_build, Vec::new(), Vec::new(), 0);
            }
            return Execution::failed(
                "analysis.empty_circuit",
                &format!(
                    "An empty circuit has no {} response to compute.",
                    kind.as_str()
                ),
            );
        }
        CircuitContent::Deck { expanded_netlist } => expanded_netlist,
    };

    let netlist = match Netlist::parse_validated(expanded_netlist) {
        Ok(netlist) => netlist,
        Err(error) => {
            return Execution::failed(
                "netlist.parse_error",
                &format!("The netlist could not be parsed: {error}"),
            );
        }
    };

    let directives: Vec<&AnalysisCommand> = netlist
        .analyses
        .iter()
        .filter(|directive| kind.matches(directive))
        .collect();
    if directives.is_empty() {
        return Execution::failed(
            "analysis.directive_missing",
            &format!(
                "The requested {} analysis needs a matching directive in the deck, \
                 and the deck declares none.",
                kind.as_str()
            ),
        );
    }

    // Production solver configuration, pinned to the ngspice dialect the
    // published netlist grammar targets. This is the exact configuration the
    // release conformance covers; changing it is an engine_build change.
    let engine = Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Ngspice,
        ..SimulationConfig::default()
    });
    let deadline = SolveDeadline {
        start: Instant::now(),
    };

    let mut measurements = Vec::new();
    let mut artifacts = Vec::new();
    for (ordinal, directive) in directives.iter().enumerate() {
        match run_directive(&engine, &netlist, directive, kind, ordinal, &deadline) {
            Ok(outcome) => {
                let Some(combined_count) = artifacts.len().checked_add(outcome.artifacts.len())
                else {
                    return Execution::failed(
                        "resource.result_artifact_limit",
                        "The run produced more result artifacts than the protocol permits.",
                    );
                };
                if let Err(failure) = validate_artifact_budget(
                    combined_count,
                    artifacts
                        .iter()
                        .chain(&outcome.artifacts)
                        .map(|artifact| artifact.content.len() as u64),
                ) {
                    return match failure {
                        DirectiveFailure::ResultArtifactLimit => Execution::failed(
                            "resource.result_artifact_limit",
                            "The run produced more result artifacts than the protocol permits.",
                        ),
                        DirectiveFailure::ResultArtifactBytes => Execution::failed(
                            "resource.result_artifact_bytes",
                            "A result artifact exceeds the protocol byte limit.",
                        ),
                        _ => unreachable!("artifact budget only returns resource failures"),
                    };
                }
                measurements.extend(outcome.measurements);
                artifacts.extend(outcome.artifacts);
            }
            Err(DirectiveFailure::Engine(error)) => return failure_execution(&error),
            Err(DirectiveFailure::NonFinite) => {
                return Execution::failed(
                    "results.nonfinite",
                    "The analysis completed with non-finite values; the operating region \
                     of this circuit is outside the solver's validated range.",
                );
            }
            Err(DirectiveFailure::SeriesBudget) => {
                return Execution::failed(
                    "resource.series_limit",
                    "The analysis produced more samples than one run may retain; \
                     shorten the window or relax the timestep.",
                );
            }
            Err(DirectiveFailure::ResultDocument(detail)) => {
                return Execution::failed("results.schema_mismatch", &detail);
            }
            Err(DirectiveFailure::ResultArtifactLimit) => {
                return Execution::failed(
                    "resource.result_artifact_limit",
                    "The run produced more result artifacts than the protocol permits.",
                );
            }
            Err(DirectiveFailure::ResultArtifactBytes) => {
                return Execution::failed(
                    "resource.result_artifact_bytes",
                    "A result artifact exceeds the protocol byte limit.",
                );
            }
        }
    }

    if let Err(failure) = validate_pending_artifact_budget(&artifacts) {
        return match failure {
            DirectiveFailure::ResultArtifactLimit => Execution::failed(
                "resource.result_artifact_limit",
                "The run produced more result artifacts than the protocol permits.",
            ),
            DirectiveFailure::ResultArtifactBytes => Execution::failed(
                "resource.result_artifact_bytes",
                "A result artifact exceeds the protocol byte limit.",
            ),
            _ => unreachable!("pending artifact budget only returns resource failures"),
        };
    }

    let directive_count = directives.len();
    succeeded(
        kind,
        engine_build,
        finalize_measurements(measurements),
        artifacts,
        directive_count,
    )
}

fn validate_artifact_budget(
    artifact_count: usize,
    byte_lengths: impl IntoIterator<Item = u64>,
) -> Result<(), DirectiveFailure> {
    if artifact_count > MAX_ENGINE_RESULT_ARTIFACTS {
        return Err(DirectiveFailure::ResultArtifactLimit);
    }
    if byte_lengths
        .into_iter()
        .any(|length| !(1..=MAX_ENGINE_ARTIFACT_BYTES).contains(&length))
    {
        return Err(DirectiveFailure::ResultArtifactBytes);
    }
    Ok(())
}

fn validate_pending_artifact_budget(artifacts: &[PendingArtifact]) -> Result<(), DirectiveFailure> {
    validate_artifact_budget(
        artifacts.len(),
        artifacts
            .iter()
            .map(|artifact| artifact.content.len() as u64),
    )
}

/// Everything one executed directive contributes to the response.
struct DirectiveOutcome {
    measurements: Vec<Measurement>,
    artifacts: Vec<PendingArtifact>,
}

/// A results file staged in memory. Files are only written after every
/// directive has succeeded, so a failed run leaves `results/` empty and the
/// response is the single source of truth about declared outputs.
pub struct PendingArtifact {
    pub file_name: String,
    pub content_type: &'static str,
    pub content: String,
}

enum DirectiveFailure {
    Engine(SimulationError),
    NonFinite,
    SeriesBudget,
    ResultDocument(String),
    ResultArtifactLimit,
    ResultArtifactBytes,
}

impl From<SimulationError> for DirectiveFailure {
    fn from(error: SimulationError) -> Self {
        Self::Engine(error)
    }
}

fn map_fft_document_error(error: FftResultDocumentError) -> DirectiveFailure {
    match error {
        FftResultDocumentError::Aborted => DirectiveFailure::Engine(SimulationError::Aborted),
        FftResultDocumentError::ArtifactTooLarge { .. } => DirectiveFailure::ResultArtifactBytes,
        other => DirectiveFailure::ResultDocument(other.to_string()),
    }
}

fn analog_document(kind: AnalysisKind, ordinal: usize) -> Option<AnalogResultDocument> {
    let analog_kind = match kind {
        AnalysisKind::OperatingPoint => AnalogAnalysisKind::OperatingPoint,
        AnalysisKind::DcSweep => AnalogAnalysisKind::DcSweep,
        AnalysisKind::Transient => AnalogAnalysisKind::Transient,
        AnalysisKind::AcSmallSignal => AnalogAnalysisKind::AcSmallSignal,
        AnalysisKind::Noise => AnalogAnalysisKind::Noise,
        AnalysisKind::MixedSignal => return None,
    };
    Some(AnalogResultDocument::new(
        analog_kind,
        kind.as_str(),
        ordinal + 1,
    ))
}

fn add_typed_artifact(
    outcome: &mut DirectiveOutcome,
    kind: AnalysisKind,
    ordinal: usize,
    document: AnalogResultDocument,
) -> Result<(), DirectiveFailure> {
    let content = document
        .to_json()
        .map_err(|error| DirectiveFailure::ResultDocument(error.to_string()))?;
    outcome.artifacts.push(PendingArtifact {
        file_name: format!("{}-{}.result.json", kind.as_str(), ordinal + 1),
        content_type: RESULT_DOCUMENT_CONTENT_TYPE,
        content,
    });
    Ok(())
}

fn add_fft_typed_artifact(
    outcome: &mut DirectiveOutcome,
    ordinal: usize,
    document: TransientFftResultDocument,
    abort: &dyn AbortSignal,
) -> Result<(), DirectiveFailure> {
    let content = document
        .to_json_with_abort(abort, MAX_ENGINE_ARTIFACT_BYTES)
        .map_err(map_fft_document_error)?;
    outcome.artifacts.push(PendingArtifact {
        file_name: format!("transient-{}.fft.result.json", ordinal + 1),
        content_type: FFT_RESULT_DOCUMENT_CONTENT_TYPE,
        content,
    });
    Ok(())
}

fn validate_fft_result_sequence(
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
        if abort.is_aborted() {
            return Err(DirectiveFailure::Engine(SimulationError::Aborted));
        }
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

fn fft_float_equal(left: f64, right: f64) -> bool {
    let scale = left.abs().max(right.abs());
    if scale == 0.0 {
        left == right
    } else {
        (left - right).abs() <= 1.0e-12 * scale
    }
}

struct RealSolutionPoint<'a> {
    result: &'a SimulationResult,
    report: Option<&'a DeviceOpReport>,
}

struct ComplexSolutionPoint<'a> {
    node_names: &'a [String],
    voltages: &'a [num_complex::Complex64],
    branch_names: &'a [String],
    currents: &'a [num_complex::Complex64],
}

fn append_complex_solution<'a>(
    document: &mut AnalogResultDocument,
    points: impl IntoIterator<Item = ComplexSolutionPoint<'a>>,
) -> Result<(), DirectiveFailure> {
    let points: Vec<_> = points.into_iter().collect();
    let mut nodes = BTreeMap::<String, String>::new();
    let mut branches = BTreeMap::<String, String>::new();
    for point in &points {
        validate_pair(
            "complex node voltage",
            point.node_names.len(),
            point.voltages.len(),
        )?;
        validate_pair(
            "complex branch current",
            point.branch_names.len(),
            point.currents.len(),
        )?;
        extend_name_union(&mut nodes, point.node_names);
        extend_name_union(&mut branches, point.branch_names);
    }
    for (canonical, display) in nodes {
        let samples = points
            .iter()
            .map(|point| named_complex(point.node_names, point.voltages, &canonical))
            .collect();
        document.signals.push(SignalDocument {
            canonical_name: format!("v({canonical})"),
            display_name: format!("V({display})"),
            kind: AnalogSignalKind::Voltage,
            owner: SignalOwner::Node { name: display },
            unit: Some(SignalUnit::Volt),
            values: SignalValues::Complex { samples },
        });
    }
    for (canonical, display) in branches {
        let samples = points
            .iter()
            .map(|point| named_complex(point.branch_names, point.currents, &canonical))
            .collect();
        document.signals.push(SignalDocument {
            canonical_name: format!("i({canonical})"),
            display_name: format!("I({display})"),
            kind: AnalogSignalKind::BranchCurrent,
            owner: SignalOwner::Branch { name: display },
            unit: Some(SignalUnit::Ampere),
            values: SignalValues::Complex { samples },
        });
    }
    Ok(())
}

fn named_complex(
    names: &[String],
    values: &[num_complex::Complex64],
    canonical: &str,
) -> Option<ComplexSample> {
    names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(canonical))
        .and_then(|index| values.get(index))
        .map(|value| ComplexSample {
            real: value.re,
            imaginary: value.im,
        })
}

/// Append the union of all real node/branch solution columns. A name absent at
/// a particular point is retained as an explicit missing sample.
fn append_real_solution(
    document: &mut AnalogResultDocument,
    points: &[RealSolutionPoint<'_>],
) -> Result<(), DirectiveFailure> {
    let mut nodes = BTreeMap::<String, String>::new();
    let mut branches = BTreeMap::<String, String>::new();
    for point in points {
        validate_pair(
            "node voltage",
            point.result.node_names.len(),
            point.result.node_voltages.len(),
        )?;
        validate_pair(
            "branch current",
            point.result.branch_names.len(),
            point.result.branch_currents.len(),
        )?;
        extend_name_union(&mut nodes, &point.result.node_names);
        extend_name_union(&mut branches, &point.result.branch_names);
    }

    for (canonical, display) in nodes {
        let samples = points
            .iter()
            .map(|point| {
                named_real(
                    &point.result.node_names,
                    &point.result.node_voltages,
                    &canonical,
                )
            })
            .collect();
        document.signals.push(SignalDocument {
            canonical_name: format!("v({canonical})"),
            display_name: format!("V({display})"),
            kind: AnalogSignalKind::Voltage,
            owner: SignalOwner::Node { name: display },
            unit: Some(SignalUnit::Volt),
            values: SignalValues::Real { samples },
        });
    }
    for (canonical, display) in branches {
        let samples = points
            .iter()
            .map(|point| {
                named_real(
                    &point.result.branch_names,
                    &point.result.branch_currents,
                    &canonical,
                )
            })
            .collect();
        document.signals.push(SignalDocument {
            canonical_name: format!("i({canonical})"),
            display_name: format!("I({display})"),
            kind: AnalogSignalKind::BranchCurrent,
            owner: SignalOwner::Branch { name: display },
            unit: Some(SignalUnit::Ampere),
            values: SignalValues::Real { samples },
        });
    }
    append_device_reports(document, points);
    append_dc_observables(document, points);
    Ok(())
}

fn append_dc_observables(document: &mut AnalogResultDocument, points: &[RealSolutionPoint<'_>]) {
    let mut observables = BTreeMap::<String, String>::new();
    for point in points {
        for (name, _) in &point.result.dc_observables {
            observables
                .entry(name.to_ascii_lowercase())
                .or_insert_with(|| name.clone());
        }
    }
    for (canonical, display) in observables {
        if has_signal(document, &canonical) {
            continue;
        }
        let samples = points
            .iter()
            .map(|point| {
                point
                    .result
                    .dc_observables
                    .iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case(&canonical))
                    .map(|(_, value)| *value)
            })
            .collect();
        let (device, parameter) = observable_owner(&display);
        document.signals.push(SignalDocument {
            canonical_name: canonical.clone(),
            display_name: display,
            kind: AnalogSignalKind::DeviceObservable,
            owner: SignalOwner::Device {
                device,
                parameter,
                device_kind: None,
            },
            unit: observable_unit(&canonical),
            values: SignalValues::Real { samples },
        });
    }
}

fn append_device_reports(document: &mut AnalogResultDocument, points: &[RealSolutionPoint<'_>]) {
    let mut devices = BTreeMap::<String, (String, String)>::new();
    let mut parameters = BTreeMap::<String, (String, String, String)>::new();
    for point in points {
        let Some(report) = point.report else {
            continue;
        };
        for entry in &report.entries {
            let device_key = entry.name.to_ascii_lowercase();
            devices
                .entry(device_key.clone())
                .or_insert_with(|| (entry.name.clone(), entry.device_kind.to_owned()));
            for (parameter, _) in &entry.params {
                let canonical = format!("@{}[{}]", device_key, parameter.to_ascii_lowercase());
                parameters.entry(canonical).or_insert_with(|| {
                    (
                        entry.name.clone(),
                        (*parameter).to_owned(),
                        entry.device_kind.to_owned(),
                    )
                });
            }
        }
    }

    for (device_key, (display, device_kind)) in devices {
        let regions = points
            .iter()
            .map(|point| {
                point.report.and_then(|report| {
                    report
                        .entries
                        .iter()
                        .find(|entry| entry.name.eq_ignore_ascii_case(&device_key))
                        .and_then(|entry| entry.region.map(str::to_owned))
                })
            })
            .collect();
        document.device_states.push(DeviceStateSeries {
            device_name: display,
            device_kind: Some(device_kind),
            regions,
        });
    }
    for (canonical, (device, parameter, device_kind)) in parameters {
        if has_signal(document, &canonical) {
            continue;
        }
        let samples = points
            .iter()
            .map(|point| {
                point.report.and_then(|report| {
                    report
                        .entries
                        .iter()
                        .find(|entry| entry.name.eq_ignore_ascii_case(&device))
                        .and_then(|entry| {
                            entry
                                .params
                                .iter()
                                .find(|(name, _)| name.eq_ignore_ascii_case(&parameter))
                                .map(|(_, value)| *value)
                        })
                })
            })
            .collect();
        document.signals.push(SignalDocument {
            canonical_name: canonical.clone(),
            display_name: format!("@{device}[{parameter}]"),
            kind: AnalogSignalKind::DeviceObservable,
            owner: SignalOwner::Device {
                device: Some(device),
                parameter: Some(parameter.clone()),
                device_kind: Some(device_kind),
            },
            unit: device_parameter_unit(&parameter),
            values: SignalValues::Real { samples },
        });
    }
}

fn has_signal(document: &AnalogResultDocument, canonical_name: &str) -> bool {
    document
        .signals
        .iter()
        .any(|signal| signal.canonical_name.eq_ignore_ascii_case(canonical_name))
}

fn validate_pair(label: &str, names: usize, values: usize) -> Result<(), DirectiveFailure> {
    if names == values {
        Ok(())
    } else {
        Err(DirectiveFailure::ResultDocument(format!(
            "{label} result has {names} names but {values} values"
        )))
    }
}

fn extend_name_union(union: &mut BTreeMap<String, String>, names: &[String]) {
    for name in names {
        union
            .entry(name.to_ascii_lowercase())
            .or_insert_with(|| name.clone());
    }
}

fn named_real(names: &[String], values: &[f64], canonical: &str) -> Option<f64> {
    names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(canonical))
        .and_then(|index| values.get(index).copied())
}

fn observable_owner(name: &str) -> (Option<String>, Option<String>) {
    if let Some((device, parameter)) = name
        .strip_prefix('@')
        .and_then(|tail| tail.strip_suffix(']'))
        .and_then(|tail| tail.split_once('['))
    {
        return (Some(device.to_owned()), Some(parameter.to_owned()));
    }
    if let Some((device, parameter)) = name.split_once(':') {
        return (Some(device.to_owned()), Some(parameter.to_owned()));
    }
    let argument = name
        .split_once('(')
        .and_then(|(_, tail)| tail.strip_suffix(')'));
    (argument.map(str::to_owned), None)
}

fn observable_unit(name: &str) -> Option<SignalUnit> {
    let lower = name.to_ascii_lowercase();
    if lower.starts_with("i(") {
        Some(SignalUnit::Ampere)
    } else if lower.starts_with("p(") {
        Some(SignalUnit::Watt)
    } else if let Some((_, parameter)) = lower.rsplit_once(':') {
        device_parameter_unit(parameter)
    } else {
        None
    }
}

fn device_parameter_unit(parameter: &str) -> Option<SignalUnit> {
    match parameter.to_ascii_lowercase().as_str() {
        "v" | "vd" | "vds" | "vgs" | "vbs" | "vbe" | "vbc" | "vce" | "vth" | "vdsat" => {
            Some(SignalUnit::Volt)
        }
        "i" | "id" | "ig" | "is" | "ib" | "ic" | "ie" => Some(SignalUnit::Ampere),
        "gm" | "gds" | "gmb" | "gd" | "go" => Some(SignalUnit::Siemens),
        "r" | "rd" | "rs" | "rb" | "rc" | "ro" => Some(SignalUnit::Ohm),
        "c" | "cgs" | "cgd" | "cgb" | "cbs" | "cbd" => Some(SignalUnit::Farad),
        "p" | "power" | "pd" => Some(SignalUnit::Watt),
        "q" | "qg" | "qd" | "qs" | "qb" => Some(SignalUnit::Coulomb),
        "l" | "w" => Some(SignalUnit::Meter),
        "m" | "nf" | "beta" => Some(SignalUnit::Dimensionless),
        _ => None,
    }
}

fn dc_axis_unit(netlist: &Netlist, source: &str) -> Option<SignalUnit> {
    if source.eq_ignore_ascii_case("temp") || source.eq_ignore_ascii_case("temper") {
        return Some(SignalUnit::DegreeCelsius);
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
}

fn analysis_scalar_signal(
    canonical_name: &str,
    display_name: &str,
    unit: Option<SignalUnit>,
    samples: Vec<Option<f64>>,
) -> SignalDocument {
    SignalDocument {
        canonical_name: canonical_name.to_owned(),
        display_name: display_name.to_owned(),
        kind: AnalogSignalKind::Scalar,
        owner: SignalOwner::Analysis,
        unit,
        values: SignalValues::Real { samples },
    }
}

fn append_noise_contributions(
    document: &mut AnalogResultDocument,
    points: &[rspice_core::analysis::noise::NoiseResult],
) {
    let mut identities = BTreeMap::<String, (String, Option<String>)>::new();
    for point in points {
        for identity in point.contribution_catalog.iter().chain(
            point
                .contributions
                .iter()
                .map(|contribution| &contribution.identity),
        ) {
            let key = noise_identity_key(&identity.device, identity.mechanism.as_deref());
            identities
                .entry(key)
                .or_insert_with(|| (identity.device.clone(), identity.mechanism.clone()));
        }
    }
    for (key, (device, mechanism)) in identities {
        let values = |select: fn(&rspice_core::analysis::noise::NoiseContribution) -> f64| {
            points
                .iter()
                .map(|point| {
                    let matching: Vec<_> = point
                        .contributions
                        .iter()
                        .filter(|contribution| {
                            contribution.identity.device.eq_ignore_ascii_case(&device)
                                && match (
                                    contribution.identity.mechanism.as_deref(),
                                    mechanism.as_deref(),
                                ) {
                                    (Some(actual), Some(expected)) => {
                                        actual.eq_ignore_ascii_case(expected)
                                    }
                                    (None, None) => true,
                                    _ => false,
                                }
                        })
                        .collect();
                    (!matching.is_empty()).then(|| matching.into_iter().map(select).sum::<f64>())
                })
                .collect::<Vec<_>>()
        };
        let owner = || SignalOwner::Device {
            device: Some(device.clone()),
            parameter: mechanism.clone(),
            device_kind: None,
        };
        document.signals.extend([
            SignalDocument {
                canonical_name: format!("noise({key}).output_density"),
                display_name: format!("Noise({key}) output density"),
                kind: AnalogSignalKind::Scalar,
                owner: owner(),
                unit: Some(SignalUnit::VoltSquaredPerHertz),
                values: SignalValues::Real {
                    samples: values(|contribution| contribution.output_contribution),
                },
            },
            SignalDocument {
                canonical_name: format!("noise({key}).input_density"),
                display_name: format!("Noise({key}) input density"),
                kind: AnalogSignalKind::Scalar,
                owner: owner(),
                unit: Some(SignalUnit::VoltSquaredPerHertz),
                values: SignalValues::Real {
                    samples: values(|contribution| contribution.input_contribution),
                },
            },
            SignalDocument {
                canonical_name: format!("noise({key}).percentage"),
                display_name: format!("Noise({key}) percentage"),
                kind: AnalogSignalKind::Scalar,
                owner: owner(),
                unit: Some(SignalUnit::Dimensionless),
                values: SignalValues::Real {
                    samples: values(|contribution| contribution.percentage),
                },
            },
        ]);
    }
}

fn noise_identity_key(device: &str, mechanism: Option<&str>) -> String {
    match mechanism {
        Some(mechanism) => format!(
            "{},{}",
            device.to_ascii_lowercase(),
            mechanism.to_ascii_lowercase()
        ),
        None => device.to_ascii_lowercase(),
    }
}

fn run_directive(
    engine: &Engine,
    netlist: &Netlist,
    directive: &AnalysisCommand,
    kind: AnalysisKind,
    ordinal: usize,
    deadline: &dyn AbortSignal,
) -> Result<DirectiveOutcome, DirectiveFailure> {
    match directive {
        AnalysisCommand::Op => {
            let (result, report) = engine.run_dc_op_with_report_and_abort(netlist, deadline)?;
            validate_pair(
                "node voltage",
                result.node_names.len(),
                result.node_voltages.len(),
            )?;
            validate_pair(
                "branch current",
                result.branch_names.len(),
                result.branch_currents.len(),
            )?;
            let mut measurements = Vec::new();
            for (index, name) in result.node_names.iter().enumerate().skip(1) {
                let value = result.node_voltages[index];
                if !value.is_finite() {
                    return Err(DirectiveFailure::NonFinite);
                }
                measurements.extend(Measurement::scalar(measurement_name("v", name), "V", value));
            }
            for (index, name) in result.branch_names.iter().enumerate() {
                let value = result.branch_currents[index];
                if !value.is_finite() {
                    return Err(DirectiveFailure::NonFinite);
                }
                measurements.extend(Measurement::scalar(measurement_name("i", name), "A", value));
            }
            // The operating point publishes its solution as a results table,
            // like every sweep class: downstream evidence contracts require
            // each successful run to retain at least one result artifact.
            let mut content = String::from("name,unit,value\n");
            for measurement in &measurements {
                content.push_str(&format!(
                    "{},{},{}\n",
                    measurement.name, measurement.unit, measurement.value_decimal,
                ));
            }
            let artifacts = vec![PendingArtifact {
                file_name: format!("{}-{}.csv", kind.as_str(), ordinal + 1),
                content_type: "text/csv",
                content,
            }];
            let mut outcome = DirectiveOutcome {
                measurements,
                artifacts,
            };
            let mut document = analog_document(kind, ordinal).expect("OP is analog");
            document.point_count = 1;
            append_real_solution(
                &mut document,
                &[RealSolutionPoint {
                    result: &result,
                    report: Some(&report),
                }],
            )?;
            add_typed_artifact(&mut outcome, kind, ordinal, document)?;
            Ok(outcome)
        }
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
                deadline,
            )?;
            if points.is_empty() {
                return Err(DirectiveFailure::NonFinite);
            }
            for point in &points {
                validate_pair(
                    "DC node voltage",
                    point.result.node_names.len(),
                    point.result.node_voltages.len(),
                )?;
                validate_pair(
                    "DC branch current",
                    point.result.branch_names.len(),
                    point.result.branch_currents.len(),
                )?;
            }
            let sweep: Vec<f64> = points.iter().map(|point| point.sweep_value).collect();
            let first = &points[0].result;
            let mut columns: Vec<(String, &'static str, Vec<f64>)> = Vec::new();
            columns.push((
                format!("sweep({})", source.to_ascii_lowercase()),
                match dc_axis_unit(netlist, source) {
                    Some(SignalUnit::Ampere) => "A",
                    Some(SignalUnit::DegreeCelsius) => "degC",
                    _ => "V",
                },
                sweep,
            ));
            for name in first.node_names.iter().skip(1) {
                let series: Option<Vec<f64>> = points
                    .iter()
                    .map(|point| {
                        named_real(&point.result.node_names, &point.result.node_voltages, name)
                    })
                    .collect();
                if let Some(series) = series {
                    columns.push((measurement_name("v", name), "V", series));
                }
            }
            for name in &first.branch_names {
                let series: Option<Vec<f64>> = points
                    .iter()
                    .map(|point| {
                        named_real(
                            &point.result.branch_names,
                            &point.result.branch_currents,
                            name,
                        )
                    })
                    .collect();
                if let Some(series) = series {
                    columns.push((measurement_name("i", name), "A", series));
                }
            }
            let mut outcome = columns_outcome(kind, ordinal, columns)?;
            let mut document = analog_document(kind, ordinal).expect("DC is analog");
            document.point_count = points.len();
            document.axes.push(AxisDocument {
                name: source.clone(),
                unit: dc_axis_unit(netlist, source),
                values: points.iter().map(|point| Some(point.sweep_value)).collect(),
            });
            if let Some(outer) = sweep2 {
                let outer_points = outer.spec().points();
                let inner_count = primary.points().len();
                let values: Vec<Option<f64>> = outer_points
                    .into_iter()
                    .flat_map(|value| std::iter::repeat_n(Some(value), inner_count))
                    .collect();
                if values.len() != points.len() {
                    return Err(DirectiveFailure::ResultDocument(
                        "nested DC result shape does not match its declared sweep grid".to_owned(),
                    ));
                }
                document.axes.push(AxisDocument {
                    name: outer.source.clone(),
                    unit: dc_axis_unit(netlist, &outer.source),
                    values,
                });
            }
            let solution_points: Vec<RealSolutionPoint<'_>> = points
                .iter()
                .map(|point| RealSolutionPoint {
                    result: &point.result,
                    report: Some(&point.device_op_report),
                })
                .collect();
            append_real_solution(&mut document, &solution_points)?;
            add_typed_artifact(&mut outcome, kind, ordinal, document)?;
            Ok(outcome)
        }
        AnalysisCommand::Tran {
            step,
            stop,
            max_step,
            uic,
            ..
        } => {
            // ngspice's default transient print ceiling: (tstop)/50 unless
            // the deck asks for a finer explicit ceiling.
            let ceiling = max_step.unwrap_or_else(|| (*stop / 50.0).max(*step));
            let result = engine.run_tran_with_startup_mode_and_abort(
                netlist,
                *stop,
                ceiling,
                rspice_core::engine::TransientStartupMode::from_uic(*uic),
                deadline,
            )?;
            if result.time.is_empty() {
                return Err(DirectiveFailure::NonFinite);
            }
            if result.time.len() > MAX_SERIES_SAMPLES {
                return Err(DirectiveFailure::SeriesBudget);
            }
            validate_fft_result_sequence(netlist, &result.fft_results, *stop, deadline)?;
            let mut columns: Vec<(String, &'static str, Vec<f64>)> = Vec::new();
            columns.push(("time".to_owned(), "s", result.time.clone()));
            for (index, name) in result.node_names.iter().enumerate() {
                let waveform = result.voltages.get(index).ok_or_else(|| {
                    DirectiveFailure::ResultDocument(
                        "transient node names and voltage waveforms are misaligned".to_owned(),
                    )
                })?;
                if waveform.len() == result.time.len() {
                    columns.push((measurement_name("v", name), "V", waveform.clone()));
                } else if !waveform.is_empty() {
                    return Err(DirectiveFailure::ResultDocument(format!(
                        "transient voltage {name:?} has {} samples for {} times",
                        waveform.len(),
                        result.time.len()
                    )));
                }
            }
            for (index, name) in result.branch_names.iter().enumerate() {
                let waveform = result.branch_currents.get(index).ok_or_else(|| {
                    DirectiveFailure::ResultDocument(
                        "transient branch names and current waveforms are misaligned".to_owned(),
                    )
                })?;
                if waveform.len() == result.time.len() {
                    columns.push((measurement_name("i", name), "A", waveform.clone()));
                } else if !waveform.is_empty() {
                    return Err(DirectiveFailure::ResultDocument(format!(
                        "transient branch current {name:?} has {} samples for {} times",
                        waveform.len(),
                        result.time.len()
                    )));
                }
            }
            let mut outcome = columns_outcome(kind, ordinal, columns)?;
            if let Some(mut document) = analog_document(kind, ordinal) {
                document.point_count = result.time.len();
                document.axes.push(AxisDocument {
                    name: "time".to_owned(),
                    unit: Some(SignalUnit::Second),
                    values: result.time.iter().copied().map(Some).collect(),
                });
                for (index, name) in result.node_names.iter().enumerate() {
                    let waveform = &result.voltages[index];
                    let samples = if waveform.is_empty() {
                        vec![None; result.time.len()]
                    } else {
                        waveform.iter().copied().map(Some).collect()
                    };
                    document.signals.push(SignalDocument {
                        canonical_name: format!("v({})", name.to_ascii_lowercase()),
                        display_name: format!("V({name})"),
                        kind: AnalogSignalKind::Voltage,
                        owner: SignalOwner::Node { name: name.clone() },
                        unit: Some(SignalUnit::Volt),
                        values: SignalValues::Real { samples },
                    });
                }
                for (index, name) in result.branch_names.iter().enumerate() {
                    let waveform = &result.branch_currents[index];
                    let samples = if waveform.is_empty() {
                        vec![None; result.time.len()]
                    } else {
                        waveform.iter().copied().map(Some).collect()
                    };
                    document.signals.push(SignalDocument {
                        canonical_name: format!("i({})", name.to_ascii_lowercase()),
                        display_name: format!("I({name})"),
                        kind: AnalogSignalKind::BranchCurrent,
                        owner: SignalOwner::Branch { name: name.clone() },
                        unit: Some(SignalUnit::Ampere),
                        values: SignalValues::Real { samples },
                    });
                }
                for trace in &result.device_op_traces {
                    validate_pair(
                        "transient device observable",
                        result.time.len(),
                        trace.values.len(),
                    )?;
                    document.signals.push(SignalDocument {
                        canonical_name: format!(
                            "@{}[{}]",
                            trace.device_name.to_ascii_lowercase(),
                            trace.parameter.to_ascii_lowercase()
                        ),
                        display_name: format!("@{}[{}]", trace.device_name, trace.parameter),
                        kind: AnalogSignalKind::DeviceObservable,
                        owner: SignalOwner::Device {
                            device: Some(trace.device_name.clone()),
                            parameter: Some(trace.parameter.clone()),
                            device_kind: None,
                        },
                        unit: device_parameter_unit(&trace.parameter),
                        values: SignalValues::Real {
                            samples: trace.values.iter().copied().map(Some).collect(),
                        },
                    });
                }
                for trace in &result.store_traces {
                    validate_pair(
                        "transient device store",
                        result.time.len(),
                        trace.values.len(),
                    )?;
                    document.signals.push(SignalDocument {
                        canonical_name: trace.name.to_ascii_lowercase(),
                        display_name: trace.name.clone(),
                        kind: AnalogSignalKind::DeviceObservable,
                        owner: SignalOwner::Device {
                            device: None,
                            parameter: Some(trace.name.clone()),
                            device_kind: None,
                        },
                        unit: None,
                        values: SignalValues::Real {
                            samples: trace.values.iter().copied().map(Some).collect(),
                        },
                    });
                }
                let parent_analysis = document.analysis.clone();
                add_typed_artifact(&mut outcome, kind, ordinal, document)?;
                if !result.fft_results.is_empty() {
                    let fft_document = TransientFftResultDocument::from_engine_results_with_abort(
                        parent_analysis,
                        &result.fft_results,
                        &netlist.fft_analyses,
                        netlist.options.fft_mode.unwrap_or_default(),
                        deadline,
                    )
                    .map_err(map_fft_document_error)?;
                    add_fft_typed_artifact(&mut outcome, ordinal, fft_document, deadline)?;
                }
            }
            Ok(outcome)
        }
        AnalysisCommand::Ac {
            variation,
            points,
            start_freq,
            stop_freq,
        } => {
            let frequencies = rspice_core::analysis::ac::ac_sweep_frequencies(
                *variation,
                *points,
                *start_freq,
                *stop_freq,
            );
            if frequencies.is_empty() {
                return Ok(DirectiveOutcome {
                    measurements: Vec::new(),
                    artifacts: Vec::new(),
                });
            }
            let results = engine.run_ac_with_abort(netlist, &frequencies, deadline)?;
            if results.is_empty() {
                return Err(DirectiveFailure::NonFinite);
            }
            for point in &results {
                validate_pair(
                    "AC node voltage",
                    point.node_names.len(),
                    point.voltages.len(),
                )?;
                validate_pair(
                    "AC branch current",
                    point.branch_names.len(),
                    point.currents.len(),
                )?;
            }
            let mut columns: Vec<(String, &'static str, Vec<f64>)> = Vec::new();
            columns.push((
                "frequency".to_owned(),
                "Hz",
                results.iter().map(|point| point.frequency).collect(),
            ));
            let first = &results[0];
            for name in &first.node_names {
                let values = results.iter().map(|point| {
                    point
                        .node_names
                        .iter()
                        .position(|candidate| candidate.eq_ignore_ascii_case(name))
                        .and_then(|index| point.voltages.get(index))
                });
                let complex: Option<Vec<_>> = values.collect();
                if let Some(complex) = complex {
                    columns.push((
                        measurement_name("vm", name),
                        "V",
                        complex.iter().map(|value| value.norm()).collect(),
                    ));
                    columns.push((
                        measurement_name("vp", name),
                        "deg",
                        complex
                            .iter()
                            .map(|value| value.arg().to_degrees())
                            .collect(),
                    ));
                }
            }
            for name in &first.branch_names {
                let values = results.iter().map(|point| {
                    point
                        .branch_names
                        .iter()
                        .position(|candidate| candidate.eq_ignore_ascii_case(name))
                        .and_then(|index| point.currents.get(index))
                });
                let complex: Option<Vec<_>> = values.collect();
                if let Some(complex) = complex {
                    columns.push((
                        measurement_name("im", name),
                        "A",
                        complex.iter().map(|value| value.norm()).collect(),
                    ));
                    columns.push((
                        measurement_name("ip", name),
                        "deg",
                        complex
                            .iter()
                            .map(|value| value.arg().to_degrees())
                            .collect(),
                    ));
                }
            }
            let mut outcome = columns_outcome(kind, ordinal, columns)?;
            let mut document = analog_document(kind, ordinal).expect("AC is analog");
            document.point_count = results.len();
            document.axes.push(AxisDocument {
                name: "frequency".to_owned(),
                unit: Some(SignalUnit::Hertz),
                values: results.iter().map(|point| Some(point.frequency)).collect(),
            });
            append_complex_solution(
                &mut document,
                results.iter().map(|point| ComplexSolutionPoint {
                    node_names: &point.node_names,
                    voltages: &point.voltages,
                    branch_names: &point.branch_names,
                    currents: &point.currents,
                }),
            )?;
            add_typed_artifact(&mut outcome, kind, ordinal, document)?;
            Ok(outcome)
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
            let frequencies = rspice_core::analysis::ac::ac_sweep_frequencies(
                *variation,
                *points,
                *start_freq,
                *stop_freq,
            );
            if frequencies.is_empty() {
                return Ok(DirectiveOutcome {
                    measurements: Vec::new(),
                    artifacts: Vec::new(),
                });
            }
            let results = engine.run_noise_named_with_input_source_and_abort(
                netlist,
                output_node,
                reference_node.as_deref(),
                input_source,
                &frequencies,
                engine.config().temperature,
                deadline,
            )?;
            if results.is_empty() {
                return Err(DirectiveFailure::NonFinite);
            }
            for point in &results {
                validate_pair(
                    "noise node voltage",
                    point.node_names.len(),
                    point.voltages.len(),
                )?;
                validate_pair(
                    "noise branch current",
                    point.branch_names.len(),
                    point.currents.len(),
                )?;
            }
            let columns: Vec<(String, &'static str, Vec<f64>)> = vec![
                (
                    "frequency".to_owned(),
                    "Hz",
                    results.iter().map(|point| point.frequency).collect(),
                ),
                (
                    // The engine's density is a power quantity (V^2/Hz);
                    // the declared unit is the amplitude density every
                    // commercial noise report uses.
                    "onoise".to_owned(),
                    "V/Hz0.5",
                    results
                        .iter()
                        .map(|point| point.output_noise_rms())
                        .collect(),
                ),
            ];
            let mut outcome = columns_outcome(kind, ordinal, columns)?;
            let mut document = analog_document(kind, ordinal).expect("noise is analog");
            document.point_count = results.len();
            document.axes.push(AxisDocument {
                name: "frequency".to_owned(),
                unit: Some(SignalUnit::Hertz),
                values: results.iter().map(|point| Some(point.frequency)).collect(),
            });
            append_complex_solution(
                &mut document,
                results.iter().map(|point| ComplexSolutionPoint {
                    node_names: &point.node_names,
                    voltages: &point.voltages,
                    branch_names: &point.branch_names,
                    currents: &point.currents,
                }),
            )?;
            document.signals.extend([
                analysis_scalar_signal(
                    "output_noise_density",
                    "Output noise density",
                    Some(SignalUnit::VoltSquaredPerHertz),
                    results
                        .iter()
                        .map(|point| Some(point.output_noise_density))
                        .collect(),
                ),
                analysis_scalar_signal(
                    "input_referred_noise_density",
                    "Input-referred noise density",
                    Some(SignalUnit::VoltSquaredPerHertz),
                    results
                        .iter()
                        .map(|point| Some(point.input_referred_density))
                        .collect(),
                ),
                analysis_scalar_signal(
                    "input_gain_squared",
                    "Input gain squared",
                    Some(SignalUnit::Dimensionless),
                    results
                        .iter()
                        .map(|point| Some(point.input_gain_squared))
                        .collect(),
                ),
            ]);
            append_noise_contributions(&mut document, &results);
            add_typed_artifact(&mut outcome, kind, ordinal, document)?;
            Ok(outcome)
        }
        // Directive classes are filtered before dispatch, so any other
        // variant reaching here is an executor logic error, not deck content.
        _ => Err(DirectiveFailure::Engine(SimulationError::Circuit(
            "unreachable directive class".to_owned(),
        ))),
    }
}

/// Converts named series columns into measurements plus one CSV artifact.
fn columns_outcome(
    kind: AnalysisKind,
    ordinal: usize,
    columns: Vec<(String, &'static str, Vec<f64>)>,
) -> Result<DirectiveOutcome, DirectiveFailure> {
    let mut measurements = Vec::new();
    for (name, unit, series) in &columns {
        if series.iter().any(|value| !value.is_finite()) {
            return Err(DirectiveFailure::NonFinite);
        }
        measurements.extend(Measurement::series(name.clone(), unit, series));
    }

    let rows = columns.first().map_or(0, |(_, _, series)| series.len());
    if columns.iter().any(|(_, _, series)| series.len() != rows) {
        return Err(DirectiveFailure::ResultDocument(
            "legacy result columns do not share a common point count".to_owned(),
        ));
    }
    let mut content = String::new();
    for (index, (name, _, _)) in columns.iter().enumerate() {
        if index > 0 {
            content.push(',');
        }
        content.push_str(name);
    }
    content.push('\n');
    for row in 0..rows {
        for (index, (_, _, series)) in columns.iter().enumerate() {
            if index > 0 {
                content.push(',');
            }
            content.push_str(&canonical_decimal(series[row]).ok_or(DirectiveFailure::NonFinite)?);
        }
        content.push('\n');
    }

    Ok(DirectiveOutcome {
        measurements,
        artifacts: vec![PendingArtifact {
            file_name: format!("{}-{}.csv", kind.as_str(), ordinal + 1),
            content_type: "text/csv",
            content,
        }],
    })
}

fn succeeded(
    kind: AnalysisKind,
    engine_build: &str,
    measurements: Vec<Measurement>,
    artifacts: Vec<PendingArtifact>,
    directive_count: usize,
) -> Execution {
    let mut manifest = serde_json::json!({
        "format": "rspice-result-v1",
        "analysis_kind": kind.as_str(),
        "engine": {"name": "rspice", "build": engine_build},
        "typed_result_schema": {
            "name": RESULT_DOCUMENT_SCHEMA,
            "version": RESULT_DOCUMENT_VERSION,
        },
        "directives": directive_count,
        "measurements": measurements
            .iter()
            .map(Measurement::to_manifest_value)
            .collect::<Vec<_>>(),
    });
    if artifacts
        .iter()
        .any(|artifact| artifact.content_type == FFT_RESULT_DOCUMENT_CONTENT_TYPE)
    {
        manifest["typed_fft_result_schema"] = serde_json::json!({
            "name": FFT_RESULT_DOCUMENT_SCHEMA,
            "version": FFT_RESULT_DOCUMENT_VERSION,
        });
    }
    let descriptors = artifacts
        .iter()
        .map(|artifact| EngineResultArtifactDescriptor {
            path: format!("results/{}", artifact.file_name),
            content_type: artifact.content_type.to_owned(),
        })
        .collect();
    Execution {
        response: EngineResponse::Succeeded {
            result_manifest: manifest,
            result_artifacts: descriptors,
        },
        artifacts,
    }
}

/// Maps every engine error onto the bounded failure vocabulary through the
/// engine's own stable descriptor, so new engine variants cannot silently
/// widen the wire surface.
fn failure_execution(error: &SimulationError) -> Execution {
    let code = match error.descriptor().code {
        rspice_core::engine::SimulationErrorCode::Aborted => "engine.time_limit".to_owned(),
        stable => format!("engine.{}", stable.as_str()),
    };
    Execution::failed(&code, &error.to_string())
}

/// Stages the complete response-gated artifact set, atomically replaces each
/// member, and restores every predecessor on a returned commit failure. The
/// caller emits the success manifest only after this function completes, so
/// an interrupted process never advertises a partial set.
pub fn write_artifacts(results_dir: &Path, artifacts: &[PendingArtifact]) -> Result<(), String> {
    validate_pending_artifact_budget(artifacts)
        .map_err(|_| "result artifact set exceeds the protocol budget".to_owned())?;
    let mut names = HashSet::new();
    for artifact in artifacts {
        let result_path = format!("results/{}", artifact.file_name);
        if !valid_result_path(&result_path) {
            return Err("result artifact set contains an invalid destination name".to_owned());
        }
        if !names.insert(artifact.file_name.to_ascii_lowercase()) {
            return Err("result artifact set contains duplicate destinations".to_owned());
        }
    }

    let options = AtomicArtifactOptions::new(Durability::SyncFileAndParent);
    let mut prepared = Vec::new();
    prepared
        .try_reserve_exact(artifacts.len())
        .map_err(|error| format!("cannot allocate result transaction: {error}"))?;
    for artifact in artifacts {
        let destination = results_dir.join(&artifact.file_name);
        let mut staged = AtomicArtifactFile::prepare(&destination, options)
            .map_err(|error| format!("failed to stage {}: {error}", destination.display()))?;
        staged
            .write_all(artifact.content.as_bytes())
            .map_err(|error| format!("failed to write {}: {error}", destination.display()))?;
        let staged = staged.prepare_for_commit().map_err(|error| {
            format!(
                "failed to synchronize staged artifact {}: {error}",
                destination.display()
            )
        })?;
        prepared.push(PreparedResultArtifact {
            destination,
            staged: Some(staged),
            predecessor: Predecessor::Absent,
            committed: false,
        });
    }

    for index in 0..prepared.len() {
        match capture_predecessor(&prepared[index].destination) {
            Ok(predecessor) => prepared[index].predecessor = predecessor,
            Err(error) => {
                return match cleanup_result_backups(&mut prepared) {
                    Ok(()) => Err(error),
                    Err(cleanup_error) => Err(format!(
                        "{error}; rollback snapshot cleanup also failed: {cleanup_error}"
                    )),
                };
            }
        }
    }

    for index in 0..prepared.len() {
        let staged = prepared[index]
            .staged
            .take()
            .expect("prepared result artifact is consumed once");
        if let Err(error) = staged.commit() {
            if commit_error_published(&error) {
                prepared[index].committed = true;
            }
            let rollback = rollback_result_transaction(&mut prepared);
            return Err(match rollback {
                Ok(()) => format!(
                    "failed to commit result artifact {}: {error}; the predecessor set was restored",
                    prepared[index].destination.display()
                ),
                Err(rollback_error) => format!(
                    "failed to commit result artifact {}: {error}; rollback also failed: {rollback_error}",
                    prepared[index].destination.display()
                ),
            });
        }
        prepared[index].committed = true;
    }

    cleanup_result_backups(&mut prepared)?;
    Ok(())
}

fn commit_error_published(error: &AtomicArtifactError<io::Error>) -> bool {
    matches!(
        error,
        AtomicArtifactError::Commit {
            destination_state: DestinationState::PublishedDurabilityUncertain,
            ..
        }
    )
}

struct PreparedResultArtifact {
    destination: PathBuf,
    staged: Option<PreparedAtomicArtifact>,
    predecessor: Predecessor,
    committed: bool,
}

enum Predecessor {
    Absent,
    File { backup: PathBuf },
    Directory,
}

impl Predecessor {
    fn remove_backup(&mut self) -> Result<(), String> {
        if let Self::File { backup } = self {
            remove_artifact_durably(&*backup)
                .map_err(|error| format!("failed to remove rollback backup: {error}"))?;
            *self = Self::Absent;
        }
        Ok(())
    }
}

fn cleanup_result_backups(members: &mut [PreparedResultArtifact]) -> Result<(), String> {
    let mut failures = Vec::new();
    for member in members.iter_mut() {
        if let Err(error) = member.predecessor.remove_backup() {
            failures.push(format!("{}: {error}", member.destination.display()));
        }
    }
    if let Err(error) = sync_result_parents(members) {
        failures.push(error);
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("; "))
    }
}

static NEXT_ROLLBACK_BACKUP: AtomicU64 = AtomicU64::new(0);

fn capture_predecessor(destination: &Path) -> Result<Predecessor, String> {
    let metadata = match std::fs::symlink_metadata(destination) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Predecessor::Absent),
        Err(error) => {
            return Err(format!(
                "cannot inspect predecessor {}: {error}",
                destination.display()
            ));
        }
    };
    if metadata.file_type().is_dir() {
        // A regular file cannot atomically replace a directory, so retaining
        // the directory is a safe way for commit to report an unchanged
        // destination. Other non-regular objects (devices, FIFOs, sockets)
        // can be replaced by rename on Unix and therefore must fail closed.
        return Ok(Predecessor::Directory);
    }
    if !metadata.file_type().is_file() {
        return Err(format!(
            "result predecessor {} is not a regular file or directory",
            destination.display()
        ));
    }
    let parent = destination
        .parent()
        .ok_or_else(|| "result destination has no parent".to_owned())?;
    let name = destination
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| "result destination name is not UTF-8".to_owned())?;
    for _ in 0..1_024 {
        let serial = NEXT_ROLLBACK_BACKUP.fetch_add(1, Ordering::Relaxed);
        let backup = parent.join(format!(
            ".{name}.rspice-adapter-rollback-{}-{serial}",
            std::process::id()
        ));
        match std::fs::hard_link(destination, &backup) {
            Ok(()) => return Ok(Predecessor::File { backup }),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(format!(
                    "cannot create exact rollback snapshot for {}: {error}",
                    destination.display()
                ));
            }
        }
    }
    Err("cannot allocate a unique result rollback backup".to_owned())
}

fn rollback_result_transaction(members: &mut [PreparedResultArtifact]) -> Result<(), String> {
    let mut failures = Vec::new();
    for member in members.iter_mut().rev() {
        let result = match &member.predecessor {
            Predecessor::Absent if member.committed => remove_artifact_durably(&member.destination),
            Predecessor::Absent | Predecessor::Directory => Ok(()),
            Predecessor::File { backup } => {
                if member.committed {
                    restore_artifact_durably(backup, &member.destination)
                } else {
                    remove_artifact_durably(backup)
                }
            }
        };
        if let Err(error) = result {
            failures.push(format!("{}: {error}", member.destination.display()));
        }
        member.staged.take();
    }
    if let Err(error) = sync_result_parents(members) {
        failures.push(error);
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("; "))
    }
}

fn sync_result_parents(members: &[PreparedResultArtifact]) -> Result<(), String> {
    let mut synchronized = HashSet::new();
    for member in members {
        let parent = member
            .destination
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();
        if synchronized.insert(parent) {
            sync_artifact_parent(&member.destination).map_err(|error| {
                format!(
                    "failed to synchronize result transaction directory for {}: {error}",
                    member.destination.display()
                )
            })?;
        }
    }
    Ok(())
}

#[cfg(test)]
fn publish_artifact<E>(
    destination: &Path,
    write: impl FnOnce(&mut dyn Write) -> Result<(), E>,
) -> Result<(), AtomicArtifactError<E>>
where
    E: std::error::Error + 'static,
{
    write_atomic(
        destination,
        AtomicArtifactOptions::new(Durability::SyncFileAndParent),
        write,
    )
}

#[cfg(test)]
mod artifact_publication_tests {
    use super::*;
    use rspice_output::stale_artifacts;
    use std::io;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct AlwaysAbort;

    impl AbortSignal for AlwaysAbort {
        fn is_aborted(&self) -> bool {
            true
        }
    }

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new(tag: &str) -> Self {
            let serial = NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "rspice-engine-adapter-{tag}-{}-{serial}",
                std::process::id()
            ));
            std::fs::create_dir(&path).expect("create adapter publication test directory");
            Self(path)
        }

        fn destination(&self) -> PathBuf {
            self.0.join("result.json")
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn seed(destination: &Path, preexisting: bool) {
        if preexisting {
            std::fs::write(destination, b"old complete artifact")
                .expect("seed existing adapter artifact");
        }
    }

    fn assert_old_or_absent(destination: &Path, preexisting: bool) {
        if preexisting {
            assert_eq!(
                std::fs::read(destination).expect("read preserved adapter artifact"),
                b"old complete artifact"
            );
        } else {
            assert!(
                !destination.exists(),
                "failed publication created a destination"
            );
        }
        assert!(
            stale_artifacts(destination)
                .expect("inspect adapter staging artifacts")
                .is_empty(),
            "failed publication left a staging artifact"
        );
    }

    #[test]
    fn artifact_write_failure_preserves_existing_or_absent_destination() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("failure");
            let destination = directory.destination();
            seed(&destination, preexisting);

            let error = publish_artifact(&destination, |writer| -> io::Result<()> {
                writer.write_all(b"partial replacement")?;
                Err(io::Error::other("injected adapter serialization failure"))
            })
            .expect_err("injected artifact write must fail");

            assert!(matches!(error, AtomicArtifactError::Write(_)));
            assert_old_or_absent(&destination, preexisting);
        }
    }

    #[test]
    fn artifact_success_replaces_existing_or_absent_destination() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("success");
            let destination = directory.destination();
            seed(&destination, preexisting);
            let artifact = PendingArtifact {
                file_name: "result.json".to_string(),
                content_type: "application/json",
                content: "{\"complete\":true}\n".to_string(),
            };

            write_artifacts(&directory.0, &[artifact]).expect("publish adapter artifact");

            assert_eq!(
                std::fs::read(&destination).expect("read published adapter artifact"),
                b"{\"complete\":true}\n"
            );
            assert!(
                stale_artifacts(&destination)
                    .expect("inspect adapter staging artifacts")
                    .is_empty(),
                "successful publication left a staging artifact"
            );
        }
    }

    #[test]
    fn result_artifact_count_and_byte_budgets_accept_only_the_exact_boundaries() {
        assert!(
            validate_artifact_budget(MAX_ENGINE_RESULT_ARTIFACTS, [MAX_ENGINE_ARTIFACT_BYTES])
                .is_ok()
        );
        assert!(matches!(
            validate_artifact_budget(MAX_ENGINE_RESULT_ARTIFACTS + 1, [0]),
            Err(DirectiveFailure::ResultArtifactLimit)
        ));
        assert!(matches!(
            validate_artifact_budget(1, [MAX_ENGINE_ARTIFACT_BYTES + 1]),
            Err(DirectiveFailure::ResultArtifactBytes)
        ));
        assert!(matches!(
            validate_artifact_budget(1, [0]),
            Err(DirectiveFailure::ResultArtifactBytes)
        ));
    }

    #[test]
    fn result_artifact_destinations_are_single_component_and_case_unique() {
        let directory = TestDirectory::new("destination-validation");
        for invalid in [
            "../escape.json",
            "sub/escape.json",
            "sub\\escape.json",
            "C:escape",
        ] {
            let artifact = PendingArtifact {
                file_name: invalid.to_owned(),
                content_type: "application/json",
                content: "complete".to_owned(),
            };
            let error = write_artifacts(&directory.0, &[artifact])
                .expect_err("invalid artifact destination must fail closed");
            assert!(error.contains("invalid destination name"), "{error}");
        }

        let duplicate = [
            PendingArtifact {
                file_name: "Result.json".to_owned(),
                content_type: "application/json",
                content: "first".to_owned(),
            },
            PendingArtifact {
                file_name: "result.json".to_owned(),
                content_type: "application/json",
                content: "second".to_owned(),
            },
        ];
        let error = write_artifacts(&directory.0, &duplicate)
            .expect_err("case-folded duplicate destinations must fail closed");
        assert!(error.contains("duplicate destinations"), "{error}");
        assert!(
            std::fs::read_dir(&directory.0)
                .expect("read destination-validation directory")
                .next()
                .is_none()
        );
    }

    #[test]
    fn later_commit_failure_rolls_back_absent_and_preexisting_predecessors() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("set-rollback");
            let first = directory.0.join("first.json");
            seed(&first, preexisting);
            let invalid_second = directory.0.join("second.json");
            std::fs::create_dir(&invalid_second).expect("create commit-failing destination");
            let artifacts = [
                PendingArtifact {
                    file_name: "first.json".to_owned(),
                    content_type: "application/json",
                    content: "new first".to_owned(),
                },
                PendingArtifact {
                    file_name: "second.json".to_owned(),
                    content_type: "application/json",
                    content: "new second".to_owned(),
                },
            ];

            let error = write_artifacts(&directory.0, &artifacts)
                .expect_err("second destination must fail commit");
            assert!(error.contains("predecessor set was restored"), "{error}");
            assert_old_or_absent(&first, preexisting);
            assert!(invalid_second.is_dir());
            let names = std::fs::read_dir(&directory.0)
                .expect("read transaction directory")
                .map(|entry| entry.expect("read transaction entry").file_name())
                .collect::<Vec<_>>();
            assert!(
                names
                    .iter()
                    .all(|name| !name.to_string_lossy().contains("rspice-adapter-rollback")),
                "rollback backup remained: {names:?}"
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn non_regular_predecessors_fail_before_any_destination_is_committed() {
        use std::os::unix::net::UnixListener;

        let directory = TestDirectory::new("non-regular-predecessor");
        let first = directory.0.join("first.json");
        std::fs::write(&first, b"old first").expect("seed first predecessor");
        let socket = directory.0.join("second.json");
        let _listener = UnixListener::bind(&socket).expect("bind predecessor socket");
        let artifacts = [
            PendingArtifact {
                file_name: "first.json".to_owned(),
                content_type: "application/json",
                content: "new first".to_owned(),
            },
            PendingArtifact {
                file_name: "second.json".to_owned(),
                content_type: "application/json",
                content: "new second".to_owned(),
            },
        ];

        let error = write_artifacts(&directory.0, &artifacts)
            .expect_err("a socket predecessor must fail closed");
        assert!(error.contains("not a regular file or directory"), "{error}");
        assert_eq!(
            std::fs::read(&first).expect("read untouched first predecessor"),
            b"old first"
        );
        assert!(socket.exists());
    }

    #[test]
    fn durability_uncertain_commit_errors_are_treated_as_published_for_rollback() {
        let error = AtomicArtifactError::Commit {
            operation: rspice_output::CommitOperation::SyncParent,
            destination_state: DestinationState::PublishedDurabilityUncertain,
            recovery_path: None,
            source: io::Error::other("injected parent sync failure"),
        };
        assert!(commit_error_published(&error));

        let unchanged = AtomicArtifactError::Commit {
            operation: rspice_output::CommitOperation::Replace,
            destination_state: DestinationState::Unchanged,
            recovery_path: None,
            source: io::Error::other("injected replace failure"),
        };
        assert!(!commit_error_published(&unchanged));
    }

    #[test]
    fn cancelled_transient_fft_returns_no_partial_directive_outcome() {
        let netlist = Netlist::parse_validated(
            "cancelled transient FFT\n\
             V1 out 0 SIN(0 1 1k)\n\
             R1 out 0 1k\n\
             .tran 1u 1m\n\
             .fft v(out) np=8\n\
             .end\n",
        )
        .expect("cancelled FFT fixture parses");
        let directive = netlist
            .analyses
            .iter()
            .find(|directive| matches!(directive, AnalysisCommand::Tran { .. }))
            .expect("transient directive");
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Ngspice,
            ..SimulationConfig::default()
        });

        match run_directive(
            &engine,
            &netlist,
            directive,
            AnalysisKind::Transient,
            0,
            &AlwaysAbort,
        ) {
            Err(DirectiveFailure::Engine(SimulationError::Aborted)) => {}
            Err(_) => panic!("cancellation returned the wrong bounded failure"),
            Ok(_) => panic!("cancellation returned a partially completed directive outcome"),
        }
    }
}
