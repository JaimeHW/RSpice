//! Benchmark execution: child-process timing, statistics, scoreboard
//! emission, and the human-readable results table.
//!
//! Methodology (also embedded verbatim in every scoreboard JSON): each deck
//! is run as a full simulator process — `rspice run <deck> -q` and
//! `ngspice -b <deck>` — with stdin/stdout/stderr attached to the null
//! device. Wall-clock time is taken with [`std::time::Instant`] around
//! spawn/wait, so parsing and output formatting are included. One untimed
//! warmup run precedes the timed repeats for every deck/simulator pair.
//! Cold OS file-cache effects are not controlled.

use crate::error::BenchError;
use crate::generate::{self, GENERATED_DECKS_DIR};
use crate::population::{self, Suite, Workload};
use clap::Args;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::Instant;
use wait_timeout::ChildExt;

/// Directory of the hand-written decks in the shipped set.
const SHIPPED_CIRCUITS_DIR: &str = "tools/rspice-bench/circuits";

/// Environment variable overriding the RSpice executable location.
const RSPICE_ENV: &str = "RSPICE_BENCH_RSPICE";

/// Environment variable supplying the ngspice executable location. When it
/// is not set the ngspice column is skipped and noted in the scoreboard.
const NGSPICE_ENV: &str = "RSPICE_BENCH_NGSPICE";

/// Environment variable supplying an ESpice executable for a second context
/// column. ESpice runs as `<exe> <deck>`, on simulating workloads only.
const ESPICE_ENV: &str = "RSPICE_BENCH_ESPICE";

/// Timing-methodology note embedded in every scoreboard.
const METHODOLOGY: &str = "wall-clock of the full child process (parse + solve + output) timed \
     with std::time::Instant around spawn and an OS-backed timed wait (no polling interval); child \
     stdin/stdout/stderr attached to the null device; one untimed warmup run per deck/simulator \
     precedes the timed repeats; cold OS cache not controlled; ngspice runs as `-b -r <raw>` \
     because its batch mode simulates nothing without an output request, while `rspice run -q` \
     writes no result file";

/// Note recorded in the scoreboard when ngspice timings are skipped.
const NGSPICE_SKIPPED_NOTE: &str =
    "RSPICE_BENCH_NGSPICE not set; ngspice columns skipped for this run";

/// Default per-deck slowdown budget when a baseline is supplied.
const DEFAULT_MAX_REGRESSION_PERCENT: f64 = 10.0;

/// Scoreboards are small; cap baseline ingestion so a corrupt path cannot
/// turn a developer or CI performance check into an unbounded allocation.
const MAX_BASELINE_BYTES: u64 = 16 * 1024 * 1024;

/// Arguments for the `run` subcommand.
#[derive(Args, Debug)]
pub struct RunArgs {
    /// Timed repetitions per deck and simulator (one untimed warmup runs first).
    #[arg(long, default_value_t = 5, value_parser = clap::value_parser!(u32).range(1..))]
    pub repeats: u32,

    /// Timed repetitions of a minute-scale suite workload (ISCAS), which
    /// takes no warm-up.
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(1..))]
    pub heavy_repeats: u32,

    /// Path of the JSON scoreboard to write. Keep measurements in ignored
    /// target/benchmarks or CI artifact storage; this file is overwritten.
    #[arg(
        long,
        default_value = "target/benchmarks/scoreboard.json",
        value_name = "PATH"
    )]
    pub out: PathBuf,

    /// Directory scanned (non-recursively) for benchmark decks (*.cir). Without
    /// it the run takes the shipped set: the hand-written decks in
    /// tools/rspice-bench/circuits plus freshly generated parametric decks.
    #[arg(long, value_name = "DIR")]
    pub circuits: Option<PathBuf>,

    /// Per-run wall-clock cap in seconds; a run that exceeds it is killed
    /// and counted as failed. Keeps a pathological deck from wedging the
    /// whole rig.
    #[arg(long, default_value_t = 120, value_parser = clap::value_parser!(u64).range(1..))]
    pub timeout_secs: u64,

    /// Previous scoreboard from the same host. When supplied, every current
    /// RSpice median is gated against the matching deck in this baseline.
    #[arg(long, value_name = "PATH")]
    pub baseline: Option<PathBuf>,

    /// Largest allowed per-deck RSpice median regression versus --baseline
    /// (default: 10 when a baseline is supplied).
    #[arg(long, value_name = "PERCENT", requires = "baseline")]
    pub max_regression_percent: Option<f64>,

    /// Permit a baseline whose OS/architecture or logical CPU count differs.
    /// This is unsafe for release gates and intended only for exploratory runs.
    #[arg(long, requires = "baseline")]
    pub allow_host_mismatch: bool,

    /// Time a named workload population (decks generated into
    /// target/benchmarks/decks/<suite>/, corpus decks run in place) instead
    /// of the default deck set.
    #[arg(long, value_enum, conflicts_with = "circuits")]
    pub suite: Option<Suite>,

    /// Time only the workloads whose id contains this text.
    #[arg(long, value_name = "TEXT")]
    pub filter: Option<String>,

    /// Free-text label recorded in the scoreboard, such as the machine state.
    #[arg(long, value_name = "TEXT")]
    pub label: Option<String>,
}

