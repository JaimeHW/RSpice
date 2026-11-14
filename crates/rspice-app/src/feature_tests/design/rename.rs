//! The rename dialog drawn and committed through a whole application.

use crate::workbench::app::dialogs::selection::rename::tests::{apply_commit, component_app};
use crate::workbench::app::dialogs::selection::rename::*;
use crate::workbench::commands::vocabulary::Command;

#[test]
fn successful_component_rename_retains_id_and_is_exactly_one_undo_step() {
    let (mut app, id) = component_app();
    let original_name = app.state.model.schematic.document().components[0]
        .name
        .clone();
    assert!(open_selected_object_rename(
        &mut app.state.design.rename_selection,
        app.state.design.object_properties.open || app.state.design.tabbed_property_dialog.open,
        &app.state.model
    ));
    let target = app.state.design.rename_selection.target.clone().unwrap();
    let commit = match validate_draft(&app.state.model.schematic, &target, "R_GAIN", &|_| true) {
        RenameValidation::Valid(commit) => commit,
        other => panic!("expected valid rename, got {other:?}"),
    };

    assert!(apply_commit(&mut app.state, *commit).unwrap());
    assert_eq!(app.state.model.schematic.document().components[0].id, id);
    assert_eq!(
        app.state.model.schematic.document().components[0].name,
        "R_GAIN"
    );
    app.execute_command_key(Command::Undo.key());
    assert_eq!(app.state.model.schematic.document().components[0].id, id);
    assert_eq!(
        app.state.model.schematic.document().components[0].name,
        original_name
    );
    assert!(
        app.state.model.project_undo_sequence().is_none() && !app.state.model.schematic.can_undo(),
        "rename created more than one undo step"
    );
    app.execute_command_key(Command::Redo.key());
    assert_eq!(
        app.state.model.schematic.document().components[0].name,
        "R_GAIN"
    );
}

fn saved_output(name: &str, expression: &str) -> crate::state::SavedOutput {
    crate::state::SavedOutput::new(
        crate::state::SavedOutputKind::RawVoltageOrCurrent,
        name,
        expression,
        crate::state::SavedOutputCompatibility::OpTranAc,
        crate::state::SavedOutputPolicy::EveryAcceptedPoint,
        crate::state::SavedOutputPrecision::FullSourcePrecision,
        crate::state::SavedOutputStreaming::StoreOnly,
    )
    .expect("a raw probe is a valid saved output")
}

#[test]
fn instance_rename_remaps_configuration_paths_and_saved_outputs() {
    let (mut app, id) = component_app();
    let configuration = app
        .state
        .model
        .workspace
        .content
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: "Release".to_owned(),
            root: app.state.model.workspace.content.active_view.clone(),
            dut_path: "/R1".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            cell_bindings: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Rename test".to_owned(),
        })
        .expect("valid configuration root");
    let plan_id = crate::product::SimulationPlanId::new();
    app.state
        .model
        .workspace
        .content
        .simulation_plan_payloads
        .push(crate::state::SimulationPlanPayloadRecord {
            plan_id,
            payload: crate::state::SimulationPlanPayload {
                saved_outputs: vec![
                    saved_output("device current", "I(R1)"),
                    saved_output("internal node", "V(/R1/mid)"),
                    saved_output("unrelated node", "V(out)"),
                ],
                ..Default::default()
            },
        });
    app.state.model.schematic.document_mut_for_test().components[0].name = "R1".to_owned();
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(id);
    assert!(open_selected_object_rename(
        &mut app.state.design.rename_selection,
        app.state.design.object_properties.open || app.state.design.tabbed_property_dialog.open,
        &app.state.model
    ));
    let target = app.state.design.rename_selection.target.clone().unwrap();
    let commit = match validate_draft(&app.state.model.schematic, &target, "RLOAD", &|_| true) {
        RenameValidation::Valid(commit) => commit,
        other => panic!("expected valid rename, got {other:?}"),
    };

    assert!(apply_commit(&mut app.state, *commit).unwrap());

    let expressions = app
        .state
        .model
        .workspace
        .content
        .simulation_plan_payloads
        .iter()
        .find(|record| record.plan_id == plan_id)
        .expect("the payload this test owns")
        .payload
        .saved_outputs
        .iter()
        .map(|output| output.source_expression.clone())
        .collect::<Vec<_>>();
    assert_eq!(
        expressions,
        vec!["I(RLOAD)", "V(/RLOAD/mid)", "V(out)"],
        "only the references that name the renamed instance move"
    );
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .configuration_sets
            .find(configuration)
            .expect("the configuration this test owns")
            .dut_path(),
        "/RLOAD",
        "the configuration DUT path follows the renamed instance"
    );
    assert!(app.state.model.workspace.content.project_metadata_dirty);
    app.execute_command_key(Command::Undo.key());
    assert_eq!(
        app.state.model.schematic.document().components[0].name,
        "R1"
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
        "/R1",
        "Undo must restore the configuration together with the schematic"
    );
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .plan_data(plan_id)
            .unwrap()
            .saved_outputs[0]
            .source_expression,
        "I(R1)",
        "Undo must restore the executable saved output"
    );
    app.execute_command_key(Command::Redo.key());
    assert_eq!(
        app.state.model.schematic.document().components[0].name,
        "RLOAD"
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
        "/RLOAD"
    );
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .plan_data(plan_id)
            .unwrap()
            .saved_outputs[0]
            .source_expression,
        "I(RLOAD)"
    );
}
