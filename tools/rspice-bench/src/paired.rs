//! Paired A/B timing of two RSpice executables over one population.
//!
//! Each workload first runs once on A and once on B writing a rawfile; the
//! two rawfiles are compared (byte-identical by default, or within
//! `--tolerance`), and a workload whose outputs differ is DISQUALIFIED: it
//! is reported but excluded from every summary figure, because a speed
//! change that moved the answer is not a speed change. Those comparison
//! runs also warm the file cache. Then `--warmups` further untimed pairs
//! and `--pairs` timed pairs run, alternating the order (A then B, then B
//! then A) so slow drift in machine load falls on both sides alike. Each
//! timed pair gives one B/A ratio; a workload reports the median ratio with
//! its spread, and the run reports the geometric mean of those medians over
//! the workloads that counted.

use crate::error::BenchError;
use crate::population::{Action, Suite, Workload};
use crate::rawcmp::{self, Comparison};
use crate::runner::{self, HostInfo, TimingStats};
use clap::Args;
use serde::Serialize;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Duration;

/// Timing-methodology note embedded in every paired report.
const METHODOLOGY: &str = "wall-clock of the full child process timed with std::time::Instant \
     around spawn and an OS-backed timed wait; child stdio attached to the null device; one \
     untimed rawfile-writing run per executable first (outputs compared, also the cache warm-up), \
     then untimed warm-up pairs, then timed pairs in alternating order (AB, BA, ...); ratio = B/A \
     per pair, summarized by the median; disqualified and failed workloads are excluded from the \
     geometric mean";

/// Arguments for the `paired` subcommand.
#[derive(Args, Debug)]
pub struct PairedArgs {
    /// Executable A, the reference (for example a build of main).
    #[arg(long, value_name = "EXE")]
    pub a: PathBuf,

    /// Executable B, the candidate. Ratios are B/A: below 1.0 B is faster.
    #[arg(long, value_name = "EXE")]
    pub b: PathBuf,

    /// Timed pairs per workload.
    #[arg(long, default_value_t = 10, value_parser = clap::value_parser!(u32).range(1..))]
    pub pairs: u32,

    /// Timed pairs of a minute-scale workload (ISCAS), which takes no
    /// warm-up pairs: its output comparison runs already warmed it.
    #[arg(long, default_value_t = 3, value_parser = clap::value_parser!(u32).range(1..))]
    pub heavy_pairs: u32,

    /// Untimed warm-up pairs per workload after the output comparison.
    #[arg(long, default_value_t = 1)]
    pub warmups: u32,

    /// The population timed (decks generated into target/benchmarks/decks/).
    #[arg(
        long,
        value_enum,
        default_value = "neospice",
        conflicts_with = "circuits"
    )]
    pub suite: Suite,

    /// Time `run -q` over every deck of this directory instead of a suite.
    #[arg(long, value_name = "DIR")]
    pub circuits: Option<PathBuf>,

    /// Time only the workloads whose id contains this text.
    #[arg(long, value_name = "TEXT")]
    pub filter: Option<String>,

    /// Accept outputs that differ by at most this relative amount (per value,
    /// scaled by the signal's peak near zero) instead of requiring
    /// byte-identical rawfiles.
    #[arg(long, value_name = "REL")]
    pub tolerance: Option<f64>,

    /// Per-run wall-clock cap in seconds; a run over it is killed and the
    /// workload counted as failed.
    #[arg(long, default_value_t = 120, value_parser = clap::value_parser!(u64).range(1..))]
    pub timeout_secs: u64,

    /// Path of the JSON report (overwritten).
    #[arg(
        long,
        default_value = "target/benchmarks/paired.json",
        value_name = "PATH"
    )]
    pub out: PathBuf,

    /// Free-text label recorded in the report, such as the machine state.
    #[arg(long, value_name = "TEXT")]
    pub label: Option<String>,
}

/// What became of one workload.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Status {
    /// Outputs agreed (or there were none) and every run exited 0.
    Ok,
    /// The outputs of A and B differ: excluded from the summary.
    Disqualified,
    /// A run exited non-zero or timed out: excluded from the summary.
    Failed,
}

impl Status {
    fn label(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Disqualified => "DISQUALIFIED",
            Self::Failed => "FAILED",
        }
    }
}