/// Complete scoreboard document serialized to `--out`.
#[derive(Serialize)]
struct Scoreboard {
    /// Tool that produced the document.
    generated_with: &'static str,
    /// Machine the run was executed on.
    host: HostInfo,
    /// Free-text label the run was given (`--label`).
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    /// The named population timed, when one was (`--suite`).
    #[serde(skip_serializing_if = "Option::is_none")]
    suite: Option<&'static str>,
    /// One-, five- and fifteen-minute load averages before and after the run.
    load_average_start: Option<[f64; 3]>,
    load_average_end: Option<[f64; 3]>,
    /// Spawning a minimal executable (`rspice-bench --spawn-floor`): the
    /// floor every process pays.
    spawn_floor_ms: TimingStats,
    /// Timed repetitions per deck/simulator.
    repeats: u32,
    /// Timing-methodology caveats for downstream consumers.
    methodology: &'static str,
    /// RSpice executable that was benchmarked.
    rspice_exe: String,
    /// ngspice executable that was benchmarked, if configured.
    ngspice_exe: Option<String>,
    /// Present when the ngspice column was skipped (env var missing).
    ngspice_note: Option<&'static str>,
    /// ESpice executable that was benchmarked, if configured.
    espice_exe: Option<String>,
    /// Per-deck timing results, in deck order.
    results: Vec<DeckResult>,
    /// Baseline comparison report when `--baseline` was supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    regression_gate: Option<RegressionGate>,
}

/// Host descriptor recorded in the scoreboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct HostInfo {
    /// Operating system and architecture, e.g. `windows x86_64`.
    os: String,
    /// Logical CPU count reported by the OS.
    cpu_count: usize,
}

/// Wall-clock statistics over the timed repeats, in milliseconds.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub(crate) struct TimingStats {
    /// Fastest timed run.
    pub min: f64,
    /// Median of the timed runs (midpoint average for even counts).
    pub median: f64,
    /// Arithmetic mean of the timed runs.
    pub mean: f64,
}

/// Result row for a single benchmark workload.
#[derive(Serialize)]
struct DeckResult {
    /// Workload id: the deck file name outside a named suite.
    deck: String,
    /// Suite the workload belongs to.
    suite: &'static str,
    /// The command timed.
    command: &'static str,
    /// Load averages when the workload started.
    load_average: Option<[f64; 3]>,
    /// RSpice timing statistics in milliseconds.
    rspice_ms: TimingStats,
    /// RSpice median minus the spawn-floor median.
    rspice_above_floor_ms: f64,
    /// Whether every RSpice run (warmup included) exited with status 0.
    rspice_all_ok: bool,
    /// ngspice timing statistics in milliseconds; `null` when skipped.
    ngspice_ms: Option<TimingStats>,
    /// Whether every ngspice run exited with status 0; `null` when skipped.
    ngspice_all_ok: Option<bool>,
    /// ESpice timing statistics in milliseconds; `null` when skipped.
    espice_ms: Option<TimingStats>,
    /// Whether every ESpice run exited with status 0; `null` when skipped.
    espice_all_ok: Option<bool>,
    /// `ngspice median / rspice median`; greater than 1.0 means RSpice is
    /// faster. `null` unless both simulators ran and succeeded.
    speedup_median: Option<f64>,
    /// True when both simulators ran this deck and every run exited 0.
    both_succeeded: bool,
}

/// Minimum baseline schema consumed by the regression gate. Additional
/// scoreboard fields remain forward-compatible through serde's default
/// unknown-field behavior.
#[derive(Debug, Deserialize)]
struct BaselineScoreboard {
    host: HostInfo,
    repeats: u32,
    methodology: String,
    results: Vec<BaselineDeckResult>,
}

#[derive(Debug, Deserialize)]
struct BaselineDeckResult {
    deck: String,
    rspice_ms: TimingStats,
    rspice_all_ok: bool,
}

/// Machine-readable outcome of comparing a run to its baseline.
#[derive(Debug, Serialize)]
struct RegressionGate {
    baseline: String,
    max_regression_percent: f64,
    host_match: bool,
    passed: bool,
    results: Vec<RegressionResult>,
}

/// Per-deck median comparison retained in the scoreboard.
#[derive(Debug, Serialize)]
struct RegressionResult {
    deck: String,
    baseline_median_ms: f64,
    current_median_ms: f64,
    change_percent: f64,
    passed: bool,
}

/// Outcome of benchmarking one deck on one simulator.
struct SimMeasurement {
    /// Statistics over the timed runs.
    stats: TimingStats,
    /// True when every run, including the warmup, exited with status 0.
    all_ok: bool,
}

