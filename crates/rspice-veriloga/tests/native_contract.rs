//! Native JIT contract tests.
//!
//! Native mode is full JIT or error. These tests intentionally exercise the
//! foundation backend before broad canonical-IR codegen exists: construction
//! must return a native JIT error, not create a device that runs the VM.
#![cfg(feature = "native")]

use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::native::compile_native;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

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
