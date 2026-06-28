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

fn multi_stamp_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_multi_stamp(p, n);
    inout p, n;
    electrical p, n;
    parameter real g1 = 0.25;
    parameter real g2 = 0.75;
    analog begin
        I(p, n) <+ g1 * V(p, n);
        I(p, n) <+ g2 * V(p, n);
    end
endmodule
"#,
    )
}

fn potential_branch_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_zres(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 2000.0 from (0:inf);
    analog V(p, n) <+ I(p, n) * r;
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

fn scalar_context_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_context_scalar(p, n);
    inout p, n;
    electrical p, n;
    real gain;
    analog begin
        gain = (($temperature - 300.0) * 1.0e-3) + (2.0 * $abstime) + (3.0 * $mfactor);
        I(p, n) <+ gain * V(p, n);
    end
endmodule
"#,
    )
}

fn flag_context_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_context_flags(p, n, opt);
    inout p, n, opt;
    electrical p, n, opt;
    parameter real rknob = 2.0 from (0:inf);
    real gain;
    analog begin
        gain = (2.0 * $param_given(rknob)) + (3.0 * $port_connected(opt));
        I(p, n) <+ gain * V(p, n);
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

fn internal_node_divider_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_divider(p, n);
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

fn current_probe_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_current_probe(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ V(p, n);
        I(p, n) <+ I(p, n) * 0.1;
    end
endmodule
"#,
    )
}

fn unavailable_current_probe_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_missing_current_probe(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ I(p, n);
endmodule
"#,
    )
}

