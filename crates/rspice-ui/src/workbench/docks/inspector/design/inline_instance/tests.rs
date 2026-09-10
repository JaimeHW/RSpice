//! Rendered inspector edits preserve drafts, reference ownership, and history.

use super::*;

struct Editor {
    ctx: egui::Context,
    app: RSpiceApp,
    id: u64,
    controls: HashMap<String, egui::Rect>,
}

impl Editor {
    fn new() -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.activate(Workspace::Design);
        let id = app
            .state
            .schematic
            .add_component(ComponentType::VoltageSource, crate::state::Point::origin());
        app.state.schematic.clear_undo_history();
        app.state.schematic.is_dirty = false;
        Self {
            ctx,
            app,
            id,
            controls: HashMap::new(),
        }
    }

    fn pass(&mut self, events: Vec<egui::Event>) {
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(360.0, 260.0),
                )),
                events,
                ..Default::default()
            },
            |ctx| {
                self.app.handle_shortcuts(ctx);
                let component = self
                    .app
                    .state
                    .schematic
                    .components
                    .iter()
                    .find(|component| component.id == self.id)
                    .unwrap()
                    .clone();
                egui::CentralPanel::default().show(ctx, |ui| {
                    edit_row_with_hint(
                        ui,
                        &mut self.app,
                        &component,
                        InlineEditField::Instance,
                        "Instance",
                        "",
                    );
                    edit_row_with_hint(
                        ui,
                        &mut self.app,
                        &component,
                        InlineEditField::Value,
                        "Value",
                        "",
                    );
                });
            },
        );
        if let Some(update) = output.platform_output.accesskit_update {
            for (_, node) in update.nodes {
                if let (Some(label), Some(bounds)) = (node.label(), node.bounds()) {
                    self.controls.insert(
                        label.to_owned(),
                        egui::Rect::from_min_max(
                            egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                            egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                        ),
                    );
                }
            }
        }
    }

    fn click(&mut self, label: &str) {
        let pos = self.controls[label].center();
        self.pass(vec![
            egui::Event::PointerMoved(pos),
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ]);
        self.pass(vec![egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }]);
    }

    fn replace(&mut self, text: &str) {
        self.pass(vec![
            key(egui::Key::A, egui::Modifiers::COMMAND),
            egui::Event::Text(text.to_owned()),
        ]);
    }

    fn edit(&mut self, label: &str, text: &str) {
        self.pass(Vec::new());
        self.click(label);
        self.replace(text);
    }
}

fn key(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers,
    }
}

#[test]
fn direct_property_dialog_open_captures_the_committed_inline_value() {
    let mut editor = Editor::new();
    editor.edit("Value", "12");
    crate::workbench::app::open_property_editor(&mut editor.app.state, editor.id);
    assert_eq!(editor.app.state.schematic.components[0].value, "12");
    assert_eq!(
        editor
            .app
            .state
            .tabbed_property_dialog
            .component_baseline
            .as_ref()
            .unwrap()
            .value,
        "12"
    );
    assert!(editor.app.state.workbench.inline_edit.session().is_none());
}

#[test]
fn escape_discards_valid_and_invalid_fields_without_design_or_history_changes() {
    for (label, draft) in [
        ("Instance", "V9"),
        ("Instance", "wrong name"),
        ("Value", "12"),
    ] {
        let mut editor = Editor::new();
        let before = crate::state::SchematicSnapshot::capture(&editor.app.state.schematic);
        let topology = editor.app.state.schematic.topology_version();
        editor.edit(label, draft);
        assert!(before.is_equal_state(&editor.app.state.schematic));
        editor.pass(vec![key(egui::Key::Escape, egui::Modifiers::NONE)]);
        assert!(
            editor.app.state.workbench.inline_edit.session().is_none(),
            "{label}: {draft}"
        );
        assert!(before.is_equal_state(&editor.app.state.schematic));
        assert!(!editor.app.state.schematic.is_dirty);
        assert_eq!(editor.app.state.schematic.topology_version(), topology);
        assert!(!editor.app.state.schematic.can_undo());
        assert!(editor.app.state.project_undo_sequence().is_none());
    }
}

