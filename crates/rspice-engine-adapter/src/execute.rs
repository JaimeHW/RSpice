//! Deck execution: analysis-kind gating, engine invocation, measurement
//! extraction, and bounded result-artifact emission.

use std::collections::BTreeMap;
use std::path::Path;
use std::time::{Duration, Instant};

use rspice_core::abort_signal::AbortSignal;
use rspice_core::circuit::DeviceOpReport;
use rspice_core::engine::{SimulationConfig, SpiceDialect};
use rspice_core::netlist::{AnalysisCommand, DcSweepSpec, ElementKind};
use rspice_core::solver::SimulationResult;
use rspice_core::{Engine, Netlist, SimulationError};
use serde::Deserialize;
use serde_json::Value;

use crate::document::CircuitContent;
use crate::measure::{Measurement, canonical_decimal, finalize_measurements, measurement_name};
use crate::result_document::{
    AnalogAnalysisKind, AnalogResultDocument, AnalogSignalKind, AxisDocument, ComplexSample,
    DeviceStateSeries, RESULT_DOCUMENT_CONTENT_TYPE, RESULT_DOCUMENT_SCHEMA,
    RESULT_DOCUMENT_VERSION, SignalDocument, SignalOwner, SignalUnit, SignalValues,
};
use crate::wire::{EngineResponse, EngineResultArtifactDescriptor};

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
        }
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
}

impl From<SimulationError> for DirectiveFailure {
    fn from(error: SimulationError) -> Self {
        Self::Engine(error)
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
    deadline: &SolveDeadline,
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
                add_typed_artifact(&mut outcome, kind, ordinal, document)?;
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
    let manifest = serde_json::json!({
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

/// Writes every staged artifact under the pre-created `results/` directory.
/// A write failure here is a sandbox-authority fault, surfaced as a process
/// error rather than a customer outcome.
pub fn write_artifacts(results_dir: &Path, artifacts: &[PendingArtifact]) -> Result<(), String> {
    for artifact in artifacts {
        let path = results_dir.join(&artifact.file_name);
        std::fs::write(&path, artifact.content.as_bytes())
            .map_err(|error| format!("failed to write {}: {error}", path.display()))?;
    }
    Ok(())
}
