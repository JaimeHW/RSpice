mod support;

use rspice_veriloga::vm::{IntegrationCoefficients, VmError};
use support::DeviceFixture;

fn assert_configuration_error(error: VmError, expected: &str) {
    match error {
        VmError::InvalidRuntimeConfiguration(message) => assert!(
            message.contains(expected),
            "expected configuration error containing {expected:?}, got {message:?}"
        ),
        other => panic!("expected invalid runtime configuration, got {other:?}"),
    }
}

fn context_model() -> DeviceFixture {
    DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module runtime_context(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ $temperature + $abstime + analysis("dc");
endmodule
"#,
    )
}

#[test]
fn scalar_runtime_updates_reject_invalid_values_without_mutation() {
    let model = context_model();
    let mut device = model.device("A1", &[1, 0]);

    assert_configuration_error(
        device
            .try_set_temperature(f64::NAN)
            .expect_err("NaN temperature must fail"),
        "temperature",
    );
    assert_configuration_error(
        device
            .try_set_temperature(0.0)
            .expect_err("absolute-zero temperature must fail"),
        "temperature",
    );
    assert_configuration_error(
        device
            .try_set_time(-1.0)
            .expect_err("negative simulation time must fail"),
        "simulation time",
    );
    assert_configuration_error(
        device
            .try_set_timestep(f64::INFINITY)
            .expect_err("infinite timestep must fail"),
        "timestep",
    );
    assert_configuration_error(
        device
            .try_set_analysis_type(5)
            .expect_err("unknown analysis type must fail"),
        "analysis type",
    );

    let value = device
        .try_evaluate()
        .expect("valid context remains usable after rejected updates")[0];
    assert!((value - 301.15).abs() <= 1.0e-12, "value: {value}");
}

#[test]
fn multiplicity_rejects_invalid_values_without_mutation() {
    let model = context_model();
    let mut device = model.device("A1", &[1, 0]);

    for value in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert_configuration_error(
            device
                .try_set_multiplicity(value)
                .expect_err("invalid multiplicity must fail"),
            "multiplicity",
        );
        assert_eq!(device.multiplicity(), 1.0);
    }

    device
        .try_set_multiplicity(2.5)
        .expect("positive finite multiplicity is valid");
    assert_eq!(device.multiplicity(), 2.5);
}

#[test]
fn companion_coefficients_are_validated_before_installation() {
    let model = context_model();
    let mut device = model.device("A1", &[1, 0]);

    let invalid_active = IntegrationCoefficients {
        active: true,
        derivative_scale: 0.0,
        previous_value_scale: 1.0,
        older_value_scale: 0.0,
        previous_derivative_scale: 0.0,
    };
    assert_configuration_error(
        device
            .try_set_integration_coefficients(invalid_active)
            .expect_err("active zero derivative scale must fail"),
        "derivative scale",
    );

    let invalid_inactive = IntegrationCoefficients {
        active: false,
        derivative_scale: 0.0,
        previous_value_scale: 1.0,
        older_value_scale: 0.0,
        previous_derivative_scale: 0.0,
    };
    assert_configuration_error(
        device
            .try_set_integration_coefficients(invalid_inactive)
            .expect_err("inactive nonzero scales must fail"),
        "inactive integration",
    );

    let nonfinite = IntegrationCoefficients {
        active: true,
        derivative_scale: 1.0,
        previous_value_scale: f64::NAN,
        older_value_scale: 0.0,
        previous_derivative_scale: 0.0,
    };
    assert_configuration_error(
        device
            .try_set_integration_coefficients(nonfinite)
            .expect_err("nonfinite companion scale must fail"),
        "finite",
    );
}

#[test]
fn solver_index_mappings_require_exact_non_ground_storage() {
    let internal_model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module internal_mapping(p, n);
    inout p, n;
    electrical p, n, middle;
    analog begin
        I(p, middle) <+ V(p, middle);
        I(middle, n) <+ V(middle, n);
    end
endmodule
"#,
    );
    let mut internal = internal_model.device("A1", &[1, 0]);
    assert_configuration_error(
        internal
            .try_set_internal_node_indices(&[])
            .expect_err("missing internal-node mapping must fail"),
        "exactly 1",
    );
    assert_configuration_error(
        internal
            .try_set_internal_node_indices(&[0])
            .expect_err("ground internal-node mapping must fail"),
        "ground",
    );
    assert_eq!(internal.internal_node_index(0), Some(0));
    internal
        .try_set_internal_node_indices(&[2])
        .expect("complete internal-node mapping is valid");
    assert_eq!(internal.internal_node_index(0), Some(2));

    let branch_model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module branch_mapping(p, n);
    inout p, n;
    electrical p, n;
    analog V(p, n) <+ 1.0;
endmodule
"#,
    );
    let mut branch = branch_model.device("A2", &[1, 0]);
    assert_configuration_error(
        branch
            .try_set_branch_current_indices(&[])
            .expect_err("missing branch-current mapping must fail"),
        "exactly 1",
    );
    assert_configuration_error(
        branch
            .try_set_branch_current_indices(&[0])
            .expect_err("ground branch-current mapping must fail"),
        "ground",
    );
    assert_eq!(branch.branch_current_index(0), Some(0));
    branch
        .try_set_branch_current_indices(&[2])
        .expect("complete branch-current mapping is valid");
    assert_eq!(branch.branch_current_index(0), Some(2));
}
