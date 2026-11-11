//! Staging a placed instance's Value for the parameter tuner.
//!
//! A literal value stages a typed candidate variable and leaves the
//! schematic and the plan untouched; a value that names a parameter
//! selects the variable it names; a value no typed quantity represents is
//! refused and stages nothing.

use crate::state::{ComponentType, Point};
use crate::workbench::RSpiceApp;
use crate::workbench::app::actions::tuning_staging::stage_component_tuning;

#[test]
fn literal_value_tuning_stages_a_typed_variable_without_mutating_authority() {
    let mut app = RSpiceApp::test_instance();
    let component_id = app
        .state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::origin());
    app.state.model.schematic.document_mut_for_test().components[0].name = "RLOAD".to_owned();
    app.state.model.schematic.document_mut_for_test().components[0].value = "10k".to_owned();
    let plan_id = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .unwrap()
        .id();
    let variables_before = app
        .state
        .model
        .workspace
        .content
        .plan_data(plan_id)
        .unwrap()
        .design_variables
        .len();
    let topology_before = app.state.model.schematic.topology_version();

    stage_component_tuning(
        &app.state.model,
        &mut app.state.simulate.verification,
        component_id,
    )
    .expect("resistor literal is tunable");

    assert_eq!(
        app.state.model.schematic.document().components[0].value,
        "10k"
    );
    assert_eq!(
        app.state.model.schematic.topology_version(),
        topology_before
    );
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .plan_data(plan_id)
            .unwrap()
            .design_variables
            .len(),
        variables_before
    );
    let binding = app
        .state
        .simulate
        .verification
        .tuning_instance_binding
        .as_ref()
        .expect("transient instance binding");
    assert!(binding.creates_variable);
    assert_eq!(binding.variable.name, "RLOAD_VALUE");
    assert_eq!(
        binding.variable.quantity,
        crate::state::DesignVariableQuantity::Resistance
    );
    assert!(binding.variable.allowed_range.is_none());
    assert_eq!(binding.binding_expression, "{RLOAD_VALUE}");
    assert!(
        app.state
            .simulate
            .verification
            .tuning_variables
            .iter()
            .any(|draft| draft.variable_id == binding.variable.id && draft.proposed)
    );
}

#[test]
fn parameter_bound_value_tuning_selects_the_existing_typed_variable() {
    let mut app = RSpiceApp::test_instance();
    let component_id = app
        .state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::origin());
    app.state.model.schematic.document_mut_for_test().components[0].value = "{RGAIN}".to_owned();
    let plan_id = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .unwrap()
        .id();
    let variable = crate::state::DesignVariable::new(
        "RGAIN",
        "499 ohm",
        crate::state::DesignVariableQuantity::Resistance,
        crate::state::DesignVariableScope::Project,
        "Gain resistor",
        None,
        crate::state::DesignVariableSweepEligibility::NestedSweepAndOptimization,
        crate::state::DesignVariableOverridePolicy::ExplicitTestLocalOverride,
    )
    .unwrap();
    let variable_id = variable.id;
    app.state
        .model
        .workspace
        .content
        .add_design_variable(plan_id, variable)
        .unwrap();

    stage_component_tuning(
        &app.state.model,
        &mut app.state.simulate.verification,
        component_id,
    )
    .expect("existing typed parameter reference is tunable");

    let session = &app.state.simulate.verification;
    let binding = session
        .tuning_instance_binding
        .as_ref()
        .expect("instance context is retained");
    assert!(!binding.creates_variable);
    assert_eq!(binding.variable.id, variable_id);
    assert_eq!(session.tuning_selected_variable, Some(variable_id));
    assert_eq!(session.tuning_focus_variable, Some(variable_id));
    assert!(session.tuning_variables.iter().all(|draft| !draft.proposed));
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .plan_data(plan_id)
            .unwrap()
            .design_variables
            .len(),
        1
    );
}

#[test]
fn value_tuning_fails_closed_when_no_truthful_quantity_exists() {
    let mut app = RSpiceApp::test_instance();
    let component_id = app
        .state
        .model
        .schematic
        .add_component(ComponentType::Inductor, Point::origin());
    app.state.model.schematic.document_mut_for_test().components[0].value = "10u".to_owned();

    let error = stage_component_tuning(
        &app.state.model,
        &mut app.state.simulate.verification,
        component_id,
    )
    .expect_err("inductance is not representable by the current typed variable schema");

    assert!(error.contains("truthful typed design-variable mapping"));
    assert!(
        app.state
            .simulate
            .verification
            .tuning_instance_binding
            .is_none()
    );
}
