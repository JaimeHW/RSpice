//! Tests for source editing, validation identity, and dirty tracking.
//!
//! An edit must preserve exact UTF-8 and invalidate the validation identity it
//! invalidates; replacement is monotonic and atomic; and validation rejects a
//! mismatched slot rather than accepting stale evidence for it.

use super::*;
use crate::state::{Point, ProjectSourceRole};

fn reference(cell: &str) -> CellViewRef {
    CellViewRef::new("work", cell, "schematic")
}

fn symbol_reference(cell: &str) -> CellViewRef {
    CellViewRef::new("work", cell, "symbol")
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn model_bound_source_validation_resolves_the_selected_lib_section() {
    let path = std::env::temp_dir().join(format!(
        "rspice-model-bound-section-{}.lib",
        uuid::Uuid::new_v4()
    ));
    std::fs::write(
            &path,
            ".lib TT\n.model nmos_18 nmos level=1\n.endl TT\n.lib FF\n.model nmos_18_fast nmos level=1\n.endl FF\n",
        )
        .expect("write sectioned model fixture");
    let mut binding = LibraryCellInstance::new("models", "nmos_18", "spice");
    binding.module_name = Some("nmos_18".to_owned());
    binding.netlist_template = Some("M{name} {nodes} {model} {params}".to_owned());
    binding.model_section = Some("TT".to_owned());

    validate_source_file(&path, ViewType::Spice, &binding)
        .expect("selected section declares the executable model");
    binding.model_section = Some("FF".to_owned());
    assert!(validate_source_file(&path, ViewType::Spice, &binding).is_err());

    std::fs::remove_file(path).expect("remove sectioned model fixture");
}

#[test]
fn configuration_override_patterns_use_most_specific_segment_match() {
    let overrides = vec![
        crate::state::ConfigurationSetOverride {
            instance_path: "/top/*".to_owned(),
            executable_views: vec!["spice".to_owned()],
            stop_view: Some("spice".to_owned()),
            model_section: None,
            eligible_platforms: crate::state::ConfigurationPlatform::ALL.to_vec(),
        },
        crate::state::ConfigurationSetOverride {
            instance_path: "/top/Xcritical".to_owned(),
            executable_views: vec!["schematic".to_owned()],
            stop_view: None,
            model_section: None,
            eligible_platforms: crate::state::ConfigurationPlatform::ALL.to_vec(),
        },
    ];
    let selected = selected_configuration_override(&overrides, "/top/xCRITICAL")
        .expect("specific override matches");
    assert_eq!(selected.instance_path, "/top/Xcritical");
    let wildcard = selected_configuration_override(&overrides, "/top/Xother")
        .expect("wildcard override matches");
    assert_eq!(wildcard.instance_path, "/top/*");
}

fn resistance_variable(name: &str, expression: &str, scope: DesignVariableScope) -> DesignVariable {
    DesignVariable::new(
        name,
        expression,
        DesignVariableQuantity::Resistance,
        scope,
        "fixture",
        Some(DesignVariableRange {
            minimum: "1 kohm".to_owned(),
            maximum: "1 Mohm".to_owned(),
        }),
        DesignVariableSweepEligibility::NestedSweepAndOptimization,
        DesignVariableOverridePolicy::ExplicitTestLocalOverride,
    )
    .expect("fixture variable is valid")
}

fn raw_output(
    name: &str,
    expression: &str,
    compatibility: SavedOutputCompatibility,
) -> SavedOutput {
    SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        name,
        expression,
        compatibility,
        SavedOutputPolicy::EveryAcceptedPoint,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation,
    )
    .expect("fixture output is valid")
}

fn scalar_spec(measurement: &str, minimum: f64) -> SpecEntry {
    SpecEntry {
        measurement: measurement.to_owned(),
        expression: format!(".meas tran {measurement} max V(out)"),
        min: Some(minimum),
        max: None,
        unit: "V".to_owned(),
        scope: SpecPointScope::AllPoints,
    }
}

#[test]
fn legacy_specifications_migrate_to_stable_governed_definitions() {
    let plan_id = SimulationPlanId::new();
    let mut workspace = ProjectWorkspace::default();
    workspace
        .simulation_plan_payloads
        .push(SimulationPlanPayloadRecord {
            plan_id,
            payload: SimulationPlanPayload {
                specs: vec![scalar_spec("peak", 1.0)],
                ..SimulationPlanPayload::default()
            },
        });

    workspace.migrate_active_plan_data(plan_id);
    let first = workspace.plan_data(plan_id).unwrap().clone();
    workspace.migrate_active_plan_data(plan_id);
    let replay = workspace.plan_data(plan_id).unwrap();

    assert_eq!(first.specification_definitions.len(), 1);
    assert_eq!(
        first.specification_definitions, replay.specification_definitions,
        "legacy migration must be deterministic across repeated access",
    );
    assert_eq!(
        replay.specification_definitions[0].projected_entry(),
        replay.specs[0],
    );
    workspace.validate_simulation_configuration().unwrap();
}

#[test]
fn scalar_spec_edits_preserve_governance_and_clone_remaps_analysis_binding() {
    let source_plan_id = SimulationPlanId::new();
    let cloned_plan_id = SimulationPlanId::new();
    let source_analysis = AnalysisInstanceId::new();
    let cloned_analysis = AnalysisInstanceId::new();
    let mut workspace = ProjectWorkspace::default();
    workspace.replace_active_specs(source_plan_id, vec![scalar_spec("peak", 1.0)]);

    let source = workspace.plan_data_mut(source_plan_id).unwrap();
    let definition = &mut source.specification_definitions[0];
    let original_id = definition.id;
    definition.requirement_key = "REQ-PEAK".to_owned();
    definition.requirement_name = "Minimum output peak".to_owned();
    definition.role = SpecificationRole::Review;
    definition.guard_band = Some(0.05);
    definition.producing_analysis = Some(source_analysis);

    workspace.replace_active_specs(source_plan_id, vec![scalar_spec("peak", 1.2)]);
    let edited = &workspace
        .plan_data(source_plan_id)
        .unwrap()
        .specification_definitions[0];
    assert_eq!(edited.id, original_id);
    assert_eq!(edited.requirement_key, "REQ-PEAK");
    assert_eq!(edited.role, SpecificationRole::Review);
    assert_eq!(edited.guard_band, Some(0.05));
    assert_eq!(edited.projected_entry().min, Some(1.2));

    workspace
        .clone_plan_data(
            source_plan_id,
            cloned_plan_id,
            true,
            false,
            &[(source_analysis, cloned_analysis)],
        )
        .unwrap();
    let cloned = &workspace
        .plan_data(cloned_plan_id)
        .unwrap()
        .specification_definitions[0];
    assert_ne!(cloned.id, original_id);
    assert_eq!(cloned.requirement_key, "REQ-PEAK");
    assert_eq!(cloned.producing_analysis, Some(cloned_analysis));
    assert_eq!(cloned.projected_entry().min, Some(1.2));
    workspace.validate_simulation_configuration().unwrap();
}

#[test]
fn legacy_projection_edit_does_not_flatten_unchanged_equality_semantics() {
    let plan_id = SimulationPlanId::new();
    let mut workspace = ProjectWorkspace::default();
    workspace.replace_active_specs(plan_id, vec![scalar_spec("offset", -0.1)]);
    let definition = &mut workspace
        .plan_data_mut(plan_id)
        .unwrap()
        .specification_definitions[0];
    definition.comparison = SpecificationComparison::EqualWithin {
        target: 0.0,
        tolerance: 0.1,
    };
    let mut projected = definition.projected_entry();
    projected.scope = SpecPointScope::Nominal;

    workspace.replace_active_specs(plan_id, vec![projected]);

    let retained = &workspace
        .plan_data(plan_id)
        .unwrap()
        .specification_definitions[0];
    assert_eq!(
        retained.comparison,
        SpecificationComparison::EqualWithin {
            target: 0.0,
            tolerance: 0.1,
        }
    );
    assert_eq!(retained.scope, SpecPointScope::Nominal);
}

#[test]
fn typed_design_variable_enforces_units_range_and_canonical_netlist_value() {
    let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
    assert_eq!(variable.resolved_value_si().unwrap(), 10_000.0);
    assert_eq!(
        variable.netlist_statement(),
        ".param RLOAD=1.00000000000000000e4"
    );

    let mut wrong_unit = variable.clone();
    wrong_unit.expression = "10 V".to_owned();
    assert!(wrong_unit.validate().unwrap_err().contains("resistance"));

    let mut outside = variable;
    outside.expression = "2 Mohm".to_owned();
    assert!(outside.validate().unwrap_err().contains("outside"));
}

#[test]
fn design_variable_expression_update_preserves_identity_and_metadata() {
    let plan_id = SimulationPlanId::new();
    let mut workspace = ProjectWorkspace::default();
    let original = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
    let variable_id = original.id;
    workspace
        .add_design_variable(plan_id, original.clone())
        .expect("fixture variable is accepted");

    workspace
        .update_design_variable_expression(plan_id, variable_id, "22 kohm")
        .expect("valid expression update commits");

    let updated = &workspace
        .plan_data(plan_id)
        .expect("plan payload remains present")
        .design_variables[0];
    assert_eq!(updated.id, original.id);
    assert_eq!(updated.name, original.name);
    assert_eq!(updated.expression, "22 kohm");
    assert_eq!(updated.quantity, original.quantity);
    assert_eq!(updated.scope, original.scope);
    assert_eq!(updated.description, original.description);
    assert_eq!(updated.allowed_range, original.allowed_range);
    assert_eq!(updated.sweep_eligibility, original.sweep_eligibility);
    assert_eq!(updated.override_policy, original.override_policy);
}

#[test]
fn out_of_range_design_variable_update_is_rejected_atomically() {
    let plan_id = SimulationPlanId::new();
    let mut workspace = ProjectWorkspace::default();
    let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
    let variable_id = variable.id;
    workspace
        .add_design_variable(plan_id, variable)
        .expect("fixture variable is accepted");
    let before = serde_json::to_value(&workspace).expect("workspace serializes");

    let error = workspace
        .update_design_variable_expression(plan_id, variable_id, "2 Mohm")
        .expect_err("out-of-range expression must be rejected");

    assert!(matches!(
        error,
        SimulationConfigurationError::InvalidDesignVariable { message, .. }
            if message.contains("outside the inclusive allowed range")
    ));
    assert_eq!(
        serde_json::to_value(&workspace).expect("workspace still serializes"),
        before
    );
}

#[test]
fn design_variable_update_rejects_a_missing_stable_identity() {
    let plan_id = SimulationPlanId::new();
    let mut workspace = ProjectWorkspace::default();
    let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
    workspace
        .add_design_variable(plan_id, variable)
        .expect("fixture variable is accepted");
    let missing_id = DesignVariableId::new();
    let before = serde_json::to_value(&workspace).expect("workspace serializes");

    assert_eq!(
        workspace.update_design_variable_expression(plan_id, missing_id, "22 kohm"),
        Err(SimulationConfigurationError::DesignVariableNotFound {
            plan_id,
            variable_id: missing_id,
        })
    );
    assert_eq!(
        serde_json::to_value(&workspace).expect("workspace still serializes"),
        before
    );
}

#[test]
fn committed_design_variable_update_advances_revision_once() {
    let plan_id = SimulationPlanId::new();
    let mut workspace = ProjectWorkspace::default();
    let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
    let variable_id = variable.id;
    let initial_revision = variable.revision;
    workspace
        .add_design_variable(plan_id, variable)
        .expect("fixture variable is accepted");

    let committed_revision = workspace
        .update_design_variable_expression(plan_id, variable_id, "22 kohm")
        .expect("valid expression update commits");

    assert_eq!(committed_revision.get(), initial_revision.get() + 1);
    assert_eq!(
        workspace
            .plan_data(plan_id)
            .expect("plan payload remains present")
            .design_variables[0]
            .revision,
        committed_revision
    );
}

#[test]
fn bulk_design_variable_update_is_all_or_nothing() {
    let plan_id = SimulationPlanId::new();
    let mut workspace = ProjectWorkspace::default();
    let first = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
    let second = resistance_variable("RBIAS", "15 kohm", DesignVariableScope::Project);
    let updates = vec![
        (first.id, "22 kohm".to_owned()),
        (second.id, "2 Mohm".to_owned()),
    ];
    workspace
        .add_design_variable(plan_id, first)
        .expect("first fixture variable is accepted");
    workspace
        .add_design_variable(plan_id, second)
        .expect("second fixture variable is accepted");
    let before = serde_json::to_value(&workspace).expect("workspace serializes");

    assert!(matches!(
        workspace.update_design_variable_expressions(plan_id, &updates),
        Err(SimulationConfigurationError::InvalidDesignVariable { index: 1, .. })
    ));
    assert_eq!(
        serde_json::to_value(&workspace).expect("workspace still serializes"),
        before
    );
}

