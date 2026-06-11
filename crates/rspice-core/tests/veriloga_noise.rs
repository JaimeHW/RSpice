//! Verilog-A noise sources in .noise analysis.
//!
//! white_noise()/flicker_noise() terms in contributions are extracted at
//! compile time (with amplitude-squared scaling and guard gating), their
//! PSDs evaluate at the operating point, and the noise engine injects
//! them like built-in device sources. Pins are closed-form:
//!
//!   - a Verilog-A resistor declaring white_noise(4kT/r) must match the
//!     native resistor's thermal output noise exactly,
//!   - a bare flicker source into a noiseless load follows S·R²/f^exp,
//!   - scale factors square into the PSD, disabled guards remove it,
//!   - series voltage noise (potential contribution) reaches the output
//!     through the branch-equation row.
#![cfg(feature = "veriloga")]

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;
use std::io::Write;

const K_BOLTZMANN: f64 = 1.380649e-23;
const T_NOM: f64 = 300.15;

/// Write a .va model to a temp file and return its path (forward slashes
/// so the netlist parser keeps it intact)
fn write_model(name: &str, source: &str) -> String {
    let dir = std::env::temp_dir().join("rspice_va_noise_tests");
    std::fs::create_dir_all(&dir).expect("temp dir");
    let path = dir.join(name);
    let mut file = std::fs::File::create(&path).expect("create model file");
    file.write_all(source.as_bytes()).expect("write model");
    path.display().to_string().replace('\\', "/")
}

/// Output noise density (V²/Hz) at the named node for each frequency
fn output_noise(deck: &str, node: &str, frequencies: &[f64]) -> Vec<f64> {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let dc = engine.run_dc_op(&netlist).expect("dc op");
    let node_idx = dc
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} in {:?}", dc.node_names));
    engine
        .run_noise_ports(&netlist, node_idx, None, frequencies, T_NOM)
        .expect("noise analysis")
        .iter()
        .map(|r| r.output_noise_density)
        .collect()
}

