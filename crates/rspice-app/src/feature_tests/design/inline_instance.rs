//! The inline instance editor drawn through a whole application: the fields it
//! offers and what committing them does.

use crate::state::CellViewRef;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design::inspector::inline_instance::tests::{Editor, key};
use crate::workbench::state::{InlineEditField, Workspace};

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
        editor.app.execute_command_key(Command::Undo.key());
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
        editor.app.execute_command_key(Command::Undo.key());
        assert_eq!(
            editor.app.state.model.schematic.document().components[0],
            before
        );
        assert!(!editor.app.state.model.schematic.can_undo());
        assert!(editor.app.state.model.project_undo_sequence().is_none());
        editor.app.execute_command_key(Command::Redo.key());
        editor.app.execute_command_key(Command::Redo.key());
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
    Command::OpenWorkspace(Workspace::Simulate).execute(&mut editor.app);
    assert_eq!(editor.app.state.workbench.workspace, Workspace::Design);
    editor
        .app
        .execute_command_key(crate::workbench::framework::command_registry::ids::RUN_SIMULATION);
    assert!(!editor.app.state.runs.preflight.take_run_and_queue_request());
    assert_eq!(
        editor.app.state.model.schematic.document().components[0].name,
        "V1"
    );
    assert!(editor.app.state.workbench.inline_edit.session().is_some());
    editor.replace("V9");
    Command::OpenWorkspace(Workspace::Simulate).execute(&mut editor.app);
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
        let mut other_schematic = crate::state::SchematicState::default();
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
            editor.app.execute_command_key(Command::Undo.key());
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
    let output = crate::state::SavedOutput::new(
        crate::state::SavedOutputKind::RawVoltageOrCurrent,
        "current",
        "I(V1)",
        crate::state::SavedOutputCompatibility::OpTranAc,
        crate::state::SavedOutputPolicy::EveryAcceptedPoint,
        crate::state::SavedOutputPrecision::FullSourcePrecision,
        crate::state::SavedOutputStreaming::StoreOnly,
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
    editor.app.execute_command_key(Command::Undo.key());
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
    editor.app.execute_command_key(Command::Redo.key());
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
