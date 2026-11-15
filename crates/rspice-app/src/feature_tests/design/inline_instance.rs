//! The inline instance editor drawn through a whole application: the fields it
//! offers and what committing them does.

use crate::feature_tests::design::inline_instance_drawn::{Editor, key};
use crate::workbench::commands::vocabulary::ShellCommand;
use rspice_app_state::state::CellViewRef;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;
use rspice_app_workbench::workbench::state::console_views::InlineEditField;

#[test]
fn switching_fields_in_either_render_order_commits_each_edit_once() {
    for order in [
        [("Instance", "V9"), ("Value", "12")],
        [("Value", "12"), ("Instance", "V9")],
    ] {
        let mut editor = Editor::new();
        let before = editor.app.state.model.schematic.document().components[0].clone();
        editor.edit(order[0].0, order[0].1);
        editor.edit(order[1].0, order[1].1);
        editor.pass(vec![key(egui::Key::Enter, egui::Modifiers::NONE)]);
        assert_eq!(
            editor.app.state.model.schematic.document().components[0].name,
            "V9"
        );
        assert_eq!(
            editor.app.state.model.schematic.document().components[0].value,
            "12"
        );
        editor.app.execute_command_key(ids::UNDO);
        if order[1].0 == "Value" {
            assert_eq!(
                editor.app.state.model.schematic.document().components[0].name,
                "V9"
            );
            assert_eq!(
                editor.app.state.model.schematic.document().components[0].value,
                before.value
            );
        } else {
            assert_eq!(
                editor.app.state.model.schematic.document().components[0].name,
                "V1"
            );
            assert_eq!(
                editor.app.state.model.schematic.document().components[0].value,
                "12"
            );
        }
        editor.app.execute_command_key(ids::UNDO);
        assert_eq!(
            editor.app.state.model.schematic.document().components[0],
            before
        );
        assert!(!editor.app.state.model.schematic.can_undo());
        assert!(editor.app.state.model.project_undo_sequence().is_none());
        editor.app.execute_command_key(ids::REDO);
        editor.app.execute_command_key(ids::REDO);
        assert_eq!(
            editor.app.state.model.schematic.document().components[0].name,
            "V9"
        );
        assert_eq!(
            editor.app.state.model.schematic.document().components[0].value,
            "12"
        );
    }
}

#[test]
fn workspace_and_run_commands_resolve_the_pending_field_first() {
    let mut editor = Editor::new();
    editor.edit("Instance", "invalid name");
    editor
        .app
        .execute_command_key(ShellCommand::OpenWorkspace(Workspace::Simulate).key());
    assert_eq!(editor.app.state.workbench.workspace, Workspace::Design);
    editor.app.execute_command_key(
        rspice_app_workbench::workbench::framework::command_registry::ids::RUN_SIMULATION,
    );
    assert!(!editor.app.state.runs.preflight.take_run_and_queue_request());
    assert_eq!(
        editor.app.state.model.schematic.document().components[0].name,
        "V1"
    );
    assert!(editor.app.state.workbench.inline_edit.session().is_some());
    editor.replace("V9");
    editor
        .app
        .execute_command_key(ShellCommand::OpenWorkspace(Workspace::Simulate).key());
    assert_eq!(editor.app.state.workbench.workspace, Workspace::Simulate);
    assert_eq!(
        editor.app.state.model.schematic.document().components[0].name,
        "V9"
    );
    assert!(editor.app.state.workbench.inline_edit.session().is_none());
}

