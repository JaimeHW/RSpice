//! The object properties dialog drawn through a whole application.

use crate::state::Bus;
use crate::state::BusDeclaration;
use crate::state::DesignNote;
use crate::state::DesignNoteKind;
use crate::state::Point;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::object_properties::tests::{
    declared_bus, draft_resolution_key, key_event, render,
};
use crate::workbench::app::dialogs::object_properties::*;
use crate::workbench::app_state::AppState;
use egui::Context;
use rspice_schematic_editor::object_properties::ObjectPropertiesDraft;

/// Why the open edit can no longer apply, over what the form reads.
fn object_property_session_error(state: &AppState) -> Option<String> {
    crate::workbench::app::dialogs::object_properties::object_property_session_error(
        &state.design.object_properties,
        &state.model,
    )
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

fn open_bus_dialog(app: &mut RSpiceApp, bus: &Bus) {
    app.state.design.object_properties.open_bus(
        bus,
        crate::workbench::app::schematic_editor_request_source(&app.state),
        app.state.model.workspace.content.active_view.display_path(),
    );
}

#[test]
fn retained_properties_reject_changed_context_without_committing() {
    for change in [
        "project",
        "view",
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
    ] {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        let master = crate::state::CellViewRef::new("work", "property_parent", "schematic");
        app.state.model.workspace.descend_into(
            "X1".to_owned(),
            master.clone(),
            crate::state::ViewType::Schematic,
        );
        let first = app
            .state
            .model
            .workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&master.key(), "Sheet 1", [])
            .unwrap();
        let bus = declared_bus(1, 0, "DATA[7:0]");
        let note = DesignNote::new(
            99,
            Point::new(0, 20),
            DesignNoteKind::PlainText,
            "Original note",
        )
        .unwrap();
        app.state
            .model
            .schematic
            .document_mut_for_test()
            .buses
            .push(bus.clone());
        app.state
            .model
            .schematic
            .document_mut_for_test()
            .design_notes
            .push(note.clone());
        app.state.model.schematic.init_undo_history();
        open_bus_dialog(&mut app, &bus);
        assert!(object_property_session_error(&app.state).is_none());
        let Some(ObjectPropertiesDraft::Bus(draft)) =
            app.state.design.object_properties.draft.as_mut()
        else {
            panic!("bus draft")
        };
        draft.declaration = "ADDR[7:0]".to_owned();
        app.state.design.object_properties.mark_edited();
        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| render(&mut app, ctx));
        let cached = ctx
            .data(|data| {
                data.get_temp::<CachedDraftResolution>(egui::Id::new(
                    "object-properties-draft-resolution-cache",
                ))
            })
            .expect("valid initial draft warms the repaint cache");
        assert!(cached.validation.can_commit());
        match change {
            "project" => app.state.model.workspace.content.project = Default::default(),
            "view" => {
                app.state.model.workspace.content.active_view =
                    crate::state::CellViewRef::new("work", "replacement", "schematic")
            }
            "design" => app.state.model.design_execution_epoch += 1,
            "buffer" => app.state.model.active_schematic_epoch += 1,
            "occurrence" => {
                app.state.model.workspace.ascend_one().unwrap();
                app.state.model.workspace.descend_into(
                    "X2".to_owned(),
                    master.clone(),
                    crate::state::ViewType::Schematic,
                );
            }
            "sheet" | "sheet-revision" => {
                let catalog = app
                    .state
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
                let topology = app.state.model.schematic.topology_version();
                assert!(
                    app.state
                        .model
                        .schematic
                        .edit_design_note_properties(
                            note,
                            DesignNoteKind::PlainText,
                            "Changed note".to_owned(),
                            None,
                        )
                        .unwrap()
                );
                assert_eq!(app.state.model.schematic.topology_version(), topology);
            }
            "topology" => app.state.model.schematic.bump_topology_version(),
            "symbol-context" => {
                app.state
                    .model
                    .workspace
                    .content
                    .schematic_buffers
                    .insert("work/other/schematic".to_owned(), Default::default());
            }
            "read-only" => app.state.model.schematic.session.read_only = true,
            "safe-mode" => app.state.model.safe_mode.activate(
                crate::state::write_policy::LocalSafeModeOptions {
                    open_project_read_only: true,
                    ..Default::default()
                },
                "object-properties test".to_owned(),
            ),
            "missing" => app.state.design.object_properties.source = None,
            _ => unreachable!(),
        }
        assert!(
            object_property_session_error(&app.state).is_some(),
            "{change}"
        );
        if !matches!(change, "read-only" | "safe-mode" | "missing") {
            let draft = app.state.design.object_properties.draft.as_ref().unwrap();
            assert_ne!(
                draft_resolution_key(&app.state, draft),
                cached.key,
                "{change}"
            );
        }
        let before = crate::state::SchematicSnapshot::capture(app.state.model.schematic.document());
        let content = app.state.model.schematic.content_version();
        let topology = app.state.model.schematic.topology_version();
        let undo = app
            .state
            .model
            .schematic
            .undo_description()
            .map(str::to_owned);
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            render(&mut app, ctx)
        });
        assert!(app.state.design.object_properties.open, "{change}");
        assert!(
            before.is_equal_document(app.state.model.schematic.document()),
            "{change}"
        );
        assert_eq!(
            app.state.model.schematic.content_version(),
            content,
            "{change}"
        );
        assert_eq!(
            app.state.model.schematic.topology_version(),
            topology,
            "{change}"
        );
        assert_eq!(
            app.state.model.schematic.undo_description(),
            undo.as_deref(),
            "{change}"
        );
    }
}

