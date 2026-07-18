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
use clap::Args;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::Instant;
use wait_timeout::ChildExt;

/// Environment variable overriding the RSpice executable location.
const RSPICE_ENV: &str = "RSPICE_BENCH_RSPICE";

/// Environment variable supplying the ngspice executable location. When it
/// is not set the ngspice column is skipped and noted in the scoreboard.
const NGSPICE_ENV: &str = "RSPICE_BENCH_NGSPICE";

/// Timing-methodology note embedded in every scoreboard.
const METHODOLOGY: &str = "wall-clock of the full child process (parse + solve + output) timed \
     with std::time::Instant around spawn and an OS-backed timed wait (no polling interval); child \
     stdin/stdout/stderr attached to the null device; one untimed warmup run per deck/simulator \
     precedes the timed repeats; cold OS cache not controlled";

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

    /// Path of the JSON scoreboard to write. The rig never date-stamps or
    /// rotates this file itself; pass an explicit dated path to archive a run.
    #[arg(
        long,
        default_value = "benchmarks/scoreboards/scoreboard.json",
        value_name = "PATH"
    )]
    pub out: PathBuf,

    /// Directory scanned (non-recursively) for benchmark decks (*.cir).
    #[arg(long, default_value = "benchmarks/circuits", value_name = "DIR")]
    pub circuits: PathBuf,

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
}

/// Complete scoreboard document serialized to `--out`.
#[derive(Serialize)]
struct Scoreboard {
    /// Tool that produced the document.
    generated_with: &'static str,
    /// Machine the run was executed on.
    host: HostInfo,
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
    /// Per-deck timing results, in deck order.
    results: Vec<DeckResult>,
    /// Baseline comparison report when `--baseline` was supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    regression_gate: Option<RegressionGate>,
}

/// Host descriptor recorded in the scoreboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct HostInfo {
    /// Operating system and architecture, e.g. `windows x86_64`.
    os: String,
    /// Logical CPU count reported by the OS.
    cpu_count: usize,
}

/// Wall-clock statistics over the timed repeats, in milliseconds.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct TimingStats {
    /// Fastest timed run.
    min: f64,
    /// Median of the timed runs (midpoint average for even counts).
    median: f64,
    /// Arithmetic mean of the timed runs.
    mean: f64,
}

/// Result row for a single benchmark deck.
#[derive(Serialize)]
struct DeckResult {
    /// Deck file name (relative to the circuits directory).
    deck: String,
    /// RSpice timing statistics in milliseconds.
    rspice_ms: TimingStats,
    /// Whether every RSpice run (warmup included) exited with status 0.
    rspice_all_ok: bool,
    /// ngspice timing statistics in milliseconds; `null` when skipped.
    ngspice_ms: Option<TimingStats>,
    /// Whether every ngspice run exited with status 0; `null` when skipped.
    ngspice_all_ok: Option<bool>,
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
    let ngspice = locate_ngspice()?;
    let decks = discover_decks(&args.circuits)?;

    println!("rspice : {}", rspice.display());
    match &ngspice {
        Some(path) => println!("ngspice: {}", path.display()),
        None => println!("ngspice: skipped ({NGSPICE_ENV} not set)"),
    }
    println!(
        "decks  : {} from `{}`; repeats: {} (+1 warmup)\n",
        decks.len(),
        args.circuits.display(),
        args.repeats
    );

    let timeout = std::time::Duration::from_secs(args.timeout_secs);
    let mut results = Vec::with_capacity(decks.len());
    let mut any_failure = false;
    for deck in &decks {
        let result = bench_deck(deck, &rspice, ngspice.as_deref(), args.repeats, timeout)?;
        any_failure |= !result.rspice_all_ok || result.ngspice_all_ok == Some(false);
        results.push(result);
    }

    let mut scoreboard = Scoreboard {
        generated_with: "rspice-bench",
        host: HostInfo {
            os: format!("{} {}", env::consts::OS, env::consts::ARCH),
            cpu_count: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1),
        },
        repeats: args.repeats,
        methodology: METHODOLOGY,
        rspice_exe: rspice.display().to_string(),
        ngspice_exe: ngspice.as_ref().map(|p| p.display().to_string()),
        ngspice_note: ngspice.is_none().then_some(NGSPICE_SKIPPED_NOTE),
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
    write_scoreboard(&args.out, &scoreboard)?;