#[test]
fn bulk_design_variable_update_rejects_duplicate_identities_atomically() {
    let plan_id = SimulationPlanId::new();
    let mut workspace = ProjectWorkspace::default();
    let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
    let variable_id = variable.id;
    workspace
        .add_design_variable(plan_id, variable)
        .expect("fixture variable is accepted");
    let before = serde_json::to_value(&workspace).expect("workspace serializes");
    let updates = vec![
        (variable_id, "22 kohm".to_owned()),
        (variable_id, "47 kohm".to_owned()),
    ];

    assert_eq!(
        workspace.update_design_variable_expressions(plan_id, &updates),
        Err(
            SimulationConfigurationError::DuplicateDesignVariableUpdate {
                plan_id,
                variable_id,
            }
        )
    );
    assert_eq!(
        serde_json::to_value(&workspace).expect("workspace still serializes"),
        before
    );
}

#[test]
fn bulk_design_variable_add_commits_the_whole_batch() {
    let plan_id = SimulationPlanId::new();
    let mut workspace = ProjectWorkspace::default();
    let batch = vec![
        resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project),
        resistance_variable("RBIAS", "15 kohm", DesignVariableScope::Project),
    ];

    workspace
        .add_design_variables(plan_id, batch)
        .expect("a valid batch commits");

    let names = workspace
        .plan_data(plan_id)
        .expect("plan payload exists")
        .design_variables
        .iter()
        .map(|variable| variable.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(names, ["RLOAD", "RBIAS"]);
}

#[test]
fn bulk_design_variable_add_rejects_an_invalid_member_atomically() {
    let plan_id = SimulationPlanId::new();
    let mut workspace = ProjectWorkspace::default();
    let mut outside = resistance_variable("RBIAS", "15 kohm", DesignVariableScope::Project);
    outside.expression = "2 Mohm".to_owned();
    let batch = vec![
        resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project),
        outside,
    ];
    let before = serde_json::to_value(&workspace).expect("workspace serializes");

    assert!(matches!(
        workspace.add_design_variables(plan_id, batch),
        Err(SimulationConfigurationError::InvalidDesignVariable { message, .. })
            if message.contains("outside the inclusive allowed range")
    ));
    assert_eq!(
        serde_json::to_value(&workspace).expect("workspace still serializes"),
        before
    );
}

#[test]
fn bulk_design_variable_add_rejects_a_name_repeated_within_the_batch() {
    let plan_id = SimulationPlanId::new();
    let mut workspace = ProjectWorkspace::default();
    let batch = vec![
        resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project),
        resistance_variable("rload", "22 kohm", DesignVariableScope::Project),
    ];
    let before = serde_json::to_value(&workspace).expect("workspace serializes");

    assert_eq!(
        workspace.add_design_variables(plan_id, batch),
        Err(SimulationConfigurationError::DesignVariableNameConflict {
            plan_id,
            name: "rload".to_owned(),
        })
    );
    assert_eq!(
        serde_json::to_value(&workspace).expect("workspace still serializes"),
        before
    );
}

#[test]
fn bulk_design_variable_add_rejects_a_name_the_plan_already_owns() {
    let plan_id = SimulationPlanId::new();
    let mut workspace = ProjectWorkspace::default();
    workspace
        .add_design_variable(
            plan_id,
            resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project),
        )
        .expect("fixture variable is accepted");
    let before = serde_json::to_value(&workspace).expect("workspace serializes");
    let batch = vec![
        resistance_variable("RTERM", "50 kohm", DesignVariableScope::Project),
        resistance_variable("RLOAD", "22 kohm", DesignVariableScope::Project),
    ];

    assert_eq!(
        workspace.add_design_variables(plan_id, batch),
        Err(SimulationConfigurationError::DesignVariableNameConflict {
            plan_id,
            name: "RLOAD".to_owned(),
        })
    );
    assert_eq!(
        serde_json::to_value(&workspace).expect("workspace still serializes"),
        before
    );
}

#[test]
fn saved_output_validation_is_kind_specific() {
    assert!(
        raw_output(
            "VOUT",
            "V(out)",
            SavedOutputCompatibility::AllCompatibleAnalyses
        )
        .validate()
        .is_ok()
    );
    let invalid = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "gain",
        "V(out) / V(in)",
        SavedOutputCompatibility::AllCompatibleAnalyses,
        SavedOutputPolicy::EveryAcceptedPoint,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    );
    assert!(invalid.unwrap_err().contains("raw output"));
    let derived = SavedOutput::new(
        SavedOutputKind::DerivedExpression,
        "gain",
        "V(out) / V(in)",
        SavedOutputCompatibility::AllCompatibleAnalyses,
        SavedOutputPolicy::OnDemandFromRetainedState,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .expect("calculator expression is valid");
    assert_eq!(derived.inferred_unit(), "resolved from expression");
}

#[test]
fn missing_row_identity_migrates_deterministically_and_null_is_rejected() {
    let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
    let mut value = serde_json::to_value(variable).unwrap();
    value.as_object_mut().unwrap().remove("id");
    let first: DesignVariable = serde_json::from_value(value.clone()).unwrap();
    let second: DesignVariable = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(first.id, second.id);

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
fn plan_payload_clone_refreshes_row_ids_and_analysis_references() {
    let source_plan_id = SimulationPlanId::new();
    let cloned_plan_id = SimulationPlanId::new();
    let source_analysis = AnalysisInstanceId::new();
    let cloned_analysis = AnalysisInstanceId::new();
    let mut workspace = ProjectWorkspace::default();
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
    let regression_rule = RegressionToleranceRule {
        target: RegressionTargetSelector {
            source_domain: AnalysisResultSourceDomain::SimulationPlan,
            source_instance_id: source_analysis,
            kind: RegressionTargetKind::Waveform,
            name: "v(out)".to_owned(),
            occurrence: 0,
        },
        method: RegressionComparisonMethod::AbsoluteRelativeEnvelope,
        absolute_tolerance: 0.01,
        relative_tolerance: 0.005,
        time_skew_allowance: 20e-6,
        comparison_window: Some(RegressionComparisonWindow {
            start: 0.0,
            end: 20e-3,
        }),
    };
    workspace
        .simulation_plan_payloads
        .push(SimulationPlanPayloadRecord {
            plan_id: source_plan_id,
            payload: SimulationPlanPayload {
                design_variables: vec![variable],
                saved_outputs: vec![output],
                regression_baseline_run: Some(RunId::new()),
                regression_tolerances: vec![regression_rule],
                ..SimulationPlanPayload::default()
            },
        });

    workspace
        .clone_plan_data(
            source_plan_id,
            cloned_plan_id,
            true,
            true,
            &[(source_analysis, cloned_analysis)],
        )
        .unwrap();
    let cloned = workspace.plan_data(cloned_plan_id).unwrap();
    assert_ne!(cloned.design_variables[0].id, variable_id);
    assert_ne!(cloned.saved_outputs[0].id, output_id);
    assert!(matches!(
        cloned.design_variables[0].scope,
        DesignVariableScope::SelectedAnalysis { analysis_id }
            if analysis_id == cloned_analysis
    ));
    assert_eq!(cloned.regression_tolerances.len(), 1);
    assert_eq!(
        cloned.regression_tolerances[0].target.source_instance_id,
        cloned_analysis
    );
    assert_eq!(
        cloned.regression_tolerances[0].comparison_window,
        Some(RegressionComparisonWindow {
            start: 0.0,
            end: 20e-3,
        })
    );
    assert!(matches!(
        cloned.saved_outputs[0].compatible_analyses,
        SavedOutputCompatibility::SelectedAnalysis { analysis_id }
            if analysis_id == cloned_analysis
    ));

    workspace
        .plan_data_mut(cloned_plan_id)
        .unwrap()
        .design_variables[0]
        .expression = "20 kohm".to_owned();
    assert_eq!(
        workspace
            .plan_data(source_plan_id)
            .unwrap()
            .design_variables[0]
            .expression,
        "10 kohm"
    );
    workspace.validate_simulation_configuration().unwrap();
}

#[test]
fn regression_tolerance_contract_round_trips_and_rejects_invalid_windows() {
    let plan_id = SimulationPlanId::new();
    let mut workspace = ProjectWorkspace::default();
    let rule = RegressionToleranceRule {
        target: RegressionTargetSelector {
            source_domain: AnalysisResultSourceDomain::ManualDeck,
            source_instance_id: AnalysisInstanceId::new(),
            kind: RegressionTargetKind::Waveform,
            name: "v(out)".to_owned(),
            occurrence: 0,
        },
        method: RegressionComparisonMethod::PointwiseRelative,
        absolute_tolerance: 1e-3,
        relative_tolerance: 0.02,
        time_skew_allowance: 1e-6,
        comparison_window: Some(RegressionComparisonWindow {
            start: 0.0,
            end: 1e-3,
        }),
    };
    workspace
        .ensure_active_plan_data(plan_id)
        .regression_tolerances = vec![rule.clone()];
    workspace.validate_simulation_configuration().unwrap();

    let json = serde_json::to_string(&workspace).unwrap();
    let restored: ProjectWorkspace = serde_json::from_str(&json).unwrap();
    assert_eq!(
        restored
            .plan_data(plan_id)
            .unwrap()
            .regression_tolerances,
        vec![rule]
    );

    let mut invalid = restored;
    invalid
        .plan_data_mut(plan_id)
        .unwrap()
        .regression_tolerances[0]
        .comparison_window = Some(RegressionComparisonWindow {
        start: 2.0,
        end: 1.0,
    });
    assert!(matches!(
        invalid.validate_simulation_configuration(),
        Err(SimulationConfigurationError::InvalidRegressionTolerance { .. })
    ));

    let mut invalid_name = workspace;
    invalid_name
        .plan_data_mut(plan_id)
        .unwrap()
        .regression_tolerances[0]
        .target
        .name = "v(out)\u{1}".to_owned();
    assert!(matches!(
        invalid_name.validate_simulation_configuration(),
        Err(SimulationConfigurationError::InvalidRegressionTolerance { .. })
    ));
}

fn add_schematic_master(
    libraries: &mut LibraryManager,
    workspace: &mut ProjectWorkspace,
    library_name: &str,
    cell_name: &str,
    schematic: SchematicState,
) {
    if libraries.get_library(library_name).is_none() {
        libraries.add_library(Library::new(library_name));
    }
    let library = libraries
        .get_library_mut(library_name)
        .expect("library exists");
    let cell = library.get_or_create_cell(cell_name);
    if cell.get_view("schematic").is_none() {
        cell.add_view(View::new("schematic", ViewType::Schematic));
    }
    workspace.schematic_buffers.insert(
        CellViewRef::new(library_name, cell_name, "schematic").key(),
        schematic,
    );
}

fn instance(library: &str, cell: &str) -> LibraryCellInstance {
    LibraryCellInstance::new(library, cell, "schematic")
}

#[test]
fn hierarchy_resolution_follows_instances_not_open_tabs() {
    let mut workspace = ProjectWorkspace::default();
    workspace.open_views.push(OpenCellView::new(
        CellViewRef::new("unrelated", "open_tab", "schematic"),
        ViewType::Schematic,
    ));
    let mut libraries = LibraryManager::default();
    workspace.ensure_library_model(&mut libraries);

    let resolution = workspace.resolve_hierarchy(&libraries);

    assert_eq!(resolution.total_instances, 1);
    assert_eq!(resolution.resolved_instances, 1);
    assert_eq!(resolution.bindings.len(), 1);
    assert_eq!(resolution.bindings[0].purpose, "testbench root");
    assert_eq!(resolution.bindings[0].reference.cell, "top");
}

#[test]
fn hierarchy_resolution_counts_transitive_repeated_instances() {
    let mut workspace = ProjectWorkspace::default();
    let mut libraries = LibraryManager::default();
    workspace.ensure_library_model(&mut libraries);

    let top = workspace
        .schematic_buffers
        .get_mut(&CellViewRef::default_top().key())
        .expect("top buffer");
    top.add_library_cell_component(Point::new(20, 20), instance("work", "amp"));
    top.add_library_cell_component(Point::new(80, 20), instance("work", "amp"));

    let mut amp = SchematicState::default();
    amp.add_library_cell_component(Point::new(40, 40), instance("work", "bias"));
    add_schematic_master(&mut libraries, &mut workspace, "work", "amp", amp);
    add_schematic_master(
        &mut libraries,
        &mut workspace,
        "work",
        "bias",
        SchematicState::default(),
    );

    let resolution = workspace.resolve_hierarchy(&libraries);

    assert!(resolution.is_valid());
    assert_eq!(resolution.total_instances, 5);
    assert_eq!(resolution.resolved_instances, 5);
    assert_eq!(resolution.bindings.len(), 3);
    let amp = resolution
        .bindings
        .iter()
        .find(|row| row.reference.cell == "amp")
        .expect("amp row");
    assert_eq!(amp.instance_count, 2);
    assert_eq!(amp.purpose, "design under test");
    assert_eq!(
        amp.view_search_order
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["schematic", "extracted", "spice"]
    );
    assert_eq!(amp.stop_view.as_deref(), Some("spice"));
    let bias = resolution
        .bindings
        .iter()
        .find(|row| row.reference.cell == "bias")
        .expect("bias row");
    assert_eq!(bias.instance_count, 2);
    assert_eq!(bias.purpose, "hierarchical cell");
}

#[test]
fn active_configuration_drives_exact_path_resolution_and_receipt_identity() {
    let mut workspace = ProjectWorkspace::default();
    let mut libraries = LibraryManager::default();
    workspace.ensure_library_model(&mut libraries);
    let top = workspace
        .schematic_buffers
        .get_mut(&CellViewRef::default_top().key())
        .expect("top buffer");
    top.add_library_cell_component(Point::new(20, 20), instance("work", "amp"));
    top.add_library_cell_component(Point::new(80, 20), instance("work", "amp"));
    add_schematic_master(
        &mut libraries,
        &mut workspace,
        "work",
        "amp",
        SchematicState::default(),
    );

    let id = workspace
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: "Lab characterization".to_owned(),
            root: CellViewRef::default_top(),
            dut_path: "/top/X1".to_owned(),
            executable_view_policy: vec!["schematic".to_owned(), "spice".to_owned()],
            stop_views: vec!["spice".to_owned()],
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: vec![crate::state::ConfigurationSetOverride {
                instance_path: "/top/X2".to_owned(),
                executable_views: vec!["spice".to_owned()],
                stop_view: Some("spice".to_owned()),
                model_section: Some("tt".to_owned()),
                eligible_platforms: crate::state::ConfigurationPlatform::ALL.to_vec(),
            }],
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Analog design".to_owned(),
        })
        .expect("create configuration");

    let resolution = workspace.resolve_hierarchy(&libraries);

    assert_eq!(resolution.configuration_id, Some(id));
    assert_eq!(resolution.configuration_revision, Some(1));
    assert_eq!(
        resolution.configuration_digest,
        workspace
            .configuration_sets
            .find(id)
            .map(|configuration| configuration.semantic_digest())
    );
    assert_eq!(resolution.total_instances, 3);
    assert_eq!(resolution.resolved_instances, 2);
    assert_eq!(resolution.unresolved_instances(), 1);
    let configured = resolution
        .bindings
        .iter()
        .find(|binding| binding.instance_paths == ["/top/X2"])
        .expect("exact overridden instance row");
    assert_eq!(configured.view_search_order, ["spice"]);
    assert_eq!(configured.model_section, "tt");
    assert_eq!(configured.status, HierarchyBindingStatus::Unresolved);
    assert!(resolution.bindings.iter().any(|binding| {
        binding.instance_paths.iter().any(|path| path == "/top/X1") && binding.status.is_resolved()
    }));
}

