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
