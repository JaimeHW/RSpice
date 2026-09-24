//! Native command history and persistence acceptance for component renames.
//! Assertions cross the schematic, configuration, plan, and durable probe owners.

use crate::product::{ObjectRevision, SavedOutputId, SimulationPlanId};
use crate::state::{
    CellViewRef, Component, ComponentType, ConfigurationBlackBoxPolicy, ConfigurationModelProfile,
    ConfigurationSetDefinition, ConfigurationSetId, Point, SavedOutput, SavedOutputCompatibility,
    SavedOutputKind, SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming, SchematicProbe,
    SchematicSnapshot, SchematicState, UnresolvedBindingPolicy,
};
use crate::workbench::app::RSpiceApp;

fn fixture() -> (RSpiceApp, Component, ConfigurationSetId, SimulationPlanId) {
    let mut app = RSpiceApp::test_instance();
    app.state
        .schematic
        .add_component(ComponentType::VoltageSource, Point::origin());
    let expected = app.state.schematic.components[0].clone();
    let source = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "current",
        "I(V1)",
        SavedOutputCompatibility::OpTranAc,
        SavedOutputPolicy::EveryAcceptedPoint,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .unwrap();
    let plan = app.state.sim_setup.stable_analysis_plan().unwrap().id();
    app.state
        .workspace
        .add_saved_output(plan, source.clone())
        .unwrap();
    let mut probe =
        SchematicProbe::new(100, Point::new(80, 20), "I(V1)", Some("I(V1)".to_owned())).unwrap();
    probe.bind_saved_output(plan, source.id);
    app.state.schematic.probes.push(probe);
    let id = app
        .state
        .workspace
        .configuration_sets
        .create(ConfigurationSetDefinition {
            name: "Rename test".to_owned(),
            root: app.state.workspace.active_schematic_reference(),
            dut_path: "/V1".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            overrides: Vec::new(),
            owner: "test".to_owned(),
            unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            model_profile: ConfigurationModelProfile::ProjectRunSetSections,
        })
        .unwrap();
    app.state.schematic.is_dirty = false;
    app.state.workspace.project_metadata_dirty = false;
    (app, expected, id, plan)
}

fn assert_reference(
    app: &RSpiceApp,
    configuration: ConfigurationSetId,
    plan: SimulationPlanId,
    name: &str,
) {
    assert_eq!(app.state.schematic.components[0].name, name);
    assert_eq!(
        app.state
            .workspace
            .configuration_sets
            .find(configuration)
            .unwrap()
            .dut_path(),
        format!("/{name}")
    );
    assert_eq!(
        app.state.workspace.plan_data(plan).unwrap().saved_outputs[0].source_expression,
        format!("I({name})")
    );
    assert_eq!(
        app.state.schematic.probes[0].source_expression.as_deref(),
        Some(format!("I({name})").as_str())
    );
}

#[test]
fn rename_undo_redo_carries_live_bindings_and_advances_revisions() {
    let (mut app, expected, configuration, plan) = fixture();
    app.state
        .schematic
        .selection
        .select_only_component(expected.id);
    let output_id = app.state.workspace.plan_data(plan).unwrap().saved_outputs[0].id;
    assert!(
        app.state
            .rename_component_transaction(&expected, "V9".to_owned())
            .unwrap()
    );
    assert_reference(&app, configuration, plan, "V9");
    assert!(
        app.state
            .schematic
            .selection
            .components
            .contains(&expected.id)
    );
    app.action_edit_undo();
    assert_reference(&app, configuration, plan, "V1");
    assert!(!app.state.schematic.can_undo());
    assert!(app.state.project_undo_sequence().is_none());
    app.action_edit_redo();
    assert_reference(&app, configuration, plan, "V9");
    assert_eq!(
        app.state
            .workspace
            .configuration_sets
            .find(configuration)
            .unwrap()
            .revision(),
        4
    );
    let output = &app.state.workspace.plan_data(plan).unwrap().saved_outputs[0];
    assert_eq!(output.id, output_id);
    assert_eq!(output.revision, ObjectRevision::new(4).unwrap());
    assert_eq!(
        app.state.schematic.probes[0].saved_output_id,
        Some(output_id)
    );
}

