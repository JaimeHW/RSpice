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
fn derivative_jacobians_match_first_transient_candidates_and_accepted_history() {
    let model = DeviceFixture::compile(
        "module first_derivative(p,n); inout p,n; electrical p,n;
         analog I(p,n)<+ddt(V(p,n)*V(p,n)); endmodule",
    );
    for (name, coefficients) in [
        (
            "BE",
            IntegrationCoefficients {
                active: true,
                derivative_scale: 4.0,
                previous_value_scale: 4.0,
                older_value_scale: 0.0,
                previous_derivative_scale: 0.0,
            },
        ),
        (
            "TR",
            IntegrationCoefficients {
                active: true,
                derivative_scale: 8.0,
                previous_value_scale: 8.0,
                older_value_scale: 0.0,
                previous_derivative_scale: 1.0,
            },
        ),
        (
            "Gear2",
            IntegrationCoefficients {
                active: true,
                derivative_scale: 6.0,
                previous_value_scale: 8.0,
                older_value_scale: -2.0,
                previous_derivative_scale: 0.0,
            },
        ),
    ] {
        let mut device = model.device(name, &[1, 0]);
        device.set_analysis_type(2);
        device.set_integration_coefficients(coefficients);
        for initialized in [false, true] {
            let voltage = 1.5;
            let mut jacobian = 0.0;
            device
                .try_stamp(
                    &[voltage],
                    |row, col, value| {
                        assert_eq!((row, col), (0, 0));
                        jacobian += value;
                    },
                    |_, _| {},
                )
                .unwrap();
            let mut values = [0.0; 2];
            for (value, delta) in values.iter_mut().zip([-1e-6, 1e-6]) {
                device.update_voltages(&[voltage + delta]);
                *value = device.try_evaluate().unwrap()[0];
            }
            let numerical = (values[1] - values[0]) / 2e-6;
            let expected = if initialized {
                2.0 * voltage * coefficients.derivative_scale
            } else {
                0.0
            };
            assert!(
                (numerical - expected).abs() < 1e-7,
                "{name} initialized={initialized}: finite difference {numerical}, expected {expected}"
            );
            assert!(
                (jacobian - numerical).abs() < 1e-7,
                "{name} initialized={initialized}: Jacobian {jacobian}, finite difference {numerical}"
            );
            device.update_voltages(&[voltage]);
            device.try_evaluate().unwrap();
            device.advance_state();
        }
    }
}

#[test]
fn derivative_curvature_preserves_initialization_and_complex_ac_transfer() {
    let model = DeviceFixture::compile(
        "module curvature(p,n); inout p,n; electrical p,n;
         analog I(p,n)<+ddx(ddt(V(p,n)*V(p,n)*V(p,n)),V(p,n)); endmodule",
    );
    let mut device = model.device("A", &[1, 0]);
    device.set_analysis_type(2);
    device.set_timestep(0.25);
    for initialized in [false, true] {
        let mut jacobian = 0.0;
        device
            .try_stamp(&[1.5], |_, _, value| jacobian += value, |_, _| {})
            .unwrap();
        assert_close(
            jacobian,
            if initialized { 36.0 } else { 0.0 },
            "DDT curvature",
        );
        assert_close(
            device.try_evaluate().unwrap()[0],
            if initialized { 27.0 } else { 0.0 },
            "DDT first partial",
        );
        device.advance_state();
    }
    // AC is independent of whether transient history has been initialized.
    for mut device in [model.device("FRESH", &[1, 0]), device] {
        device.set_analysis_type(1);
        let mut jacobian = (0.0, 0.0);
        device
            .try_stamp_small_signal_complex(&[1.5], 2.0, |_, _, re, im| {
                jacobian.0 += re;
                jacobian.1 += im;
            })
            .unwrap();
        assert_eq!(jacobian.0, 0.0);
        assert_close(jacobian.1, 18.0 * std::f64::consts::TAU, "AC curvature");
    }
}

