//! Native JIT contract tests.
//!
//! Native mode is full JIT or error. These tests intentionally exercise the
//! foundation backend before broad canonical-IR codegen exists: construction
//! must return a native JIT error, not create a device that runs the VM.
#![cfg(feature = "native")]

use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::native::compile_native;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::collections::HashMap;

fn compile(source: &str) -> rspice_veriloga::CompiledModel {
    VerilogACompiler::new(CompilerOptions::default())
        .compile(source)
        .expect("Verilog-A source must compile")
}

fn hybrid_fallback_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module hybrid(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nseg = 5 from [1:8];
    parameter real scale = 1.0;
    real w[1:8];
    integer i;
    real total, gmod;
    analog begin
        gmod = (12 % 5) * 1.0e-4;
        if ($param_given(scale))
            gmod = gmod * scale;
        for (i = 1; i <= nseg; i = i + 1)
            w[i] = 0.001 * i * V(p, n);
        total = 0.0;
        for (i = 1; i <= nseg; i = i + 1)
            total = total + w[i];
        I(p, n) <+ total + gmod * V(p, n);
    end
endmodule
"#,
    )
}

fn simple_resistor_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module rnative(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 2.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
    )
}

fn assignment_fed_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module assign_native(p, n);
    inout p, n;
    electrical p, n;
    real g;
    analog begin
        g = 0.25;
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
    )
}

fn chained_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module assign_chain_native(p, n);
    inout p, n;
    electrical p, n;
    real a, b;
    analog begin
        a = 0.25;
        b = a * 2.0;
        I(p, n) <+ b * V(p, n);
    end
endmodule
"#,
    )
}

fn runtime_loop_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module loop_native(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nseg = 2 from [1:4];
    integer i;
    real total;
    analog begin
        total = 0.0;
        for (i = 0; i < nseg; i = i + 1)
            total = total + V(p, n);
        I(p, n) <+ total;
    end
endmodule
"#,
    )
}

fn indexed_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module indexed_native(p, n);
    inout p, n;
    electrical p, n;
    parameter integer idx = 1 from [1:2];
    real w[1:2];
    analog begin
        w[idx] = 0.25;
        I(p, n) <+ w[idx] * V(p, n);
    end
endmodule
"#,
    )
}

fn assert_native_hard_fail_message(msg: &str) {
    assert!(
        msg.contains("native JIT"),
        "error must identify native JIT failure, got: {msg}"
    );
    assert!(
        msg.contains("no interpreter fallback"),
        "error must state the hard-fail contract, got: {msg}"
    );
}

fn stamp_device(
    device: &mut VerilogADevice,
    voltages: &[f64],
) -> (HashMap<(usize, usize), f64>, HashMap<usize, f64>) {
    let mut matrix = HashMap::new();
    let mut rhs = HashMap::new();

    device.stamp(
        voltages,
        |row, col, value| {
            *matrix.entry((row, col)).or_insert(0.0) += value;
        },
        |row, value| {
            *rhs.entry(row).or_insert(0.0) += value;
        },
    );

    (matrix, rhs)
}

fn noise_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module noisy(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ V(p, n) * 1.0e-3 + white_noise(1.0e-18, "thermal");
    end
endmodule
"#,
    )
}