#[test]
fn active_configuration_rejects_missing_dut_and_override_paths() {
    let mut workspace = ProjectWorkspace::default();
    let mut libraries = LibraryManager::default();
    workspace.ensure_library_model(&mut libraries);
    workspace
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: "Missing bindings".to_owned(),
            root: CellViewRef::default_top(),
            dut_path: "/top/XMISSING".to_owned(),
            executable_view_policy: vec!["schematic".to_owned(), "spice".to_owned()],
            stop_views: vec!["spice".to_owned()],
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: vec![crate::state::ConfigurationSetOverride {
                instance_path: "/top/XOTHER".to_owned(),
                executable_views: vec!["schematic".to_owned()],
                stop_view: None,
                model_section: None,
                eligible_platforms: crate::state::ConfigurationPlatform::ALL.to_vec(),
            }],
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Local project".to_owned(),
        })
        .expect("create configuration");

    let resolution = workspace.resolve_hierarchy(&libraries);

    assert_eq!(resolution.total_instances, 3);
    assert_eq!(resolution.resolved_instances, 1);
    assert_eq!(resolution.unresolved_instances(), 2);
    assert!(resolution.bindings.iter().any(|binding| {
        binding.diagnostic.as_deref().is_some_and(|diagnostic| {
            diagnostic.contains("configured DUT path /top/XMISSING does not exist")
        })
    }));
    assert!(resolution.bindings.iter().any(|binding| {
        binding.diagnostic.as_deref().is_some_and(|diagnostic| {
            diagnostic.contains("scoped configuration override /top/XOTHER does not exist")
        })
    }));
}

#[test]
fn reviewed_fallback_is_resolved_and_retained_in_the_hierarchy_receipt() {
    let mut workspace = ProjectWorkspace::default();
    let mut libraries = LibraryManager::default();
    workspace.ensure_library_model(&mut libraries);
    let top = workspace
        .schematic_buffers
        .get_mut(&CellViewRef::default_top().key())
        .expect("top buffer");
    top.add_library_cell_component(Point::new(20, 20), instance("work", "amp"));
    add_schematic_master(
        &mut libraries,
        &mut workspace,
        "work",
        "amp",
        SchematicState::default(),
    );
    workspace
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: "Reviewed fallback".to_owned(),
            root: CellViewRef::default_top(),
            dut_path: "/top/X1".to_owned(),
            executable_view_policy: vec!["spice".to_owned()],
            stop_views: vec!["spice".to_owned()],
            unresolved_policy: crate::state::UnresolvedBindingPolicy::ExplicitFallbackWithReview,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Local project".to_owned(),
        })
        .expect("create configuration");

    let resolution = workspace.resolve_hierarchy(&libraries);
    let fallback = resolution
        .bindings
        .iter()
        .find(|binding| binding.instance_paths == ["/top/X1"])
        .expect("child binding");

    assert!(fallback.status.is_resolved());
    assert!(fallback.used_review_fallback);
    assert_eq!(fallback.reference.view, "schematic");
    assert_eq!(
        fallback.view_search_order,
        ["spice", "schematic", "extracted"]
    );
}

#[test]
fn configuration_catalog_replacement_advances_project_revision_atomically() {
    let mut workspace = ProjectWorkspace::default();
    let original_revision = workspace.project.revision;
    let mut candidate = workspace.configuration_sets.clone();
    candidate
        .create(crate::state::ConfigurationSetDefinition {
            name: "Release".to_owned(),
            root: CellViewRef::default_top(),
            dut_path: "/top/X1".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Local project".to_owned(),
        })
        .expect("candidate configuration");

    let committed_revision = workspace
        .replace_configuration_sets(candidate.clone())
        .expect("publish configuration catalog");
    assert_eq!(workspace.project.revision, committed_revision);
    assert_ne!(workspace.project.revision, original_revision);
    assert_eq!(workspace.configuration_sets, candidate);
    assert!(workspace.project_metadata_dirty);

    let committed = workspace.clone();
    assert_eq!(
        workspace.replace_configuration_sets(candidate),
        Err(ProjectConfigurationMutationError::NoChanges)
    );
    assert_eq!(workspace.project.revision, committed.project.revision);
    assert_eq!(workspace.configuration_sets, committed.configuration_sets);
}

#[test]
fn configuration_catalog_replacement_rejects_unmaterialized_roots_atomically() {
    let mut workspace = ProjectWorkspace::default();
    let before = workspace.clone();
    let mut candidate = crate::state::ConfigurationSetCatalog::default();
    candidate
        .create(crate::state::ConfigurationSetDefinition {
            name: "Missing root".to_owned(),
            root: CellViewRef::new("user", "missing", "schematic"),
            dut_path: "/top/X1".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Local project".to_owned(),
        })
        .expect("structurally valid candidate");

    assert!(matches!(
        workspace.replace_configuration_sets(candidate),
        Err(ProjectConfigurationMutationError::MissingRootBuffer { .. })
    ));
    assert_eq!(workspace.project.revision, before.project.revision);
    assert_eq!(workspace.configuration_sets, before.configuration_sets);
    assert_eq!(
        workspace.project_metadata_dirty,
        before.project_metadata_dirty
    );
}

#[test]
fn design_management_projection_namespaces_sheets_and_materializes_explicit_ports() {
    use crate::state::{
        CrossSheetDiscipline, CrossSheetPortAnchor, CrossSheetPortDefinition,
        CrossSheetPortDirection, CrossSheetPortEndpoint, CrossSheetSignalType,
        MoveBoundaryResolution, MoveSelectionRequest, SheetDefinition, SheetPortPolicy,
        SheetTemplate,
    };

    let mut workspace = ProjectWorkspace::default();
    let key = CellViewRef::default_top().key();
    let mut schematic = SchematicState::default();
    let first = schematic
        .add_wire(vec![Point::origin(), Point::new(10, 0)])
        .expect("first wire");
    let second = schematic
        .add_wire(vec![Point::origin(), Point::new(0, 10)])
        .expect("second wire");
    let component = schematic.add_component(ComponentType::Resistor, Point::new(20, 0));
    let terminal_name = schematic
        .components
        .iter()
        .find(|candidate| candidate.id == component)
        .expect("component")
        .terminal_positions_resolved(None)
        .into_iter()
        .find(|(_, point)| *point == Point::origin())
        .map(|(name, _)| name)
        .expect("terminal at the second-wire anchor");
    schematic
        .connections
        .push(crate::state::WireConnection::new(
            second,
            0,
            component,
            terminal_name.clone(),
        ));
    let source_sheet = workspace
        .design_management
        .bootstrap_for_cell_view(&key, "Input", [first, second, component])
        .expect("bootstrap sheet ownership");
    let catalog = workspace
        .design_management
        .sheet_catalog_mut(&key)
        .expect("sheet catalog");
    let destination_sheet = catalog
        .create_sheet(
            SheetDefinition {
                name: "Output".to_owned(),
                template: SheetTemplate::AnalogSchematic,
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(source_sheet),
        )
        .expect("second sheet");
    catalog
        .move_selection(MoveSelectionRequest {
            expected_catalog_revision: catalog.revision(),
            object_ids: vec![second, component],
            destination_sheet_id: destination_sheet,
            boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
                ports: vec![CrossSheetPortDefinition {
                    net_name: "BIAS".to_owned(),
                    first: CrossSheetPortEndpoint {
                        sheet_id: source_sheet,
                        anchor: CrossSheetPortAnchor::WirePoint {
                            wire_id: first,
                            point: Point::origin(),
                        },
                    },
                    second: CrossSheetPortEndpoint {
                        sheet_id: destination_sheet,
                        anchor: CrossSheetPortAnchor::ComponentTerminal {
                            component_id: component,
                            terminal_name,
                        },
                    },
                    direction: CrossSheetPortDirection::Output,
                    signal_type: CrossSheetSignalType::Analog,
                    discipline: CrossSheetDiscipline::Electrical,
                }],
            },
        })
        .expect("move with explicit boundary contract");

    let projected = workspace
        .materialize_design_management_schematic(&key, &schematic)
        .expect("materialize governed design");
    let first_position = projected
        .wires
        .iter()
        .find(|wire| wire.id == first)
        .and_then(|wire| wire.points.first())
        .copied()
        .expect("first wire");
    let second_position = projected
        .wires
        .iter()
        .find(|wire| wire.id == second)
        .and_then(|wire| wire.points.first())
        .copied()
        .expect("second wire");
    assert_ne!(first_position, second_position);
    assert_eq!(first_position, Point::origin());
    assert_eq!(second_position, Point::new(1_000_000, 0));

    let mut port_positions = projected
        .net_labels
        .iter()
        .filter(|label| label.name == "BIAS")
        .map(|label| label.pos)
        .collect::<Vec<_>>();
    port_positions.sort_by_key(|point| point.x);
    assert_eq!(port_positions, [first_position, second_position]);
}

