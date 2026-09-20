//! What the mixed-signal interleave costs against simulating the same circuit
//! entirely in the analog domain.
//!
//! # Why this is a separate target
//!
//! `verilog_mixed_benchmarks` is the correctness gate and runs on every push.
//! This is a wall-clock measurement, and wall-clock measurements belong in
//! nightly and in release for the two reasons `verilog_rnm_performance` gives:
//! a timing assertion on a shared per-push runner is a flake generator, and the
//! two sides here are dominated by different work — a Verilog front end and an
//! event wheel on one, matrix solves on the other — that `opt-level = 0` slows
//! by different factors, so a debug measurement is a different ratio rather
//! than a slower one.
//!
//! # What is asserted, and what is only printed
//!
//! Asserted: that both representations of each circuit still agree. A ratio
//! between two answers that differ is not a speedup, so the correctness
//! precondition is checked here too rather than assumed from the other target.
//!
//! Printed: the ratio. Not asserted, because there is no honest a-priori claim
//! that the mixed representation is faster on an arbitrary circuit. It is
//! faster exactly when the digital half would otherwise cost the analog solver
//! more matrix than the interleave costs in trial machinery — and two of the
//! three circuits here are small enough that it does not. The table says which,
//! and by how much, which is what a reader needs to decide whether their own
//! design is on the winning side of that trade.
#![cfg(feature = "verilog-digital")]

use rspice_conformance::suites::verilog::mixed_benchmarks::{self as bench, BenchmarkOutcome};

/// One row of the measurement, in the shape the table prints.
struct Row {
    name: &'static str,
    mixed_seconds: f64,
    reference_seconds: f64,
    mixed_points: usize,
    reference_points: usize,
}

impl Row {
    fn ratio(&self) -> f64 {
        self.mixed_seconds / self.reference_seconds.max(f64::MIN_POSITIVE)
    }
}

#[test]
fn mixed_against_all_analog_wall_clock() {
    let mut rows = Vec::new();
    for benchmark in bench::all() {
        let outcome: BenchmarkOutcome = benchmark().unwrap_or_else(|error| panic!("{error}"));
        assert!(
            outcome.agrees(),
            "`{}` disagreed between its two representations, so no ratio taken from it means \
             anything:{}",
            outcome.name,
            outcome.report()
        );
        println!("{}", outcome.report());
        rows.push(Row {
            name: outcome.name,
            mixed_seconds: outcome.mixed_wall.as_secs_f64(),
            reference_seconds: outcome.reference_wall.as_secs_f64(),
            mixed_points: outcome.mixed_points,
            reference_points: outcome.reference_points,
        });
    }

    println!(
        "\n{:<14} {:>12} {:>12} {:>10} {:>12} {:>12} {:>10}",
        "benchmark",
        "mixed(ms)",
        "analog(ms)",
        "mixed/ref",
        "mixed pts",
        "analog pts",
        "ms/kpt mix"
    );
    for row in &rows {
        println!(
            "{:<14} {:>12.2} {:>12.2} {:>9.2}x {:>12} {:>12} {:>10.3}",
            row.name,
            row.mixed_seconds * 1.0e3,
            row.reference_seconds * 1.0e3,
            row.ratio(),
            row.mixed_points,
            row.reference_points,
            row.mixed_seconds * 1.0e6 / row.mixed_points.max(1) as f64
        );
    }

    // The one thing worth stating without asserting it: where the mixed route
    // wins, it wins because the analog reference needs more of the solver's
    // time, not less of it. The accepted-point counts beside the times are what
    // says so, and they are printed for every row rather than only the winning
    // one.
    let winners: Vec<&str> = rows
        .iter()
        .filter(|row| row.ratio() < 1.0)
        .map(|row| row.name)
        .collect();
    println!(
        "\nmixed is cheaper on: {}",
        if winners.is_empty() {
            "none of these circuits".to_string()
        } else {
            winners.join(", ")
        }
    );
    println!(
        "The interleave costs a probe trial per Newton evaluation and a settle per accepted \
         timepoint, and saves whatever the digital half would have cost the matrix. On these \
         circuits the digital half is a handful of registers, so the saving is only visible \
         where the all-analog representation has to spend accepted timepoints resolving what \
         the discrete domain decides for free — which is what the accepted-point columns show."
    );

    assert!(
        rows.len() == 3,
        "every benchmark must be measured, saw {}",
        rows.len()
    );
}
