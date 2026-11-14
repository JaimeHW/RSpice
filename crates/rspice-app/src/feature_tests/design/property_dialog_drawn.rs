//! The component property dialog opened, drawn and committed through the whole application:
//! open_property_editor takes the whole application, and a commit is undone as a step of the
//! project's design history.

use crate::state::{CellViewRef, Component, ComponentType, Point, PropertyValue};
use crate::workbench::app::dialogs::design_request::DesignRequest;
use crate::workbench::app::dialogs::property_dialog::{
    PropertyDialog, component_property_session_error,
};
use crate::workbench::app::open_property_editor;
use crate::workbench::app_state::AppState;
use rspice_design::schematic::document_policy::PropertyCommitPolicy;
use rspice_schematic_editor::component_properties::ComponentPropertyDialogResult;

/// Draw the editor as its dialog step does, over the application's parts,
/// then answer what it raised as the shell does straight after.
fn render_property_dialog(
    ctx: &egui::Context,
    state: &mut AppState,
) -> ComponentPropertyDialogResult {
    let result = PropertyDialog {
        dialog: &mut state.design.tabbed_property_dialog,
        registry: &mut state.design.property_registry,
        stimulus_link: &mut state.design.stimulus_link,
        selected_stimulus_definition: &mut state.design.selected_stimulus_definition,
        model: &mut state.model,
        workbench: &mut state.workbench,
    }
    .render_property_dialog(ctx);
    if let Some(DesignRequest::ReopenPropertyEditor(component_id)) =
        state.workbench.outbox.take::<DesignRequest>()
    {
        open_property_editor(state, component_id);
    }
    result
}

fn dialog_input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1_100.0, 850.0),
        )),
        events,
        ..Default::default()
    }
}

fn key_event(key: egui::Key) -> egui::Event {
    key_event_with_modifiers(key, egui::Modifiers::NONE)
}

fn key_event_with_modifiers(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers,
    }
}

fn state_with_resistor() -> AppState {
    let mut state = AppState::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    let mut component = Component::new(44, ComponentType::Resistor, Point::new(4, 8));
    component.name = "R1".to_owned();
    component.value = "1k".to_owned();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(component);
    state.model.schematic.clear_undo_history();
    state
}

fn add_current_output(state: &mut AppState) -> crate::product::SimulationPlanId {
    let plan = state.model.sim_setup.stable_analysis_plan().unwrap().id();
    let output = crate::state::SavedOutput::new(
        crate::state::SavedOutputKind::RawVoltageOrCurrent,
        "resistor current",
        "I(R1)",
        crate::state::SavedOutputCompatibility::OpTranAc,
        crate::state::SavedOutputPolicy::EveryAcceptedPoint,
        crate::state::SavedOutputPrecision::FullSourcePrecision,
        crate::state::SavedOutputStreaming::StoreOnly,
    )
    .unwrap();
    state
        .model
        .workspace
        .content
        .add_saved_output(plan, output)
        .unwrap();
    plan
}

