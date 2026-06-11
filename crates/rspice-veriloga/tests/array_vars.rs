//! Array variable support: element storage, compile-time-index resolution,
//! runtime-indexed reads/writes, shadow-array Jacobians, initializers, and
//! the rejection diagnostics.
//!
//! Numerical pins use the companion-form identities from device_eval.rs:
//! the stamp matrix carries G = dI/dV and the RHS carries
//! -/+ Ieq = -/+(I - G*V).

use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::collections::HashMap;

fn compile(source: &str) -> rspice_veriloga::CompiledModel {
    VerilogACompiler::new(CompilerOptions::default())
        .compile(source)
        .expect("compilation failed")
}

fn compile_err(source: &str) -> String {
    match VerilogACompiler::new(CompilerOptions::default()).compile(source) {
        Ok(_) => panic!("compilation must fail"),
        Err(err) => err.to_string(),
    }
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

/// Reconstruct the device current at the positive terminal (row 0) from
/// the companion stamps: I = sum(G*x) - rhs[0] (rhs[0] holds -Ieq)
fn terminal_current(device: &mut VerilogADevice, voltages: &[f64]) -> f64 {
    let (matrix, rhs) = collect_stamps(device, voltages);
    let mut current = -rhs.get(&0).copied().unwrap_or(0.0);
    for ((row, col), g) in &matrix {
        if *row == 0 {
            current += g * voltages.get(*col).copied().unwrap_or(0.0);
        }
    }
    current
}

#[test]
fn const_index_elements_resolve_under_unrolled_loop() {
    // Literal loop bounds unroll, so coef[i] resolves to element variables
    // at compile time; G = (1+2+3+4) mS = 10 mS
    let model = compile(
        r#"
`include "disciplines.vams"
module polysum(p, n);
    inout p, n;
    electrical p, n;
    real coef[0:3];
    integer i;
    real g;
    analog begin
        coef[0] = 1.0e-3;
        coef[1] = 2.0e-3;
        coef[2] = 3.0e-3;
        coef[3] = 4.0e-3;
        g = 0.0;
        for (i = 0; i < 4; i = i + 1)
            g = g + coef[i];
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
    );
    let mut device = VerilogADevice::new("A1", model, &[1, 0]);
    let (matrix, rhs) = collect_stamps(&mut device, &[2.0]);

    assert!((matrix[&(0, 0)] - 10.0e-3).abs() < 1e-15);
    // Linear element: zero Ieq
    let total_rhs: f64 = rhs.values().map(|v| v.abs()).sum();
    assert!(total_rhs < 1e-15);
    // Element introspection by name
    assert_eq!(device.variable("coef[2]"), Some(3.0e-3));
}

#[test]
fn assignment_pattern_initializer_fills_elements() {
    // LRM 2.4 '{...} assignment pattern as a declaration initializer
    let model = compile(
        r#"
`include "disciplines.vams"
module cinit(p, n);
    inout p, n;
    electrical p, n;
    real c[0:2] = '{0.5e-3, 1.5e-3, 2.0e-3};
    analog I(p, n) <+ (c[0] + c[1] + c[2]) * V(p, n);
endmodule
"#,
    );
    let mut device = VerilogADevice::new("A1", model, &[1, 0]);
    let (matrix, _) = collect_stamps(&mut device, &[1.0]);
    assert!((matrix[&(0, 0)] - 4.0e-3).abs() < 1e-15);
    assert_eq!(device.variable("c[1]"), Some(1.5e-3));
}

#[test]
fn runtime_loop_writes_and_reads_dynamic_indexes() {
    // Parameter-bounded loops stay runtime loops, so w[i] uses the
    // dynamic indexed write/read path. Default n=4: G = (1+2+3+4) mS.
    let source = r#"
`include "disciplines.vams"
module dynsum(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nseg = 4 from [1:8];
    real w[1:8];
    integer i;
    real total;
    analog begin
        for (i = 1; i <= nseg; i = i + 1)
            w[i] = 0.001 * i;
        total = 0.0;
        for (i = 1; i <= nseg; i = i + 1)
            total = total + w[i];
        I(p, n) <+ total * V(p, n);
    end
endmodule
"#;
    let model = compile(source);

    let mut device = VerilogADevice::new("A1", model.clone(), &[1, 0]);
    let (matrix, _) = collect_stamps(&mut device, &[1.0]);
    assert!((matrix[&(0, 0)] - 10.0e-3).abs() < 1e-12);
    assert_eq!(device.variable("w[3]"), Some(0.003));
    // Elements beyond the loop bound stay zero
    assert_eq!(device.variable("w[7]"), Some(0.0));

    // Per-instance override exercises the runtime bound: n=6 -> 21 mS
    let mut device = VerilogADevice::new("A2", model, &[1, 0]);
    assert!(device.set_parameter("nseg", 6.0));
    let (matrix, _) = collect_stamps(&mut device, &[1.0]);
    assert!((matrix[&(0, 0)] - 21.0e-3).abs() < 1e-12);
}

#[test]
fn voltage_dependent_elements_carry_shadow_jacobian() {
    // q[i] = V^2 * i mS flows through the dynamic indexed path, so the
    // Jacobian needs shadow arrays: I = 6m*V^2, dI/dV = 12m*V
    let model = compile(
        r#"
`include "disciplines.vams"
module varr(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nseg = 3 from [1:6];
    real q[1:6];
    integer i;
    real itot;
    analog begin
        for (i = 1; i <= nseg; i = i + 1)
            q[i] = V(p, n) * V(p, n) * 0.001 * i;
        itot = 0.0;
        for (i = 1; i <= nseg; i = i + 1)
            itot = itot + q[i];
        I(p, n) <+ itot;
    end
endmodule
"#,
    );
    let mut device = VerilogADevice::new("A1", model, &[1, 0]);

    let v = 0.5;
    let (matrix, rhs) = collect_stamps(&mut device, &[v]);

    // G = 12m * 0.5 = 6 mS
    let g_analytic = matrix[&(0, 0)];
    assert!(
        (g_analytic - 6.0e-3).abs() < 1e-12,
        "shadow-array Jacobian: got {g_analytic:.6e}"
    );
    // Ieq = I - G*V = 1.5m - 3m = -1.5m; rhs[0] -= Ieq
    assert!((rhs[&0] - 1.5e-3).abs() < 1e-12);

    // Cross-check the analytic conductance against finite differences
    let delta = 1e-7;
    let i0 = terminal_current(&mut device, &[v]);
    let i1 = terminal_current(&mut device, &[v + delta]);
    let g_fd = (i1 - i0) / delta;
    assert!(
        ((g_analytic - g_fd) / g_fd).abs() < 1e-5,
        "analytic {g_analytic:.6e} vs FD {g_fd:.6e}"
    );
}

#[test]
fn guarded_const_index_assignment_keeps_previous_value() {
    let source = r#"
`include "disciplines.vams"
module gsel(p, n);
    inout p, n;
    electrical p, n;
    parameter integer sel = 0;
    real w[0:1];
    analog begin
        w[0] = 1.0e-3;
        w[1] = 2.0e-3;
        if (sel)
            w[1] = 5.0e-3;
        I(p, n) <+ w[1] * V(p, n);
    end
endmodule
"#;
    let model = compile(source);

    let mut device = VerilogADevice::new("A1", model.clone(), &[1, 0]);
    let (matrix, _) = collect_stamps(&mut device, &[1.0]);
    assert!((matrix[&(0, 0)] - 2.0e-3).abs() < 1e-15);

    let mut device = VerilogADevice::new("A2", model, &[1, 0]);
    assert!(device.set_parameter("sel", 1.0));
    let (matrix, _) = collect_stamps(&mut device, &[1.0]);
    assert!((matrix[&(0, 0)] - 5.0e-3).abs() < 1e-15);
}

#[test]
fn guarded_dynamic_index_assignment_in_runtime_loop() {
    // The guard falls back to re-reading the same element, so the skipped
    // slot keeps its zero: n=4, skip=2 -> G = (1+3+4) mS = 8 mS
    let source = r#"
`include "disciplines.vams"
module gdyn(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nseg = 4 from [1:8];
    parameter integer skip = 2;
    real w[1:8];
    integer i;
    real total;
    analog begin
        for (i = 1; i <= nseg; i = i + 1) begin
            w[i] = 0.0;
            if (i != skip)
                w[i] = 0.001 * i;
        end
        total = 0.0;
        for (i = 1; i <= nseg; i = i + 1)
            total = total + w[i];
        I(p, n) <+ total * V(p, n);
    end
endmodule
"#;
    let model = compile(source);

    let mut device = VerilogADevice::new("A1", model.clone(), &[1, 0]);
    let (matrix, _) = collect_stamps(&mut device, &[1.0]);
    assert!((matrix[&(0, 0)] - 8.0e-3).abs() < 1e-12);
    assert_eq!(device.variable("w[2]"), Some(0.0));

    // skip=5 lies past the loop bound: every written element survives
    let mut device = VerilogADevice::new("A2", model, &[1, 0]);
    assert!(device.set_parameter("skip", 5.0));
    let (matrix, _) = collect_stamps(&mut device, &[1.0]);
    assert!((matrix[&(0, 0)] - 10.0e-3).abs() < 1e-12);
}

#[test]
fn localparam_sized_array_bounds_fold() {
    let model = compile(
        r#"
`include "disciplines.vams"
module lpsize(p, n);
    inout p, n;
    electrical p, n;
    localparam integer SIZE = 4;
    real w[0:SIZE-1];
    analog begin
        w[SIZE-1] = 2.0e-3;
        I(p, n) <+ w[3] * V(p, n);
    end
endmodule
"#,
    );
    let mut device = VerilogADevice::new("A1", model, &[1, 0]);
    let (matrix, _) = collect_stamps(&mut device, &[1.0]);
    assert!((matrix[&(0, 0)] - 2.0e-3).abs() < 1e-15);
}

#[test]
fn const_index_out_of_bounds_is_a_compile_error() {
    let err = compile_err(
        r#"
`include "disciplines.vams"
module oob(p, n);
    inout p, n;
    electrical p, n;
    real w[0:3];
    analog begin
        w[4] = 1.0;
        I(p, n) <+ w[0] * V(p, n);
    end
endmodule
"#,
    );
    assert!(err.contains("outside"), "got: {err}");
}

#[test]
fn const_index_out_of_bounds_read_is_a_compile_error() {
    let err = compile_err(
        r#"
`include "disciplines.vams"
module oobr(p, n);
    inout p, n;
    electrical p, n;
    real w[1:3];
    analog I(p, n) <+ w[0] * V(p, n);
endmodule
"#,
    );
    assert!(err.contains("outside"), "got: {err}");
}

#[test]
fn multidimensional_arrays_are_rejected() {
    let err = compile_err(
        r#"
`include "disciplines.vams"
module mdim(p, n);
    inout p, n;
    electrical p, n;
    real m[0:1][0:2];
    analog I(p, n) <+ 1.0e-3 * V(p, n);
endmodule
"#,
    );
    assert!(err.contains("multi-dimensional"), "got: {err}");
}

#[test]
fn parameter_dependent_bounds_are_rejected() {
    // Parameter-shaped storage would vary per instance
    let err = compile_err(
        r#"
`include "disciplines.vams"
module pdim(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nseg = 4;
    real w[0:nseg-1];
    analog I(p, n) <+ 1.0e-3 * V(p, n);
endmodule
"#,
    );
    assert!(err.contains("compile-time constants"), "got: {err}");
}

#[test]
fn descending_ranges_normalize() {
    // [3:0] declares the same four elements as [0:3]
    let model = compile(
        r#"
`include "disciplines.vams"
module desc(p, n);
    inout p, n;
    electrical p, n;
    real w[3:0];
    analog begin
        w[0] = 1.0e-3;
        w[3] = 3.0e-3;
        I(p, n) <+ (w[0] + w[3]) * V(p, n);
    end
endmodule
"#,
    );
    let mut device = VerilogADevice::new("A1", model, &[1, 0]);
    let (matrix, _) = collect_stamps(&mut device, &[1.0]);
    assert!((matrix[&(0, 0)] - 4.0e-3).abs() < 1e-15);
}
