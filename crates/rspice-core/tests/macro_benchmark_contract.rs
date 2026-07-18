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
}

#[test]
fn rc_ladder_1k_remains_a_healthy_linear_benchmark() {
    assert_linear_ladder_runs(RC_LADDER_1K);
}

#[test]
fn rc_ladder_10k_remains_a_healthy_linear_benchmark() {
    assert_linear_ladder_runs(RC_LADDER_10K);
}
