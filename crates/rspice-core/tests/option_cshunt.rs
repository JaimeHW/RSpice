//! `.OPTIONS CSHUNT` — a real capacitor from every voltage node to ground.
//!
//! ngspice realizes this in `inppas4.c` by adding one capacitor instance per
//! voltage node, which is why it changes the waveform rather than merely
//! conditioning the matrix. A deck that asks for it and does not get it is
//! simulating a different circuit.

use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect, TransientResult};
use rspice_core::netlist::Netlist;

fn ngspice_engine() -> Engine {
    Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Ngspice,
        ..SimulationConfig::default()
    })
}

fn node_series<'a>(result: &'a TransientResult, node: &str) -> &'a [f64] {
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from {:?}", result.node_names));
    &result.voltages[index]
}

fn value_at(times: &[f64], values: &[f64], target: f64) -> f64 {
    let index = times
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            (*a - target)
                .abs()
                .partial_cmp(&(*b - target).abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(index, _)| index)
        .expect("samples");
    values[index]
}

/// One microfarad behind a one-kilohm series resistor is a one-millisecond
/// time constant, so the node reaches 1 - 1/e of the rail at 1 ms.
#[test]
fn cshunt_loads_every_node_with_a_real_capacitor() {
    let deck = "\
* cshunt damping
v1 in 0 dc 0 pulse(0 1 0 1n 1n 10m 20m)
r1 in out 1k
r2 out 0 1g
.options cshunt=1u
.tran 20u 4m
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = ngspice_engine()
        .run_tran(&netlist, 4.0e-3, 2.0e-5)
        .expect("transient solves");
    let out = node_series(&result, "out");

    let one_tau = value_at(&result.time, out, 1.0e-3);
    let three_tau = value_at(&result.time, out, 3.0e-3);
    assert!(
        (one_tau - 0.632).abs() < 0.02,
        "cshunt=1u behind 1k is a 1 ms time constant; expected ~0.632 V at 1 ms, got {one_tau}"
    );
    assert!(
        (three_tau - 0.950).abs() < 0.02,
        "expected ~0.950 V at three time constants, got {three_tau}"
    );
}

/// Without the option the same deck has no capacitance at all, so `out`
/// follows the source's own 1 ns edge. This is what makes the test above a
/// measurement of `CSHUNT` rather than of the source.
#[test]
fn without_cshunt_the_same_node_follows_the_source_edge() {
    let deck = "\
* no cshunt
v1 in 0 dc 0 pulse(0 1 0 1n 1n 10m 20m)
r1 in out 1k
r2 out 0 1g
.tran 20u 4m
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = ngspice_engine()
        .run_tran(&netlist, 4.0e-3, 2.0e-5)
        .expect("transient solves");
    let out = node_series(&result, "out");

    let early = value_at(&result.time, out, 1.0e-3);
    assert!(
        early > 0.99,
        "with no shunt capacitance the node is already at the rail, got {early}"
    );
}

/// The shunt capacitors are simulator-generated, not authored devices, so
/// they must not appear in the deck's own device inventory.
#[test]
fn cshunt_capacitors_are_not_reported_as_authored_devices() {
    let deck = "\
* cshunt provenance
v1 in 0 dc 1
r1 in out 1k
r2 out 0 1k
c1 out 0 1p
.options cshunt=100f
.op
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    // `build_circuit` reads the engine's own config, so the deck's `.OPTIONS`
    // have to be folded in first — the same order every analysis entry point
    // uses.
    let config = rspice_core::engine::resolve_simulation_config(
        &SimulationConfig {
            spice_dialect: SpiceDialect::Ngspice,
            ..SimulationConfig::default()
        },
        Some(&netlist.options),
        &Default::default(),
    );
    let circuit = Engine::new(config)
        .build_circuit(&netlist)
        .expect("circuit builds");
    assert_eq!(
        circuit.capacitor_storage().authored_len(),
        1,
        "only C1 was authored; the CSHUNT capacitors carry internal provenance"
    );
    assert!(
        circuit.capacitor_storage().names.len() > 1,
        "CSHUNT should have installed capacitors on IN and OUT"
    );
}
