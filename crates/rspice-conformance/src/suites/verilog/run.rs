//! Running one case through one engine, and comparing two engines on it.
//!
//! # The RSpice arm refuses on purpose
//!
//! [`run_case`] dispatches on [`VerilogEngine`], and the
//! [`VerilogEngine::Rspice`] arm returns
//! [`RunError::RspiceExecutionUnimplemented`]. That is not a placeholder to be
//! tidied up later; it is the whole reason this module has the shape it has.
//!
//! Nothing in RSpice executes digital Verilog yet. The plumbing that will
//! compare RSpice against an oracle is nevertheless finished and exercised
//! today, because [`compare_engines`] does not care which engines it is given:
//! it runs both, parses both traces, and diffs them. Two oracles go through it
//! now; RSpice and an oracle go through the same code the moment `run_case`
//! learns the third arm. Nothing about the comparison has to be invented at
//! that point, which is when inventing it would be most likely to go wrong —
//! the temptation, with a fresh implementation in hand and a deadline, is to
//! build the comparison that makes it pass.
//!
//! The refusal is a named variant rather than a panic or a `None` so that the
//! test asserting it (`rspice_execution_is_not_implemented_yet`) fails loudly
//! when the arm is implemented. That failing test is the checklist item saying
//! the harness now has a third participant.
//!
//! # Isolation
//!
//! Every case-and-engine pair gets its own scratch directory, so nothing is
//! shared between two runs but the read-only corpus. Each external tool runs as
//! a child process under a hard timeout with its output captured to files; a
//! design that fails to terminate is killed and reported, and cannot take the
//! suite with it. This is the same discipline as the ngspice case runner, with
//! one simplification: that suite has to fork a subprocess deliberately because
//! the deck runs *inside* the test process, whereas here the simulator is
//! already a separate program.

use std::fs;
use std::path::{Path, PathBuf};

use super::corpus::Case;
use super::oracle::{
    OracleAvailability, OracleTools, ProcessError, ProcessOutcome, VerilogEngine, run_process,
    scratch_dir,
};
use super::testbench;
use super::trace::{Divergence, Trace, TraceError, compare_traces, parse_trace};

/// Default ceiling for one compile or one simulation.
///
/// Generous because Verilator compiles the design to C++ and then invokes a C++
/// compiler on it, which dominates everything else this suite does. Small
/// enough that a runaway design is caught in a coffee break rather than a CI
/// timeout.
pub const DEFAULT_TIMEOUT_MS: u64 = 300_000;

/// One engine's answer for one case.
#[derive(Debug, Clone)]
pub struct RunOutcome {
    pub engine: VerilogEngine,
    pub case: String,
    pub trace: Trace,
    pub compile_ms: u128,
    pub simulate_ms: u128,
}

/// Everything that can stop a case from producing a trace.
#[derive(Debug, Clone)]
pub enum RunError {
    /// RSpice cannot execute digital Verilog yet.
    ///
    /// Removed by W2.3, the CFG interpreter. Until then this is the honest
    /// answer, and it is a distinct variant so that nothing can mistake it for
    /// a tool being absent or a design being broken.
    RspiceExecutionUnimplemented,
    /// An oracle was asked for but is not installed.
    OracleUnavailable(Box<OracleAvailability>),
    /// The manifest does not list this engine for this case — the X/Z case
    /// against Verilator, for instance. A refusal rather than a skip: asking is
    /// a mistake in the caller, and a silent skip would hide it.
    CaseNotAdmitted { case: String, engine: VerilogEngine },
    /// The engine's compile step failed.
    Compile {
        engine: VerilogEngine,
        command: String,
        detail: String,
    },
    /// The compiled design failed to run.
    Simulate {
        engine: VerilogEngine,
        command: String,
        detail: String,
    },
    /// The run produced output that is not a trace.
    Trace {
        engine: VerilogEngine,
        error: TraceError,
    },
    /// The harness itself failed.
    Harness(String),
}

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RspiceExecutionUnimplemented => write!(
                f,
                "RSpice does not execute digital Verilog yet; the oracle harness is in place \
                 and this arm is enabled by W2.3 (the CFG interpreter)"
            ),
            Self::OracleUnavailable(availability) => write!(f, "{}", availability.diagnostic()),
            Self::CaseNotAdmitted { case, engine } => write!(
                f,
                "the manifest does not list {engine} as an oracle for '{case}'"
            ),
            Self::Compile {
                engine,
                command,
                detail,
            } => write!(f, "{engine} failed to compile ({command}): {detail}"),
            Self::Simulate {
                engine,
                command,
                detail,
            } => write!(f, "{engine} failed to simulate ({command}): {detail}"),
            Self::Trace { engine, error } => {
                write!(f, "{engine} produced no usable trace: {error}")
            }
            Self::Harness(detail) => write!(f, "harness failure: {detail}"),
        }
    }
}

