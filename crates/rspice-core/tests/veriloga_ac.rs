//! AC small-signal pins for Verilog-A reactive elements.
//!
//! ddt() charges previously contributed nothing to AC analysis (only the
//! resistive Jacobian was stamped), so any Verilog-A capacitor or
//! inductor was invisible to .ac. These tests pin the reactive stamping
//! (jw * dQ/dx) against closed-form single-pole responses.
#![cfg(feature = "veriloga")]

use rspice_core::{Engine, Netlist};
use std::io::Write;
use std::path::PathBuf;

fn write_model(name: &str, source: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("rspice_ac_{}_{}.va", name, std::process::id()));
    let mut file = std::fs::File::create(&path).expect("create model file");
    file.write_all(source.as_bytes()).expect("write model");
    path
}

fn deck_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}

/// RC lowpass with a Verilog-A capacitor: H(s) = 1/(1 + sRC).
/// R = 1k, C = 1u -> fc = 1/(2 pi RC) ~= 159.155 Hz.
#[test]
fn veriloga_capacitor_rc_lowpass_ac() {
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
        "* veriloga RC lowpass\n\
         V1 in 0 DC 0 AC 1\n\
         R1 in out 1k\n\
         XC1 out 0 va_cap c=1u\n\
         .va \"{}\" va_cap\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let fc = 1.0 / (2.0 * std::f64::consts::PI * 1e3 * 1e-6);
    let freqs = [fc / 100.0, fc, 100.0 * fc];
    let results = Engine::default().run_ac(&netlist, &freqs).expect("ac runs");

    let out_idx = results[0]
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("out"))
        .expect("out node");

    // Far below fc: |H| ~ 1, phase ~ 0
    let h_low = results[0].voltages[out_idx];
    assert!(
        (h_low.norm() - 1.0).abs() < 2e-4,
        "low-frequency magnitude, got {}",
        h_low.norm()
    );

    // At fc: |H| = 1/sqrt(2), phase = -45 degrees
    let h_fc = results[1].voltages[out_idx];
    assert!(
        (h_fc.norm() - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6,
        "corner magnitude, got {}",
        h_fc.norm()
    );
    let phase_deg = h_fc.arg().to_degrees();
    assert!(
        (phase_deg + 45.0).abs() < 1e-3,
        "corner phase, got {phase_deg}"
    );

    // Far above fc: |H| ~ fc/f
    let h_high = results[2].voltages[out_idx];
    assert!(
        (h_high.norm() - 0.01).abs() < 1e-4,
        "high-frequency rolloff, got {}",
        h_high.norm()
    );

    let _ = std::fs::remove_file(model);
}