/// Entry point of the `run` subcommand.
///
/// Returns `ExitCode::FAILURE` (after still writing the scoreboard) when any
/// simulator run exited non-zero, so CI can gate on deck health; a missing
/// ngspice configuration is a skip, not a failure.
pub fn run(args: &RunArgs) -> Result<ExitCode, BenchError> {
    let max_regression_percent = args
        .max_regression_percent
        .unwrap_or(DEFAULT_MAX_REGRESSION_PERCENT);
    if !max_regression_percent.is_finite() || max_regression_percent < 0.0 {
        return Err(BenchError::BenchmarkPolicy {
            message: format!(
                "--max-regression-percent must be a finite non-negative number, got {}",
                max_regression_percent
            ),
        });
    }
    let baseline = args.baseline.as_deref().map(load_baseline).transpose()?;
    let rspice = locate_rspice()?;
    let ngspice = locate_context(NGSPICE_ENV)?;
    let espice = locate_context(ESPICE_ENV)?;
    let (workloads, source) = select_workloads(args.suite, args.circuits.as_deref())?;
    let workloads = filter_workloads(workloads, args.filter.as_deref())?;

    println!("rspice : {}", rspice.display());
    match &ngspice {
        Some(path) => println!("ngspice: {}", path.display()),
        None => println!("ngspice: skipped ({NGSPICE_ENV} not set)"),
    }
    if let Some(path) = &espice {
        println!("espice : {}", path.display());
    }
    println!(
        "decks  : {} from {source}; repeats: {} (+1 warmup)\n",
        workloads.len(),
        args.repeats
    );

    let timeout = std::time::Duration::from_secs(args.timeout_secs);
    let load_average_start = load_average();
    let spawn_floor = spawn_floor(args.repeats, timeout)?;
    println!("spawn floor: median {:.2} ms", spawn_floor.median);
    let mut results = Vec::with_capacity(workloads.len());
    let mut any_failure = false;
    for workload in &workloads {
        let result = bench_workload(
            workload,
            &rspice,
            Context {
                ngspice: ngspice.as_deref(),
                espice: espice.as_deref(),
            },
            Repeats {
                light: args.repeats,
                heavy: args.heavy_repeats,
            },
            timeout,
            spawn_floor.median,
        )?;
        any_failure |= !result.rspice_all_ok
            || result.ngspice_all_ok == Some(false)
            || result.espice_all_ok == Some(false);
        results.push(result);
    }

    let mut scoreboard = Scoreboard {
        generated_with: "rspice-bench",
        host: host_info(),
        label: args.label.clone(),
        suite: args.suite.map(Suite::name),
        load_average_start,
        load_average_end: load_average(),
        spawn_floor_ms: spawn_floor,
        repeats: args.repeats,
        methodology: METHODOLOGY,
        rspice_exe: rspice.display().to_string(),
        ngspice_exe: ngspice.as_ref().map(|p| p.display().to_string()),
        ngspice_note: ngspice.is_none().then_some(NGSPICE_SKIPPED_NOTE),
        espice_exe: espice.as_ref().map(|p| p.display().to_string()),
        results,
        regression_gate: None,
    };
    if let (Some(path), Some(baseline)) = (args.baseline.as_deref(), baseline.as_ref()) {
        let gate = evaluate_regression(
            &scoreboard,
            baseline,
            path,
            max_regression_percent,
            args.allow_host_mismatch,
        )?;
        any_failure |= !gate.passed;
        scoreboard.regression_gate = Some(gate);
    }
    write_json(&args.out, &scoreboard)?;

    println!();
    print_table(
        &scoreboard.results,
        spawn_floor.median,
        args.label.as_deref(),
    );
    if let Some(gate) = &scoreboard.regression_gate {
        println!();
        print_regression_gate(gate);
    }
    println!("\nscoreboard written to {}", args.out.display());

    Ok(if any_failure {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

/// The workloads a run times: a named suite, every deck of a directory, or
/// the default deck set, with a description of where they came from.
pub(crate) fn select_workloads(
    suite: Option<Suite>,
    circuits: Option<&Path>,
) -> Result<(Vec<Workload>, String), BenchError> {
    Ok(match (suite, circuits) {
        (Some(suite), _) => (
            population::population(
                suite,
                &crate::workspace_root(),
                Path::new(GENERATED_DECKS_DIR),
            )?,
            format!("suite `{}`", suite.name()),
        ),
        (None, Some(dir)) => (
            discover_decks(dir)?
                .into_iter()
                .map(Workload::run_quiet)
                .collect(),
            format!("`{}`", dir.display()),
        ),
        (None, None) => (
            shipped_decks()?
                .into_iter()
                .map(Workload::run_quiet)
                .collect(),
            format!("`{SHIPPED_CIRCUITS_DIR}` and `{GENERATED_DECKS_DIR}`"),
        ),
    })
}

/// Keep the workloads whose id contains `filter`; refuse an empty selection,
/// which would otherwise pass vacuously.
pub(crate) fn filter_workloads(
    workloads: Vec<Workload>,
    filter: Option<&str>,
) -> Result<Vec<Workload>, BenchError> {
    let Some(filter) = filter else {
        return Ok(workloads);
    };
    let kept: Vec<Workload> = workloads
        .into_iter()
        .filter(|workload| workload.id.contains(filter))
        .collect();
    if kept.is_empty() {
        return Err(BenchError::BenchmarkPolicy {
            message: format!("--filter `{filter}` matches no workload"),
        });
    }
    Ok(kept)
}

/// The context simulators a run times beside RSpice.
#[derive(Clone, Copy)]
pub(crate) struct Context<'a> {
    pub ngspice: Option<&'a Path>,
    pub espice: Option<&'a Path>,
}

/// Times the spawn floor ([`population::SPAWN_FLOOR_ARG`]), the cost under
/// every measurement.
pub(crate) fn spawn_floor(
    repeats: u32,
    timeout: std::time::Duration,
) -> Result<TimingStats, BenchError> {
    let (exe, args) = population::spawn_floor_command()?;
    let measured = measure_args(&exe, &args, Path::new("."), 1, repeats, timeout)?;
    if !measured.all_ok {
        return Err(BenchError::BenchmarkPolicy {
            message: format!("the spawn floor `{}` failed", exe.display()),
        });
    }
    Ok(measured.stats)
}

/// Timed runs per workload: `light` after one untimed warm-up, `heavy`
/// (minute-scale) workloads with none.
#[derive(Clone, Copy)]
struct Repeats {
    light: u32,
    heavy: u32,
}

/// Benchmarks one workload on RSpice and the configured context simulators.
fn bench_workload(
    workload: &Workload,
    rspice: &Path,
    context: Context<'_>,
    repeats: Repeats,
    timeout: std::time::Duration,
    floor_ms: f64,
) -> Result<DeckResult, BenchError> {
    let cwd = workload.cwd();
    let scratch = tempfile::tempdir().map_err(|error| {
        BenchError::io("failed to create a scratch directory for rawfiles", error)
    })?;
    let raw = (workload.action == population::Action::RunRaw).then(|| scratch.path().join("x.raw"));
    let (warmups, repeats) = if workload.heavy {
        (0, repeats.heavy)
    } else {
        (1, repeats.light)
    };
    let load_average = load_average();

    println!("benchmarking {} ({})", workload.id, workload.action.label());
    let (exe, args) = workload.rspice_command(rspice, raw.as_deref())?;
    let rspice_run = measure_args(&exe, &args, &cwd, warmups, repeats, timeout)?;
    print_progress("rspice", &rspice_run);

    let ngspice_raw = scratch.path().join("ngspice.raw");
    let ngspice_run = match (context.ngspice, workload.ngspice_command(&ngspice_raw)) {
        (Some(exe), Some(args)) => {
            let measurement = measure_args(exe, &args, &cwd, warmups, repeats, timeout)?;
            print_progress("ngspice", &measurement);
            Some(measurement)
        }
        _ => None,
    };
    let espice_run = match (context.espice, workload.espice_command()) {
        (Some(exe), Some(args)) => {
            let measurement = measure_args(exe, &args, &cwd, warmups, repeats, timeout)?;
            print_progress("espice", &measurement);
            Some(measurement)
        }
        _ => None,
    };

    let both_succeeded = rspice_run.all_ok
        && ngspice_run
            .as_ref()
            .is_some_and(|measurement| measurement.all_ok);
    let speedup_median = match &ngspice_run {
        Some(measurement) if both_succeeded && rspice_run.stats.median > 0.0 => {
            Some(measurement.stats.median / rspice_run.stats.median)
        }
        _ => None,
    };

    Ok(DeckResult {
        deck: workload.id.clone(),
        suite: workload.suite,
        command: workload.action.label(),
        load_average,
        rspice_ms: rspice_run.stats,
        rspice_above_floor_ms: rspice_run.stats.median - floor_ms,
        rspice_all_ok: rspice_run.all_ok,
        ngspice_ms: ngspice_run.as_ref().map(|measurement| measurement.stats),
        ngspice_all_ok: ngspice_run.as_ref().map(|measurement| measurement.all_ok),
        espice_ms: espice_run.as_ref().map(|measurement| measurement.stats),
        espice_all_ok: espice_run.as_ref().map(|measurement| measurement.all_ok),
        speedup_median,
        both_succeeded,
    })
}

/// Prints the per-simulator progress line shown while a deck is measured.
fn print_progress(label: &str, measurement: &SimMeasurement) {
    let status = if measurement.all_ok { "ok" } else { "FAILED" };
    println!(
        "  {label:<7} median {:>10.1} ms  min {:>10.1} ms  [{status}]",
        measurement.stats.median, measurement.stats.min
    );
}

fn measure_args(
    exe: &Path,
    args: &[OsString],
    cwd: &Path,
    warmups: u32,
    repeats: u32,
    timeout: std::time::Duration,
) -> Result<SimMeasurement, BenchError> {
    let args: Vec<&OsStr> = args.iter().map(OsString::as_os_str).collect();
    measure(exe, &args, cwd, warmups, repeats, timeout)
}

/// Runs `warmups` untimed plus `repeats` timed executions of `exe args` in
/// `cwd`. A run that exceeds `timeout` is killed and the whole measurement
/// is marked failed; the remaining repeats are skipped (they would only
/// repeat the timeout).
fn measure(
    exe: &Path,
    args: &[&OsStr],
    cwd: &Path,
    warmups: u32,
    repeats: u32,
    timeout: std::time::Duration,
) -> Result<SimMeasurement, BenchError> {
    let mut all_ok = true;
    for _ in 0..warmups {
        let (_, warmup_ok) = time_child(exe, args, cwd, timeout)?;
        all_ok &= warmup_ok;
    }
    let mut samples_ms = Vec::with_capacity(repeats as usize);
    for _ in 0..repeats {
        let (elapsed_ms, ok) = time_child(exe, args, cwd, timeout)?;
        all_ok &= ok;
        samples_ms.push(elapsed_ms);
        if !ok {
            break;
        }
    }
    let stats =
        timing_stats(&samples_ms).ok_or(BenchError::Internal("timing sample set was empty"))?;
    Ok(SimMeasurement { stats, all_ok })
}

/// Spawns one child process and returns `(elapsed_ms, exited_zero)`.
///
/// stdin/stdout/stderr are attached to the null device so console throughput
/// does not dominate the measurement; the elapsed time still includes the
/// child's own parsing and output formatting work. A child that outlives
/// `timeout` is killed and reported as failed.
pub(crate) fn time_child(
    exe: &Path,
    args: &[&OsStr],
    cwd: &Path,
    timeout: std::time::Duration,
) -> Result<(f64, bool), BenchError> {
    let start = Instant::now();
    let mut child = Command::new(exe)
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|err| {
            BenchError::io(
                format!("failed to launch `{}` for benchmarking", exe.display()),
                err,
            )
        })?;

    let status = child.wait_timeout(timeout).map_err(|error| {
        BenchError::io(
            format!("failed to wait for `{}` during benchmarking", exe.display()),
            error,
        )
    })?;
    if let Some(status) = status {
        return Ok((start.elapsed().as_secs_f64() * 1e3, status.success()));
    }

    let _ = child.kill();
    let _ = child.wait();
    eprintln!(
        "  TIMEOUT: `{}` exceeded {}s and was killed",
        exe.display(),
        timeout.as_secs()
    );
    Ok((start.elapsed().as_secs_f64() * 1e3, false))
}

/// Computes min/median/mean over a non-empty sample set (milliseconds).
/// Returns `None` for an empty input instead of panicking.
pub(crate) fn timing_stats(samples_ms: &[f64]) -> Option<TimingStats> {
    if samples_ms.is_empty() {
        return None;
    }
    let mut sorted = samples_ms.to_vec();
    sorted.sort_by(f64::total_cmp);
    let count = sorted.len();
    let median = if count.is_multiple_of(2) {
        (sorted[count / 2 - 1] + sorted[count / 2]) / 2.0
    } else {
        sorted[count / 2]
    };
    Some(TimingStats {
        min: sorted[0],
        median,
        mean: sorted.iter().sum::<f64>() / count as f64,
    })
}

/// Locates the RSpice executable: `RSPICE_BENCH_RSPICE` override first, then
/// a sibling of the running benchmark binary (both live in
/// `target/release/`), then `target/release/` relative to the working
/// directory.
pub(crate) fn locate_rspice() -> Result<PathBuf, BenchError> {
    if let Some(raw) = env::var_os(RSPICE_ENV) {
        let path = PathBuf::from(raw);
        return if path.is_file() {
            Ok(path)
        } else {
            Err(BenchError::ExeNotFound {
                env_var: RSPICE_ENV,
                path,
            })
        };
    }
    let exe_name = if cfg!(windows) {
        "rspice.exe"
    } else {
        "rspice"
    };
    let mut tried = Vec::new();
    if let Ok(current) = env::current_exe() {
        let sibling = current.with_file_name(exe_name);
        if sibling.is_file() {
            return Ok(sibling);
        }
        tried.push(sibling);
    }
    let fallback = Path::new("target").join("release").join(exe_name);
    if fallback.is_file() {
        return Ok(fallback);
    }
    tried.push(fallback);
    Err(BenchError::MissingRspice { tried })
}

/// Resolves an optional context executable (`RSPICE_BENCH_NGSPICE`,
/// `RSPICE_BENCH_ESPICE`). An unset variable is a skip; a set-but-invalid
/// path is a hard error.
fn locate_context(env_var: &'static str) -> Result<Option<PathBuf>, BenchError> {
    match env::var_os(env_var) {
        None => Ok(None),
        Some(raw) => {
            let path = PathBuf::from(raw);
            if path.is_file() {
                Ok(Some(path))
            } else {
                Err(BenchError::ExeNotFound { env_var, path })
            }
        }
    }
}

/// This machine, as a baseline must match it.
pub(crate) fn host_info() -> HostInfo {
    HostInfo {
        os: format!("{} {}", env::consts::OS, env::consts::ARCH),
        cpu_count: std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1),
    }
}

