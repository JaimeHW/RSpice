#![cfg(all(feature = "veriloga-builtins", rspice_veriloga_builtins_generated))]

use rspice_core::device::veriloga_generated::builtins;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn transient_node_series<'a>(
    result: &'a rspice_core::engine::TransientResult,
    node: &str,
) -> &'a [f64] {
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} not found in {:?}", result.node_names));
    &result.voltages[index]
}

#[test]
fn generated_builtin_resistor_runs_without_veriloga_directive() {
    if !fixture_builtins_available() {
        return;
    }

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
fn generated_builtin_capacitor_charges_in_transient() {
    if !fixture_builtins_available() {
        return;
    }

    let deck = r#"
v1 in 0 pulse(0 1 0 1n 1n 1 2)
r1 in out 1000
Xc out 0 generated_cap c=1e-6
.tran 10u 5m
.end
"#;
    let netlist = Netlist::parse(deck).expect("parse deck");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 5e-3, 10e-6)
        .expect("generated capacitor transient");

    let out = transient_node_series(&result, "out");
    let first = out
        .first()
        .copied()
        .expect("transient should record first sample");
    let final_value = out
        .last()
        .copied()
        .expect("transient should record final sample");

    assert!(
        first.abs() < 0.05,
        "generated capacitor should initially hold output near 0 V, got {first}"
    );
    assert!(
        (0.90..1.02).contains(&final_value),
        "generated capacitor RC output should charge near 1 V, got {final_value}"
    );
}

#[test]
fn generated_builtin_capacitor_participates_in_ac_reactive_stamp() {
    if !fixture_builtins_available() {
        return;
    }

    let deck = r#"
vin in 0 dc 0 ac 1
r1 in out 1000
Xc out 0 generated_cap c=1e-6
.ac lin 1 159.15494309189535 159.15494309189535
.end
"#;
    let netlist = Netlist::parse(deck).expect("parse deck");
    let result = Engine::new(SimulationConfig::default())
        .run_ac(&netlist, &[159.15494309189535])
        .expect("generated capacitor AC");
    let sample = result.first().expect("one AC sample");
    let out = sample
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out node");
    let magnitude = sample.voltages[out].norm();

    assert!(
        (magnitude - std::f64::consts::FRAC_1_SQRT_2).abs() < 5e-3,
        "generated capacitor low-pass magnitude should be near -3 dB, got {magnitude}"
    );
}

#[test]
fn generated_builtin_assignment_resistor_runs_without_veriloga_directive() {
    if !fixture_builtins_available() {
        return;
    }

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
    if !fixture_builtins_available() {
        return;
    }

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
    if !fixture_builtins_available() {
        return;
    }

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

fn fixture_builtins_available() -> bool {
    let required = ["simple_res", "assigned_res", "generated_cap"];
    let available = required.iter().all(|expected| {
        builtins::builtin_names()
            .iter()
            .any(|name| name.eq_ignore_ascii_case(expected))
    });
    if !available {
        eprintln!("fixture generated builtins not present; skipping fixture simulation test");
    }
    available
}
