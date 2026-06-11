//! End-to-end Verilog-A device regression pins.
//!
//! Compiles small Verilog-A models through the full netlist -> engine path
//! and checks DC and transient results against closed-form solutions. These
//! pin the companion-form stamping (G into both KCL rows, Ieq on the RHS)
//! and the backward-Euler ddt() state pipeline.
#![cfg(feature = "veriloga")]

use rspice_core::{Engine, Netlist};
use std::io::Write;
use std::path::PathBuf;

fn write_model(name: &str, source: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("rspice_test_{}_{}.va", name, std::process::id()));
    let mut file = std::fs::File::create(&path).expect("create model file");
    file.write_all(source.as_bytes()).expect("write model");
    path
}

/// Netlist-safe path text (the deck parser treats backslashes as escapes)
fn deck_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}

fn node_series<'a>(names: &[String], voltages: &'a [Vec<f64>], want: &str) -> &'a [f64] {
    let idx = names
        .iter()
        .position(|n| n.eq_ignore_ascii_case(want))
        .unwrap_or_else(|| panic!("node {want} not found in {names:?}"));
    &voltages[idx]
}

/// DC voltage divider: native 1k on top, Verilog-A 2k resistor on the
/// bottom. v(out) = 1 V * 2/(1+2) = 2/3 V.
#[test]
fn veriloga_resistor_divider_dc() {
    let model = write_model(
        "res",
        r#"
`include "disciplines.vams"
module va_res(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
    );

    let deck = format!(
        "* veriloga divider\n\
         V1 in 0 1.0\n\
         R1 in out 1k\n\
         XR2 out 0 va_res r=2k\n\
         .va \"{}\" va_res\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 1e-4, 1e-5)
        .expect("transient run");

    let out = node_series(&result.node_names, &result.voltages, "out");
    let v_final = *out.last().expect("samples");
    let expected = 2.0 / 3.0;
    assert!(
        (v_final - expected).abs() < 1e-6,
        "divider with Verilog-A resistor: got {v_final}, want {expected}"
    );

    let _ = std::fs::remove_file(model);
}

/// RC charging: native 1k resistor, Verilog-A 1uF capacitor (ddt-based).
/// v(out) follows 1 - exp(-t/tau) with tau = 1 ms.
#[test]
fn veriloga_capacitor_rc_charge_matches_analytic() {
    let model = write_model(
        "cap",
        r#"
`include "disciplines.vams"
module va_cap(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1e-6 from (0:inf);
    analog I(p, n) <+ ddt(c * V(p, n));
endmodule
"#,
    );

    let deck = format!(
        "* veriloga RC charge\n\
         V1 in 0 PULSE(0 1 0 1u 1u 1 2)\n\
         R1 in out 1k\n\
         XC1 out 0 va_cap c=1u\n\
         .va \"{}\" va_cap\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 3e-3, 5e-6)
        .expect("transient run");

    let out = node_series(&result.node_names, &result.voltages, "out");
    let tau = 1e-3;

    // Compare against the analytic charging curve away from the pulse edge.
    let mut checked = 0usize;
    for (i, &t) in result.time.iter().enumerate() {
        if t < 5.0 * 5e-6 {
            continue; // skip the source ramp
        }
        let expected = 1.0 - (-(t - 1e-6) / tau).exp();
        let got = out[i];
        assert!(
            (got - expected).abs() < 0.02,
            "RC charge at t={t}: got {got}, want {expected}"
        );
        checked += 1;
    }
    assert!(checked > 50, "expected many compared samples, got {checked}");

    // Near-final value should be close to 1 V
    let v_final = *out.last().expect("samples");
    assert!(
        (v_final - 1.0).abs() < 0.06,
        "RC settles to the source voltage, got {v_final}"
    );

    let _ = std::fs::remove_file(model);
}

/// A Verilog-A voltage contribution drives a node through a branch-current
/// unknown: V(p,n) <+ level must force v(out) = level.
#[test]
fn veriloga_voltage_source_drives_node() {
    let model = write_model(
        "vsrc",
        r#"
`include "disciplines.vams"
module va_vsrc(p, n);
    inout p, n;
    electrical p, n;
    parameter real level = 1.0;
    analog V(p, n) <+ level;
endmodule
"#,
    );

    let deck = format!(
        "* veriloga voltage source\n\
         XV1 out 0 va_vsrc level=2.5\n\
         R1 out 0 1k\n\
         .va \"{}\" va_vsrc\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 1e-4, 1e-5)
        .expect("transient run");

    let out = node_series(&result.node_names, &result.voltages, "out");
    let v_final = *out.last().expect("samples");
    assert!(
        (v_final - 2.5).abs() < 1e-9,
        "Verilog-A voltage source must pin the node, got {v_final}"
    );

    let _ = std::fs::remove_file(model);
}

/// An impedance-form resistor (V <+ I*r, the BSIM4 substrate-network
/// pattern) divides correctly against a native resistor.
#[test]
fn veriloga_impedance_resistor_divider() {
    let model = write_model(
        "zres",
        r#"
`include "disciplines.vams"
module va_zres(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1.0 from (0:inf);
    analog V(p, n) <+ I(p, n) * r;
endmodule
"#,
    );

    // 1 V through native 1k on top, impedance-form 2k on the bottom:
    // v(out) = 2/3 V
    let deck = format!(
        "* veriloga impedance divider\n\
         V1 in 0 1.0\n\
         R1 in out 1k\n\
         XZ1 out 0 va_zres r=2k\n\
         .va \"{}\" va_zres\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 1e-4, 1e-5)
        .expect("transient run");

    let out = node_series(&result.node_names, &result.voltages, "out");
    let v_final = *out.last().expect("samples");
    let expected = 2.0 / 3.0;
    assert!(
        (v_final - expected).abs() < 1e-6,
        "impedance-form resistor divider: got {v_final}, want {expected}"
    );

    let _ = std::fs::remove_file(model);
}

/// Runtime (parameter-bounded) loops evaluate correctly through the
/// engine: conductance accumulated over nf iterations.
#[test]
fn veriloga_runtime_loop_conductance() {
    let model = write_model(
        "nfres",
        r#"
`include "disciplines.vams"
module va_nfres(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nf = 1 from [1:inf);
    integer i;
    real g;
    analog begin
        g = 0.0;
        for (i = 0; i < nf; i = i + 1)
            g = g + 1.0e-3;
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
    );

    // nf=4 fingers of 1mS each = 4mS = 250 ohm against 1k:
    // v(out) = 1 * 250/(1000+250) = 0.2 V
    let deck = format!(
        "* veriloga runtime loop\n\
         V1 in 0 1.0\n\
         R1 in out 1k\n\
         XN1 out 0 va_nfres nf=4\n\
         .va \"{}\" va_nfres\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 1e-4, 1e-5)
        .expect("transient run");

    let out = node_series(&result.node_names, &result.voltages, "out");
    let v_final = *out.last().expect("samples");
    assert!(
        (v_final - 0.2).abs() < 1e-9,
        "nf=4 runtime loop conductance: got {v_final}, want 0.2"
    );

    let _ = std::fs::remove_file(model);
}

/// Nonlinear Verilog-A conductance in a feedback divider converges via
/// Newton with the companion stamps: I = g*V^2 against a series resistor.
#[test]
fn veriloga_square_law_converges() {
    let model = write_model(
        "sql",
        r#"
`include "disciplines.vams"
module va_sql(p, n);
    inout p, n;
    electrical p, n;
    parameter real g = 1.0 from (0:inf);
    real vd;
    analog begin
        vd = V(p, n);
        I(p, n) <+ g * vd * vd;
    end
endmodule
"#,
    );

    // 1 V source, 1k resistor, square-law device to ground:
    // KCL at out: (1 - v)/1000 = 1e-3 * v^2  =>  v^2 + v - 1 = 0
    // v = (sqrt(5) - 1)/2 ~= 0.61803
    let deck = format!(
        "* veriloga square law\n\
         V1 in 0 1.0\n\
         R1 in out 1k\n\
         XQ1 out 0 va_sql g=1m\n\
         .va \"{}\" va_sql\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 1e-4, 1e-5)
        .expect("transient run");

    let out = node_series(&result.node_names, &result.voltages, "out");
    let v_final = *out.last().expect("samples");
    let expected = (5.0_f64.sqrt() - 1.0) / 2.0;
    assert!(
        (v_final - expected).abs() < 1e-4,
        "square-law operating point: got {v_final}, want {expected}"
    );

    let _ = std::fs::remove_file(model);
}
