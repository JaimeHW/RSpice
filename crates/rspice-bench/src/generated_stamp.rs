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

    /// Fail if the corpus median exceeds this multiple of the same-run
    /// hand-written compact-model reference.
    #[arg(long, value_name = "RATIO")]
    pub max_corpus_median_reference_ratio: Option<f64>,

    /// Fail if any measured model exceeds this multiple of the same-run
    /// hand-written compact-model reference.
    #[arg(long, value_name = "RATIO")]
    pub max_model_reference_ratio: Option<f64>,

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
    ratio_vs_reference: Option<f64>,
}

impl ModelReport {
    fn from_result(result: &GeneratedStampBenchResult, reference_ns: Option<f64>) -> Self {
        Self {
            model_name: result.model_name.to_string(),
            node_count: result.node_count,
            branch_count: result.branch_count,
            linked_slot_count: result.linked_slot_count,
            ns_per_stamp_median: result.ns_per_stamp_median,
            ns_per_stamp_p95: result.ns_per_stamp_p95,
            ns_per_stamp_min: result.ns_per_stamp_min,
            ratio_vs_reference: reference_ns
                .map(|reference| result.ns_per_stamp_median / reference.max(f64::MIN_POSITIVE)),
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
    schema_version: u32,
    iterations: usize,
    samples: usize,
    measured: Vec<ModelReport>,
    /// Hand-written model measured in the same run, when available.
    reference: Option<ReferenceReport>,
    corpus_median_ns: f64,
    corpus_median_reference_ratio: Option<f64>,
    max_median_ns_per_stamp: Option<f64>,
    max_corpus_median_reference_ratio: Option<f64>,
    max_model_reference_ratio: Option<f64>,
    passed: bool,
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
    let middle = medians.len() / 2;
    if medians.len().is_multiple_of(2) {
        (medians[middle - 1] + medians[middle]) * 0.5
    } else {
        medians[middle]
    }
}

pub fn run(args: &GeneratedStampArgs) -> Result<ExitCode, BenchError> {
    validate_positive_budget("--max-median-ns-per-stamp", args.max_median_ns_per_stamp)?;
    validate_positive_budget(
        "--max-corpus-median-reference-ratio",
        args.max_corpus_median_reference_ratio,
    )?;
    validate_positive_budget(
        "--max-model-reference-ratio",
        args.max_model_reference_ratio,
    )?;

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
    let reference_ns = reference
        .as_ref()
        .ok()
        .map(|result| result.ns_per_stamp_median);
    let corpus_median_ns = median_of(&measured);
    let corpus_median_reference_ratio =
        reference_ns.map(|reference| corpus_median_ns / reference.max(f64::MIN_POSITIVE));
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
            if corpus_median_ns > 0.0 && result.ns_per_stamp_median > 0.0 {
                println!(
                    "\ncorpus median {:.1} ns is {:.2}x the hand-written reference ({:.1} ns)",
                    corpus_median_ns,
                    corpus_median_ns / result.ns_per_stamp_median,
                    result.ns_per_stamp_median,
                );
            }
        }
        Err(error) => eprintln!("reference measurement unavailable: {error}"),
    }

    if let Some(budget) = args.max_median_ns_per_stamp {
        for result in &measured {
            if result.ns_per_stamp_median > budget {
                failed.push(format!(
                    "{} median {:.1} ns exceeds {:.1} ns",
                    result.model_name, result.ns_per_stamp_median, budget
                ));
            }
        }
    }
    if args.max_corpus_median_reference_ratio.is_some() || args.max_model_reference_ratio.is_some()
    {
        match reference_ns {
            Some(reference_ns) => {
                if let Some(budget) = args.max_corpus_median_reference_ratio {
                    let ratio = corpus_median_ns / reference_ns.max(f64::MIN_POSITIVE);
                    if ratio > budget {
                        failed.push(format!(
                            "corpus median/reference ratio {ratio:.3} exceeds {budget:.3}"
                        ));
                    }
                }
                if let Some(budget) = args.max_model_reference_ratio {
                    for result in &measured {
                        let ratio =
                            result.ns_per_stamp_median / reference_ns.max(f64::MIN_POSITIVE);
                        if ratio > budget {
                            failed.push(format!(
                                "{} median/reference ratio {:.3} exceeds {:.3}",
                                result.model_name, ratio, budget
                            ));
                        }
                    }
                }
            }
            None => failed.push(
                "a hand-written reference is required by the configured ratio budget".to_string(),
            ),
        }
    }

    let report = StampReport {
        schema_version: 1,
        iterations: args.iterations,
        samples: args.samples,
        measured: measured
            .iter()
            .map(|result| ModelReport::from_result(result, reference_ns))
            .collect(),
        reference: reference.as_ref().ok().map(|result| ReferenceReport {
            model_name: result.model_name.to_string(),
            node_count: result.node_count,
            ns_per_stamp_median: result.ns_per_stamp_median,
            ns_per_stamp_p95: result.ns_per_stamp_p95,
            ns_per_stamp_min: result.ns_per_stamp_min,
        }),
        corpus_median_ns,
        corpus_median_reference_ratio,
        max_median_ns_per_stamp: args.max_median_ns_per_stamp,
        max_corpus_median_reference_ratio: args.max_corpus_median_reference_ratio,
        max_model_reference_ratio: args.max_model_reference_ratio,
        passed: failed.is_empty(),
        failed: failed.clone(),
    };
    if let Some(path) = &args.out {
        crate::report::write(path, "rspice-generated-stamp", &report, true)?;
    }

    for failure in &failed {
        eprintln!("FAIL {failure}");
    }
    Ok(if report.passed {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

fn validate_positive_budget(label: &str, value: Option<f64>) -> Result<(), BenchError> {
    if value.is_some_and(|value| !value.is_finite() || value <= 0.0) {
        return Err(BenchError::BenchmarkPolicy {
            message: format!("{label} must be finite and greater than zero"),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result(median: f64) -> GeneratedStampBenchResult {
        GeneratedStampBenchResult {
            model_name: "fixture",
            node_count: 2,
            branch_count: 0,
            linked_slot_count: 4,
            ns_per_stamp_median: median,
            ns_per_stamp_p95: median,
            ns_per_stamp_min: median,
        }
    }

    #[test]
    fn corpus_median_averages_the_middle_pair() {
        assert_eq!(median_of(&[result(40.0), result(10.0)]), 25.0);
        assert_eq!(median_of(&[result(30.0), result(10.0), result(20.0)]), 20.0);
    }

    #[test]
    fn ratio_reports_use_the_same_run_reference() {
        let report = ModelReport::from_result(&result(750.0), Some(500.0));
        assert_eq!(report.ratio_vs_reference, Some(1.5));
    }

    #[test]
    fn performance_budgets_reject_non_positive_or_non_finite_values() {
        for value in [0.0, -1.0, f64::INFINITY, f64::NAN] {
            assert!(validate_positive_budget("--fixture", Some(value)).is_err());
        }
        validate_positive_budget("--fixture", None).expect("an unset budget is valid");
        validate_positive_budget("--fixture", Some(1.0)).expect("a positive budget is valid");
    }
}