    println!();
    print_table(&scoreboard.results);
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

/// Benchmarks one deck on RSpice and (when configured) ngspice.
fn bench_deck(
    deck: &Path,
    rspice: &Path,
    ngspice: Option<&Path>,
    repeats: u32,
    timeout: std::time::Duration,
) -> Result<DeckResult, BenchError> {
    let deck_file = deck
        .file_name()
        .ok_or(BenchError::Internal("deck path has no file name"))?;
    let deck_name = deck_file.to_string_lossy().into_owned();
    // Children run inside the circuits directory and receive the bare file
    // name, so any relative artifacts a simulator might emit land there.
    let cwd = deck.parent().unwrap_or_else(|| Path::new("."));

    println!("benchmarking {deck_name}");
    let rspice_args = [OsStr::new("run"), deck_file, OsStr::new("-q")];
    let rspice_run = measure(rspice, &rspice_args, cwd, repeats, timeout)?;
    print_progress("rspice", &rspice_run);

    let ngspice_run = match ngspice {
        Some(exe) => {
            let ngspice_args = [OsStr::new("-b"), deck_file];
            let measurement = measure(exe, &ngspice_args, cwd, repeats, timeout)?;
            print_progress("ngspice", &measurement);
            Some(measurement)
        }
        None => None,
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
        deck: deck_name,
        rspice_ms: rspice_run.stats,
        rspice_all_ok: rspice_run.all_ok,
        ngspice_ms: ngspice_run.as_ref().map(|measurement| measurement.stats),
        ngspice_all_ok: ngspice_run.as_ref().map(|measurement| measurement.all_ok),
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

/// Runs one warmup plus `repeats` timed executions of `exe args` in `cwd`.
/// A run that exceeds `timeout` is killed and the whole measurement is
/// marked failed; the remaining repeats are skipped (they would only
/// repeat the timeout).
fn measure(
    exe: &Path,
    args: &[&OsStr],
    cwd: &Path,
    repeats: u32,
    timeout: std::time::Duration,
) -> Result<SimMeasurement, BenchError> {
    let (_, warmup_ok) = time_child(exe, args, cwd, timeout)?;
    let mut all_ok = warmup_ok;
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
fn time_child(
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
fn timing_stats(samples_ms: &[f64]) -> Option<TimingStats> {
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
fn locate_rspice() -> Result<PathBuf, BenchError> {
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

/// Resolves the optional ngspice executable from `RSPICE_BENCH_NGSPICE`.
/// An unset variable is a skip; a set-but-invalid path is a hard error.
fn locate_ngspice() -> Result<Option<PathBuf>, BenchError> {
    match env::var_os(NGSPICE_ENV) {
        None => Ok(None),
        Some(raw) => {
            let path = PathBuf::from(raw);
            if path.is_file() {
                Ok(Some(path))
            } else {
                Err(BenchError::ExeNotFound {
                    env_var: NGSPICE_ENV,
                    path,
                })
            }
        }
    }
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

/// Serializes the scoreboard to pretty JSON and writes it to `out`,
/// creating parent directories as needed.
fn write_scoreboard(out: &Path, scoreboard: &Scoreboard) -> Result<(), BenchError> {
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

/// Prints the aligned, human-readable results table to stdout.
fn print_table(results: &[DeckResult]) {
    let name_width = results
        .iter()
        .map(|result| result.deck.len())
        .chain(std::iter::once("deck".len()))
        .max()
        .unwrap_or(4);
    println!(
        "{:<name_width$}  {:>14}  {:>14}  {:>15}  {:>15}  {:>8}  status",
        "deck", "rspice med ms", "rspice min ms", "ngspice med ms", "ngspice min ms", "speedup"
    );
    for result in results {
        let (ngspice_median, ngspice_min) = match result.ngspice_ms {
            Some(stats) => (format!("{:.1}", stats.median), format!("{:.1}", stats.min)),
            None => ("-".to_string(), "-".to_string()),
        };
        let speedup = match result.speedup_median {
            Some(ratio) => format!("{ratio:.2}x"),
            None => "-".to_string(),
        };
        let status = if !result.rspice_all_ok {
            "rspice FAILED"
        } else if result.ngspice_ms.is_none() {
            "ngspice skipped"
        } else if result.ngspice_all_ok == Some(false) {
            "ngspice FAILED"
        } else {
            "ok"
        };
        println!(
            "{:<name_width$}  {:>14.1}  {:>14.1}  {:>15}  {:>15}  {:>8}  {status}",
            result.deck,
            result.rspice_ms.median,
            result.rspice_ms.min,
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
            repeats: 5,
            methodology: METHODOLOGY,
            rspice_exe: "rspice".to_string(),
            ngspice_exe: None,
            ngspice_note: Some(NGSPICE_SKIPPED_NOTE),
            results: results
                .iter()
                .map(|(deck, median)| DeckResult {
                    deck: (*deck).to_string(),
                    rspice_ms: timing(*median),
                    rspice_all_ok: true,
                    ngspice_ms: None,
                    ngspice_all_ok: None,
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
    fn timing_statistics_are_order_independent_and_use_true_median() {
        let stats = timing_stats(&[9.0, 1.0, 5.0, 3.0]).expect("non-empty samples");
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.median, 4.0);
        assert_eq!(stats.mean, 4.5);
    }
}
