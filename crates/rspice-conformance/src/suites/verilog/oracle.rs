//! Finding the oracle binaries, and running them without hanging the suite.
//!
//! # Absence is a value, not an exception
//!
//! Neither Icarus Verilog nor Verilator is installed on CI, and neither was
//! installed on the machine where this harness was written. That is the normal
//! case for now, so [`detect`] returns a typed [`OracleAvailability`] carrying
//! what was probed and what to install, and the caller decides what to do about
//! it.
//!
//! This is where the ngspice suite's shape is deliberately *not* copied.
//! `capture_ngspice_oracles` takes an explicit executable path and returns
//! `Err` when it is wrong, because it is a maintenance command a person invokes
//! on purpose. This runs inside an ordinary test, where the same design would
//! turn "the tool is not installed" into a red build on every developer
//! machine.
//!
//! # But a silent skip is the failure mode a conformance suite must not have
//!
//! So skipping is loud and it is switchable. Absence prints a diagnostic naming
//! every path probed and the package to install. Setting
//! `RSPICE_VERILOG_ORACLES_REQUIRED=1` turns that same absence into a failure,
//! which is what CI should set once the binaries are on the image — the
//! difference between "we have not installed it yet" and "it silently stopped
//! being installed" is otherwise invisible, and the second one is a regression.
//!
//! # Overrides
//!
//! `RSPICE_IVERILOG_EXE`, `RSPICE_VVP_EXE`, and `RSPICE_VERILATOR_EXE` name
//! executables explicitly. Unset, the program is looked up on `PATH`. An
//! override that does not resolve is [`OracleAvailability::Unusable`] rather
//! than `Missing`: someone who names a path has said the tool is there, and
//! quietly falling back to `PATH` would run a different binary than the one
//! they asked for.

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

/// How long a version probe may take before the tool is considered unusable.
const VERSION_PROBE_TIMEOUT_MS: u64 = 30_000;

/// Environment variable that promotes a missing oracle from skip to failure.
pub const ORACLES_REQUIRED_ENV: &str = "RSPICE_VERILOG_ORACLES_REQUIRED";

/// A simulator this suite can ask for an answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VerilogEngine {
    /// Icarus Verilog: compile with `iverilog`, run the result with `vvp`.
    /// Four-state, event-driven.
    Icarus,
    /// Verilator: compile to a native binary and run it. Two-state and cycle
    /// oriented, which is why the manifest excludes it from the X/Z case.
    Verilator,
    /// RSpice itself. Present so the comparison plumbing is complete and
    /// refuses explicitly; see [`super::run`].
    Rspice,
}

impl VerilogEngine {
    /// The two engines that are independent of RSpice.
    pub const ORACLES: [Self; 2] = [Self::Icarus, Self::Verilator];

    pub fn manifest_token(self) -> &'static str {
        match self {
            Self::Icarus => "iverilog",
            Self::Verilator => "verilator",
            Self::Rspice => "rspice",
        }
    }

    pub fn from_manifest_token(token: &str) -> Option<Self> {
        match token {
            "iverilog" => Some(Self::Icarus),
            "verilator" => Some(Self::Verilator),
            // `rspice` is intentionally not accepted. Nothing in RSpice
            // executes digital Verilog, so a manifest row claiming it as an
            // oracle would be describing a capability that does not exist.
            _ => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Icarus => "Icarus Verilog",
            Self::Verilator => "Verilator",
            Self::Rspice => "RSpice",
        }
    }

    /// Whether this engine is an independent oracle rather than the system
    /// under test.
    pub fn is_oracle(self) -> bool {
        matches!(self, Self::Icarus | Self::Verilator)
    }

    /// The programs the engine needs, each with the role it plays, the
    /// environment override that names it, and the flag that prints its
    /// version.
    fn programs(self) -> &'static [ProgramSpec] {
        match self {
            Self::Icarus => &[
                ProgramSpec {
                    role: "compiler",
                    default: "iverilog",
                    override_env: "RSPICE_IVERILOG_EXE",
                    version_flag: "-V",
                },
                ProgramSpec {
                    role: "runtime",
                    default: "vvp",
                    override_env: "RSPICE_VVP_EXE",
                    version_flag: "-V",
                },
            ],
            Self::Verilator => &[ProgramSpec {
                role: "compiler",
                default: "verilator",
                override_env: "RSPICE_VERILATOR_EXE",
                version_flag: "--version",
            }],
            Self::Rspice => &[],
        }
    }

    /// What a person has to install to arm this oracle.
    pub fn install_hint(self) -> &'static str {
        match self {
            Self::Icarus => {
                "Windows: `winget install --id IcarusVerilog.IcarusVerilog` or \
                 `choco install iverilog`. Debian/Ubuntu: `apt install iverilog`. \
                 macOS: `brew install icarus-verilog`. Provides both `iverilog` and `vvp`."
            }
            Self::Verilator => {
                "Debian/Ubuntu: `apt install verilator` (5.x required for `--binary`/`--timing`). \
                 macOS: `brew install verilator`. Windows has no first-class native build; use \
                 WSL, MSYS2 (`pacman -S mingw-w64-x86_64-verilator`), or a container. Verilator \
                 also needs a C++ toolchain and `make`, because it compiles the design to a \
                 native executable."
            }
            Self::Rspice => "RSpice does not execute digital Verilog yet.",
        }
    }
}

