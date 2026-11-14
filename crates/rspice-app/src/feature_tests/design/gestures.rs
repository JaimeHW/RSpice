//! Canvas gestures driven through a whole application's frame.

use crate::state::{Point, SchematicState, Tool};
use crate::workbench::app::gesture_tests::Fixture;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::DragType;

#[test]
fn symbol_draft_survives_passive_window_projection() {
    use crate::state::{
        Cell, CellViewRef, SymbolDocument, SymbolEditorMetadata, SymbolShape, View, ViewType,
    };
    use crate::workbench::{SymbolTool, state::WorkspaceDocumentId};

    let mut failures = Vec::new();
    for other_is_symbol in [false, true] {
        let mut fixture = Fixture::new(false);
        let library = fixture
            .app
            .state
            .model
            .workspace
            .content
            .active_view
            .library
            .clone();
        let document = SymbolDocument {
            body: vec![SymbolShape::Circle {
                center: Point::origin(),
                radius: 10,
            }],
            ..Default::default()
        };
        let mut add_symbol = |name: &str| {
            let reference = CellViewRef::new(&library, name, "symbol");
            let mut cell = Cell::new(name);
            cell.add_view(View::new("symbol", ViewType::Symbol));
            fixture
                .app
                .state
                .model
                .library_manager
                .get_library_mut(&library)
                .unwrap()
                .add_cell(cell);
            fixture.app.state.open_workspace_view(reference.clone());
            fixture
                .app
                .state
                .store_active_symbol_editor_bundle(
                    &document,
                    &SymbolEditorMetadata::for_document(&document),
                )
                .unwrap();
            reference
        };
        let primary_view = add_symbol("symbol-owner");
        let other = if other_is_symbol {
            add_symbol("symbol-other")
        } else {
            fixture.second_document()
        };
        fixture.app.state.open_workspace_view(primary_view.clone());
        fixture.frame(vec![], true);
        let points = vec![Point::origin(), Point::new(20, 10)];
        fixture.app.state.design.symbol.editor.tool = SymbolTool::Line;
        fixture.app.state.design.symbol.editor.pending_polyline = points.clone();
        fixture.app.state.design.symbol.editor.select_shape(0);
        fixture.app.state.design.symbol.editor.dragging_origin = true;
        fixture.app.state.design.symbol.editor.save_revision_note = "Owner draft".into();
        fixture.app.state.design.symbol.editor.clipboard.shapes = document.body.clone();
        let primary = fixture.app.state.workbench.window_session.primary();
        let layout = fixture.app.state.workbench.current_workspace_layout();
        let secondary = fixture
            .app
            .state
            .workbench
            .window_session
            .detach_document(WorkspaceDocumentId::CellView(other), "Other", layout, false)
            .unwrap();
        fixture.app.capture_application_window_projection(primary);
        assert!(fixture.app.project_application_window(secondary, layout));
        fixture.frame(vec![], true);
        if other_is_symbol {
            assert!(
                fixture
                    .app
                    .state
                    .design
                    .symbol
                    .editor
                    .pending_polyline
                    .is_empty()
            );
            assert!(
                fixture
                    .app
                    .state
                    .design
                    .symbol
                    .editor
                    .save_revision_note
                    .is_empty()
            );
            assert_eq!(
                fixture.app.state.design.symbol.editor.clipboard.shapes,
                document.body
            );
            fixture.app.state.design.symbol.editor.shape_start = Some(Point::new(30, 40));
        }
        assert!(fixture.app.project_application_window(primary, layout));
        fixture.frame(vec![], true);
        if fixture.app.state.design.symbol.editor.pending_polyline != points
            || !fixture
                .app
                .state
                .design
                .symbol
                .editor
                .effective_selection()
                .shapes
                .contains(&0)
        {
            failures.push(if other_is_symbol {
                "symbol"
            } else {
                "schematic"
            });
        }
        assert_eq!(
            fixture.app.state.model.workspace.content.active_view,
            primary_view
        );
        assert_eq!(
            fixture
                .app
                .state
                .model
                .load_active_symbol_document()
                .unwrap(),
            document
        );
        assert_eq!(
            fixture
                .app
                .state
                .design
                .symbol
                .history
                .undo_depth(&primary_view.key()),
            0
        );
        assert_eq!(
            fixture.app.state.design.symbol.editor.save_revision_note,
            "Owner draft"
        );
        assert!(fixture.app.state.design.symbol.editor.dragging_origin);
        if other_is_symbol {
            assert!(fixture.app.project_application_window(secondary, layout));
            fixture.frame(vec![], true);
            assert_eq!(
                fixture.app.state.design.symbol.editor.shape_start,
                Some(Point::new(30, 40))
            );
            assert!(fixture.app.project_application_window(primary, layout));
            fixture.frame(vec![], true);
        }
        fixture
            .app
            .execute_shortcut_command(crate::workbench::commands::vocabulary::Command::Cancel);
        let edited = fixture
            .app
            .state
            .model
            .load_active_symbol_document()
            .unwrap();
        assert_eq!(edited.body.len(), document.body.len() + 1);
        assert_eq!(
            fixture
                .app
                .state
                .design
                .symbol
                .history
                .undo_depth(&primary_view.key()),
            1
        );
        assert!(fixture.app.state.undo_active_symbol_document().unwrap());
        assert_eq!(
            fixture
                .app
                .state
                .model
                .load_active_symbol_document()
                .unwrap(),
            document
        );
        assert!(fixture.app.state.redo_active_symbol_document().unwrap());
        assert_eq!(
            fixture
                .app
                .state
                .model
                .load_active_symbol_document()
                .unwrap(),
            edited
        );
    }
    assert!(
        failures.is_empty(),
        "Passive projections discarded the symbol draft: {failures:?}"
    );
}

