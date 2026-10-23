//! Operating-point probe for suites that check node voltages of one deck.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

/// DC operating-point voltage of `node` (case-insensitive) in `deck`.
pub fn op_voltage(deck: &str, node: &str) -> f64 {
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