const NOISY_RES: &str = r#"
`include "disciplines.vams"
module nres(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    analog begin
        I(p, n) <+ V(p, n) / r + white_noise(4.0 * 1.380649e-23 * $temperature / r, "thermal");
    end
endmodule
"#;

const QUIET_RES: &str = r#"
`include "disciplines.vams"
module qres(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;

#[test]
fn va_white_noise_matches_native_resistor_thermal() {
    let model = write_model("nres.va", NOISY_RES);
    let frequencies = [1.0, 1.0e3, 1.0e6];

    // Native resistor: onoise(out) = 4kTR exactly
    let native = output_noise(
        "* native thermal\n\
         i1 0 out dc 0\n\
         r1 out 0 1k\n\
         .end\n",
        "out",
        &frequencies,
    );

    let va = output_noise(
        &format!(
            "* va thermal\n\
             i1 0 out dc 0\n\
             X1 out 0 nres r=1k\n\
             .va \"{model}\" nres\n\
             .end\n"
        ),
        "out",
        &frequencies,
    );

    let analytic = 4.0 * K_BOLTZMANN * T_NOM * 1.0e3;
    for (i, &freq) in frequencies.iter().enumerate() {
        assert!(
            ((native[i] - analytic) / analytic).abs() < 1e-9,
            "native thermal at {freq} Hz: {:.6e} vs 4kTR {analytic:.6e}",
            native[i]
        );
        assert!(
            ((va[i] - native[i]) / native[i]).abs() < 1e-9,
            "VA white_noise at {freq} Hz: {:.6e} vs native {:.6e}",
            va[i],
            native[i]
        );
    }
}

const FLICKER_SOURCE: &str = r#"
`include "disciplines.vams"
module flsrc(p, n);
    inout p, n;
    electrical p, n;
    parameter real s1hz = 1.0e-18;
    parameter real ex = 1.0;
    analog I(p, n) <+ flicker_noise(s1hz, ex, "fl");
endmodule
"#;

#[test]
fn va_flicker_noise_follows_one_over_f() {
    let fl = write_model("flsrc.va", FLICKER_SOURCE);
    let quiet = write_model("qres.va", QUIET_RES);
    let frequencies = [1.0, 10.0, 100.0];

    // S/f into a noiseless 1k load: V² = S·R²/f
    let onoise = output_noise(
        &format!(
            "* flicker into quiet load\n\
             i1 0 out dc 0\n\
             X1 out 0 flsrc\n\
             X2 out 0 qres r=1k\n\
             .va \"{fl}\" flsrc\n\
             .va \"{quiet}\" qres\n\
             .end\n"
        ),
        "out",
        &frequencies,
    );
    for (i, &freq) in frequencies.iter().enumerate() {
        let analytic = 1.0e-18 * 1.0e6 / freq;
        assert!(
            ((onoise[i] - analytic) / analytic).abs() < 1e-9,
            "flicker at {freq} Hz: {:.6e} vs {analytic:.6e}",
            onoise[i]
        );
    }

    // Exponent 2 doubles the slope
    let onoise2 = output_noise(
        &format!(
            "* flicker exp 2\n\
             i1 0 out dc 0\n\
             X1 out 0 flsrc ex=2\n\
             X2 out 0 qres r=1k\n\
             .va \"{fl}\" flsrc\n\
             .va \"{quiet}\" qres\n\
             .end\n"
        ),
        "out",
        &frequencies,
    );
    for (i, &freq) in frequencies.iter().enumerate() {
        let analytic = 1.0e-18 * 1.0e6 / freq.powi(2);
        assert!(
            ((onoise2[i] - analytic) / analytic).abs() < 1e-9,
            "flicker^2 at {freq} Hz: {:.6e} vs {analytic:.6e}",
            onoise2[i]
        );
    }
}

const SCALED_GATED: &str = r#"
`include "disciplines.vams"
module sgn(p, n);
    inout p, n;
    electrical p, n;
    parameter real s = 1.0e-18;
    parameter real gain = 1.0;
    parameter integer en = 1;
    analog begin
        if (en)
            I(p, n) <+ gain * white_noise(s, "scaled");
    end
endmodule
"#;

#[test]
fn amplitude_squares_into_psd_and_guards_gate_it() {
    let sgn = write_model("sgn.va", SCALED_GATED);
    let quiet = write_model("qres2.va", QUIET_RES);
    let frequencies = [1.0e3];

    let deck = |params: &str| {
        format!(
            "* scaled noise\n\
             i1 0 out dc 0\n\
             X1 out 0 sgn {params}\n\
             X2 out 0 qres r=1k\n\
             .va \"{sgn}\" sgn\n\
             .va \"{quiet}\" qres\n\
             .end\n"
        )
    };

    // gain scales the PSD by gain²: 3² · 1e-18 · (1k)² = 9e-12
    let scaled = output_noise(&deck("gain=3"), "out", &frequencies);
    let analytic = 9.0 * 1.0e-18 * 1.0e6;
    assert!(
        ((scaled[0] - analytic) / analytic).abs() < 1e-9,
        "gain²·S·R²: {:.6e} vs {analytic:.6e}",
        scaled[0]
    );

    // Disabled guard removes the source entirely
    let gated = output_noise(&deck("en=0"), "out", &frequencies);
    assert!(
        gated[0].abs() < 1e-30,
        "disabled source must contribute nothing, got {:.3e}",
        gated[0]
    );
}

const SERIES_VNOISE: &str = r#"
`include "disciplines.vams"
module vn(p, n);
    inout p, n;
    electrical p, n;
    parameter real sv = 1.0e-12;
    analog V(p, n) <+ white_noise(sv, "emf");
endmodule
"#;

#[test]
fn series_voltage_noise_reaches_the_output() {
    let vn = write_model("vn.va", SERIES_VNOISE);
    let frequencies = [1.0e3, 1.0e5];

    // v1 (ideal, 0 V) - X1 series EMF - out loaded by 1k: the full EMF
    // appears at out, so onoise = sv
    let onoise = output_noise(
        &format!(
            "* series voltage noise\n\
             v1 a 0 dc 0\n\
             X1 a out vn\n\
             r1 out 0 1k\n\
             .va \"{vn}\" vn\n\
             .end\n"
        ),
        "out",
        &frequencies,
    );
    for (i, &freq) in frequencies.iter().enumerate() {
        // r1's own thermal noise is shorted by the ideal EMF loop:
        // the output impedance at `out` is 0, so only sv remains
        let analytic = 1.0e-12;
        assert!(
            ((onoise[i] - analytic) / analytic).abs() < 1e-9,
            "series EMF at {freq} Hz: {:.6e} vs {analytic:.6e}",
            onoise[i]
        );
    }
}
