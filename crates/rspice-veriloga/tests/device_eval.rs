//! Numerical verification of compiled device evaluation and MNA stamping.
//!
//! These tests check the companion-model math against hand-derived values:
//! the engine solves A*V = z, so a device must stamp G into all rows of its
//! KCL pair and -/+ Ieq = -/+(I - G*V) into the RHS.

#[cfg(not(feature = "native"))]
use rspice_veriloga::codegen::{BytecodeProgram, Instruction};
use rspice_veriloga::device::VerilogADevice;
use std::collections::HashMap;

mod support;

use support::DeviceFixture;

fn compile(source: &str) -> DeviceFixture {
    DeviceFixture::compile(source)
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
    let mut device = model.device("R1", &[1, 0]);

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
    let mut device = model.device("R1", &[1, 2]);

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
    let mut device = model.device("Q1", &[1, 0]);

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

    let mut device = model.device("X1", &[1, 0]);
    device.update_voltages(&[3.0]);
    let currents = device.evaluate();
    let total: f64 = currents.iter().sum();
    assert!((total - 6.0).abs() < 1e-12, "V=3 selects the 2*V branch");

    let mut device = model.device("X2", &[1, 0]);
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

    let mut device = model.device("D1", &[1, 0]);
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
    let mut device = model.device("G1", &[1, 2]);
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
    let mut device = model.device("B1", &[1, 0]);
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
    let mut device = model.device("C1", &[1, 0]);

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
    let mut device = model.device("DX1", &[1, 0]);
    // V(p) = 3 => ddx(V(p)^2, V(p)) = 2*V(p) = 6; I = 6 * 3 = 18
    device.update_voltages(&[3.0]);
    let currents = device.evaluate();
    assert!((currents[0] - 18.0).abs() < 1e-9, "got {}", currents[0]);
}

#[test]
fn ddx_accepts_named_branch_probe() {
    let model = compile(
        r#"
`include "disciplines.vams"
module ddxbr(p, n);
    inout p, n;
    electrical p, n;
    branch (p, n) sense;
    real g, h;
    analog begin
        g = ddx(V(<sense>) * V(<sense>), V(<sense>));
        h = ddx(V(sense) * V(sense), V(sense));
        I(p, n) <+ 0.5 * (g + h) * V(p, n);
    end
endmodule
"#,
    );
    let mut device = model.device("DXB1", &[1, 0]);
    device.update_voltages(&[3.0]);
    let currents = device.evaluate();

    assert!((device.variable("g").unwrap() - 6.0).abs() < 1e-12);
    assert!((device.variable("h").unwrap() - 6.0).abs() < 1e-12);
    assert!((currents[0] - 18.0).abs() < 1e-9, "got {}", currents[0]);
}

#[test]
fn voltage_contribution_stamps_branch_unknown() {
    let model = compile(
        r#"
`include "disciplines.vams"
module vsrc(p, n);
    inout p, n;
    electrical p, n;
    parameter real level = 1.5;
    analog V(p, n) <+ level;
endmodule
"#,
    );
    assert_eq!(model.branch_sources.len(), 1, "one branch unknown");

    // p -> node1 (row 0), n -> node2 (row 1), branch unknown -> node3 (row 2)
    let mut device = model.device("V1", &[1, 2]);
    device.set_branch_current_indices(&[3]);

    let (matrix, rhs) = collect_stamps(&mut device, &[0.0, 0.0, 0.0]);

    // Structural coupling: KCL rows gain the branch column, the branch row
    // reads the node potentials
    assert!((matrix[&(0, 2)] - 1.0).abs() < 1e-12);
    assert!((matrix[&(1, 2)] + 1.0).abs() < 1e-12);
    assert!((matrix[&(2, 0)] - 1.0).abs() < 1e-12);
    assert!((matrix[&(2, 1)] + 1.0).abs() < 1e-12);

    // Branch row RHS carries the source value: V(p) - V(n) = 1.5
    assert!((rhs[&2] - 1.5).abs() < 1e-12, "rhs: {rhs:?}");
}

