mod support;

use rspice_veriloga::vm::IntegrationCoefficients;
use support::DeviceFixture;

fn assert_close(actual: f64, expected: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= 1.0e-12,
        "{label}: expected {expected}, got {actual}"
    );
}

#[test]
fn analog_integrators_follow_solver_companion_coefficients() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module integration_contract(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ ddt(V(p, n));
        I(p, n) <+ idt(2.0 * $abstime, 0.0);
    end
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.set_analysis_type(2);

    device.set_time(0.0);
    device.set_timestep(0.0);
    device.update_voltages(&[1.0]);
    let initial = device.try_evaluate().expect("initial evaluation");
    assert_close(initial[0], 0.0, "initial ddt");
    assert_close(initial[1], 0.0, "initial idt");
    device.advance_state();

    device.set_time(0.5);
    device.set_timestep(0.5);
    device.set_integration_coefficients(IntegrationCoefficients {
        active: true,
        derivative_scale: 4.0,
        previous_value_scale: 4.0,
        older_value_scale: 0.0,
        previous_derivative_scale: 1.0,
    });
    device.update_voltages(&[3.0]);
    let trapezoidal = device.try_evaluate().expect("trapezoidal evaluation");
    assert_close(trapezoidal[0], 8.0, "trapezoidal ddt");
    assert_close(trapezoidal[1], 0.25, "trapezoidal idt");
    let repeated = device.try_evaluate().expect("repeated Newton evaluation");
    assert_eq!(trapezoidal, repeated, "candidate state must be idempotent");
    device.advance_state();

    device.set_time(1.0);
    device.set_timestep(0.5);
    device.set_integration_coefficients(IntegrationCoefficients {
        active: true,
        derivative_scale: 3.0,
        previous_value_scale: 4.0,
        older_value_scale: -1.0,
        previous_derivative_scale: 0.0,
    });
    device.update_voltages(&[6.0]);
    let gear2 = device.try_evaluate().expect("Gear-2 evaluation");
    assert_close(gear2[0], 7.0, "Gear-2 ddt");
    assert_close(gear2[1], 1.0, "Gear-2 idt");
}

#[test]
fn first_transient_step_uses_the_operating_point_as_history() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module operating_point_history(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ ddt(V(p, n));
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.set_analysis_type(2);

    device.set_time(0.0);
    device.set_timestep(0.0);
    device.update_voltages(&[5.0]);
    assert_close(
        device.try_evaluate().expect("operating-point evaluation")[0],
        0.0,
        "operating-point ddt",
    );

    // The operating point is intentionally not accepted as a transient
    // step. Starting integration must still seed its value as history.
    device.set_time(0.25);
    device.set_timestep(0.25);
    device.update_voltages(&[6.0]);
    assert_close(
        device.try_evaluate().expect("first transient evaluation")[0],
        4.0,
        "first-step ddt",
    );
}

#[test]
fn rejected_first_integration_candidate_does_not_initialize_accepted_history() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module rejected_first_candidate(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ ddt(V(p, n));
        I(p, n) <+ idt(V(p, n), 10.0);
        I(p, n) <+ idtmod(V(p, n), 0.25, 2.0, 0.0);
    end
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.set_analysis_type(2);
    device.set_timestep(0.5);

    device.update_voltages(&[4.0]);
    let rejected = device.try_evaluate().expect("first candidate evaluation");
    assert_close(rejected[0], 0.0, "first-candidate ddt");
    assert_close(rejected[1], 12.0, "first-candidate idt");
    assert_close(rejected[2], 0.25, "first-candidate idtmod");

    device.update_voltages(&[6.0]);
    let accepted = device
        .try_evaluate()
        .expect("replacement candidate evaluation");
    assert_close(accepted[0], 0.0, "replacement-candidate ddt");
    assert_close(accepted[1], 13.0, "replacement-candidate idt");
    assert_close(accepted[2], 1.25, "replacement-candidate idtmod");
    device.advance_state();

    device.set_time(0.5);
    device.update_voltages(&[8.0]);
    let next = device
        .try_evaluate()
        .expect("next accepted-history evaluation");
    assert_close(next[0], 4.0, "accepted-history ddt");
    assert_close(next[1], 17.0, "accepted-history idt");
    assert_close(next[2], 1.25, "accepted-history idtmod");
}

