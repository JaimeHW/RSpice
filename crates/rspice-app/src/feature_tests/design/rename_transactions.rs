//! Rename transactions committed through a whole application: one undo entry per
//! rename, refusals that leave the design and its history alone.

#![cfg(not(target_arch = "wasm32"))]

use crate::product::{ObjectRevision, SavedOutputId, SimulationPlanId};
use crate::state::{
    CellViewRef, ComponentType, ConfigurationSetId, Point, SchematicSnapshot, SchematicState,
};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::selection::rename::transaction_tests::fixture;
use crate::workbench::commands::vocabulary::Command;

fn assert_reference(
    app: &RSpiceApp,
    configuration: ConfigurationSetId,
    plan: SimulationPlanId,
    name: &str,
) {
    assert_eq!(
        app.state.model.schematic.document().components[0].name,
        name
    );
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .configuration_sets
            .find(configuration)
            .unwrap()
            .dut_path(),
        format!("/{name}")
    );
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .plan_data(plan)
            .unwrap()
            .saved_outputs[0]
            .source_expression,
        format!("I({name})")
    );
    assert_eq!(
        app.state.model.schematic.document().probes[0]
            .source_expression
            .as_deref(),
        Some(format!("I({name})").as_str())
    );
}

#[test]
fn rename_undo_redo_carries_live_bindings_and_advances_revisions() {
    let (mut app, expected, configuration, plan) = fixture();
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(expected.id);
    let output_id = app
        .state
        .model
        .workspace
        .content
        .plan_data(plan)
        .unwrap()
        .saved_outputs[0]
        .id;
    assert!(
        app.state
            .model
            .rename_component_transaction(&expected, "V9".to_owned())
            .unwrap()
    );
    assert_reference(&app, configuration, plan, "V9");
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .components
            .contains(&expected.id)
    );
    app.execute_command_key(Command::Undo.key());
    assert_reference(&app, configuration, plan, "V1");
    assert!(!app.state.model.schematic.can_undo());
    assert!(app.state.model.project_undo_sequence().is_none());
    app.execute_command_key(Command::Redo.key());
    assert_reference(&app, configuration, plan, "V9");
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .configuration_sets
            .find(configuration)
            .unwrap()
            .revision(),
        4
    );
    let output = &app
        .state
        .model
        .workspace
        .content
        .plan_data(plan)
        .unwrap()
        .saved_outputs[0];
    assert_eq!(output.id, output_id);
    assert_eq!(output.revision, ObjectRevision::new(4).unwrap());
    assert_eq!(
        app.state.model.schematic.document().probes[0].saved_output_id,
        Some(output_id)
    );
}

#[test]
fn undo_preserves_unrelated_outputs_and_current_output_metadata() {
    let (mut app, expected, configuration, plan) = fixture();
    app.state
        .model
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
    let payload = app
        .state
        .model
        .workspace
        .content
        .plan_data_mut(plan)
        .unwrap();
    payload.saved_outputs[0].name = "reviewed current".to_owned();
    payload.saved_outputs[0].revision = payload.saved_outputs[0].revision.next().unwrap();
    let mut unrelated = payload.saved_outputs[0].clone();
    unrelated.id = SavedOutputId::new();
    unrelated.name = "unrelated".to_owned();
    unrelated.source_expression = "V(out)".to_owned();
    payload.saved_outputs.push(unrelated.clone());
    app.execute_command_key(Command::Undo.key());
    assert_reference(&app, configuration, plan, "V1");
    let payload = app.state.model.workspace.content.plan_data(plan).unwrap();
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
        .model
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
    let sequence = app.state.model.project_undo_sequence();
    let revision = app
        .state
        .model
        .workspace
        .content
        .plan_data(plan)
        .unwrap()
        .saved_outputs[0]
        .revision;
    app.state
        .model
        .workspace
        .content
        .plan_data_mut(plan)
        .unwrap()
        .saved_outputs[0]
        .revision = ObjectRevision::new(u64::MAX).unwrap();
    assert!(app.state.undo_project_design().is_err());
    assert_reference(&app, configuration, plan, "V9");
    assert_eq!(sequence, app.state.model.project_undo_sequence());
    app.state
        .model
        .workspace
        .content
        .plan_data_mut(plan)
        .unwrap()
        .saved_outputs[0]
        .revision = revision;
    app.execute_command_key(Command::Undo.key());
    assert_reference(&app, configuration, plan, "V1");
}

