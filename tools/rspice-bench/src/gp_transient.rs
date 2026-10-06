//! Native GP public-run qualification. Timings remain local artifacts.

mod observation;
mod workload;

use crate::{error::BenchError, provenance, report};
use clap::Args;
use rspice_core::{Engine, GpTransientPhaseModel};
use serde::Serialize;
use std::path::PathBuf;
use std::process::ExitCode;
use workload::{Cancellation, Waveform, Workload, policy};

#[derive(Args, Debug)]
pub struct GpTransientArgs {
    /// Measured full runs and cancellation runs per case, after one warmup.
    #[arg(long, default_value_t = 5, value_parser = clap::value_parser!(u32).range(2..=100))]
    samples: u32,
    /// Parallel GP instances sharing driven external terminals, with private R nodes.
    #[arg(long, value_delimiter = ',', default_value = "1,16", value_parser = clap::value_parser!(u32).range(1..=128))]
    devices: Vec<u32>,
    /// Horizon in 4 ps maximum-step intervals; the adaptive solver may take more.
    #[arg(long, default_value_t = 1024, value_parser = clap::value_parser!(u32).range(256..=1_000_000))]
    steps: u32,
    /// Immutable report path, normally under ignored target/benchmarks.
    #[arg(long, default_value = "target/benchmarks/gp-transient.json")]
    out: PathBuf,
    /// Permit a debug or dirty build; its report is not release qualification.
    #[arg(long)]
    exploratory: bool,
    /// Optional per-case median public-run budget, in milliseconds.
    #[arg(long)]
    max_run_ms: Option<f64>,
    /// Optional maximum request-to-public-return latency, in milliseconds.
    #[arg(long)]
    max_cancel_ms: Option<f64>,
    /// Optional maximum charged transport-record capacity, including retained copies.
    #[arg(long)]
    max_transport_bytes: Option<usize>,
}

#[derive(Serialize)]
struct Case {
    devices: u32,
    model: &'static str,
    checkpoints: bool,
    deck: String,
    deck_blake3: String,
    checkpoint_times: Vec<f64>,
    waveform: Waveform,
    run_ms: Vec<f64>,
    median_run_ms: f64,
    cancellation: Vec<Cancellation>,
    /// High-water charge established at the public resource refusal boundary.
    peak_transport_bytes: usize,
    passed: bool,
}

#[derive(Serialize)]
struct Report<'a> {
    methodology: &'static str,
    configuration: &'static str,
    core_default_features_requested: bool,
    exploratory: bool,
    samples: u32,
    steps: u32,
    max_step_seconds: f64,
    max_run_ms: Option<f64>,
    max_cancel_ms: Option<f64>,
    max_transport_bytes: Option<usize>,
    cases: &'a [Case],
    passed: bool,
}

const METHODOLOGY: &str = "One warmup per case, then repeated public runs with an accepted-sample \
    observer and cancellation at the first accepted sample at or beyond half the horizon. Time includes circuit construction, \
    OP, solve, observation, scheduled checkpoint capture and release; excludes netlist parsing, \
    output validation/hash and waveform release. Cancellation measures callback request to \
    public return including unwind/destruction, without another thread's scheduling delay. \
    All repeats and post-cancellation reuse must preserve waveform bits and work counts. \
    Peak storage is the smallest successful max_transport_history_bytes quota, verified at \
    one byte below and repeated before/after timings; it charges live transport-record \
    capacities, transient copies and retained checkpoints, not fixed BJT state, allocator \
    overhead, result/solver buffers or process RSS. Peak probing is outside timed runs. \
    Wall times are host-dependent; raw repeats are retained, no cold-cache control. \
    Numerical correctness is qualified separately by core analytical/reference tests.";

