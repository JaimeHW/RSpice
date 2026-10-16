//! `rspice-mixed-corpus`: the mixed-signal reference corpus runner.
//!
//! ```text
//! rspice-mixed-corpus run [--design C1,C5] [--variant v] [--analysis a] [--record] [--extended]
//!                         [--producer "<cmd>"] [--expect-route interp|native] [--jobs N]
//! rspice-mixed-corpus case <design-dir> <variant> <analysis> <out.json> [--budget-s N] [--o5]
//! rspice-mixed-corpus capture --oracle O1|O4|O2|O5-O1|O5-O4 [--design C1] [--update] [--jobs N]
//! rspice-mixed-corpus probe a|b|c [--record] [--jobs N]
//! ```
//!
//! The route is the build: `--features veriloga-native` is the native route.
//! `--jobs` is the most cells (or captures) running at once; the default is
//! `runner::default_jobs()`.

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

use clap::{Parser, Subcommand};
use rspice_conformance::suites::mixed::{Route, design::Design, dispatch, runner};

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
        expect_route: Option<String>,
        #[arg(long, value_parser = job_count)]
        jobs: Option<usize>,
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
    /// Run one P0.6 probe.
    Probe {
        which: String,
        #[arg(long)]
        record: bool,
        #[arg(long, value_parser = job_count)]
        jobs: Option<usize>,
    },
}

/// `--jobs`: a positive count.
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
            eprintln!("rspice-mixed-corpus: {error}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<ExitCode, String> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Case {
            design_dir,
            variant,
            analysis,
            out,
            budget_s,
            o5,
        } => {
            let design = Design::load(&design_dir)?;
            let result = dispatch::run_case(&dispatch::CaseRequest {
                design: &design,
                variant: &variant,
                analysis: &analysis,
                o5,
                budget: Duration::from_secs(budget_s),
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
            expect_route,
            jobs,
        } => {
            check_route(expect_route.as_deref())?;
            run_corpus(runner::RunOptions {
                designs: design,
                variant,
                analysis,
                record,
                extended,
                producer,
                jobs: jobs.unwrap_or_default(),
                ..options()?
            })
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
            run_corpus(runner::RunOptions {
                designs: vec![id.to_owned()],
                record,
                extended: true,
                jobs: jobs.unwrap_or_default(),
                ..options()?
            })
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

fn check_route(expected: Option<&str>) -> Result<(), String> {
    let Some(expected) = expected else {
        return Ok(());
    };
    let expected =
        Route::from_token(expected).ok_or_else(|| format!("unknown route {expected}"))?;
    if expected != Route::compiled() {
        return Err(format!(
            "--expect-route {expected} but this binary is the {} route; build with{} --features veriloga-native",
            Route::compiled(),
            if expected == Route::Native { "" } else { "out" }
        ));
    }
    Ok(())
}

fn options() -> Result<runner::RunOptions, String> {
    let bin = std::env::current_exe().map_err(|error| error.to_string())?;
    let work = std::env::temp_dir().join(format!("rspice-mixed-corpus-{}", std::process::id()));
    // Per route: the two builds' cells take different times.
    let timings = std::env::temp_dir().join(format!(
        "rspice-mixed-corpus-timings-{}.tsv",
        Route::compiled().token()
    ));
    Ok(runner::RunOptions {
        bin,
        work,
        timings: Some(timings),
        ..Default::default()
    })
}

fn run_corpus(options: runner::RunOptions) -> Result<ExitCode, String> {
    let started = std::time::Instant::now();
    let reports = runner::run(&options)?;
    let _ = std::fs::remove_dir_all(&options.work);
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
        for drift in &report.drift {
            println!("  {drift}");
            failed = true;
        }
    }
    Ok(if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}