/// One-, five- and fifteen-minute load averages: `/proc/loadavg` on Linux,
/// `sysctl -n vm.loadavg` on macOS. `None` elsewhere: Windows has no load
/// average, so a run there records the machine state only in `--label`.
pub(crate) fn load_average() -> Option<[f64; 3]> {
    parse_load_average(&load_average_text()?)
}

#[cfg(target_os = "linux")]
fn load_average_text() -> Option<String> {
    fs::read_to_string("/proc/loadavg").ok()
}

#[cfg(target_os = "macos")]
fn load_average_text() -> Option<String> {
    Command::new("sysctl")
        .args(["-n", "vm.loadavg"])
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn load_average_text() -> Option<String> {
    None
}

/// The first three numbers of `/proc/loadavg` (`0.52 0.58 0.59 1/389 12`)
/// or of macOS's `{ 0.52 0.58 0.59 }`.
fn parse_load_average(text: &str) -> Option<[f64; 3]> {
    let mut values = text
        .split(|character: char| character.is_whitespace() || character == '{' || character == '}')
        .filter(|field| !field.is_empty())
        .map(str::parse::<f64>);
    let mut next = || values.next()?.ok();
    Some([next()?, next()?, next()?])
}

/// The shipped deck set: the hand-written decks plus the generated ones,
/// written fresh so a run always measures the current generator. Sorted by
/// file name like [`discover_decks`].
fn shipped_decks() -> Result<Vec<PathBuf>, BenchError> {
    let generated = generate::write_decks(Path::new(GENERATED_DECKS_DIR))?;
    let mut decks = discover_decks(Path::new(SHIPPED_CIRCUITS_DIR))?;
    decks.extend(generated);
    decks.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    Ok(decks)
}

/// Collects `*.cir` decks from `dir` (non-recursive), sorted by file name
/// for a deterministic scoreboard order.
fn discover_decks(dir: &Path) -> Result<Vec<PathBuf>, BenchError> {
    let entries = fs::read_dir(dir).map_err(|err| {
        BenchError::io(
            format!("failed to read circuits directory `{}`", dir.display()),
            err,
        )
    })?;
    let mut decks = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|err| {
            BenchError::io(
                format!("failed to enumerate circuits in `{}`", dir.display()),
                err,
            )
        })?;
        let path = entry.path();
        let is_cir = path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("cir"));
        if is_cir && path.is_file() {
            decks.push(path);
        }
    }
    decks.sort();
    if decks.is_empty() {
        return Err(BenchError::NoDecks {
            dir: dir.to_path_buf(),
        });
    }
    Ok(decks)
}