#[test]
fn design_management_projection_applies_active_variant_and_annotation() {
    use std::collections::BTreeMap;

    use crate::state::{
        AnnotationObject, AnnotationPosition, AssemblyVariantDraft, ComponentSubstitution,
        ProtectedReferencePolicy, RenumberOrder, RenumberRequest, RenumberScope,
        SchematicObjectKey, VariantInheritance, VariantObjectOverride, VariantQualificationPlan,
        VariantQualificationState,
    };

    let mut workspace = ProjectWorkspace::default();
    let key = CellViewRef::default_top().key();
    let mut schematic = SchematicState::default();
    let substituted = schematic.add_component(ComponentType::Resistor, Point::new(10, 10));
    let omitted = schematic.add_component(ComponentType::Capacitor, Point::new(20, 10));
    let variant = workspace
        .design_management
        .variants_mut()
        .create(AssemblyVariantDraft {
            name: "Automotive".to_owned(),
            parent_id: None,
            inheritance: VariantInheritance::OverrideChangedObjectsOnly,
            qualification_plan: VariantQualificationPlan::InvalidateAffectedTests,
            overrides: BTreeMap::from([
                (
                    SchematicObjectKey::new(&key, substituted)
                        .expect("scoped substituted identity"),
                    VariantObjectOverride::Substitute {
                        replacement: ComponentSubstitution {
                            library: "qualified".to_owned(),
                            cell: "resistor_aecq".to_owned(),
                            view: "schematic".to_owned(),
                            value_override: Some("2 kohm".to_owned()),
                            model_section: Some("automotive".to_owned()),
                            port_equivalence_digest: Some(ContentDigest::from_bytes([9; 32])),
                            qualification: VariantQualificationState::Current,
                        },
                    },
                ),
                (
                    SchematicObjectKey::new(&key, omitted).expect("scoped omitted identity"),
                    VariantObjectOverride::DoNotPopulate {
                        approval_reference: "ECO-104".to_owned(),
                    },
                ),
            ]),
        })
        .expect("create governed variant");
    workspace
        .design_management
        .variants_mut()
        .set_active(variant)
        .expect("activate variant");

    let request = RenumberRequest {
        scope: RenumberScope::WholeProject,
        order: RenumberOrder::HierarchyThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![AnnotationObject {
            object: SchematicObjectKey::new(&key, substituted).expect("scoped annotation identity"),
            current_reference: "R42".to_owned(),
            device_family: "R".to_owned(),
            sheet_id: None,
            hierarchy_path: "/top".to_owned(),
            position: AnnotationPosition { x: 10, y: 10 },
            connectivity_order: Some(1),
            locked: false,
            external: false,
            imported: false,
        }],
    };
    let preview = workspace
        .design_management
        .annotation()
        .preview_renumbering(&request)
        .expect("preview annotation");
    workspace
        .design_management
        .annotation_mut()
        .commit_renumbering(&preview, &request)
        .expect("commit annotation receipt");

    let projected = workspace
        .materialize_design_management_schematic(&key, &schematic)
        .expect("materialize variant and annotation");
    assert!(
        projected
            .components
            .iter()
            .all(|component| component.id != omitted)
    );
    assert!(
        projected
            .connections
            .iter()
            .all(|connection| connection.component_id != omitted)
    );
    let component = projected
        .components
        .iter()
        .find(|component| component.id == substituted)
        .expect("substituted component");
    let binding = component
        .library_cell
        .as_ref()
        .expect("qualified cell binding");
    assert_eq!(binding.library, "qualified");
    assert_eq!(binding.cell, "resistor_aecq");
    assert_eq!(component.value, "2 kohm");
    assert!(component.params.contains("model_section=automotive"));
    assert_eq!(component.name, "R1");
}

#[test]
fn hierarchy_resolution_reports_unbound_and_recursive_masters() {
    let mut workspace = ProjectWorkspace::default();
    let mut libraries = LibraryManager::default();
    workspace.ensure_library_model(&mut libraries);
    workspace
        .schematic_buffers
        .get_mut(&CellViewRef::default_top().key())
        .expect("top buffer")
        .add_library_cell_component(Point::new(20, 20), instance("missing", "unbound"));

    let unresolved = workspace.resolve_hierarchy(&libraries);
    assert_eq!(unresolved.total_instances, 2);
    assert_eq!(unresolved.resolved_instances, 1);
    assert_eq!(unresolved.unresolved_instances(), 1);
    assert_eq!(
        unresolved.bindings[1].status,
        HierarchyBindingStatus::Unresolved
    );
    assert!(unresolved.bindings[1].diagnostic.is_some());

    let top = workspace
        .schematic_buffers
        .get_mut(&CellViewRef::default_top().key())
        .expect("top buffer");
    top.components.clear();
    top.add_library_cell_component(Point::new(20, 20), instance("work", "loop"));
    let mut loop_master = SchematicState::default();
    loop_master.add_library_cell_component(Point::new(20, 20), instance("work", "loop"));
    add_schematic_master(&mut libraries, &mut workspace, "work", "loop", loop_master);

    let recursive = workspace.resolve_hierarchy(&libraries);
    assert_eq!(recursive.total_instances, 3);
    assert_eq!(recursive.resolved_instances, 2);
    let loop_row = recursive
        .bindings
        .iter()
        .find(|row| row.reference.cell == "loop")
        .expect("loop row");
    assert_eq!(loop_row.instance_count, 2);
    assert_eq!(loop_row.status, HierarchyBindingStatus::Recursive);
    assert!(
        loop_row
            .diagnostic
            .as_deref()
            .is_some_and(|diagnostic| diagnostic.contains("work/loop → work/loop"))
    );
}

#[test]
fn hierarchy_resolution_projects_unsaved_active_topology() {
    let mut workspace = ProjectWorkspace::default();
    let mut libraries = LibraryManager::default();
    workspace.ensure_library_model(&mut libraries);
    let mut live = workspace
        .schematic_buffers
        .get(&CellViewRef::default_top().key())
        .expect("top buffer")
        .clone();
    live.add_library_cell_component(Point::new(20, 20), instance("missing", "live_child"));

    let persisted = workspace.resolve_hierarchy(&libraries);
    let projected =
        workspace.resolve_hierarchy_with_active(&libraries, &workspace.active_view, &live);

    assert_eq!(persisted.total_instances, 1);
    assert_eq!(projected.total_instances, 2);
    assert_eq!(projected.unresolved_instances(), 1);
    assert!(
        projected
            .bindings
            .iter()
            .any(|binding| binding.reference.cell == "live_child"
                && binding.status == HierarchyBindingStatus::Unresolved)
    );
}

#[test]
fn hierarchy_resolution_rejects_orphan_schematic_buffers() {
    let mut workspace = ProjectWorkspace::default();
    let mut libraries = LibraryManager::default();
    workspace.ensure_library_model(&mut libraries);
    workspace
        .schematic_buffers
        .get_mut(&CellViewRef::default_top().key())
        .expect("top buffer")
        .add_library_cell_component(Point::new(20, 20), instance("orphan", "amp"));
    workspace.schematic_buffers.insert(
        CellViewRef::new("orphan", "amp", "schematic").key(),
        SchematicState::default(),
    );

    let resolution = workspace.resolve_hierarchy(&libraries);

    assert_eq!(resolution.unresolved_instances(), 1);
    assert!(
        resolution
            .bindings
            .iter()
            .any(|binding| binding.reference.cell == "amp"
                && binding.status == HierarchyBindingStatus::Unresolved)
    );
}

#[test]
fn configuration_veriloga_binding_uses_exact_project_bundle_on_all_targets() {
    let mut workspace = ProjectWorkspace::default();
    let mut libraries = LibraryManager::default();
    workspace.ensure_library_model(&mut libraries);
    let reference = CellViewRef::new("models", "amp", "veriloga");
    let mut view = View::new("veriloga", ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), "project_amp".to_owned());
    view.metadata
        .insert("veriloga.ports".to_owned(), r#"["in","out"]"#.to_owned());
    let mut cell = Cell::new("amp");
    cell.add_view(view);
    let mut library = Library::new("models");
    library.add_cell(cell);
    libraries.add_library(library);

    let bundle = ProjectSourceBundle::try_new(
            ProjectSourceOwner::cell_view(reference.clone()),
            ProjectSourceLanguage::VerilogA,
            "models/amp.va",
            "module project_amp(input in, output out); electrical in, out; analog V(out) <+ V(in); endmodule\n",
            [],
            [],
        )
        .expect("valid project source bundle");
    let bundle_id = bundle.id();
    workspace
        .project_sources
        .insert_bundle(bundle)
        .expect("attach project source bundle");

    let mut placed = LibraryCellInstance::new("models", "amp", "schematic");
    placed.terminal_order = vec!["in".to_owned(), "out".to_owned()];
    workspace
        .schematic_buffers
        .get_mut(&CellViewRef::default_top().key())
        .expect("top buffer")
        .add_library_cell_component(Point::new(20, 20), placed);
    workspace
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: "Mixed-signal".to_owned(),
            root: CellViewRef::default_top(),
            dut_path: "/top/X1".to_owned(),
            executable_view_policy: vec!["veriloga".to_owned()],
            stop_views: vec!["veriloga".to_owned()],
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Mixed-signal design".to_owned(),
        })
        .expect("create mixed-signal configuration");

    let active = workspace
        .active_schematic()
        .expect("active schematic")
        .clone();
    let projection = workspace
        .configuration_execution_projection(&libraries, &CellViewRef::default_top(), &active)
        .expect("resolve project-owned Verilog-A binding");
    let execution = projection
        .plan()
        .and_then(|plan| plan.binding("/top/X1"))
        .expect("exact execution binding");
    let behavioral = execution
        .project_veriloga()
        .expect("project Verilog-A contract");
    assert_eq!(behavioral.source_bundle_id(), bundle_id);
    assert_eq!(behavioral.selected_module(), "project_amp");
    assert!(behavioral.source_key().starts_with("__rspice_project__/"));
    assert_eq!(
        execution
            .materialized_binding()
            .and_then(|binding| binding.source_path.as_deref()),
        Some(Path::new(behavioral.source_key()))
    );
    assert_eq!(
        execution
            .materialized_binding()
            .and_then(|binding| binding.module_name.as_deref()),
        Some(behavioral.netlist_alias())
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn hierarchy_resolution_rejects_missing_and_conflicting_source_bindings() {
    let mut workspace = ProjectWorkspace::default();
    let mut libraries = LibraryManager::default();
    workspace.ensure_library_model(&mut libraries);
    let base = std::env::temp_dir().join(format!("rspice-hierarchy-{}", Uuid::new_v4()));
    let authoritative = base.join("amp.cir");
    let conflicting = base.join("other.cir");
    std::fs::create_dir_all(&base).expect("create source fixture directory");
    std::fs::write(&authoritative, ".subckt amp in out\n.ends amp\n")
        .expect("write authoritative source");
    std::fs::write(&conflicting, ".subckt amp in out\n.ends amp\n")
        .expect("write conflicting source");

    let missing_path = base.join("missing.cir");
    let mut library = Library::new("models");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("spice", ViewType::Spice).with_path(missing_path.clone()));
    library.add_cell(cell);
    libraries.add_library(library);

    let mut binding = LibraryCellInstance::new("models", "amp", "spice");
    binding.terminal_order = vec!["in".to_owned(), "out".to_owned()];
    binding.source_path = Some(missing_path);
    workspace
        .schematic_buffers
        .get_mut(&CellViewRef::default_top().key())
        .expect("top buffer")
        .add_library_cell_component(Point::new(20, 20), binding.clone());

    let missing = workspace.resolve_hierarchy(&libraries);
    assert_eq!(missing.unresolved_instances(), 1);
    assert!(missing.bindings.iter().any(|row| {
        row.diagnostic
            .as_deref()
            .is_some_and(|diagnostic| diagnostic.contains("cannot read"))
    }));

    libraries
        .get_library_mut("models")
        .and_then(|library| library.get_cell_mut("amp"))
        .and_then(|cell| cell.get_view_mut("spice"))
        .expect("authoritative source view")
        .file_path = Some(authoritative);
    binding.source_path = Some(conflicting);
    workspace
        .schematic_buffers
        .get_mut(&CellViewRef::default_top().key())
        .expect("top buffer")
        .components
        .last_mut()
        .expect("source-backed instance")
        .library_cell = Some(binding);
    let conflicting = workspace.resolve_hierarchy(&libraries);
    assert_eq!(conflicting.unresolved_instances(), 1);
    assert!(conflicting.bindings.iter().any(|row| {
        row.diagnostic
            .as_deref()
            .is_some_and(|diagnostic| diagnostic.contains("conflicts"))
    }));

    std::fs::remove_dir_all(base).expect("remove source fixture directory");
}

#[test]
fn descend_records_the_instance_names() {
    let mut workspace = ProjectWorkspace::default();
    workspace.open_as_root(reference("tb_ota"), ViewType::Schematic);
    workspace.descend_into("X1".into(), reference("ota_5t"), ViewType::Schematic);
    workspace.descend_into("XB".into(), reference("bias_2t"), ViewType::Schematic);

    assert_eq!(workspace.occurrence_labels(), ["tb_ota", "X1", "XB"]);
    assert_eq!(workspace.active_view.cell, "bias_2t");
}

#[test]
fn breadcrumb_focus_truncates_the_occurrence_path() {
    let mut workspace = ProjectWorkspace::default();
    workspace.open_as_root(reference("tb_ota"), ViewType::Schematic);
    workspace.descend_into("X1".into(), reference("ota_5t"), ViewType::Schematic);
    workspace.descend_into("XB".into(), reference("bias_2t"), ViewType::Schematic);

    workspace.focus_breadcrumb(1);
    assert_eq!(workspace.occurrence_labels(), ["tb_ota", "X1"]);
    assert_eq!(workspace.active_view.cell, "ota_5t");

    workspace.ascend_one();
    assert_eq!(workspace.occurrence_labels(), ["tb_ota"]);
    assert_eq!(workspace.active_view.cell, "tb_ota");
    // At the root, ascending is a no-op.
    assert!(workspace.ascend_one().is_none());
}