#[test]
fn preparation_failure_never_publishes_any_owner_or_history() {
    for refusal in [
        "configuration revision",
        "output revision",
        "read only",
        "stale target",
        "duplicate",
    ] {
        let (mut app, expected, configuration, plan) = fixture();
        match refusal {
            "configuration revision" => {
                let mut json =
                    serde_json::to_value(&app.state.workspace.configuration_sets).unwrap();
                json["configurations"][0]["revision"] = serde_json::json!(u64::MAX);
                app.state.workspace.configuration_sets = serde_json::from_value(json).unwrap();
            }
            "output revision" => {
                app.state
                    .workspace
                    .plan_data_mut(plan)
                    .unwrap()
                    .saved_outputs[0]
                    .revision = ObjectRevision::new(u64::MAX).unwrap()
            }
            "read only" => app.state.schematic.read_only = true,
            "stale target" => app.state.schematic.components[0].value = "8".to_owned(),
            "duplicate" => {
                let id = app
                    .state
                    .schematic
                    .add_component(ComponentType::VoltageSource, Point::new(100, 0));
                app.state
                    .schematic
                    .components
                    .iter_mut()
                    .find(|component| component.id == id)
                    .unwrap()
                    .name = "V9".to_owned();
            }
            _ => unreachable!(),
        }
        let before = SchematicSnapshot::capture(&app.state.schematic);
        let catalog = app.state.workspace.configuration_sets.clone();
        let payloads = app.state.workspace.simulation_plan_payloads.clone();
        let dirty = app.state.schematic.is_dirty;
        let epoch = app.state.design_execution_epoch;
        let mut candidate = expected.clone();
        candidate.name = "V9".to_owned();
        candidate.value = "12".to_owned();
        candidate.params = "ac=1".to_owned();
        assert!(
            app.state
                .edit_component_transaction(&expected, candidate, "edit properties")
                .is_err(),
            "{refusal}"
        );
        assert!(before.is_equal_state(&app.state.schematic), "{refusal}");
        assert_eq!(catalog, app.state.workspace.configuration_sets);
        assert_eq!(payloads, app.state.workspace.simulation_plan_payloads);
        assert_eq!(dirty, app.state.schematic.is_dirty);
        assert!(!app.state.workspace.project_metadata_dirty);
        assert_eq!(epoch, app.state.design_execution_epoch);
        assert!(app.state.project_undo_sequence().is_none());
        assert_eq!(
            app.state
                .workspace
                .configuration_sets
                .find(configuration)
                .unwrap()
                .dut_path(),
            "/V1"
        );
    }
}

#[test]
fn undo_preserves_unrelated_outputs_and_current_output_metadata() {
    let (mut app, expected, configuration, plan) = fixture();
    app.state
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
    let payload = app.state.workspace.plan_data_mut(plan).unwrap();
    payload.saved_outputs[0].name = "reviewed current".to_owned();
    payload.saved_outputs[0].revision = payload.saved_outputs[0].revision.next().unwrap();
    let mut unrelated = payload.saved_outputs[0].clone();
    unrelated.id = SavedOutputId::new();
    unrelated.name = "unrelated".to_owned();
    unrelated.source_expression = "V(out)".to_owned();
    payload.saved_outputs.push(unrelated.clone());
    app.action_edit_undo();
    assert_reference(&app, configuration, plan, "V1");
    let payload = app.state.workspace.plan_data(plan).unwrap();
    assert_eq!(payload.saved_outputs[0].name, "reviewed current");
    assert_eq!(
        payload.saved_outputs[0].revision,
        ObjectRevision::new(4).unwrap()
    );
    assert_eq!(payload.saved_outputs[1], unrelated);
}

#[test]
fn history_refusal_keeps_the_transaction_available_for_retry() {
    let (mut app, expected, configuration, plan) = fixture();
    app.state
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
    let sequence = app.state.project_undo_sequence();
    let revision = app.state.workspace.plan_data(plan).unwrap().saved_outputs[0].revision;
    app.state
        .workspace
        .plan_data_mut(plan)
        .unwrap()
        .saved_outputs[0]
        .revision = ObjectRevision::new(u64::MAX).unwrap();
    assert!(app.state.undo_project_design().is_err());
    assert_reference(&app, configuration, plan, "V9");
    assert_eq!(sequence, app.state.project_undo_sequence());
    app.state
        .workspace
        .plan_data_mut(plan)
        .unwrap()
        .saved_outputs[0]
        .revision = revision;
    app.action_edit_undo();
    assert_reference(&app, configuration, plan, "V1");
}