/// Read the comparison input before the current scoreboard can overwrite it.
fn load_baseline(path: &Path) -> Result<BaselineScoreboard, BenchError> {
    let file = fs::File::open(path).map_err(|error| {
        BenchError::io(
            format!("failed to read benchmark baseline `{}`", path.display()),
            error,
        )
    })?;
    let mut bytes = Vec::new();
    file.take(MAX_BASELINE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| {
            BenchError::io(
                format!("failed to read benchmark baseline `{}`", path.display()),
                error,
            )
        })?;
    if bytes.len() as u64 > MAX_BASELINE_BYTES {
        return Err(BenchError::BenchmarkPolicy {
            message: format!(
                "benchmark baseline `{}` exceeds the {} MiB input limit",
                path.display(),
                MAX_BASELINE_BYTES / (1024 * 1024)
            ),
        });
    }
    serde_json::from_slice(&bytes).map_err(|error| BenchError::Json {
        context: format!(
            "failed to parse benchmark baseline `{}` as a scoreboard",
            path.display()
        ),
        source: error,
    })
}

/// Compare every current RSpice median to an exact same-deck baseline.
fn evaluate_regression(
    current: &Scoreboard,
    baseline: &BaselineScoreboard,
    baseline_path: &Path,
    max_regression_percent: f64,
    allow_host_mismatch: bool,
) -> Result<RegressionGate, BenchError> {
    if baseline.methodology != METHODOLOGY {
        return Err(BenchError::BenchmarkPolicy {
            message: format!(
                "baseline `{}` uses a different timing methodology and cannot be compared",
                baseline_path.display()
            ),
        });
    }
    if baseline.repeats != current.repeats {
        return Err(BenchError::BenchmarkPolicy {
            message: format!(
                "baseline `{}` used {} timed repeats per deck, but the current run uses {}; rerun with matching --repeats",
                baseline_path.display(),
                baseline.repeats,
                current.repeats
            ),
        });
    }

    let host_match = current.host == baseline.host;
    if !host_match && !allow_host_mismatch {
        return Err(BenchError::BenchmarkPolicy {
            message: format!(
                "baseline `{}` was recorded on {} with {} logical CPUs, but the current host is {} with {}; rerun on the baseline host or pass --allow-host-mismatch for an exploratory comparison",
                baseline_path.display(),
                baseline.host.os,
                baseline.host.cpu_count,
                current.host.os,
                current.host.cpu_count,
            ),
        });
    }

    let mut baseline_by_deck = BTreeMap::new();
    for result in &baseline.results {
        if baseline_by_deck
            .insert(result.deck.as_str(), result)
            .is_some()
        {
            return Err(BenchError::BenchmarkPolicy {
                message: format!(
                    "baseline `{}` contains duplicate deck `{}`",
                    baseline_path.display(),
                    result.deck
                ),
            });
        }
        if !result.rspice_all_ok {
            return Err(BenchError::BenchmarkPolicy {
                message: format!(
                    "baseline `{}` records a failed RSpice run for `{}`",
                    baseline_path.display(),
                    result.deck
                ),
            });
        }
        if !result.rspice_ms.median.is_finite() || result.rspice_ms.median <= 0.0 {
            return Err(BenchError::BenchmarkPolicy {
                message: format!(
                    "baseline `{}` has an invalid RSpice median for `{}`: {}",
                    baseline_path.display(),
                    result.deck,
                    result.rspice_ms.median
                ),
            });
        }
    }

    let current_decks: BTreeSet<&str> = current
        .results
        .iter()
        .map(|result| result.deck.as_str())
        .collect();
    let baseline_decks: BTreeSet<&str> = baseline_by_deck.keys().copied().collect();
    if current_decks != baseline_decks {
        let missing = current_decks
            .difference(&baseline_decks)
            .copied()
            .collect::<Vec<_>>();
        let stale = baseline_decks
            .difference(&current_decks)
            .copied()
            .collect::<Vec<_>>();
        return Err(BenchError::BenchmarkPolicy {
            message: format!(
                "baseline `{}` deck set differs from the current suite (missing from baseline: [{}]; absent from current suite: [{}])",
                baseline_path.display(),
                missing.join(", "),
                stale.join(", ")
            ),
        });
    }

    let mut results = Vec::with_capacity(current.results.len());
    for current_result in &current.results {
        let baseline_result = baseline_by_deck
            .get(current_result.deck.as_str())
            .ok_or(BenchError::Internal("validated baseline deck disappeared"))?;
        let current_median = current_result.rspice_ms.median;
        if !current_median.is_finite() || current_median <= 0.0 {
            return Err(BenchError::BenchmarkPolicy {
                message: format!(
                    "current scoreboard has an invalid RSpice median for `{}`: {}",
                    current_result.deck, current_median
                ),
            });
        }
        let change_percent = (current_median / baseline_result.rspice_ms.median - 1.0) * 100.0;
        let passed = current_result.rspice_all_ok && change_percent <= max_regression_percent;
        results.push(RegressionResult {
            deck: current_result.deck.clone(),
            baseline_median_ms: baseline_result.rspice_ms.median,
            current_median_ms: current_median,
            change_percent,
            passed,
        });
    }

    Ok(RegressionGate {
        baseline: baseline_path.display().to_string(),
        max_regression_percent,
        host_match,
        passed: results.iter().all(|result| result.passed),
        results,
    })
}