#[test]
fn legacy_stacks_fall_back_to_cell_names() {
    let mut workspace = ProjectWorkspace::default();
    workspace.open_as_root(reference("tb_ota"), ViewType::Schematic);
    // Simulate an older save: stack grew without instance labels.
    workspace.hierarchy_stack.push(reference("ota_5t"));
    assert_eq!(workspace.occurrence_labels(), ["tb_ota", "ota_5t"]);
}

#[test]
fn symbol_active_view_does_not_allocate_schematic_buffer() {
    let reference = symbol_reference("ota_5t");
    let mut workspace = ProjectWorkspace {
        active_view: reference.clone(),
        open_views: vec![OpenCellView::new(reference.clone(), ViewType::Symbol)],
        hierarchy_stack: vec![reference.clone()],
        schematic_buffers: HashMap::new(),
        ..ProjectWorkspace::default()
    };
    let mut libraries = LibraryManager::default();
    let mut library = Library::new("work");
    let mut cell = Cell::new("ota_5t");
    cell.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(cell);
    libraries.add_library(library);

    workspace.ensure_library_model(&mut libraries);

    assert_eq!(workspace.active_view_type(), ViewType::Symbol);
    assert!(
        !workspace.schematic_buffers.contains_key(&reference.key()),
        "symbol views must not be backed by stale schematic buffers"
    );
    let symbol_view = libraries
        .get_library("work")
        .and_then(|library| library.get_cell("ota_5t"))
        .and_then(|cell| cell.get_view("symbol"))
        .expect("symbol view still exists");
    assert_eq!(symbol_view.view_type, ViewType::Symbol);
}

#[test]
fn saving_while_symbol_active_does_not_create_symbol_schematic_buffer() {
    let reference = symbol_reference("ota_5t");
    let mut workspace = ProjectWorkspace {
        active_view: reference.clone(),
        open_views: vec![OpenCellView::new(reference.clone(), ViewType::Symbol)],
        hierarchy_stack: vec![reference.clone()],
        schematic_buffers: HashMap::new(),
        ..ProjectWorkspace::default()
    };

    workspace.save_active_schematic(&SchematicState::default());

    assert!(
        !workspace.schematic_buffers.contains_key(&reference.key()),
        "session restore/save paths must not persist default schematics under symbol views"
    );
}

#[test]
fn project_identity_is_stable_and_rename_is_atomic() {
    let mut project = ProjectDescriptor::default();
    let id = project.id();
    let initial_revision = project.revision();

    let renamed_revision = project
        .rename("Precision ΔΣ ADC")
        .expect("valid Unicode name");

    assert_eq!(project.id(), id);
    assert_eq!(project.name(), "Precision ΔΣ ADC");
    assert_eq!(renamed_revision.get(), initial_revision.get() + 1);
    assert_eq!(
        project.rename("Precision ΔΣ ADC").expect("no-op rename"),
        renamed_revision
    );

    let rejected = project.rename("bad/name");
    assert!(matches!(
        rejected,
        Err(ProjectDescriptorError::PathSeparator('/'))
    ));
    assert_eq!(project.name(), "Precision ΔΣ ADC");
    assert_eq!(project.revision(), renamed_revision);
    assert_eq!(project.id(), id);
}

#[test]
fn legacy_project_descriptor_identity_migration_is_deterministic() {
    let original = ProjectDescriptor::default();
    let mut legacy = serde_json::to_value(&original).expect("descriptor serializes");
    legacy
        .as_object_mut()
        .expect("descriptor is an object")
        .remove("id");
    legacy
        .as_object_mut()
        .expect("descriptor is an object")
        .remove("schema_version");
    legacy
        .as_object_mut()
        .expect("descriptor is an object")
        .remove("revision");
    let legacy_json = serde_json::to_string(&legacy).expect("legacy descriptor serializes");

    let first: ProjectDescriptor =
        serde_json::from_str(&legacy_json).expect("legacy descriptor restores");
    let second: ProjectDescriptor =
        serde_json::from_str(&legacy_json).expect("legacy descriptor restores again");

    assert_eq!(first.id(), second.id());
    assert!(!first.id().as_uuid().is_nil());
    assert_ne!(first.id(), original.id());

    let persisted = serde_json::to_value(&first).expect("migrated descriptor serializes");
    assert_eq!(
        persisted.get("id"),
        Some(&serde_json::to_value(first.id()).expect("identity serializes"))
    );
}

#[test]
fn versioned_or_explicitly_null_project_identity_and_schema_are_rejected() {
    let project = ProjectDescriptor::default();
    let mut missing = serde_json::to_value(&project).expect("descriptor serializes");
    missing
        .as_object_mut()
        .expect("descriptor object")
        .remove("id");
    let missing_error = serde_json::from_value::<ProjectDescriptor>(missing)
        .expect_err("versioned descriptor must retain identity");
    assert!(
        missing_error
            .to_string()
            .contains("missing its stable identity")
    );

    let mut null = serde_json::to_value(&project).expect("descriptor serializes");
    null["id"] = serde_json::Value::Null;
    let null_error = serde_json::from_value::<ProjectDescriptor>(null)
        .expect_err("explicit null identity is not legacy absence");
    assert!(
        null_error
            .to_string()
            .contains("must not be explicitly null")
    );

    let mut unversioned_null = serde_json::to_value(&project).expect("descriptor serializes");
    unversioned_null
        .as_object_mut()
        .expect("descriptor object")
        .remove("schema_version");
    unversioned_null["id"] = serde_json::Value::Null;
    let unversioned_null_error = serde_json::from_value::<ProjectDescriptor>(unversioned_null)
        .expect_err("unversioned explicit null is not genuine legacy absence");
    assert!(
        unversioned_null_error
            .to_string()
            .contains("must not be explicitly null")
    );

    let mut null_schema = serde_json::to_value(&project).expect("descriptor serializes");
    null_schema["schema_version"] = serde_json::Value::Null;
    let null_schema_error = serde_json::from_value::<ProjectDescriptor>(null_schema)
        .expect_err("explicit null schema is not an unversioned descriptor");
    assert!(
        null_schema_error
            .to_string()
            .contains("schema version must not be explicitly null")
    );
}

#[test]
fn project_name_contract_counts_graphemes_and_rejects_unsafe_text() {
    let family = "👨‍👩‍👧‍👦";
    assert!(ProjectDescriptor::validate_name(&family.repeat(120)).is_ok());
    assert!(matches!(
        ProjectDescriptor::validate_name(&family.repeat(121)),
        Err(ProjectDescriptorError::NameTooLong {
            grapheme_count: 121
        })
    ));
    assert!(matches!(
        ProjectDescriptor::validate_name(" leading"),
        Err(ProjectDescriptorError::SurroundingWhitespace)
    ));
    assert!(matches!(
        ProjectDescriptor::validate_name("line\nfeed"),
        Err(ProjectDescriptorError::ControlCharacter('\n'))
    ));
    assert!(matches!(
        ProjectDescriptor::validate_name("path\\name"),
        Err(ProjectDescriptorError::PathSeparator('\\'))
    ));
}

#[test]
fn cell_view_name_contract_keeps_slash_delimited_keys_injective() {
    for valid in ["user", "bandgap_2", "ΔΣ"] {
        assert!(validate_cell_view_name_segment(valid).is_ok(), "{valid}");
    }
    assert_eq!(
        validate_cell_view_name_segment(""),
        Err(CellViewNameError::Empty)
    );
    assert_eq!(
        validate_cell_view_name_segment("bad/name"),
        Err(CellViewNameError::UnsupportedCharacter('/'))
    );
    assert_eq!(
        validate_cell_view_name_segment("has space"),
        Err(CellViewNameError::UnsupportedCharacter(' '))
    );
}

#[test]
fn changing_source_path_does_not_rename_an_existing_project() {
    let mut project = ProjectDescriptor::default();
    project.set_path(PathBuf::from("first-save.rspiceproj"));
    let revision = project.revision();

    assert_eq!(project.name(), "first-save");
    project.set_path(PathBuf::from("moved-copy.rspiceproj"));

    assert_eq!(project.name(), "first-save");
    assert_eq!(project.revision(), revision);
    assert_eq!(
        project.path.as_deref(),
        Some(Path::new("moved-copy.rspiceproj"))
    );
}

#[test]
fn project_copy_has_independent_identity_without_rebinding_source() {
    let mut source = ProjectDescriptor::default();
    source
        .rename("Precision reference")
        .expect("source name is valid");
    source.set_path(PathBuf::from("source.rspiceproj"));
    let source_id = source.id();
    let source_revision = source.revision();
    let source_path = source.path.clone();

    let copy = source.fork_copy_at(PathBuf::from("copy.rspiceproj"));

    assert_ne!(copy.id(), source_id);
    assert_eq!(copy.revision(), ObjectRevision::INITIAL);
    assert_eq!(copy.name(), source.name());
    assert_eq!(copy.path.as_deref(), Some(Path::new("copy.rspiceproj")));
    assert_eq!(source.id(), source_id);
    assert_eq!(source.revision(), source_revision);
    assert_eq!(source.path, source_path);
}

#[test]
fn generated_netlist_cannot_be_promoted_by_an_editor_write() {
    let mut workspace = ProjectWorkspace::default();

    assert!(!workspace.replace_editable_netlist_source("edited\n.end\n".to_owned()));
    assert!(workspace.netlist_source.is_none());
    assert!(!workspace.netlist_source_dirty);
    assert!(!workspace.any_dirty());
}

#[test]
fn explicit_editable_copy_enters_project_dirty_lifecycle() {
    let mut workspace = ProjectWorkspace::default();
    workspace.netlist_source_path = Some(PathBuf::from("generated.sp"));

    assert!(workspace.make_netlist_editable_copy("generated\n.op\n.end\n"));
    assert_eq!(
        workspace.netlist_source.as_deref(),
        Some("generated\n.op\n.end\n")
    );
    assert!(workspace.netlist_source_path.is_none());
    assert!(workspace.netlist_source_dirty);
    assert!(workspace.any_dirty());

    workspace.mark_all_clean();
    assert!(workspace.has_editable_netlist_source());
    assert!(!workspace.netlist_source_dirty);
    assert!(!workspace.any_dirty());
}

#[test]
fn editable_copy_does_not_overwrite_existing_owned_source() {
    let mut workspace = ProjectWorkspace::default();
    workspace.netlist_source = Some("owned\n.end\n".to_owned());
    workspace.netlist_source_path = Some(PathBuf::from("owned.cir"));

    assert!(!workspace.make_netlist_editable_copy("generated\n.end\n"));
    assert_eq!(workspace.netlist_source.as_deref(), Some("owned\n.end\n"));
    assert_eq!(
        workspace.netlist_source_path.as_deref(),
        Some(Path::new("owned.cir"))
    );
    assert!(!workspace.netlist_source_dirty);
}

#[test]
fn editing_imported_source_preserves_its_dependency_origin() {
    let mut workspace = ProjectWorkspace::default();
    workspace.netlist_source = Some("owned\n.end\n".to_owned());
    workspace.netlist_source_path = Some(PathBuf::from("decks/owned.cir"));

    assert!(workspace.replace_editable_netlist_source("edited\n.end\n".to_owned()));
    assert_eq!(
        workspace.netlist_source_path.as_deref(),
        Some(Path::new("decks/owned.cir"))
    );
    assert!(workspace.netlist_source_dirty);
}

#[test]
fn returning_to_generated_output_is_saved_as_a_project_change() {
    let mut workspace = ProjectWorkspace::default();
    workspace.netlist_source = Some("owned\n.end\n".to_owned());
    workspace.netlist_source_path = Some(PathBuf::from("owned.cir"));

    assert!(workspace.return_to_generated_netlist());
    assert!(workspace.netlist_source.is_none());
    assert!(workspace.netlist_source_path.is_none());
    assert!(workspace.netlist_source_dirty);
    assert!(workspace.any_dirty());
    assert!(!workspace.return_to_generated_netlist());
}

fn technology_binding_fixture() -> ProjectTechnologyBinding {
    let root = PathBuf::from(r"C:\qualified-pdk\models.lib");
    ProjectTechnologyBinding {
        schema_version: PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION,
        package_name: "Qualified analog models".to_owned(),
        package_version: Some("2026.07".to_owned()),
        technology_node: Some("180 nm".to_owned()),
        model_library: "qualified_analog".to_owned(),
        root_source: root.clone(),
        source_closure: vec![crate::state::model_library::ModelSourcePin {
            path: root,
            digest: crate::product::ContentDigest::from_bytes([0x4a; 32]),
        }],
        source_edges: Vec::new(),
        model_count: 14,
        process_sections: vec!["ff".to_owned(), "ss".to_owned(), "tt".to_owned()],
        signed_package: None,
    }
}

