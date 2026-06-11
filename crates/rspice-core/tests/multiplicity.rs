//! Parallel-multiplicity (`m=`) semantics across devices and hierarchy.
//!
//! The defining property of `m` is exact parallel equivalence: an instance
//! with `m=N` must produce the same solution as N explicit copies wired in
//! parallel. Every test below checks that property (or composition of the
//! factor through subcircuit hierarchy) at the operating point.

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
        .unwrap_or_else(|| panic!("node {node} missing from OP result"));
    op.node_voltages[idx]
}

fn resistor_conductance(deck: &str, name: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let circuit = Engine::default()
        .build_circuit(&netlist)
        .expect("circuit builds");
    let resistors = circuit.resistor_storage();
    let idx = resistors
        .names
        .iter()
        .position(|n| n.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("resistor {name} missing; have {:?}", resistors.names));
    resistors.conductances[idx]
}

fn capacitance_of(deck: &str, name: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let circuit = Engine::default()
        .build_circuit(&netlist)
        .expect("circuit builds");
    let capacitors = circuit.capacitor_storage();
    let idx = capacitors
        .names
        .iter()
        .position(|n| n.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("capacitor {name} missing; have {:?}", capacitors.names));
    capacitors.capacitances[idx]
}

#[test]
fn resistor_m_matches_explicit_parallel_copies() {
    let with_m = "\
* divider with m
v1 in 0 dc 10
r1 in out 3k m=3
r2 out 0 1k
.op
.end
";
    let explicit = "\
* divider with explicit parallel resistors
v1 in 0 dc 10
r1a in out 3k
r1b in out 3k
r1c in out 3k
r2 out 0 1k
.op
.end
";
    let vm = op_voltage(with_m, "out");
    let ve = op_voltage(explicit, "out");
    assert!(
        (vm - ve).abs() < 1e-12,
        "m=3 resistor must equal three parallel copies: {vm} vs {ve}"
    );
}

#[test]
fn diode_m_matches_explicit_parallel_copies() {
    let with_m = "\
* diode with m
v1 in 0 dc 5
r1 in a 1k
d1 a 0 dmod m=2
.model dmod D IS=1e-14 N=1.5
.op
.end
";
    let explicit = "\
* explicit parallel diodes
v1 in 0 dc 5
r1 in a 1k
d1a a 0 dmod
d1b a 0 dmod
.model dmod D IS=1e-14 N=1.5
.op
.end
";
    let vm = op_voltage(with_m, "a");
    let ve = op_voltage(explicit, "a");
    assert!(
        (vm - ve).abs() < 1e-9,
        "m=2 diode must equal two parallel copies: {vm} vs {ve}"
    );
}

#[test]
fn diode_area_scales_like_multiplicity_for_lumped_junction() {
    let with_area = "\
* diode with area
v1 in 0 dc 5
r1 in a 1k
d1 a 0 dmod 2
.model dmod D IS=1e-14 N=1.5
.op
.end
";
    let with_m = "\
* diode with m
v1 in 0 dc 5
r1 in a 1k
d1 a 0 dmod m=2
.model dmod D IS=1e-14 N=1.5
.op
.end
";
    let va = op_voltage(with_area, "a");
    let vm = op_voltage(with_m, "a");
    assert!(
        (va - vm).abs() < 1e-12,
        "positional AREA and m must scale the lumped junction identically: {va} vs {vm}"
    );
}

