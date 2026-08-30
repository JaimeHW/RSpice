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

fn jacobian_sum(device: &mut VerilogADevice, voltages: &[f64]) -> f64 {
    let mut sum = 0.0;
    device.stamp(voltages, |_, _, value| sum += value, |_, _| {});
    sum
}

/// First-order IIR lowpass: y[n] = 0.25 x[n] + 0.75 y[n-1], H(1) = 1
const IIR: &str = r#"
`include "disciplines.vams"
module ziavg(p, n);
    inout p, n;
    electrical p, n;
    real y;
    analog begin
        y = zi_nd(V(p, n), '{0.25}, '{1.0, -0.75}, 1.0e-6, 0.0);
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
    device.try_begin_analysis(2).unwrap();
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
        y = zi_zp(V(p, n), '{0.0, 0.0}, '{0.5, 0.0}, 1.0e-6);
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
fn scalar_parameter_period_freezes_and_refreezes_per_instance() {
    let source = r#"
`include "disciplines.vams"
module zparam(p, n);
    inout p, n;
    electrical p, n;
    parameter real ts = 1.0e-6;
    real y;
    analog begin
        y = zi_nd(V(p, n), '{1.0}, '{1.0}, ts, 0.0);
        I(p, n) <+ y;
    end
endmodule
"#;
    let mut fast = compile_device("ZF", source);
    let mut slow = compile_device("ZS", source);
    assert!(fast.try_set_parameter("ts", 0.5e-6).unwrap());
    assert!(slow.try_set_parameter("ts", 2.0e-6).unwrap());
    fast.try_resolve_parameter_defaults().unwrap();
    slow.try_resolve_parameter_defaults().unwrap();

    for device in [&mut fast, &mut slow] {
        device.try_begin_analysis(2).unwrap();
        device.set_time(0.0);
        device.set_timestep(0.0);
        stamp_once(device, &[1.0]);
        device.try_advance_state().unwrap();
    }
    assert_eq!(fast.try_transient_bound_step().unwrap(), Some(0.5e-6));
    assert_eq!(slow.try_transient_bound_step().unwrap(), Some(2.0e-6));

    assert!(fast.try_set_parameter("ts", 0.75e-6).unwrap());
    fast.try_resolve_parameter_defaults().unwrap();
    fast.try_begin_analysis(2).unwrap();
    fast.set_time(0.0);
    fast.set_timestep(0.0);
    stamp_once(&mut fast, &[1.0]);
    fast.try_advance_state().unwrap();
    assert_eq!(fast.try_transient_bound_step().unwrap(), Some(0.75e-6));
    assert_eq!(slow.try_transient_bound_step().unwrap(), Some(2.0e-6));
}

#[test]
fn direct_zero_transition_is_rejected_but_positive_timing_forms_compile() {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let zero = r#"
`include "disciplines.vams"
module ztransition_zero(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ (1 ? 2.0 * zi_nd(V(p, n), '{1.0}, '{1.0}, 1.0e-6, 0.0) : 0.0);
endmodule
"#;
    let error = compiler
        .compile(zero)
        .expect_err("a statically abrupt Zi may not contribute directly")
        .to_string();
    assert!(
        error.contains("cannot be contributed directly"),
        "got: {error}"
    );
    assert!(error.contains("strictly positive"), "got: {error}");

    for suffix in ["", ", 1.0e-9", ", 1.0e-9, 2.0e-7"] {
        let source = format!(
            r#"
`include "disciplines.vams"
module ztransition(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ zi_nd(V(p, n), '{{1.0}}, '{{1.0}}, 1.0e-6{suffix});
endmodule
"#
        );
        compiler
            .compile(&source)
            .expect("omitted or positive transition and non-negative t0 are legal");
    }
}