#[test]
fn undo_does_not_cross_a_blocked_rename_into_older_local_history() {
    let (mut app, expected, _, plan) = fixture();
    app.state
        .model
        .schematic
        .with_undo("change value", |schematic| {
            schematic.document_mut_for_test().components[0].value = "2".to_owned();
        });
    let mut expected = expected;
    expected.value = "2".to_owned();
    app.state
        .model
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
    app.state
        .model
        .workspace
        .content
        .plan_data_mut(plan)
        .unwrap()
        .saved_outputs[0]
        .source_expression = "I(V_other)".to_owned();
    let snapshot = SchematicSnapshot::capture(app.state.model.schematic.document());
    let sequence = app.state.model.project_undo_sequence();
    app.execute_command_key(Command::Undo.key());
    assert!(snapshot.is_equal_document(app.state.model.schematic.document()));
    assert_eq!(sequence, app.state.model.project_undo_sequence());
    assert!(
        app.state.model.schematic.can_undo(),
        "the older value edit was not consumed"
    );
}

#[test]
fn local_edit_and_project_rename_follow_one_global_history_order() {
    let (mut app, expected, configuration, plan) = fixture();
    app.state
        .model
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
    app.state
        .model
        .schematic
        .with_undo("change value", |schematic| {
            schematic.document_mut_for_test().components[0].value = "2".to_owned();
        });
    app.execute_command_key(Command::Undo.key());
    assert_reference(&app, configuration, plan, "V9");
    assert_eq!(
        app.state.model.schematic.document().components[0].value,
        expected.value
    );
    app.execute_command_key(Command::Undo.key());
    assert_reference(&app, configuration, plan, "V1");
    app.execute_command_key(Command::Redo.key());
    assert_reference(&app, configuration, plan, "V9");
    app.execute_command_key(Command::Redo.key());
    assert_eq!(
        app.state.model.schematic.document().components[0].value,
        "2"
    );
    assert_reference(&app, configuration, plan, "V9");
}

#[test]
fn rename_starts_a_new_history_branch_without_stale_local_redo() {
    let (mut app, expected, configuration, plan) = fixture();
    app.state
        .model
        .schematic
        .with_undo("change value", |schematic| {
            schematic.document_mut_for_test().components[0].value = "2".to_owned();
        });
    app.execute_command_key(Command::Undo.key());
    assert!(app.state.model.schematic.can_redo());
    app.state
        .model
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
    assert!(!app.state.model.schematic.can_redo());
    app.execute_command_key(Command::Redo.key());
    assert_reference(&app, configuration, plan, "V9");
    assert_eq!(
        app.state.model.schematic.document().components[0].value,
        expected.value
    );
}

#[test]
fn switching_documents_does_not_redirect_rename_history() {
    let (mut app, expected, configuration, plan) = fixture();
    let owner = app
        .state
        .model
        .workspace
        .content
        .active_schematic_reference();
    app.state
        .model
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
    let other = CellViewRef::new("work", "other", "schematic");
    app.state
        .model
        .workspace
        .activate_view(other.clone(), crate::state::ViewType::Schematic);
    app.state.model.schematic = SchematicState::default();
    app.state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(50, 50));
    let other_snapshot = SchematicSnapshot::capture(app.state.model.schematic.document());
    app.execute_command_key(Command::Undo.key());
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .active_schematic_reference(),
        owner
    );
    assert_reference(&app, configuration, plan, "V1");
    assert!(other_snapshot.is_equal_document(
        app.state.model.workspace.content.schematic_buffers[&other.key()].document()
    ));
    app.execute_command_key(Command::Redo.key());
    assert_reference(&app, configuration, plan, "V9");
    assert!(other_snapshot.is_equal_document(
        app.state.model.workspace.content.schematic_buffers[&other.key()].document()
    ));
}

