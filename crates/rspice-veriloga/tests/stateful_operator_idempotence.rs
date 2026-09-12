#![cfg(not(feature = "native"))]

use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::sync::Arc;

fn device(source: &str) -> VerilogADevice {
    let model = VerilogACompiler::new(CompilerOptions::default())
        .compile(source)
        .expect("stateful-operator model compiles");
    VerilogADevice::try_new("A1", Arc::new(model), &[1, 0]).expect("device constructs")
}

fn evaluate(device: &mut VerilogADevice, time: f64, voltage: f64) -> f64 {
    device.set_time(time);
    device.update_voltages(&[voltage]);
    device.try_evaluate().expect("evaluation succeeds")[0]
}

#[test]
fn laplace_recomputes_from_the_last_accepted_state() {
    let mut device = device(
        r#"
`include "disciplines.vams"
module laplace_idempotent(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ laplace_nd(1.0, '{1.0}, '{1.0, 1.0});
endmodule
"#,
    );
    device.set_analysis_type(2);
    device.set_timestep(1.0);

    let first = evaluate(&mut device, 1.0, 0.0);
    let repeated = evaluate(&mut device, 1.0, 0.0);
    assert!((first - 0.5).abs() < 1.0e-12, "first={first}");
    assert_eq!(first.to_bits(), repeated.to_bits());

    device.advance_state();
    let next = evaluate(&mut device, 2.0, 0.0);
    assert!((next - 0.75).abs() < 1.0e-12, "next={next}");
}

#[test]
fn absdelay_does_not_consume_history_during_newton_reevaluation() {
    let mut device = device(
        r#"
`include "disciplines.vams"
module delay_idempotent(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ absdelay(V(p, n), 2.0, 10.0);
endmodule
"#,
    );
    device.set_analysis_type(2);

    assert_eq!(evaluate(&mut device, 0.0, 1.0), 1.0);
    device.advance_state();

    let first = evaluate(&mut device, 2.0, 2.0);
    for _ in 0..2048 {
        assert_eq!(evaluate(&mut device, 2.0, 2.0), first);
    }
    assert_eq!(first, 1.0);
}

#[test]
fn absdelay_rejects_nonpositive_delay() {
    let mut device = device(
        r#"
`include "disciplines.vams"
module dynamic_delay_history(p, n);
    inout p, n;
    electrical p, n;
    parameter real td = 0.0;
    analog I(p, n) <+ absdelay(V(p, n), td);
endmodule
"#,
    );
    device.set_analysis_type(2);

    device.set_time(0.0);
    device.update_voltages(&[1.0]);
    let zero = device
        .try_evaluate()
        .expect_err("zero absdelay td must fail closed");
    assert!(zero.to_string().contains("strictly positive"));

    assert!(device.set_parameter("td", -1.0));
    let negative = device
        .try_evaluate()
        .expect_err("negative absdelay td must fail closed");
    assert!(negative.to_string().contains("strictly positive"));
}

#[test]
fn transition_restarts_each_candidate_from_accepted_state() {
    let mut device = device(
        r#"
`include "disciplines.vams"
module transition_idempotent(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ transition(V(p, n), 0.0, 2.0, 2.0);
endmodule
"#,
    );
    device.set_analysis_type(2);

    assert_eq!(evaluate(&mut device, 0.0, 0.0), 0.0);
    device.advance_state();
    assert_eq!(evaluate(&mut device, 1.0, 1.0), 0.0);
    device.advance_state();

    let first = evaluate(&mut device, 2.0, 1.0);
    let alternate = evaluate(&mut device, 2.0, 2.0);
    let repeated = evaluate(&mut device, 2.0, 1.0);
    assert!((first - 0.5).abs() < 1.0e-12, "first={first}");
    assert_eq!(first.to_bits(), repeated.to_bits());
    assert_eq!(alternate.to_bits(), first.to_bits());
}

#[test]
fn transition_promotes_the_operating_point_before_first_transient_candidate() {
    let mut device = device(
        r#"
`include "disciplines.vams"
module transition_op_startup(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ transition(V(p, n), 0.0, 2.0, 2.0);
endmodule
"#,
    );

    device.set_analysis_type(0);
    assert_eq!(evaluate(&mut device, 0.0, 5.0).to_bits(), 5.0_f64.to_bits());
    let op_jacobians = device
        .try_compute_jacobian()
        .expect("transition operating-point Jacobian succeeds");
    assert_eq!(op_jacobians.len(), 4, "jacobians={op_jacobians:?}");
    assert!(
        op_jacobians
            .iter()
            .all(|entry| entry.value.abs().to_bits() == 1.0_f64.to_bits()),
        "jacobians={op_jacobians:?}"
    );

    device.set_analysis_type(2);
    assert_eq!(evaluate(&mut device, 0.0, 5.0).to_bits(), 5.0_f64.to_bits());
    device.advance_state();
    assert_eq!(evaluate(&mut device, 1.0, 7.0).to_bits(), 5.0_f64.to_bits());
}

#[test]
fn transition_omitted_fall_time_reuses_the_effective_rise_time() {
    let mut device = device(
        r#"
`include "disciplines.vams"
module transition_fall_default(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ transition(V(p, n), 0.0, 2.0);
endmodule
"#,
    );
    device.set_analysis_type(2);

    assert_eq!(evaluate(&mut device, 0.0, 1.0).to_bits(), 1.0_f64.to_bits());
    device.advance_state();
    assert_eq!(evaluate(&mut device, 1.0, 0.0).to_bits(), 1.0_f64.to_bits());
    device.advance_state();
    assert_eq!(evaluate(&mut device, 2.0, 0.0).to_bits(), 0.5_f64.to_bits());
    device.advance_state();
    assert_eq!(evaluate(&mut device, 3.0, 0.0).to_bits(), 0.0_f64.to_bits());
}

#[test]
fn transition_finite_ramp_has_zero_transient_input_jacobian() {
    let mut device = device(
        r#"
`include "disciplines.vams"
module transition_ramp_jacobian(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ transition(V(p, n), 0.0, 2.0, 2.0);
endmodule
"#,
    );
    device.set_analysis_type(2);

    assert_eq!(evaluate(&mut device, 0.0, 0.0), 0.0);
    device.advance_state();
    assert_eq!(evaluate(&mut device, 1.0, 1.0), 0.0);
    let jacobians = device
        .try_compute_jacobian()
        .expect("transition ramp Jacobian succeeds");
    assert_eq!(jacobians.len(), 4, "jacobians={jacobians:?}");
    assert!(
        jacobians.iter().all(|entry| entry.value == 0.0),
        "finite transition ramp must be history-driven: {jacobians:?}"
    );
}

#[test]
fn transition_instantaneous_candidate_has_unity_transient_input_jacobian() {
    let mut device = device(
        r#"
`include "disciplines.vams"
`default_transition 0.0
module transition_direct_jacobian(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ transition(V(p, n));
endmodule
"#,
    );
    device.set_analysis_type(2);

    assert_eq!(evaluate(&mut device, 0.0, 0.0), 0.0);
    device.advance_state();
    assert_eq!(evaluate(&mut device, 1.0, 1.0), 1.0);
    let jacobians = device
        .try_compute_jacobian()
        .expect("instantaneous transition Jacobian succeeds");
    assert_eq!(jacobians.len(), 4, "jacobians={jacobians:?}");
    assert!(
        jacobians
            .iter()
            .all(|entry| entry.value.abs().to_bits() == 1.0_f64.to_bits()),
        "instantaneous transition must track its input: {jacobians:?}"
    );
}

#[test]
fn slew_restarts_each_candidate_from_accepted_state() {
    let mut device = device(
        r#"
`include "disciplines.vams"
module slew_idempotent(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ slew(V(p, n), 0.5, -0.5);
endmodule
"#,
    );
    device.set_analysis_type(2);

    assert_eq!(evaluate(&mut device, 0.0, 0.0), 0.0);
    device.advance_state();
    assert_eq!(evaluate(&mut device, 1.0, 1.0), 0.5);
    device.advance_state();

    let first = evaluate(&mut device, 2.0, 1.0);
    let alternate = evaluate(&mut device, 2.0, 0.0);
    let repeated = evaluate(&mut device, 2.0, 1.0);
    assert_eq!(first.to_bits(), 1.0_f64.to_bits());
    assert_eq!(alternate.to_bits(), 0.0_f64.to_bits());
    assert_eq!(first.to_bits(), repeated.to_bits());
}

#[test]
fn slew_seeds_direct_transient_startup_from_the_first_input() {
    let mut device = device(
        r#"
`include "disciplines.vams"
module slew_uic_startup(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ slew(V(p, n), 1.0, -1.0);
endmodule
"#,
    );
    device.set_analysis_type(2);
    device.set_timestep(1.0);

    assert_eq!(evaluate(&mut device, 0.0, 5.0).to_bits(), 5.0_f64.to_bits());
    device.advance_state();
    assert_eq!(
        evaluate(&mut device, 1.0, 10.0).to_bits(),
        6.0_f64.to_bits()
    );
}

#[test]
fn slew_promotes_the_operating_point_before_first_transient_candidate() {
    let mut device = device(
        r#"
`include "disciplines.vams"
module slew_op_startup(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ slew(V(p, n), 1.0, -1.0);
endmodule
"#,
    );

    // The converged DC value is a candidate until transient integration is
    // activated. Activation must promote it atomically before evaluating the
    // first transient candidate.
    assert_eq!(evaluate(&mut device, 0.0, 5.0).to_bits(), 5.0_f64.to_bits());
    let op_jacobians = device
        .try_compute_jacobian()
        .expect("slew operating-point Jacobian succeeds");
    assert_eq!(op_jacobians.len(), 4, "jacobians={op_jacobians:?}");
    assert!(
        op_jacobians
            .iter()
            .all(|entry| entry.value.abs().to_bits() == 1.0_f64.to_bits()),
        "jacobians={op_jacobians:?}"
    );
    device.set_analysis_type(2);
    device.set_timestep(1.0);
    assert_eq!(
        evaluate(&mut device, 1.0, 10.0).to_bits(),
        6.0_f64.to_bits()
    );
}

#[test]
fn slew_dynamic_rate_supplies_saturated_jacobian_when_input_is_axis_independent() {
    let mut device = device(
        r#"
`include "disciplines.vams"
module slew_dynamic_rate_jacobian(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ slew(
        $abstime > 0.0 ? 10.0 : 0.0,
        1.0 + 0.25 * V(p, n),
        -(1.0 + 0.25 * V(p, n))
    );
endmodule
"#,
    );
    device.set_analysis_type(2);
    device.set_timestep(1.0);

    assert_eq!(evaluate(&mut device, 0.0, 4.0).to_bits(), 0.0_f64.to_bits());
    device.advance_state();
    assert_eq!(evaluate(&mut device, 1.0, 4.0).to_bits(), 2.0_f64.to_bits());
    let jacobians = device
        .try_compute_jacobian()
        .expect("dynamic slew rate Jacobian succeeds");
    assert_eq!(jacobians.len(), 4, "jacobians={jacobians:?}");
    assert!(
        jacobians
            .iter()
            .all(|entry| (entry.value.abs() - 0.25).abs() < 1.0e-12),
        "jacobians={jacobians:?}"
    );
}

#[test]
fn slew_rejects_invalid_dynamic_rate_without_mutating_accepted_state() {
    let mut device = device(
        r#"
`include "disciplines.vams"
module slew_invalid_dynamic_rate(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ slew(V(p, n), V(p, n), -1.0);
endmodule
"#,
    );
    device.set_analysis_type(2);

    device.set_time(0.0);
    device.update_voltages(&[-1.0]);
    let error = device
        .try_evaluate()
        .expect_err("nonpositive positive slew rate must fail closed");
    assert!(error.to_string().contains("slew"), "error={error}");

    // A valid replacement evaluation starts from a clean uninitialized state,
    // proving the rejected candidate was not accepted speculatively.
    assert_eq!(evaluate(&mut device, 0.0, 2.0).to_bits(), 2.0_f64.to_bits());
}

#[test]
fn slew_without_rates_is_an_exact_stateless_passthrough() {
    let mut device = device(
        r#"
`include "disciplines.vams"
module slew_passthrough(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ slew(V(p, n));
endmodule
"#,
    );
    device.set_analysis_type(2);

    for (time, voltage) in [(0.0, 5.0), (1.0, -7.0), (1.0, 11.0)] {
        assert_eq!(
            evaluate(&mut device, time, voltage).to_bits(),
            voltage.to_bits()
        );
    }
}

#[test]
fn slew_ac_and_noise_evaluation_are_read_only_and_checkpoint_ready() {
    let source = r#"
`include "disciplines.vams"
module slew_small_signal_read_only(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ slew(V(p, n), 1.0, -1.0);
endmodule
"#;

    for analysis in [1, 3] {
        let mut device = device(source);
        device.set_analysis_type(analysis);
        assert_eq!(evaluate(&mut device, 0.0, 5.0).to_bits(), 5.0_f64.to_bits());
        let checkpoint = device
            .checkpoint_state()
            .unwrap_or_else(|error| panic!("analysis {analysis} checkpoint failed: {error}"));
        assert_eq!(checkpoint.accepted.slew_filters.len(), 1);
        assert!(!checkpoint.accepted.slew_filters[0].initialized);
        assert_eq!(
            checkpoint.accepted.slew_filters[0].output.to_bits(),
            0.0_f64.to_bits()
        );
    }
}

#[test]
fn crossing_remains_visible_for_every_newton_reevaluation() {
    let mut device = device(
        r#"
`include "disciplines.vams"
module cross_idempotent(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ cross(V(p, n), +1);
endmodule
"#,
    );

    device.set_analysis_type(0);
    assert_eq!(evaluate(&mut device, 0.0, -1.0), 0.0);
    device.advance_state();

    device.set_analysis_type(2);
    let first = evaluate(&mut device, 1.0, 1.0);
    let repeated = evaluate(&mut device, 1.0, 1.0);
    assert_eq!(first.to_bits(), 1.0_f64.to_bits());
    assert_eq!(first.to_bits(), repeated.to_bits());
}

/// A reactive stamp between the value pass and acceptance leaves the `ddt`
/// history for `advance_state` to rotate.
///
/// A transient step is a nonlinear value pass, the small-signal observations
/// of that same point, and then the acceptance that commits it. An
/// observation may not reopen the integration candidate: it runs only the
/// assignment pass and the entries it was asked for, and this route's
/// reactive Jacobian is `dQ/dx` alone, so nothing in it republishes a
/// candidate the pass discarded and the acceptance rotates nothing. A 1 nF
/// capacitor driven 0 V, 1 V, 0.5 V, -0.25 V on 1 ns, 1 ns and 2 ns
/// backward-Euler steps then read +0.5005 A at t = 2 ns — a capacitor
/// charging from zero a second time — where its accepted history gives
/// -0.4995 A, and it stayed at the operating point for the whole walk.
#[test]
fn a_reactive_observation_before_acceptance_leaves_the_ddt_history_to_rotate() {
    const SOURCE: &str = r#"
`include "disciplines.vams"
module reactive_observation_before_accept(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1.0e-9;
    analog I(p, n) <+ c * ddt(V(p, n)) + V(p, n) * 1.0e-3;
endmodule
"#;

    let walk = |observe: bool| {
        let mut device = device(SOURCE);
        device.set_analysis_type(0);
        assert_eq!(evaluate(&mut device, 0.0, 0.0), 0.0);
        device.advance_state();

        device.set_analysis_type(2);
        let mut currents = Vec::new();
        for (time, dt, bias) in [
            (1.0e-9, 1.0e-9, 1.0),
            (2.0e-9, 1.0e-9, 0.5),
            (4.0e-9, 2.0e-9, -0.25),
        ] {
            device.set_timestep(dt);
            currents.push(evaluate(&mut device, time, bias));
            if observe {
                device
                    .try_stamp_reactive(&[bias], |_, _, _| {})
                    .expect("reactive observation of the point just evaluated");
            }
            device.advance_state();
        }
        currents
    };

    let observed = walk(true);
    let plain = walk(false);
    for (index, (with, without)) in observed.iter().zip(&plain).enumerate() {
        assert_eq!(
            with.to_bits(),
            without.to_bits(),
            "point {index}: {with} with the reactive observation, {without} without it"
        );
    }
    for (index, (current, expected)) in observed.iter().zip([1.001, -0.4995, -0.37525]).enumerate()
    {
        assert!(
            (current - expected).abs() <= 1.0e-12,
            "point {index}: expected {expected}, got {current} (walk {observed:?})"
        );
    }
}