#[test]
fn derivative_companions_preserve_finite_large_value_differences() {
    let model = DeviceFixture::compile(
        "module large_derivative(p,n); inout p,n; electrical p,n;
         analog I(p,n)<+ddt(V(p,n)); endmodule",
    );
    for (scale, previous_scale, older_scale, derivative_scale) in [
        (4.0, 4.0, 0.0, 0.0),
        (8.0, 8.0, 0.0, 1.0),
        (6.0, 8.0, -2.0, 0.0),
    ] {
        let mut device = model.device("A", &[1, 0]);
        device.set_analysis_type(2);
        device.set_integration_coefficients(IntegrationCoefficients {
            active: true,
            derivative_scale: scale,
            previous_value_scale: previous_scale,
            older_value_scale: older_scale,
            previous_derivative_scale: derivative_scale,
        });
        let previous = 1e308_f64;
        device.update_voltages(&[previous]);
        assert_eq!(device.try_evaluate().unwrap()[0], 0.0);
        device.advance_state();
        let current = f64::from_bits(previous.to_bits() + 1);
        device.update_voltages(&[current]);
        assert_eq!(
            device.try_evaluate().unwrap()[0],
            scale * (current - previous)
        );
    }
}

#[test]
fn circular_integrator_jacobians_follow_the_initialization_and_wrap_branch() {
    let model = DeviceFixture::compile(
        r#"
module circular_jacobian(p,n,x,m,c,o);
    inout p,n,x,m,c,o; electrical p,n,x,m,c,o;
    analog I(p,n) <+ idtmod(V(x,n),V(c,n),V(m,n),V(o,n));
endmodule
"#,
    );
    let check = |device: &mut rspice_veriloga::device::VerilogADevice,
                 voltages: [f64; 5],
                 expected: [f64; 4]| {
        let mut jacobian = [0.0; 5];
        device
            .try_stamp(
                &voltages,
                |row, col, value| {
                    assert_eq!(row, 0);
                    jacobian[col] += value;
                },
                |_, _| {},
            )
            .unwrap();
        for (column, expected) in (1..5).zip(expected) {
            let mut values = [0.0; 2];
            for (value, delta) in values.iter_mut().zip([-1.0e-6, 1.0e-6]) {
                let mut probe = voltages;
                probe[column] += delta;
                device.update_voltages(&probe);
                *value = device.try_evaluate().unwrap()[0];
            }
            let finite_difference = (values[1] - values[0]) / 2.0e-6;
            assert!(
                (finite_difference - expected).abs() < 1.0e-8,
                "column {column}: finite difference {finite_difference}, expected {expected}"
            );
            assert_close(
                jacobian[column],
                expected,
                &format!("Jacobian column {column}"),
            );
        }
        device.update_voltages(&voltages);
        device.try_evaluate().unwrap();
    };
    let mut device = model.device("A1", &[1, 0, 2, 3, 4, 5]);
    check(
        &mut device,
        [0.0, 1.5, 3.0, 5.0, 0.25],
        [0.0, -1.0, 1.0, 0.0],
    );
    device.advance_state();
    device.set_analysis_type(2);
    device.set_timestep(0.25);
    for (modulus, modulus_gain) in [(3.0, -1.0), (2.0, -2.0), (4.0, -1.0)] {
        check(
            &mut device,
            [0.0, 1.5, modulus, 5.0, 0.25],
            [0.25, modulus_gain, 0.0, 0.0],
        );
    }
    let mut direct = model.device("A2", &[1, 0, 2, 3, 4, 5]);
    direct.set_analysis_type(2);
    direct.set_integration_coefficients(IntegrationCoefficients {
        active: true,
        derivative_scale: 8.0,
        previous_value_scale: 8.0,
        older_value_scale: 0.0,
        previous_derivative_scale: 1.0,
    });
    check(
        &mut direct,
        [0.0, 1.5, 3.0, 5.0, 0.25],
        [0.25, -1.0, 1.0, 0.0],
    );
}