#[test]
fn switching_fields_in_either_render_order_commits_each_edit_once() {
    for order in [
        [("Instance", "V9"), ("Value", "12")],
        [("Value", "12"), ("Instance", "V9")],
    ] {
        let mut editor = Editor::new();
        let before = editor.app.state.schematic.components[0].clone();
        editor.edit(order[0].0, order[0].1);
        editor.edit(order[1].0, order[1].1);
        editor.pass(vec![key(egui::Key::Enter, egui::Modifiers::NONE)]);
        assert_eq!(editor.app.state.schematic.components[0].name, "V9");
        assert_eq!(editor.app.state.schematic.components[0].value, "12");
        editor.app.action_edit_undo();
        if order[1].0 == "Value" {
            assert_eq!(editor.app.state.schematic.components[0].name, "V9");
            assert_eq!(editor.app.state.schematic.components[0].value, before.value);
        } else {
            assert_eq!(editor.app.state.schematic.components[0].name, "V1");
            assert_eq!(editor.app.state.schematic.components[0].value, "12");
        }
        editor.app.action_edit_undo();
        assert_eq!(editor.app.state.schematic.components[0], before);
        assert!(!editor.app.state.schematic.can_undo());
        assert!(editor.app.state.project_undo_sequence().is_none());
        editor.app.action_edit_redo();
        editor.app.action_edit_redo();
        assert_eq!(editor.app.state.schematic.components[0].name, "V9");
        assert_eq!(editor.app.state.schematic.components[0].value, "12");
    }
}

#[test]
fn a_rejected_field_keeps_its_text_and_focus_until_repaired_or_cancelled() {
    let mut editor = Editor::new();
    editor.edit("Instance", "not a designator");
    editor.click("Value");
    let session = editor.app.state.workbench.inline_edit.session().unwrap();
    assert_eq!(session.field, InlineEditField::Instance);
    assert_eq!(session.buffer, "not a designator");
    assert!(session.error.is_some());
    assert_eq!(editor.ctx.memory(|memory| memory.focused()), session.widget);
    assert_eq!(editor.app.state.schematic.components[0].name, "V1");
    editor.replace("V9");
    editor.pass(vec![key(egui::Key::Enter, egui::Modifiers::NONE)]);
    assert_eq!(editor.app.state.schematic.components[0].name, "V9");
    assert!(editor.app.state.workbench.inline_edit.session().is_none());
}

#[test]
fn workspace_and_run_commands_resolve_the_pending_field_first() {
    let mut editor = Editor::new();
    editor.edit("Instance", "invalid name");
    Command::OpenWorkspace(Workspace::Simulate).execute(&mut editor.app);
    assert_eq!(editor.app.state.workbench.workspace, Workspace::Design);
    Command::RunSimulation.execute(&mut editor.app);
    assert!(
        !editor
            .app
            .state
            .workbench
            .preflight
            .take_run_and_queue_request()
    );
    editor.app.execute_shortcut_command(Command::RunSimulation);
    assert!(
        !editor
            .app
            .state
            .workbench
            .preflight
            .take_run_and_queue_request()
    );
    assert_eq!(editor.app.state.schematic.components[0].name, "V1");
    assert!(editor.app.state.workbench.inline_edit.session().is_some());
    editor.replace("V9");
    Command::OpenWorkspace(Workspace::Simulate).execute(&mut editor.app);
    assert_eq!(editor.app.state.workbench.workspace, Workspace::Simulate);
    assert_eq!(editor.app.state.schematic.components[0].name, "V9");
    assert!(editor.app.state.workbench.inline_edit.session().is_none());
}