#[test]
fn gesture_window_projection_preserves_ownership_and_inactive_save_baselines() {
    use crate::workbench::state::WorkspaceDocumentId;
    let mut fixture = Fixture::new(false);
    let original = fixture
        .app
        .state
        .model
        .workspace
        .content
        .active_schematic_reference();
    let second = fixture.second_document();
    let primary = fixture.app.state.workbench.window_session.primary();
    let layout = fixture.app.state.workbench.current_workspace_layout();
    let secondary = fixture
        .app
        .state
        .workbench
        .window_session
        .detach_document(
            WorkspaceDocumentId::CellView(second.clone()),
            "Second",
            layout,
            false,
        )
        .unwrap();
    fixture.app.capture_application_window_projection(primary);
    fixture.drag(DragType::MoveSelection);
    let operation = fixture.app.state.model.schematic.pending_operation_id();
    fixture.app.state.sync_active_schematic_to_workspace();

    for _ in 0..3 {
        assert!(fixture.app.project_application_window(secondary, layout));
        assert_eq!(
            fixture.app.state.model.workspace.content.active_view,
            second
        );
        assert!(
            !fixture
                .app
                .state
                .schematic_drag_owned_by_context(&fixture.ctx)
        );
        fixture
            .app
            .state
            .reconcile_schematic_drag(&fixture.ctx, true);
        let snapshot =
            crate::workbench::lifecycle::project_lifecycle::snapshot(&fixture.app.state).unwrap();
        assert_eq!(
            snapshot.file.workspace.schematic_buffers[&original.key()]
                .document()
                .components[0]
                .pos,
            Point::new(100, 100)
        );
        let restored: AppState =
            serde_json::from_str(&serde_json::to_string(&fixture.app.state).unwrap()).unwrap();
        assert_eq!(
            restored.model.workspace.content.schematic_buffers[&original.key()]
                .document()
                .components[0]
                .pos,
            Point::new(100, 100)
        );
        fixture.app.state.sync_active_schematic_to_workspace();
        assert!(fixture.app.project_application_window(primary, layout));
        assert!(
            fixture
                .app
                .state
                .schematic_drag_owned_by_context(&fixture.ctx)
        );
        assert_eq!(
            fixture.app.state.model.schematic.pending_operation_id(),
            operation
        );
        assert_ne!(
            fixture.app.state.model.schematic.document().components[0].pos,
            Point::new(100, 100)
        );
    }
    fixture.frame(vec![fixture.button(170.0, false)], true);
    assert!(fixture.app.state.model.schematic.undo());
    assert_eq!(
        fixture.app.state.model.schematic.document().components[0].pos,
        Point::new(100, 100)
    );
    assert!(!fixture.app.state.model.schematic.can_undo());
}