fn reactive_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module capjit(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1.0e-12;
    analog begin
        I(p, n) <+ ddt(c * V(p, n));
    end
endmodule
"#,
    )
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_compile_accepts_simple_resistor_subset() {
    let model = simple_resistor_model();

    let native = compile_native(&model).expect("x64 native JIT must compile simple resistor");

    assert_eq!(native.native_stamp_count(), model.stamp_programs.len());
    assert_eq!(
        native.plan_stats().jacobian_entry_points,
        model
            .stamp_programs
            .iter()
            .map(|stamp| stamp.jacobian_programs.len())
            .sum::<usize>()
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_stamps_simple_resistor_without_interpreter_fallback() {
    let model = simple_resistor_model();
    let mut device =
        VerilogADevice::try_new("RN1", model, &[1, 0]).expect("simple resistor uses native JIT");
    assert!(device.is_using_native());

    let (matrix, rhs) = stamp_device(&mut device, &[4.0]);

    assert!(
        (matrix.get(&(0, 0)).copied().unwrap_or_default() - 0.5).abs() < 1e-12,
        "matrix: {matrix:?}"
    );
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_scalar_assignment_pass() {
    let model = assignment_fed_model();
    let mut device = VerilogADevice::try_new("AN1", model, &[1, 0])
        .expect("assignment-fed model uses native JIT");
    assert!(device.is_using_native());

    let (matrix, rhs) = stamp_device(&mut device, &[8.0]);

    assert!(
        (matrix.get(&(0, 0)).copied().unwrap_or_default() - 0.25).abs() < 1e-12,
        "matrix: {matrix:?}"
    );
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_scalar_assignments_in_source_order() {
    let model = chained_assignment_model();
    assert!(
        model.assignment_steps.len() >= 2,
        "fixture must contain multiple scalar assignment steps"
    );
    let mut device = VerilogADevice::try_new("ACHAIN1", model, &[1, 0])
        .expect("assignment-chain model uses native JIT");
    assert!(device.is_using_native());
    assert_eq!(device.native_plan_stats().assignment_entry_points, 1);

    let (matrix, rhs) = stamp_device(&mut device, &[8.0]);

    assert!(
        (matrix.get(&(0, 0)).copied().unwrap_or_default() - 0.5).abs() < 1e-12,
        "matrix: {matrix:?}"
    );
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
    );
}

#[test]
fn native_compile_rejects_runtime_loop_assignments_without_fallback() {
    let model = runtime_loop_model();
    assert!(
        model
            .assignment_steps
            .iter()
            .any(|step| matches!(step, rspice_veriloga::codegen::AssignmentStep::Loop { .. })),
        "fixture must contain a runtime assignment loop"
    );

    let err = compile_native(&model).expect_err("native JIT must reject runtime assignment loops");
    let msg = err.to_string();

    assert_native_hard_fail_message(&msg);
    assert!(msg.contains("Loop"), "error must name Loop, got: {msg}");
}

#[test]
fn native_compile_rejects_indexed_assignments_without_fallback() {
    let model = indexed_assignment_model();
    assert!(
        model.assignment_steps.iter().any(|step| matches!(
            step,
            rspice_veriloga::codegen::AssignmentStep::AssignIndexed { .. }
        )),
        "fixture must contain an indexed assignment"
    );

    let err = compile_native(&model).expect_err("native JIT must reject indexed assignments");
    let msg = err.to_string();

    assert_native_hard_fail_message(&msg);
    assert!(
        msg.contains("AssignIndexed") || msg.contains("PushVariableDyn"),
        "error must name indexed assignment coverage, got: {msg}"
    );
}

#[test]
fn native_compile_rejects_noise_sources_without_fallback() {
    let model = noise_model();
    assert!(
        !model.noise_sources.is_empty(),
        "fixture must contain a compiled noise source"
    );

    let err =
        compile_native(&model).expect_err("native JIT must reject unsupported noise coverage");
    let msg = err.to_string();

    assert_native_hard_fail_message(&msg);
    assert!(
        msg.contains("NoiseSources"),
        "error must name unsupported noise coverage, got: {msg}"
    );
}

#[test]
fn native_compile_rejects_reactive_jacobians_without_fallback() {
    let model = reactive_model();
    assert!(
        model
            .stamp_programs
            .iter()
            .any(|stamp| !stamp.reactive_jacobians.is_empty()),
        "fixture must contain compiled reactive Jacobians"
    );

    let err =
        compile_native(&model).expect_err("native JIT must reject unsupported reactive coverage");
    let msg = err.to_string();

    assert_native_hard_fail_message(&msg);
    assert!(
        msg.contains("ReactiveJacobians"),
        "error must name unsupported reactive coverage, got: {msg}"
    );
}

#[test]
fn native_compile_failure_is_not_interpreter_fallback() {
    let model = hybrid_fallback_model();

    let err = VerilogADevice::try_new("H1", model, &[1, 0])
        .expect_err("native mode must fail until a complete native image exists");
    let msg = err.to_string();

    assert_native_hard_fail_message(&msg);
}

#[test]
fn native_new_panics_instead_of_falling_back() {
    let model = hybrid_fallback_model();

    let panic = std::panic::catch_unwind(|| {
        let _ = VerilogADevice::new("H2", model, &[1, 0]);
    })
    .expect_err("unchecked constructor must panic on native JIT failure");

    let msg = panic
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| panic.downcast_ref::<&'static str>().copied())
        .unwrap_or("<non-string panic>");
    assert_native_hard_fail_message(msg);
}
