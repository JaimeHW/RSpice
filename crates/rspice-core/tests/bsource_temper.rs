//! `temper` and `hertz` in behavioral-source expressions.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn op_voltage(deck: &str, node: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let op = engine.run_dc_op(&netlist).expect("operating point solves");
    let idx = op
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing"));
    op.node_voltages[idx]
}

#[test]
fn temper_reads_default_circuit_temperature() {
    let deck = "\
* temper default
b1 out 0 v=temper
r1 out 0 1k
.op
.end
";
    let v = op_voltage(deck, "out");
    assert!(
        (v - 27.0).abs() < 1e-9,
        "temper must read 27 C at the default temperature, got {v}"
    );
}

#[test]
fn temper_follows_options_temp() {
    let deck = "\
* temper at 85C
b1 out 0 v=temper
r1 out 0 1k
.options temp=85
.op
.end
";
    let v = op_voltage(deck, "out");
    assert!(
        (v - 85.0).abs() < 1e-9,
        "temper must follow .options temp=85, got {v}"
    );
}

#[test]
fn temper_composes_in_expressions() {
    let deck = "\
* expression using temper
b1 out 0 v={(temper-25)*0.1}
r1 out 0 1k
.options temp=75
.op
.end
";
    let v = op_voltage(deck, "out");
    assert!(
        (v - 5.0).abs() < 1e-9,
        "(75-25)*0.1 must give 5, got {v}"
    );
}

#[test]
fn hertz_parses_and_reads_zero_at_dc() {
    // ngspice semantics: hertz is the AC frequency variable and reads 0 in
    // DC/transient contexts.
    let deck = "\
* hertz at dc
b1 out 0 v={1+hertz}
r1 out 0 1k
.op
.end
";
    let v = op_voltage(deck, "out");
    assert!(
        (v - 1.0).abs() < 1e-9,
        "hertz must read 0 at DC, got 1+hertz = {v}"
    );
}
