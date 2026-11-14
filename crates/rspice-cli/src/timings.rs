//! `--timings`: where `run` and `check` spend their wall time, as JSON.
//!
//! The front end is timed here at its phase boundaries; the engine records
//! elaboration, operating-point and transient phases into the collector this
//! run hands it through its configuration. Without `--timings` none of this
//! exists and no clock is read for it.

use crate::cli::CliError;
use rspice_core::config::{PhaseTimingRecord, PhaseTimings};
use serde_json::{Value as Json, json};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// The JSON keys `--timings` writes, for the command-line help.
pub(crate) const TIMINGS_HELP: &str = "\
Write phase timings to PATH as JSON (`-` writes them to stderr). Times are \
wall-clock seconds; result files are identical with and without this flag.

Top level: schema (\"rspice-timings/1\"), command, netlist, total_wall_s; \
expand_s (reading the deck with its includes and libraries, and .ALTER/.DATA \
expansion), parse_s, preflight_s (run planning and request checks, any \
elaboration they do included), flatten_s and build_s (every circuit \
elaboration: hierarchy flattening, and the rest of the build), \
circuit_builds, output_s (writing result files); operating_point and \
transient summed over the run; analyses, one entry per analysis run.

operating_point: wall_s, solves, newton_iterations.

transient (null without one): runs, wall_s (stepping loop, initial operating \
point excluded), device_evaluation_s and matrix_assembly_s (the Newton \
assemblies), linear_solve_s (the Newton solves; of which factor_s, solve_s \
and certificate_s, the backward-error check with its refinement and \
fallback), convergence_check_s (device update and convergence tests at each \
candidate), residual_proof_s (true-residual restamp), truncation_s, \
output_s (recording accepted samples), other_s (the rest of wall_s); \
accepted_steps, rejected_steps, newton_iterations, factorizations, \
refactorizations.

analyses[]: analysis, wall_s, flatten_s, build_s, circuit_builds, output_s, \
operating_point, transient.

`check` writes the same top level without analyses.";

/// One front-end phase the CLI times itself.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Phase {
    Expand,
    Parse,
    Preflight,
    /// A flattening the CLI runs outside an engine elaboration.
    Flatten,
    Output,
}

/// The timings of one `run` or `check` invocation.
#[derive(Debug)]
pub(crate) struct RunTimings {
    engine: Arc<PhaseTimings>,
    started: Instant,
    state: Mutex<State>,
}

#[derive(Debug, Default)]
struct State {
    expand: Duration,
    parse: Duration,
    preflight: Duration,
    flatten: Duration,
    output: Duration,
    engine: PhaseTimingRecord,
    analyses: Vec<AnalysisTiming>,
}

#[derive(Debug)]
struct AnalysisTiming {
    analysis: String,
    wall: Duration,
    output: Duration,
    engine: PhaseTimingRecord,
}

