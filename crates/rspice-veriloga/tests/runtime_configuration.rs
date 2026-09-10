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

    let nonaffine = IntegrationCoefficients {
        active: true,
        derivative_scale: 3.0,
        previous_value_scale: 4.0,
        older_value_scale: 0.0,
        previous_derivative_scale: 0.0,
    };
    assert_configuration_error(
        device
            .try_set_integration_coefficients(nonaffine)
            .expect_err("nonaffine history scales must fail"),
        "must sum to the derivative scale",
    );

    let catastrophic_cancellation = IntegrationCoefficients {
        active: true,
        derivative_scale: 1.0,
        previous_value_scale: f64::MAX,
        older_value_scale: -f64::MAX,
        previous_derivative_scale: 0.0,
    };
    assert_configuration_error(
        device
            .try_set_integration_coefficients(catastrophic_cancellation)
            .expect_err("catastrophically cancelling history scales must fail"),
        "must sum to the derivative scale",
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

#[test]
fn simparam_values_follow_configuration_and_missing_value_policy() {
    use rspice_veriloga_runtime::{GeneratedSimulationParameters, SimulationParameter};
    for (name, default) in [("gmin", 1e-12), ("tnom", 27.0), ("simulatorVersion", 1.0)] {
        let model = DeviceFixture::compile(&format!(
            "module query(p); inout p; electrical p; analog I(p)<+$simparam(\"{name}\",123.0); endmodule"
        ));
        let mut device = model.device("QUERY", &[1]);
        assert_eq!(device.try_evaluate().unwrap(), [default]);
        let mut parameters = GeneratedSimulationParameters::default();
        let parameter = SimulationParameter::from_name(name).unwrap();
        for value in [Some(0.0), Some(2.5), None, Some(default)] {
            parameters.try_set(parameter, value).unwrap();
            device.set_simulation_parameters(parameters);
            assert_eq!(
                device.try_evaluate().unwrap(),
                [value.unwrap_or(123.0)],
                "{name}"
            );
        }
    }
    let model = DeviceFixture::compile(
        "module query(p); inout p; electrical p; analog I(p)<+$simparam(\"imax\"); endmodule",
    );
    let mut device = model.device("REQUIRED", &[1]);
    assert!(device.try_evaluate().is_err());
    let mut parameters = GeneratedSimulationParameters::default();
    parameters
        .try_set(SimulationParameter::Imax, Some(8.0))
        .unwrap();
    device.set_simulation_parameters(parameters);
    assert_eq!(device.try_evaluate().unwrap(), [8.0]);
    parameters.try_set(SimulationParameter::Imax, None).unwrap();
    device.set_simulation_parameters(parameters);
    assert!(device.try_evaluate().is_err());
    assert!(rspice_veriloga::VerilogACompiler::default().compile_runtime(
        "module query(p); inout p; electrical p; analog I(p)<+$simparam(\"unavailable_query\"); endmodule", None).is_err());
}

#[test]
fn simparam_fallbacks_preserve_derivatives_and_skip_untaken_domain_errors() {
    use rspice_veriloga_runtime::{GeneratedSimulationParameters, SimulationParameter};
    for order in 0..=2 {
        let mut expression = "$simparam(\"imax\",V(p)*V(p)*V(p))".to_owned();
        for _ in 0..order {
            expression = format!("ddx({expression},V(p))");
        }
        let model = DeviceFixture::compile(&format!(
            "module query(p); inout p; electrical p; real y; analog begin y={expression}; I(p)<+y; end endmodule"
        ));
        let mut device = model.device("FALLBACK", &[1]);
        for configured in [None, Some(0.0), Some(9.0), None] {
            let mut parameters = GeneratedSimulationParameters::default();
            parameters
                .try_set(SimulationParameter::Imax, configured)
                .unwrap();
            device.set_simulation_parameters(parameters);
            for v in [0.25_f64, 2.0, -3.0] {
                let (value, slope) = if let Some(value) = configured {
                    (if order == 0 { value } else { 0.0 }, 0.0)
                } else {
                    match order {
                        0 => (v.powi(3), 3.0 * v * v),
                        1 => (3.0 * v * v, 6.0 * v),
                        _ => (6.0 * v, 6.0),
                    }
                };
                device.update_voltages(&[v]);
                assert_eq!(device.try_evaluate().unwrap(), [value]);
                assert_eq!(
                    device
                        .try_compute_jacobian()
                        .unwrap()
                        .first()
                        .map_or(0.0, |entry| entry.value),
                    slope
                );
                model.observe(&mut device);
                assert_eq!(device.variable("y"), Some(value));
            }
        }
    }
    let model = DeviceFixture::compile(
        "module query(p); inout p; electrical p; analog I(p)<+$simparam(\"gmin\",1.0/V(p)); endmodule",
    );
    let mut device = model.device("LAZY", &[1]);
    device.update_voltages(&[0.0]);
    assert_eq!(device.try_evaluate().unwrap(), [1e-12]);
}

#[test]
fn simparam_environment_precedes_default_resolution_and_defaults_remain_fixed() {
    use rspice_veriloga::device::VerilogADevice;
    use rspice_veriloga_runtime::{GeneratedSimulationParameters, SimulationParameter};
    let model = DeviceFixture::compile(
        "module query(p); inout p; electrical p; parameter real x=$simparam(\"imax\"); analog I(p)<+x+$simparam(\"imax\"); endmodule",
    );
    assert!(model.try_device("MISSING", &[1]).is_err());
    let mut parameters = GeneratedSimulationParameters::default();
    parameters
        .try_set(SimulationParameter::Imax, Some(2.0))
        .unwrap();
    let mut device = VerilogADevice::try_new_with_simulation_parameters_and_control(
        "CONFIGURED",
        model.model.clone(),
        Some(&model.canonical_ir),
        &[1],
        &[],
        parameters,
        &rspice_veriloga::NoPipelineControl,
    )
    .unwrap();
    assert_eq!(device.try_evaluate().unwrap(), [4.0]);
    parameters
        .try_set(SimulationParameter::Imax, Some(5.0))
        .unwrap();
    device.set_simulation_parameters(parameters);
    assert_eq!(device.try_evaluate().unwrap(), [7.0]);
    device.try_resolve_parameter_defaults().unwrap();
    assert_eq!(device.try_evaluate().unwrap(), [10.0]);
}

#[test]
fn simparam_required_queries_cannot_hide_missing_values_inside_ddx() {
    for order in 0..=2 {
        let mut expression = "$simparam(\"imax\")".to_owned();
        for _ in 0..order {
            expression = format!("ddx({expression},V(p))");
        }
        let model = DeviceFixture::compile(&format!(
            "module query(p); inout p; electrical p; analog I(p)<+{expression}; endmodule"
        ));
        let mut device = model.device("MISSING", &[1]);
        assert!(
            device.try_evaluate().is_err(),
            "missing query hidden by derivative order {order}"
        );
    }
}

#[test]
fn simparam_guard_updates_switch_current_and_potential_contributions() {
    use rspice_veriloga_runtime::GeneratedSimulationParameters;
    for (contribution, expected) in [("I(p)<+2*V(p)", 4.0), ("V(p)<+2", 2.0)] {
        for assigned in [false, true] {
            let (declaration, assignment, guard) = if assigned {
                ("real enabled;", "enabled=$simparam(\"gmin\")>1;", "enabled")
            } else {
                ("", "", "$simparam(\"gmin\")>1")
            };
            let model = DeviceFixture::compile(&format!(
                "module query(p); inout p; electrical p; {declaration} analog begin {assignment} if ({guard}) {contribution}; end endmodule"
            ));
            let mut device = model.device("GUARD", &[1]);
            device.update_voltages(&[2.0]);
            for gmin in [0.0, 2.0, 0.0, 2.0] {
                let mut parameters = GeneratedSimulationParameters::default();
                parameters.set_gmin(gmin);
                device.set_simulation_parameters(parameters);
                assert_eq!(
                    device.try_evaluate().unwrap(),
                    vec![if gmin > 1.0 { expected } else { 0.0 }],
                    "{contribution}, assigned={assigned}, gmin={gmin}"
                );
            }
        }
    }
}

#[test]
fn simparam_signed_zero_updates_reach_named_observations() {
    use rspice_veriloga_runtime::{GeneratedSimulationParameters, SimulationParameter};
    let model = DeviceFixture::compile(
        "module query(p); inout p; electrical p; real y; analog begin y=$simparam(\"imax\"); I(p)<+y; end endmodule",
    );
    let mut device = model.device("ZERO", &[1]);
    let mut parameters = GeneratedSimulationParameters::default();
    for zero in [0.0_f64, -0.0, 0.0] {
        parameters
            .try_set(SimulationParameter::Imax, Some(zero))
            .unwrap();
        device.set_simulation_parameters(parameters);
        device.try_evaluate().unwrap();
        model.observe(&mut device);
        assert_eq!(device.variable("y").unwrap().to_bits(), zero.to_bits());
    }
}