#[test]
fn retained_component_properties_reject_changed_context_without_committing() {
    for change in [
        "project",
        "view",
        "symbol-tab",
        "design",
        "buffer",
        "occurrence",
        "sheet",
        "sheet-revision",
        "content",
        "topology",
        "symbol-context",
        "read-only",
        "safe-mode",
        "missing",
        "object",
    ] {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut state = state_with_resistor();
        let master = CellViewRef::new("work", "property_parent", "schematic");
        state.model.workspace.descend_into(
            "X1".to_owned(),
            master.clone(),
            crate::state::ViewType::Schematic,
        );
        let first = state
            .model
            .workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&master.key(), "Sheet 1", [])
            .unwrap();
        let note = crate::state::DesignNote::new(
            99,
            Point::new(0, 20),
            crate::state::DesignNoteKind::PlainText,
            "Original note",
        )
        .unwrap();
        state
            .model
            .schematic
            .document_mut_for_test()
            .design_notes
            .push(note.clone());
        state.model.schematic.init_undo_history();
        open_property_editor(&mut state, 44);
        assert_eq!(
            state.design.tabbed_property_dialog.source,
            Some(crate::workbench::app::schematic_editor_request_source(
                &state
            ))
        );
        assert!(
            component_property_session_error(&state.design.tabbed_property_dialog, &state.model)
                .is_none()
        );
        state
            .design
            .tabbed_property_dialog
            .draft
            .set_value("r", PropertyValue::Expression("2k".to_owned()));
        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        assert!(state.design.tabbed_property_dialog.session_error.is_none());
        match change {
            "project" => state.model.workspace.content.project = Default::default(),
            "view" => {
                state.model.workspace.content.active_view =
                    CellViewRef::new("work", "replacement", "schematic")
            }
            "symbol-tab" => {
                state.model.workspace.content.active_view =
                    CellViewRef::new("work", "property_parent", "symbol")
            }
            "design" => state.model.design_execution_epoch += 1,
            "buffer" => state.model.active_schematic_epoch += 1,
            "occurrence" => {
                state.model.workspace.ascend_one().unwrap();
                state.model.workspace.descend_into(
                    "X2".to_owned(),
                    master.clone(),
                    crate::state::ViewType::Schematic,
                );
            }
            "sheet" | "sheet-revision" => {
                let catalog = state
                    .model
                    .workspace
                    .content
                    .design_management
                    .sheet_catalog_mut(&master.key())
                    .unwrap();
                let second = catalog
                    .create_sheet(
                        crate::state::SheetDefinition {
                            name: "Sheet 2".to_owned(),
                            template: crate::state::SheetTemplate::AnalogSchematic,
                            port_policy: crate::state::SheetPortPolicy::TypedOffSheetPorts,
                            explicit_page_number: Some(2),
                        },
                        Some(first),
                    )
                    .unwrap();
                if change == "sheet" {
                    catalog.set_active(second).unwrap();
                }
            }
            "content" => {
                let topology = state.model.schematic.topology_version();
                assert!(
                    state
                        .model
                        .schematic
                        .edit_design_note_properties(
                            note,
                            crate::state::DesignNoteKind::PlainText,
                            "Changed note".to_owned(),
                            None,
                        )
                        .unwrap()
                );
                assert_eq!(state.model.schematic.topology_version(), topology);
            }
            "topology" => state.model.schematic.bump_topology_version(),
            "symbol-context" => {
                state
                    .model
                    .workspace
                    .content
                    .schematic_buffers
                    .insert("work/other/schematic".to_owned(), Default::default());
            }
            "read-only" => state.model.schematic.session.read_only = true,
            "safe-mode" => state.model.safe_mode.activate(
                crate::state::write_policy::LocalSafeModeOptions {
                    open_project_read_only: true,
                    ..Default::default()
                },
                "component-properties test".to_owned(),
            ),
            "missing" => state.design.tabbed_property_dialog.source = None,
            "object" => {
                state.model.schematic.document_mut_for_test().components[0].name = "R2".to_owned()
            }
            _ => unreachable!(),
        }
        assert!(
            component_property_session_error(&state.design.tabbed_property_dialog, &state.model)
                .is_some(),
            "{change}"
        );
        let source = state.design.tabbed_property_dialog.source.clone();
        let before = crate::state::SchematicSnapshot::capture(state.model.schematic.document());
        let versions = (
            state.model.schematic.content_version(),
            state.model.schematic.topology_version(),
        );
        let undo = state.model.schematic.undo_description().map(str::to_owned);
        let project_undo = state.model.project_undo_sequence();
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        assert!(state.design.tabbed_property_dialog.open, "{change}");
        assert!(
            state.design.tabbed_property_dialog.session_error.is_some(),
            "{change}"
        );
        assert_eq!(
            state.design.tabbed_property_dialog.source, source,
            "{change}"
        );
        assert_eq!(
            state.design.tabbed_property_dialog.draft.values["r"],
            PropertyValue::Expression("2k".to_owned()),
            "{change}"
        );
        assert!(
            state.design.tabbed_property_dialog.draft.is_modified("r"),
            "{change}"
        );
        assert!(
            before.is_equal_document(state.model.schematic.document()),
            "{change}"
        );
        assert_eq!(
            (
                state.model.schematic.content_version(),
                state.model.schematic.topology_version()
            ),
            versions,
            "{change}"
        );
        assert_eq!(
            state.model.schematic.undo_description(),
            undo.as_deref(),
            "{change}"
        );
        assert_eq!(
            state.model.project_undo_sequence(),
            project_undo,
            "{change}"
        );
    }
}

