//! Instance multiplicity ($mfactor / m=): the device stamps as m parallel
//! copies. Flow stamps (current, charge, current noise) scale by m;
//! potential rows, probed currents, internal voltages, and $mfactor reads
//! stay per-copy; series voltage noise averages (PSD / m).

use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::collections::HashMap;

fn compile(source: &str) -> rspice_veriloga::CompiledModel {
    VerilogACompiler::new(CompilerOptions::default())
        .compile(source)
        .expect("compilation failed")
}

fn collect_stamps(
    device: &mut VerilogADevice,
    voltages: &[f64],
) -> (HashMap<(usize, usize), f64>, HashMap<usize, f64>) {
    let mut matrix: HashMap<(usize, usize), f64> = HashMap::new();
    let mut rhs: HashMap<usize, f64> = HashMap::new();
    device.stamp(
        voltages,
        |row, col, value| *matrix.entry((row, col)).or_insert(0.0) += value,
        |node, value| *rhs.entry(node).or_insert(0.0) += value,
    );
    (matrix, rhs)
}

const RESISTOR: &str = r#"
`include "disciplines.vams"
module res(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;

#[test]
fn linear_resistor_scales_conductance_by_m() {
    let model = compile(RESISTOR);
    let mut device = VerilogADevice::new("R1", model, &[1, 0]);
    device.set_multiplicity(4.0);

    let (matrix, rhs) = collect_stamps(&mut device, &[2.0]);
    // G = m/r = 4 mS
    assert!((matrix[&(0, 0)] - 4.0e-3).abs() < 1e-15);
    // Linear: zero companion residual even scaled
    let total: f64 = rhs.values().map(|v| v.abs()).sum();
    assert!(total < 1e-15);
}

const SQUARE_LAW: &str = r#"
`include "disciplines.vams"
module sq(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ 2.0 * V(p, n) * V(p, n);
endmodule
"#;

#[test]
fn nonlinear_companion_scales_consistently() {
    let model = compile(SQUARE_LAW);
    let mut device = VerilogADevice::new("Q1", model, &[1, 0]);
    device.set_multiplicity(3.0);

    let v = 0.5;
    let (matrix, rhs) = collect_stamps(&mut device, &[v]);

    // Per-copy: I = 2v^2 = 0.5, dI/dV = 4v = 2; scaled by m = 3
    assert!((matrix[&(0, 0)] - 6.0).abs() < 1e-12);
    // Ieq = m*(I - G*v) = 3*(0.5 - 2*0.5) = -1.5; rhs[0] -= Ieq
    assert!((rhs[&0] - 1.5).abs() < 1e-12);
}

const VSOURCE: &str = r#"
`include "disciplines.vams"
module vsrc(p, n);
    inout p, n;
    electrical p, n;
    parameter real vdc = 1.0;
    analog V(p, n) <+ vdc;
endmodule
"#;

#[test]
fn potential_row_stays_per_copy_with_scaled_kcl_coupling() {
    let model = compile(VSOURCE);
    let mut device = VerilogADevice::new("V1", model, &[1, 2]);
    device.set_branch_current_indices(&[3]);
    device.set_multiplicity(5.0);

    let (matrix, rhs) = collect_stamps(&mut device, &[0.0, 0.0, 0.0]);

    // KCL columns carry m (m copies inject m * I_branch)
    assert!((matrix[&(0, 2)] - 5.0).abs() < 1e-12);
    assert!((matrix[&(1, 2)] + 5.0).abs() < 1e-12);
    // The source row itself is per-copy: V(p) - V(n) = vdc
    assert!((matrix[&(2, 0)] - 1.0).abs() < 1e-12);
    assert!((matrix[&(2, 1)] + 1.0).abs() < 1e-12);
    assert!((rhs[&2] - 1.0).abs() < 1e-12);
}

const MFACTOR_READ: &str = r#"
`include "disciplines.vams"
module mread(p, n);
    inout p, n;
    electrical p, n;
    real seen;
    analog begin
        seen = $mfactor;
        I(p, n) <+ V(p, n) * 1.0e-3;
    end
endmodule
"#;

#[test]
fn dollar_mfactor_reads_the_instance_multiplicity() {
    let model = compile(MFACTOR_READ);
    let mut device = VerilogADevice::new("M1", model, &[1, 0]);
    device.set_multiplicity(7.0);
    let _ = collect_stamps(&mut device, &[1.0]);
    assert_eq!(device.variable("seen"), Some(7.0));
}

const NOISY: &str = r#"
`include "disciplines.vams"
module noisy(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n) * 1.0e-3 + white_noise(1.0e-18, "wn");
endmodule
"#;

#[test]
fn current_noise_power_scales_by_m() {
    let model = compile(NOISY);
    let mut device = VerilogADevice::new("N1", model, &[1, 0]);
    device.set_multiplicity(4.0);
    let sources = device.noise_sources(&[0.3]);
    assert_eq!(sources.len(), 1);
    assert!((sources[0].psd - 4.0e-18).abs() < 1e-30);
}

const VNOISY: &str = r#"
`include "disciplines.vams"
module vnoisy(p, n);
    inout p, n;
    electrical p, n;
    analog V(p, n) <+ 1.0 + white_noise(1.0e-12, "emf");
endmodule
"#;

#[test]
fn series_voltage_noise_power_divides_by_m() {
    let model = compile(VNOISY);
    let mut device = VerilogADevice::new("N2", model, &[1, 2]);
    device.set_branch_current_indices(&[3]);
    device.set_multiplicity(4.0);
    let sources = device.noise_sources(&[0.0, 0.0, 0.0]);
    assert_eq!(sources.len(), 1);
    assert!((sources[0].psd - 0.25e-12).abs() < 1e-26);
}