impl fmt::Display for VerilogEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

#[derive(Debug, Clone, Copy)]
struct ProgramSpec {
    role: &'static str,
    default: &'static str,
    override_env: &'static str,
    version_flag: &'static str,
}

/// A program that was found and answered a version probe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedProgram {
    pub role: &'static str,
    pub program: String,
    pub version: String,
    /// Whether the path came from an environment override rather than `PATH`.
    pub from_override: bool,
}

/// Every program one engine needs, all resolved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OracleTools {
    pub engine: VerilogEngine,
    pub programs: Vec<ResolvedProgram>,
}

impl OracleTools {
    pub fn program(&self, role: &str) -> Option<&ResolvedProgram> {
        self.programs.iter().find(|program| program.role == role)
    }

    /// A one-line identity for the whole toolchain, for diagnostics and for
    /// recording alongside a captured oracle.
    pub fn identity(&self) -> String {
        self.programs
            .iter()
            .map(|program| format!("{}={}", program.role, program.version))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// A program that could not be found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingProgram {
    pub role: &'static str,
    /// What was looked for: an override value, or a bare name searched on
    /// `PATH`.
    pub probed: String,
    pub override_env: &'static str,
    pub from_override: bool,
    pub detail: String,
}

/// The result of looking for one engine's toolchain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OracleAvailability {
    /// Every program was found and reported a version.
    Available(OracleTools),
    /// At least one program is absent. Carries every probe that failed, so the
    /// diagnostic names all of them rather than only the first.
    Missing {
        engine: VerilogEngine,
        missing: Vec<MissingProgram>,
        install_hint: &'static str,
    },
    /// The programs exist but cannot be used — a version probe that hung,
    /// crashed, or printed nothing recognisable.
    Unusable {
        engine: VerilogEngine,
        program: String,
        reason: String,
    },
    /// [`VerilogEngine::Rspice`] is the system under test, not an oracle.
    /// Detection is meaningless for it and says so rather than inventing an
    /// answer.
    NotAnOracle(VerilogEngine),
}

impl OracleAvailability {
    pub fn tools(&self) -> Option<&OracleTools> {
        match self {
            Self::Available(tools) => Some(tools),
            _ => None,
        }
    }

    pub fn is_available(&self) -> bool {
        matches!(self, Self::Available(_))
    }