#[test]
fn rendered_primary_revalidates_and_commits_one_undoable_bus_transaction() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    let bus = declared_bus(1, 0, "DATA[7:0]");
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .buses
        .push(bus.clone());
    app.state.model.schematic.clear_undo_history();
    open_bus_dialog(&mut app, &bus);
    let Some(ObjectPropertiesDraft::Bus(draft)) = app.state.design.object_properties.draft.as_mut()
    else {
        panic!("bus draft")
    };
    draft.declaration = "ADDR<0:15>".to_owned();
    app.state.design.object_properties.mark_edited();

    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| render(&mut app, ctx));
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
        render(&mut app, ctx)
    });

    assert!(!app.state.design.object_properties.open);
    assert!(app.state.design.object_properties.source.is_none());
    assert_eq!(
        app.state.model.schematic.document().buses[0].declaration,
        Some(BusDeclaration::parse("ADDR<0:15>").unwrap())
    );
    assert!(app.state.model.schematic.undo());
    assert_eq!(app.state.model.schematic.document().buses[0], bus);
    open_bus_dialog(&mut app, &bus);
    assert!(object_property_session_error(&app.state).is_none());
}

/// A property draft owns nothing until Apply, so Escape leaves at once
/// rather than asking a question about a document it never touched.
#[test]
fn rendered_escape_leaves_without_a_second_question() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    let bus = declared_bus(1, 0, "DATA[7:0]");
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .buses
        .push(bus.clone());
    open_bus_dialog(&mut app, &bus);
    let Some(ObjectPropertiesDraft::Bus(draft)) = app.state.design.object_properties.draft.as_mut()
    else {
        panic!("bus draft")
    };
    draft.declaration = "ADDR[7:0]".to_owned();
    app.state.design.object_properties.mark_edited();

    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| render(&mut app, ctx));
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
        render(&mut app, ctx)
    });

    assert!(!app.state.design.object_properties.open);
    assert_eq!(app.state.model.schematic.document().buses[0], bus);
}

/// The dialog is named for the object it was opened on, and says nothing
/// about models or parameters, which none of these objects has.
#[test]
fn the_title_names_the_selected_object_kind() {
    use rspice_schematic_editor::object_properties::title;
    let bus = declared_bus(1, 0, "DATA[7:0]");
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .buses
        .push(bus.clone());
    open_bus_dialog(&mut app, &bus);
    let draft = app
        .state
        .design
        .object_properties
        .draft
        .as_ref()
        .expect("bus draft");
    assert_eq!(title(draft), "Bus properties");
}

#[test]
fn accessibility_tree_exposes_modal_fields_and_actions() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    let bus = declared_bus(1, 0, "DATA[7:0]");
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .buses
        .push(bus.clone());
    open_bus_dialog(&mut app, &bus);

    let output = ctx.run_ui(dialog_input(Vec::new()), |ctx| render(&mut app, ctx));
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("object properties access tree")
        .nodes;
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Dialog && node.label() == Some("Bus properties")
    }));
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::TextInput && node.label() == Some("Bus declaration")
    }));
    for label in [PRIMARY, "Cancel"] {
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
        }));
    }
}
