//! Engine pins for Verilog-A indirect contributions.
//!
//! `V(x): lhs == rhs` adds an unknown source on the target branch whose
//! value the Newton solve picks so the constraint holds — the ideal-opamp
//! idiom. Pins are closed-form: a follower copies its input, a divider
//! feedback sets a gain of 2, an implicit transcendental equation solves
//! to ln(2), a flow-target constraint regulates a node, and a disabled
//! constraint opens the branch.
#![cfg(feature = "veriloga")]

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;
use std::io::Write;

fn write_model(name: &str, source: &str) -> String {
    let dir = std::env::temp_dir().join("rspice_va_indirect_tests");
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

const OPAMP: &str = r#"
`include "disciplines.vams"
module opamp(out, inp, inn);
    inout out, inp, inn;
    electrical out, inp, inn;
    analog V(out): V(inp, inn) == 0.0;
endmodule
"#;

#[test]
fn ideal_opamp_follower_copies_its_input() {
    let model = write_model("opamp.va", OPAMP);
    let v = node_voltage(
        &format!(
            "* unity follower\n\
             v1 in 0 dc 1.5\n\
             X1 out in out opamp\n\
             rl out 0 1k\n\
             .va \"{model}\" opamp\n\
             .end\n"
        ),
        "out",
    );
    assert!((v - 1.5).abs() < 1e-9, "follower: got {v}");
}

#[test]
fn ideal_opamp_noninverting_gain_of_two() {
    let model = write_model("opamp.va", OPAMP);
    // Feedback divider R1=R2: V(inn) = V(out)/2, constraint forces
    // V(inn) = V(in) -> V(out) = 2*V(in)
    let v = node_voltage(
        &format!(
            "* non-inverting amplifier\n\
             v1 in 0 dc 0.75\n\
             X1 out in fb opamp\n\
             r1 out fb 10k\n\
             r2 fb 0 10k\n\
             .va \"{model}\" opamp\n\
             .end\n"
        ),
        "out",
    );
    assert!((v - 1.5).abs() < 1e-9, "gain of 2: got {v}");
}

const IMPLICIT_EXP: &str = r#"
`include "disciplines.vams"
module impexp(out);
    inout out;
    electrical out;
    analog V(out): exp(V(out)) == 2.0;
endmodule
"#;

#[test]
fn implicit_transcendental_equation_solves() {
    let model = write_model("impexp.va", IMPLICIT_EXP);
    // exp(v) = 2 -> v = ln 2 (Newton iterates on the constraint row)
    let v = node_voltage(
        &format!(
            "* implicit equation\n\
             X1 out impexp\n\
             rl out 0 1k\n\
             .va \"{model}\" impexp\n\
             .end\n"
        ),
        "out",
    );
    assert!(
        (v - std::f64::consts::LN_2).abs() < 1e-9,
        "ln(2): got {v}"
    );
}

const CURRENT_REGULATOR: &str = r#"
`include "disciplines.vams"
module ireg(p, n);
    inout p, n;
    electrical p, n;
    parameter real vtarget = 1.0;
    analog I(p, n): V(p, n) == vtarget;
endmodule
"#;

#[test]
fn flow_target_constraint_regulates_the_node() {
    let model = write_model("ireg.va", CURRENT_REGULATOR);
    // The unknown current through X1 makes V(out) = 2.0 across the load
    let v = node_voltage(
        &format!(
            "* current source regulating a node\n\
             X1 out 0 ireg vtarget=2.0\n\
             rl out 0 1k\n\
             .va \"{model}\" ireg\n\
             .end\n"
        ),
        "out",
    );
    assert!((v - 2.0).abs() < 1e-9, "regulated node: got {v}");
}

const GATED: &str = r#"
`include "disciplines.vams"
module gated(out);
    inout out;
    electrical out;
    parameter integer en = 1;
    analog begin
        if (en)
            V(out): V(out) == 3.0;
    end
endmodule
"#;

#[test]
fn disabled_constraint_opens_the_branch() {
    let model = write_model("gated.va", GATED);
    let deck = |params: &str| {
        format!(
            "* gated constraint\n\
             v1 ref 0 dc 0.25\n\
             r1 ref out 1k\n\
             X1 out gated {params}\n\
             .va \"{model}\" gated\n\
             .end\n"
        )
    };

    // Enabled: the constraint drives out to 3 V
    let v_on = node_voltage(&deck("en=1"), "out");
    assert!((v_on - 3.0).abs() < 1e-9, "enabled: got {v_on}");

    // Disabled: the branch opens; out follows the divider (no current
    // through r1, so out sits at ref)
    let v_off = node_voltage(&deck("en=0"), "out");
    assert!((v_off - 0.25).abs() < 1e-9, "disabled: got {v_off}");
}