#[test]
fn undo_does_not_cross_a_blocked_rename_into_older_local_history() {
    let (mut app, expected, _, plan) = fixture();
    app.state.schematic.with_undo("change value", |schematic| {
        schematic.components[0].value = "2".to_owned();
    });
    let expected = Component {
        value: "2".to_owned(),
        ..expected
    };
    app.state
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
    app.state
        .workspace
        .plan_data_mut(plan)
        .unwrap()
        .saved_outputs[0]
        .source_expression = "I(V_other)".to_owned();
    let snapshot = SchematicSnapshot::capture(&app.state.schematic);
    let sequence = app.state.project_undo_sequence();
    app.action_edit_undo();
    assert!(snapshot.is_equal_state(&app.state.schematic));
    assert_eq!(sequence, app.state.project_undo_sequence());
    assert!(
        app.state.schematic.can_undo(),
        "the older value edit was not consumed"
    );
}

#[test]
fn local_edit_and_project_rename_follow_one_global_history_order() {
    let (mut app, expected, configuration, plan) = fixture();
    app.state
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
    app.state.schematic.with_undo("change value", |schematic| {
        schematic.components[0].value = "2".to_owned();
    });
    app.action_edit_undo();
    assert_reference(&app, configuration, plan, "V9");
    assert_eq!(app.state.schematic.components[0].value, expected.value);
    app.action_edit_undo();
    assert_reference(&app, configuration, plan, "V1");
    app.action_edit_redo();
    assert_reference(&app, configuration, plan, "V9");
    app.action_edit_redo();
    assert_eq!(app.state.schematic.components[0].value, "2");
    assert_reference(&app, configuration, plan, "V9");
}

#[test]
fn rename_starts_a_new_history_branch_without_stale_local_redo() {
    let (mut app, expected, configuration, plan) = fixture();
    app.state.schematic.with_undo("change value", |schematic| {
        schematic.components[0].value = "2".to_owned();
    });
    app.action_edit_undo();
    assert!(app.state.schematic.can_redo());
    app.state
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
    assert!(!app.state.schematic.can_redo());
    app.action_edit_redo();
    assert_reference(&app, configuration, plan, "V9");
    assert_eq!(app.state.schematic.components[0].value, expected.value);
}

#[test]
fn switching_documents_does_not_redirect_rename_history() {
    let (mut app, expected, configuration, plan) = fixture();
    let owner = app.state.workspace.active_schematic_reference();
    app.state
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
    let other = CellViewRef::new("work", "other", "schematic");
    app.state
        .workspace
        .activate_view(other.clone(), crate::state::ViewType::Schematic);
    app.state.schematic = SchematicState::default();
    app.state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(50, 50));
    let other_snapshot = SchematicSnapshot::capture(&app.state.schematic);
    app.action_edit_undo();
    assert_eq!(app.state.workspace.active_schematic_reference(), owner);
    assert_reference(&app, configuration, plan, "V1");
    assert!(other_snapshot.is_equal_state(&app.state.workspace.schematic_buffers[&other.key()]));
    app.action_edit_redo();
    assert_reference(&app, configuration, plan, "V9");
    assert!(other_snapshot.is_equal_state(&app.state.workspace.schematic_buffers[&other.key()]));
}

