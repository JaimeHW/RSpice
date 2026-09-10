//! Pointer transactions through real egui frames, navigation, and persistence.

use super::*;
use crate::state::{ComponentType, Point, SchematicState, Tool, Wire};
use crate::workbench::app_state::DragType;
use crate::workbench::state::Workspace;

struct Fixture {
    ctx: Context,
    app: RSpiceApp,
    origin: egui::Pos2,
    time: f64,
}

impl Fixture {
    fn new(wires: bool) -> Self {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.project_open = true;
        app.state.workbench.activate(Workspace::Design);
        app.state.schematic = SchematicState::default();
        app.state.schematic.needs_fit = false;
        app.state.schematic.needs_drawing_sheet_fit = false;
        if wires {
            app.state.schematic.wires = vec![
                Wire::new(1, vec![Point::new(100, 100), Point::new(200, 100)]),
                Wire::new(2, vec![Point::new(100, 100), Point::new(100, 200)]),
            ];
            app.state.schematic.recalculate_runtime_state();
        } else {
            app.state
                .schematic
                .add_component(ComponentType::Resistor, Point::new(100, 100));
        }
        app.state.schematic.clear_undo_history();
        app.state.schematic.is_dirty = false;
        app.state.sync_active_schematic_to_workspace();
        let mut fixture = Self {
            ctx,
            app,
            origin: egui::Pos2::ZERO,
            time: 0.0,
        };
        fixture.frame(vec![], true);
        fixture
    }

    fn frame(&mut self, events: Vec<egui::Event>, focused: bool) {
        self.time += 0.02;
        let Self {
            ctx,
            app,
            origin,
            time,
        } = self;
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(600.0, 400.0),
                )),
                events,
                focused,
                time: Some(*time),
                ..Default::default()
            },
            |ui| {
                app.state.reconcile_schematic_drag(ui.ctx(), false);
                app.handle_shortcuts(ui.ctx());
                *origin = ui.available_rect_before_wrap().min;
                crate::schematic::view::render_schematic_view(ui, &mut app.state, None);
                app.state.reconcile_schematic_drag(ui.ctx(), true);
            },
        );
    }

    fn button(&self, x: f32, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos: self.origin + egui::vec2(x, 100.0),
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        }
    }

    fn drag(&mut self, expected: DragType) {
        self.frame(
            vec![
                egui::Event::PointerMoved(self.origin + egui::vec2(100.0, 100.0)),
                self.button(100.0, true),
            ],
            true,
        );
        self.frame(
            vec![egui::Event::PointerMoved(
                self.origin + egui::vec2(110.0, 100.0),
            )],
            true,
        );
        self.frame(
            vec![egui::Event::PointerMoved(
                self.origin + egui::vec2(170.0, 100.0),
            )],
            true,
        );
        assert_eq!(self.app.state.dialogs.interaction.drag.drag_type, expected);
        assert!(self.app.state.schematic_drag_in_progress());
    }

    fn second_document(&mut self) -> crate::state::CellViewRef {
        use crate::state::{Cell, CellViewRef, OpenCellView, View, ViewType};
        let original = self.app.state.workspace.active_schematic_reference();
        let reference = CellViewRef::new(&original.library, "second", "schematic");
        let mut cell = Cell::new(&reference.cell);
        cell.add_view(View::new(&reference.view, ViewType::Schematic));
        self.app
            .state
            .library_manager
            .get_library_mut(&reference.library)
            .unwrap()
            .add_cell(cell);
        self.app
            .state
            .workspace
            .open_views
            .push(OpenCellView::new(reference.clone(), ViewType::Schematic));
        self.app
            .state
            .workspace
            .schematic_buffers
            .insert(reference.key(), SchematicState::default());
        reference
    }
}

#[test]
fn gesture_release_commits_selection_and_wire_vertex_moves_once() {
    for wires in [false, true] {
        let mut fixture = Fixture::new(wires);
        let original = fixture.app.state.schematic.clone();
        fixture.drag(if wires {
            DragType::WireVertex
        } else {
            DragType::MoveSelection
        });
        assert!(
            original.components != fixture.app.state.schematic.components
                || original.wires != fixture.app.state.schematic.wires
        );
        fixture.frame(vec![fixture.button(170.0, false)], true);
        assert!(!fixture.app.state.schematic.has_pending_operation());
        assert!(fixture.app.state.schematic.undo());
        assert_eq!(original.components, fixture.app.state.schematic.components);
        assert_eq!(original.wires, fixture.app.state.schematic.wires);
        assert!(!fixture.app.state.schematic.can_undo());
    }
}

