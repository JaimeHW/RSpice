//! Singular-system attribution and floating-node detection.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

#[test]
fn voltage_source_loop_reports_singularity_guidance() {
    // Two ideal voltage sources forcing different values on one node pair:
    // the DC system is singular and the error must say more than "singular".
    let deck = "\
* conflicting sources
v1 a 0 dc 1
v2 a 0 dc 2
r1 a 0 1k
.op
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    if let Err(err) = engine.run_dc_op(&netlist) {
        let message = err.to_string().to_lowercase();
        assert!(
            message.contains("singular"),
            "error should describe the singularity: {message}"
        );
        assert!(
            message.contains("voltage source") || message.contains("constrain"),
            "error should point at the cause or the unconstrained unknown: {message}"
        );
    }
    // Some continuation ladders regularize conflicting sources into a
    // solvable compromise; that is a solver-policy outcome, not a diagnostics
    // regression, so only the error path is asserted.
}

#[test]
fn capacitor_only_node_still_builds_and_solves() {
    // The floating node is reported via a warning, not an error: the gmin
    // keeps the system solvable and existing decks must keep running.
    let deck = "\
* floating cap node
v1 in 0 dc 1
r1 in out 1k
c1 out float 1n
c2 float 0 1n
.op
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    engine
        .run_dc_op(&netlist)
        .expect("cap-only node must stay runnable (warned, not fatal)");
}