#[test]
fn dynamic_direct_transition_rejects_a_zero_instance_override() {
    let mut device = compile_device(
        "ZD",
        r#"
`include "disciplines.vams"
module zdynamic_tau(p, n);
    inout p, n;
    electrical p, n;
    parameter real tau = 1.0e-9;
    analog I(p, n) <+ 2.0 * zi_nd(V(p, n), '{1.0}, '{1.0}, 1.0e-6, tau);
endmodule
"#,
    );
    assert!(device.try_set_parameter("tau", 0.0).unwrap());
    device.try_resolve_parameter_defaults().unwrap();
    device.try_begin_analysis(2).unwrap();
    device.set_time(0.0);
    let error = device
        .try_stamp(&[1.0], |_, _, _| {}, |_, _| {})
        .expect_err("a dynamic zero transition on a direct contribution must fail")
        .to_string();
    assert!(error.contains("zero-transition Zi output"), "got: {error}");
    assert!(error.contains("4.5.12"), "got: {error}");
}

#[test]
fn explicit_zero_transition_is_legal_through_an_intermediate_variable() {
    let mut device = compile_device(
        "ZI",
        r#"
`include "disciplines.vams"
module zintermediate(p, n);
    inout p, n;
    electrical p, n;
    real y;
    analog begin
        y = zi_nd(V(p, n), '{1.0}, '{1.0}, 1.0e-6, 0.0);
        I(p, n) <+ y;
    end
endmodule
"#,
    );
    device.try_begin_analysis(2).unwrap();
    device.set_time(0.0);
    stamp_once(&mut device, &[2.0]);
    assert_eq!(device.variable("y"), Some(2.0));
}