#[test]
fn active_and_inactive_plan_references_survive_native_save_and_reopen() {
    use crate::workbench::lifecycle::project_lifecycle::{self, DestinationAuthority, SaveScope};

    let (mut app, expected, configuration, first_plan) = fixture();
    // This test's voltage source is a primitive, not a hierarchical DUT.
    let mut definition = app
        .state
        .workspace
        .configuration_sets
        .find(configuration)
        .unwrap()
        .definition()
        .clone();
    definition.dut_path = "/".to_owned();
    app.state
        .workspace
        .configuration_sets
        .update(configuration, 1, definition)
        .unwrap();
    let mut source = app
        .state
        .workspace
        .plan_data(first_plan)
        .unwrap()
        .saved_outputs[0]
        .clone();
    source.id = SavedOutputId::new();
    let second_plan = app.state.sim_setup.create_plan("Second plan").unwrap();
    app.state
        .workspace
        .add_saved_output(second_plan, source)
        .unwrap();
    app.state
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
    app.state.sim_setup.activate_plan(first_plan).unwrap();
    let path = std::env::temp_dir().join(format!(
        "rspice-component-rename-{}.rspiceproj",
        uuid::Uuid::new_v4()
    ));
    for name in ["V9", "V1", "V9"] {
        project_lifecycle::save_native(
            &mut app.state,
            SaveScope::AllDocuments,
            &path,
            if path.exists() {
                DestinationAuthority::Canonical
            } else {
                DestinationAuthority::UserSelected
            },
        )
        .unwrap();
        let loaded = crate::io::load_project_file(&path).unwrap();
        let schematic = &loaded.workspace.schematic_buffers
            [&loaded.workspace.active_schematic_reference().key()];
        assert_eq!(schematic.components[0].name, name);
        assert_eq!(
            schematic.probes[0].source_expression.as_deref(),
            Some(format!("I({name})").as_str())
        );
        for plan in [first_plan, second_plan] {
            assert_eq!(
                loaded.workspace.plan_data(plan).unwrap().saved_outputs[0].source_expression,
                format!("I({name})")
            );
        }
        if name == "V9" {
            app.action_edit_undo();
        } else {
            app.action_edit_redo();
        }
    }
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("rspiceproj.bak"));
    let mut lock = path.into_os_string();
    lock.push(".rspice.lock");
    let _ = std::fs::remove_file(std::path::PathBuf::from(lock));
}

#[test]
fn a_same_spelled_instance_in_another_configuration_root_is_unchanged() {
    let (mut app, expected, configuration, plan) = fixture();
    let mut unrelated = app
        .state
        .workspace
        .configuration_sets
        .find(configuration)
        .unwrap()
        .definition()
        .clone();
    unrelated.name = "Another root".to_owned();
    unrelated.root.cell = "other".to_owned();
    let id = app
        .state
        .workspace
        .configuration_sets
        .create(unrelated)
        .unwrap();
    let original = app
        .state
        .workspace
        .configuration_sets
        .find(id)
        .unwrap()
        .clone();
    app.state
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
    assert_reference(&app, configuration, plan, "V9");
    assert_eq!(
        app.state.workspace.configuration_sets.find(id),
        Some(&original)
    );
    app.action_edit_undo();
    assert_eq!(
        app.state.workspace.configuration_sets.find(id),
        Some(&original)
    );
}

#[test]
fn imported_primitive_current_probes_follow_the_emitted_card_identity() {
    let (mut app, mut expected, _, plan) = fixture();
    expected.name = "bias".to_owned();
    app.state.schematic.components[0] = expected.clone();
    app.state
        .workspace
        .plan_data_mut(plan)
        .unwrap()
        .saved_outputs[0]
        .source_expression = "I(Vbias)".to_owned();
    app.state.schematic.probes[0].source_expression = Some("I(Vbias)".to_owned());
    app.state.schematic.probes[0].reference = "I(Vbias)".to_owned();
    app.state
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
    assert_eq!(
        app.state.workspace.plan_data(plan).unwrap().saved_outputs[0].source_expression,
        "I(V9)"
    );
    assert_eq!(app.state.schematic.probes[0].reference, "I(V9)");
    app.action_edit_undo();
    assert_eq!(
        app.state.workspace.plan_data(plan).unwrap().saved_outputs[0].source_expression,
        "I(Vbias)"
    );
    assert_eq!(app.state.schematic.components[0].name, "bias");
}

#[test]
fn an_unchanged_name_preserves_authored_reference_spelling_and_history() {
    let (mut app, expected, _, _) = fixture();
    let id = app
        .state
        .schematic
        .add_component(ComponentType::Cccs, Point::new(100, 0));
    app.state
        .schematic
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .unwrap()
        .params = "vref=v1".to_owned();
    let before = SchematicSnapshot::capture(&app.state.schematic);
    let epoch = app.state.design_execution_epoch;
    assert!(
        !app.state
            .rename_component_transaction(&expected, expected.name.clone())
            .unwrap()
    );
    assert!(before.is_equal_state(&app.state.schematic));
    assert_eq!(app.state.design_execution_epoch, epoch);
    assert!(app.state.project_undo_sequence().is_none());
    assert!(!app.state.workspace.project_metadata_dirty);
}
