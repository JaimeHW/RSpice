mod support;

use rspice_veriloga::vm::VmError;
use support::DeviceFixture;

fn assert_numeric_error(error: VmError, expected_context: &str) {
    match error {
        VmError::InvalidNumericResult(message) => assert!(
            message.contains(expected_context),
            "expected numeric error containing {expected_context:?}, got {message:?}"
        ),
        other => panic!("expected invalid numeric result, got {other:?}"),
    }
}

#[test]
fn nonfinite_contributions_are_reported_instead_of_zeroed() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module invalid_value(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ sqrt(V(p, n));
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.update_voltages(&[-1.0]);

    let error = device
        .try_evaluate()
        .expect_err("NaN contribution must be a runtime diagnostic");
    assert_numeric_error(error, "contribution 0");
}

#[test]
fn nonfinite_jacobians_are_reported_instead_of_dropped() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module invalid_jacobian(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ sqrt(V(p, n));
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    let error = device
        .try_stamp(&[0.0], |_, _, _| {}, |_, _| {})
        .expect_err("infinite derivative must be a runtime diagnostic");
    assert_numeric_error(error, "Jacobian 0:0");
}

#[test]
fn nonfinite_reactive_jacobians_are_reported_instead_of_dropped() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module invalid_reactive_jacobian(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ ddt(sqrt(V(p, n)));
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    let error = device
        .try_stamp_reactive(&[0.0], |_, _, _| {})
        .expect_err("infinite reactive derivative must be a runtime diagnostic");
    assert_numeric_error(error, "reactive Jacobian 0:0");
}

#[test]
fn invalid_noise_power_is_reported_instead_of_suppressed() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module invalid_noise(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ white_noise(V(p, n), "invalid");
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    let error = device
        .try_noise_sources(&[-1.0])
        .expect_err("negative noise power must be a runtime diagnostic");
    assert_numeric_error(error, "negative value");
}

#[test]
fn nonfinite_noise_exponents_are_reported() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module invalid_noise_exponent(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ flicker_noise(1.0, sqrt(V(p, n)), "invalid");
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    let error = device
        .try_noise_sources(&[-1.0])
        .expect_err("NaN noise exponent must be a runtime diagnostic");
    assert_numeric_error(error, "exponent");
}
