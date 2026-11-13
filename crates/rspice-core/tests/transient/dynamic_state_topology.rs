//! DC-topology semantics for parser-lowered rational controlled sources.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn dc_engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

#[test]
fn stable_laplace_state_system_is_dc_determined() {
    let netlist = Netlist::parse(
        "\
* stable first-order dynamic source
vin in 0 dc 1
e1 out 0 laplace {v(in)} = {1/(1+s)}
rl out 0 1k
.end
",
    )
    .expect("stable LAPLACE deck parses");

    let op = dc_engine()
        .run_dc_op(&netlist)
        .expect("a nonzero denominator constant determines the state at DC");
    let out = op
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case("out"))
        .expect("output node is present");
    assert!((op.node_voltages[out] - 1.0).abs() < 1e-9);
}

#[test]
fn integrator_state_is_not_falsely_marked_dc_determined() {
    let netlist = Netlist::parse(
        "\
* ideal integrator has no unique DC state
vin in 0 dc 1
e1 out 0 laplace {v(in)} = {1/s}
rl out 0 1k
.end
",
    )
    .expect("integrator LAPLACE deck parses");

    let error = dc_engine()
        .run_dc_op(&netlist)
        .expect_err("a pole at the origin must retain the floating-state diagnostic")
        .to_string();
    assert!(
        error.contains("no DC path to ground"),
        "unexpected integrator error: {error}"
    );
    assert!(
        error.contains("E1.__X1"),
        "diagnostic must identify the generated integrator state: {error}"
    );
}

#[test]
fn hierarchical_second_order_state_system_retains_dc_certificate() {
    let netlist = Netlist::parse(
        "\
* hierarchical stable second-order dynamic source
.subckt filter in out
e1 out 0 laplace {v(in)} = {1/(s^2+s+1)}
.ends filter
vin in 0 dc 1
xfilter in out filter
rl out 0 1k
.end
",
    )
    .expect("hierarchical stable LAPLACE deck parses");

    let op = dc_engine()
        .run_dc_op(&netlist)
        .expect("flattening must preserve a nonzero denominator DC certificate");
    let out = op
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case("out"))
        .expect("output node is present");
    assert!((op.node_voltages[out] - 1.0).abs() < 1e-9);
}

#[test]
fn hierarchical_pole_at_origin_remains_fatal_after_flattening() {
    let netlist = Netlist::parse(
        "\
* hierarchical second-order system with an ideal integrator
.subckt filter in out
e1 out 0 laplace {v(in)} = {1/(s*(s+1))}
.ends filter
vin in 0 dc 1
xfilter in out filter
rl out 0 1k
.end
",
    )
    .expect("hierarchical integrator LAPLACE deck parses");

    let error = dc_engine()
        .run_dc_op(&netlist)
        .expect_err("flattening must not invent a DC certificate for a pole at the origin")
        .to_string();
    assert!(
        error.contains("no DC path to ground"),
        "unexpected hierarchical integrator error: {error}"
    );
    assert!(
        error.contains("XFILTER.E1.__X"),
        "diagnostic must retain hierarchical generated-state ownership: {error}"
    );
}
