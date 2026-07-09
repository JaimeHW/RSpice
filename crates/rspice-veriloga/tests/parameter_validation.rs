mod support;

use rspice_veriloga::device::ParameterValueError;
use support::DeviceFixture;

fn parameter_model() -> DeviceFixture {
    DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module checked_parameters(p, n);
    inout p, n;
    electrical p, n;
    parameter integer count = 2 from [1:4] exclude 3;
    parameter real gain = 0.5 from (0:1];
    analog I(p, n) <+ count * gain * V(p, n);
endmodule
"#,
    )
}

#[test]
fn compiled_parameters_preserve_declared_constraints() {
    let model = parameter_model();
    let count = &model.parameters[0];
    assert!(count.is_integer);
    assert_eq!(count.min, Some(1.0));
    assert_eq!(count.max, Some(4.0));
    assert!(!count.min_exclusive);
    assert!(!count.max_exclusive);
    assert_eq!(count.exclude, [3.0]);

    let gain = &model.parameters[1];
    assert!(!gain.is_integer);
    assert!(gain.min_exclusive);
    assert!(!gain.max_exclusive);
}

#[test]
fn checked_parameter_assignment_rejects_invalid_values_without_mutation() {
    let model = parameter_model();
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(device.try_set_parameter("missing", 1.0), Ok(false));
    assert_eq!(device.try_set_parameter("count", 4.0), Ok(true));

    assert!(matches!(
        device.try_set_parameter("count", 2.5),
        Err(ParameterValueError::NonInteger { .. })
    ));
    assert!(matches!(
        device.try_set_parameter("count", 3.0),
        Err(ParameterValueError::Excluded { .. })
    ));
    assert!(matches!(
        device.try_set_parameter("count", 5.0),
        Err(ParameterValueError::OutOfRange { .. })
    ));
    assert!(matches!(
        device.try_set_parameter("gain", 0.0),
        Err(ParameterValueError::OutOfRange { .. })
    ));
    assert!(matches!(
        device.try_set_parameter("gain", f64::NAN),
        Err(ParameterValueError::NonFinite { .. })
    ));

    device.update_voltages(&[2.0]);
    let current = device.try_evaluate().expect("valid state evaluates")[0];
    assert_eq!(current.to_bits(), 4.0_f64.to_bits());
}

#[test]
fn unsupported_range_forms_fail_at_compile_time() {
    let compiler = rspice_veriloga::VerilogACompiler::default();
    let multiple = compiler
        .compile(
            r#"
module multiple_ranges(p, n);
    inout p, n;
    electrical p, n;
    parameter real x = 0.5 from [0:1] from [2:3];
    analog I(p, n) <+ x * V(p, n);
endmodule
"#,
        )
        .expect_err("multiple ranges must never be silently truncated");
    assert!(
        multiple.to_string().contains("multiple parameter"),
        "{multiple}"
    );

    let dependent = compiler
        .compile(
            r#"
module dependent_range(p, n);
    inout p, n;
    electrical p, n;
    parameter real lower = 0.0;
    parameter real x = 0.5 from [lower:1];
    analog I(p, n) <+ x * V(p, n);
endmodule
"#,
        )
        .expect_err("dependent ranges must never be silently ignored");
    assert!(
        dependent.to_string().contains("parameter-dependent"),
        "{dependent}"
    );
}