#[test]
fn impedance_form_resistor_via_voltage_contribution() {
    // V(p,n) <+ I(p,n) * r is a resistor written in impedance form;
    // the branch row must read V(p) - V(n) - r*i_br = 0
    let model = compile(
        r#"
`include "disciplines.vams"
module zres(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 2000.0 from (0:inf);
    analog V(p, n) <+ I(p, n) * r;
endmodule
"#,
    );
    assert_eq!(model.branch_sources.len(), 1);

    let mut device = model.device("Z1", &[1, 2]);
    device.set_branch_current_indices(&[3]);

    // Branch current solution value 1 mA
    let (matrix, _rhs) = collect_stamps(&mut device, &[1.0, 0.5, 1e-3]);

    // Constitutive row: dE/di = r stamps -r at (branch, branch)
    assert!(
        (matrix[&(2, 2)] + 2000.0).abs() < 1e-9,
        "got {:?}",
        matrix.get(&(2, 2))
    );
}

#[test]
fn try_stamp_reports_missing_branch_current_solution_slot() {
    let model = compile(
        r#"
`include "disciplines.vams"
module zres(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 2000.0 from (0:inf);
    analog V(p, n) <+ I(p, n) * r;
endmodule
"#,
    );

    let mut device = model.device("Z1", &[1, 2]);
    device.set_branch_current_indices(&[3]);

    let err = device
        .try_stamp(&[1.0, 0.5], |_, _, _| {}, |_, _| {})
        .expect_err("missing branch-current solution slot must be reported");

    assert!(
        err.to_string()
            .contains("missing branch-current solution slot"),
        "unexpected error: {err}"
    );
}

#[test]
fn mode_disabled_voltage_contribution_leaves_branch_open() {
    // A potential contribution under a parameter-only guard must leave
    // the branch OPEN when disabled, not short it to zero volts
    let model = compile(
        r#"
`include "disciplines.vams"
module modal(p, n);
    inout p, n;
    electrical p, n;
    parameter integer shorted = 0;
    analog begin
        if (shorted > 0)
            V(p, n) <+ 0.0;
    end
endmodule
"#,
    );
    assert_eq!(model.branch_sources.len(), 1);

    // Disabled (default): branch row pinned to zero current, no coupling
    let mut device = model.device("M1", &[1, 2]);
    device.set_branch_current_indices(&[3]);
    let (matrix, _) = collect_stamps(&mut device, &[1.0, 0.0, 0.0]);
    assert!((matrix[&(2, 2)] - 1.0).abs() < 1e-12, "identity pin");
    assert!(!matrix.contains_key(&(0, 2)), "no KCL coupling when open");

    // Enabled: structural short V(p)-V(n)=0
    let mut device = model.device("M2", &[1, 2]);
    device.set_branch_current_indices(&[3]);
    device.set_parameter("shorted", 1.0);
    device.resolve_parameter_defaults();
    let (matrix, _) = collect_stamps(&mut device, &[1.0, 0.0, 0.0]);
    assert!((matrix[&(0, 2)] - 1.0).abs() < 1e-12);
    assert!((matrix[&(2, 0)] - 1.0).abs() < 1e-12);
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
    let mut device = model.device("F1", &[1, 0]);
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
    let mut device = model.device("W1", &[1, 0]);
    device.update_voltages(&[5.0]);
    assert!((device.evaluate()[0] - 0.5).abs() < 1e-12);

    // Override w=2 => rs default must recompute to 5; I = V/5
    let mut device = model.device("W2", &[1, 0]);
    device.set_parameter("w", 2.0);
    device.resolve_parameter_defaults();
    device.update_voltages(&[5.0]);
    assert!((device.evaluate()[0] - 1.0).abs() < 1e-12);

    // Explicit rs wins over its default regardless of w
    let mut device = model.device("W3", &[1, 0]);
    device.set_parameter("w", 2.0);
    device.set_parameter("rs", 50.0);
    device.resolve_parameter_defaults();
    device.update_voltages(&[5.0]);
    assert!((device.evaluate()[0] - 0.1).abs() < 1e-12);
}

#[cfg(not(feature = "native"))]
#[test]
fn try_new_reports_dependent_parameter_default_runtime_errors() {
    let mut model = compile(
        r#"
`include "disciplines.vams"
module bad_default(p, n);
    inout p, n;
    electrical p, n;
    parameter real w = 1.0 from (0:inf);
    parameter real r = 10.0 / w from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
    );
    model.parameters[1].default_program = Some(BytecodeProgram {
        instructions: vec![Instruction::PushParam(99)],
    });

    let err = model
        .try_device("BD1", &[1, 0])
        .expect_err("checked construction must report invalid dependent parameter defaults");
    let text = err.to_string();
    assert!(
        text.contains("parameter") || text.contains("Invalid instruction"),
        "diagnostic should identify the dependent default failure, got: {text}"
    );
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
    let mut device = model.device("P1", &[1, 0]);
    device.update_voltages(&[2.0]);
    assert!((device.evaluate()[0] - 0.5).abs() < 1e-12);

    // Given: the explicit value applies even though it equals the default
    let mut device = model.device("P2", &[1, 0]);
    device.set_parameter("rknob", 1.0);
    device.update_voltages(&[2.0]);
    assert!((device.evaluate()[0] - 2.0).abs() < 1e-12);
}

#[test]
fn port_connected_reflects_omitted_trailing_terminal() {
    let model = compile(
        r#"
`include "disciplines.vams"

module optional_port_probe(p, n, opt);
    inout p, n, opt;
    electrical p, n, opt;
    analog I(p, n) <+ ($port_connected(opt) ? 10.0 : 1.0) * V(p, n);
endmodule
"#,
    );

    let mut omitted = model.device("X1", &[1, 0]);
    omitted.update_voltages(&[2.0]);
    assert!((omitted.evaluate()[0] - 2.0).abs() < 1e-12);

    let mut grounded = model.device("X2", &[1, 0, 0]);
    grounded.update_voltages(&[2.0]);
    assert!((grounded.evaluate()[0] - 20.0).abs() < 1e-12);
}

#[test]
fn runtime_array_index_errors_do_not_evaluate_as_zero() {
    let model = compile(
        r#"
`include "disciplines.vams"

module runtime_oob(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nseg = 5;
    real w[1:4];
    integer i;
    real total;
    analog begin
        total = 0.0;
        for (i = 1; i <= nseg; i = i + 1) begin
            w[i] = 0.001 * i;
            total = total + w[i];
        end
        I(p, n) <+ total * V(p, n);
    end
endmodule
"#,
    );

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut device = model.device("X1", &[1, 0]);
        device.update_voltages(&[1.0]);
        let _ = device.evaluate();
    }));

    assert!(
        result.is_err(),
        "runtime array bounds errors must not be converted into a numeric current"
    );
}

