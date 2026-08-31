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

use rspice_core::analysis::NoiseContributionProbe;
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
fn noise_initial_step_initializes_the_committed_psd_for_every_frequency() {
    let model = write_model(
        "initial_step_noise.va",
        r#"
`include "disciplines.vams"
module va_noise_initial_step(p, n);
    inout p, n;
    electrical p, n;
    real density;
    analog begin
        @(initial_step("noise")) density = density + 4.0e-18;
        I(p, n) <+ white_noise(density, "initialized");
    end
endmodule
"#,
    );
    let frequencies = [
        10.0, 20.0, 50.0, 100.0, 200.0, 500.0, 1.0e3, 1.0e4, 1.0e5, 1.0e6,
    ];
    let observed = output_noise(
        &format!(
            "* noise initial-step lifecycle\n\
             R1 out 0 1k\n\
             X1 out 0 va_noise_initial_step\n\
             .va \"{model}\" va_noise_initial_step\n\
             .end\n"
        ),
        "out",
        &frequencies,
    );
    let expected = 4.0e-18 * 1.0e6 + 4.0 * K_BOLTZMANN * T_NOM * 1.0e3;
    for (frequency, density) in frequencies.into_iter().zip(observed) {
        assert!(
            ((density - expected) / expected).abs() < 1.0e-12,
            "noise initial_step state was not retained at {frequency} Hz: {density:.16e}"
        );
    }

    let port_deck = Netlist::parse(&format!(
        "* port-noise initial-step lifecycle\n\
         VPORT out 0 0\n\
         X1 out 0 va_noise_initial_step\n\
         .va \"{model}\" va_noise_initial_step\n\
         .end\n"
    ))
    .expect("port-noise lifecycle deck parses");
    let port_results = Engine::default()
        .run_port_noise_correlation(&port_deck, &["VPORT".to_owned()], &frequencies, T_NOM)
        .expect("port-noise lifecycle run");
    for result in port_results {
        let density = result.current_correlation[0][0];
        assert_eq!(density.im, 0.0);
        assert!(
            ((density.re - 4.0e-18) / 4.0e-18).abs() < 1.0e-12,
            "port-noise initial_step state was not retained at {} Hz: {density}",
            result.frequency
        );
    }
}

#[test]
fn noise_final_step_marks_only_the_global_final_frequency() {
    let model = write_model(
        "final_step_noise.va",
        r#"
`include "disciplines.vams"
module va_noise_final_step(p, n);
    inout p, n;
    electrical p, n;
    real count;
    analog begin
        @(initial_step("noise")) count = count + 1.0;
        @(final_step("noise")) count = count + 1.0;
        I(p, n) <+ count * 1.0e-3 * V(p, n);
        I(p, n) <+ ddt(count * 1.0e-9 * V(p, n));
        I(p, n) <+ white_noise(count * 4.0e-18, "lifecycle");
        if (count > 1.5)
            I(p, n) <+ white_noise(8.0e-18, "final_only");
    end
endmodule
"#,
    );
    let quiet = write_model("final_step_qres.va", QUIET_RES);
    let frequencies = [
        10.0, 20.0, 50.0, 100.0, 200.0, 500.0, 1.0e3, 2.0e3, 5.0e3, 1.0e4, 1.0e5, 5.0e5, 1.0e6,
    ];
    let ordinary_deck = Netlist::parse(&format!(
        "* ordinary noise final-step lifecycle\n\
         VREF in 0 DC 0 AC 1\n\
         XLINK in out qres r=1k\n\
         X1 out 0 va_noise_final_step\n\
         .va \"{quiet}\" qres\n\
         .va \"{model}\" va_noise_final_step\n\
         .end\n"
    ))
    .expect("ordinary final-step deck parses");

    let ordinary_run = |workers| {
        let mut config = SimulationConfig::default();
        config.resource_limits.max_parallel_workers = workers;
        Engine::new(config)
            .run_noise_named_with_input_source(
                &ordinary_deck,
                "out",
                None,
                "VREF",
                &frequencies,
                T_NOM,
            )
            .expect("ordinary final-step noise runs")
    };
    let serial = ordinary_run(1);
    let chunk_parallel = ordinary_run(4);
    assert_eq!(serial.len(), frequencies.len());
    assert_eq!(chunk_parallel.len(), frequencies.len());
    let final_only_probe =
        NoiseContributionProbe::parse("DNO(X1, final_only)").expect("final-only DNO parses");
    let expected_catalog = serial[0].contribution_catalog.clone();
    assert!(
        expected_catalog.iter().any(|identity| {
            identity.device.eq_ignore_ascii_case("X1")
                && identity
                    .mechanism
                    .as_deref()
                    .is_some_and(|mechanism| mechanism.eq_ignore_ascii_case("final_only"))
        }),
        "activation-independent catalog omits final-only Verilog-A mechanism: {expected_catalog:?}"
    );
    for (index, (serial_point, parallel_point)) in serial.iter().zip(&chunk_parallel).enumerate() {
        assert_eq!(
            serial_point.contribution_catalog, expected_catalog,
            "ordinary-noise catalog changed at serial index {index}"
        );
        assert_eq!(
            parallel_point.contribution_catalog, expected_catalog,
            "ordinary-noise catalog changed at chunk-parallel index {index}"
        );
        assert_eq!(
            serial_point.output_noise_density.to_bits(),
            parallel_point.output_noise_density.to_bits(),
            "ordinary noise differs between serial and chunk-parallel index {index}"
        );
        let count = if index + 1 == frequencies.len() {
            2.0
        } else {
            1.0
        };
        let final_only_psd = if index + 1 == frequencies.len() {
            8.0e-18
        } else {
            0.0
        };
        let real_y = (1.0 + count) * 1.0e-3;
        let imag_y = 2.0 * std::f64::consts::PI * frequencies[index] * count * 1.0e-9;
        let transfer_denominator = real_y * real_y + imag_y * imag_y;
        let expected = (count * 4.0e-18 + final_only_psd) / transfer_denominator;
        let expected_final_only = final_only_psd / transfer_denominator;
        let observed_final_only = serial_point
            .contribution(&final_only_probe)
            .expect("cataloged final-only contribution resolves");
        assert!(
            (observed_final_only - expected_final_only).abs()
                <= 1.0e-10 * expected_final_only.max(f64::MIN_POSITIVE),
            "final-only contribution at index {index} was not exact: actual={observed_final_only:.16e}, expected={expected_final_only:.16e}"
        );
        assert_eq!(
            parallel_point
                .contribution(&final_only_probe)
                .expect("parallel cataloged final-only contribution resolves")
                .to_bits(),
            observed_final_only.to_bits(),
            "final-only contribution differs between serial and chunk-parallel index {index}"
        );
        assert!(
            (serial_point.output_noise_density - expected).abs() <= 1.0e-10 * expected,
            "ordinary noise final_step count at index {index} was not exact: actual={:.16e}, expected={expected:.16e}",
            serial_point.output_noise_density
        );
    }

    let port_deck = Netlist::parse(&format!(
        "* SP port-noise final-step lifecycle\n\
         VPORT port 0 0\n\
         XLINK port out qres r=1k\n\
         X1 out 0 va_noise_final_step\n\
         .va \"{quiet}\" qres\n\
         .va \"{model}\" va_noise_final_step\n\
         .end\n"
    ))
    .expect("port final-step deck parses");
    let port_run = |workers| {
        let mut config = SimulationConfig::default();
        config.resource_limits.max_parallel_workers = workers;
        Engine::new(config)
            .run_port_noise_correlation(&port_deck, &["VPORT".to_owned()], &frequencies, T_NOM)
            .expect("port final-step noise runs")
    };
    let port_serial = port_run(1);
    let port_parallel_config = port_run(4);
    for (index, (serial_point, parallel_point)) in
        port_serial.iter().zip(&port_parallel_config).enumerate()
    {
        let serial_density = serial_point.current_correlation[0][0];
        let parallel_density = parallel_point.current_correlation[0][0];
        assert_eq!(serial_density.re.to_bits(), parallel_density.re.to_bits());
        assert_eq!(serial_density.im.to_bits(), parallel_density.im.to_bits());
        let count = if index + 1 == frequencies.len() {
            2.0
        } else {
            1.0
        };
        let final_only_psd = if index + 1 == frequencies.len() {
            8.0e-18
        } else {
            0.0
        };
        let real_y = (1.0 + count) * 1.0e-3;
        let imag_y = 2.0 * std::f64::consts::PI * frequencies[index] * count * 1.0e-9;
        let expected =
            (count * 4.0e-18 + final_only_psd) * 1.0e-6 / (real_y * real_y + imag_y * imag_y);
        assert!(
            (serial_density.re - expected).abs() <= 1.0e-10 * expected
                && serial_density.im.abs() <= 1.0e-30,
            "port noise final_step count at index {index} was not exact: actual={serial_density:?}, expected={expected:.16e}"
        );
    }

    let _ = std::fs::remove_file(model);
    let _ = std::fs::remove_file(quiet);
}

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

