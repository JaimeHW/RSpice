//! `rspice-mixed-corpus`: the mixed-signal reference corpus runner.
//!
//! ```text
//! rspice-mixed-corpus run [--design C1,C5] [--variant v] [--analysis a] [--record] [--extended]
//!                         [--producer "<cmd>"] [--worker <web-dir>]
//!                         [--expect-route interp|native|worker] [--jobs N] [--cases <dir>]
//!                         [--shard K/N]
//! rspice-mixed-corpus case <design-dir> <variant> <analysis> <out.json> [--budget-s N] [--o5]
//!                          [--worker <web-dir>]
//! rspice-mixed-corpus capture --oracle O1|O4|O2|O5-O1|O5-O4|O8 [--design C1] [--update] [--jobs N]
//! rspice-mixed-corpus probe a|b|c [--record] [--jobs N]
//! rspice-mixed-corpus peer toggle
//! ```
//!
//! The route is the build: `--features veriloga-native` is the native route.
//! `--worker` runs the cells in the browser worker instead, under Node: the
//! directory holds the application's `simulation-worker.js` beside the
//! worker image's wasm-bindgen `pkg/`, as `crates/rspice-app/web` does once
//! the browser build has generated it.
//! `--jobs` is the most cells (or captures) running at once; the default is
//! `runner::default_jobs()`. `--cases` keeps each cell's case result file in
//! the directory it names, to compare two routes' values afterwards; by
//! default they go to a temporary directory removed when the run ends.
//! `--shard K/N` runs the K-th of N disjoint sets of designs, balanced by
//! their declared budgets; the N shards together run every selected design
//! once, so one build can serve N machines.
//!
//! With `RSPICE_CONFORMANCE_PERTURB` set ([`perturb`]) a run classifies every
//! cell with RSpice's values moved by that relative error and reports each
//! cell that still passes as blind, in place of the ratchet; it refuses
//! `--record`, which would write the perturbed outcomes as the baseline.
//! `RSPICE_CONFORMANCE_PERTURB_TIME` asks the same of RSpice's time
//! coordinates, stretched by that relative error, over the cells whose
//! result has a time axis. A cell whose every passing quantity states why
//! the perturbation cannot move it (`insensitive`, `insensitive_time`) is
//! listed with its reasons instead of as blind, and a passing quantity in a
//! cell that another quantity failed is listed as masked.

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

use clap::{Parser, Subcommand};
use rspice_conformance::suites::mixed::{
    Outcome, Route,
    design::{Design, analysis_kind},
    dispatch, peer, runner,
};
use rspice_conformance::suites::perturb::{self, BlindCase};

#[derive(Parser)]
#[command(
    name = "rspice-mixed-corpus",
    about = "Run the mixed-signal reference corpus"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run cells, compare, classify, and ratchet (or record) outcomes.
    Run {
        #[arg(long, value_delimiter = ',')]
        design: Vec<String>,
        #[arg(long)]
        variant: Option<String>,
        #[arg(long)]
        analysis: Option<String>,
        #[arg(long)]
        record: bool,
        #[arg(long)]
        extended: bool,
        #[arg(long)]
        producer: Option<String>,
        #[arg(long)]
        worker: Option<PathBuf>,
        #[arg(long)]
        expect_route: Option<String>,
        #[arg(long, value_parser = job_count)]
        jobs: Option<usize>,
        #[arg(long)]
        cases: Option<PathBuf>,
        #[arg(long, value_parser = shard)]
        shard: Option<(usize, usize)>,
    },
    /// Run one case in this process and write case-result.json.
    Case {
        design_dir: PathBuf,
        variant: String,
        analysis: String,
        out: PathBuf,
        #[arg(long, default_value_t = 900)]
        budget_s: u64,
        #[arg(long)]
        o5: bool,
        #[arg(long)]
        worker: Option<PathBuf>,
    },
    /// Capture (or verify) checked-in oracle references.
    Capture {
        #[arg(long)]
        oracle: String,
        #[arg(long, value_delimiter = ',')]
        design: Vec<String>,
        #[arg(long)]
        update: bool,
        #[arg(long, value_parser = job_count)]
        jobs: Option<usize>,
    },
    /// Run one probe.
    Probe {
        which: String,
        #[arg(long)]
        record: bool,
        #[arg(long, value_parser = job_count)]
        jobs: Option<usize>,
    },
    /// Serve a corpus deck's `d_process` peer on stdin and stdout.
    Peer { name: String },
}

/// `--jobs`: a positive count.
fn job_count(text: &str) -> Result<usize, String> {
    match text.parse::<usize>() {
        Ok(0) | Err(_) => Err(format!("{text} is not a positive job count")),
        Ok(count) => Ok(count),
    }
}