#[test]
fn signed_technology_pin_is_strictly_validated_and_round_trips_with_the_project_binding() {
    let binding = technology_binding_fixture();
    let mut value = serde_json::to_value(&binding).expect("binding serializes");
    value["signed_package"] = serde_json::json!({
        "schema_version": PROJECT_SIGNED_TECHNOLOGY_PIN_SCHEMA_VERSION,
        "package_id": "demo180",
        "revision": "2.3.1",
        "manifest_digest": crate::product::ContentDigest::from_bytes([0x11; 32]),
        "archive_digest": crate::product::ContentDigest::from_bytes([0x22; 32]),
        "technology_name": "Demo 180 nm",
        "publisher_id": "rspice-foundry-demo",
        "signing_key_id": "ceremony-01",
        "process_node_nm": 180,
        "stack_name": "1P2M",
        "execution_targets": ["desktop", "web-assembly", "mobile"]
    });
    let restored: ProjectTechnologyBinding =
        serde_json::from_value(value.clone()).expect("strict signed pin deserializes");
    restored.validate().expect("signed binding validates");
    assert_eq!(
        serde_json::from_str::<ProjectTechnologyBinding>(
            &serde_json::to_string(&restored).expect("binding serializes")
        )
        .expect("binding round trips"),
        restored
    );

    value["signed_package"]["execution_targets"] = serde_json::json!(["desktop", "desktop"]);
    let duplicate_targets: ProjectTechnologyBinding =
        serde_json::from_value(value).expect("shape deserializes");
    assert!(matches!(
        duplicate_targets.validate(),
        Err(TechnologyBindingError::InvalidSignedPackage(_))
    ));
}

#[test]
fn technology_attachment_is_atomic_revisioned_and_idempotent() {
    let mut project = ProjectDescriptor::default();
    let initial_revision = project.revision();
    let binding = technology_binding_fixture();

    let committed = project
        .attach_technology(binding.clone())
        .expect("valid binding commits");
    assert_eq!(committed.get(), initial_revision.get() + 1);
    assert_eq!(project.technology_binding(), Some(&binding));
    assert_eq!(
        project.technology.as_deref(),
        Some(binding.display_label().as_str())
    );
    assert_eq!(
        project
            .attach_technology(binding)
            .expect("identical binding is a no-op"),
        committed
    );

    let mut rejected = technology_binding_fixture();
    rejected.model_count = 0;
    let before = project.clone();
    assert!(matches!(
        project.attach_technology(rejected),
        Err(ProjectDescriptorError::Technology(
            TechnologyBindingError::NoModels
        ))
    ));
    assert_eq!(project.revision(), before.revision());
    assert_eq!(project.technology, before.technology);
    assert_eq!(project.technology_binding(), before.technology_binding());
}

#[test]
fn cloud_publication_binding_is_revisioned_idempotent_and_validated() {
    let mut project = ProjectDescriptor::default();
    let initial_revision = project.revision();
    let binding = ProjectCloudPublicationBinding::new(
        "0198c1c2-aaaa-7000-8000-000000000001".to_owned(),
        "0198c1c2-bbbb-7000-8000-000000000002".to_owned(),
    )
    .expect("service identifiers validate");

    let committed = project
        .bind_cloud_publication(binding.clone())
        .expect("binding commits");
    assert_eq!(committed.get(), initial_revision.get() + 1);
    assert_eq!(project.cloud_publication(), Some(&binding));
    project.validate().expect("bound project validates");
    assert_eq!(
        project
            .bind_cloud_publication(binding.clone())
            .expect("identical binding is a no-op"),
        committed
    );

    let mut value = serde_json::to_value(&project).expect("descriptor serializes");
    let restored: ProjectDescriptor =
        serde_json::from_value(value.clone()).expect("descriptor round trips");
    assert_eq!(restored.cloud_publication(), Some(&binding));
    // Legacy descriptors without the field restore to an unbound project.
    value
        .as_object_mut()
        .expect("descriptor object")
        .remove("cloud_publication");
    let unbound: ProjectDescriptor =
        serde_json::from_value(value).expect("legacy shape deserializes");
    assert_eq!(unbound.cloud_publication(), None);

    assert!(matches!(
        ProjectCloudPublicationBinding::new(String::new(), "circuit".to_owned()),
        Err(ProjectDescriptorError::InvalidCloudPublicationBinding(
            "workspace_id"
        ))
    ));
    assert!(matches!(
        ProjectCloudPublicationBinding::new("workspace".to_owned(), "has space".to_owned()),
        Err(ProjectDescriptorError::InvalidCloudPublicationBinding(
            "circuit_id"
        ))
    ));
}

#[test]
fn technology_change_receipts_are_checkpoint_bound_atomic_and_tamper_evident() {
    let mut project = ProjectDescriptor::default();
    let initial_revision = project.revision();
    let authority = ProjectTechnologyChangeAuthority::new(
        "engineer.james",
        "project-technology-admin",
        "Attach the qualified tapeout technology",
    )
    .expect("authority validates");
    let context = ProjectTechnologyChangeContext::new(
        authority,
        Uuid::new_v4(),
        initial_revision,
        1_785_430_000_000,
        crate::product::ContentDigest::from_bytes([0x71; 32]),
        4_096,
    )
    .expect("checkpoint context validates");
    let binding = technology_binding_fixture();
    let (attached_revision, first_receipt) = project
        .attach_technology_audited(binding.clone(), context)
        .expect("audited attachment commits");
    assert_eq!(first_receipt.sequence(), 1);
    assert_eq!(
        first_receipt.action(),
        ProjectTechnologyChangeAction::Attach
    );
    assert_eq!(attached_revision.get(), initial_revision.get() + 1);
    assert_eq!(project.technology_change_audit().len(), 1);
    project.validate().expect("audited project validates");

    let mut bypass = binding.clone();
    bypass.package_version = Some("2026.08".to_owned());
    assert_eq!(
        project.attach_technology(bypass.clone()),
        Err(ProjectDescriptorError::TechnologyAuditRequired)
    );
    assert_eq!(project.technology_binding(), Some(&binding));

    project
        .rename("Audited technology project")
        .expect("unrelated project revision advances");
    let replacement_from = project.revision();
    let replacement_context = ProjectTechnologyChangeContext::new(
        ProjectTechnologyChangeAuthority::new(
            "engineer.james",
            "project-technology-admin",
            "Adopt qualified model update",
        )
        .expect("replacement authority validates"),
        Uuid::new_v4(),
        replacement_from,
        1_785_430_100_000,
        crate::product::ContentDigest::from_bytes([0x72; 32]),
        8_192,
    )
    .expect("replacement checkpoint context validates");
    let (_, replacement_receipt) = project
        .attach_technology_audited(bypass.clone(), replacement_context)
        .expect("audited replacement commits");
    assert_eq!(replacement_receipt.sequence(), 2);
    assert_eq!(
        replacement_receipt.action(),
        ProjectTechnologyChangeAction::Replace
    );
    assert_eq!(
        replacement_receipt.source_project_revision(),
        replacement_from
    );
    assert_eq!(project.technology_binding(), Some(&bypass));
    project.validate().expect("replacement chain validates");

    let encoded = serde_json::to_vec(&project).expect("descriptor serializes");
    let restored: ProjectDescriptor =
        serde_json::from_slice(&encoded).expect("descriptor round trips");
    restored.validate().expect("restored audit validates");
    assert_eq!(
        serde_json::to_value(&restored).expect("restored descriptor serializes"),
        serde_json::to_value(&project).expect("source descriptor serializes")
    );

    let mut tampered = serde_json::to_value(&project).expect("descriptor serializes to value");
    tampered["technology_change_audit"][0]["reason"] =
        serde_json::Value::String("tampered reason".to_owned());
    let tampered: ProjectDescriptor =
        serde_json::from_value(tampered).expect("tampered shape deserializes");
    assert!(matches!(
        tampered.validate(),
        Err(ProjectDescriptorError::TechnologyAuditCorrupted(_))
    ));

    let copied = project.fork_copy_at(PathBuf::from(r"C:\projects\fork.rspice"));
    assert!(copied.technology_change_audit().is_empty());
    copied
        .validate()
        .expect("independent copy starts fresh history");
}

#[test]
fn signed_revision_replacement_requires_exact_diff_evidence_and_retains_it() {
    let (baseline_bytes, candidate_bytes, trust, authority) =
        crate::state::pdk_config::signed_technology_diff_test_fixture();
    let mut registry = crate::state::pdk_config::PdkTechnologyRegistry::default();
    registry
        .install_archive_bytes(&baseline_bytes, &trust, &authority, "install baseline")
        .expect("baseline installs");
    registry
        .install_archive_bytes(&candidate_bytes, &trust, &authority, "install candidate")
        .expect("candidate installs");
    let baseline = registry
        .validated_packages()
        .iter()
        .find(|package| package.manifest().revision == "2.3.1")
        .expect("baseline package");
    let candidate = registry
        .validated_packages()
        .iter()
        .find(|package| package.manifest().revision == "2.4.0")
        .expect("candidate package");
    let diff = crate::state::pdk_config::PdkTechnologyRevisionDiff::between(baseline, candidate)
        .expect("exact revision diff");
    let evidence = crate::state::pdk_config::PdkTechnologyMigrationEvidence::from_diff(&diff)
        .expect("migration evidence");
    let baseline_binding = technology_binding_fixture()
        .with_signed_package(baseline)
        .expect("baseline binding");
    let candidate_binding = technology_binding_fixture()
        .with_signed_package(candidate)
        .expect("candidate binding");
    let context = |revision, digest_byte| {
        ProjectTechnologyChangeContext::new(
            ProjectTechnologyChangeAuthority::new(
                "engineer.james",
                "project-technology-admin",
                "Review exact signed PDK revision transition",
            )
            .unwrap(),
            Uuid::new_v4(),
            revision,
            1_785_430_000_000,
            crate::product::ContentDigest::from_bytes([digest_byte; 32]),
            4_096,
        )
        .unwrap()
    };

    let mut project = ProjectDescriptor::default();
    let initial = project.revision();
    project
        .attach_technology_audited(baseline_binding, context(initial, 0x81))
        .expect("initial attachment");
    let before_replacement = project.clone();
    assert_eq!(
        project.attach_technology_audited(
            candidate_binding.clone(),
            context(project.revision(), 0x82),
        ),
        Err(ProjectDescriptorError::MissingTechnologyMigrationEvidence)
    );
    assert_eq!(
        serde_json::to_value(&project).expect("project serializes"),
        serde_json::to_value(&before_replacement).expect("baseline serializes")
    );

    let replacement_context = context(project.revision(), 0x83)
        .with_migration_evidence(evidence.clone())
        .expect("evidence context");
    let (_, receipt) = project
        .attach_technology_audited(candidate_binding, replacement_context)
        .expect("reviewed replacement commits");
    assert_eq!(receipt.migration_evidence(), Some(&evidence));
    project.validate().expect("evidenced chain validates");
}

#[test]
fn attached_technology_detects_exact_catalog_drift() {
    let root = PathBuf::from(r"C:\qualified-pdk\models.lib");
    let bytes = b".model nch nmos level=1\n".to_vec();
    let digest = crate::product::ContentDigest::from_bytes(sha2::Sha256::digest(&bytes).into());
    let mut library = crate::state::model_library::ModelLibrary::new("qualified_analog")
        .with_technology("Qualified analog models", "180 nm");
    library.version = "2026.07".to_owned();
    library.root_path = Some(root.clone());
    library.source_closure = vec![crate::state::model_library::ModelSourcePin {
        path: root.clone(),
        digest,
    }];
    library.source_contents =
        vec![crate::state::model_library::ModelSourceContent { path: root, bytes }];
    library.add_model(crate::state::model_library::DeviceModel::new(
        "nch",
        crate::state::model_library::ModelType::Nmos,
    ));
    let binding = ProjectTechnologyBinding::from_model_library(&library)
        .expect("exact retained source is attachable");
    binding
        .validate_model_library(&library)
        .expect("unchanged catalog matches");

    library.version = "2026.08".to_owned();
    assert!(matches!(
        binding.validate_model_library(&library),
        Err(TechnologyBindingError::CatalogDrift { .. })
    ));
}

#[test]
fn technology_binding_persists_while_runtime_dirty_state_resets() {
    let mut workspace = ProjectWorkspace::default();
    let binding = technology_binding_fixture();
    workspace
        .attach_technology(binding.clone())
        .expect("valid binding commits");
    assert!(workspace.any_dirty());

    let bytes = serde_json::to_vec(&workspace).expect("workspace serializes");
    let restored: ProjectWorkspace = serde_json::from_slice(&bytes).expect("workspace restores");

    assert_eq!(restored.project.technology_binding(), Some(&binding));
    restored
        .project
        .validate()
        .expect("restored binding validates");
    assert!(!restored.any_dirty());
}

