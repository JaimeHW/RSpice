use rspice_ui::product::{AnalysisInstanceId, SimulationPlanId};
use rspice_ui::simulation::netlist_gen::{
    DesignVariableNetlistContext, design_variable_parameter_lines,
};
use rspice_ui::state::{
    CellViewRef, DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity,
    DesignVariableRange, DesignVariableScope, DesignVariableSweepEligibility, ProjectWorkspace,
    SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
    SavedOutputPrecision, SavedOutputStreaming, SimulationPlanPayload, SimulationPlanPayloadRecord,
};

fn resistance_variable(name: &str, expression: &str, scope: DesignVariableScope) -> DesignVariable {
    DesignVariable::new(
        name,
        expression,
        DesignVariableQuantity::Resistance,
        scope,
        "integration contract",
        Some(DesignVariableRange {
            minimum: "1 kohm".to_owned(),
            maximum: "100 kohm".to_owned(),
        }),
        DesignVariableSweepEligibility::NestedSweepAndOptimization,
        DesignVariableOverridePolicy::ExplicitTestLocalOverride,
    )
    .unwrap()
}

fn raw_output(
    name: &str,
    expression: &str,
    compatible_analyses: SavedOutputCompatibility,
) -> SavedOutput {
    SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        name,
        expression,
        compatible_analyses,
        SavedOutputPolicy::EveryAcceptedPoint,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .unwrap()
}

#[test]
fn typed_values_emit_canonical_deterministic_parameter_lines() {
    let active_cell = CellViewRef::default_top();
    let selected_analysis = AnalysisInstanceId::new();
    let variables = vec![
        resistance_variable("ZLOAD", "20 kohm", DesignVariableScope::Project),
        resistance_variable("ALOAD", "10 kohm", DesignVariableScope::Testbench),
        resistance_variable(
            "OTHER_CELL",
            "30 kohm",
            DesignVariableScope::SelectedCell {
                cell: CellViewRef::new("user", "other", "schematic"),
            },
        ),
        resistance_variable(
            "ANALYSIS_ONLY",
            "40 kohm",
            DesignVariableScope::SelectedAnalysis {
                analysis_id: selected_analysis,
            },
        ),
    ];

    let lines = design_variable_parameter_lines(
        &variables,
        DesignVariableNetlistContext {
            active_cell: &active_cell,
            analysis_instances: &[selected_analysis],
        },
    )
    .unwrap();
    assert_eq!(
        lines,
        [
            ".param ALOAD=1.00000000000000000e4",
            ".param ANALYSIS_ONLY=4.00000000000000000e4",
            ".param ZLOAD=2.00000000000000000e4",
        ]
    );
}

#[test]
fn legacy_missing_row_identity_is_deterministic_but_null_is_rejected() {
    let mut value = serde_json::to_value(resistance_variable(
        "RLOAD",
        "10 kohm",
        DesignVariableScope::Project,
    ))
    .unwrap();
    value.as_object_mut().unwrap().remove("id");
    let first: DesignVariable = serde_json::from_value(value.clone()).unwrap();
    let replay: DesignVariable = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(first.id, replay.id);

    value
        .as_object_mut()
        .unwrap()
        .insert("id".to_owned(), serde_json::Value::Null);
    assert!(
        serde_json::from_value::<DesignVariable>(value)
            .unwrap_err()
            .to_string()
            .contains("must not be null")
    );
}

#[test]
fn plan_payload_round_trip_preserves_nested_ownership() {
    let plan_id = SimulationPlanId::new();
    let workspace = ProjectWorkspace {
        simulation_plan_payloads: vec![SimulationPlanPayloadRecord {
            plan_id,
            payload: SimulationPlanPayload {
                design_variables: vec![resistance_variable(
                    "RLOAD",
                    "10 kohm",
                    DesignVariableScope::Testbench,
                )],
                saved_outputs: vec![raw_output(
                    "VOUT",
                    "V(out)",
                    SavedOutputCompatibility::OpTranAc,
                )],
                ..SimulationPlanPayload::default()
            },
        }],
        ..ProjectWorkspace::default()
    };

    let json = serde_json::to_string(&workspace).unwrap();
    let restored: ProjectWorkspace = serde_json::from_str(&json).unwrap();
    restored.validate_simulation_configuration().unwrap();
    let payload = restored.active_plan_data(plan_id).unwrap();
    assert_eq!(payload.design_variables[0].name, "RLOAD");
    assert_eq!(payload.saved_outputs[0].name, "VOUT");
}

#[test]
fn cloning_refreshes_row_identity_and_remaps_analysis_ownership() {
    let source_plan = SimulationPlanId::new();
    let cloned_plan = SimulationPlanId::new();
    let source_analysis = AnalysisInstanceId::new();
    let cloned_analysis = AnalysisInstanceId::new();
    let variable = resistance_variable(
        "RLOAD",
        "10 kohm",
        DesignVariableScope::SelectedAnalysis {
            analysis_id: source_analysis,
        },
    );
    let output = raw_output(
        "VOUT",
        "V(out)",
        SavedOutputCompatibility::SelectedAnalysis {
            analysis_id: source_analysis,
        },
    );
    let variable_id = variable.id;
    let output_id = output.id;
    let mut workspace = ProjectWorkspace {
        simulation_plan_payloads: vec![SimulationPlanPayloadRecord {
            plan_id: source_plan,
            payload: SimulationPlanPayload {
                design_variables: vec![variable],
                saved_outputs: vec![output],
                ..SimulationPlanPayload::default()
            },
        }],
        ..ProjectWorkspace::default()
    };

    workspace
        .clone_plan_data(
            source_plan,
            cloned_plan,
            true,
            false,
            &[(source_analysis, cloned_analysis)],
        )
        .unwrap();
    let cloned = workspace.active_plan_data(cloned_plan).unwrap();
    assert_ne!(cloned.design_variables[0].id, variable_id);
    assert_ne!(cloned.saved_outputs[0].id, output_id);
    assert!(matches!(
        cloned.design_variables[0].scope,
        DesignVariableScope::SelectedAnalysis { analysis_id }
            if analysis_id == cloned_analysis
    ));
    assert!(matches!(
        cloned.saved_outputs[0].compatible_analyses,
        SavedOutputCompatibility::SelectedAnalysis { analysis_id }
            if analysis_id == cloned_analysis
    ));
}