#[test]
fn diode_series_resistance_participates_in_dc_solution() {
    // 5V through 1k into a diode whose model declares RS=100. If RS is
    // honored, the junction sees less voltage than the RS=0 case and the
    // anode node (in front of RS) sits measurably higher.
    let rs_deck = "\
* diode with series resistance
v1 in 0 dc 5
r1 in a 1k
d1 a 0 dmod
.model dmod D IS=1e-14 N=1.5 RS=100
.op
.end
";
    let no_rs_deck = "\
* diode without series resistance
v1 in 0 dc 5
r1 in a 1k
d1 a 0 dmod
.model dmod D IS=1e-14 N=1.5
.op
.end
";
    let v_rs = op_voltage(rs_deck, "a");
    let v_no = op_voltage(no_rs_deck, "a");
    assert!(
        v_rs > v_no + 0.05,
        "RS=100 must raise the anode voltage above the RS=0 case: {v_rs} vs {v_no}"
    );

    // And the explicit-equivalent topology must match exactly.
    let explicit = "\
* diode with explicit external 100 ohm
v1 in 0 dc 5
r1 in a 1k
rser a aj 100
d1 aj 0 dmod
.model dmod D IS=1e-14 N=1.5
.op
.end
";
    let v_explicit = op_voltage(explicit, "a");
    assert!(
        (v_rs - v_explicit).abs() < 1e-9,
        "internal RS must equal an explicit series resistor: {v_rs} vs {v_explicit}"
    );
}

#[test]
fn subcircuit_x_multiplicity_scales_passives() {
    let deck = "\
* x-line multiplicity
v1 in 0 dc 1
x1 in 0 cell m=3
.subckt cell a b
r1 a b 3k
c1 a b 1p
.ends
.op
.end
";
    let g = resistor_conductance(deck, "x1.r1");
    assert!(
        (g - 3.0 / 3000.0).abs() < 1e-15,
        "X m=3 must triple the subcircuit resistor conductance, got {g}"
    );
    let c = capacitance_of(deck, "x1.c1");
    assert!(
        (c - 3e-12).abs() < 1e-24,
        "X m=3 must triple the subcircuit capacitance, got {c}"
    );
}

#[test]
fn nested_subcircuit_multiplicity_composes_multiplicatively() {
    let deck = "\
* nested multiplicity
v1 in 0 dc 1
x1 in 0 outer m=2
.subckt outer a b
x2 a b inner m=3
.ends
.subckt inner a b
r1 a b 6k
.ends
.op
.end
";
    let g = resistor_conductance(deck, "x1.x2.r1");
    assert!(
        (g - 6.0 / 6000.0).abs() < 1e-15,
        "nested m=2 * m=3 must compose to 6 parallel copies, got conductance {g}"
    );
}

#[test]
fn x_multiplicity_scales_current_sources() {
    let with_m = "\
* current source inside multiplied subckt
x1 out 0 cell m=4
r1 out 0 1k
.subckt cell a b
i1 b a dc 1m
.ends
.op
.end
";
    let v = op_voltage(with_m, "out");
    assert!(
        (v - 4.0).abs() < 1e-9,
        "m=4 over a 1mA source into 1k must give 4V, got {v}"
    );
}

#[test]
fn subcircuit_declaring_formal_m_keeps_parameter_semantics() {
    // The author owns the name M here: it sizes the resistor expression and
    // must NOT be hijacked as a parallel multiplier.
    let deck = "\
* formal m parameter
v1 in 0 dc 1
x1 in 0 cell m=5
.subckt cell a b m=1
r1 a b {1k*m}
.ends
.op
.end
";
    let g = resistor_conductance(deck, "x1.r1");
    assert!(
        (g - 1.0 / 5000.0).abs() < 1e-15,
        "formal M must behave as an ordinary parameter (R=5k), got conductance {g}"
    );
}

#[test]
fn capacitor_and_inductor_instance_m() {
    let deck = "\
* passive m
v1 a 0 dc 1
c1 a 0 1u m=4
l1 a 0 10m m=5
.op
.end
";
    let c = capacitance_of(deck, "c1");
    assert!(
        (c - 4e-6).abs() < 1e-18,
        "capacitor m=4 must quadruple capacitance, got {c}"
    );

    let netlist = Netlist::parse(deck).expect("deck parses");
    let circuit = Engine::default()
        .build_circuit(&netlist)
        .expect("circuit builds");
    let inductors = circuit.inductor_storage();
    let idx = inductors
        .names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("l1"))
        .expect("inductor stored");
    let l = inductors.inductances[idx];
    assert!(
        (l - 2e-3).abs() < 1e-15,
        "inductor m=5 must divide inductance to 2mH, got {l}"
    );
}