    /// The line printed when a suite decides to skip because of this result.
    ///
    /// Deliberately verbose: a skip that does not say exactly what was looked
    /// for and how to fix it trains people to ignore skips.
    pub fn diagnostic(&self) -> String {
        match self {
            Self::Available(tools) => {
                format!("{} available: {}", tools.engine, tools.identity())
            }
            Self::Missing {
                engine,
                missing,
                install_hint,
            } => {
                let probes = missing
                    .iter()
                    .map(|program| {
                        let source = if program.from_override {
                            format!("{} override", program.override_env)
                        } else {
                            "PATH".to_string()
                        };
                        format!(
                            "  {} '{}' (from {}): {}",
                            program.role, program.probed, source, program.detail
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                format!(
                    "{engine} is not installed; skipping the cases that need it.\n\
                     Probed:\n{probes}\n\
                     To install: {install_hint}\n\
                     Set {ORACLES_REQUIRED_ENV}=1 to make this absence a failure instead."
                )
            }
            Self::Unusable {
                engine,
                program,
                reason,
            } => format!("{engine} program '{program}' is present but unusable: {reason}"),
            Self::NotAnOracle(engine) => {
                format!("{engine} is the system under test, not an independent oracle")
            }
        }
    }
}

/// Whether a missing oracle should fail the run rather than skip it.
pub fn oracles_required() -> bool {
    std::env::var(ORACLES_REQUIRED_ENV).is_ok_and(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        )
    })
}

/// Look for one engine's toolchain.
pub fn detect(engine: VerilogEngine) -> OracleAvailability {
    if !engine.is_oracle() {
        return OracleAvailability::NotAnOracle(engine);
    }

    let mut resolved = Vec::new();
    let mut missing = Vec::new();
    for spec in engine.programs() {
        let overridden = std::env::var(spec.override_env)
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let from_override = overridden.is_some();
        let program = overridden.unwrap_or_else(|| spec.default.to_string());

        match probe_version(&program, spec.version_flag) {
            Ok(version) => resolved.push(ResolvedProgram {
                role: spec.role,
                program,
                version,
                from_override,
            }),
            Err(ProbeFailure::NotFound(detail)) if !from_override => missing.push(MissingProgram {
                role: spec.role,
                probed: program,
                override_env: spec.override_env,
                from_override,
                detail,
            }),
            // An override that does not resolve is a mistake worth surfacing,
            // not a reason to look somewhere else.
            Err(ProbeFailure::NotFound(detail)) => {
                return OracleAvailability::Unusable {
                    engine,
                    program,
                    reason: format!(
                        "{} names it but it could not be run: {detail}",
                        spec.override_env
                    ),
                };
            }
            Err(ProbeFailure::Unusable(reason)) => {
                return OracleAvailability::Unusable {
                    engine,
                    program,
                    reason,
                };
            }
        }
    }

    if missing.is_empty() {
        OracleAvailability::Available(OracleTools {
            engine,
            programs: resolved,
        })
    } else {
        OracleAvailability::Missing {
            engine,
            missing,
            install_hint: engine.install_hint(),
        }
    }
}

enum ProbeFailure {
    /// The program could not be spawned at all.
    NotFound(String),
    /// It ran but did not behave like the tool it claims to be.
    Unusable(String),
}

fn probe_version(program: &str, flag: &str) -> Result<String, ProbeFailure> {
    let outcome = run_process(program, &[flag.to_string()], None, VERSION_PROBE_TIMEOUT_MS)
        .map_err(|err| match err {
            ProcessError::Spawn(detail) => ProbeFailure::NotFound(detail),
            other => ProbeFailure::Unusable(other.to_string()),
        })?;

    // Both tools print their banner on stdout, but neither promises to, and
    // `iverilog -V` has historically used stderr on some builds. Take the first
    // non-empty line of either.
    let combined = format!("{}\n{}", outcome.stdout, outcome.stderr);
    combined
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(str::to_string)
        .ok_or_else(|| {
            ProbeFailure::Unusable(format!(
                "'{program} {flag}' printed nothing; it may not be the tool it is named after"
            ))
        })
}

/// What a finished child process produced.
#[derive(Debug, Clone)]
pub struct ProcessOutcome {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
    pub status: String,
    pub duration_ms: u128,
}

/// Everything that can go wrong running a child.
#[derive(Debug, Clone)]
pub enum ProcessError {
    /// The program could not be started — the usual shape of "not installed".
    Spawn(String),
    /// It started and did not finish in time. The child is killed and reaped
    /// before this is returned.
    Timeout { program: String, timeout_ms: u64 },
    /// The harness itself failed: a temporary file, a poll, a read.
    Harness(String),
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Spawn(detail) => write!(f, "could not be started: {detail}"),
            Self::Timeout {
                program,
                timeout_ms,
            } => write!(
                f,
                "'{program}' exceeded its {timeout_ms}ms timeout and was killed"
            ),
            Self::Harness(detail) => write!(f, "harness failure: {detail}"),
        }
    }
}

impl std::error::Error for ProcessError {}

/// Run a program to completion under a hard timeout.
///
/// Output goes to temporary files rather than pipes. A pipe that fills while
/// nobody is reading it blocks the child, and the child blocking is exactly the
/// case this timeout exists to catch — with pipes the harness would deadlock
/// waiting for a process that is itself waiting for the harness. Verilator in
/// particular is capable of many megabytes of warnings.
pub fn run_process(
    program: &str,
    args: &[String],
    working_dir: Option<&Path>,
    timeout_ms: u64,
) -> Result<ProcessOutcome, ProcessError> {
    let started = Instant::now();
    let temp = tempfile::Builder::new()
        .prefix("rspice-verilog-oracle-")
        .tempdir()
        .map_err(|err| ProcessError::Harness(format!("temporary directory: {err}")))?;
    let stdout_path = temp.path().join("stdout.txt");
    let stderr_path = temp.path().join("stderr.txt");
    let stdout_file = fs::File::create(&stdout_path)
        .map_err(|err| ProcessError::Harness(format!("stdout file: {err}")))?;
    let stderr_file = fs::File::create(&stderr_path)
        .map_err(|err| ProcessError::Harness(format!("stderr file: {err}")))?;

    let mut command = headless_command(program);
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file));
    if let Some(dir) = working_dir {
        command.current_dir(dir);
    }

    let mut child = command
        .spawn()
        .map_err(|err| ProcessError::Spawn(err.to_string()))?;

    let timeout = Duration::from_millis(timeout_ms);
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if started.elapsed() >= timeout => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(ProcessError::Timeout {
                    program: program.to_string(),
                    timeout_ms,
                });
            }
            Ok(None) => thread::sleep(Duration::from_millis(25)),
            Err(err) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(ProcessError::Harness(format!("poll failed: {err}")));
            }
        }
    };

    Ok(ProcessOutcome {
        stdout: fs::read_to_string(&stdout_path).unwrap_or_default(),
        stderr: fs::read_to_string(&stderr_path).unwrap_or_default(),
        success: status.success(),
        status: status.to_string(),
        duration_ms: started.elapsed().as_millis(),
    })
}

