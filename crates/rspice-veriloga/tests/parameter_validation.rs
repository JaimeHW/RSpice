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

fn dependent_range_model() -> DeviceFixture {
    DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module dependent_ranges(p, n);
    inout p, n;
    electrical p, n;
    parameter real lower = 1.0;
    parameter real upper = 5.0;
    parameter real forbidden = 4.0;
    parameter real value = 3.0 from [lower:upper] exclude forbidden;
    analog I(p, n) <+ value * V(p, n);
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
fn compiled_parameters_preserve_dependent_range_references() {
    let model = dependent_range_model();
    let value = &model.parameters[3];

    assert_eq!(value.min, None);
    assert_eq!(value.max, None);
    assert_eq!(value.min_parameter, Some(0));
    assert_eq!(value.max_parameter, Some(1));
    assert_eq!(value.exclude_parameters, [2]);
}

#[test]
fn dependent_ranges_validate_the_completed_parameter_vector() {
    let model = dependent_range_model();

    let mut valid_in_any_order = model.device("DYNAMIC_OK", &[1, 0]);
    assert_eq!(valid_in_any_order.try_set_parameter("value", 8.0), Ok(true));
    assert_eq!(
        valid_in_any_order.try_set_parameter("upper", 10.0),
        Ok(true)
    );
    valid_in_any_order
        .try_resolve_parameter_defaults()
        .expect("completed overrides satisfy the dependent range");

    let mut tightened_lower = model.device("DYNAMIC_LOW", &[1, 0]);
    tightened_lower
        .try_set_parameter("lower", 3.5)
        .expect("scalar assignment is valid");
    let error = tightened_lower
        .try_resolve_parameter_defaults()
        .expect_err("later lower-bound overrides must invalidate an earlier value");
    assert!(error.to_string().contains("lower=3.5"), "{error}");

    let mut excluded = model.device("DYNAMIC_EXCLUDE", &[1, 0]);
    excluded
        .try_set_parameter("forbidden", 3.0)
        .expect("scalar assignment is valid");
    let error = excluded
        .try_resolve_parameter_defaults()
        .expect_err("a referenced exclusion must use its final instance value");
    assert!(error.to_string().contains("explicitly excluded"), "{error}");

    let mut empty = model.device("DYNAMIC_EMPTY", &[1, 0]);
    empty
        .try_set_parameter("lower", 6.0)
        .expect("scalar assignment is valid");
    let error = empty
        .try_resolve_parameter_defaults()
        .expect_err("inverted dependent bounds must be diagnosed");
    assert!(error.to_string().contains("range is empty"), "{error}");
}

#[test]
fn invalid_dependent_range_defaults_fail_device_construction() {
    let model = DeviceFixture::compile(
        r#"
module invalid_dependent_default(p, n);
    inout p, n;
    electrical p, n;
    parameter real lower = 2.0;
    parameter real value = 1.0 from [lower:inf);
    analog I(p, n) <+ value * V(p, n);
endmodule
"#,
    );
    let error = model
        .try_device("INVALID_DEFAULT", &[1, 0])
        .expect_err("dependent range violations must not instantiate");
    assert!(error.to_string().contains("lower=2"), "{error}");
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
    parameter real x = 0.5 from [lower + 0.1:1];
    analog I(p, n) <+ x * V(p, n);
endmodule
"#,
        )
        .expect_err("complex dependent ranges must never be silently ignored");
    assert!(
        dependent.to_string().contains("parameter-dependent"),
        "{dependent}"
    );

    let self_referential = compiler
        .compile(
            r#"
module self_referential_range(p, n);
    inout p, n;
    electrical p, n;
    parameter real x = 0.5 from [x:1];
    analog I(p, n) <+ x * V(p, n);
endmodule
"#,
        )
        .expect_err("self-referential ranges must be rejected");
    assert!(
        self_referential.to_string().contains("Circular dependency"),
        "{self_referential}"
    );

    let unresolved = compiler
        .compile(
            r#"
module unresolved_range(p, n);
    inout p, n;
    electrical p, n;
    parameter real x = 0.5 from [missing_bound:1];
    analog I(p, n) <+ x * V(p, n);
endmodule
"#,
        )
        .expect_err("unknown range expressions must not become unbounded");
    assert!(
        unresolved
            .to_string()
            .contains("constant expressions or direct parameter references"),
        "{unresolved}"
    );
}