const TABLE_SOURCE: &str = r#"
`include "disciplines.vams"
module tblsrc(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ noise_table('{1.0, 1.0e-18, 100.0, 3.0e-18}, "tbl");
endmodule
"#;

const TABLE_LOG_SOURCE: &str = r#"
`include "disciplines.vams"
module tbllog(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ noise_table_log('{1.0, 1.0e-18, 100.0, 1.0e-16}, "tbl");
endmodule
"#;

#[test]
fn noise_table_interpolates_and_clamps() {
    let tbl = write_model("tblsrc.va", TABLE_SOURCE);
    let quiet = write_model("qres3.va", QUIET_RES);
    // Endpoints, the linear midpoint of [1, 100] at 50.5 Hz, and a
    // clamped point beyond the table
    let frequencies = [1.0, 50.5, 100.0, 1.0e4];
    let onoise = output_noise(
        &format!(
            "* table noise into quiet load\n\
             i1 0 out dc 0\n\
             X1 out 0 tblsrc\n\
             X2 out 0 qres r=1k\n\
             .va \"{tbl}\" tblsrc\n\
             .va \"{quiet}\" qres\n\
             .end\n"
        ),
        "out",
        &frequencies,
    );
    let r_sq = 1.0e6;
    let expected = [1.0e-18, 2.0e-18, 3.0e-18, 3.0e-18];
    for i in 0..frequencies.len() {
        let analytic = expected[i] * r_sq;
        assert!(
            ((onoise[i] - analytic) / analytic).abs() < 1e-9,
            "table at {} Hz: {:.6e} vs {analytic:.6e}",
            frequencies[i],
            onoise[i]
        );
    }
}

#[test]
fn noise_table_log_interpolates_in_log_coordinates() {
    let tbl = write_model("tbllog.va", TABLE_LOG_SOURCE);
    let quiet = write_model("qres4.va", QUIET_RES);
    // Two decades from 1e-18 to 1e-16: the log-log midpoint at 10 Hz is
    // exactly 1e-17
    let onoise = output_noise(
        &format!(
            "* log table noise\n\
             i1 0 out dc 0\n\
             X1 out 0 tbllog\n\
             X2 out 0 qres r=1k\n\
             .va \"{tbl}\" tbllog\n\
             .va \"{quiet}\" qres\n\
             .end\n"
        ),
        "out",
        &[10.0],
    );
    let analytic = 1.0e-17 * 1.0e6;
    assert!(
        ((onoise[0] - analytic) / analytic).abs() < 1e-9,
        "log-log midpoint: {:.6e} vs {analytic:.6e}",
        onoise[0]
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

#[test]
fn va_noise_runtime_errors_are_simulation_errors_not_panics() {
    let model = write_model(
        "noise_runtime_oob.va",
        r#"
`include "disciplines.vams"

module va_noise_oob(p, n);
    inout p, n;
    electrical p, n;
    real w[1:4];
    integer i;
    analog begin
        i = analysis("noise") ? 5 : 1;
        w[i] = 1.0e-18;
        I(p, n) <+ V(p, n) * 1.0e-3 + white_noise(w[i], "bad");
    end
endmodule
"#,
    );

    let deck = format!(
        "* veriloga noise runtime diagnostic\n\
         V1 in 0 DC 1 AC 1\n\
         XBAD in 0 va_noise_oob\n\
         .va \"{model}\" va_noise_oob\n\
         .end\n"
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let engine = Engine::new(SimulationConfig::default());
    let dc = engine.run_dc_op(&netlist).expect("dc op");
    let input = dc
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("in"))
        .expect("input node");
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        engine.run_noise_ports(&netlist, input, None, &[1.0e3], T_NOM)
    }));

    let _ = std::fs::remove_file(model);

    let result = result.expect("Verilog-A noise runtime errors must not panic");
    let err = result.expect_err("noise runtime error must be reported to the caller");
    let text = err.to_string();
    assert!(
        text.contains("Verilog-A") && (text.contains("Array index 5") || text.contains("[1:4]")),
        "diagnostic should identify the Verilog-A noise array bounds error, got: {text}"
    );
}

#[test]
fn noise_transfer_matrices_expose_noise_identity_without_changing_ac() {
    let model = write_model(
        "noise_analysis_identity.va",
        r#"
`include "disciplines.vams"

module va_noise_analysis_identity(p, n);
    inout p, n;
    electrical p, n;
    parameter real g_dc = 1.0e-3;
    parameter real g_ac = 1.0e-3;
    parameter real g_noise = 1.0e-3;
    parameter real c_ac = 0.0;
    parameter real c_noise = 0.0;
    real g;
    real c;
    analog begin
        g = analysis("noise") ? g_noise : (analysis("ac") ? g_ac : g_dc);
        c = analysis("noise") ? c_noise : c_ac;
        I(p, n) <+ g * V(p, n);
        I(p, n) <+ ddt(c * V(p, n));
    end
endmodule
"#,
    );
    let source_model = write_model(
        "noise_analysis_identity_source.va",
        r#"
`include "disciplines.vams"

module va_noise_analysis_identity_source(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ white_noise(4.0e-18, "source");
endmodule
"#,
    );

    let deck = format!(
        "* veriloga physical small-signal analysis identity\n\
         VPORT port 0 DC 0 AC 1\n\
         XLINK port out va_noise_analysis_identity g_dc=1m g_ac=1m g_noise=1m c_ac=0 c_noise=0\n\
         XCOND out 0 va_noise_analysis_identity g_dc=1m g_ac=4m g_noise=1m c_ac=0 c_noise=1u\n\
         XNOISE out 0 va_noise_analysis_identity_source\n\
         .va \"{model}\" va_noise_analysis_identity\n\
         .va \"{source_model}\" va_noise_analysis_identity_source\n\
         .end\n"
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let engine = Engine::new(SimulationConfig::default());
    let frequencies = [10.0, 1.0e3];

    // AC must retain its own identity: the 1 mS link drives the 4 mS
    // analysis("ac") shunt, so V(out)/V(port) = 1 / (1 + 4) = 0.2.
    let ac = engine.run_ac(&netlist, &frequencies).expect("AC runs");
    let output = ac[0]
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("output node");
    for point in &ac {
        let gain = point.voltages[output];
        assert!(
            (gain.re - 0.2).abs() <= 1.0e-12 && gain.im.abs() <= 1.0e-12,
            "AC must expose analysis(\"ac\"), got V(out)={gain:?}"
        );
    }

    // Noise must instead use the 1 mS and 1 uF analysis("noise") shunt.
    // The 4e-18 A²/Hz source sees 2 mS + jw*1 uF in total.
    let ordinary = engine
        .run_noise_named_with_input_source(&netlist, "out", None, "VPORT", &frequencies, T_NOM)
        .expect("ordinary noise runs");
    for point in &ordinary {
        let omega_c = 2.0 * std::f64::consts::PI * point.frequency * 1.0e-6;
        let denominator_norm_squared = (2.0e-3_f64).powi(2) + omega_c.powi(2);
        let expected = 4.0e-18 / denominator_norm_squared;
        assert!(
            (point.output_noise_density - expected).abs() <= 1.0e-9 * expected,
            "ordinary noise matrix must expose analysis(\"noise\"): actual={:.16e}, expected={expected:.16e}, contributions={:?}",
            point.output_noise_density,
            point.contributions
        );
    }

    // SP port noise uses the same physical noise identity. With VPORT
    // shorted, the Norton current is the source current scaled by the 1 mS
    // link over the same complex total admittance.
    let port = engine
        .run_port_noise_correlation(&netlist, &["VPORT".to_owned()], &frequencies, T_NOM)
        .expect("SP port noise runs");
    for point in &port {
        let density = point.current_correlation[0][0];
        let omega_c = 2.0 * std::f64::consts::PI * point.frequency * 1.0e-6;
        let denominator_norm_squared = (2.0e-3_f64).powi(2) + omega_c.powi(2);
        let expected = 4.0e-18 * (1.0e-3_f64).powi(2) / denominator_norm_squared;
        assert!(
            (density.re - expected).abs() <= 1.0e-9 * expected && density.im.abs() <= 1.0e-30,
            "SP port-noise matrix must expose analysis(\"noise\"): actual={density:?}, expected={expected:.16e}"
        );
    }

    let _ = std::fs::remove_file(model);
    let _ = std::fs::remove_file(source_model);
}

#[test]
fn va_noise_linearization_errors_are_simulation_errors_not_panics() {
    let model = write_model(
        "noise_linearization_oob.va",
        r#"
`include "disciplines.vams"

module va_noise_linearization_oob(p, n);
    inout p, n;
    electrical p, n;
    real w[1:4];
    integer armed;
    integer i;
    analog begin
        i = (analysis("noise") && armed != 0) ? 5 : 1;
        @(initial_step("noise")) armed = 1;
        w[i] = 1.0e-3;
        I(p, n) <+ w[i] * V(p, n) + white_noise(1.0e-18, "wn");
    end
endmodule
"#,
    );

    let deck = format!(
        "* veriloga noise-linearization diagnostic\n\
         V1 in 0 DC 1 AC 1\n\
         XBAD in 0 va_noise_linearization_oob\n\
         .va \"{model}\" va_noise_linearization_oob\n\
         .end\n"
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let engine = Engine::new(SimulationConfig::default());
    let dc = engine.run_dc_op(&netlist).expect("dc op");
    let input = dc
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("in"))
        .expect("input node");
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        engine.run_noise_ports(&netlist, input, None, &[1.0e3], T_NOM)
    }));

    let _ = std::fs::remove_file(model);

    let result = result.expect("Verilog-A noise-linearization errors must not panic");
    let err = result.expect_err("noise-linearization runtime error must be reported to the caller");
    let text = err.to_string();
    assert!(
        text.contains("Verilog-A") && (text.contains("Array index 5") || text.contains("[1:4]")),
        "diagnostic should identify the Verilog-A noise-linearization array bounds error, got: {text}"
    );
}
