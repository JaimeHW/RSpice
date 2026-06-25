#![cfg(all(feature = "veriloga-builtins", rspice_veriloga_builtins_generated))]

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

#[test]
fn generated_builtin_resistor_runs_without_veriloga_directive() {
    let deck = r#"
v1 in 0 dc 1
X1 in out simple_res r=1000
X2 out 0 simple_res r=1000
.op
.end
"#;
    let netlist = Netlist::parse(deck).expect("parse deck");
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("dc op");

    let out = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out node");
    assert!(
        (result.node_voltages[out] - 0.5).abs() < 1e-10,
        "expected generated divider output near 0.5 V, got {}",
        result.node_voltages[out]
    );
}

#[test]
fn generated_builtin_assignment_resistor_runs_without_veriloga_directive() {
    let deck = r#"
v1 in 0 dc 1
X1 in out assigned_res r=1000
X2 out 0 assigned_res r=1000
.op
.end
"#;
    let netlist = Netlist::parse(deck).expect("parse deck");
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("dc op");

    let out = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out node");
    assert!(
        (result.node_voltages[out] - 0.5).abs() < 1e-10,
        "expected generated assignment-fed divider output near 0.5 V, got {}",
        result.node_voltages[out]
    );
}

#[test]
fn generated_builtin_rejects_unknown_parameters() {
    let deck = r#"
v1 in 0 dc 1
X1 in out simple_res not_a_parameter=1000
.op
.end
"#;
    let netlist = Netlist::parse(deck).expect("parse deck");
    let error = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect_err("unknown generated parameter should fail");

    assert!(
        error
            .to_string()
            .contains("unknown parameter 'not_a_parameter'"),
        "expected unknown generated parameter diagnostic, got {error}"
    );
}

#[test]
fn generated_builtin_rejects_out_of_range_parameters() {
    let deck = r#"
v1 in 0 dc 1
X1 in out simple_res r=0
.op
.end
"#;
    let netlist = Netlist::parse(deck).expect("parse deck");
    let error = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect_err("out-of-range generated parameter should fail");

    assert!(
        error.to_string().contains("parameter 'r' must be > 0.0"),
        "expected generated range diagnostic, got {error}"
    );
}
