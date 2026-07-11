//! zi_* sampled-data filters: DC steady state at H(1), sample-and-hold
//! difference-equation evolution in transient, and the candidate/commit
//! protocol that keeps Newton re-evaluations idempotent.

use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

fn compile_device(instance: &str, source: &str) -> VerilogADevice {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler
        .compile(source)
        .expect("compile sampled-data filter model");
    #[cfg(feature = "native")]
    {
        let canonical_ir = compiler
            .compile_canonical_ir(source)
            .expect("compile sampled-data filter canonical IR");
        VerilogADevice::try_new_with_canonical_ir(instance, model, &canonical_ir, &[1, 0])
            .expect("construct sampled-data filter device from canonical IR")
    }
    #[cfg(not(feature = "native"))]
    {
        VerilogADevice::try_new(instance, model, &[1, 0])
            .expect("construct sampled-data filter bytecode device")
    }
}

fn stamp_once(device: &mut VerilogADevice, voltages: &[f64]) {
    device.stamp(voltages, |_, _, _| {}, |_, _| {});
}

/// First-order IIR lowpass: y[n] = 0.25 x[n] + 0.75 y[n-1], H(1) = 1
const IIR: &str = r#"
`include "disciplines.vams"
module ziavg(p, n);
    inout p, n;
    electrical p, n;
    real y;
    analog begin
        y = zi_nd(V(p, n), {0.25}, {1.0, -0.75}, 1.0e-6);
        I(p, n) <+ y * 1.0e-3;
    end
endmodule
"#;

#[test]
fn dc_sits_at_unity_gain_steady_state() {
    let mut device = compile_device("Z1", IIR);
    device.set_analysis_type(0);
    stamp_once(&mut device, &[2.0]);
    // H(1) = 0.25 / (1 - 0.75) = 1 -> y = input
    assert!((device.variable("y").unwrap() - 2.0).abs() < 1e-12);
}

#[test]
fn transient_follows_the_difference_equation_with_hold() {
    let mut device = compile_device("Z1", IIR);
    device.set_analysis_type(2);
    device.set_timestep(0.5e-6);

    // Step input of 1.0 from t=0. Samples land every 1 us; the first
    // sample at t=0 gives y = 0.25, then 0.4375, ...
    let mut expected = 0.0;
    for step in 0..6 {
        let t = step as f64 * 0.5e-6;
        device.set_time(t);
        // Two Newton-style evaluations at the same point must agree
        stamp_once(&mut device, &[1.0]);
        let first = device.variable("y").unwrap();
        stamp_once(&mut device, &[1.0]);
        let second = device.variable("y").unwrap();
        assert_eq!(first, second, "evaluation must be idempotent");

        let on_sample = step % 2 == 0;
        if on_sample {
            expected = 0.25 * 1.0 + 0.75 * expected;
        }
        assert!(
            (second - expected).abs() < 1e-12,
            "t={t:.2e}: y={second} expected {expected}"
        );
        device.advance_state();
    }
}

#[test]
fn zp_form_expands_z_roots() {
    // One zero at z=0, one pole at z=0.5: H(z) = z/(z-0.5), H(1) = 2
    let mut device = compile_device(
        "Z2",
        r#"
`include "disciplines.vams"
module zizp(p, n);
    inout p, n;
    electrical p, n;
    real y;
    analog begin
        y = zi_zp(V(p, n), {0.0, 0.0}, {0.5, 0.0}, 1.0e-6);
        I(p, n) <+ y * 1.0e-3;
    end
endmodule
"#,
    );
    device.set_analysis_type(0);
    stamp_once(&mut device, &[1.5]);
    assert!((device.variable("y").unwrap() - 3.0).abs() < 1e-12);
}

#[test]
fn non_constant_period_is_rejected() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"
module zbad(p, n);
    inout p, n;
    electrical p, n;
    parameter real ts = 1.0e-6;
    analog I(p, n) <+ zi_nd(V(p, n), {1.0}, {1.0}, ts);
endmodule
"#,
        )
        .expect_err("parameter-dependent period must be rejected")
        .to_string();
    assert!(err.contains("compile-time constant"), "got: {err}");
}
