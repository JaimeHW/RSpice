//! Correctness contracts for the macro-benchmark workloads.
//!
//! A performance result is meaningless when the measured process exits early.
//! Keep representative scale decks on the ordinary core path so benchmark
//! health cannot drift independently from simulator correctness.

use rspice_core::{Engine, Netlist};
use std::fmt::Write as _;

/// The RC ladder `rspice-bench run` generates (`tools/rspice-bench/src/generate.rs`):
/// identical 100-ohm / 1 pF sections driven by a PULSE source, printing the
/// far-end node.
fn rc_ladder_deck(sections: usize, pulse: &str, tstep: &str, tstop: &str) -> String {
    let mut deck = format!("* rc_ladder_{sections}\nVIN in 0 DC 0 PULSE({pulse})\n");
    let mut prev = String::from("in");
    for section in 1..=sections {
        let node = format!("n{section}");
        let _ = writeln!(deck, "R{section} {prev} {node} 100");
        let _ = writeln!(deck, "C{section} {node} 0 1p");
        prev = node;
    }
    let _ = write!(
        deck,
        ".options noacct\n.tran {tstep} {tstop}\n.print tran v({prev})\n.end\n"
    );
    deck
}

fn assert_linear_ladder_runs(source: &str) {
    let netlist = Netlist::parse(source).expect("benchmark deck parses");
    let engine = Engine::default();
    let circuit = engine
        .build_circuit(&netlist)
        .expect("benchmark circuit builds");
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
    let retained_voltages = result
        .voltages
        .iter()
        .filter(|waveform| !waveform.is_empty())
        .collect::<Vec<_>>();
    assert_eq!(
        retained_voltages.len(),
        1,
        "a voltage-only .PRINT must retain only its requested node waveform"
    );
    assert_eq!(result.time.len(), retained_voltages[0].len());
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
    assert_linear_ladder_runs(&rc_ladder_deck(1_000, "0 1 0 1n 1n 1.999u 4u", "2n", "20u"));
}

#[test]
fn rc_ladder_10k_remains_a_healthy_linear_benchmark() {
    assert_linear_ladder_runs(&rc_ladder_deck(10_000, "0 1 0 1n 1n 0.999m 2m", "2u", "2m"));
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

#[test]
fn expression_current_dependencies_are_retained() {
    let deck = "expression derived currents\n\
                V1 in 0 PULSE(0 1 0 1n 1n 8n 20n)\n\
                R1 in out 1k\n\
                C1 out 0 1p\n\
                .tran 1n 5n\n\
                .print tran {1/I(R1)} {I(C1)+I(R1)}\n\
                .end\n";
    let netlist = Netlist::parse(deck).expect("expression selection deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 5.0e-9, 1.0e-9)
        .expect("expression selection deck runs");

    for expected in ["R1", "C1"] {
        assert!(
            result
                .branch_names
                .iter()
                .any(|name| name.eq_ignore_ascii_case(expected)),
            "an expression dependency on I({expected}) must retain its derived current channel"
        );
    }
}