/// `--shard K/N`: the K-th of N shards, 1 <= K <= N.
fn shard(text: &str) -> Result<(usize, usize), String> {
    let invalid = || format!("{text} is not a shard K/N with 1 <= K <= N");
    let (index, count) = text.split_once('/').ok_or_else(invalid)?;
    let index: usize = index.parse().map_err(|_| invalid())?;
    let count: usize = count.parse().map_err(|_| invalid())?;
    if index == 0 || index > count {
        return Err(invalid());
    }
    Ok((index, count))
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("rspice-mixed-corpus: {error}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<ExitCode, String> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Peer { name } => {
            peer::serve(&name)?;
            Ok(ExitCode::SUCCESS)
        }
        Commands::Case {
            design_dir,
            variant,
            analysis,
            out,
            budget_s,
            o5,
            worker,
        } => {
            let design = Design::load(&design_dir)?;
            let result = dispatch::run_case(&dispatch::CaseRequest {
                design: &design,
                variant: &variant,
                analysis: &analysis,
                o5,
                budget: Duration::from_secs(budget_s),
                worker: worker.as_deref(),
            });
            result.write(&out)?;
            Ok(ExitCode::SUCCESS)
        }
        Commands::Run {
            design,
            variant,
            analysis,
            record,
            extended,
            producer,
            worker,
            expect_route,
            jobs,
            cases,
            shard,
        } => {
            let route = if worker.is_some() {
                Route::Worker
            } else {
                Route::compiled()
            };
            check_route(expect_route.as_deref(), route)?;
            let defaults = options(route)?;
            let keep = cases.is_some();
            run_corpus(
                runner::RunOptions {
                    designs: design,
                    variant,
                    analysis,
                    record,
                    extended,
                    producer,
                    worker,
                    jobs: jobs.unwrap_or_default(),
                    shard,
                    work: cases.unwrap_or_else(|| defaults.work.clone()),
                    ..defaults
                },
                keep,
            )
        }
        Commands::Probe {
            which,
            record,
            jobs,
        } => {
            let id = match which.as_str() {
                "a" => "P06a",
                "b" => "P06b",
                "c" => "P06c",
                "d" => "P06d",
                other => return Err(format!("unknown probe {other}; use a, b, c or d")),
            };
            run_corpus(
                runner::RunOptions {
                    designs: vec![id.to_owned()],
                    record,
                    extended: true,
                    jobs: jobs.unwrap_or_default(),
                    ..options(Route::compiled())?
                },
                false,
            )
        }
        Commands::Capture {
            oracle,
            design,
            update,
            jobs,
        } => {
            let jobs = jobs.unwrap_or_else(runner::default_jobs);
            let reports = runner::capture(&design, &oracle, update, jobs)?;
            let mut failed = false;
            for report in &reports {
                println!("{}: {}", report.reference, report.status);
                failed |=
                    report.status.starts_with("FAILED") || report.status.starts_with("DIFFERS");
            }
            println!("capture {oracle}: {} references", reports.len());
            Ok(if failed {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            })
        }
    }
}

fn check_route(expected: Option<&str>, route: Route) -> Result<(), String> {
    let Some(expected) = expected else {
        return Ok(());
    };
    let expected =
        Route::from_token(expected).ok_or_else(|| format!("unknown route {expected}"))?;
    if expected == Route::Worker && route != Route::Worker {
        return Err(
            "--expect-route worker but no --worker directory names the worker to run".to_owned(),
        );
    }
    if route == Route::Worker && expected != Route::Worker {
        return Err(format!(
            "--expect-route {expected} but --worker makes this run the worker route"
        ));
    }
    if expected != route {
        return Err(format!(
            "--expect-route {expected} but this run is the {route} route; build with{} --features veriloga-native",
            if expected == Route::Native { "" } else { "out" }
        ));
    }
    Ok(())
}

fn options(route: Route) -> Result<runner::RunOptions, String> {
    let bin = std::env::current_exe().map_err(|error| error.to_string())?;
    let work = std::env::temp_dir().join(format!("rspice-mixed-corpus-{}", std::process::id()));
    // Per route: the two builds' cells take different times.
    let timings =
        std::env::temp_dir().join(format!("rspice-mixed-corpus-timings-{}.tsv", route.token()));
    Ok(runner::RunOptions {
        bin,
        work,
        timings: Some(timings),
        ..Default::default()
    })
}

