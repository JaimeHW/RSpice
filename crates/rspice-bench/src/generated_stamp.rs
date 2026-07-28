//! Stamp-throughput gate for the generated Verilog-A built-ins.
//!
//! Complements `generated-rust`, which answers how much source rustc must
//! ingest, with the runtime half of the same question: how long a generated
//! device takes to evaluate and stamp inside the Newton loop. The measurement
//! itself lives in `rspice-core`, next to the private device state it drives;
//! this is the command-line surface over it.

use crate::error::BenchError;
use clap::Args;
use rspice_conformance::suites::veriloga::bench::{
    GeneratedStampBenchConfig, GeneratedStampBenchResult, run_generated_stamp_benchmarks,
};
use rspice_conformance::suites::veriloga::reference::{
    ReferenceStampBenchConfig, run_reference_stamp_benchmark,
};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

/// Arguments for the `generated-stamp` subcommand.
#[derive(Args, Debug)]
pub struct GeneratedStampArgs {
    /// Stamp calls per timed sample.
    #[arg(long, default_value_t = 2_000)]
    pub iterations: usize,

    /// Timed samples; statistics are taken across these, not within one.
    #[arg(long, default_value_t = 7)]
    pub samples: usize,

    /// Restrict the sweep to these model names. Repeatable; default is all.
    #[arg(long = "model", value_name = "NAME")]
    pub models: Vec<String>,

    /// Fail if any measured model exceeds this median, in ns per stamp.
    #[arg(long, value_name = "NS")]
    pub max_median_ns_per_stamp: Option<f64>,

    /// Optional JSON report path.
    #[arg(long, value_name = "PATH")]
    pub out: Option<PathBuf>,
}

#[derive(Serialize)]
struct ModelReport {
    model_name: String,
    node_count: usize,
    branch_count: usize,
    linked_slot_count: usize,
    ns_per_stamp_median: f64,
    ns_per_stamp_p95: f64,
    ns_per_stamp_min: f64,
}

impl From<&GeneratedStampBenchResult> for ModelReport {
    fn from(result: &GeneratedStampBenchResult) -> Self {
        Self {
            model_name: result.model_name.to_string(),
            node_count: result.node_count,
            branch_count: result.branch_count,
            linked_slot_count: result.linked_slot_count,
            ns_per_stamp_median: result.ns_per_stamp_median,
            ns_per_stamp_p95: result.ns_per_stamp_p95,
            ns_per_stamp_min: result.ns_per_stamp_min,
        }
    }
}

#[derive(Serialize)]
struct ReferenceReport {
    model_name: String,
    node_count: usize,
    ns_per_stamp_median: f64,
    ns_per_stamp_p95: f64,
    ns_per_stamp_min: f64,
}

#[derive(Serialize)]
struct StampReport {
    iterations: usize,
    samples: usize,
    measured: Vec<ModelReport>,
    /// Hand-written model measured in the same run, when available.
    reference: Option<ReferenceReport>,
    failed: Vec<String>,
}

/// Median of the sweep, over a list already sorted slowest-first.
fn median_of(measured: &[GeneratedStampBenchResult]) -> f64 {
    if measured.is_empty() {
        return 0.0;
    }
    let mut medians: Vec<f64> = measured
        .iter()
        .map(|result| result.ns_per_stamp_median)
        .collect();
    medians.sort_by(f64::total_cmp);
    medians[medians.len() / 2]
}

pub fn run(args: &GeneratedStampArgs) -> Result<ExitCode, BenchError> {
    let config = GeneratedStampBenchConfig {
        iterations: args.iterations,
        samples: args.samples,
        models: args.models.clone(),
    };

    let mut measured = Vec::new();
    let mut failed = Vec::new();
    for outcome in run_generated_stamp_benchmarks(&config) {
        match outcome {
            Ok(result) => measured.push(result),
            Err(error) => failed.push(error.to_string()),
        }
    }

    // Slowest first: the table exists to show which models cost the most.
    measured.sort_by(|left, right| {
        right
            .ns_per_stamp_median
            .total_cmp(&left.ns_per_stamp_median)
    });

    println!(
        "{:<24} {:>6} {:>7} {:>8} {:>12} {:>12}",
        "model", "nodes", "branch", "slots", "median ns", "p95 ns"
    );
    for result in &measured {
        println!(
            "{:<24} {:>6} {:>7} {:>8} {:>12.1} {:>12.1}",
            result.model_name,
            result.node_count,
            result.branch_count,
            result.linked_slot_count,
            result.ns_per_stamp_median,
            result.ns_per_stamp_p95,
        );
    }
    // The hand-written reference, in the same units and on the same machine.
    // Printed last and labelled, because it is the yardstick the generated
    // numbers above are meant to be read against rather than another entry in
    // the sweep.
    let reference = run_reference_stamp_benchmark(&ReferenceStampBenchConfig {
        iterations: args.iterations,
        samples: args.samples,
    });
    match &reference {
        Ok(result) => {
            println!(
                "{:<24} {:>6} {:>7} {:>8} {:>12.1} {:>12.1}   <- hand-written reference",
                result.model_name,
                result.node_count,
                "-",
                "-",
                result.ns_per_stamp_median,
                result.ns_per_stamp_p95,
            );
            let median = median_of(&measured);
            if median > 0.0 && result.ns_per_stamp_median > 0.0 {
                println!(
                    "\ncorpus median {:.1} ns is {:.2}x the hand-written reference ({:.1} ns)",
                    median,
                    median / result.ns_per_stamp_median,
                    result.ns_per_stamp_median,
                );
            }
        }
        Err(error) => eprintln!("reference measurement unavailable: {error}"),
    }

    for failure in &failed {
        eprintln!("skipped: {failure}");
    }

    let report = StampReport {
        iterations: args.iterations,
        samples: args.samples,
        measured: measured.iter().map(ModelReport::from).collect(),
        reference: reference.as_ref().ok().map(|result| ReferenceReport {
            model_name: result.model_name.to_string(),
            node_count: result.node_count,
            ns_per_stamp_median: result.ns_per_stamp_median,
            ns_per_stamp_p95: result.ns_per_stamp_p95,
            ns_per_stamp_min: result.ns_per_stamp_min,
        }),
        failed: failed.clone(),
    };
    if let Some(path) = &args.out {
        let json = serde_json::to_string_pretty(&report)
            .map_err(|error| BenchError::GeneratedStamp { message: format!("serializing stamp report: {error}") })?;
        fs::write(path, json).map_err(|error| {
            BenchError::GeneratedStamp { message: format!("writing {}: {error}", path.display()) }
        })?;
    }

    // A model that could not be measured is a gate failure, not a footnote:
    // silently dropping it would let a device regress to unbenchmarkable.
    if !failed.is_empty() {
        return Ok(ExitCode::FAILURE);
    }
    if let Some(budget) = args.max_median_ns_per_stamp {
        let over: Vec<&GeneratedStampBenchResult> = measured
            .iter()
            .filter(|result| result.ns_per_stamp_median > budget)
            .collect();
        if !over.is_empty() {
            for result in over {
                eprintln!(
                    "over budget: {} median {:.1} ns > {:.1} ns",
                    result.model_name, result.ns_per_stamp_median, budget
                );
            }
            return Ok(ExitCode::FAILURE);
        }
    }
    Ok(ExitCode::SUCCESS)
}