#[test]
fn rendered_component_primary_commits_name_with_one_real_undo_record() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut state = state_with_resistor();
    let plan = add_current_output(&mut state);
    open_property_editor(&mut state, 44);
    state
        .design
        .tabbed_property_dialog
        .draft
        .set_value("name", PropertyValue::String("R99".to_owned()));
    state
        .design
        .tabbed_property_dialog
        .draft
        .set_value("r", PropertyValue::Expression("2k".to_owned()));

    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
        render_property_dialog(ctx, &mut state);
    });
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
        render_property_dialog(ctx, &mut state);
    });

    assert!(!state.design.tabbed_property_dialog.open);
    assert!(state.design.tabbed_property_dialog.source.is_none());
    assert_eq!(state.model.schematic.document().components[0].name, "R99");
    assert_eq!(state.model.schematic.document().components[0].value, "2k");
    assert_eq!(
        state
            .model
            .workspace
            .content
            .plan_data(plan)
            .unwrap()
            .saved_outputs[0]
            .source_expression,
        "I(R99)"
    );
    assert!(!state.model.schematic.can_undo());
    assert_eq!(
        state.undo_project_design().unwrap(),
        Some("edit properties".to_owned())
    );
    assert_eq!(state.model.schematic.document().components[0].name, "R1");
    assert_eq!(state.model.schematic.document().components[0].value, "1k");
    assert_eq!(
        state
            .model
            .workspace
            .content
            .plan_data(plan)
            .unwrap()
            .saved_outputs[0]
            .source_expression,
        "I(R1)"
    );
    assert!(state.model.project_undo_sequence().is_none());
    assert!(state.redo_project_design().unwrap().is_some());
    assert_eq!(state.model.schematic.document().components[0].name, "R99");
    assert_eq!(state.model.schematic.document().components[0].value, "2k");
    assert_eq!(
        state
            .model
            .workspace
            .content
            .plan_data(plan)
            .unwrap()
            .saved_outputs[0]
            .source_expression,
        "I(R99)"
    );
}

#[test]
fn rendered_property_commit_refusal_keeps_the_design_and_draft_intact() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut state = state_with_resistor();
    let plan = add_current_output(&mut state);
    state
        .model
        .workspace
        .content
        .plan_data_mut(plan)
        .unwrap()
        .saved_outputs[0]
        .revision = crate::product::ObjectRevision::new(u64::MAX).unwrap();
    let before = crate::state::SchematicSnapshot::capture(state.model.schematic.document());
    let payloads = state
        .model
        .workspace
        .content
        .simulation_plan_payloads
        .clone();
    open_property_editor(&mut state, 44);
    let opened_source = state.design.tabbed_property_dialog.source.clone();
    state
        .design
        .tabbed_property_dialog
        .draft
        .set_value("name", PropertyValue::String("R99".to_owned()));
    state
        .design
        .tabbed_property_dialog
        .draft
        .set_value("r", PropertyValue::Expression("2k".to_owned()));
    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
        render_property_dialog(ctx, &mut state);
    });
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
        render_property_dialog(ctx, &mut state);
    });
    assert!(before.is_equal_document(state.model.schematic.document()));
    assert_eq!(
        state.model.workspace.content.simulation_plan_payloads,
        payloads
    );
    assert!(state.design.tabbed_property_dialog.open);
    assert!(
        state
            .design
            .tabbed_property_dialog
            .draft
            .commit_error
            .is_some()
    );
    assert_eq!(state.design.tabbed_property_dialog.source, opened_source);
    assert!(
        state
            .design
            .tabbed_property_dialog
            .draft
            .is_modified("name")
    );
    assert!(state.design.tabbed_property_dialog.draft.is_modified("r"));
    assert_eq!(
        state.design.tabbed_property_dialog.draft.values["name"],
        PropertyValue::String("R99".to_owned())
    );
    assert!(!state.model.schematic.can_undo());
    assert!(state.model.project_undo_sequence().is_none());
}