/// One workload's paired result.
#[derive(Debug, Serialize)]
struct PairedResult {
    id: String,
    suite: &'static str,
    command: &'static str,
    status: Status,
    /// Why the workload did not count, when it did not.
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    /// Timed pairs completed.
    pairs: usize,
    /// A's and B's wall time over the timed runs, in milliseconds.
    a_ms: Option<TimingStats>,
    b_ms: Option<TimingStats>,
    /// Median, smallest and largest per-pair B/A ratio.
    ratio_median: Option<f64>,
    ratio_min: Option<f64>,
    ratio_max: Option<f64>,
    /// The rawfile comparison; absent for workloads that write no result.
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<Comparison>,
    /// One-, five- and fifteen-minute load averages around the workload.
    load_average_start: Option<[f64; 3]>,
    load_average_end: Option<[f64; 3]>,
}

/// The JSON document `paired` writes.
#[derive(Debug, Serialize)]
struct PairedReport {
    generated_with: &'static str,
    host: HostInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    source: String,
    a_exe: String,
    b_exe: String,
    pairs: u32,
    heavy_pairs: u32,
    warmups: u32,
    /// `None`: byte-identical outputs required.
    tolerance: Option<f64>,
    methodology: &'static str,
    spawn_floor_ms: TimingStats,
    load_average_start: Option<[f64; 3]>,
    load_average_end: Option<[f64; 3]>,
    workloads: Vec<PairedResult>,
    /// Geometric mean of the counted workloads' median B/A ratios.
    geomean_ratio: Option<f64>,
    counted: usize,
    disqualified: usize,
    failed: usize,
}

