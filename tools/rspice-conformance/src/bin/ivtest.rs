//! `rspice-ivtest`: the Icarus Verilog ivtest suite run through RSpice.
//!
//! ```text
//! rspice-ivtest run [--jobs N] [--budget-s S] [--only a,b] [--record] [--manifest PATH]
//! rspice-ivtest oracle [--jobs N] [--budget-s S] [--only a,b]
//! rspice-ivtest select
//! rspice-ivtest worker
//! ```
//!
//! The checkout is `RSPICE_IVTEST_DIR`; without it the run reports the oracle
//! absent and succeeds, unless `RSPICE_VERILOG_ORACLES_REQUIRED=1`. `run`
//! ratchets every outcome against `tests/verilog/ivtest-manifest.tsv` and
//! exits 1 on any drift; `--record` rewrites the manifest instead. `oracle`
//! runs the selection through Icarus (`iverilog`/`vvp`, or the
//! `RSPICE_IVERILOG_EXE`/`RSPICE_VVP_EXE` overrides) and exits 1 unless every
//! selected test passes there. `select` prints the selection without running
//! it. `worker` is the child `run` starts: it reads one case spec (JSON) per
//! line and answers each with one result line, which also makes it the way to
//! run a single case by hand.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::{Duration, Instant};

use clap::{Parser, Subcommand};
use rspice_conformance::suites::verilog::ivtest::{self, judge::Outcome, manifest, runner, select};
use rspice_conformance::suites::verilog::oracle::oracles_required;

#[derive(Parser)]
#[command(
    name = "rspice-ivtest",
    about = "Run the ivtest selection through RSpice"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the selection and ratchet (or record) its outcomes.
    Run {
        /// The most runs at once; the default is half the hardware threads.
        #[arg(long, value_parser = job_count)]
        jobs: Option<usize>,
        /// One run's wall-clock budget, in seconds.
        #[arg(long, default_value_t = runner::DEFAULT_BUDGET.as_secs())]
        budget_s: u64,
        /// Run only these tests (and ratchet only them).
        #[arg(long, value_delimiter = ',')]
        only: Vec<String>,
        /// Rewrite the manifest from this run instead of ratcheting.
        #[arg(long)]
        record: bool,
        /// The manifest; the default is `tests/verilog/ivtest-manifest.tsv`.
        #[arg(long)]
        manifest: Option<PathBuf>,
    },
    /// Run the selection through Icarus Verilog, which must pass all of it.
    Oracle {
        #[arg(long, value_parser = job_count)]
        jobs: Option<usize>,
        #[arg(long, default_value_t = runner::DEFAULT_BUDGET.as_secs())]
        budget_s: u64,
        #[arg(long, value_delimiter = ',')]
        only: Vec<String>,
    },
    /// Print every candidate with its class, bucket and exclusion.
    Select,
    /// Run case specs from standard input, one per line (the child `run`
    /// starts).
    Worker,
}