impl std::error::Error for RunError {}

/// Run one case through one engine.
///
/// `tools` is the resolved toolchain for `engine`, or `None` for an engine that
/// needs none. `workspace` is a directory the caller owns and can delete;
/// intermediate build products go under it.
pub fn run_case(
    case: &Case,
    engine: VerilogEngine,
    tools: Option<&OracleTools>,
    workspace: &Path,
    timeout_ms: u64,
) -> Result<RunOutcome, RunError> {
    match engine {
        // The one arm that is deliberately absent. See the module docs.
        VerilogEngine::Rspice => Err(RunError::RspiceExecutionUnimplemented),
        VerilogEngine::Icarus | VerilogEngine::Verilator => {
            if !case.admits(engine) {
                return Err(RunError::CaseNotAdmitted {
                    case: case.name.clone(),
                    engine,
                });
            }
            let tools = tools.ok_or_else(|| {
                RunError::OracleUnavailable(Box::new(OracleAvailability::NotAnOracle(engine)))
            })?;
            let bench_dir = prepare(case, engine, workspace)?;
            match engine {
                VerilogEngine::Icarus => run_icarus(case, tools, &bench_dir, timeout_ms),
                VerilogEngine::Verilator => run_verilator(case, tools, &bench_dir, timeout_ms),
                VerilogEngine::Rspice => Err(RunError::RspiceExecutionUnimplemented),
            }
        }
    }
}

/// Write the generated testbench into a fresh scratch directory.
fn prepare(case: &Case, engine: VerilogEngine, workspace: &Path) -> Result<PathBuf, RunError> {
    let dir = scratch_dir(workspace, &case.name, engine);
    fs::create_dir_all(&dir)
        .map_err(|err| RunError::Harness(format!("cannot create '{}': {err}", dir.display())))?;
    let bench = testbench::render(&case.stimulus, &case.name);
    let path = dir.join(testbench::FILE_NAME);
    fs::write(&path, bench)
        .map_err(|err| RunError::Harness(format!("cannot write '{}': {err}", path.display())))?;
    Ok(dir)
}

fn run_icarus(
    case: &Case,
    tools: &OracleTools,
    dir: &Path,
    timeout_ms: u64,
) -> Result<RunOutcome, RunError> {
    let compiler = program(tools, "compiler", VerilogEngine::Icarus)?;
    let runtime = program(tools, "runtime", VerilogEngine::Icarus)?;
    let object = dir.join("design.vvp");

    // `-g2012` because the corpus uses constructs (notably `always @(*)`) that
    // predate SystemVerilog but postdate the 1995 default Icarus still applies.
    let compile_args = vec![
        "-g2012".to_string(),
        "-o".to_string(),
        path_arg(&object),
        "-s".to_string(),
        testbench::TOP_MODULE.to_string(),
        path_arg(&case.source),
        path_arg(&dir.join(testbench::FILE_NAME)),
    ];
    let compiled = run_process(&compiler.program, &compile_args, Some(dir), timeout_ms)
        .map_err(|err| compile_error(VerilogEngine::Icarus, &compiler.program, &err))?;
    if !compiled.success {
        return Err(RunError::Compile {
            engine: VerilogEngine::Icarus,
            command: compiler.program.clone(),
            detail: failure_detail(&compiled),
        });
    }

    let simulated = run_process(
        &runtime.program,
        &[path_arg(&object)],
        Some(dir),
        timeout_ms,
    )
    .map_err(|err| simulate_error(VerilogEngine::Icarus, &runtime.program, &err))?;
    if !simulated.success {
        return Err(RunError::Simulate {
            engine: VerilogEngine::Icarus,
            command: runtime.program.clone(),
            detail: failure_detail(&simulated),
        });
    }

    outcome(case, VerilogEngine::Icarus, &compiled, &simulated)
}