/// Serializes a report to pretty JSON and writes it to `out`, creating
/// parent directories as needed.
pub(crate) fn write_json(out: &Path, scoreboard: &impl Serialize) -> Result<(), BenchError> {
    if let Some(parent) = out.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent).map_err(|err| {
            BenchError::io(
                format!(
                    "failed to create scoreboard directory `{}`",
                    parent.display()
                ),
                err,
            )
        })?;
    }
    let mut json = serde_json::to_string_pretty(scoreboard).map_err(|err| BenchError::Json {
        context: "failed to serialize the scoreboard to JSON".to_string(),
        source: err,
    })?;
    json.push('\n');
    fs::write(out, json).map_err(|err| {
        BenchError::io(
            format!("failed to write scoreboard `{}`", out.display()),
            err,
        )
    })
}

/// The table's machine-state lines: the label, and a note where the
/// platform reports no load average for the `load1` column.
pub(crate) fn print_machine_state(label: Option<&str>, load_reported: bool) {
    if let Some(label) = label {
        println!("label: {label}");
    }
    if !load_reported {
        println!(
            "load average: not reported on {}; the label records the machine state",
            env::consts::OS
        );
    }
}

/// Prints the aligned, human-readable results table to stdout.
fn print_table(results: &[DeckResult], floor_ms: f64, label: Option<&str>) {
    let name_width = results
        .iter()
        .map(|result| result.deck.len())
        .chain(std::iter::once("deck".len()))
        .max()
        .unwrap_or(4);
    print_machine_state(
        label,
        results.iter().any(|result| result.load_average.is_some()),
    );
    println!("spawn floor: median {floor_ms:.2} ms");
    let espice = results.iter().any(|result| result.espice_ms.is_some());
    // A workload ngspice has no counterpart for (`--version`, `check`)
    // shows `-` in its columns; "skipped" means no ngspice was configured.
    let ngspice_configured = results.iter().any(|result| result.ngspice_ms.is_some());
    println!(
        "{:<name_width$}  {:<16}  {:>14}  {:>14}  {:>13}  {:>15}  {:>15}  {:>8}{}  {:>6}  status",
        "deck",
        "command",
        "rspice med ms",
        "rspice min ms",
        "above floor",
        "ngspice med ms",
        "ngspice min ms",
        "speedup",
        if espice { "  espice med ms" } else { "" },
        "load1"
    );
    for result in results {
        let (ngspice_median, ngspice_min) = match result.ngspice_ms {
            Some(stats) => (format!("{:.1}", stats.median), format!("{:.1}", stats.min)),
            None => ("-".to_string(), "-".to_string()),
        };
        let espice_column = match (espice, result.espice_ms) {
            (false, _) => String::new(),
            (true, Some(stats)) => format!("  {:>13.1}", stats.median),
            (true, None) => format!("  {:>13}", "-"),
        };
        let speedup = match result.speedup_median {
            Some(ratio) => format!("{ratio:.2}x"),
            None => "-".to_string(),
        };
        let status = if !result.rspice_all_ok {
            "rspice FAILED"
        } else if result.ngspice_all_ok == Some(false) {
            "ngspice FAILED"
        } else if result.espice_all_ok == Some(false) {
            "espice FAILED"
        } else if !ngspice_configured {
            "ngspice skipped"
        } else {
            "ok"
        };
        let load = result
            .load_average
            .map_or_else(|| "-".to_string(), |load| format!("{:.1}", load[0]));
        println!(
            "{:<name_width$}  {:<16}  {:>14.1}  {:>14.1}  {:>13.1}  {:>15}  {:>15}  {:>8}{espice_column}  {load:>6}  {status}",
            result.deck,
            result.command,
            result.rspice_ms.median,
            result.rspice_ms.min,
            result.rspice_above_floor_ms,
            ngspice_median,
            ngspice_min,
            speedup
        );
    }
}