#[test]
fn hardcopy_page_setup_persists_and_uses_project_dirty_lifecycle() {
    use crate::hardcopy::{
        ActiveHardcopySource, HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope,
        HardcopySetup, SetupSaveDisposition,
    };

    let source = ActiveHardcopySource::try_new(
        HardcopyDocumentId::try_from_uuid(uuid::Uuid::from_u128(0x4852_4450_5901))
            .expect("stable fixture identity"),
        crate::product::ObjectRevision::INITIAL,
        crate::product::ContentDigest::from_bytes([0x48; 32]),
        "top / schematic",
        HardcopyDocumentKind::SchematicOrSymbol,
        HardcopyScope::CurrentSheet,
    )
    .expect("valid hardcopy source");
    let mut workspace = ProjectWorkspace::default();

    let first = workspace
        .save_hardcopy_setup(&source, HardcopySetup::default())
        .expect("page setup commits");
    assert_eq!(first.disposition(), SetupSaveDisposition::Inserted);
    assert!(workspace.hardcopy_setups_dirty);
    assert!(workspace.any_dirty());

    let bytes = serde_json::to_vec(&workspace).expect("workspace serializes");
    let mut restored: ProjectWorkspace =
        serde_json::from_slice(&bytes).expect("workspace restores");
    assert_eq!(restored.hardcopy_setups.len(), 1);
    assert!(!restored.hardcopy_setups_dirty);
    assert!(!restored.any_dirty());

    let unchanged = restored
        .save_hardcopy_setup(&source, HardcopySetup::default())
        .expect("identical setup is accepted");
    assert_eq!(unchanged.disposition(), SetupSaveDisposition::Unchanged);
    assert!(!restored.hardcopy_setups_dirty);
    assert!(!restored.any_dirty());
}

#[test]
fn project_print_mapping_routes_through_project_dirty_lifecycle() {
    let mapping = crate::hardcopy::PrintMappingTable::try_new(
        crate::hardcopy::PrintMappingSaveScope::ProjectPrintSet("documentation".to_owned()),
        Vec::new(),
    )
    .unwrap();
    let mut workspace = ProjectWorkspace::default();
    let receipt = workspace
        .save_project_print_mapping(mapping.clone())
        .unwrap();
    assert_eq!(
        receipt.disposition(),
        crate::hardcopy::PrintMappingSaveDisposition::Created
    );
    assert!(workspace.project_print_mappings_dirty);
    assert!(workspace.any_dirty());

    let bytes = serde_json::to_vec(&workspace).unwrap();
    let mut restored: ProjectWorkspace = serde_json::from_slice(&bytes).unwrap();
    assert!(
        restored
            .project_print_mappings
            .get("documentation")
            .is_some()
    );
    assert!(!restored.any_dirty());

    let unchanged = restored.save_project_print_mapping(mapping).unwrap();
    assert_eq!(
        unchanged.disposition(),
        crate::hardcopy::PrintMappingSaveDisposition::Unchanged
    );
    assert!(!restored.any_dirty());
}

#[test]
fn hardcopy_source_sets_persist_validate_and_use_project_dirty_lifecycle() {
    use crate::hardcopy::sources::{HardcopySourceSet, HardcopySourceSetMember};
    use crate::hardcopy::{HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope};

    let member_id =
        HardcopyDocumentId::try_from_uuid(uuid::Uuid::from_u128(0x4853_4d45_4d42_4552)).unwrap();
    let set_id =
        HardcopyDocumentId::try_from_uuid(uuid::Uuid::from_u128(0x4853_5345_5449_4431)).unwrap();
    let member = HardcopySourceSetMember::try_new(
        "project:test:sheet:1",
        "Sheet 1",
        member_id,
        crate::product::ObjectRevision::INITIAL,
        crate::product::ContentDigest::from_bytes([0x51; 32]),
        HardcopyScope::CurrentSheet,
    )
    .unwrap();
    let source_set = HardcopySourceSet::try_new(
        set_id,
        crate::product::ObjectRevision::INITIAL,
        "Review set",
        HardcopyDocumentKind::SchematicOrSymbol,
        HardcopyScope::NamedPrintSet("Review set".to_owned()),
        vec![member],
    )
    .unwrap();
    let source_key = source_set.source_key();
    let mut workspace = ProjectWorkspace::default();

    assert!(workspace.save_hardcopy_source_set(source_set).unwrap());
    assert!(!workspace.hardcopy_source_sets().is_empty());
    assert!(workspace.hardcopy_source_set(&source_key).is_some());
    assert!(workspace.any_dirty());

    let bytes = serde_json::to_vec(&workspace).unwrap();
    let mut restored: ProjectWorkspace = serde_json::from_slice(&bytes).unwrap();
    restored.validate_simulation_configuration().unwrap();
    assert_eq!(restored.hardcopy_source_sets().len(), 1);
    assert!(!restored.any_dirty());
    assert!(restored.remove_hardcopy_source_set(&source_key));
    assert!(restored.hardcopy_source_sets().is_empty());
    assert!(restored.any_dirty());
}

#[test]
fn hardcopy_source_set_catalog_rejects_case_folded_duplicate_names() {
    use crate::hardcopy::sources::{HardcopySourceSet, HardcopySourceSetMember};
    use crate::hardcopy::{HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope};

    let build_set = |seed: u128, name: &str| {
        let member_id = HardcopyDocumentId::try_from_uuid(uuid::Uuid::from_u128(seed)).unwrap();
        let set_id =
            HardcopyDocumentId::try_from_uuid(uuid::Uuid::from_u128(seed + 0x1000)).unwrap();
        let member = HardcopySourceSetMember::try_new(
            format!("project:test:sheet:{seed}"),
            format!("Sheet {seed}"),
            member_id,
            crate::product::ObjectRevision::INITIAL,
            crate::product::ContentDigest::from_bytes([(seed & 0xff) as u8; 32]),
            HardcopyScope::CurrentSheet,
        )
        .unwrap();
        HardcopySourceSet::try_new(
            set_id,
            crate::product::ObjectRevision::INITIAL,
            name,
            HardcopyDocumentKind::SchematicOrSymbol,
            HardcopyScope::NamedPrintSet(name.to_owned()),
            vec![member],
        )
        .unwrap()
    };
    let mut workspace = ProjectWorkspace::default();
    workspace
        .save_hardcopy_source_set(build_set(0x5100, "Tapeout"))
        .unwrap();
    let error = workspace
        .save_hardcopy_source_set(build_set(0x5200, "tapeout"))
        .unwrap_err();
    assert!(matches!(
        error,
        HardcopySourceSetPersistenceError::DuplicateName { .. }
    ));
    assert_eq!(workspace.hardcopy_source_sets().len(), 1);
}

#[test]
fn corrupted_persisted_technology_contract_fails_project_validation() {
    let mut project = ProjectDescriptor::default();
    project
        .attach_technology(technology_binding_fixture())
        .expect("fixture binding commits");
    let mut encoded = serde_json::to_value(&project).expect("descriptor serializes");
    encoded["technology_binding"]["root_source"] =
        serde_json::Value::String("relative/models.lib".to_owned());
    let restored: ProjectDescriptor =
        serde_json::from_value(encoded).expect("descriptor shape restores");

    assert!(matches!(
        restored.validate(),
        Err(ProjectDescriptorError::Technology(
            TechnologyBindingError::NonAbsoluteSource(_)
        ))
    ));
}

#[test]
fn legacy_workspaces_restore_with_no_project_source_examples() {
    let mut value = serde_json::to_value(ProjectWorkspace::default()).unwrap();
    value.as_object_mut().unwrap().remove("project_sources");

    let restored: ProjectWorkspace = serde_json::from_value(value).unwrap();

    assert!(restored.project_sources.is_empty());
    assert!(!restored.project_sources_dirty);
}

#[test]
fn only_bootstrapped_projects_receive_exact_canonical_code_sources() {
    let mut libraries = LibraryManager::default();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let verilog_a = workspace
        .project_sources
        .get(ProjectSourceLanguage::VerilogA)
        .unwrap();
    let automation = workspace
        .project_sources
        .get(ProjectSourceLanguage::RSpiceAutomation)
        .unwrap();

    assert_eq!(verilog_a.file_name(), "sensor_bridge.va");
    assert_eq!(
        verilog_a.content(),
        "`include \"constants.vams\"\nmodule sensor_bridge(out, inp, inn);\n  parameter real gain = 100.0 from (0:inf);\n  analog V(out) <+ gain * (V(inp)-V(inn));\nendmodule"
    );
    assert_eq!(automation.file_name(), "characterize.py");
    assert_eq!(
        automation.content(),
        crate::state::DEFAULT_AUTOMATION_PYTHON,
    );
    let automation_bundle = workspace
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::code_workspace(
            ProjectSourceLanguage::RSpiceAutomation,
        ))
        .expect("bootstrapped Automation bundle");
    assert_eq!(
        automation_bundle
            .files()
            .iter()
            .map(|file| (file.logical_path(), file.content()))
            .collect::<Vec<_>>(),
        vec![
            (
                "permissions.toml",
                crate::state::DEFAULT_AUTOMATION_PERMISSIONS,
            ),
            ("requirements.lock", crate::state::DEFAULT_ENVIRONMENT_LOCK,),
            (
                "runplan.rspice.yaml",
                crate::state::DEFAULT_AUTOMATION_RUN_PLAN,
            ),
        ]
    );
    assert_eq!(automation_bundle.dependencies().len(), 3);
    assert_eq!(
        automation_bundle
            .roles()
            .iter()
            .map(|binding| (binding.logical_path(), binding.role()))
            .collect::<Vec<_>>(),
        vec![
            ("characterize.py", ProjectSourceRole::AutomationEntry),
            ("runplan.rspice.yaml", ProjectSourceRole::AutomationRunPlan,),
            (
                "requirements.lock",
                ProjectSourceRole::AutomationEnvironmentLock,
            ),
            (
                "permissions.toml",
                ProjectSourceRole::AutomationPermissionManifest,
            ),
        ]
    );
    assert!(!workspace.any_dirty());
    assert!(ProjectWorkspace::default().project_sources.is_empty());
}

#[test]
fn file_new_bootstrap_is_empty_but_keeps_a_valid_project_hierarchy() {
    let mut libraries = LibraryManager::default();
    let workspace = ProjectWorkspace::new_empty_bootstrapped(&mut libraries, "Afe", "afe", "core");

    assert!(workspace.project_sources.is_empty());
    assert!(!workspace.project_sources_dirty);
    assert!(
        libraries
            .get_library(&workspace.active_view.library)
            .and_then(|library| library.get_cell(&workspace.active_view.cell))
            .and_then(|cell| cell.get_view(&workspace.active_view.view))
            .is_some()
    );
}

#[test]
fn project_source_names_are_portable_and_extensions_are_case_insensitive() {
    assert!(
        ProjectSourceDocument::try_new(
            "MODEL.VA",
            ProjectSourceLanguage::VerilogA,
            "module model; endmodule",
        )
        .is_ok()
    );
    assert!(matches!(
        ProjectSourceDocument::try_new(
            "bad\"name.va",
            ProjectSourceLanguage::VerilogA,
            "module model; endmodule",
        ),
        Err(ProjectSourceError::InvalidFileNameCharacters { .. })
    ));
    assert!(matches!(
        ProjectSourceDocument::try_new(
            "COM1.va",
            ProjectSourceLanguage::VerilogA,
            "module model; endmodule",
        ),
        Err(ProjectSourceError::ReservedFileName { .. })
    ));
}

#[test]
fn project_source_payload_limit_is_enforced_before_compilation() {
    let oversized = "x".repeat(MAX_PROJECT_CODE_SOURCE_BYTES + 1);
    assert!(matches!(
        ProjectSourceDocument::try_new(
            "oversized.va",
            ProjectSourceLanguage::VerilogA,
            oversized,
        ),
        Err(ProjectSourceError::SourceTooLarge {
            bytes,
            limit: MAX_PROJECT_CODE_SOURCE_BYTES,
            ..
        }) if bytes == MAX_PROJECT_CODE_SOURCE_BYTES + 1
    ));
}