#[test]
fn try_stamp_reports_runtime_array_index_errors() {
    let model = compile(
        r#"
`include "disciplines.vams"

module stamp_runtime_oob(p, n);
    inout p, n;
    electrical p, n;
    real w[1:4];
    integer i;
    analog begin
        i = (V(p, n) > 0.5) ? 5 : 1;
        w[i] = 1.0e-3;
        I(p, n) <+ w[i] * V(p, n);
    end
endmodule
"#,
    );

    let mut device = model.device("X1", &[1, 0]);
    let err = device
        .try_stamp(&[1.0], |_, _, _| {}, |_, _| {})
        .expect_err("checked stamping must report runtime array bounds errors");
    let text = err.to_string();
    assert!(
        text.contains("Array index 5") || text.contains("[1:4]"),
        "diagnostic should identify the runtime array bounds error, got: {text}"
    );
}

#[test]
fn try_compute_jacobian_reports_runtime_array_index_errors() {
    let model = compile(
        r#"
`include "disciplines.vams"

module jac_runtime_oob(p, n);
    inout p, n;
    electrical p, n;
    real w[1:4];
    integer i;
    analog begin
        i = (V(p, n) > 0.5) ? 5 : 1;
        w[i] = 1.0e-3;
        I(p, n) <+ w[i] * V(p, n);
    end
endmodule
"#,
    );

    let mut device = model.device("X1", &[1, 0]);
    device.update_voltages(&[1.0]);
    let err = device
        .try_compute_jacobian()
        .expect_err("checked Jacobian evaluation must report runtime array bounds errors");
    let text = err.to_string();
    assert!(
        text.contains("Array index 5") || text.contains("[1:4]"),
        "diagnostic should identify the runtime array bounds error, got: {text}"
    );
}

