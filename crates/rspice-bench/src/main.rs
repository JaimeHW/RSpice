//! `rspice-bench` — the WS0/M0.5 macro-benchmark rig.
//!
//! This binary times *whole simulator processes* (RSpice vs. a locally
//! installed ngspice) over the shared benchmark deck set in
//! `benchmarks/circuits/` and emits a JSON scoreboard plus a human-readable
//! table. Its headline `run` measurement is deliberately a macro-benchmark: it
//! times end-to-end wall-clock of `rspice run <deck> -q` and `ngspice -b
//! <deck>`, including parsing and output. The in-process subcommands alongside
//! it isolate one layer each, so an optimization there is attributable.
//!
//! Subcommands:
//!
//! * `gen` — deterministically regenerate the generated benchmark decks
//!   (the RC ladders). The emitted text is byte-stable so the checked-in
//!   decks can be reproduced exactly.
//! * `generated-rust` — authenticate and gate generated Verilog-A Rust
//!   source resources.
//! * `klu` — time the KLU solver kernels (analyze/factor/refactor/solve) in
//!   isolation on circuit-shaped matrices, with optional per-nonzero budgets.
//! * `native-jit` — in-process Verilog-A native JIT gate.
//! * `run` — execute the benchmark suite and write the JSON scoreboard.
//!
//! Two further subcommands, `generated-stamp` and `veriloga-golden`, are
//! behind the `generated-stamp` feature because they pull in the corpus.
//!
//! See `benchmarks/README.md` for methodology and operating conventions.

mod error;
mod generate;
mod generated_rust;
#[cfg(feature = "generated-stamp")]
mod generated_stamp;
mod klu;
mod native_jit;
mod runner;
#[cfg(feature = "generated-stamp")]
mod veriloga_golden;

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
    /// Authenticate and gate generated Verilog-A Rust source resources.
    GeneratedRust(generated_rust::GeneratedRustArgs),
    /// Measure and gate generated Verilog-A built-in stamp throughput.
    #[cfg(feature = "generated-stamp")]
    GeneratedStamp(generated_stamp::GeneratedStampArgs),
    /// Measure the KLU solver kernels in isolation, with optional budgets.
    Klu(klu::KluArgs),
    /// Run the in-process native Verilog-A JIT benchmark gate.
    NativeJit(native_jit::NativeJitArgs),
    /// Run the benchmark suite and emit a JSON scoreboard.
    Run(runner::RunArgs),
    /// Capture, verify, and independently audit generated Verilog-A numerics.
    #[cfg(feature = "generated-stamp")]
    VerilogAGolden(veriloga_golden::VerilogAGoldenArgs),
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let outcome = match cli.command {
        BenchCommand::Gen(args) => generate::generate(&args).map(|()| ExitCode::SUCCESS),
        BenchCommand::GeneratedRust(args) => generated_rust::run(&args),
        #[cfg(feature = "generated-stamp")]
        BenchCommand::GeneratedStamp(args) => generated_stamp::run(&args),
        BenchCommand::Klu(args) => klu::run(&args),
        BenchCommand::NativeJit(args) => native_jit::run(&args),
        BenchCommand::Run(args) => runner::run(&args),
        #[cfg(feature = "generated-stamp")]
        BenchCommand::VerilogAGolden(args) => veriloga_golden::run(&args),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regression_policy_flags_require_a_baseline() {
        let error = Cli::try_parse_from(["rspice-bench", "run", "--max-regression-percent", "5"])
            .expect_err("a threshold without a baseline is meaningless");
        assert_eq!(
            error.kind(),
            clap::error::ErrorKind::MissingRequiredArgument
        );
    }

    #[test]
    fn klu_budgets_are_off_unless_asked_for() {
        let cli = Cli::try_parse_from(["rspice-bench", "klu"]).expect("klu needs no arguments");
        let BenchCommand::Klu(args) = cli.command else {
            panic!("klu command expected");
        };
        // Budgets cannot be chosen before baselines exist, so an unqualified
        // run measures and reports without ever failing.
        assert_eq!(args.max_refactor_ns_per_lu_nnz, None);
        assert_eq!(args.max_solve_ns_per_lu_nnz, None);
        assert_eq!(args.max_fill_ratio, None);
        assert_eq!(args.sizes, vec![100, 1_000, 10_000]);
    }

    #[test]
    fn klu_sizes_accept_a_comma_separated_sweep() {
        let cli = Cli::try_parse_from(["rspice-bench", "klu", "--sizes", "64,256"])
            .expect("comma-separated sizes parse");
        let BenchCommand::Klu(args) = cli.command else {
            panic!("klu command expected");
        };
        assert_eq!(args.sizes, vec![64, 256]);
    }

    #[test]
    fn baseline_uses_the_runtime_default_threshold() {
        let cli = Cli::try_parse_from(["rspice-bench", "run", "--baseline", "baseline.json"])
            .expect("baseline arguments parse");
        let BenchCommand::Run(args) = cli.command else {
            panic!("run command expected");
        };
        assert_eq!(args.max_regression_percent, None);
        assert!(!args.allow_host_mismatch);
    }
}