#[test]
fn conductor_routes_validate_source_before_finishing() {
    use crate::workbench::state::WorkspaceDocumentId;

    for tool in [Tool::Wire, Tool::Bus] {
        for change in [
            "unchanged",
            "occurrence",
            "sheet",
            "topology",
            "read-only",
            "replacement",
            "document round trip",
            "passive window",
            "window transfer",
        ] {
            let mut fixture = Fixture::new(false);
            let root = fixture
                .app
                .state
                .model
                .workspace
                .content
                .active_view
                .clone();
            let master = fixture.enter_reused_master();
            fixture.app.state.model.schematic.arm_tool(tool);
            for x in [300.0, 400.0] {
                fixture.frame(
                    vec![
                        egui::Event::PointerMoved(fixture.origin + egui::vec2(x, 100.0)),
                        fixture.button(x, true),
                    ],
                    true,
                );
                fixture.frame(vec![fixture.button(x, false)], true);
            }
            let editor = &fixture.app.state.model.schematic.session.editor;
            assert!(if tool == Tool::Wire {
                editor.wire_drawing.active && editor.wire_drawing.points.len() >= 2
            } else {
                editor.bus_drawing.active && editor.bus_drawing.points.len() >= 2
            });

            match change {
                "unchanged" => {}
                "occurrence" => {
                    fixture.app.state.navigation().focus_workspace_breadcrumb(0);
                    fixture
                        .app
                        .state
                        .navigation()
                        .descend_into_instance(Some("X2".into()), master);
                }
                "sheet" => {
                    let id = fixture.app.state.model.schematic.document().components[0].id;
                    fixture
                        .app
                        .state
                        .model
                        .workspace
                        .content
                        .design_management
                        .bootstrap_for_cell_view(&master.key(), "Sheet 1", [id])
                        .unwrap();
                }
                "topology" => fixture.app.state.model.schematic.bump_topology_version(),
                "read-only" => fixture.app.state.model.schematic.session.read_only = true,
                "replacement" => {
                    let document = fixture.app.state.model.schematic.document().clone();
                    fixture.app.state.model.schematic = SchematicState::from_document(document);
                    fixture.app.state.bump_active_schematic_epoch();
                }
                "document round trip" => {
                    fixture.app.state.open_workspace_view(root);
                    fixture.app.state.open_workspace_view(master);
                }
                "passive window" | "window transfer" => {
                    let primary = fixture.app.state.workbench.window_session.primary();
                    let layout = fixture.app.state.workbench.current_workspace_layout();
                    fixture.app.capture_application_window_projection(primary);
                    let secondary = fixture
                        .app
                        .state
                        .workbench
                        .window_session
                        .detach_document(
                            WorkspaceDocumentId::CellView(if change == "passive window" {
                                root
                            } else {
                                master
                            }),
                            "Second",
                            layout,
                            false,
                        )
                        .unwrap();
                    assert!(fixture.app.project_application_window(secondary, layout));
                    if change == "passive window" {
                        fixture.frame(vec![], true);
                        fixture.app.state.sync_active_schematic_to_workspace();
                        assert!(fixture.app.project_application_window(primary, layout));
                    }
                }
                _ => unreachable!(),
            }
            rspice_schematic_editor::view::canvas::request_focus(&fixture.ctx);
            fixture.frame(
                vec![egui::Event::Key {
                    key: egui::Key::Enter,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                }],
                true,
            );
            let document = fixture.app.state.model.schematic.document();
            let count = if tool == Tool::Wire {
                document.wires.len()
            } else {
                document.buses.len()
            };
            let can_finish = matches!(
                change,
                "unchanged" | "document round trip" | "passive window"
            );
            assert_eq!(count, usize::from(can_finish), "{tool:?}: {change}");
            assert!(
                !fixture
                    .app
                    .state
                    .model
                    .schematic
                    .session
                    .editor
                    .wire_drawing
                    .active
            );
            assert!(
                !fixture
                    .app
                    .state
                    .model
                    .schematic
                    .session
                    .editor
                    .bus_drawing
                    .active
            );
            assert_eq!(
                fixture.app.state.model.schematic.can_undo(),
                can_finish,
                "{tool:?}: {change}"
            );
            if can_finish {
                assert!(fixture.app.state.model.schematic.undo());
                assert!(
                    fixture
                        .app
                        .state
                        .model
                        .schematic
                        .document()
                        .wires
                        .is_empty()
                );
                assert!(
                    fixture
                        .app
                        .state
                        .model
                        .schematic
                        .document()
                        .buses
                        .is_empty()
                );
                assert!(!fixture.app.state.model.schematic.can_undo());
            }
        }
    }
}

#[test]
fn gesture_new_window_owner_and_owner_window_close_restore_the_old_buffer() {
    use crate::workbench::state::WorkspaceDocumentId;
    for close_owner in [false, true] {
        let mut fixture = Fixture::new(false);
        let original = fixture
            .app
            .state
            .model
            .workspace
            .content
            .active_schematic_reference();
        let second = fixture.second_document();
        let primary = fixture.app.state.workbench.window_session.primary();
        let layout = fixture.app.state.workbench.current_workspace_layout();
        let secondary = fixture
            .app
            .state
            .workbench
            .window_session
            .detach_document(
                WorkspaceDocumentId::CellView(second),
                "Second",
                layout,
                false,
            )
            .unwrap();
        fixture.app.capture_application_window_projection(primary);
        fixture.drag(DragType::MoveSelection);
        fixture.app.state.sync_active_schematic_to_workspace();
        assert!(fixture.app.project_application_window(secondary, layout));
        // Closing an unrelated window must not cancel the primary gesture.
        fixture.app.state.cancel_schematic_drag_in_window(secondary);
        assert!(
            fixture.app.state.model.workspace.content.schematic_buffers[&original.key()]
                .pending_operation_id()
                .is_some()
        );
        if close_owner {
            fixture.app.state.cancel_schematic_drag_in_window(primary);
        } else {
            assert!(fixture.app.state.begin_schematic_drag(
                (0, 0),
                DragType::MoveSelection,
                &fixture.ctx
            ));
            assert!(
                fixture
                    .app
                    .state
                    .schematic_drag_owned_by_context(&fixture.ctx)
            );
        }
        let buffer = &fixture.app.state.model.workspace.content.schematic_buffers[&original.key()];
        assert!(buffer.pending_operation_id().is_none());
        assert_eq!(buffer.document().components[0].pos, Point::new(100, 100));
    }
}