#[test]
fn gesture_escape_restores_geometry_and_release_cannot_recommit_it() {
    let mut fixture = Fixture::new(false);
    fixture.drag(DragType::MoveSelection);
    let selection = fixture.app.state.schematic.selection.clone();
    fixture.frame(
        vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
        true,
    );
    assert_eq!(
        fixture.app.state.schematic.components[0].pos,
        Point::new(100, 100)
    );
    assert_eq!(fixture.app.state.schematic.selection, selection);
    fixture.frame(vec![fixture.button(170.0, false)], true);
    assert!(!fixture.app.state.schematic.can_undo());
    assert!(!fixture.app.state.schematic.is_dirty);
    fixture.drag(DragType::MoveSelection);
    fixture.frame(vec![fixture.button(170.0, false)], true);
    assert!(fixture.app.state.schematic.can_undo());
}

#[test]
fn gesture_returning_to_start_preserves_the_original_dirty_state() {
    for was_dirty in [false, true] {
        let mut fixture = Fixture::new(false);
        fixture.app.state.schematic.is_dirty = was_dirty;
        fixture.drag(DragType::MoveSelection);
        fixture.frame(
            vec![egui::Event::PointerMoved(
                fixture.origin + egui::vec2(100.0, 100.0),
            )],
            true,
        );
        fixture.frame(vec![fixture.button(100.0, false)], true);
        assert_eq!(
            fixture.app.state.schematic.components[0].pos,
            Point::new(100, 100)
        );
        assert_eq!(fixture.app.state.schematic.is_dirty, was_dirty);
        assert!(!fixture.app.state.schematic.can_undo());
        assert!(!fixture.app.state.schematic.has_pending_operation());
    }
}

#[test]
fn gesture_focus_loss_pointer_loss_and_touch_cancel_restore_the_baseline() {
    for cause in 0..3 {
        let mut fixture = Fixture::new(false);
        fixture.drag(DragType::MoveSelection);
        let events = match cause {
            0 => vec![],
            1 => vec![egui::Event::PointerGone],
            _ => vec![
                egui::Event::Touch {
                    device_id: egui::TouchDeviceId(1),
                    id: egui::TouchId(1),
                    phase: egui::TouchPhase::Cancel,
                    pos: fixture.origin + egui::vec2(170.0, 100.0),
                    force: None,
                },
                fixture.button(170.0, false),
            ],
        };
        fixture.frame(events, cause != 0);
        assert_eq!(
            fixture.app.state.schematic.components[0].pos,
            Point::new(100, 100)
        );
        assert!(!fixture.app.state.schematic.has_pending_operation());
        assert!(!fixture.app.state.schematic.can_undo());
    }
}

#[test]
fn gesture_tool_and_workspace_changes_cancel_without_starting_a_new_edit() {
    for workspace_change in [false, true] {
        let mut fixture = Fixture::new(false);
        fixture.drag(DragType::MoveSelection);
        if workspace_change {
            fixture.app.state.workbench.activate(Workspace::Results);
        } else {
            fixture.app.state.schematic.arm_tool(Tool::Wire);
        }
        fixture.frame(vec![], true);
        assert_eq!(
            fixture.app.state.schematic.components[0].pos,
            Point::new(100, 100)
        );
        assert!(!fixture.app.state.schematic.has_pending_operation());
        assert!(!fixture.app.state.schematic.wire_drawing.active);
    }
}

#[test]
fn gesture_stale_owner_cannot_cancel_a_new_transaction() {
    let mut fixture = Fixture::new(false);
    fixture.drag(DragType::MoveSelection);
    fixture.app.state.schematic.cancel_operation();
    fixture.app.state.schematic.begin_operation("new edit");
    fixture.app.state.schematic.components[0].value = "2k".to_owned();
    fixture.app.state.cancel_schematic_drag();
    assert!(fixture.app.state.schematic.has_pending_operation());
    assert_eq!(fixture.app.state.schematic.components[0].value, "2k");
    assert!(fixture.app.state.schematic.end_operation());
}