#[test]
fn malformed_parameter_refusal_keeps_rendered_property_edits_and_references_isolated() {
    for source in ["note='unterminated", "temp=27 TEMP=85"] {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut state = state_with_resistor();
        state.model.schematic.document_mut_for_test().components[0].params = source.to_owned();
        add_current_output(&mut state);
        let before = crate::state::SchematicSnapshot::capture(state.model.schematic.document());
        let payloads = state
            .model
            .workspace
            .content
            .simulation_plan_payloads
            .clone();
        open_property_editor(&mut state, 44);
        state
            .design
            .tabbed_property_dialog
            .draft
            .set_value("name", PropertyValue::String("R99".to_owned()));
        state
            .design
            .tabbed_property_dialog
            .draft
            .set_value("r", PropertyValue::Expression("2k".to_owned()));
        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        assert!(before.is_equal_document(state.model.schematic.document()));
        assert_eq!(
            state.model.workspace.content.simulation_plan_payloads,
            payloads
        );
        assert!(state.design.tabbed_property_dialog.open);
        assert!(
            state
                .design
                .tabbed_property_dialog
                .draft
                .commit_error
                .as_ref()
                .or(state.design.tabbed_property_dialog.session_error.as_ref())
                .unwrap()
                .contains("Parameters")
        );
        assert!(
            state
                .design
                .tabbed_property_dialog
                .draft
                .is_modified("name")
        );
        assert!(state.design.tabbed_property_dialog.draft.is_modified("r"));
        assert!(!state.model.schematic.can_undo());
        assert!(state.model.project_undo_sequence().is_none());
    }
}

#[test]
fn component_dirty_escape_discards_the_isolated_draft() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut state = state_with_resistor();
    open_property_editor(&mut state, 44);
    state
        .design
        .tabbed_property_dialog
        .draft
        .set_value("name", PropertyValue::String("R99".to_owned()));

    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
        render_property_dialog(ctx, &mut state);
    });
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
        render_property_dialog(ctx, &mut state);
    });
    assert!(!state.design.tabbed_property_dialog.open);
    assert!(state.design.tabbed_property_dialog.source.is_none());
    assert_eq!(state.model.schematic.document().components[0].name, "R1");
}

#[test]
fn returning_to_the_same_view_cannot_reauthorize_an_old_dialog() {
    let mut state = state_with_resistor();
    open_property_editor(&mut state, 44);
    let captured_epoch = state
        .design
        .tabbed_property_dialog
        .source
        .as_ref()
        .unwrap()
        .document_epoch;
    let original_view = state.model.workspace.content.active_view.clone();

    state.open_workspace_view(CellViewRef::new("work", "detour", "schematic"));
    state.open_workspace_view(original_view.clone());

    assert_eq!(state.model.workspace.content.active_view, original_view);
    assert_ne!(state.model.active_schematic_epoch, captured_epoch);
    assert!(
        component_property_session_error(&state.design.tabbed_property_dialog, &state.model)
            .is_some()
    );
}

#[test]
fn escape_closes_nested_model_browser_before_dirty_parent_dialog() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut state = state_with_resistor();
    open_property_editor(&mut state, 44);
    state
        .design
        .tabbed_property_dialog
        .draft
        .set_value("name", PropertyValue::String("R99".to_owned()));
    state.design.tabbed_property_dialog.model_browser.open = true;

    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
        render_property_dialog(ctx, &mut state);
    });
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
        render_property_dialog(ctx, &mut state);
    });

    assert!(!state.design.tabbed_property_dialog.model_browser.open);
    assert!(state.design.tabbed_property_dialog.open);
    assert_eq!(state.model.schematic.document().components[0].name, "R1");
}