#[test]
fn try_stamp_reactive_reports_runtime_array_index_errors() {
    let model = compile(
        r#"
`include "disciplines.vams"

module reactive_runtime_oob(p, n);
    inout p, n;
    electrical p, n;
    real w[1:4];
    integer i;
    analog begin
        i = (V(p, n) > 0.5) ? 5 : 1;
        w[i] = 1.0e-6;
        I(p, n) <+ ddt(w[i] * V(p, n));
    end
endmodule
"#,
    );

    let mut device = model.device("X1", &[1, 0]);
    device.set_analysis_type(1);
    let err = device
        .try_stamp_reactive(&[1.0], |_, _, _| {})
        .expect_err("checked reactive stamping must report runtime array bounds errors");
    let text = err.to_string();
    assert!(
        text.contains("Array index 5") || text.contains("[1:4]"),
        "diagnostic should identify the runtime array bounds error, got: {text}"
    );
}

#[test]
fn try_noise_sources_reports_runtime_array_index_errors() {
    let model = compile(
        r#"
`include "disciplines.vams"

module noise_runtime_oob(p, n);
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

    let mut device = model.device("X1", &[1, 0]);
    device.set_analysis_type(3);
    let err = device
        .try_noise_sources(&[1.0])
        .expect_err("checked noise evaluation must report runtime array bounds errors");
    let text = err.to_string();
    assert!(
        text.contains("Array index 5") || text.contains("[1:4]"),
        "diagnostic should identify the runtime array bounds error, got: {text}"
    );
}

#[test]
fn analysis_aliases_match_generated_runtime_semantics() {
    let model = compile(
        r#"
`include "disciplines.vams"

module analysis_aliases(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ analysis("op")
        + 2.0 * analysis("smallsig")
        + 4.0 * analysis("smallsignal")
        + 8.0 * analysis("small_signal");
endmodule
"#,
    );

    let mut device = model.device("X1", &[1, 0]);
    for (analysis_type, expected) in [(0, 1.0), (1, 14.0), (2, 0.0), (3, 14.0), (4, 0.0)] {
        device.set_analysis_type(analysis_type);
        device.update_voltages(&[0.0]);
        let currents = device.evaluate();
        assert_eq!(
            currents[0].to_bits(),
            f64::to_bits(expected),
            "analysis_type: {analysis_type}, currents: {currents:?}"
        );
    }
}

#[test]
fn try_noise_sources_preserves_flicker_and_table_metadata() {
    let model = compile(
        r#"
`include "disciplines.vams"

module noise_metadata(p, n);
    inout p, n;
    electrical p, n;
    parameter real s = 1.0e-18;
    parameter real ex = 2.0;
    analog begin
        I(p, n) <+ flicker_noise(s, ex, "fl");
        I(p, n) <+ noise_table('{1.0, 2.0e-18, 10.0, 4.0e-18}, "tbl");
    end
endmodule
"#,
    );

    let mut device = model.device("X1", &[1, 0]);
    device.set_analysis_type(3);
    let sources = device
        .try_noise_sources(&[0.0])
        .expect("checked noise metadata");

    let flicker = sources
        .iter()
        .find(|source| source.name == "fl")
        .expect("flicker source");
    assert!((flicker.psd - 1.0e-18).abs() < 1.0e-30);
    assert_eq!(flicker.exponent, Some(2.0));
    assert!(flicker.table.is_none());

    let table = sources
        .iter()
        .find(|source| source.name == "tbl")
        .expect("table source");
    assert_eq!(table.psd, 1.0);
    assert_eq!(
        table
            .table
            .as_ref()
            .map(|(points, log)| (points.len(), *log)),
        Some((2, false))
    );
}

#[test]
fn try_noise_sources_evaluates_current_probe_psd() {
    let model = compile(
        r#"
`include "disciplines.vams"

module noise_current_probe(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ V(p, n) * 2.0e-3
            + white_noise(abs(I(p, n)) * 4.0, "shot");
    end
endmodule
"#,
    );

    let mut device = model.device("X1", &[1, 0]);
    device.set_analysis_type(3);
    let sources = device
        .try_noise_sources(&[3.0])
        .expect("checked current-probe noise evaluation");

    let shot = sources
        .iter()
        .find(|source| source.name == "shot")
        .expect("shot noise source");
    assert_eq!(shot.node_pos, 1);
    assert_eq!(shot.node_neg, 0);
    assert!((shot.psd - 2.4e-2).abs() < 1.0e-15, "shot={shot:?}");
}

#[test]
fn idtmod_wraps_the_integral() {
    // Phase accumulator: phi = idtmod(rate, 0, 1) folds into [0, 1)
    let model = compile(
        r#"
`include "disciplines.vams"
module phase(p, n);
    inout p, n;
    electrical p, n;
    parameter real rate = 1.0e6;
    real phi;
    analog begin
        phi = idtmod(rate, 0.0, 1.0);
        I(p, n) <+ phi * 1.0e-3;
    end
endmodule
"#,
    );
    let mut device = model.device("PH1", &[1, 0]);
    device.update_voltages(&[0.0]);

    // DC: integral sits at its initial condition
    assert!(device.evaluate()[0].abs() < 1e-18);
    device.advance_state();

    // rate * dt = 0.25 per step; the fourth step wraps 1.0 -> 0.0
    device.set_analysis_type(2);
    device.set_timestep(0.25e-6);
    let mut phases = Vec::new();
    for _ in 0..6 {
        let current = device.evaluate()[0];
        phases.push(current / 1.0e-3);
        device.advance_state();
    }
    let expected = [0.25, 0.5, 0.75, 0.0, 0.25, 0.5];
    for (i, (got, want)) in phases.iter().zip(expected).enumerate() {
        assert!(
            (got - want).abs() < 1e-9,
            "step {i}: phase {got} != {want} (all: {phases:?})"
        );
    }
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

    let mut device = model.device("D1", &[1, 0]);
    device.update_voltages(&[bias]);
    let i0 = device.evaluate()[0];
    let (matrix, _) = collect_stamps(&mut device, &[bias]);
    let g_analytic = matrix[&(0, 0)];

    let mut device2 = model.device("D2", &[1, 0]);
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