#[test]
fn direct_transient_seeds_nonzero_initial_history_before_gear2() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module direct_transient_history(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ ddt(V(p, n));
        I(p, n) <+ idt(1.0, 0.5);
        I(p, n) <+ idtmod(1.0, 0.5, 1.0, 0.0);
    end
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.set_analysis_type(2);
    device.set_time(0.5);
    device.set_integration_coefficients(
        IntegrationCoefficients::backward_euler(0.5).expect("a representable interval"),
    );
    device.update_voltages(&[2.0]);

    let first = device.try_evaluate().expect("direct first transient step");
    assert_close(first[0], 0.0, "direct first-step ddt");
    assert_close(first[1], 1.0, "direct first-step idt");
    assert_close(first[2], 0.0, "direct first-step idtmod");
    device.advance_state();

    let checkpoint = device
        .checkpoint_state()
        .expect("capture direct first-step history");
    assert_eq!(checkpoint.accepted.state_values_prev, vec![2.0, 1.0, 0.0]);
    assert_eq!(
        checkpoint.accepted.state_values_older,
        vec![2.0, 0.5, -0.5],
        "startup-seeded logical older lanes must be accepted atomically"
    );

    device.set_time(1.0);
    device.set_integration_coefficients(IntegrationCoefficients {
        active: true,
        derivative_scale: 3.0,
        previous_value_scale: 4.0,
        older_value_scale: -1.0,
        previous_derivative_scale: 0.0,
    });
    device.update_voltages(&[3.0]);
    let gear2 = device
        .try_evaluate()
        .expect("Gear-2 after direct first step");
    assert_close(gear2[0], 3.0, "direct-start Gear-2 ddt");
    assert_close(gear2[1], 1.5, "direct-start Gear-2 idt");
    assert_close(gear2[2], 0.5, "direct-start Gear-2 idtmod");
}

#[test]
fn idtmod_rebases_multistep_history_across_wraps_and_checkpoint_restore() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module idtmod_gear_history(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ idtmod(1.0, 0.0, 1.0, 0.0);
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.set_analysis_type(2);

    for (time, expected) in [(0.6, 0.6), (1.2, 0.2)] {
        device.set_time(time);
        device.set_integration_coefficients(
            IntegrationCoefficients::backward_euler(0.6).expect("a representable interval"),
        );
        let phase = device.try_evaluate().expect("backward-Euler phase")[0];
        assert_close(phase, expected, "backward-Euler wrapped phase");
        device.advance_state();
    }

    let checkpoint = device
        .checkpoint_state()
        .expect("capture accepted common-branch history");
    assert_close(
        checkpoint.accepted.state_values_prev[0],
        0.2,
        "checkpoint wrapped previous lane",
    );
    assert_close(
        checkpoint.accepted.state_values_older[0],
        -0.4,
        "checkpoint rebased older lane",
    );

    let mut restored = model.device("A1", &[1, 0]);
    restored
        .validate_checkpoint_state(&checkpoint)
        .expect("checkpoint matches a fresh instance");
    restored.apply_validated_checkpoint_state(&checkpoint);

    let gear2 = IntegrationCoefficients {
        active: true,
        derivative_scale: 2.5,
        previous_value_scale: 10.0 / 3.0,
        older_value_scale: -5.0 / 6.0,
        previous_derivative_scale: 0.0,
    };
    for candidate in [&mut device, &mut restored] {
        candidate.set_analysis_type(2);
        candidate.set_time(1.8);
        candidate.set_integration_coefficients(gear2);
        let phase = candidate.try_evaluate().expect("Gear-2 wrapped phase")[0];
        assert_close(phase, 0.8, "Gear-2 phase after a wrap");
        assert_eq!(
            phase.to_bits(),
            candidate.try_evaluate().expect("repeated Gear-2 phase")[0].to_bits(),
            "repeated Newton evaluation must be idempotent",
        );
    }
}

#[test]
fn idtmod_rejects_an_explicit_nonpositive_modulus() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module invalid_idtmod_modulus(p, n);
    inout p, n;
    electrical p, n;
    parameter real modulus = 1.0;
    analog I(p, n) <+ idtmod(1.0, 0.0, modulus, 0.0);
endmodule
"#,
    );

    for modulus in [0.0, -1.0] {
        let mut device = model.device("A1", &[1, 0]);
        assert!(device.set_parameter("modulus", modulus));
        device.set_analysis_type(2);
        device.set_timestep(0.5);
        let error = device
            .try_evaluate()
            .expect_err("an explicitly supplied modulus must be positive");
        assert!(
            error
                .to_string()
                .contains("modulus must be finite and greater than zero"),
            "unexpected modulus={modulus} error: {error}"
        );
    }
}