#[test]
fn first_parameter_editor_retains_invalid_source_across_repaints() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut state = state_with_resistor();
    open_property_editor(&mut state, 44);
    let original = state
        .design
        .tabbed_property_dialog
        .draft
        .numeric_text_draft("r")
        .expect("resistance source")
        .to_owned();

    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
        render_property_dialog(ctx, &mut state);
    });
    let _ = ctx.run_ui(
        dialog_input(vec![
            // Move explicitly to the end. Ctrl+A is Select All on
            // Windows/Linux but an Emacs-style line-start command on
            // macOS, which made this retained-source contract depend on
            // the host platform rather than the editor behavior under
            // test.
            key_event(egui::Key::End),
            egui::Event::Text("1e".to_owned()),
        ]),
        |ctx| {
            render_property_dialog(ctx, &mut state);
        },
    );
    let expected = format!("{original}1e");
    assert_eq!(
        state
            .design
            .tabbed_property_dialog
            .draft
            .numeric_text_draft("r"),
        Some(expected.as_str())
    );
    assert!(
        state
            .design
            .tabbed_property_dialog
            .draft
            .validation_errors
            .contains_key("r")
    );

    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
        render_property_dialog(ctx, &mut state);
    });
    assert_eq!(
        state
            .design
            .tabbed_property_dialog
            .draft
            .numeric_text_draft("r"),
        Some(expected.as_str())
    );
}

/// A document an earlier build stored with "apply valid fields" still
/// loads, and its edits apply whole: one invalid field holds back the
/// valid one beside it until it is fixed.
#[test]
fn a_stored_partial_commit_setting_applies_edits_whole() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut state = state_with_resistor();
    state
        .model
        .schematic
        .document_mut_for_test()
        .document_policy
        .property_commit = PropertyCommitPolicy::ApplyValidFields;
    open_property_editor(&mut state, 44);
    let opened_source = state.design.tabbed_property_dialog.source.clone();
    state
        .design
        .tabbed_property_dialog
        .draft
        .set_value("name", PropertyValue::String(" R99 ".to_owned()));
    state
        .design
        .tabbed_property_dialog
        .draft
        .update_numeric_text_draft(
            "m",
            "0".to_owned(),
            Some("Multiplier must be at least 1".to_owned()),
        );

    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
        render_property_dialog(ctx, &mut state);
    });
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
        render_property_dialog(ctx, &mut state);
    });

    // The valid name is held back with the invalid multiplier.
    assert!(state.design.tabbed_property_dialog.open);
    assert_eq!(state.model.schematic.document().components[0].name, "R1");
    assert_eq!(state.model.schematic.document().components[0].params, "");
    assert_eq!(state.model.schematic.history().undo_count(), 0);
    assert_eq!(state.design.tabbed_property_dialog.source, opened_source);
    assert_eq!(
        state
            .design
            .tabbed_property_dialog
            .draft
            .numeric_text_draft("m"),
        Some("0")
    );
    assert!(state.design.tabbed_property_dialog.draft.is_modified("m"));
    assert!(
        state
            .design
            .tabbed_property_dialog
            .draft
            .is_modified("name")
    );

    state
        .design
        .tabbed_property_dialog
        .draft
        .update_numeric_text_draft("m", "2".to_owned(), None);
    state
        .design
        .tabbed_property_dialog
        .draft
        .set_value("m", PropertyValue::number(2.0));
    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
        render_property_dialog(ctx, &mut state);
    });
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
        render_property_dialog(ctx, &mut state);
    });

    assert!(!state.design.tabbed_property_dialog.open);
    assert!(state.design.tabbed_property_dialog.source.is_none());
    assert_eq!(state.model.schematic.document().components[0].name, "R99");
    assert_eq!(state.model.schematic.document().components[0].params, "m=2");
}
