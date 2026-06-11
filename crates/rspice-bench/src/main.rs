//! `rspice-bench` — the WS0/M0.5 macro-benchmark rig.
//!
//! This binary times *whole simulator processes* (RSpice vs. a locally
//! installed ngspice) over the shared benchmark deck set in
//! `benchmarks/circuits/` and emits a JSON scoreboard plus a human-readable
//! table. It is deliberately a macro-benchmark runner: it measures end-to-end
//! wall-clock time of `rspice run <deck> -q` and `ngspice -b <deck>`,
//! including parsing and output, not isolated solver phases.
//!
//! Subcommands:
//!
//! * `gen` — deterministically regenerate the generated benchmark decks
//!   (the RC ladders). The emitted text is byte-stable so the checked-in
//!   decks can be reproduced exactly.
//! * `run` — execute the benchmark suite and write the JSON scoreboard.
//!
//! See `benchmarks/README.md` for methodology and operating conventions.

mod error;
mod generate;
mod runner;

use clap::{Parser, Subcommand};
use std::process::ExitCode;

/// Command-line interface of the benchmark rig.
#[derive(Parser, Debug)]
#[command(name = "rspice-bench", version, about = "RSpice macro-benchmark rig")]
struct Cli {
    /// Selected subcommand.
    #[command(subcommand)]
    command: BenchCommand,
}

/// Top-level subcommands.
#[derive(Subcommand, Debug)]
enum BenchCommand {
    /// Regenerate the deterministic, generated benchmark decks (RC ladders).
    Gen(generate::GenArgs),
    /// Run the benchmark suite and emit a JSON scoreboard.
    Run(runner::RunArgs),
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let outcome = match cli.command {
        BenchCommand::Gen(args) => generate::generate(&args).map(|()| ExitCode::SUCCESS),
        BenchCommand::Run(args) => runner::run(&args),
    };
    match outcome {
        Ok(code) => code,
        Err(err) => {
            eprintln!("rspice-bench: error: {err}");
            let mut source = std::error::Error::source(&err);
            while let Some(cause) = source {
                eprintln!("  caused by: {cause}");
                source = cause.source();
            }
            ExitCode::FAILURE
        }
    }
}
