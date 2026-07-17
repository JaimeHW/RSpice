use std::sync::Arc;

use rspice_core::{Engine, Netlist, SimulationConfig, SimulationConfigError, SimulationError};

#[test]
fn default_configuration_is_valid() {
    let config = SimulationConfig::default();

    config.validate().expect("default configuration validates");
    Engine::try_new(config).expect("default engine construction succeeds");
}

#[test]
fn invalid_scalar_is_reported_with_stable_field_context() {
    let config = SimulationConfig {
        tolerance: f64::NAN,
        ..SimulationConfig::default()
    };

    assert!(matches!(
        Engine::try_new(config),
        Err(SimulationConfigError::InvalidValue {
            field: "tolerance",
            value,
            requirement: "a positive finite number",
        }) if value.is_nan()
    ));
}

#[test]
fn timestep_bounds_and_initial_step_are_validated() {
    let reversed = SimulationConfig {
        min_timestep: 2.0e-9,
        max_timestep: 1.0e-9,
        ..SimulationConfig::default()
    };
    assert_eq!(
        reversed.validate(),
        Err(SimulationConfigError::InvalidTimestepRange {
            min_timestep: 2.0e-9,
            max_timestep: 1.0e-9,
        })
    );

    let oversized_initial = SimulationConfig {
        max_timestep: 1.0e-9,
        transient_initial_timestep: Some(2.0e-9),
        ..SimulationConfig::default()
    };
    assert_eq!(
        oversized_initial.validate(),
        Err(SimulationConfigError::InitialTimestepExceedsMaximum {
            initial_timestep: 2.0e-9,
            max_timestep: 1.0e-9,
        })
    );
}

#[test]
fn nested_convergence_and_bypass_settings_are_validated() {
    let mut convergence = SimulationConfig::default();
    convergence.convergence_config.residual_reltol = f64::INFINITY;
    assert!(matches!(
        convergence.validate(),
        Err(SimulationConfigError::InvalidValue {
            field: "convergence_config.residual_reltol",
            ..
        })
    ));

    let mut bypass = SimulationConfig::default();
    bypass.bypass_config.abstol = -1.0;
    assert!(matches!(
        bypass.validate(),
        Err(SimulationConfigError::InvalidValue {
            field: "bypass_config.abstol",
            ..
        })
    ));

    let mut reversed_gmin = SimulationConfig::default();
    reversed_gmin.convergence_config.gmin_initial = 1.0e-15;
    reversed_gmin.convergence_config.gmin_target = 1.0e-12;
    assert_eq!(
        reversed_gmin.validate(),
        Err(SimulationConfigError::InvalidGminRange {
            gmin_initial: 1.0e-15,
            gmin_target: 1.0e-12,
        })
    );

    let mut disabled_junction_floor = SimulationConfig::default();
    disabled_junction_floor
        .convergence_config
        .junction_gmin_target = 0.0;
    disabled_junction_floor
        .validate()
        .expect("zero is the explicit device-junction GMIN disable value");

    disabled_junction_floor
        .convergence_config
        .junction_gmin_target = -1.0;
    assert!(matches!(
        disabled_junction_floor.validate(),
        Err(SimulationConfigError::InvalidValue {
            field: "convergence_config.junction_gmin_target",
            requirement: "a non-negative finite number",
            ..
        })
    ));
}

#[test]
fn locked_time_grid_rejects_invalid_and_non_increasing_points() {
    let invalid = SimulationConfig {
        locked_time_grid: Some(Arc::new(vec![0.0, f64::INFINITY])),
        ..SimulationConfig::default()
    };
    assert!(matches!(
        invalid.validate(),
        Err(SimulationConfigError::InvalidLockedTimeGridPoint {
            index: 1,
            value,
        }) if value.is_infinite()
    ));

    let unsorted = SimulationConfig {
        locked_time_grid: Some(Arc::new(vec![0.0, 2.0e-9, 1.0e-9])),
        ..SimulationConfig::default()
    };
    assert_eq!(
        unsorted.validate(),
        Err(SimulationConfigError::NonIncreasingLockedTimeGrid {
            index: 2,
            value: 1.0e-9,
            previous_index: 1,
            previous: 2.0e-9,
        })
    );
}

#[test]
fn digital_delay_selector_is_closed_over_documented_values() {
    for valid in 0..=3 {
        let config = SimulationConfig {
            digital_delay_type: Some(valid),
            ..SimulationConfig::default()
        };
        config.validate().expect("documented selector validates");
    }

    let invalid = SimulationConfig {
        digital_delay_type: Some(4),
        ..SimulationConfig::default()
    };
    assert_eq!(
        invalid.validate(),
        Err(SimulationConfigError::InvalidDigitalDelayType(4))
    );
}

#[test]
fn compatibility_constructor_cannot_execute_invalid_configuration() {
    let engine = Engine::new(SimulationConfig {
        max_iterations: 0,
        ..SimulationConfig::default()
    });
    assert!(matches!(
        engine.configuration_error(),
        Some(SimulationConfigError::InvalidCount {
            field: "max_iterations",
            value: 0,
        })
    ));

    let netlist = Netlist::parse("invalid config guard\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n")
        .expect("fixture parses");
    assert!(matches!(
        engine.run_dc_op(&netlist),
        Err(SimulationError::Configuration(
            SimulationConfigError::InvalidCount {
                field: "max_iterations",
                value: 0,
            }
        ))
    ));
}