fn job_count(text: &str) -> Result<usize, String> {
    match text.parse::<usize>() {
        Ok(0) | Err(_) => Err(format!("{text} is not a positive job count")),
        Ok(count) => Ok(count),
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("rspice-ivtest: {error}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<ExitCode, String> {
    match Cli::parse().command {
        Commands::Worker => {
            runner::serve(std::io::stdin().lock(), std::io::stdout().lock())?;
            Ok(ExitCode::SUCCESS)
        }
        Commands::Select => {
            let Some(root) = checkout()? else {
                return Ok(absent_exit());
            };
            for candidate in select::candidates(&root)? {
                println!(
                    "{}\t{}\t{}\t{}\t{}",
                    candidate.entry.name,
                    candidate.entry.list.token(),
                    candidate.class.map_or("-", select::Class::token),
                    if candidate.selected() {
                        candidate.lane.token()
                    } else {
                        "-"
                    },
                    candidate.exclusion.as_deref().unwrap_or("")
                );
            }
            Ok(ExitCode::SUCCESS)
        }
        Commands::Run {
            jobs,
            budget_s,
            only,
            record,
            manifest,
        } => {
            let Some(root) = checkout()? else {
                return Ok(absent_exit());
            };
            run_selection(root, jobs, budget_s, &only, record, manifest)
        }
        Commands::Oracle {
            jobs,
            budget_s,
            only,
        } => {
            let Some(root) = checkout()? else {
                return Ok(absent_exit());
            };
            run_oracle(root, jobs, budget_s, &only)
        }
    }
}

/// The selection, or the named part of it.
fn chosen(root: &std::path::Path, only: &[String]) -> Result<Vec<select::Candidate>, String> {
    let mut candidates = select::candidates(root)?;
    if !only.is_empty() {
        candidates.retain(|candidate| only.contains(&candidate.entry.name));
        let missing: Vec<&String> = only
            .iter()
            .filter(|name| {
                !candidates
                    .iter()
                    .any(|candidate| &candidate.entry.name == *name)
            })
            .collect();
        if !missing.is_empty() {
            return Err(format!("no candidate test named {missing:?}"));
        }
    }
    Ok(candidates)
}

fn run_oracle(
    root: PathBuf,
    jobs: Option<usize>,
    budget_s: u64,
    only: &[String],
) -> Result<ExitCode, String> {
    use rspice_conformance::suites::verilog::VerilogEngine;
    use rspice_conformance::suites::verilog::oracle::{self, OracleAvailability};

    let tools = match oracle::detect(VerilogEngine::Icarus) {
        OracleAvailability::Available(tools) => tools,
        other => {
            eprintln!("{}", other.diagnostic());
            return Ok(absent_exit());
        }
    };
    let started = Instant::now();
    let candidates = chosen(&root, only)?;
    let jobs = jobs.unwrap_or_else(runner::default_jobs);
    let reports = ivtest::icarus::run(
        &root,
        &tools,
        &candidates,
        jobs,
        Duration::from_secs(budget_s),
        |report| {
            if report.outcome != Outcome::Pass {
                eprintln!(
                    "icarus: {} {} - {}",
                    report.name, report.outcome, report.detail
                );
            }
        },
    )?;
    let failed: Vec<&str> = reports
        .iter()
        .filter(|report| report.outcome != Outcome::Pass)
        .map(|report| report.name.as_str())
        .collect();
    let residual: Vec<&str> = reports
        .iter()
        .filter(|report| report.residual)
        .map(|report| report.name.as_str())
        .collect();
    println!(
        "icarus ({}): {} selected, {} pass, {} not passing, --jobs {jobs}, {:.1} s wall",
        tools.identity(),
        reports.len(),
        reports.len() - failed.len(),
        failed.len(),
        started.elapsed().as_secs_f64()
    );
    println!(
        "  gold tests whose Icarus log keeps text past the normalization: {} ({})",
        residual.len(),
        residual.join(", ")
    );
    Ok(if failed.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

/// The checkout, or `None` (with the reason printed) when it is absent.
fn checkout() -> Result<Option<PathBuf>, String> {
    match ivtest::ivtest_dir() {
        Ok(root) => Ok(Some(root)),
        Err(absent) => {
            eprintln!("{absent}");
            Ok(None)
        }
    }
}

fn absent_exit() -> ExitCode {
    if oracles_required() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn run_selection(
    root: PathBuf,
    jobs: Option<usize>,
    budget_s: u64,
    only: &[String],
    record: bool,
    manifest_path: Option<PathBuf>,
) -> Result<ExitCode, String> {
    let started = Instant::now();
    let candidates = chosen(&root, only)?;
    let complete = only.is_empty();
    let options = runner::RunOptions {
        root,
        bin: std::env::current_exe().map_err(|error| error.to_string())?,
        jobs: jobs.unwrap_or_default(),
        budget: Duration::from_secs(budget_s),
        timings: Some(std::env::temp_dir().join("rspice-ivtest-timings.tsv")),
    };
    let reports = runner::run(&options, &candidates, |report| {
        if !matches!(report.row.outcome, Outcome::Excluded(_)) {
            eprintln!(
                "ivtest: {} {} {} {} - {} ({:.2} s)",
                report.row.name,
                report.row.class.map_or("-", select::Class::token),
                report.row.lane.map_or("-", select::Lane::token),
                report.row.outcome,
                report.row.detail,
                report.elapsed_s
            );
        }
    });
    let reports = reports?;
    let wall = started.elapsed();

    summarize(&reports, &options, wall);
    let gaps: Vec<&str> = candidates
        .iter()
        .filter(|candidate| candidate.selected() && candidate.gap.is_some())
        .map(|candidate| candidate.entry.name.as_str())
        .collect();
    println!(
        "  testbench gaps (run without an option Icarus's run sets up): {} ({})",
        gaps.len(),
        gaps.join(", ")
    );

    let path = manifest_path.unwrap_or_else(ivtest::manifest_path);
    let recorded = match std::fs::read_to_string(&path) {
        Ok(text) => manifest::parse(&text)?,
        Err(_) if record => Vec::new(),
        Err(error) => return Err(format!("cannot read {}: {error}", path.display())),
    };
    let observed: Vec<manifest::Row> = reports.iter().map(|report| report.row.clone()).collect();
    if record {
        let rows = manifest::record(&recorded, &observed, complete);
        std::fs::write(&path, manifest::render(&rows))
            .map_err(|error| format!("cannot write {}: {error}", path.display()))?;
        println!("recorded {} rows into {}", rows.len(), path.display());
        return Ok(ExitCode::SUCCESS);
    }
    let drift = manifest::ratchet(&recorded, &observed, complete);
    for line in &drift {
        println!("  {line}");
    }
    if drift.is_empty() {
        println!("no drift from {}", path.display());
        Ok(ExitCode::SUCCESS)
    } else {
        println!("{} tests drifted from {}", drift.len(), path.display());
        Ok(ExitCode::FAILURE)
    }
}

/// Counts by class and by bucket, the wall time, and the slowest tests.
fn summarize(reports: &[runner::TestReport], options: &runner::RunOptions, wall: Duration) {
    const KINDS: [&str; 5] = ["pass", "fail", "refused", "racy", "excluded"];
    let mut by_class: BTreeMap<&str, BTreeMap<&str, usize>> = BTreeMap::new();
    let mut by_lane: BTreeMap<select::Lane, BTreeMap<&str, usize>> = BTreeMap::new();
    let mut refusals: BTreeMap<String, usize> = BTreeMap::new();
    for report in reports {
        let row = &report.row;
        let kind = row.outcome.kind();
        let class = if kind == "excluded" {
            "excluded"
        } else {
            row.class.map_or("-", select::Class::token)
        };
        *by_class.entry(class).or_default().entry(kind).or_default() += 1;
        if let Some(lane) = row.lane {
            *by_lane.entry(lane).or_default().entry(kind).or_default() += 1;
        }
        if let Outcome::Refused(token) = &row.outcome {
            *refusals.entry(token.clone()).or_default() += 1;
        }
    }
    let line = |counts: &BTreeMap<&str, usize>| {
        KINDS
            .iter()
            .filter_map(|kind| counts.get(kind).map(|count| format!("{kind} {count}")))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let selected = reports
        .iter()
        .filter(|report| !matches!(report.row.outcome, Outcome::Excluded(_)))
        .count();
    println!(
        "ivtest: {} candidates, {selected} selected, --jobs {}, budget {} s, {:.1} s wall",
        reports.len(),
        options.jobs(),
        options.budget.as_secs(),
        wall.as_secs_f64()
    );
    for (class, counts) in &by_class {
        let label = if *class == "ce" {
            format!("{class} (pass = CE match)")
        } else {
            (*class).to_owned()
        };
        println!("  class {label}: {}", line(counts));
    }
    for (lane, counts) in &by_lane {
        println!("  bucket {lane}: {}", line(counts));
    }
    let mut refusals: Vec<(String, usize)> = refusals.into_iter().collect();
    refusals.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    println!(
        "  refusals: {}",
        refusals
            .iter()
            .map(|(token, count)| format!("{token} {count}"))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let timeouts = reports.iter().filter(|report| report.timed_out).count();
    let mut slowest: Vec<&runner::TestReport> = reports.iter().collect();
    slowest.sort_by(|a, b| b.elapsed_s.total_cmp(&a.elapsed_s));
    println!(
        "  timeouts: {timeouts}; slowest: {}",
        slowest
            .iter()
            .take(8)
            .map(|report| format!("{} {:.2} s", report.row.name, report.elapsed_s))
            .collect::<Vec<_>>()
            .join(", ")
    );
}