/// Build a command that cannot pop a console window on a developer's desktop.
fn headless_command(program: &str) -> Command {
    #[cfg_attr(not(windows), allow(unused_mut))]
    let mut command = Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        // CREATE_NO_WINDOW. A suite that flashes console windows for every one
        // of hundreds of child processes is unusable on Windows.
        command.creation_flags(0x0800_0000);
    }
    command
}

/// Where an engine's intermediate build products go.
///
/// Each case gets its own directory so two cases running concurrently cannot
/// collide on an object file or an executable name.
pub fn scratch_dir(parent: &Path, case: &str, engine: VerilogEngine) -> PathBuf {
    parent.join(format!("{case}-{}", engine.manifest_token()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_tokens_round_trip_for_oracles_only() {
        for engine in VerilogEngine::ORACLES {
            assert_eq!(
                VerilogEngine::from_manifest_token(engine.manifest_token()),
                Some(engine)
            );
            assert!(engine.is_oracle());
        }
        // The system under test must not be nameable as its own oracle.
        assert_eq!(VerilogEngine::from_manifest_token("rspice"), None);
        assert!(!VerilogEngine::Rspice.is_oracle());
        assert_eq!(VerilogEngine::from_manifest_token("modelsim"), None);
    }

    #[test]
    fn detection_of_the_system_under_test_is_refused_rather_than_guessed() {
        assert_eq!(
            detect(VerilogEngine::Rspice),
            OracleAvailability::NotAnOracle(VerilogEngine::Rspice)
        );
    }

    #[test]
    fn a_program_that_does_not_exist_is_missing_rather_than_an_error() {
        let outcome = run_process(
            "rspice-verilog-oracle-that-does-not-exist",
            &[],
            None,
            1_000,
        );

        assert!(
            matches!(outcome, Err(ProcessError::Spawn(_))),
            "expected a spawn failure, got {outcome:?}"
        );
    }

    #[test]
    fn a_missing_oracle_diagnostic_names_every_probe_and_the_fix() {
        let availability = OracleAvailability::Missing {
            engine: VerilogEngine::Icarus,
            missing: vec![
                MissingProgram {
                    role: "compiler",
                    probed: "iverilog".to_string(),
                    override_env: "RSPICE_IVERILOG_EXE",
                    from_override: false,
                    detail: "not found".to_string(),
                },
                MissingProgram {
                    role: "runtime",
                    probed: "vvp".to_string(),
                    override_env: "RSPICE_VVP_EXE",
                    from_override: false,
                    detail: "not found".to_string(),
                },
            ],
            install_hint: VerilogEngine::Icarus.install_hint(),
        };

        let diagnostic = availability.diagnostic();

        // Both probes, not just the first one that failed.
        assert!(diagnostic.contains("iverilog"), "{diagnostic}");
        assert!(diagnostic.contains("vvp"), "{diagnostic}");
        // The way out, and the way to stop it being a skip.
        assert!(diagnostic.contains("winget"), "{diagnostic}");
        assert!(diagnostic.contains(ORACLES_REQUIRED_ENV), "{diagnostic}");
        assert!(!availability.is_available());
    }

    #[test]
    fn scratch_directories_are_unique_per_case_and_engine() {
        let root = Path::new("scratch");
        let a = scratch_dir(root, "c17", VerilogEngine::Icarus);
        let b = scratch_dir(root, "c17", VerilogEngine::Verilator);
        let c = scratch_dir(root, "c18", VerilogEngine::Icarus);

        assert_ne!(a, b);
        assert_ne!(a, c);
    }
}
