//! Deck execution: analysis-kind gating, engine invocation, measurement
//! extraction, and bounded result-artifact emission.

use std::path::Path;
use std::time::{Duration, Instant};

use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::{SimulationConfig, SpiceDialect};
use rspice_core::netlist::AnalysisCommand;
use rspice_core::{Engine, Netlist, SimulationError};
use serde::Deserialize;
use serde_json::Value;

use crate::document::CircuitContent;
use crate::measure::{Measurement, canonical_decimal, finalize_measurements, measurement_name};
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
    // qualification corpus measures; changing it is an engine_build change.
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
}

impl From<SimulationError> for DirectiveFailure {
    fn from(error: SimulationError) -> Self {
        Self::Engine(error)
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
            let result = engine.run_dc_op_with_abort(netlist, deadline)?;
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
            Ok(DirectiveOutcome {
                measurements,
                artifacts,
            })
        }
        AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            sweep2,
            ..
        } => {
            let points = match sweep2 {
                None => engine
                    .run_dc_sweep_with_abort(netlist, source, *start, *stop, *step, deadline)?,
                Some(outer) => engine.run_dc_sweep2_with_abort(
                    netlist,
                    source,
                    *start,
                    *stop,
                    *step,
                    Some(outer),
                    deadline,
                )?,
            };
            if points.is_empty() {
                return Err(DirectiveFailure::NonFinite);
            }
            let sweep: Vec<f64> = points.iter().map(|(value, _)| *value).collect();
            let first = &points[0].1;
            let mut columns: Vec<(String, &'static str, Vec<f64>)> = Vec::new();
            columns.push((
                format!("sweep({})", source.to_ascii_lowercase()),
                "V",
                sweep,
            ));
            for (index, name) in first.node_names.iter().enumerate().skip(1) {
                let series: Vec<f64> = points
                    .iter()
                    .map(|(_, result)| result.node_voltages[index])
                    .collect();
                columns.push((measurement_name("v", name), "V", series));
            }
            for (index, name) in first.branch_names.iter().enumerate() {
                let series: Vec<f64> = points
                    .iter()
                    .map(|(_, result)| result.branch_currents[index])
                    .collect();
                columns.push((measurement_name("i", name), "A", series));
            }
            columns_outcome(kind, ordinal, columns)
        }
        AnalysisCommand::Tran {
            step,
            stop,
            max_step,
            ..
        } => {
            // ngspice's default transient print ceiling: (tstop)/50 unless
            // the deck asks for a finer explicit ceiling.
            let ceiling = max_step.unwrap_or_else(|| (*stop / 50.0).max(*step));
            let result = engine.run_tran_with_abort(netlist, *stop, ceiling, deadline)?;
            if result.time.is_empty() {
                return Err(DirectiveFailure::NonFinite);
            }
            if result.time.len() > MAX_SERIES_SAMPLES {
                return Err(DirectiveFailure::SeriesBudget);
            }
            let mut columns: Vec<(String, &'static str, Vec<f64>)> = Vec::new();
            columns.push(("time".to_owned(), "s", result.time.clone()));
            for (index, name) in result.node_names.iter().enumerate() {
                columns.push((
                    measurement_name("v", name),
                    "V",
                    result.voltages[index].clone(),
                ));
            }
            for (index, name) in result.branch_names.iter().enumerate() {
                columns.push((
                    measurement_name("i", name),
                    "A",
                    result.branch_currents[index].clone(),
                ));
            }
            columns_outcome(kind, ordinal, columns)
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
            let mut columns: Vec<(String, &'static str, Vec<f64>)> = Vec::new();
            columns.push((
                "frequency".to_owned(),
                "Hz",
                results.iter().map(|point| point.frequency).collect(),
            ));
            let first = &results[0];
            for (index, name) in first.node_names.iter().enumerate() {
                let magnitude: Vec<f64> = results
                    .iter()
                    .map(|point| point.voltages[index].norm())
                    .collect();
                let phase: Vec<f64> = results
                    .iter()
                    .map(|point| point.voltages[index].arg().to_degrees())
                    .collect();
                columns.push((measurement_name("vm", name), "V", magnitude));
                columns.push((measurement_name("vp", name), "deg", phase));
            }
            columns_outcome(kind, ordinal, columns)
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
            columns_outcome(kind, ordinal, columns)
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