#[test]
fn omitted_transition_uses_default_ramp_and_exact_corner_breakpoint() {
    let mut device = compile_device(
        "ZR",
        r#"
`include "disciplines.vams"
module zdefault_ramp(p, n);
    inout p, n;
    electrical p, n;
    real y;
    analog begin
        y = zi_nd(V(p, n), '{0.25}, '{1.0, -0.75}, 1.0e-6);
        I(p, n) <+ y;
    end
endmodule
"#,
    );
    device.try_begin_analysis(2).unwrap();
    device.set_time(0.0);
    device.set_timestep(0.0);
    stamp_once(&mut device, &[1.0]);
    assert_eq!(device.variable("y"), Some(0.0));
    assert_eq!(device.try_transient_bound_step().unwrap(), Some(1.0e-9));
    device.try_advance_state().unwrap();

    device.set_time(0.5e-9);
    device.set_timestep(0.5e-9);
    stamp_once(&mut device, &[1.0]);
    assert!((device.variable("y").unwrap() - 0.125).abs() < 1.0e-15);
    assert_eq!(device.try_transient_bound_step().unwrap(), Some(0.5e-9));
    device.try_advance_state().unwrap();

    device.set_time(1.0e-9);
    device.set_timestep(0.5e-9);
    stamp_once(&mut device, &[1.0]);
    assert!((device.variable("y").unwrap() - 0.25).abs() < 1.0e-15);
}

#[test]
fn zi_requests_every_exact_sample_breakpoint() {
    let mut device = compile_device("Z1", IIR);
    device.try_begin_analysis(2).unwrap();
    device.set_timestep(0.0);
    device.set_time(0.0);
    stamp_once(&mut device, &[1.0]);
    assert_eq!(device.try_transient_bound_step().unwrap(), Some(1.0e-6));
    device.try_advance_state().unwrap();

    device.set_time(0.4e-6);
    device.set_timestep(0.4e-6);
    stamp_once(&mut device, &[1.0]);
    assert_eq!(device.try_transient_bound_step().unwrap(), Some(0.6e-6));
    device.try_advance_state().unwrap();

    device.set_time(1.0e-6);
    device.set_timestep(0.6e-6);
    stamp_once(&mut device, &[1.0]);
    assert_eq!(device.try_transient_bound_step().unwrap(), Some(1.0e-6));
}

#[test]
fn a_step_crossing_many_sample_edges_fails_closed() {
    let mut device = compile_device("Z1", IIR);
    device.try_begin_analysis(2).unwrap();
    device.set_time(0.0);
    stamp_once(&mut device, &[1.0]);
    device.try_advance_state().unwrap();

    device.set_time(1.0);
    device.set_timestep(1.0);
    let error = device
        .try_stamp(&[1.0], |_, _, _| {}, |_, _| {})
        .expect_err("crossing a million sample edges must not silently skip them")
        .to_string();
    assert!(error.contains("sample edge"), "got: {error}");
    assert!(error.contains("zi breakpoint"), "got: {error}");
}

#[test]
fn rejected_sample_candidate_rolls_back_exactly() {
    let mut device = compile_device("Z1", IIR);
    device.try_begin_analysis(2).unwrap();
    device.set_time(0.0);
    stamp_once(&mut device, &[1.0]);
    assert!((device.variable("y").unwrap() - 0.25).abs() < 1.0e-15);
    device.try_advance_state().unwrap();

    // Propose the next edge with a large input, then reject it.
    device.set_time(1.0e-6);
    device.set_timestep(1.0e-6);
    stamp_once(&mut device, &[10.0]);
    assert!((device.variable("y").unwrap() - 2.6875).abs() < 1.0e-15);

    // Accept an earlier retry. It must preserve only the t=0 history.
    device.set_time(0.5e-6);
    device.set_timestep(0.5e-6);
    stamp_once(&mut device, &[4.0]);
    assert!((device.variable("y").unwrap() - 0.25).abs() < 1.0e-15);
    device.try_advance_state().unwrap();

    device.set_time(1.0e-6);
    stamp_once(&mut device, &[1.0]);
    assert!((device.variable("y").unwrap() - 0.4375).abs() < 1.0e-15);
}

#[test]
fn explicit_analysis_begin_clears_all_prior_transient_history() {
    let mut device = compile_device("Z1", IIR);
    device.try_begin_analysis(2).unwrap();
    device.set_time(0.0);
    stamp_once(&mut device, &[4.0]);
    assert!((device.variable("y").unwrap() - 1.0).abs() < 1.0e-15);
    device.try_advance_state().unwrap();
    device.set_time(1.0e-6);
    stamp_once(&mut device, &[4.0]);
    assert!((device.variable("y").unwrap() - 1.75).abs() < 1.0e-15);
    device.try_advance_state().unwrap();

    device
        .try_begin_analysis(2)
        .expect("same-code fresh transient must reset Zi state");
    device.set_time(0.0);
    stamp_once(&mut device, &[4.0]);
    assert!((device.variable("y").unwrap() - 1.0).abs() < 1.0e-15);
    assert_eq!(device.try_transient_bound_step().unwrap(), Some(1.0e-6));
    device.try_advance_state().unwrap();
}

#[test]
fn singular_dc_gain_is_a_typed_runtime_error() {
    let mut device = compile_device(
        "ZS",
        r#"
`include "disciplines.vams"
module zsingular(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ zi_nd(V(p, n), '{1.0}, '{1.0, -1.0}, 1.0e-6);
endmodule
"#,
    );
    let error = device
        .try_stamp(&[1.0], |_, _, _| {}, |_, _| {})
        .expect_err("H(1) with a zero denominator must fail")
        .to_string();
    assert!(error.contains("singular zi DC equilibrium"), "got: {error}");
}

#[test]
fn invalid_zi_definitions_are_compile_errors() {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    for (definition, expected) in [
        ("'{0.0}, 1.0e-6", "a0 must be nonzero"),
        ("'{1.0}, 0.0", "greater than zero"),
    ] {
        let source = format!(
            r#"
`include "disciplines.vams"
module zbad(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ zi_nd(V(p, n), '{{1.0}}, {definition});
endmodule
"#
        );
        let error = compiler
            .compile(&source)
            .expect_err("invalid zi definition must fail compilation")
            .to_string();
        assert!(error.contains(expected), "got: {error}");
    }

    let nonconjugate = r#"
`include "disciplines.vams"
module zbadroot(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ zi_zp(V(p, n), '{0.5, 0.25}, '{0.0, 0.0}, 1.0e-6);
endmodule
"#;
    let error = compiler
        .compile(nonconjugate)
        .expect_err("non-conjugate z root must fail compilation")
        .to_string();
    assert!(error.contains("conjugate"), "got: {error}");

    let negative_t0 = r#"
`include "disciplines.vams"
module zbadt0(p, n);
    inout p, n;
    electrical p, n;
    real y;
    analog begin
        y = zi_nd(V(p, n), '{1.0}, '{1.0}, 1.0e-6, 0.0, -1.0e-9);
        I(p, n) <+ y;
    end
endmodule
"#;
    let error = compiler
        .compile(negative_t0)
        .expect_err("a wholly constant negative first-transition time must fail compilation")
        .to_string();
    assert!(error.contains("non-negative"), "got: {error}");
}

#[test]
fn parameterized_invalid_zi_definitions_fail_at_instance_freeze() {
    let source = r#"
`include "disciplines.vams"
module zbadparam(p, n);
    inout p, n;
    electrical p, n;
    parameter real a0 = 1.0;
    parameter real ts = 1.0e-6;
    parameter real t0 = 0.0;
    real y;
    analog begin
        y = zi_nd(V(p, n), '{1.0}, '{a0}, ts, 0.0, t0);
        I(p, n) <+ y;
    end
endmodule
"#;
    for (parameter, value, expected) in [
        ("a0", 0.0, "a0 must be nonzero"),
        ("ts", 0.0, "greater than zero"),
        ("t0", -1.0e-9, "non-negative"),
    ] {
        let mut device = compile_device("ZP", source);
        assert!(device.try_set_parameter(parameter, value).unwrap());
        let error = match device.try_resolve_parameter_defaults() {
            Err(error) => error.to_string(),
            Ok(()) => {
                device.try_begin_analysis(2).unwrap();
                device.set_time(0.0);
                device
                    .try_stamp(&[1.0], |_, _, _| {}, |_, _| {})
                    .expect_err("invalid per-instance Zi definition must fail during lazy freeze")
                    .to_string()
            }
        };
        assert!(error.contains(expected), "got: {error}");
    }
}

#[test]
fn zi_operand_ceiling_is_platform_uniform_for_coefficients_and_mixed_roots() {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let coefficients = |len: usize| vec!["1.0"; len].join(", ");
    let source = |operator: &str, numerator: String, denominator: String| {
        format!(
            r#"
`include "disciplines.vams"
module zbudget(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ {operator}(V(p, n), '{{{numerator}}}, '{{{denominator}}}, 1.0e-6);
endmodule
"#
        )
    };

    compiler
        .compile(&source("zi_nd", coefficients(1019), coefficients(1)))
        .expect("1,020 definition scalars plus four fixed operands are supported");
    let error = compiler
        .compile(&source("zi_nd", coefficients(1020), coefficients(1)))
        .expect_err("1,025 runtime operands must fail in the shared frontend")
        .to_string();
    assert!(
        error.contains("platform-uniform maximum 1024"),
        "got: {error}"
    );

    let roots = |len: usize| vec!["0.0, 0.0"; len].join(", ");
    compiler
        .compile(&source("zi_zd", roots(509), "1.0, 0.0".into()))
        .expect("mixed roots/coefficients at exactly 1,024 operands are supported");
    let error = compiler
        .compile(&source("zi_zd", roots(510), "1.0, 0.0".into()))
        .expect_err("mixed roots/coefficients over the shared ceiling must fail")
        .to_string();
    assert!(
        error.contains("platform-uniform maximum 1024"),
        "got: {error}"
    );
}

#[test]
fn transient_jacobian_is_feedthrough_on_edges_and_zero_on_hold() {
    let mut device = compile_device(
        "ZJ",
        r#"
`include "disciplines.vams"
module zjac(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ zi_nd(V(p, n), '{0.25}, '{1.0, -0.75}, 1.0e-6);
endmodule
"#,
    );
    device.try_begin_analysis(2).unwrap();
    device.set_time(0.0);
    assert_eq!(
        jacobian_sum(&mut device, &[1.0]).to_bits(),
        0.0_f64.to_bits(),
        "a positive default transition hides the sampled target Jacobian"
    );
    device.try_advance_state().unwrap();

    device.set_time(0.5e-6);
    device.set_timestep(0.5e-6);
    assert_eq!(
        jacobian_sum(&mut device, &[2.0]).to_bits(),
        0.0_f64.to_bits()
    );

    let mut abrupt = compile_device(
        "ZJA",
        r#"
`include "disciplines.vams"
module zjac_abrupt(p, n);
    inout p, n;
    electrical p, n;
    real y;
    analog begin
        y = zi_nd(V(p, n), '{0.25}, '{1.0, -0.75}, 1.0e-6, 0.0);
        I(p, n) <+ y;
    end
endmodule
"#,
    );
    abrupt.try_begin_analysis(2).unwrap();
    abrupt.set_time(0.0);
    assert!((jacobian_sum(&mut abrupt, &[1.0]) - 0.25).abs() < 1.0e-15);
    abrupt.try_advance_state().unwrap();
    abrupt.set_time(0.5e-6);
    abrupt.set_timestep(0.5e-6);
    assert_eq!(
        jacobian_sum(&mut abrupt, &[2.0]).to_bits(),
        0.0_f64.to_bits()
    );
}

#[test]
fn runtime_conditional_zi_is_rejected_but_parameter_static_zi_is_legal() {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let dynamic_error = compiler
        .compile(
            r#"
`include "disciplines.vams"
module zdynamic(p, n);
    inout p, n;
    electrical p, n;
    analog if (V(p, n) > 0.0)
        I(p, n) <+ zi_nd(V(p, n), '{1.0}, '{1.0}, 1.0e-6);
endmodule
"#,
        )
        .expect_err("a runtime-varying Zi guard violates analog-operator evaluation rules")
        .to_string();
    assert!(
        dynamic_error.contains("every Newton iteration"),
        "got: {dynamic_error}"
    );
    assert!(dynamic_error.contains("4.5.15"), "got: {dynamic_error}");

    for enabled in [0, 1] {
        let source = format!(
            r#"
`include "disciplines.vams"
module zstatic(p, n);
    inout p, n;
    electrical p, n;
    parameter integer enabled = {enabled};
    analog if (enabled)
        I(p, n) <+ zi_nd(V(p, n), '{{1.0}}, '{{1.0}}, 1.0e-6);
endmodule
"#
        );
        let mut device = compile_device("ZS", &source);
        device.try_begin_analysis(2).unwrap();
        device.set_time(0.0);
        stamp_once(&mut device, &[2.0]);
        device
            .try_advance_state()
            .expect("active and dormant analysis slots must remain synchronized");
    }
}

#[test]
fn zi_is_rejected_in_dynamic_case_non_genvar_loop_and_analog_function() {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let sources = [
        r#"
`include "disciplines.vams"
module zcase(p, n);
    inout p, n; electrical p, n; real y;
    analog begin
        case (V(p, n) > 0.0)
            1: y = zi_nd(V(p, n), '{1.0}, '{1.0}, 1.0e-6);
            default: y = 0.0;
        endcase
        I(p, n) <+ y;
    end
endmodule
"#,
        r#"
`include "disciplines.vams"
module zloop(p, n);
    inout p, n; electrical p, n; integer i; real y;
    analog begin
        y = 0.0;
        for (i = 0; i < 1; i = i + 1)
            y = zi_nd(V(p, n), '{1.0}, '{1.0}, 1.0e-6);
        I(p, n) <+ y;
    end
endmodule
"#,
        r#"
`include "disciplines.vams"
module zfunction(p, n);
    inout p, n; electrical p, n;
    analog function real sampled;
        input x;
        real x;
        begin
            sampled = zi_nd(x, '{1.0}, '{1.0}, 1.0e-6);
        end
    endfunction
    analog I(p, n) <+ sampled(V(p, n));
endmodule
"#,
    ];

    for source in sources {
        let error = compiler
            .compile(source)
            .expect_err("LRM-restricted Zi placement must fail")
            .to_string();
        assert!(error.contains("every Newton iteration"), "got: {error}");
        assert!(error.contains("4.5.15"), "got: {error}");
    }
}