#[test]
fn opening_another_document_commits_to_the_original_component_and_preserves_refusals() {
    for valid in [false, true] {
        let mut editor = Editor::new();
        let original = editor.app.state.workspace.active_schematic_reference();
        let other = CellViewRef::new(&original.library, "other", "schematic");
        let mut other_schematic = crate::state::SchematicState::default();
        other_schematic
            .components
            .push(editor.app.state.schematic.components[0].clone());
        editor
            .app
            .state
            .workspace
            .schematic_buffers
            .insert(other.key(), other_schematic);
        editor.edit("Instance", if valid { "V9" } else { "invalid name" });
        editor.app.state.open_workspace_view(other.clone());
        if valid {
            assert_eq!(
                editor.app.state.workspace.active_schematic_reference(),
                other
            );
            assert_eq!(editor.app.state.schematic.components[0].name, "V1");
            assert_eq!(
                editor.app.state.workspace.schematic_buffers[&original.key()].components[0].name,
                "V9"
            );
            editor.app.action_edit_undo();
            assert_eq!(
                editor.app.state.workspace.active_schematic_reference(),
                original
            );
            assert_eq!(editor.app.state.schematic.components[0].name, "V1");
        } else {
            assert_eq!(
                editor.app.state.workspace.active_schematic_reference(),
                original
            );
            assert_eq!(editor.app.state.schematic.components[0].name, "V1");
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
fn changed_document_authority_or_component_never_reauthorizes_an_old_draft() {
    for change in ["design", "buffer", "project", "view", "component"] {
        let mut editor = Editor::new();
        editor.edit("Instance", "V9");
        match change {
            "design" => editor.app.state.design_execution_epoch += 1,
            "buffer" => editor.app.state.active_schematic_epoch += 1,
            "project" => {
                editor.app.state.workspace.project = crate::state::ProjectDescriptor::default()
            }
            "view" => editor.app.state.workspace.active_view.cell = "different".to_owned(),
            "component" => editor.app.state.schematic.components[0].value = "99".to_owned(),
            _ => unreachable!(),
        }
        let before = crate::state::SchematicSnapshot::capture(&editor.app.state.schematic);
        assert!(
            editor.app.state.commit_inline_component_edit().is_err(),
            "{change}"
        );
        assert!(
            before.is_equal_state(&editor.app.state.schematic),
            "{change}"
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
            "V9"
        );
        assert!(editor.app.state.project_undo_sequence().is_none());
    }
}

#[test]
fn rendered_instance_name_is_a_draft_until_one_reference_transaction_commits() {
    let mut editor = Editor::new();
    let plan = editor
        .app
        .state
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
        .workspace
        .add_saved_output(plan, output)
        .unwrap();
    editor.pass(Vec::new());
    editor.click("Instance");
    editor.replace("V9");
    assert_eq!(
        editor.app.state.schematic.components[0].name, "V1",
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
    assert!(!editor.app.state.schematic.can_undo());
    editor.pass(vec![key(egui::Key::Enter, egui::Modifiers::NONE)]);
    assert_eq!(editor.app.state.schematic.components[0].name, "V9");
    assert_eq!(
        editor
            .app
            .state
            .workspace
            .plan_data(plan)
            .unwrap()
            .saved_outputs[0]
            .source_expression,
        "I(V9)"
    );
    editor.app.action_edit_undo();
    assert_eq!(editor.app.state.schematic.components[0].name, "V1");
    assert_eq!(
        editor
            .app
            .state
            .workspace
            .plan_data(plan)
            .unwrap()
            .saved_outputs[0]
            .source_expression,
        "I(V1)"
    );
    assert!(editor.app.state.project_undo_sequence().is_none());
    assert!(!editor.app.state.schematic.can_undo());
    editor.app.action_edit_redo();
    assert_eq!(editor.app.state.schematic.components[0].name, "V9");
    assert_eq!(
        editor
            .app
            .state
            .workspace
            .plan_data(plan)
            .unwrap()
            .saved_outputs[0]
            .source_expression,
        "I(V9)"
    );
}