#[test]
fn integral_jacobians_preserve_dc_initialization_and_complex_ac_transfer() {
    for operator in [
        "idt(V(x,n),V(c,n)*V(c,n))",
        "idtmod(V(x,n),V(c,n)*V(c,n),V(m,n),0.25)",
    ] {
        let source = format!(
            "module integral(p,n,x,c,m); inout p,n,x,c,m; electrical p,n,x,c,m;
            analog I(p,n)<+{operator}; endmodule"
        );
        let model = DeviceFixture::compile(&source);
        let wrapped = operator.starts_with("idtmod");
        let bias = [0.0, 1.5, 5.0, 3.0];
        for (coefficients, input_gain, ic_gain) in [
            (IntegrationCoefficients::inactive(), 0.0, 10.0),
            (
                IntegrationCoefficients {
                    active: true,
                    derivative_scale: 8.0,
                    previous_value_scale: 8.0,
                    older_value_scale: 0.0,
                    previous_derivative_scale: 1.0,
                },
                0.25,
                10.0,
            ),
        ] {
            let mut device = model.device("A1", &[1, 0, 2, 3, 4]);
            device.set_analysis_type(if coefficients.active { 2 } else { 0 });
            device.set_integration_coefficients(coefficients);
            let mut jacobian = [0.0; 4];
            device
                .try_stamp(
                    &bias,
                    |row, col, value| {
                        assert_eq!(row, 0);
                        jacobian[col] += value;
                    },
                    |_, _| {},
                )
                .unwrap();
            assert_eq!(
                jacobian,
                [0.0, input_gain, ic_gain, if wrapped { -8.0 } else { 0.0 }],
                "{operator}"
            );
        }
        let mut device = model.device("A1", &[1, 0, 2, 3, 4]);
        device.set_analysis_type(1);
        let frequency = 2.0;
        let mut jacobian = [(0.0, 0.0); 4];
        device
            .try_stamp_small_signal_complex(&bias, frequency, |row, col, re, im| {
                assert_eq!(row, 0);
                jacobian[col].0 += re;
                jacobian[col].1 += im;
            })
            .unwrap();
        assert_close(
            jacobian[1].1,
            -1.0 / (std::f64::consts::TAU * frequency),
            "AC integrand derivative",
        );
        assert_eq!(jacobian[1].0, 0.0);
        assert_eq!(
            jacobian[2],
            (0.0, 0.0),
            "AC holds the initial condition fixed"
        );
        assert_eq!(jacobian[3], (if wrapped { -8.0 } else { 0.0 }, 0.0));
    }
}

