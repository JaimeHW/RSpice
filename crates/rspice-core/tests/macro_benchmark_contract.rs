//! Correctness contracts for the checked-in macro-benchmark decks.
//!
//! A performance result is meaningless when the measured process exits early.
//! Keep representative scale decks on the ordinary core path so benchmark
//! health cannot drift independently from simulator correctness.

use rspice_core::{Engine, Netlist};

const RC_LADDER_1K: &str = include_str!("../../../benchmarks/circuits/rc_ladder_1000.cir");
const RC_LADDER_10K: &str = include_str!("../../../benchmarks/circuits/rc_ladder_10000.cir");

fn assert_linear_ladder_runs(source: &str) {
    let netlist = Netlist::parse(source).expect("checked-in benchmark deck parses");
    let engine = Engine::default();
    let circuit = engine
        .build_circuit(&netlist)
        .expect("checked-in benchmark circuit builds");
    assert!(
        !circuit.has_nonlinear_devices(),
        "an RC ladder must stay on the one-solve linear transient path"
    );

    let (step, stop) = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            rspice_core::netlist::AnalysisCommand::Tran { step, stop, .. } => Some((*step, *stop)),
            _ => None,
        })
        .expect("benchmark deck has .TRAN");
    // Exercise startup, the PULSE edge, and multiple ordinary steps without
    // making the correctness suite duplicate the full benchmark workload.
    let smoke_stop = stop.min(step * 8.0);
    let result = engine
        .run_tran(&netlist, smoke_stop, step)
        .expect("linear benchmark transient completes");
    assert!(result.time.len() > 2, "benchmark retained transient points");
    assert_eq!(result.time.len(), result.voltages[0].len());
    assert!(
        result
            .branch_names
            .iter()
            .all(|name| !name.starts_with('R') && !name.starts_with('C')),
        "voltage-only .PRINT must not retain every derived R/C current"
    );
}

#[test]
fn rc_ladder_1k_remains_a_healthy_linear_benchmark() {
    assert_linear_ladder_runs(RC_LADDER_1K);
}

#[test]
fn rc_ladder_10k_remains_a_healthy_linear_benchmark() {
    assert_linear_ladder_runs(RC_LADDER_10K);
}

#[test]
fn explicitly_requested_derived_current_is_retained() {
    let deck = "selected derived current\n\
                V1 in 0 PULSE(0 1 0 1n 1n 8n 20n)\n\
                R1 in out 1k\n\
                C1 out 0 1p\n\
                .tran 1n 5n\n\
                .print tran I(R1)\n\
                .end\n";
    let netlist = Netlist::parse(deck).expect("selection deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 5.0e-9, 1.0e-9)
        .expect("selection deck runs");

    assert!(
        result
            .branch_names
            .iter()
            .any(|name| name.eq_ignore_ascii_case("R1")),
        "I(R1) must retain the derived resistor-current channel"
    );
    assert!(
        result
            .branch_names
            .iter()
            .all(|name| !name.eq_ignore_ascii_case("C1")),
        "an unrequested derived capacitor current must not be retained"
    );
}