#[test]
fn active_and_inactive_plan_references_survive_native_save_and_reopen() {
    use crate::workbench::lifecycle::project_lifecycle::{self, DestinationAuthority, SaveScope};

    let (mut app, expected, configuration, first_plan) = fixture();
    // This test's voltage source is a primitive, not a hierarchical DUT.
    let mut definition = app
        .state
        .model
        .workspace
        .content
        .configuration_sets
        .find(configuration)
        .unwrap()
        .definition()
        .clone();
    definition.dut_path = "/".to_owned();
    app.state
        .model
        .workspace
        .content
        .configuration_sets
        .update(configuration, 1, definition)
        .unwrap();
    let mut source = app
        .state
        .model
        .workspace
        .content
        .plan_data(first_plan)
        .unwrap()
        .saved_outputs[0]
        .clone();
    source.id = SavedOutputId::new();
    let second_plan = app
        .state
        .model
        .sim_setup
        .create_plan("Second plan")
        .unwrap();
    app.state
        .model
        .workspace
        .content
        .add_saved_output(second_plan, source)
        .unwrap();
    app.state
        .model
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
    app.state.model.sim_setup.activate_plan(first_plan).unwrap();
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
        let schematic = &loaded.file.workspace.schematic_buffers
            [&loaded.file.workspace.active_schematic_reference().key()];
        assert_eq!(schematic.document().components[0].name, name);
        assert_eq!(
            schematic.document().probes[0].source_expression.as_deref(),
            Some(format!("I({name})").as_str())
        );
        for plan in [first_plan, second_plan] {
            assert_eq!(
                loaded.file.workspace.plan_data(plan).unwrap().saved_outputs[0].source_expression,
                format!("I({name})")
            );
        }
        if name == "V9" {
            app.execute_command_key(Command::Undo.key());
        } else {
            app.execute_command_key(Command::Redo.key());
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
        .model
        .workspace
        .content
        .configuration_sets
        .find(configuration)
        .unwrap()
        .definition()
        .clone();
    unrelated.name = "Another root".to_owned();
    unrelated.root.cell = "other".to_owned();
    let id = app
        .state
        .model
        .workspace
        .content
        .configuration_sets
        .create(unrelated)
        .unwrap();
    let original = app
        .state
        .model
        .workspace
        .content
        .configuration_sets
        .find(id)
        .unwrap()
        .clone();
    app.state
        .model
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
    assert_reference(&app, configuration, plan, "V9");
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .configuration_sets
            .find(id),
        Some(&original)
    );
    app.execute_command_key(Command::Undo.key());
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .configuration_sets
            .find(id),
        Some(&original)
    );
}

#[test]
fn imported_primitive_current_probes_follow_the_emitted_card_identity() {
    let (mut app, mut expected, _, plan) = fixture();
    expected.name = "bias".to_owned();
    app.state.model.schematic.document_mut_for_test().components[0] = expected.clone();
    app.state
        .model
        .workspace
        .content
        .plan_data_mut(plan)
        .unwrap()
        .saved_outputs[0]
        .source_expression = "I(Vbias)".to_owned();
    app.state.model.schematic.document_mut_for_test().probes[0].source_expression =
        Some("I(Vbias)".to_owned());
    app.state.model.schematic.document_mut_for_test().probes[0].reference = "I(Vbias)".to_owned();
    app.state
        .model
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .plan_data(plan)
            .unwrap()
            .saved_outputs[0]
            .source_expression,
        "I(V9)"
    );
    assert_eq!(
        app.state.model.schematic.document().probes[0].reference,
        "I(V9)"
    );
    app.execute_command_key(Command::Undo.key());
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .plan_data(plan)
            .unwrap()
            .saved_outputs[0]
            .source_expression,
        "I(Vbias)"
    );
    assert_eq!(
        app.state.model.schematic.document().components[0].name,
        "bias"
    );
}