impl RunTimings {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            engine: Arc::default(),
            started: Instant::now(),
            state: Mutex::default(),
        })
    }

    /// The collector the engine records into.
    pub(crate) fn engine(&self) -> Arc<PhaseTimings> {
        Arc::clone(&self.engine)
    }

    fn with_state(&self, update: impl FnOnce(&mut State)) {
        if let Ok(mut state) = self.state.lock() {
            update(&mut state);
        }
    }

    /// Add `elapsed` to a front-end phase.
    pub(crate) fn add(&self, phase: Phase, elapsed: Duration) {
        self.with_state(|state| {
            let slot = match phase {
                Phase::Expand => &mut state.expand,
                Phase::Parse => &mut state.parse,
                Phase::Preflight => &mut state.preflight,
                Phase::Flatten => &mut state.flatten,
                Phase::Output => &mut state.output,
            };
            *slot += elapsed;
        });
    }

    /// Fold what the engine recorded since the last drain into the run total
    /// and hand it back.
    fn drain_engine(&self, state: &mut State) -> PhaseTimingRecord {
        let recorded = self.engine.take();
        accumulate(&mut state.engine, &recorded);
        recorded
    }

    /// Run one analysis, attributing what the engine records meanwhile to
    /// it. With several workers (`-j`) the attribution is approximate; the
    /// run totals are exact either way.
    fn analysis<T>(&self, name: String, run: impl FnOnce() -> T) -> T {
        let mut output_before = Duration::ZERO;
        self.with_state(|state| {
            self.drain_engine(state);
            output_before = state.output;
        });
        let started = Instant::now();
        let outcome = run();
        let wall = started.elapsed();
        self.with_state(|state| {
            let engine = self.drain_engine(state);
            let output = state.output.saturating_sub(output_before);
            state.analyses.push(AnalysisTiming {
                analysis: name,
                wall,
                output,
                engine,
            });
        });
        outcome
    }

    /// Write the report for `command` on `netlist` to `destination`.
    pub(crate) fn write(
        &self,
        destination: &Path,
        command: &str,
        netlist: &Path,
    ) -> Result<(), CliError> {
        let total = self.started.elapsed();
        let mut document = Json::Null;
        self.with_state(|state| {
            self.drain_engine(state);
            document = report(state, command, netlist, total);
        });
        let mut text =
            serde_json::to_string_pretty(&document).map_err(|error| CliError::InternalError {
                message: format!("failed to serialize --timings: {error}"),
            })?;
        text.push('\n');
        if destination.as_os_str() == "-" {
            use std::io::Write as _;
            std::io::stderr()
                .write_all(text.as_bytes())
                .map_err(|error| CliError::output_error(destination, error))
        } else {
            std::fs::write(destination, text)
                .map_err(|error| CliError::output_error(destination, error))
        }
    }
}

/// Time `run` as `phase` when timings are on; run it untimed otherwise.
pub(crate) fn time<T>(timings: Option<&RunTimings>, phase: Phase, run: impl FnOnce() -> T) -> T {
    let _timing = guard(timings, phase);
    run()
}

/// Time `phase` from now until the returned guard drops, whichever way its
/// scope ends. `None`, reading no clock, while timings are off.
pub(crate) fn guard(timings: Option<&RunTimings>, phase: Phase) -> Option<PhaseGuard<'_>> {
    timings.map(|timings| PhaseGuard {
        timings,
        phase,
        started: Instant::now(),
    })
}

/// A phase being timed; see [`guard`].
pub(crate) struct PhaseGuard<'a> {
    timings: &'a RunTimings,
    phase: Phase,
    started: Instant,
}

impl Drop for PhaseGuard<'_> {
    fn drop(&mut self) {
        self.timings.add(self.phase, self.started.elapsed());
    }
}

/// Run one analysis card, reported as `name` when timings are on.
pub(crate) fn analysis<T>(timings: Option<&RunTimings>, name: &str, run: impl FnOnce() -> T) -> T {
    match timings {
        None => run(),
        Some(timings) => timings.analysis(name.to_string(), run),
    }
}

fn accumulate(total: &mut PhaseTimingRecord, more: &PhaseTimingRecord) {
    total.flatten += more.flatten;
    total.elaboration += more.elaboration;
    total.circuit_builds += more.circuit_builds;
    total.operating_point += more.operating_point;
    total.operating_points += more.operating_points;
    total.operating_point_newton_iterations += more.operating_point_newton_iterations;
    total.transient_runs += more.transient_runs;
    total.transient_wall += more.transient_wall;
    total.transient_device_evaluation += more.transient_device_evaluation;
    total.transient_matrix_assembly += more.transient_matrix_assembly;
    total.transient_linear_solve += more.transient_linear_solve;
    total.transient_factor += more.transient_factor;
    total.transient_solve += more.transient_solve;
    total.transient_certificate += more.transient_certificate;
    total.transient_convergence_check += more.transient_convergence_check;
    total.transient_residual_proof += more.transient_residual_proof;
    total.transient_truncation += more.transient_truncation;
    total.transient_output += more.transient_output;
    total.transient_accepted_steps += more.transient_accepted_steps;
    total.transient_rejected_steps += more.transient_rejected_steps;
    total.transient_newton_iterations += more.transient_newton_iterations;
    total.transient_factorizations += more.transient_factorizations;
    total.transient_refactorizations += more.transient_refactorizations;
}

