//! Running one case through one engine, and comparing two engines on it.
//!
//! # The RSpice arm
//!
//! [`run_case`] dispatches on [`VerilogEngine`], and the
//! [`VerilogEngine::Rspice`] arm compiles the case's design through the
//! Verilog-A front end and runs it on the native digital host. It goes through
//! [`compare_engines`] exactly as an oracle does — same trace grammar, same
//! parser, same comparator — because that comparison was built and exercised
//! against two oracles before this arm existed. Nothing about it had to be
//! invented once there was an implementation in hand, which is when inventing
//! it would have been most likely to go wrong: the temptation with a fresh
//! implementation and a deadline is to build the comparison that makes it pass.
//!
//! In particular the arm does not hand the comparator a [`Trace`] it built
//! directly. It renders the same text the generated testbench makes an oracle
//! print and hands *that* to [`parse_trace`], so a formatting difference is a
//! failure here rather than a tolerance in the comparator.
//!
//! ## What it refuses
//!
//! A construct the front end cannot compile, or a design the host cannot run,
//! becomes [`RunError::RspiceRefused`] carrying the engine's own diagnostic.
//! That is a distinct variant from an oracle being absent and from a design
//! being broken, and it is never a trace: a corpus case RSpice cannot execute
//! must not be able to pass by producing nothing.
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
    /// RSpice refused to compile or run this case, and said why.
    ///
    /// A distinct variant so that nothing can mistake it for a tool being
    /// absent or a design being broken: it is the system under test declining
    /// a construct, which is the outcome the fail-closed rule demands when the
    /// alternative is a plausible wrong answer.
    RspiceRefused {
        /// The case that was refused.
        case: String,
        /// The engine's own diagnostic.
        detail: String,
    },
    /// The build has no digital Verilog execution compiled in.
    ///
    /// The `verilog-digital` feature forwards `rspice-core/veriloga`, which is
    /// what links the front end and the host. Without it there is nothing to
    /// call, and saying so is better than reporting an empty trace that would
    /// agree with anything.
    RspiceNotCompiledIn,
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
            Self::RspiceRefused { case, detail } => {
                write!(f, "RSpice refused case '{case}': {detail}")
            }
            Self::RspiceNotCompiledIn => write!(
                f,
                "this build has no digital Verilog execution; enable the `verilog-digital` \
                 feature, which forwards `rspice-core/veriloga`"
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
        // The system under test. It needs no toolchain and no scratch
        // directory: the front end and the host are compiled into this binary.
        VerilogEngine::Rspice => run_rspice(case),
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
                VerilogEngine::Rspice => run_rspice(case),
            }
        }
    }
}

/// Compile one case through the Verilog-A front end and run it on the native
/// digital host.
///
/// The stimulus is translated rather than reused: `rspice-core` cannot see this
/// crate's types, and a conformance suite that handed the engine its own case
/// representation would be testing the engine against the harness's idea of a
/// stimulus rather than against the `.stim` file both oracles are given.
#[cfg(feature = "verilog-digital")]
fn run_rspice(case: &Case) -> Result<RunOutcome, RunError> {
    use rspice_core::xspice::verilog::{
        DigitalClock, DigitalPort, DigitalStimulus, run_digital_verilog,
    };
    use std::time::Instant;

    let source = fs::read_to_string(&case.source).map_err(|err| {
        RunError::Harness(format!("cannot read '{}': {err}", case.source.display()))
    })?;

    let port = |port: &super::corpus::CasePort| DigitalPort {
        name: port.name.clone(),
        width: port.width,
    };
    let stimulus = DigitalStimulus {
        module: Some(case.stimulus.module.clone()),
        inputs: case
            .stimulus
            .driven_inputs()
            .into_iter()
            .map(port)
            .collect(),
        outputs: case
            .stimulus
            .observed_outputs()
            .into_iter()
            .map(port)
            .collect(),
        clock: case.stimulus.clock.as_ref().map(|clock| DigitalClock {
            port: clock.port.clone(),
            half_period: clock.half_period,
        }),
        step: case.stimulus.step,
        settle: case.stimulus.settle,
        vectors: case.stimulus.vectors.clone(),
    };

    let started = Instant::now();
    let report =
        run_digital_verilog(&source, &stimulus).map_err(|error| RunError::RspiceRefused {
            case: case.name.clone(),
            detail: error.to_string(),
        })?;
    let simulate_ms = started.elapsed().as_millis();

    // Rendered to the same text an oracle prints and read back with the same
    // parser, rather than assembled into a `Trace` directly. A `Trace` built
    // here would be compared against one that had been through `parse_trace`,
    // and the difference between the two is exactly the class of formatting
    // defect the generated testbench exists to rule out.
    let mut text = String::from(testbench::TRACE_HEADER);
    text.push('\n');
    for observation in &report.observations {
        text.push_str(&format!("@{}", observation.step));
        for (name, value) in &observation.values {
            text.push_str(&format!(" {name}={value}"));
        }
        text.push('\n');
    }
    let trace = parse_trace(&text).map_err(|error| RunError::Trace {
        engine: VerilogEngine::Rspice,
        error,
    })?;

    Ok(RunOutcome {
        engine: VerilogEngine::Rspice,
        case: case.name.clone(),
        trace,
        compile_ms: 0,
        simulate_ms,
    })
}