pub fn run(args: &GpTransientArgs) -> Result<ExitCode, BenchError> {
    for (name, value) in [
        ("--max-run-ms", args.max_run_ms),
        ("--max-cancel-ms", args.max_cancel_ms),
    ] {
        if value.is_some_and(|value| !value.is_finite() || value <= 0.0) {
            return Err(policy(format!("{name} must be finite and positive")));
        }
    }
    if args.out.exists() {
        return Err(policy(format!(
            "report `{}` already exists",
            args.out.display()
        )));
    }
    let tool = provenance::tool();
    if !args.exploratory {
        provenance::require_release(&tool)?;
        if tool.git_commit.is_none() || tool.git_dirty != Some(false) {
            return Err(policy(
                "GP qualification requires a clean committed checkout; use --exploratory for development",
            ));
        }
    }
    let mut cases = Vec::new();
    for &devices in &args.devices {
        for (model, name) in [
            (GpTransientPhaseModel::ExactDelay, "exact_delay"),
            (GpTransientPhaseModel::NgspiceWeil, "ngspice_weil"),
        ] {
            for checkpoints in [false, true] {
                let workload = Workload::new(devices, args.steps, model, checkpoints)?;
                let engine = Engine::new(workload.config.clone());
                let (_, waveform) = workload.measure(&engine)?;
                let peak_transport_bytes = workload.peak_transport_bytes()?;
                let mut run_ms = Vec::new();
                let mut cancellation = Vec::new();
                // Warm cancellation too: first-use cleanup is not hidden among
                // the repeated steady-state measurements.
                workload.cancel(&engine)?;
                for _ in 0..args.samples {
                    let (elapsed, repeated) = workload.measure(&engine)?;
                    if repeated != waveform {
                        return Err(policy(format!(
                            "nonrepeatable GP waveform/work counts for {name}, {devices} devices"
                        )));
                    }
                    run_ms.push(elapsed);
                    cancellation.push(workload.cancel(&engine)?);
                }
                // The last cancellation also has to leave the engine reusable.
                if workload.measure(&engine)?.1 != waveform
                    || workload.peak_transport_bytes()? != peak_transport_bytes
                {
                    return Err(policy("GP engine reuse or peak transport charge changed"));
                }
                let mut limited = workload.config.clone();
                limited.resource_limits.max_transport_history_bytes = peak_transport_bytes;
                if workload.measure(&Engine::new(limited))?.1 != waveform {
                    return Err(policy("GP transport quota changed the numerical result"));
                }
                let median_run_ms = median(&run_ms);
                let passed = args.max_run_ms.is_none_or(|limit| median_run_ms <= limit)
                    && args
                        .max_transport_bytes
                        .is_none_or(|limit| peak_transport_bytes <= limit)
                    && args.max_cancel_ms.is_none_or(|limit| {
                        cancellation
                            .iter()
                            .all(|sample| sample.request_to_return_ms <= limit)
                    });
                println!(
                    "GP {name}, {devices} devices, checkpoints={checkpoints}: median {median_run_ms:.3} ms, transport {peak_transport_bytes} bytes, passed={passed}"
                );
                cases.push(Case {
                    devices,
                    model: name,
                    checkpoints,
                    deck_blake3: blake3::hash(workload.deck.as_bytes()).to_hex().to_string(),
                    deck: workload.deck,
                    checkpoint_times: workload.schedule,
                    waveform,
                    run_ms,
                    median_run_ms,
                    cancellation,
                    peak_transport_bytes,
                    passed,
                });
            }
        }
    }
    let passed = cases.iter().all(|case| case.passed);
    report::write(
        &args.out,
        "rspice-gp-transient",
        &Report {
            methodology: METHODOLOGY,
            configuration: "Ngspice dialect; OP startup; NPN; private RB/RE/RC; TF=1ns; PTF=21deg; 0.6V + 5mV sine at 1GHz; adaptive grid; default solver/tolerances except deck options; default resource limits outside quota probing",
            core_default_features_requested: cfg!(feature = "core-transient-default"),
            exploratory: args.exploratory,
            samples: args.samples,
            steps: args.steps,
            max_step_seconds: 4e-12,
            max_run_ms: args.max_run_ms,
            max_cancel_ms: args.max_cancel_ms,
            max_transport_bytes: args.max_transport_bytes,
            cases: &cases,
            passed,
        },
        true,
    )?;
    Ok(if passed {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

fn median(values: &[f64]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let mid = sorted.len() / 2;
    if sorted.len().is_multiple_of(2) {
        (sorted[mid - 1] + sorted[mid]) * 0.5
    } else {
        sorted[mid]
    }
}

#[cfg(test)]
mod tests;
