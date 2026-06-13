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
use serde::Serialize;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::Instant;

/// Environment variable overriding the RSpice executable location.
const RSPICE_ENV: &str = "RSPICE_BENCH_RSPICE";

/// Environment variable supplying the ngspice executable location. When it
/// is not set the ngspice column is skipped and noted in the scoreboard.
const NGSPICE_ENV: &str = "RSPICE_BENCH_NGSPICE";

/// Timing-methodology note embedded in every scoreboard.
const METHODOLOGY: &str = "wall-clock of the full child process (parse + solve + output) timed \
     with std::time::Instant around spawn/wait; child stdin/stdout/stderr attached to the null \
     device; one untimed warmup run per deck/simulator precedes the timed repeats; cold OS \
     cache not controlled";

/// Note recorded in the scoreboard when ngspice timings are skipped.
const NGSPICE_SKIPPED_NOTE: &str =
    "RSPICE_BENCH_NGSPICE not set; ngspice columns skipped for this run";

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
}

/// Host descriptor recorded in the scoreboard.
#[derive(Serialize)]
struct HostInfo {
    /// Operating system and architecture, e.g. `windows x86_64`.
    os: String,
    /// Logical CPU count reported by the OS.
    cpu_count: usize,
}

/// Wall-clock statistics over the timed repeats, in milliseconds.
#[derive(Serialize, Clone, Copy)]
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

    let scoreboard = Scoreboard {
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
    };
    write_scoreboard(&args.out, &scoreboard)?;

    println!();
    print_table(&scoreboard.results);
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

    loop {
        match child.try_wait().map_err(|err| {
            BenchError::io(
                format!("failed to poll `{}` during benchmarking", exe.display()),
                err,
            )
        })? {
            Some(status) => {
                return Ok((start.elapsed().as_secs_f64() * 1e3, status.success()));
            }
            None if start.elapsed() >= timeout => {
                let _ = child.kill();
                let _ = child.wait();
                eprintln!(
                    "  TIMEOUT: `{}` exceeded {}s and was killed",
                    exe.display(),
                    timeout.as_secs()
                );
                return Ok((start.elapsed().as_secs_f64() * 1e3, false));
            }
            None => std::thread::sleep(std::time::Duration::from_millis(5)),
        }
    }
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
    let median = if count % 2 == 0 {
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
    let exe_name = if cfg!(windows) { "rspice.exe" } else { "rspice" };
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

/// Serializes the scoreboard to pretty JSON and writes it to `out`,
/// creating parent directories as needed.
fn write_scoreboard(out: &Path, scoreboard: &Scoreboard) -> Result<(), BenchError> {
    if let Some(parent) = out.parent() {
        if !parent.as_os_str().is_empty() {
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