#[cfg(not(feature = "verilog-digital"))]
fn run_rspice(_case: &Case) -> Result<RunOutcome, RunError> {
    Err(RunError::RspiceNotCompiledIn)
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

    /// The arm runs, needs no toolchain, and leaves no debris.
    ///
    /// This replaces the test that asserted the arm was unimplemented. That
    /// test's job was to fail the moment the arm became real; it has done it,
    /// and what stands in its place is the property the arm now has to keep.
    #[cfg(feature = "verilog-digital")]
    #[test]
    fn rspice_executes_a_case_without_a_toolchain_or_a_scratch_directory() {
        let case = any_case();
        let workspace = std::env::temp_dir().join("rspice-verilog-arm-probe");

        let outcome = run_case(&case, VerilogEngine::Rspice, None, &workspace, 1_000)
            .expect("c17 is compiled and run in this process");

        assert_eq!(outcome.engine, VerilogEngine::Rspice);
        assert_eq!(outcome.trace.rows.len(), case.stimulus.vectors.len());
        // The front end and the host are linked into this binary, so nothing
        // is compiled on disk and nothing is left behind.
        assert!(
            !workspace.exists(),
            "the RSpice arm must not touch the filesystem"
        );
    }

    /// A construct RSpice cannot compile is a named refusal, never a trace.
    ///
    /// Fail-closed is the whole rule: a corpus case the engine cannot execute
    /// must not be able to pass by producing nothing, and the diagnostic has to
    /// name the construct so the gap is actionable rather than a mystery.
    #[cfg(feature = "verilog-digital")]
    #[test]
    fn a_construct_rspice_cannot_compile_is_refused_by_name() {
        let corpus = Corpus::load(&corpus_dir()).expect("corpus loads");
        let case = corpus
            .case("ripple_adder")
            .expect("the generate-unrolled case is in the corpus");
        let workspace = std::env::temp_dir().join("rspice-verilog-refusal-probe");

        let error = run_case(case, VerilogEngine::Rspice, None, &workspace, 1_000)
            .expect_err("generate regions have no elaborated form yet");

        let RunError::RspiceRefused { case: name, detail } = &error else {
            panic!("expected the named refusal, got {error:?}");
        };
        assert_eq!(name, "ripple_adder");
        assert!(detail.contains("genvar"), "{detail}");
    }

    /// The plumbing must propagate a refusal rather than treat an unrunnable
    /// engine as producing an empty trace, which would then agree with
    /// anything.
    #[cfg(feature = "verilog-digital")]
    #[test]
    fn comparison_plumbing_propagates_a_refusal_rather_than_reporting_agreement() {
        let corpus = Corpus::load(&corpus_dir()).expect("corpus loads");
        let case = corpus.case("ripple_adder").expect("in the corpus");
        let workspace = std::env::temp_dir().join("rspice-verilog-refusal-probe-compare");

        let error = compare_engines(
            case,
            (VerilogEngine::Rspice, None),
            (VerilogEngine::Rspice, None),
            &workspace,
            1_000,
        )
        .expect_err("a comparison of a case RSpice refuses cannot succeed");

        assert!(matches!(error, RunError::RspiceRefused { .. }), "{error:?}");
    }

    /// RSpice compared against itself agrees, which is the weakest thing the
    /// comparator can say and the one thing a broken comparator would get
    /// wrong.
    #[cfg(feature = "verilog-digital")]
    #[test]
    fn comparing_rspice_against_itself_reports_agreement() {
        let case = any_case();
        let workspace = std::env::temp_dir().join("rspice-verilog-self-compare");
        let divergences = compare_engines(
            &case,
            (VerilogEngine::Rspice, None),
            (VerilogEngine::Rspice, None),
            &workspace,
            1_000,
        )
        .expect("c17 runs");
        assert!(divergences.is_empty());
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