#[test]
fn source_edits_preserve_exact_utf8_and_invalidate_validation_identity() {
    let mut registry = ProjectSourceRegistry::try_from_documents([ProjectSourceDocument::try_new(
        "sensor_bridge.va",
        ProjectSourceLanguage::VerilogA,
        "module sensor_bridge; endmodule\r\n",
    )
    .unwrap()])
    .unwrap();
    let first_identity = registry
        .mark_validated(ProjectSourceLanguage::VerilogA)
        .unwrap();
    assert!(
        registry
            .get(ProjectSourceLanguage::VerilogA)
            .unwrap()
            .validation_is_current()
    );

    let source = "module sensor_bridge; // Δ温度\nendmodule\n".to_owned();
    assert!(
        registry
            .replace_content(ProjectSourceLanguage::VerilogA, source.clone())
            .unwrap()
    );
    let edited = registry.get(ProjectSourceLanguage::VerilogA).unwrap();
    assert_eq!(edited.content(), source);
    assert_eq!(edited.revision().get(), 2);
    assert!(edited.validated_identity().is_none());
    assert_ne!(edited.content_digest(), first_identity.content_digest());
    let edited_revision = edited.revision();
    assert!(
        !registry
            .replace_content(ProjectSourceLanguage::VerilogA, source)
            .unwrap()
    );
    assert_eq!(
        registry
            .get(ProjectSourceLanguage::VerilogA)
            .unwrap()
            .revision(),
        edited_revision
    );
}

#[test]
fn imported_source_replacement_is_monotonic_validated_and_atomic() {
    let mut registry = ProjectSourceRegistry::try_from_documents([ProjectSourceDocument::try_new(
        "first.va",
        ProjectSourceLanguage::VerilogA,
        "module first; endmodule\n",
    )
    .unwrap()])
    .unwrap();
    registry
        .mark_validated(ProjectSourceLanguage::VerilogA)
        .unwrap();

    assert!(
        registry
            .replace_imported(
                ProjectSourceLanguage::VerilogA,
                "second.va".to_owned(),
                "module second; endmodule\r\n".to_owned(),
            )
            .unwrap()
    );
    let imported = registry.get(ProjectSourceLanguage::VerilogA).unwrap();
    assert_eq!(imported.file_name(), "second.va");
    assert_eq!(imported.content(), "module second; endmodule\r\n");
    assert_eq!(imported.revision().get(), 2);
    assert!(imported.validated_identity().is_none());

    let before = registry.clone();
    assert!(matches!(
        registry.replace_imported(
            ProjectSourceLanguage::VerilogA,
            "wrong.txt".to_owned(),
            "module wrong; endmodule\n".to_owned(),
        ),
        Err(ProjectSourceError::InvalidFileNameExtension { .. })
    ));
    assert_eq!(registry, before);
}

#[test]
fn workspace_source_dirty_state_tracks_edits_validation_and_cleaning() {
    let mut libraries = LibraryManager::default();
    let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);

    workspace
        .replace_project_source(
            ProjectSourceLanguage::RSpiceAutomation,
            "plan = project.plan(\"Unicode Δ\")".to_owned(),
        )
        .unwrap();
    assert!(workspace.project_sources_dirty);
    assert!(workspace.any_dirty());
    workspace.mark_project_sources_clean();
    assert!(!workspace.any_dirty());

    let identity = workspace
        .mark_project_source_validated(ProjectSourceLanguage::RSpiceAutomation)
        .unwrap();
    assert!(workspace.project_sources_dirty);
    assert_eq!(
        workspace
            .project_sources
            .get(ProjectSourceLanguage::RSpiceAutomation)
            .unwrap()
            .validated_identity(),
        Some(identity)
    );
    workspace.mark_all_clean();
    assert!(!workspace.any_dirty());

    let repeated = workspace
        .mark_project_source_validated(ProjectSourceLanguage::RSpiceAutomation)
        .unwrap();
    assert_eq!(repeated, identity);
    assert!(!workspace.any_dirty());
}

#[test]
fn project_source_validation_rejects_mismatched_slots_and_stale_evidence() {
    let document = ProjectSourceDocument::try_new(
        "sensor_bridge.va",
        ProjectSourceLanguage::VerilogA,
        "module sensor_bridge; endmodule",
    )
    .unwrap();
    let mut registry = ProjectSourceRegistry::try_from_documents([document]).unwrap();
    registry
        .mark_validated(ProjectSourceLanguage::VerilogA)
        .unwrap();
    let mut value = serde_json::to_value(&registry).unwrap();
    value["bundles"][0]["root"]["content"] = serde_json::Value::String("changed".to_owned());
    assert!(serde_json::from_value::<ProjectSourceRegistry>(value).is_err());

    let root = serde_json::to_value(
        registry
            .get(ProjectSourceLanguage::VerilogA)
            .expect("fixture root exists"),
    )
    .unwrap();
    let mut legacy = serde_json::json!({ "verilog_a": root });
    legacy["verilog_a"]["language"] = serde_json::Value::String("rspice-automation".to_owned());
    assert!(serde_json::from_value::<ProjectSourceRegistry>(legacy).is_err());
}

#[test]
fn project_library_mutation_receipts_are_exact_hash_chained_and_tamper_evident() {
    let mut project = ProjectDescriptor::default();
    let first = project
        .prepare_library_mutation(
            ProjectLibraryMutation::CreateCell {
                library: "work".to_owned(),
                cell: "amp".to_owned(),
            },
            7,
        )
        .expect("prepare cell creation");
    assert_eq!(first.minimum_library_revision(), 8);
    let first = project
        .publish_library_mutation(first, 8)
        .expect("publish cell creation");

    let second = project
        .prepare_library_mutation(
            ProjectLibraryMutation::CreateView {
                library: "work".to_owned(),
                cell: "amp".to_owned(),
                view: "symbol".to_owned(),
            },
            8,
        )
        .expect("prepare view creation");
    let second = project
        .publish_library_mutation(second, 10)
        .expect("publish view creation");

    assert_eq!(project.revision().get(), 3);
    assert_eq!(project.library_mutation_audit().len(), 2);
    assert_eq!(first.sequence(), 1);
    assert_eq!(second.sequence(), 2);
    assert_eq!(first.source_project_revision().get(), 1);
    assert_eq!(first.target_project_revision().get(), 2);
    assert_eq!(second.source_project_revision().get(), 2);
    assert_eq!(second.target_project_revision().get(), 3);
    assert_eq!(first.source_library_revision(), 7);
    assert_eq!(first.target_library_revision(), 8);
    assert_eq!(second.source_library_revision(), 8);
    assert_eq!(second.target_library_revision(), 10);
    assert_eq!(
        second.previous_receipt_digest(),
        Some(first.receipt_digest())
    );
    project.validate().expect("valid receipt chain");

    let json = serde_json::to_string(&project).expect("serialize project descriptor");
    let restored: ProjectDescriptor =
        serde_json::from_str(&json).expect("restore project descriptor");
    restored
        .validate()
        .expect("restored receipt chain is valid");

    let mut tampered: serde_json::Value =
        serde_json::from_str(&json).expect("parse project descriptor");
    tampered["library_mutation_audit"][0]["mutation"]["cell"] =
        serde_json::Value::String("tampered".to_owned());
    let tampered: ProjectDescriptor =
        serde_json::from_value(tampered).expect("receipt schema remains structurally valid");
    assert!(matches!(
        tampered.validate(),
        Err(ProjectDescriptorError::LibraryAuditCorrupted(_))
    ));
}

#[test]
fn library_mutation_preflight_rejects_noops_and_revision_exhaustion_atomically() {
    let project = ProjectDescriptor::default();
    let revision = project.revision();

    assert!(matches!(
        project.prepare_library_mutation(
            ProjectLibraryMutation::RenameCell {
                library: "work".to_owned(),
                from_cell: "amp".to_owned(),
                to_cell: "AMP".to_owned(),
            },
            4,
        ),
        Err(ProjectDescriptorError::LibraryAuditCorrupted(_))
    ));
    assert!(matches!(
        project.prepare_library_mutation(
            ProjectLibraryMutation::DeleteCell {
                library: "work".to_owned(),
                cell: "amp".to_owned(),
            },
            u64::MAX,
        ),
        Err(ProjectDescriptorError::LibraryRevisionExhausted)
    ));
    assert_eq!(project.revision(), revision);
    assert!(project.library_mutation_audit().is_empty());
}

#[test]
fn library_manager_revision_survives_serialization_for_audit_continuity() {
    let mut libraries = LibraryManager::new();
    libraries.add_library(Library::new("work"));
    let revision = libraries.revision();
    assert!(revision > 0);

    let json = serde_json::to_string(&libraries).expect("serialize library manager");
    let restored: LibraryManager = serde_json::from_str(&json).expect("restore library manager");
    assert_eq!(restored.revision(), revision);
}

/// A plan written before the point scope existed judged every retained point,
/// so it has to keep doing exactly that after it is reloaded. Anything else
/// would narrow a requirement nobody narrowed.
#[test]
fn a_specification_written_before_the_point_scope_loads_as_every_point() {
    let legacy = r#"{
        "measurement": "gain",
        "expression": "meas ac gain max V(out)",
        "min": 10.0,
        "max": null,
        "unit": "dB"
    }"#;

    let spec: SpecEntry = serde_json::from_str(legacy).expect("legacy specification restores");

    assert_eq!(spec.scope, SpecPointScope::AllPoints);
    assert!(spec.validate().is_ok());
    // The default is also what an untouched specification writes back, so a
    // project reopened by an older build still reads the same requirement.
    let json = serde_json::to_string(&spec).expect("serialize specification");
    assert!(
        !json.contains("scope"),
        "an unscoped specification must not grow a field: {json}"
    );
}

#[test]
fn a_scoped_specification_round_trips_and_rejects_a_nonsense_scope() {
    let scoped = SpecEntry {
        measurement: "gain".to_owned(),
        expression: String::new(),
        min: Some(10.0),
        max: None,
        unit: "dB".to_owned(),
        scope: SpecPointScope::SelectedCorners {
            corners: vec!["SS".to_owned(), "FF".to_owned()],
        },
    };
    scoped.validate().expect("a named corner set is valid");

    let json = serde_json::to_string(&scoped).expect("serialize scoped specification");
    let restored: SpecEntry = serde_json::from_str(&json).expect("scoped specification restores");
    assert_eq!(restored, scoped);

    let nominal = SpecEntry {
        scope: SpecPointScope::Nominal,
        ..scoped.clone()
    };
    let json = serde_json::to_string(&nominal).expect("serialize nominal specification");
    assert_eq!(
        serde_json::from_str::<SpecEntry>(&json).expect("nominal specification restores"),
        nominal
    );

    let empty = SpecEntry {
        scope: SpecPointScope::SelectedCorners {
            corners: Vec::new(),
        },
        ..scoped.clone()
    };
    assert!(
        empty
            .validate()
            .is_err_and(|error| error.contains("at least one")),
        "a corner scope that names nothing selects nothing and is not a requirement"
    );

    let repeated = SpecEntry {
        scope: SpecPointScope::SelectedCorners {
            corners: vec!["ss".to_owned(), "SS".to_owned()],
        },
        ..scoped
    };
    assert!(
        repeated
            .validate()
            .is_err_and(|error| error.contains("repeats corner")),
        "corners are matched case-insensitively, so a repeat is a repeat"
    );
}

/// The scope decides what counts as evidence, so the rule lives in one place
/// and every surface reads the same answer from it.
#[test]
fn a_narrowed_scope_admits_only_evidence_attributed_to_a_point() {
    let nominal = AnalysisResultPvtPoint::new("TT", Some(1.8), 27.0, None, true)
        .expect("valid nominal point");
    let hot_corner = AnalysisResultPvtPoint::new("SS", Some(1.62), 125.0, None, false)
        .expect("valid corner point");

    assert!(SpecPointScope::AllPoints.admits(None));
    assert!(SpecPointScope::AllPoints.admits(Some(&nominal)));

    assert!(SpecPointScope::Nominal.admits(Some(&nominal)));
    assert!(!SpecPointScope::Nominal.admits(Some(&hot_corner)));
    assert!(
        !SpecPointScope::Nominal.admits(None),
        "an unattributed result is not proof that the nominal point passed"
    );

    let corners = SpecPointScope::SelectedCorners {
        corners: vec!["ss".to_owned()],
    };
    assert!(corners.admits(Some(&hot_corner)));
    assert!(!corners.admits(Some(&nominal)));
    assert!(!corners.admits(None));
}

#[test]
fn legacy_saved_output_migrates_to_plan_owned_plot_intent() {
    let output = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "V(out)",
        "V(out)",
        SavedOutputCompatibility::AllCompatibleAnalyses,
        SavedOutputPolicy::SelectedAndFinalPoints,
        SavedOutputPrecision::DisplayCacheWithFullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .expect("saved output");
    let expected_id = output.id;
    let mut wire = serde_json::to_value(output).expect("serialize");
    let object = wire.as_object_mut().expect("object");
    object.remove("origin");
    object.remove("display_intent");

    let restored: SavedOutput = serde_json::from_value(wire).expect("legacy saved output");
    assert_eq!(restored.id, expected_id);
    assert_eq!(restored.origin, SavedOutputOrigin::Plan);
    assert_eq!(restored.display_intent, SavedOutputDisplayIntent::Plot);
}