#[test]
fn gesture_project_and_session_snapshots_preserve_committed_geometry() {
    let mut fixture = Fixture::new(false);
    fixture.drag(DragType::MoveSelection);
    fixture.app.state.sync_active_schematic_to_workspace();
    let project =
        crate::workbench::lifecycle::project_lifecycle::snapshot(&fixture.app.state).unwrap();
    let key = fixture.app.state.workspace.active_key();
    assert_eq!(
        project.workspace.schematic_buffers[&key].components[0].pos,
        Point::new(100, 100)
    );
    let json = serde_json::to_string(&fixture.app.state).unwrap();
    let restored: AppState = serde_json::from_str(&json).unwrap();
    assert_eq!(
        restored.workspace.schematic_buffers[&key].components[0].pos,
        Point::new(100, 100)
    );
    assert!(fixture.app.state.schematic_drag_in_progress());
    assert_ne!(
        fixture.app.state.schematic.components[0].pos,
        Point::new(100, 100)
    );
}

#[test]
fn gesture_document_navigation_rolls_back_before_buffering_the_original() {
    let mut fixture = Fixture::new(false);
    let original = fixture.app.state.workspace.active_schematic_reference();
    let second = fixture.second_document();
    fixture.drag(DragType::MoveSelection);
    fixture.app.state.open_workspace_view(second.clone());
    assert_eq!(fixture.app.state.workspace.active_view, second);
    let buffer = &fixture.app.state.workspace.schematic_buffers[&original.key()];
    assert_eq!(buffer.components[0].pos, Point::new(100, 100));
    assert!(!buffer.has_pending_operation());
    fixture.app.state.open_workspace_view(original);
    fixture.frame(vec![fixture.button(170.0, false)], true);
    assert!(!fixture.app.state.schematic.can_undo());
    assert_eq!(
        fixture.app.state.schematic.components[0].pos,
        Point::new(100, 100)
    );
}

#[test]
fn gesture_window_projection_preserves_ownership_and_inactive_save_baselines() {
    use crate::workbench::state::WorkspaceDocumentId;
    let mut fixture = Fixture::new(false);
    let original = fixture.app.state.workspace.active_schematic_reference();
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
    let operation = fixture.app.state.schematic.pending_operation_id();
    fixture.app.state.sync_active_schematic_to_workspace();

    for _ in 0..3 {
        assert!(fixture.app.project_application_window(secondary, layout));
        assert_eq!(fixture.app.state.workspace.active_view, second);
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
            snapshot.workspace.schematic_buffers[&original.key()].components[0].pos,
            Point::new(100, 100)
        );
        let restored: AppState =
            serde_json::from_str(&serde_json::to_string(&fixture.app.state).unwrap()).unwrap();
        assert_eq!(
            restored.workspace.schematic_buffers[&original.key()].components[0].pos,
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
            fixture.app.state.schematic.pending_operation_id(),
            operation
        );
        assert_ne!(
            fixture.app.state.schematic.components[0].pos,
            Point::new(100, 100)
        );
    }
    fixture.frame(vec![fixture.button(170.0, false)], true);
    assert!(fixture.app.state.schematic.undo());
    assert_eq!(
        fixture.app.state.schematic.components[0].pos,
        Point::new(100, 100)
    );
    assert!(!fixture.app.state.schematic.can_undo());
}

#[test]
fn gesture_new_window_owner_and_owner_window_close_restore_the_old_buffer() {
    use crate::workbench::state::WorkspaceDocumentId;
    for close_owner in [false, true] {
        let mut fixture = Fixture::new(false);
        let original = fixture.app.state.workspace.active_schematic_reference();
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
            fixture.app.state.workspace.schematic_buffers[&original.key()].has_pending_operation()
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
        let buffer = &fixture.app.state.workspace.schematic_buffers[&original.key()];
        assert!(!buffer.has_pending_operation());
        assert_eq!(buffer.components[0].pos, Point::new(100, 100));
    }
}