/// Entry point of the `paired` subcommand. Exits with failure (after the
/// report is written) when any workload was disqualified or failed.
pub fn run(args: &PairedArgs) -> Result<ExitCode, BenchError> {
    if let Some(tolerance) = args.tolerance
        && !(tolerance.is_finite() && tolerance >= 0.0)
    {
        return Err(BenchError::BenchmarkPolicy {
            message: format!("--tolerance must be a finite non-negative number, got {tolerance}"),
        });
    }
    for (flag, exe) in [("--a", &args.a), ("--b", &args.b)] {
        if !exe.is_file() {
            return Err(BenchError::BenchmarkPolicy {
                message: format!("{flag} `{}` is not a file", exe.display()),
            });
        }
    }
    let suite = args.circuits.is_none().then_some(args.suite);
    let (workloads, source) = runner::select_workloads(suite, args.circuits.as_deref())?;
    let workloads = runner::filter_workloads(workloads, args.filter.as_deref())?;
    let timeout = Duration::from_secs(args.timeout_secs);

    println!("A: {}", args.a.display());
    println!("B: {}", args.b.display());
    println!(
        "workloads: {} from {source}; {} timed pairs (+{} warm-up), outputs {}\n",
        workloads.len(),
        args.pairs,
        args.warmups,
        match args.tolerance {
            Some(tolerance) => format!("within {tolerance:e} relative"),
            None => "byte-identical".to_string(),
        }
    );
    let load_average_start = runner::load_average();
    let spawn_floor = runner::spawn_floor(args.pairs, timeout)?;
    let mut results = Vec::with_capacity(workloads.len());
    for workload in &workloads {
        let result = pair_workload(workload, args, timeout)?;
        println!(
            "  {:<32} {:>8}  {}",
            result.id,
            result
                .ratio_median
                .map_or_else(|| "-".to_string(), |ratio| format!("{ratio:.3}")),
            result.detail.as_deref().unwrap_or(result.status.label()),
        );
        results.push(result);
    }

    let counted: Vec<f64> = results
        .iter()
        .filter(|result| result.status == Status::Ok)
        .filter_map(|result| result.ratio_median)
        .collect();
    let count = |status| results.iter().filter(|r| r.status == status).count();
    let report = PairedReport {
        generated_with: "rspice-bench paired",
        host: runner::host_info(),
        label: args.label.clone(),
        source,
        a_exe: args.a.display().to_string(),
        b_exe: args.b.display().to_string(),
        pairs: args.pairs,
        heavy_pairs: args.heavy_pairs,
        warmups: args.warmups,
        tolerance: args.tolerance,
        methodology: METHODOLOGY,
        spawn_floor_ms: spawn_floor,
        load_average_start,
        load_average_end: runner::load_average(),
        geomean_ratio: geometric_mean(&counted),
        counted: counted.len(),
        disqualified: count(Status::Disqualified),
        failed: count(Status::Failed),
        workloads: results,
    };
    runner::write_json(&args.out, &report)?;
    println!();
    print_table(&report);
    println!("\nreport written to {}", args.out.display());
    Ok(if report.disqualified + report.failed > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

/// Run one executable once; `(milliseconds, exited zero)`.
fn run_once(
    workload: &Workload,
    exe: &Path,
    raw: Option<&Path>,
    timeout: Duration,
) -> Result<(f64, bool), BenchError> {
    let (program, args) = workload.rspice_command(exe, raw)?;
    let args: Vec<&OsStr> = args.iter().map(OsString::as_os_str).collect();
    runner::time_child(&program, &args, &workload.cwd(), timeout)
}

fn pair_workload(
    workload: &Workload,
    args: &PairedArgs,
    timeout: Duration,
) -> Result<PairedResult, BenchError> {
    let scratch = tempfile::tempdir()
        .map_err(|error| BenchError::io("failed to create a scratch directory", error))?;
    let mut result = PairedResult {
        id: workload.id.clone(),
        suite: workload.suite,
        command: workload.action.label(),
        status: Status::Ok,
        detail: None,
        pairs: 0,
        a_ms: None,
        b_ms: None,
        ratio_median: None,
        ratio_min: None,
        ratio_max: None,
        output: None,
        load_average_start: runner::load_average(),
        load_average_end: None,
    };
    let fail = |mut result: PairedResult, detail: String| {
        result.status = Status::Failed;
        result.detail = Some(detail);
        result.load_average_end = runner::load_average();
        result
    };

    // The comparison runs: the result each side writes, and the warm-up.
    // Each side writes into a directory of its own, because a deck with
    // several analyses writes one rawfile per analysis beside the name given.
    if workload.action.simulates() {
        let mut outputs = Vec::with_capacity(2);
        for (side, exe) in [("a", &args.a), ("b", &args.b)] {
            let dir = scratch.path().join(side);
            std::fs::create_dir_all(&dir)
                .map_err(|error| BenchError::io("failed to create a rawfile directory", error))?;
            let (_, ok) = run_once(workload, exe, Some(&dir.join("out.raw")), timeout)?;
            if !ok {
                let side = side.to_ascii_uppercase();
                return Ok(fail(result, format!("{side} failed writing its rawfile")));
            }
            outputs.push(dir);
        }
        let comparison = compare_dirs(&outputs[0], &outputs[1], args.tolerance)?;
        let matches = comparison.matches;
        result.output = Some(comparison);
        if !matches {
            result.status = Status::Disqualified;
            result.detail = Some("DISQUALIFIED: outputs differ".to_string());
            result.load_average_end = runner::load_average();
            return Ok(result);
        }
    }

    // A timed RunRaw workload writes its rawfile every time, as it would
    // in use; each side to its own file.
    let timed_raw =
        |name: &str| (workload.action == Action::RunRaw).then(|| scratch.path().join(name));
    let (raw_a, raw_b) = (timed_raw("timed_a.raw"), timed_raw("timed_b.raw"));
    let (warmups, pairs) = if workload.heavy {
        (0, args.heavy_pairs)
    } else {
        (args.warmups, args.pairs)
    };
    let mut a_ms = Vec::with_capacity(pairs as usize);
    let mut b_ms = Vec::with_capacity(pairs as usize);
    for index in 0..warmups + pairs {
        let a_first = index % 2 == 0;
        let mut times = [(0.0, true); 2];
        for side in 0..2 {
            let run_a = (side == 0) == a_first;
            let (exe, raw) = if run_a {
                (&args.a, raw_a.as_deref())
            } else {
                (&args.b, raw_b.as_deref())
            };
            times[usize::from(!run_a)] = run_once(workload, exe, raw, timeout)?;
        }
        let [(a, ok_a), (b, ok_b)] = times;
        if !(ok_a && ok_b) {
            let side = if ok_a { "B" } else { "A" };
            return Ok(fail(result, format!("{side} failed in pair {}", index + 1)));
        }
        if index >= warmups {
            a_ms.push(a);
            b_ms.push(b);
        }
    }
    let ratios: Vec<f64> = a_ms
        .iter()
        .zip(&b_ms)
        .filter(|(a, _)| **a > 0.0)
        .map(|(a, b)| b / a)
        .collect();
    result.pairs = a_ms.len();
    result.a_ms = runner::timing_stats(&a_ms);
    result.b_ms = runner::timing_stats(&b_ms);
    if let Some(summary) = runner::timing_stats(&ratios) {
        result.ratio_median = Some(summary.median);
        result.ratio_min = Some(summary.min);
        result.ratio_max = ratios.iter().copied().reduce(f64::max);
    }
    result.load_average_end = runner::load_average();
    Ok(result)
}

/// The rawfiles a run wrote into `dir`, by name.
fn rawfiles(dir: &Path) -> Result<Vec<(String, Vec<u8>)>, BenchError> {
    let entries = std::fs::read_dir(dir).map_err(|error| {
        BenchError::io(
            format!("failed to list rawfiles in `{}`", dir.display()),
            error,
        )
    })?;
    let mut files = Vec::new();
    for entry in entries {
        let path = entry
            .map_err(|error| BenchError::io("failed to list rawfiles", error))?
            .path();
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let bytes = std::fs::read(&path).map_err(|error| {
            BenchError::io(
                format!("failed to read rawfile `{}`", path.display()),
                error,
            )
        })?;
        files.push((name, bytes));
    }
    files.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(files)
}

/// Compare every rawfile A wrote with the one of the same name B wrote.
fn compare_dirs(a: &Path, b: &Path, tolerance: Option<f64>) -> Result<Comparison, BenchError> {
    let (files_a, files_b) = (rawfiles(a)?, rawfiles(b)?);
    let names = |files: &[(String, Vec<u8>)]| {
        files
            .iter()
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>()
    };
    if names(&files_a) != names(&files_b) {
        return Ok(Comparison {
            matches: false,
            plots: 0,
            values: 0,
            max_relative_difference: f64::INFINITY,
            detail: Some(format!(
                "A wrote {:?}, B wrote {:?}",
                names(&files_a),
                names(&files_b)
            )),
        });
    }
    let mut total = Comparison {
        matches: true,
        plots: 0,
        values: 0,
        max_relative_difference: 0.0,
        detail: files_a
            .is_empty()
            .then(|| "neither side wrote a rawfile".to_string()),
    };
    for ((name, bytes_a), (_, bytes_b)) in files_a.iter().zip(&files_b) {
        let comparison = rawcmp::compare(bytes_a, bytes_b, tolerance);
        total.plots += comparison.plots;
        total.values += comparison.values;
        total.max_relative_difference = total
            .max_relative_difference
            .max(comparison.max_relative_difference);
        if !comparison.matches {
            total.matches = false;
            total.detail = Some(format!("{name}: {}", comparison.detail.unwrap_or_default()));
            break;
        }
    }
    Ok(total)
}

/// Geometric mean of positive ratios; `None` for an empty set.
fn geometric_mean(ratios: &[f64]) -> Option<f64> {
    let positive: Vec<f64> = ratios.iter().copied().filter(|r| *r > 0.0).collect();
    if positive.is_empty() {
        return None;
    }
    let log_sum: f64 = positive.iter().map(|ratio| ratio.ln()).sum();
    Some((log_sum / positive.len() as f64).exp())
}

fn format_option(value: Option<f64>, precision: usize) -> String {
    value.map_or_else(|| "-".to_string(), |value| format!("{value:.precision$}"))
}

fn print_table(report: &PairedReport) {
    let width = report
        .workloads
        .iter()
        .map(|result| result.id.len())
        .chain(std::iter::once("workload".len()))
        .max()
        .unwrap_or(8);
    runner::print_machine_state(report.label.as_deref(), report.load_average_start.is_some());
    println!(
        "A: {}\nB: {}\nspawn floor: median {:.2} ms; ratio = B/A (below 1 means B is faster)",
        report.a_exe, report.b_exe, report.spawn_floor_ms.median
    );
    println!(
        "{:<width$}  {:<16}  {:>5}  {:>11}  {:>11}  {:>8}  {:>8}  {:>8}  {:>6}  status",
        "workload", "command", "pairs", "A med ms", "B med ms", "B/A med", "min", "max", "load1"
    );
    for result in &report.workloads {
        println!(
            "{:<width$}  {:<16}  {:>5}  {:>11}  {:>11}  {:>8}  {:>8}  {:>8}  {:>6}  {}",
            result.id,
            result.command,
            result.pairs,
            format_option(result.a_ms.map(|stats| stats.median), 1),
            format_option(result.b_ms.map(|stats| stats.median), 1),
            format_option(result.ratio_median, 3),
            format_option(result.ratio_min, 3),
            format_option(result.ratio_max, 3),
            format_option(result.load_average_start.map(|load| load[0]), 1),
            result.detail.as_deref().unwrap_or(result.status.label()),
        );
    }
    println!(
        "geometric mean B/A over {} counted workloads: {} ({} disqualified, {} failed)",
        report.counted,
        format_option(report.geomean_ratio, 3),
        report.disqualified,
        report.failed
    );
    for result in report
        .workloads
        .iter()
        .filter(|result| result.status != Status::Ok)
    {
        let output = result
            .output
            .as_ref()
            .and_then(|comparison| comparison.detail.as_deref())
            .map_or_else(String::new, |detail| format!(" ({detail})"));
        println!(
            "excluded {}: {}{output}",
            result.id,
            result.detail.as_deref().unwrap_or(result.status.label())
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_geometric_mean_skips_non_positive_ratios() {
        assert_eq!(geometric_mean(&[]), None);
        let mean = geometric_mean(&[0.5, 2.0]).expect("two ratios");
        assert!((mean - 1.0).abs() < 1e-12, "{mean}");
        let mean = geometric_mean(&[0.0, 4.0]).expect("one positive ratio");
        assert!((mean - 4.0).abs() < 1e-12, "{mean}");
    }

    /// A stand-in simulator script named `name`: copies `payload` to the
    /// path after `-o`, or exits 1 when `payload` is `None`.
    #[cfg(unix)]
    fn fake_simulator(dir: &Path, name: &str, payload: Option<&[u8]>) -> PathBuf {
        use std::os::unix::fs::PermissionsExt as _;
        let script = dir.join(name);
        let body = match payload {
            None => "#!/bin/sh\nexit 1\n".to_string(),
            Some(payload) => {
                let source = dir.join(format!("{name}.raw.src"));
                std::fs::write(&source, payload).expect("payload");
                format!(
                    "#!/bin/sh\nwhile [ $# -gt 0 ]; do\n  if [ \"$1\" = -o ]; then cp '{}' \"$2\"; fi\n  shift\ndone\nexit 0\n",
                    source.display()
                )
            }
        };
        std::fs::write(&script, body).expect("script");
        let mut permissions = std::fs::metadata(&script).expect("metadata").permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&script, permissions).expect("chmod");
        script
    }

    /// A stand-in simulator batch file named `name`: copies `payload` to
    /// the path after `-o`, or exits 1 when `payload` is `None`.
    #[cfg(windows)]
    fn fake_simulator(dir: &Path, name: &str, payload: Option<&[u8]>) -> PathBuf {
        let script = dir.join(format!("{name}.cmd"));
        let body = match payload {
            None => "@exit /b 1\r\n".to_string(),
            Some(payload) => {
                let source = dir.join(format!("{name}.raw.src"));
                std::fs::write(&source, payload).expect("payload");
                format!(
                    "@echo off\r\n:next\r\nif \"%~1\"==\"\" exit /b 0\r\n\
                     if \"%~1\"==\"-o\" copy /b /y \"{}\" \"%~2\" >nul\r\nshift\r\ngoto next\r\n",
                    source.display()
                )
            }
        };
        std::fs::write(&script, body).expect("script");
        script
    }

    fn rawfile(value: f64) -> Vec<u8> {
        let mut bytes = b"Title: t\nDate: d\nPlotname: Operating Point\nFlags: real\n\
No. Variables: 1\nNo. Points: 1\nVariables:\n\t0\tV(A)\tvoltage\nBinary:\n"
            .to_vec();
        bytes.extend_from_slice(&value.to_le_bytes());
        bytes
    }

    fn args(a: PathBuf, b: PathBuf, tolerance: Option<f64>) -> PairedArgs {
        PairedArgs {
            a,
            b,
            pairs: 3,
            heavy_pairs: 1,
            warmups: 1,
            suite: Suite::Neospice,
            circuits: None,
            filter: None,
            tolerance,
            timeout_secs: 30,
            out: PathBuf::from("unused.json"),
            label: None,
        }
    }

    #[test]
    fn identical_outputs_count_and_different_ones_are_disqualified() {
        let dir = tempfile::tempdir().expect("scratch");
        let deck = dir.path().join("d.cir");
        std::fs::write(&deck, "* d\nV1 a 0 1\nR1 a 0 1\n.op\n.end\n").expect("deck");
        let workload = Workload::run_quiet(deck);
        let timeout = Duration::from_secs(30);

        let a = fake_simulator(dir.path(), "a", Some(&rawfile(1.0)));
        let same = fake_simulator(dir.path(), "same", Some(&rawfile(1.0)));
        let result = pair_workload(&workload, &args(a.clone(), same, None), timeout).unwrap();
        assert_eq!(result.status, Status::Ok, "{result:?}");
        assert_eq!(result.pairs, 3);
        assert!(result.ratio_median.is_some());
        assert!(result.ratio_min <= result.ratio_median);
        assert!(result.ratio_max >= result.ratio_median);

        let close = fake_simulator(dir.path(), "close", Some(&rawfile(1.0 + 1e-12)));
        let result =
            pair_workload(&workload, &args(a.clone(), close.clone(), None), timeout).unwrap();
        assert_eq!(result.status, Status::Disqualified, "{result:?}");
        assert_eq!(result.pairs, 0, "a disqualified workload is not timed");
        assert_eq!(result.ratio_median, None);

        let result = pair_workload(&workload, &args(a, close, Some(1e-9)), timeout).unwrap();
        assert_eq!(result.status, Status::Ok, "{result:?}");
    }

    #[test]
    fn every_rawfile_of_a_multi_analysis_run_is_compared_by_name() {
        let dir = tempfile::tempdir().expect("scratch");
        let (a, b) = (dir.path().join("a"), dir.path().join("b"));
        for side in [&a, &b] {
            std::fs::create_dir_all(side).unwrap();
            std::fs::write(side.join("out.op-001.raw"), rawfile(1.0)).unwrap();
        }
        std::fs::write(a.join("out.ac-001.raw"), rawfile(2.0)).unwrap();
        let missing = compare_dirs(&a, &b, None).unwrap();
        assert!(!missing.matches, "{missing:?}");
        std::fs::write(b.join("out.ac-001.raw"), rawfile(2.5)).unwrap();
        let different = compare_dirs(&a, &b, Some(1e-3)).unwrap();
        assert!(!different.matches, "{different:?}");
        assert!(different.detail.unwrap().starts_with("out.ac-001.raw"));
        std::fs::write(b.join("out.ac-001.raw"), rawfile(2.0)).unwrap();
        let same = compare_dirs(&a, &b, None).unwrap();
        assert!(same.matches, "{same:?}");
        assert_eq!(same.values, 2);
    }

    #[test]
    fn a_failing_side_fails_the_workload() {
        let dir = tempfile::tempdir().expect("scratch");
        let deck = dir.path().join("d.cir");
        std::fs::write(&deck, "* d\n.op\n.end\n").expect("deck");
        let workload = Workload::run_quiet(deck);
        let a = fake_simulator(dir.path(), "a", Some(&rawfile(1.0)));
        let failing = fake_simulator(dir.path(), "failing", None);
        let result =
            pair_workload(&workload, &args(a, failing, None), Duration::from_secs(30)).unwrap();
        assert_eq!(result.status, Status::Failed, "{result:?}");
        assert_eq!(
            result.detail.as_deref(),
            Some("B failed writing its rawfile")
        );
        assert_eq!(result.ratio_median, None);
    }
}