/// Print the baseline gate after the ordinary simulator comparison table.
fn print_regression_gate(gate: &RegressionGate) {
    println!(
        "RSpice regression gate: baseline `{}`, allowed +{:.2}% (host match: {})",
        gate.baseline, gate.max_regression_percent, gate.host_match
    );
    println!(
        "{:<28}  {:>15}  {:>15}  {:>10}  status",
        "deck", "baseline med ms", "current med ms", "change"
    );
    for result in &gate.results {
        println!(
            "{:<28}  {:>15.1}  {:>15.1}  {:>+9.2}%  {}",
            result.deck,
            result.baseline_median_ms,
            result.current_median_ms,
            result.change_percent,
            if result.passed { "ok" } else { "REGRESSION" }
        );
    }
    println!("gate: {}", if gate.passed { "PASS" } else { "FAIL" });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn timing(median: f64) -> TimingStats {
        TimingStats {
            min: median,
            median,
            mean: median,
        }
    }

    fn current_scoreboard(results: &[(&str, f64)]) -> Scoreboard {
        Scoreboard {
            generated_with: "rspice-bench",
            host: HostInfo {
                os: "test x86_64".to_string(),
                cpu_count: 8,
            },
            label: None,
            suite: None,
            load_average_start: None,
            load_average_end: None,
            spawn_floor_ms: timing(1.0),
            repeats: 5,
            methodology: METHODOLOGY,
            rspice_exe: "rspice".to_string(),
            ngspice_exe: None,
            ngspice_note: Some(NGSPICE_SKIPPED_NOTE),
            espice_exe: None,
            results: results
                .iter()
                .map(|(deck, median)| DeckResult {
                    deck: (*deck).to_string(),
                    suite: "circuits",
                    command: "run -q",
                    load_average: None,
                    rspice_ms: timing(*median),
                    rspice_above_floor_ms: *median - 1.0,
                    rspice_all_ok: true,
                    ngspice_ms: None,
                    ngspice_all_ok: None,
                    espice_ms: None,
                    espice_all_ok: None,
                    speedup_median: None,
                    both_succeeded: false,
                })
                .collect(),
            regression_gate: None,
        }
    }

    fn baseline_scoreboard(results: &[(&str, f64)]) -> BaselineScoreboard {
        BaselineScoreboard {
            host: HostInfo {
                os: "test x86_64".to_string(),
                cpu_count: 8,
            },
            repeats: 5,
            methodology: METHODOLOGY.to_string(),
            results: results
                .iter()
                .map(|(deck, median)| BaselineDeckResult {
                    deck: (*deck).to_string(),
                    rspice_ms: timing(*median),
                    rspice_all_ok: true,
                })
                .collect(),
        }
    }

    #[test]
    fn regression_gate_compares_every_deck_and_fails_over_budget() {
        let current = current_scoreboard(&[("fast.cir", 109.0), ("slow.cir", 121.0)]);
        let baseline = baseline_scoreboard(&[("fast.cir", 100.0), ("slow.cir", 100.0)]);

        let gate =
            evaluate_regression(&current, &baseline, Path::new("baseline.json"), 10.0, false)
                .expect("comparable baseline");

        assert!(gate.host_match);
        assert!(!gate.passed);
        assert!(gate.results[0].passed);
        assert!(!gate.results[1].passed);
        assert!((gate.results[0].change_percent - 9.0).abs() < 1.0e-12);
        assert!((gate.results[1].change_percent - 21.0).abs() < 1.0e-12);
    }

    #[test]
    fn regression_gate_rejects_incomparable_hosts_and_deck_sets() {
        let current = current_scoreboard(&[("only.cir", 10.0)]);
        let mut other_repeats = baseline_scoreboard(&[("only.cir", 10.0)]);
        other_repeats.repeats = 7;
        assert!(matches!(
            evaluate_regression(
                &current,
                &other_repeats,
                Path::new("other-repeats.json"),
                10.0,
                false,
            ),
            Err(BenchError::BenchmarkPolicy { message }) if message.contains("matching --repeats")
        ));

        let mut other_host = baseline_scoreboard(&[("only.cir", 10.0)]);
        other_host.host.cpu_count = 16;
        assert!(matches!(
            evaluate_regression(
                &current,
                &other_host,
                Path::new("other-host.json"),
                10.0,
                false,
            ),
            Err(BenchError::BenchmarkPolicy { message }) if message.contains("--allow-host-mismatch")
        ));

        let other_deck = baseline_scoreboard(&[("stale.cir", 10.0)]);
        assert!(matches!(
            evaluate_regression(
                &current,
                &other_deck,
                Path::new("stale.json"),
                10.0,
                false,
            ),
            Err(BenchError::BenchmarkPolicy { message }) if message.contains("deck set differs")
        ));
    }

    #[test]
    fn load_averages_parse_from_linux_and_macos_text() {
        assert_eq!(
            parse_load_average("0.52 0.58 0.59 1/389 12\n"),
            Some([0.52, 0.58, 0.59])
        );
        assert_eq!(
            parse_load_average("{ 1.25 2.50 3.75 }\n"),
            Some([1.25, 2.5, 3.75])
        );
        assert_eq!(parse_load_average("0.5 0.6"), None);
        if let Some([one, five, fifteen]) = load_average() {
            assert!(one >= 0.0 && five >= 0.0 && fifteen >= 0.0);
        }
    }

    #[test]
    fn a_filter_that_matches_nothing_is_refused() {
        let workloads = vec![Workload::run_quiet(PathBuf::from("a/rc.cir"))];
        assert_eq!(
            filter_workloads(workloads.clone(), Some("rc"))
                .unwrap()
                .len(),
            1
        );
        assert!(filter_workloads(workloads, Some("mesh")).is_err());
    }

    #[test]
    fn timing_statistics_are_order_independent_and_use_true_median() {
        let stats = timing_stats(&[9.0, 1.0, 5.0, 3.0]).expect("non-empty samples");
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.median, 4.0);
        assert_eq!(stats.mean, 4.5);
    }
}