fn seconds(duration: Duration) -> f64 {
    duration.as_secs_f64()
}

fn operating_point(record: &PhaseTimingRecord) -> Json {
    json!({
        "wall_s": seconds(record.operating_point),
        "solves": record.operating_points,
        "newton_iterations": record.operating_point_newton_iterations,
    })
}

fn transient(record: &PhaseTimingRecord) -> Json {
    if record.transient_runs == 0 {
        return Json::Null;
    }
    let accounted = record.transient_device_evaluation
        + record.transient_matrix_assembly
        + record.transient_linear_solve
        + record.transient_convergence_check
        + record.transient_residual_proof
        + record.transient_truncation
        + record.transient_output;
    json!({
        "runs": record.transient_runs,
        "wall_s": seconds(record.transient_wall),
        "device_evaluation_s": seconds(record.transient_device_evaluation),
        "matrix_assembly_s": seconds(record.transient_matrix_assembly),
        "linear_solve_s": seconds(record.transient_linear_solve),
        "factor_s": seconds(record.transient_factor),
        "solve_s": seconds(record.transient_solve),
        "certificate_s": seconds(record.transient_certificate),
        "convergence_check_s": seconds(record.transient_convergence_check),
        "residual_proof_s": seconds(record.transient_residual_proof),
        "truncation_s": seconds(record.transient_truncation),
        "output_s": seconds(record.transient_output),
        "other_s": seconds(record.transient_wall.saturating_sub(accounted)),
        "accepted_steps": record.transient_accepted_steps,
        "rejected_steps": record.transient_rejected_steps,
        "newton_iterations": record.transient_newton_iterations,
        "factorizations": record.transient_factorizations,
        "refactorizations": record.transient_refactorizations,
    })
}

/// Flattening and the rest of elaboration, with the CLI's own flattening
/// counted as flattening.
fn elaboration(record: &PhaseTimingRecord, cli_flatten: Duration) -> (Json, Json) {
    (
        json!(seconds(record.flatten + cli_flatten)),
        json!(seconds(record.elaboration.saturating_sub(record.flatten))),
    )
}

fn report(state: &State, command: &str, netlist: &Path, total: Duration) -> Json {
    let (flatten, build) = elaboration(&state.engine, state.flatten);
    let mut document = json!({
        "schema": "rspice-timings/1",
        "command": command,
        "netlist": netlist.display().to_string(),
        "total_wall_s": seconds(total),
        "expand_s": seconds(state.expand),
        "parse_s": seconds(state.parse),
        "preflight_s": seconds(state.preflight),
        "flatten_s": flatten,
        "build_s": build,
        "circuit_builds": state.engine.circuit_builds,
        "output_s": seconds(state.output),
        "operating_point": operating_point(&state.engine),
        "transient": transient(&state.engine),
    });
    if command == "run"
        && let Some(object) = document.as_object_mut()
    {
        let analyses = state
            .analyses
            .iter()
            .map(|entry| {
                let (flatten, build) = elaboration(&entry.engine, Duration::ZERO);
                json!({
                    "analysis": entry.analysis,
                    "wall_s": seconds(entry.wall),
                    "flatten_s": flatten,
                    "build_s": build,
                    "circuit_builds": entry.engine.circuit_builds,
                    "output_s": seconds(entry.output),
                    "operating_point": operating_point(&entry.engine),
                    "transient": transient(&entry.engine),
                })
            })
            .collect::<Vec<_>>();
        object.insert("analyses".to_string(), Json::Array(analyses));
    }
    document
}