fn nonfinite_prior_current_probe_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_nonfinite_current_probe(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ V(p, n) / 0.0;
        I(p, n) <+ I(p, n);
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
fn native_model_image_publishes_multiple_stamp_and_jacobian_entries() {
    let model = multi_stamp_model();
    assert_eq!(model.stamp_programs.len(), 2);

    let native = compile_native(&model).expect("multi-stamp model compiles native");

    assert_eq!(native.native_stamp_count(), 2);
    assert_eq!(native.plan_stats().stamp_value_entry_points, 2);
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
fn native_device_stamps_multiple_flow_contributions_from_one_image() {
    let model = multi_stamp_model();
    let mut device =
        VerilogADevice::try_new("MS1", model, &[1, 0]).expect("multi-stamp model uses native JIT");

    let currents = {
        device.update_voltages(&[4.0]);
        device.try_evaluate().expect("native multi-stamp evaluate")
    };
    assert_eq!(currents.len(), 2);
    assert!((currents[0] - 1.0).abs() < 1e-12, "currents: {currents:?}");
    assert!((currents[1] - 3.0).abs() < 1e-12, "currents: {currents:?}");

    let jacobians = device
        .try_compute_jacobian()
        .expect("native multi-stamp jacobian evaluate");
    let jacobian_order = jacobians
        .iter()
        .map(|entry| (entry.program_idx, entry.jacobian_idx, entry.value))
        .collect::<Vec<_>>();
    let expected = [
        (0, 0, 0.25),
        (0, 1, -0.25),
        (0, 2, -0.25),
        (0, 3, 0.25),
        (1, 0, 0.75),
        (1, 1, -0.75),
        (1, 2, -0.75),
        (1, 3, 0.75),
    ];
    assert_eq!(
        jacobian_order.len(),
        expected.len(),
        "jacobians: {jacobians:?}"
    );
    for (actual, expected) in jacobian_order.iter().zip(expected) {
        assert_eq!(actual.0, expected.0, "jacobians: {jacobians:?}");
        assert_eq!(actual.1, expected.1, "jacobians: {jacobians:?}");
        assert!(
            (actual.2 - expected.2).abs() < 1e-12,
            "jacobians: {jacobians:?}"
        );
    }

    let (matrix, rhs) = stamp_device(&mut device, &[4.0]);
    assert!((matrix.get(&(0, 0)).copied().unwrap_or_default() - 1.0).abs() < 1e-12);
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_stamps_potential_branch_unknowns() {
    let model = potential_branch_model();
    assert_eq!(model.branch_sources.len(), 1);

    let mut device =
        VerilogADevice::try_new("Z1", model, &[1, 2]).expect("potential branch uses native JIT");
    device.set_branch_current_indices(&[3]);

    let (matrix, rhs) = stamp_device(&mut device, &[1.0, 0.5, 1.0e-3]);

    assert!((matrix.get(&(0, 2)).copied().unwrap_or_default() - 1.0).abs() < 1e-12);
    assert!((matrix.get(&(1, 2)).copied().unwrap_or_default() + 1.0).abs() < 1e-12);
    assert!((matrix.get(&(2, 0)).copied().unwrap_or_default() - 1.0).abs() < 1e-12);
    assert!((matrix.get(&(2, 1)).copied().unwrap_or_default() + 1.0).abs() < 1e-12);
    assert!((matrix.get(&(2, 2)).copied().unwrap_or_default() + 2000.0).abs() < 1e-9);
    assert!(
        rhs.get(&2).copied().unwrap_or_default().abs() < 1e-12,
        "rhs: {rhs:?}"
    );
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
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

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_scalar_simulator_context_reads() {
    let model = scalar_context_model();
    let mut device = VerilogADevice::try_new("CTX1", model, &[1, 0])
        .expect("context scalar model uses native JIT");
    device.set_temperature(310.0);
    device.set_time(2.0);
    device.set_multiplicity(3.0);
    device.update_voltages(&[4.0]);

    let currents = device
        .try_evaluate()
        .expect("native context scalar evaluation succeeds");

    assert!(
        (currents[0] - 52.04).abs() < 1e-12,
        "currents: {currents:?}"
    );
    assert!((device.variable("gain").unwrap() - 13.01).abs() < 1e-12);
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_param_given_and_port_connected_reads() {
    let model = flag_context_model();

    let mut omitted = VerilogADevice::try_new("FLG1", model.clone(), &[1, 0])
        .expect("flag model uses native JIT");
    omitted.update_voltages(&[2.0]);
    assert_eq!(omitted.try_evaluate().unwrap()[0], 0.0);
    assert_eq!(omitted.variable("gain"), Some(0.0));

    let mut param_only = VerilogADevice::try_new("FLG2", model.clone(), &[1, 0])
        .expect("flag model uses native JIT");
    assert!(param_only.set_parameter("rknob", 2.0));
    param_only.update_voltages(&[2.0]);
    let currents = param_only.try_evaluate().unwrap();
    assert!((currents[0] - 4.0).abs() < 1e-12, "currents: {currents:?}");
    assert_eq!(param_only.variable("gain"), Some(2.0));

    let mut port_only = VerilogADevice::try_new("FLG3", model.clone(), &[1, 0, 0])
        .expect("flag model uses native JIT");
    port_only.update_voltages(&[2.0]);
    let currents = port_only.try_evaluate().unwrap();
    assert!((currents[0] - 6.0).abs() < 1e-12, "currents: {currents:?}");
    assert_eq!(port_only.variable("gain"), Some(3.0));

    let mut connected =
        VerilogADevice::try_new("FLG4", model, &[1, 0, 0]).expect("flag model uses native JIT");
    assert!(connected.set_parameter("rknob", 2.0));
    connected.update_voltages(&[2.0]);

    let currents = connected.try_evaluate().unwrap();
    assert!((currents[0] - 10.0).abs() < 1e-12, "currents: {currents:?}");
    assert_eq!(connected.variable("gain"), Some(5.0));
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_evaluates_internal_node_voltage_contributions() {
    let model = internal_node_divider_model();
    assert_eq!(model.internal_nodes, 1);
    assert_eq!(model.stamp_programs.len(), 2);

    let mut device =
        VerilogADevice::try_new("DIV1", model, &[1, 0]).expect("divider uses native JIT");
    assert!(device.is_using_native());
    device.set_internal_node_indices(&[2]);
    device.update_all_voltages(&[2.0, 0.5]);

    let currents = device
        .try_evaluate()
        .expect("native internal-node evaluation succeeds");

    assert!((currents[0] - 1.5).abs() < 1e-12, "currents: {currents:?}");
    assert!((currents[1] - 0.5).abs() < 1e-12, "currents: {currents:?}");
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_stamps_internal_node_jacobians() {
    let model = internal_node_divider_model();
    let mut device =
        VerilogADevice::try_new("DIV2", model, &[1, 0]).expect("divider uses native JIT");
    device.set_internal_node_indices(&[2]);

    let (matrix, rhs) = stamp_device(&mut device, &[2.0, 0.5]);

    assert!((matrix.get(&(0, 0)).copied().unwrap_or_default() - 1.0).abs() < 1e-12);
    assert!((matrix.get(&(0, 1)).copied().unwrap_or_default() + 1.0).abs() < 1e-12);
    assert!((matrix.get(&(1, 0)).copied().unwrap_or_default() + 1.0).abs() < 1e-12);
    assert!((matrix.get(&(1, 1)).copied().unwrap_or_default() - 2.0).abs() < 1e-12);
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

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_terminal_pair_current_probes_in_source_order() {
    let model = current_probe_model();
    let mut device = VerilogADevice::try_new("CP1", model, &[1, 0])
        .expect("current probe model uses native JIT");
    assert!(device.is_using_native());
    device.update_voltages(&[4.0]);

    let currents = device
        .try_evaluate()
        .expect("native current-probe evaluation succeeds");

    assert_eq!(currents.len(), 2);
    assert!((currents[0] - 4.0).abs() < 1e-12, "currents: {currents:?}");
    assert!((currents[1] - 0.4).abs() < 1e-12, "currents: {currents:?}");
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_compile_rejects_unavailable_terminal_pair_current_probes_without_fallback() {
    let model = unavailable_current_probe_model();

    let err = compile_native(&model).expect_err("missing current probe source must not compile");
    let msg = err.to_string();

    assert_native_hard_fail_message(&msg);
    assert!(
        msg.contains("PushCurrent terminal pair 0,1 unavailable"),
        "error must name unavailable current pair, got: {msg}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_evaluate_rejects_nonfinite_terminal_pair_current_probes_without_fallback() {
    let model = nonfinite_prior_current_probe_model();
    let mut device = VerilogADevice::try_new("CPINF1", model, &[1, 0])
        .expect("structurally available current probe compiles native");
    device.update_voltages(&[4.0]);

    let err = device
        .try_evaluate()
        .expect_err("non-finite prior terminal-pair current must be a runtime error");
    let msg = err.to_string();

    assert!(
        msg.contains("missing terminal-pair current slot"),
        "error must match interpreter current-probe semantics, got: {msg}"
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