#[test]
fn integral_initial_conditions_retain_higher_order_derivatives() {
    for expression in [
        "idt(0.0,V(c,n)*V(c,n)*V(c,n))",
        "idtmod(0.0,V(c,n)*V(c,n)*V(c,n),7.0,0.25)",
    ] {
        let model = DeviceFixture::compile(&format!(
            "module curvature(p,n,c); inout p,n,c; electrical p,n,c;
            analog I(p,n)<+ddx({expression},V(c,n)); endmodule"
        ));
        let mut device = model.device("A1", &[1, 0, 2]);
        let mut jacobian = 0.0;
        device
            .try_stamp(
                &[0.0, 5.0],
                |row, col, value| {
                    assert_eq!((row, col), (0, 1));
                    jacobian += value;
                },
                |_, _| {},
            )
            .unwrap();
        assert_eq!(jacobian, 30.0, "{expression}");
        assert_eq!(device.try_evaluate().unwrap(), vec![75.0]);
    }
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

#[test]
fn integrators_preserve_large_initial_conditions_and_local_wrapped_history() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module large_integral(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ idt(0.0, 1.0e308);
        I(p, n) <+ idtmod(0.0, 1.0e16, 1.0, -0.5);
        I(p, n) <+ idtmod(0.0, 1.0e16, 1.0, 0.25);
    end
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.set_analysis_type(2);
    for (time, coefficients) in [
        (0.0, IntegrationCoefficients::inactive()),
        (0.25, IntegrationCoefficients::backward_euler(0.25).unwrap()),
        (
            0.5,
            IntegrationCoefficients {
                active: true,
                derivative_scale: 6.0,
                previous_value_scale: 8.0,
                older_value_scale: -2.0,
                previous_derivative_scale: 0.0,
            },
        ),
    ] {
        device.set_time(time);
        device.set_integration_coefficients(coefficients);
        let values = device.try_evaluate().expect("finite integral candidates");
        assert_eq!(values, vec![1.0e308, 0.0, 1.0], "time={time}");
        assert_eq!(device.try_evaluate().unwrap(), values, "Newton retry");
        device.advance_state();
    }
}

#[test]
fn idtmod_dynamic_modulus_preserves_accumulated_integral() {
    use rspice_veriloga::device::VerilogADeviceCheckpoint;
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module dynamic_modulus(p, n, o);
    inout p, n, o;
    electrical p, n, o;
    analog I(p, n) <+ idtmod(0.0, 5.0, V(p, n), V(o, n));
endmodule
"#,
    );
    // Both explicit acceptance and operating-point promotion must retain 5,
    // although the visible initial phase is only 1.
    for accept_operating_point in [false, true] {
        let mut device = model.device("A1", &[1, 0, 2]);
        device.set_analysis_type(2);
        device.set_timestep(0.0);
        device.update_voltages(&[2.0, 0.0]);
        assert_close(device.try_evaluate().unwrap()[0], 1.0, "initial phase");
        if accept_operating_point {
            device.advance_state();
        }
        device.set_time(0.25);
        device.set_timestep(0.25);
        for (modulus, offset, expected) in [(3.0, 0.0, 2.0), (4.0, 0.25, 1.0), (2.0, 1.5, 3.0)] {
            device.update_voltages(&[modulus, offset]);
            for _ in 0..2 {
                assert_close(device.try_evaluate().unwrap()[0], expected, "Newton retry");
                assert!(
                    device.checkpoint_state().is_err(),
                    "an in-flight candidate cannot be serialized"
                );
            }
        }
        device.update_voltages(&[0.0, 0.0]);
        assert!(device.try_evaluate().is_err());
        assert!(
            device.try_advance_state().is_err(),
            "a failed retry cannot be accepted"
        );
        device.update_voltages(&[3.0, 0.0]);
        assert_close(
            device.try_evaluate().unwrap()[0],
            2.0,
            "retry after invalid modulus",
        );
        device.advance_state();
        let checkpoint = device.checkpoint_state().unwrap();
        let decoded = VerilogADeviceCheckpoint::from_words(
            checkpoint.instance_name.clone(),
            checkpoint.model_name.clone(),
            checkpoint.source_digest.clone(),
            checkpoint.shape_identity.clone(),
            &checkpoint.to_words(),
        )
        .unwrap();
        assert_eq!(decoded, checkpoint);
        let mut restored = model.device("A1", &[1, 0, 2]);
        let mut missing = decoded.clone();
        missing.accepted.idtmod_origins.clear();
        assert!(restored.validate_checkpoint_state(&missing).is_err());
        restored.validate_checkpoint_state(&decoded).unwrap();
        restored.apply_validated_checkpoint_state(&decoded);
        for candidate in [&mut device, &mut restored] {
            candidate.set_analysis_type(2);
            candidate.set_time(0.5);
            candidate.set_timestep(0.25);
            candidate.update_voltages(&[2.0, 0.0]);
            assert_close(
                candidate.try_evaluate().unwrap()[0],
                1.0,
                "checkpoint continuation",
            );
            candidate.advance_state();
            candidate.try_begin_analysis(2).unwrap();
            candidate.update_voltages(&[4.0, 0.0]);
            assert_close(candidate.try_evaluate().unwrap()[0], 1.0, "fresh analysis");
        }
    }
}

#[test]
fn idtmod_direct_transient_retains_small_increments_after_a_large_initial_value() {
    let model = DeviceFixture::compile(
        r#"
module direct_circular(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ idtmod(1.0, 1.0e300, V(p, n), 0.0);
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.set_analysis_type(2);
    for (time, modulus, expected) in [(0.25, 1.0, 0.25), (0.5, 3.0, (1.0e300_f64 % 3.0) + 0.5)] {
        device.set_time(time);
        device.set_timestep(0.25);
        device.update_voltages(&[modulus]);
        assert_eq!(device.try_evaluate().unwrap()[0], expected);
        device.advance_state();
    }
}