/// RL highpass with a Verilog-A inductor written in flux form
/// (V <+ ddt(L*I)): H(s) = sL/R / (1 + sL/R).
/// R = 100, L = 10m -> fc = R/(2 pi L) ~= 1591.55 Hz.
#[test]
fn veriloga_inductor_rl_highpass_ac() {
    let model = write_model(
        "ind",
        r#"
`include "disciplines.vams"
module va_ind(p, n);
    inout p, n;
    electrical p, n;
    parameter real l = 10e-3 from (0:inf);
    analog V(p, n) <+ ddt(l * I(p, n));
endmodule
"#,
    );

    let deck = format!(
        "* veriloga RL highpass\n\
         V1 in 0 DC 0 AC 1\n\
         R1 in out 100\n\
         XL1 out 0 va_ind l=10m\n\
         .va \"{}\" va_ind\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let fc = 100.0 / (2.0 * std::f64::consts::PI * 10e-3);
    let freqs = [fc / 100.0, fc, 100.0 * fc];
    let results = Engine::default().run_ac(&netlist, &freqs).expect("ac runs");

    let out_idx = results[0]
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("out"))
        .expect("out node");

    // Far below fc the inductor shorts the output: |H| ~ f/fc
    let h_low = results[0].voltages[out_idx];
    assert!(
        (h_low.norm() - 0.01).abs() < 1e-4,
        "low-frequency shorting, got {}",
        h_low.norm()
    );

    // At fc: |H| = 1/sqrt(2), phase = +45 degrees
    let h_fc = results[1].voltages[out_idx];
    assert!(
        (h_fc.norm() - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6,
        "corner magnitude, got {}",
        h_fc.norm()
    );
    let phase_deg = h_fc.arg().to_degrees();
    assert!(
        (phase_deg - 45.0).abs() < 1e-3,
        "corner phase, got {phase_deg}"
    );

    // Far above fc the inductor is open: |H| ~ 1
    let h_high = results[2].voltages[out_idx];
    assert!(
        (h_high.norm() - 1.0).abs() < 2e-4,
        "high-frequency passband, got {}",
        h_high.norm()
    );

    let _ = std::fs::remove_file(model);
}

#[test]
fn veriloga_ac_runtime_stamp_errors_are_simulation_errors_not_panics() {
    let model = write_model(
        "ac_oob",
        r#"
`include "disciplines.vams"
module va_ac_oob(p, n);
    inout p, n;
    electrical p, n;
    real w[1:4];
    integer i;
    analog begin
        i = analysis("ac") ? 5 : 1;
        w[i] = 1.0e-6;
        I(p, n) <+ w[i] * V(p, n);
    end
endmodule
"#,
    );

    let deck = format!(
        "* veriloga AC runtime diagnostic\n\
         V1 in 0 DC 0 AC 1\n\
         R1 in 0 1k\n\
         XBAD in 0 va_ac_oob\n\
         .va \"{}\" va_ac_oob\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = std::panic::catch_unwind(|| Engine::default().run_ac(&netlist, &[1.0e3]));

    let _ = std::fs::remove_file(model);

    let result = result.expect("Verilog-A AC runtime stamp errors must not panic");
    let err = result.expect_err("AC runtime stamp error must be reported to the caller");
    let text = err.to_string();
    assert!(
        text.contains("Verilog-A") && (text.contains("Array index 5") || text.contains("[1:4]")),
        "diagnostic should identify the Verilog-A AC array bounds error, got: {text}"
    );
}

/// Bias-dependent charge: Q = 0.5*k*V^2 gives C(V) = kV, so the corner
/// frequency moves with the DC operating point.
#[test]
fn veriloga_nonlinear_charge_linearizes_at_bias() {
    let model = write_model(
        "varactor",
        r#"
`include "disciplines.vams"
module va_varactor(p, n);
    inout p, n;
    electrical p, n;
    parameter real k = 1e-6 from (0:inf);
    real q;
    analog begin
        q = 0.5 * k * V(p, n) * V(p, n);
        I(p, n) <+ ddt(q);
    end
endmodule
"#,
    );

    // DC bias 2 V through a large feed resistor sets C = k*Vdc = 2 uF;
    // the AC divider R=1k, C=2u has fc = 1/(2 pi R C) ~= 79.58 Hz
    let deck = format!(
        "* veriloga varactor bias-dependent corner\n\
         V1 in 0 DC 2 AC 1\n\
         R1 in out 1k\n\
         XQ1 out 0 va_varactor k=1u\n\
         .va \"{}\" va_varactor\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    // DC operating point: no DC path to ground through the cap, so
    // V(out) settles to 2 V and C(Vdc) = 2 uF
    let c_bias = 1e-6 * 2.0;
    let fc = 1.0 / (2.0 * std::f64::consts::PI * 1e3 * c_bias);
    let results = Engine::default().run_ac(&netlist, &[fc]).expect("ac runs");

    let out_idx = results[0]
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("out"))
        .expect("out node");
    let h = results[0].voltages[out_idx];
    assert!(
        (h.norm() - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-3,
        "bias-dependent corner: |H(fc)| = {} (C must linearize at Vdc=2)",
        h.norm()
    );

    let _ = std::fs::remove_file(model);
}
