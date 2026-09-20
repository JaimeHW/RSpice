//! The mixed-signal exit-gate benchmarks: three circuits, each simulated with
//! its digital half executed by the Verilog-AMS interleave and again with every
//! part of it analog.
//!
//! This is the correctness half. The wall-clock half is
//! `tests/verilog_mixed_performance.rs`, which is a measurement and belongs in
//! nightly and in release — and which depends on this one, because a ratio
//! between two representations that disagree is not a speedup.
#![cfg(feature = "verilog-digital")]

use rspice_conformance::suites::verilog::mixed_benchmarks::{
    self as bench, BenchmarkOutcome, REQUIRED_BENCHMARKS,
};

/// The margin every declared bound must have over the error actually observed.
///
/// Two, and it is a floor rather than a target. A bound that is only just
/// holding is a bound that will stop holding on a machine whose rounding
/// differs, and the point of deriving one from named physical terms is that it
/// should not need to be close.
const REQUIRED_MARGIN: f64 = 2.0;

fn assert_agrees(outcome: &BenchmarkOutcome) {
    assert!(
        outcome.agrees(),
        "`{}` disagreed between its two representations:{}",
        outcome.name,
        outcome.report()
    );
    assert!(
        outcome.worst_margin() >= REQUIRED_MARGIN,
        "`{}` agreed, but its tightest bound had only {:.2}x of margin, under the {REQUIRED_MARGIN}x \
         floor. Either the bound is too tight for what the circuit actually does, or something \
         has moved and is about to fail:{}",
        outcome.name,
        outcome.worst_margin(),
        outcome.report()
    );
    assert!(
        !outcome.measurements.is_empty(),
        "`{}` measured nothing",
        outcome.name
    );
    assert!(
        !outcome.simplifications.is_empty(),
        "`{}` claims no simplifications; every one of these designs works around something the \
         mixed route does not carry, and the list is what keeps that honest",
        outcome.name
    );
    println!("{}", outcome.report());
}

#[test]
fn a_successive_approximation_converter_agrees_with_a_flash_reference() {
    let outcome = bench::sar_adc().unwrap_or_else(|error| panic!("{error}"));
    assert_eq!(outcome.name, "sar_adc");
    assert_eq!(
        outcome.measurements.len(),
        6,
        "one measurement per converted code"
    );
    assert_agrees(&outcome);
}

#[test]
fn a_phase_locked_loop_agrees_with_an_all_analog_phase_detector() {
    let outcome = bench::pll().unwrap_or_else(|error| panic!("{error}"));
    assert_eq!(outcome.name, "pll");
    // The three quantities the brief asks a PLL benchmark for, and the control
    // voltage they both come out of.
    let quantities: Vec<&str> = outcome
        .measurements
        .iter()
        .map(|measurement| measurement.quantity.as_str())
        .collect();
    assert!(quantities.contains(&"settled control voltage"));
    assert!(quantities.contains(&"locked VCO frequency"));
    assert!(quantities.contains(&"lock time"));
    assert_agrees(&outcome);
}

#[test]
fn a_sigma_delta_modulator_agrees_with_an_all_analog_quantizer() {
    let outcome = bench::sigma_delta().unwrap_or_else(|error| panic!("{error}"));
    assert_eq!(outcome.name, "sigma_delta");
    assert_agrees(&outcome);
}

/// Every benchmark in the list runs, and the list is the list.
///
/// A count would say the set changed size; this says which circuit stopped
/// being covered. It runs all three again rather than reusing the tests above,
/// because what it is checking is the registry — that `all()` reaches each of
/// them — which the individual tests cannot see.
#[test]
fn every_required_benchmark_is_registered_and_runs() {
    let outcomes: Vec<BenchmarkOutcome> = bench::all()
        .into_iter()
        .map(|benchmark| benchmark().unwrap_or_else(|error| panic!("{error}")))
        .collect();
    let names: Vec<&str> = outcomes.iter().map(|outcome| outcome.name).collect();
    for required in REQUIRED_BENCHMARKS {
        assert!(
            names.contains(&required),
            "`{required}` is required and `all()` does not reach it; it reached {names:?}"
        );
    }
    assert_eq!(
        names.len(),
        REQUIRED_BENCHMARKS.len(),
        "a benchmark was added to `all()` without a row in REQUIRED_BENCHMARKS"
    );
    for outcome in &outcomes {
        assert!(
            outcome.agrees(),
            "`{}` disagreed:{}",
            outcome.name,
            outcome.report()
        );
    }
}