/// Run `options`, then remove the case files unless the caller `keep`s them.
fn run_corpus(options: runner::RunOptions, keep: bool) -> Result<ExitCode, String> {
    let stretch = perturb::active_time();
    let perturbation = match (perturb::active(), stretch) {
        (Some(_), Some(_)) => {
            return Err(format!(
                "{} and {} are both set: one run asks one question",
                perturb::PERTURB_ENV,
                perturb::PERTURB_TIME_ENV
            ));
        }
        (Some(scale), None) => Some((scale, perturb::PERTURB_ENV, "values moved")),
        (None, Some(stretch)) => Some((stretch, perturb::PERTURB_TIME_ENV, "times stretched")),
        (None, None) => None,
    };
    if let Some((_, variable, _)) = perturbation
        && options.record
    {
        return Err(format!(
            "{variable} is set: refusing --record, which would write perturbed outcomes as the baseline"
        ));
    }
    let started = std::time::Instant::now();
    let reports = runner::run(&options)?;
    if !keep {
        let _ = std::fs::remove_dir_all(&options.work);
    }
    // On stderr with the per-cell lines; stdout is the same for any --jobs.
    eprintln!(
        "mixed-corpus: {} cells, --jobs {}, {:.1} s",
        reports
            .iter()
            .map(|report| report.cells.len())
            .sum::<usize>(),
        options.jobs(),
        started.elapsed().as_secs_f64()
    );
    let mut failed = false;
    for report in &reports {
        let rows: Vec<_> = report.cells.iter().map(|cell| cell.row.clone()).collect();
        println!("{}: {}", report.design, runner::ledger_counts(&rows));
        for cell in &report.cells {
            println!(
                "  {}\t{}\t{}\t{}\t{}\t[{}]",
                cell.row.variant,
                cell.row.analysis,
                cell.row.route,
                cell.row.outcome,
                cell.row.detail,
                cell.dispatch
            );
        }
        for finding in &report.findings {
            println!("  integrity: {finding}");
            failed = true;
        }
        // Under a perturbation every passing cell is meant to drift; the
        // blind cells below are the finding instead.
        if perturbation.is_none() {
            for drift in &report.drift {
                println!("  {drift}");
                failed = true;
            }
        }
    }
    if let Some((perturbation, _, what)) = perturbation {
        let mut checked = 0usize;
        let mut blind = Vec::new();
        let mut stated = Vec::new();
        let mut masked = Vec::new();
        for report in &reports {
            for cell in &report.cells {
                // A stretch moves nothing in an operating point or a
                // frequency-domain result.
                if stretch.is_some() && !has_time_axis(&cell.row.analysis) {
                    continue;
                }
                let name = format!(
                    "{} {} {} {}",
                    report.design, cell.row.variant, cell.row.analysis, cell.row.route
                );
                let passing: Vec<&runner::QuantityReport> = cell
                    .quantities
                    .iter()
                    .filter(|quantity| quantity.passed == Some(true))
                    .collect();
                match cell.row.outcome {
                    Outcome::Pass => {
                        checked += 1;
                        if !passing.is_empty()
                            && passing
                                .iter()
                                .all(|quantity| quantity.insensitive.is_some())
                        {
                            let reasons: Vec<String> = passing
                                .iter()
                                .map(|quantity| {
                                    format!(
                                        "{}: {}",
                                        quantity.name,
                                        quantity.insensitive.as_deref().unwrap_or_default()
                                    )
                                })
                                .collect();
                            stated.push(format!("  STATED {name}  [{}]", reasons.join("; ")));
                        } else {
                            blind.push(BlindCase::new(name, &cell.row.detail));
                        }
                    }
                    Outcome::Fail | Outcome::Wrong => {
                        checked += 1;
                        for quantity in passing
                            .iter()
                            .filter(|quantity| quantity.insensitive.is_none())
                        {
                            masked.push(format!(
                                "  MASKED {name}: {}  [{}]",
                                quantity.name, quantity.detail
                            ));
                        }
                    }
                    Outcome::Unverified | Outcome::Refused => {}
                }
            }
        }
        stated.sort();
        masked.sort();
        if !stated.is_empty() {
            println!(
                "{} passing cell(s) state why the perturbation cannot move them:",
                stated.len()
            );
            println!("{}", stated.join("\n"));
        }
        if !masked.is_empty() {
            println!(
                "{} quantity(ies) still pass inside a cell another quantity failed:",
                masked.len()
            );
            println!("{}", masked.join("\n"));
        }
        match perturb::blind_verdict_for("mixed corpus", what, perturbation, checked, &blind) {
            Ok(summary) => println!("{summary}"),
            Err(report) => {
                println!("{report}");
                failed = true;
            }
        }
    }
    Ok(if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

/// Whether an analysis's result has a time axis a stretch moves.
fn has_time_axis(analysis: &str) -> bool {
    matches!(
        analysis_kind(analysis).unwrap_or(analysis),
        "tran" | "trnoise" | "mc-tran" | "step-tran" | "ckpt" | "pss"
    )
}
