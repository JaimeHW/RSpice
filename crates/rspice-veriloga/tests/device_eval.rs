//! Numerical verification of compiled device evaluation and MNA stamping.
//!
//! These tests check the companion-model math against hand-derived values:
//! the engine solves A*V = z, so a device must stamp G into all rows of its
//! KCL pair and -/+ Ieq = -/+(I - G*V) into the RHS.

use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::collections::HashMap;

fn compile(source: &str) -> rspice_veriloga::CompiledModel {
    VerilogACompiler::new(CompilerOptions::default())
        .compile(source)
        .expect("compilation failed")
}

/// Collect matrix and RHS stamps into maps for inspection
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
module res2(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 2.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;

#[test]
fn resistor_stamps_companion_form() {
    let model = compile(RESISTOR);
    // p -> circuit node 1, n -> ground
    let mut device = VerilogADevice::new("R1", model, &[1, 0]);

    // Node 1 at 4 V
    let (matrix, rhs) = collect_stamps(&mut device, &[4.0]);

    // G = 1/r = 0.5 at (0,0); rows/cols touching ground are dropped
    assert_eq!(matrix.len(), 1);
    assert!((matrix[&(0, 0)] - 0.5).abs() < 1e-12);

    // Linear element: Ieq = I - G*V = 2 - 0.5*4 = 0
    let total_rhs: f64 = rhs.values().map(|v| v.abs()).sum();
    assert!(total_rhs < 1e-12, "linear resistor must have zero Ieq");
}

#[test]
fn floating_resistor_stamps_all_four_positions() {
    let model = compile(RESISTOR);
    // Both terminals on non-ground circuit nodes 1 and 2
    let mut device = VerilogADevice::new("R1", model, &[1, 2]);

    let (matrix, _rhs) = collect_stamps(&mut device, &[3.0, 1.0]);

    // Full two-terminal conductance pattern
    assert!((matrix[&(0, 0)] - 0.5).abs() < 1e-12);
    assert!((matrix[&(0, 1)] + 0.5).abs() < 1e-12);
    assert!((matrix[&(1, 0)] + 0.5).abs() < 1e-12);
    assert!((matrix[&(1, 1)] - 0.5).abs() < 1e-12);
}

#[test]
fn nonlinear_companion_rhs_matches_analytic() {
    // I = 2*V^2 through an intermediate variable (exercises the
    // shadow-variable chain rule): dI/dV = 4V
    let model = compile(
        r#"
`include "disciplines.vams"
module sqlaw(p, n);
    inout p, n;
    electrical p, n;
    real gm;
    analog begin
        gm = 2.0 * V(p, n);
        I(p, n) <+ gm * V(p, n);
    end
endmodule
"#,
    );
    let mut device = VerilogADevice::new("Q1", model, &[1, 0]);

    let v = 3.0;
    let (matrix, rhs) = collect_stamps(&mut device, &[v]);

    // G = dI/dV = 4*V = 12
    assert!(
        (matrix[&(0, 0)] - 12.0).abs() < 1e-9,
        "chain rule through variables must reach the Jacobian, got {}",
        matrix[&(0, 0)]
    );

    // Ieq = I - G*V = 18 - 36 = -18; rhs[p] -= Ieq => +18
    assert!((rhs[&0] - 18.0).abs() < 1e-9, "got rhs {:?}", rhs);
}

#[test]
fn conditional_branches_select_correct_equation() {
    let model = compile(
        r#"
`include "disciplines.vams"
module piecewise(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        if (V(p, n) > 1.0)
            I(p, n) <+ 2.0 * V(p, n);
        else
            I(p, n) <+ V(p, n);
    end
endmodule
"#,
    );

    let mut device = VerilogADevice::new("X1", model.clone(), &[1, 0]);
    device.update_voltages(&[3.0]);
    let currents = device.evaluate();
    let total: f64 = currents.iter().sum();
    assert!((total - 6.0).abs() < 1e-12, "V=3 selects the 2*V branch");

    let mut device = VerilogADevice::new("X2", model, &[1, 0]);
    device.update_voltages(&[0.5]);
    let currents = device.evaluate();
    let total: f64 = currents.iter().sum();
    assert!((total - 0.5).abs() < 1e-12, "V=0.5 selects the V branch");
}

#[test]
fn internal_node_voltage_is_readable() {
    let model = compile(
        r#"
`include "disciplines.vams"
module divider(p, n);
    inout p, n;
    electrical p, n;
    electrical mid;
    parameter real r = 1.0 from (0:inf);
    analog begin
        I(p, mid) <+ (V(p) - V(mid)) / r;
        I(mid, n) <+ V(mid, n) / r;
    end
endmodule
"#,
    );
    assert_eq!(model.internal_nodes, 1);

    let mut device = VerilogADevice::new("D1", model, &[1, 0]);
    // Internal node mapped to circuit node 2
    device.set_internal_node_indices(&[2]);
    // node1 = 2 V, node2 (internal mid) = 0.5 V
    device.update_all_voltages(&[2.0, 0.5]);
    let currents = device.evaluate();

    // I(p,mid) = (2 - 0.5)/1 = 1.5 ; I(mid,n) = 0.5/1 = 0.5
    assert!((currents[0] - 1.5).abs() < 1e-12, "got {:?}", currents);
    assert!((currents[1] - 0.5).abs() < 1e-12, "got {:?}", currents);
}

#[test]
fn single_ended_access_references_global_ground() {
    // V(p) must read the potential of p against ground, NOT against
    // terminal 0 of the device
    let model = compile(
        r#"
`include "disciplines.vams"
module gprobe(a, b);
    inout a, b;
    electrical a, b;
    analog I(a, b) <+ V(b);
endmodule
"#,
    );
    // a -> node1, b -> node2
    let mut device = VerilogADevice::new("G1", model, &[1, 2]);
    device.update_voltages(&[5.0, 1.25]);
    let currents = device.evaluate();
    assert!(
        (currents[0] - 1.25).abs() < 1e-12,
        "V(b) must be 1.25 (potential vs ground), got {}",
        currents[0]
    );
}

#[test]
fn named_branch_evaluation() {
    let model = compile(
        r#"
`include "disciplines.vams"
module br_res(p, n);
    inout p, n;
    electrical p, n;
    branch (p, n) res;
    analog I(res) <+ V(res) / 4.0;
endmodule
"#,
    );
    let mut device = VerilogADevice::new("B1", model, &[1, 0]);
    device.update_voltages(&[2.0]);
    let currents = device.evaluate();
    assert!((currents[0] - 0.5).abs() < 1e-12);
}

#[test]
fn capacitor_ddt_backward_euler() {
    let model = compile(
        r#"
`include "disciplines.vams"
module cap(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1e-6 from (0:inf);
    analog I(p, n) <+ ddt(c * V(p, n));
endmodule
"#,
    );
    let mut device = VerilogADevice::new("C1", model, &[1, 0]);

    // DC: charge recorded, current is zero
    device.update_voltages(&[1.0]);
    let dc = device.evaluate();
    assert!(dc[0].abs() < 1e-18, "ddt must be 0 at DC, got {}", dc[0]);
    device.advance_state();

    // Transient step: V goes 1.0 -> 2.0 over dt=1us
    // i = C*dV/dt = 1e-6 * 1.0 / 1e-6 = 1.0
    device.set_analysis_type(2);
    device.set_timestep(1e-6);
    device.update_voltages(&[2.0]);
    let tr = device.evaluate();
    assert!(
        (tr[0] - 1.0).abs() < 1e-9,
        "backward-Euler capacitor current, got {}",
        tr[0]
    );

    // Jacobian must contain the companion conductance C/dt = 1.0
    let (matrix, _rhs) = collect_stamps(&mut device, &[2.0]);
    assert!(
        (matrix[&(0, 0)] - 1.0).abs() < 1e-9,
        "companion conductance C/dt, got {}",
        matrix[&(0, 0)]
    );
}

#[test]
fn ddx_computes_partial_derivative() {
    let model = compile(
        r#"
`include "disciplines.vams"
module ddxm(p, n);
    inout p, n;
    electrical p, n;
    real g;
    analog begin
        g = ddx(V(p) * V(p), V(p));
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
    );
    let mut device = VerilogADevice::new("DX1", model, &[1, 0]);
    // V(p) = 3 => ddx(V(p)^2, V(p)) = 2*V(p) = 6; I = 6 * 3 = 18
    device.update_voltages(&[3.0]);
    let currents = device.evaluate();
    assert!((currents[0] - 18.0).abs() < 1e-9, "got {}", currents[0]);
}

#[test]
fn voltage_contribution_is_a_clean_error() {
    let result = VerilogACompiler::new(CompilerOptions::default()).compile(
        r#"
`include "disciplines.vams"
module vsrc(p, n);
    inout p, n;
    electrical p, n;
    analog V(p, n) <+ 1.5;
endmodule
"#,
    );
    assert!(
        result.is_err(),
        "voltage contributions must fail loudly until branch unknowns exist"
    );
}

#[test]
fn user_function_device_evaluates() {
    let model = compile(
        r#"
`include "disciplines.vams"
module fres(p, n);
    inout p, n;
    electrical p, n;
    analog function real conduct;
        input v;
        begin
            if (v > 0.0)
                conduct = 2.0 * v;
            else
                conduct = v;
        end
    endfunction
    analog I(p, n) <+ conduct(V(p, n));
endmodule
"#,
    );
    let mut device = VerilogADevice::new("F1", model, &[1, 0]);
    device.update_voltages(&[2.0]);
    assert!((device.evaluate()[0] - 4.0).abs() < 1e-12);
    device.update_voltages(&[-1.0]);
    assert!((device.evaluate()[0] + 1.0).abs() < 1e-12);
}

#[test]
fn dependent_parameter_defaults_track_overrides() {
    let model = compile(
        r#"
`include "disciplines.vams"
module wres(p, n);
    inout p, n;
    electrical p, n;
    parameter real w = 1.0 from (0:inf);
    parameter real rs = 10.0 / w from (0:inf);
    analog I(p, n) <+ V(p, n) / rs;
endmodule
"#,
    );

    // Default w=1 => rs defaults to 10; I = V/10
    let mut device = VerilogADevice::new("W1", model.clone(), &[1, 0]);
    device.update_voltages(&[5.0]);
    assert!((device.evaluate()[0] - 0.5).abs() < 1e-12);

    // Override w=2 => rs default must recompute to 5; I = V/5
    let mut device = VerilogADevice::new("W2", model.clone(), &[1, 0]);
    device.set_parameter("w", 2.0);
    device.resolve_parameter_defaults();
    device.update_voltages(&[5.0]);
    assert!((device.evaluate()[0] - 1.0).abs() < 1e-12);

    // Explicit rs wins over its default regardless of w
    let mut device = VerilogADevice::new("W3", model, &[1, 0]);
    device.set_parameter("w", 2.0);
    device.set_parameter("rs", 50.0);
    device.resolve_parameter_defaults();
    device.update_voltages(&[5.0]);
    assert!((device.evaluate()[0] - 0.1).abs() < 1e-12);
}

#[test]
fn param_given_reflects_instance_overrides() {
    let model = compile(
        r#"
`include "disciplines.vams"
module pg(p, n);
    inout p, n;
    electrical p, n;
    parameter real rknob = 1.0 from (0:inf);
    real geff;
    analog begin
        if ($param_given(rknob))
            geff = 1.0 / rknob;
        else
            geff = 0.25;
        I(p, n) <+ geff * V(p, n);
    end
endmodule
"#,
    );

    // Not given: the model's fallback conductance applies
    let mut device = VerilogADevice::new("P1", model.clone(), &[1, 0]);
    device.update_voltages(&[2.0]);
    assert!((device.evaluate()[0] - 0.5).abs() < 1e-12);

    // Given: the explicit value applies even though it equals the default
    let mut device = VerilogADevice::new("P2", model, &[1, 0]);
    device.set_parameter("rknob", 1.0);
    device.update_voltages(&[2.0]);
    assert!((device.evaluate()[0] - 2.0).abs() < 1e-12);
}

#[test]
fn jacobian_matches_finite_difference_for_diode() {
    let model = compile(
        r#"
`include "disciplines.vams"
module diode(a, c);
    inout a, c;
    electrical a, c;
    parameter real is_sat = 1e-14 from (0:inf);
    analog I(a, c) <+ is_sat * (limexp(V(a, c) / $vt) - 1.0);
endmodule
"#,
    );

    let bias = 0.6;
    let delta = 1e-7;

    let mut device = VerilogADevice::new("D1", model.clone(), &[1, 0]);
    device.update_voltages(&[bias]);
    let i0 = device.evaluate()[0];
    let (matrix, _) = collect_stamps(&mut device, &[bias]);
    let g_analytic = matrix[&(0, 0)];

    let mut device2 = VerilogADevice::new("D2", model, &[1, 0]);
    device2.update_voltages(&[bias + delta]);
    let i1 = device2.evaluate()[0];

    let g_fd = (i1 - i0) / delta;
    let rel_err = ((g_analytic - g_fd) / g_fd).abs();
    assert!(
        rel_err < 1e-3,
        "analytic {} vs finite-difference {} (rel err {})",
        g_analytic,
        g_fd,
        rel_err
    );
}