fn run_verilator(
    case: &Case,
    tools: &OracleTools,
    dir: &Path,
    timeout_ms: u64,
) -> Result<RunOutcome, RunError> {
    let compiler = program(tools, "compiler", VerilogEngine::Verilator)?;
    let executable_name = "rspice_verilog_sim";

    // `--binary` builds a standalone executable; `--timing` is what makes the
    // testbench's `#` delays and free-running clock work at all, and Verilator
    // silently produces a different design without it. `-Wno-fatal` keeps
    // lint opinions -- width inference, unoptimisable flattening -- from
    // failing a build whose semantics are the thing under test; they go to
    // stderr, which never reaches the trace.
    let compile_args = vec![
        "--binary".to_string(),
        "--timing".to_string(),
        "-Wno-fatal".to_string(),
        "--Mdir".to_string(),
        path_arg(dir),
        "--top-module".to_string(),
        testbench::TOP_MODULE.to_string(),
        "-o".to_string(),
        executable_name.to_string(),
        path_arg(&case.source),
        path_arg(&dir.join(testbench::FILE_NAME)),
    ];
    let compiled = run_process(&compiler.program, &compile_args, Some(dir), timeout_ms)
        .map_err(|err| compile_error(VerilogEngine::Verilator, &compiler.program, &err))?;
    if !compiled.success {
        return Err(RunError::Compile {
            engine: VerilogEngine::Verilator,
            command: compiler.program.clone(),
            detail: failure_detail(&compiled),
        });
    }

    let executable = dir.join(format!("{executable_name}{}", std::env::consts::EXE_SUFFIX));
    if !executable.is_file() {
        return Err(RunError::Compile {
            engine: VerilogEngine::Verilator,
            command: compiler.program.clone(),
            detail: format!(
                "reported success but produced no executable at '{}'; {}",
                executable.display(),
                failure_detail(&compiled)
            ),
        });
    }
    let simulated = run_process(&path_arg(&executable), &[], Some(dir), timeout_ms)
        .map_err(|err| simulate_error(VerilogEngine::Verilator, executable_name, &err))?;
    if !simulated.success {
        return Err(RunError::Simulate {
            engine: VerilogEngine::Verilator,
            command: executable_name.to_string(),
            detail: failure_detail(&simulated),
        });
    }

    outcome(case, VerilogEngine::Verilator, &compiled, &simulated)
}

fn outcome(
    case: &Case,
    engine: VerilogEngine,
    compiled: &ProcessOutcome,
    simulated: &ProcessOutcome,
) -> Result<RunOutcome, RunError> {
    let trace =
        parse_trace(&simulated.stdout).map_err(|error| RunError::Trace { engine, error })?;
    Ok(RunOutcome {
        engine,
        case: case.name.clone(),
        trace,
        compile_ms: compiled.duration_ms,
        simulate_ms: simulated.duration_ms,
    })
}

fn program<'a>(
    tools: &'a OracleTools,
    role: &str,
    engine: VerilogEngine,
) -> Result<&'a super::oracle::ResolvedProgram, RunError> {
    tools.program(role).ok_or_else(|| {
        RunError::Harness(format!("{engine} toolchain is missing its {role} program"))
    })
}

fn compile_error(engine: VerilogEngine, command: &str, err: &ProcessError) -> RunError {
    RunError::Compile {
        engine,
        command: command.to_string(),
        detail: err.to_string(),
    }
}

fn simulate_error(engine: VerilogEngine, command: &str, err: &ProcessError) -> RunError {
    RunError::Simulate {
        engine,
        command: command.to_string(),
        detail: err.to_string(),
    }
}

fn failure_detail(outcome: &ProcessOutcome) -> String {
    format!(
        "exited with {}; stdout: {}; stderr: {}",
        outcome.status,
        clip(&outcome.stdout),
        clip(&outcome.stderr)
    )
}

fn clip(value: &str) -> String {
    const LIMIT: usize = 600;
    let value = value.trim();
    if value.chars().count() <= LIMIT {
        return value.to_string();
    }
    let clipped = value.chars().take(LIMIT).collect::<String>();
    format!("{clipped}...")
}

