//! Engine pins for `m=` instance multiplicity on Verilog-A devices.
//!
//! `m=` on an instance whose model does not declare an `m` parameter is
//! the standard parallel-multiplicity: the device stamps as m parallel
//! copies. A divider built from one m=4 resistor must match four
//! explicitly paralleled instances, and thermal noise must scale with the
//! effective conductance.
#![cfg(feature = "veriloga")]

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;
use std::io::Write;

fn write_model(name: &str, source: &str) -> String {
    let dir = std::env::temp_dir().join("rspice_va_mfactor_tests");
    std::fs::create_dir_all(&dir).expect("temp dir");
    let path = dir.join(name);
    let mut file = std::fs::File::create(&path).expect("create model file");
    file.write_all(source.as_bytes()).expect("write model");
    path.display().to_string().replace('\\', "/")
}

fn node_voltage(deck: &str, node: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let op = engine.run_dc_op(&netlist).expect("operating point solves");
    let idx = op
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} in {:?}", op.node_names));
    op.node_voltages[idx]
}

const RESISTOR: &str = r#"
`include "disciplines.vams"
module vres(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;

#[test]
fn m_equals_parallel_copies_in_a_divider() {
    let model = write_model("vres.va", RESISTOR);

    // Divider: 1k on top, m=4 x 4k on the bottom (= 1k) -> V(out) = 0.5
    let multiplied = node_voltage(
        &format!(
            "* m=4 divider\n\
             v1 in 0 dc 1.0\n\
             X1 in out vres r=1k\n\
             X2 out 0 vres r=4k m=4\n\
             .va \"{model}\" vres\n\
             .end\n"
        ),
        "out",
    );
    assert!(
        (multiplied - 0.5).abs() < 1e-12,
        "m=4 divider: got {multiplied}"
    );

    // Reference: four explicit parallel copies
    let explicit = node_voltage(
        &format!(
            "* explicit parallel\n\
             v1 in 0 dc 1.0\n\
             X1 in out vres r=1k\n\
             Xa out 0 vres r=4k\n\
             Xb out 0 vres r=4k\n\
             Xc out 0 vres r=4k\n\
             Xd out 0 vres r=4k\n\
             .va \"{model}\" vres\n\
             .end\n"
        ),
        "out",
    );
    assert!(
        (multiplied - explicit).abs() < 1e-12,
        "m=4 vs explicit: {multiplied} vs {explicit}"
    );
}

#[test]
fn model_declared_m_parameter_takes_precedence() {
    // A model that declares its own m handles the scaling itself; the
    // engine must not double-apply it
    let model = write_model(
        "own_m.va",
        r#"
`include "disciplines.vams"
module ownm(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    parameter real m = 1.0 from (0:inf);
    analog I(p, n) <+ m * V(p, n) / r;
endmodule
"#,
    );
    // Divider: 1k over (4k with model-handled m=4 -> 1k) -> 0.5
    let v = node_voltage(
        &format!(
            "* model-owned m\n\
             v1 in 0 dc 1.0\n\
             X1 in out ownm r=1k\n\
             X2 out 0 ownm r=4k m=4\n\
             .va \"{model}\" ownm\n\
             .end\n"
        ),
        "out",
    );
    assert!((v - 0.5).abs() < 1e-12, "model-owned m: got {v}");
}
