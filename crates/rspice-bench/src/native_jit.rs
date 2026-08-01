//! In-process native Verilog-A JIT benchmark gate.
//!
//! This subcommand complements the process-level `run` benchmark. It measures
//! generated native entrypoint and public device-evaluation paths directly
//! against bytecode VM references on a dense synthetic model and can fail on
//! median speedup or native p95 budgets.

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
    #[arg(long, default_value_t = 3.0)]
    pub min_speedup: f64,

    /// Required dense entrypoint bytecode/native median speedup.
    #[arg(long, default_value_t = 2.0)]
    pub min_dense_speedup: f64,

    /// Required full-stamp bytecode/native median speedup.
    #[arg(long, default_value_t = 2.0)]
    pub min_full_stamp_speedup: f64,

    /// Maximum canonical-to-native setup time in milliseconds for each case.
    #[arg(long, value_name = "MS", default_value_t = 10.0)]
    pub max_native_setup_ms: f64,

    /// Absolute native p95 budget in ns per benchmark sweep.
    #[arg(long, value_name = "NS", default_value_t = 5_000.0)]
    pub max_native_p95_ns_per_sweep: f64,

    /// Maximum native sample relative standard deviation (for example 0.05 = 5%).
    #[arg(long, value_name = "RATIO", default_value_t = 0.25)]
    pub max_relative_stddev: f64,

    /// Maximum generated native image size for each benchmark case.
    #[arg(long, value_name = "BYTES", default_value_t = 16_384)]
    pub max_native_code_bytes: usize,

    /// Optional JSON report path.
    #[arg(long, value_name = "PATH")]
    pub out: Option<PathBuf>,
}

pub fn run(args: &NativeJitArgs) -> Result<ExitCode, BenchError> {
    let config = NativeBenchConfig {
        iterations: args.iterations,
        samples: args.samples,
        min_dense_speedup: args.min_dense_speedup,
        min_speedup: args.min_speedup,
        min_full_stamp_speedup: args.min_full_stamp_speedup,
        max_native_setup_ms: Some(args.max_native_setup_ms),
        max_native_p95_ns_per_sweep: Some(args.max_native_p95_ns_per_sweep),
        max_relative_stddev: Some(args.max_relative_stddev),
        max_native_code_bytes: Some(args.max_native_code_bytes),
    };
    let report =
        run_native_x64_benchmarks(config).map_err(|message| BenchError::NativeJit { message })?;

    println!(
        "native-jit target={} iterations={} samples={} min-dense-speedup={:.3} min-speedup={:.3} min-full-stamp-speedup={:.3}",
        report.target,
        report.iterations,
        report.samples,
        report.min_dense_speedup,
        report.min_speedup,
        report.min_full_stamp_speedup,
    );
    for case in &report.cases {
        println!(
            "  {name:<24} native median {native_median:>10.3} ns p95 {native_p95:>10.3} ns rsd {native_rsd:>6.2}%  bytecode median {bytecode_median:>10.3} ns  speedup {speedup:>7.3}x/{required_speedup:.3}x  setup {setup_us:>10.3} us code {code_bytes:>8} B entries {entry_points:>4}  [{status}]",
            name = case.name,
            native_median = case.native_ns_per_sweep.median,
            native_p95 = case.native_ns_per_sweep.p95,
            native_rsd = case.native_ns_per_sweep.relative_standard_deviation * 100.0,
            bytecode_median = case.bytecode_ns_per_sweep.median,
            speedup = case.speedup_median,
            required_speedup = case.required_speedup,
            setup_us = case.native_setup_ns / 1_000.0,
            code_bytes = case.native_code_bytes,
            entry_points = case.plan_stats.total_entry_points(),
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