fn path_arg(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

/// Run one case through two engines and diff the traces.
///
/// This is the comparison contract in one function, and it is engine-agnostic
/// on purpose: it is what compares two oracles today and what will compare
/// RSpice against an oracle when [`run_case`] grows its third arm. An empty
/// result means the two engines produced identical observations.
pub fn compare_engines(
    case: &Case,
    left: (VerilogEngine, Option<&OracleTools>),
    right: (VerilogEngine, Option<&OracleTools>),
    workspace: &Path,
    timeout_ms: u64,
) -> Result<Vec<Divergence>, RunError> {
    let left_outcome = run_case(case, left.0, left.1, workspace, timeout_ms)?;
    let right_outcome = run_case(case, right.0, right.1, workspace, timeout_ms)?;
    Ok(compare_traces(&left_outcome.trace, &right_outcome.trace))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::suites::verilog::corpus::Corpus;
    use crate::suites::verilog::corpus_dir;

    fn any_case() -> Case {
        let corpus = Corpus::load(&corpus_dir()).expect("the vendored corpus loads");
        corpus.case("c17").expect("c17 is in the corpus").clone()
    }

    /// The refusal that W2.3 removes.
    ///
    /// When the CFG interpreter can execute this corpus, this test fails. That
    /// failure is the point: it is the one place that has to be revisited, and
    /// it says so rather than letting a newly working arm go unnoticed behind a
    /// harness that still reports "not implemented".
    #[test]
    fn rspice_execution_is_not_implemented_yet() {
        let case = any_case();
        let workspace = std::env::temp_dir().join("rspice-verilog-refusal-probe");

        let error = run_case(&case, VerilogEngine::Rspice, None, &workspace, 1_000)
            .expect_err("RSpice cannot execute digital Verilog yet");

        assert!(
            matches!(error, RunError::RspiceExecutionUnimplemented),
            "expected the named refusal, got {error:?}"
        );
        let message = error.to_string();
        assert!(
            message.contains("does not execute digital Verilog yet"),
            "{message}"
        );
        assert!(message.contains("W2.3"), "{message}");
        // Nothing was written: the refusal happens before any scratch directory
        // is created, so it cannot leave debris on a machine that only ever
        // reaches this arm.
        assert!(
            !workspace.exists(),
            "the refusal must not touch the filesystem"
        );
    }

    #[test]
    fn comparison_plumbing_propagates_the_refusal_rather_than_reporting_agreement() {
        let case = any_case();
        let workspace = std::env::temp_dir().join("rspice-verilog-refusal-probe-compare");

        // The failure this guards against is the plumbing treating an
        // unrunnable engine as producing an empty trace, which would then
        // "agree" with anything.
        let error = compare_engines(
            &case,
            (VerilogEngine::Rspice, None),
            (VerilogEngine::Rspice, None),
            &workspace,
            1_000,
        )
        .expect_err("a comparison involving RSpice cannot succeed yet");

        assert!(
            matches!(error, RunError::RspiceExecutionUnimplemented),
            "{error:?}"
        );
    }

    #[test]
    fn an_oracle_the_manifest_excludes_is_refused_rather_than_skipped() {
        let corpus = Corpus::load(&corpus_dir()).expect("corpus loads");
        let case = corpus
            .case("xz_propagation")
            .expect("the four-state case is in the corpus");
        let workspace = std::env::temp_dir().join("rspice-verilog-admission-probe");

        // Verilator is two-state and the manifest says so. Asking anyway is a
        // caller mistake, and it must not quietly turn into a pass.
        let error = run_case(case, VerilogEngine::Verilator, None, &workspace, 1_000)
            .expect_err("the manifest excludes Verilator here");

        assert!(
            matches!(&error, RunError::CaseNotAdmitted { engine, .. } if *engine == VerilogEngine::Verilator),
            "{error:?}"
        );
    }

    #[test]
    fn an_admitted_oracle_without_a_toolchain_reports_unavailability() {
        let case = any_case();
        let workspace = std::env::temp_dir().join("rspice-verilog-toolless-probe");

        let error = run_case(&case, VerilogEngine::Icarus, None, &workspace, 1_000)
            .expect_err("no toolchain was supplied");

        assert!(matches!(error, RunError::OracleUnavailable(_)), "{error:?}");
    }
}
