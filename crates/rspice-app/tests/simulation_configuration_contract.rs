use rspice_app::{
    AnalysisInstanceId, DesignVariable, DesignVariableQuantity, DesignVariableRange,
    ProjectWorkspace, SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
    SavedOutputStreaming, SimulationPlanId, SimulationPlanPayload,
    SimulationPlanPayloadRecord,
};
use rspice_simulation::netlist_gen::design_variable_parameter_lines;

fn resistance_variable(name: &str, expression: &str) -> DesignVariable {
    DesignVariable::new(
        name,
        expression,
        DesignVariableQuantity::Resistance,
        "integration contract",
        Some(DesignVariableRange {
            minimum: "1 kohm".to_owned(),
            maximum: "100 kohm".to_owned(),
        }),
        true,
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
        SavedOutputStreaming::StoreOnly,
    )
    .unwrap()
}

#[test]
fn typed_values_emit_canonical_deterministic_parameter_lines() {
    let variables = vec![
        resistance_variable("ZLOAD", "20 kohm"),
        resistance_variable("ALOAD", "10 kohm"),
        resistance_variable("MLOAD", "40 kohm"),
    ];

    let lines = design_variable_parameter_lines(&variables).unwrap();
    assert_eq!(
        lines,
        [
            ".param ALOAD={10k}",
            ".param MLOAD={40k}",
            ".param ZLOAD={20k}",
        ]
    );
}

#[test]
fn plan_payload_round_trip_preserves_nested_ownership() {
    let plan_id = SimulationPlanId::new();
    // Assign rather than struct-update: `ProjectWorkspace` keeps its
    // digest-authenticated hardcopy source sets private, and functional update
    // syntax still requires every field to be visible from here.
    let mut workspace = ProjectWorkspace::default();
    workspace.content.simulation_plan_payloads = vec![SimulationPlanPayloadRecord {
        plan_id,
        payload: SimulationPlanPayload {
            design_variables: vec![resistance_variable("RLOAD", "10 kohm")],
            saved_outputs: vec![raw_output(
                "VOUT",
                "V(out)",
                SavedOutputCompatibility::OpTranAc,
            )],
            ..SimulationPlanPayload::default()
        },
    }];

    let json = serde_json::to_string(&workspace).unwrap();
    let restored: ProjectWorkspace = serde_json::from_str(&json).unwrap();
    restored
        .content
        .validate_simulation_configuration()
        .unwrap();
    let payload = restored.content.plan_data(plan_id).unwrap();
    assert_eq!(payload.design_variables[0].name, "RLOAD");
    assert_eq!(payload.saved_outputs[0].name, "VOUT");
}

#[test]
fn cloning_refreshes_row_identity_and_remaps_analysis_ownership() {
    let source_plan = SimulationPlanId::new();
    let cloned_plan = SimulationPlanId::new();
    let source_analysis = AnalysisInstanceId::new();
    let cloned_analysis = AnalysisInstanceId::new();
    let variable = resistance_variable("RLOAD", "10 kohm");
    let output = raw_output(
        "VOUT",
        "V(out)",
        SavedOutputCompatibility::SelectedAnalysis {
            analysis_id: source_analysis,
        },
    );
    let variable_id = variable.id;
    let output_id = output.id;
    let mut workspace = ProjectWorkspace::default();
    workspace.content.simulation_plan_payloads = vec![SimulationPlanPayloadRecord {
        plan_id: source_plan,
        payload: SimulationPlanPayload {
            design_variables: vec![variable],
            saved_outputs: vec![output],
            ..SimulationPlanPayload::default()
        },
    }];

    workspace
        .content
        .clone_plan_data(
            source_plan,
            cloned_plan,
            true,
            false,
            &[(source_analysis, cloned_analysis)],
        )
        .unwrap();
    let cloned = workspace.content.plan_data(cloned_plan).unwrap();
    assert_ne!(cloned.design_variables[0].id, variable_id);
    assert_ne!(cloned.saved_outputs[0].id, output_id);
    assert!(matches!(
        cloned.saved_outputs[0].compatible_analyses,
        SavedOutputCompatibility::SelectedAnalysis { analysis_id }
            if analysis_id == cloned_analysis
    ));
}