#[test]
fn opening_another_document_commits_to_the_original_component_and_preserves_refusals() {
    for valid in [false, true] {
        let mut editor = Editor::new();
        let original = editor
            .app
            .state
            .model
            .workspace
            .content
            .active_schematic_reference();
        let other = CellViewRef::new(&original.library, "other", "schematic");
        let mut other_cell = rspice_app_state::state::Cell::new("other");
        other_cell.add_view(rspice_app_state::state::View::new(
            "schematic",
            rspice_app_state::state::ViewType::Schematic,
        ));
        editor
            .app
            .state
            .model
            .library_manager
            .edit_library(&original.library)
            .expect("the project library")
            .add_cell(other_cell);
        let mut other_schematic = rspice_app_state::state::SchematicState::default();
        other_schematic
            .document_mut_for_test()
            .components
            .push(editor.app.state.model.schematic.document().components[0].clone());
        editor
            .app
            .state
            .model
            .workspace
            .insert_schematic_editor(other.key(), other_schematic);
        editor.edit("Instance", if valid { "V9" } else { "invalid name" });
        editor.app.state.open_workspace_view(other.clone());
        if valid {
            assert_eq!(
                editor
                    .app
                    .state
                    .model
                    .workspace
                    .content
                    .active_schematic_reference(),
                other
            );
            assert_eq!(
                editor.app.state.model.schematic.document().components[0].name,
                "V1"
            );
            assert_eq!(
                editor.app.state.model.workspace.content.schematic_buffers[&original.key()]
                    .document()
                    .components[0]
                    .name,
                "V9"
            );
            editor.app.execute_command_key(ids::UNDO);
            assert_eq!(
                editor
                    .app
                    .state
                    .model
                    .workspace
                    .content
                    .active_schematic_reference(),
                original
            );
            assert_eq!(
                editor.app.state.model.schematic.document().components[0].name,
                "V1"
            );
        } else {
            assert_eq!(
                editor
                    .app
                    .state
                    .model
                    .workspace
                    .content
                    .active_schematic_reference(),
                original
            );
            assert_eq!(
                editor.app.state.model.schematic.document().components[0].name,
                "V1"
            );
            assert_eq!(
                editor
                    .app
                    .state
                    .workbench
                    .inline_edit
                    .session()
                    .unwrap()
                    .buffer,
                "invalid name"
            );
        }
    }
}

#[test]
fn rendered_instance_name_is_a_draft_until_one_reference_transaction_commits() {
    let mut editor = Editor::new();
    let plan = editor
        .app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .unwrap()
        .id();
    let output = rspice_app_state::state::SavedOutput::new(
        rspice_app_state::state::SavedOutputKind::RawVoltageOrCurrent,
        "current",
        "I(V1)",
        rspice_app_state::state::SavedOutputCompatibility::OpTranAc,
        rspice_app_state::state::SavedOutputPolicy::EveryAcceptedPoint,
        rspice_app_state::state::SavedOutputPrecision::FullSourcePrecision,
        rspice_app_state::state::SavedOutputStreaming::StoreOnly,
    )
    .unwrap();
    editor
        .app
        .state
        .model
        .workspace
        .content
        .add_saved_output(plan, output)
        .unwrap();
    editor.pass(Vec::new());
    editor.click("Instance");
    editor.replace("V9");
    assert_eq!(
        editor.app.state.model.schematic.document().components[0].name,
        "V1",
        "typing must not publish a partial rename"
    );
    assert_eq!(
        editor
            .app
            .state
            .workbench
            .inline_edit
            .buffer_for(editor.id, &InlineEditField::Instance),
        Some("V9")
    );
    assert!(!editor.app.state.model.schematic.can_undo());
    editor.pass(vec![key(egui::Key::Enter, egui::Modifiers::NONE)]);
    assert_eq!(
        editor.app.state.model.schematic.document().components[0].name,
        "V9"
    );
    assert_eq!(
        editor
            .app
            .state
            .model
            .workspace
            .content
            .plan_data(plan)
            .unwrap()
            .saved_outputs[0]
            .source_expression,
        "I(V9)"
    );
    editor.app.execute_command_key(ids::UNDO);
    assert_eq!(
        editor.app.state.model.schematic.document().components[0].name,
        "V1"
    );
    assert_eq!(
        editor
            .app
            .state
            .model
            .workspace
            .content
            .plan_data(plan)
            .unwrap()
            .saved_outputs[0]
            .source_expression,
        "I(V1)"
    );
    assert!(editor.app.state.model.project_undo_sequence().is_none());
    assert!(!editor.app.state.model.schematic.can_undo());
    editor.app.execute_command_key(ids::REDO);
    assert_eq!(
        editor.app.state.model.schematic.document().components[0].name,
        "V9"
    );
    assert_eq!(
        editor
            .app
            .state
            .model
            .workspace
            .content
            .plan_data(plan)
            .unwrap()
            .saved_outputs[0]
            .source_expression,
        "I(V9)"
    );
}
