//! In-process native Verilog-A JIT benchmark gate.
//!
//! This subcommand complements the process-level `run` benchmark. It measures
//! the generated native entrypoint path directly against the bytecode VM on a
//! dense synthetic model and can fail on median speedup or native p95 budgets.

use crate::error::BenchError;
use clap::Args;
use rspice_veriloga::native::bench::{NativeBenchConfig, run_native_x64_benchmarks};
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

/// Arguments for the `native-jit` subcommand.
#[derive(Args, Debug)]
pub struct NativeJitArgs {
    /// Entry-point sweeps per timed sample.
    #[arg(long, default_value_t = 200_000)]
    pub iterations: usize,

    /// Timed samples; report gates on median and p95 rather than the fastest run.
    #[arg(long, default_value_t = 7)]
    pub samples: usize,

    /// Required bytecode/native median speedup. Set lower for exploratory local runs.
    #[arg(long, default_value_t = 1.10)]
    pub min_speedup: f64,

    /// Optional absolute native p95 budget in ns per benchmark sweep.
    #[arg(long, value_name = "NS")]
    pub max_native_p95_ns_per_sweep: Option<f64>,

    /// Optional JSON report path.
    #[arg(long, value_name = "PATH")]
    pub out: Option<PathBuf>,
}

pub fn run(args: &NativeJitArgs) -> Result<ExitCode, BenchError> {
    let config = NativeBenchConfig {
        iterations: args.iterations,
        samples: args.samples,
        min_speedup: args.min_speedup,
        max_native_p95_ns_per_sweep: args.max_native_p95_ns_per_sweep,
    };
    let report =
        run_native_x64_benchmarks(config).map_err(|message| BenchError::NativeJit { message })?;

    println!(
        "native-jit target={} iterations={} samples={} min-speedup={:.3}",
        report.target, report.iterations, report.samples, report.min_speedup
    );
    for case in &report.cases {
        println!(
            "  {name:<24} native median {native_median:>10.3} ns p95 {native_p95:>10.3} ns  bytecode median {bytecode_median:>10.3} ns  speedup {speedup:>7.3}x  [{status}]",
            name = case.name,
            native_median = case.native_ns_per_sweep.median,
            native_p95 = case.native_ns_per_sweep.p95,
            bytecode_median = case.bytecode_ns_per_sweep.median,
            speedup = case.speedup_median,
            status = if case.passed { "ok" } else { "failed" },
        );
        if let Some(failure) = &case.failure {
            println!("    {failure}");
        }
    }

    if let Some(path) = &args.out {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent).map_err(|source| {
                BenchError::io(
                    format!("create native JIT report dir `{}`", parent.display()),
                    source,
                )
            })?;
        }
        let json = serde_json::to_string_pretty(&report).map_err(|source| BenchError::Json {
            context: "serialize native JIT benchmark report".into(),
            source,
        })?;
        fs::write(path, json).map_err(|source| {
            BenchError::io(
                format!("write native JIT report `{}`", path.display()),
                source,
            )
        })?;
    }

    Ok(if report.passed {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}
