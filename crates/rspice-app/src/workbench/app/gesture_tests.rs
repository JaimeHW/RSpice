//! Pointer transactions through real egui frames, navigation, and persistence.

use super::*;
use crate::state::{ComponentType, Point, SchematicState, Tool, Wire};
use crate::workbench::app_state::DragType;
use crate::workbench::state::Workspace;

pub(crate) struct Fixture {
    pub(crate) ctx: Context,
    pub(crate) app: RSpiceApp,
    pub(crate) origin: egui::Pos2,
    time: f64,
}

impl Fixture {
    pub(crate) fn new(wires: bool) -> Self {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        app.state.open_project_session();
        app.state.workbench.activate(Workspace::Design);
        app.state.model.schematic = SchematicState::default();
        app.state.model.schematic.session.editor.needs_fit = false;
        app.state
            .model
            .schematic
            .session
            .editor
            .needs_drawing_sheet_fit = false;
        if wires {
            app.state.model.schematic.document_mut_for_test().wires = vec![
                Wire::new(1, vec![Point::new(100, 100), Point::new(200, 100)]),
                Wire::new(2, vec![Point::new(100, 100), Point::new(100, 200)]),
            ];
            app.state.model.schematic.recalculate_runtime_state();
        } else {
            app.state
                .model
                .schematic
                .add_component(ComponentType::Resistor, Point::new(100, 100));
        }
        app.state.model.schematic.clear_undo_history();
        app.state.model.schematic.session.is_dirty = false;
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

    pub(crate) fn frame(&mut self, events: Vec<egui::Event>, focused: bool) {
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
                if app.state.model.workspace.content.active_view_type()
                    == crate::state::ViewType::Symbol
                {
                    crate::schematic::symbol_editor::show(
                        ui,
                        &mut crate::schematic::view::canvas_frame(&mut app.state),
                        None,
                    );
                } else {
                    crate::schematic::view::render_schematic_view(
                        ui,
                        &mut crate::schematic::view::canvas_frame(&mut app.state),
                        None,
                    );
                }
                app.state.reconcile_schematic_drag(ui.ctx(), true);
            },
        );
    }

    pub(crate) fn button(&self, x: f32, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos: self.origin + egui::vec2(x, 100.0),
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        }
    }

    pub(crate) fn drag(&mut self, expected: DragType) {
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
        assert_eq!(self.app.state.design.interaction.drag.drag_type, expected);
        assert!(self.app.state.schematic_drag_in_progress());
    }

    pub(crate) fn second_document(&mut self) -> crate::state::CellViewRef {
        use crate::state::{Cell, CellViewRef, OpenCellView, View, ViewType};
        let original = self
            .app
            .state
            .model
            .workspace
            .content
            .active_schematic_reference();
        let reference = CellViewRef::new(&original.library, "second", "schematic");
        let mut cell = Cell::new(&reference.cell);
        cell.add_view(View::new(&reference.view, ViewType::Schematic));
        self.app
            .state
            .model
            .library_manager
            .get_library_mut(&reference.library)
            .unwrap()
            .add_cell(cell);
        self.app
            .state
            .model
            .workspace
            .content
            .open_views
            .push(OpenCellView::new(reference.clone(), ViewType::Schematic));
        self.app
            .state
            .model
            .workspace
            .insert_schematic_editor(reference.key(), SchematicState::default());
        reference
    }
    pub(crate) fn enter_reused_master(&mut self) -> crate::state::CellViewRef {
        use crate::state::LibraryCellInstance;
        let master = self.second_document();
        let child = self.app.state.model.schematic.clone();
        self.app.state.model.schematic = SchematicState::default();
        for x in [0, 200] {
            self.app.state.model.schematic.add_library_cell_component(
                Point::new(x, 0),
                LibraryCellInstance::new(&master.library, &master.cell, &master.view),
            );
        }
        self.app.state.sync_active_schematic_to_workspace();
        self.app
            .state
            .model
            .workspace
            .insert_schematic_editor(master.key(), child);
        self.app
            .state
            .navigation()
            .descend_into_instance(Some("X1".into()), master.clone());
        master
    }
}

#[test]
fn gesture_release_commits_selection_and_wire_vertex_moves_once() {
    for wires in [false, true] {
        let mut fixture = Fixture::new(wires);
        let original = fixture.app.state.model.schematic.clone();
        fixture.drag(if wires {
            DragType::WireVertex
        } else {
            DragType::MoveSelection
        });
        assert!(
            original.document().components
                != fixture.app.state.model.schematic.document().components
                || original.document().wires != fixture.app.state.model.schematic.document().wires
        );
        fixture.frame(vec![fixture.button(170.0, false)], true);
        assert!(!fixture.app.state.model.schematic.has_pending_operation());
        assert!(fixture.app.state.model.schematic.undo());
        assert_eq!(
            original.document().components,
            fixture.app.state.model.schematic.document().components
        );
        assert_eq!(
            original.document().wires,
            fixture.app.state.model.schematic.document().wires
        );
        assert!(!fixture.app.state.model.schematic.can_undo());
    }
}

#[test]
fn gesture_escape_restores_geometry_and_release_cannot_recommit_it() {
    let mut fixture = Fixture::new(false);
    fixture.drag(DragType::MoveSelection);
    let selection = fixture
        .app
        .state
        .model
        .schematic
        .session
        .editor
        .selection
        .clone();
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
        fixture.app.state.model.schematic.document().components[0].pos,
        Point::new(100, 100)
    );
    assert_eq!(
        fixture.app.state.model.schematic.session.editor.selection,
        selection
    );
    fixture.frame(vec![fixture.button(170.0, false)], true);
    assert!(!fixture.app.state.model.schematic.can_undo());
    assert!(!fixture.app.state.model.schematic.session.is_dirty);
    fixture.drag(DragType::MoveSelection);
    fixture.frame(vec![fixture.button(170.0, false)], true);
    assert!(fixture.app.state.model.schematic.can_undo());
}

#[test]
fn gesture_returning_to_start_preserves_the_original_dirty_state() {
    for was_dirty in [false, true] {
        let mut fixture = Fixture::new(false);
        fixture.app.state.model.schematic.session.is_dirty = was_dirty;
        fixture.drag(DragType::MoveSelection);
        fixture.frame(
            vec![egui::Event::PointerMoved(
                fixture.origin + egui::vec2(100.0, 100.0),
            )],
            true,
        );
        fixture.frame(vec![fixture.button(100.0, false)], true);
        assert_eq!(
            fixture.app.state.model.schematic.document().components[0].pos,
            Point::new(100, 100)
        );
        assert_eq!(
            fixture.app.state.model.schematic.session.is_dirty,
            was_dirty
        );
        assert!(!fixture.app.state.model.schematic.can_undo());
        assert!(!fixture.app.state.model.schematic.has_pending_operation());
    }
}

/// The press copies the drag's undo baseline before the drag begins. An edit
/// made while the button is held moves the document on from that copy, so
/// the drag captures its own and undoing it keeps the edit.
#[test]
fn an_edit_while_the_button_is_held_survives_undoing_the_drag() {
    let mut fixture = Fixture::new(false);
    fixture.frame(
        vec![
            egui::Event::PointerMoved(fixture.origin + egui::vec2(100.0, 100.0)),
            fixture.button(100.0, true),
        ],
        true,
    );
    let add_wire = |schematic: &mut SchematicState| {
        schematic.document_mut_for_test().wires.push(Wire::new(
            9,
            vec![Point::new(300, 300), Point::new(340, 300)],
        ));
    };
    assert!(
        fixture
            .app
            .state
            .model
            .schematic
            .with_undo("add wire", add_wire)
    );
    let edited = fixture.app.state.model.schematic.document().wires.clone();
    for x in [110.0, 170.0] {
        fixture.frame(
            vec![egui::Event::PointerMoved(
                fixture.origin + egui::vec2(x, 100.0),
            )],
            true,
        );
    }
    assert!(fixture.app.state.schematic_drag_in_progress());
    fixture.frame(vec![fixture.button(170.0, false)], true);
    assert_ne!(
        fixture.app.state.model.schematic.document().components[0].pos,
        Point::new(100, 100)
    );

    assert!(fixture.app.state.model.schematic.undo());
    assert_eq!(
        fixture.app.state.model.schematic.document().components[0].pos,
        Point::new(100, 100)
    );
    assert_eq!(fixture.app.state.model.schematic.document().wires, edited);
    assert!(fixture.app.state.model.schematic.undo());
    assert!(!fixture.app.state.model.schematic.can_undo());
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
            fixture.app.state.model.schematic.document().components[0].pos,
            Point::new(100, 100)
        );
        assert!(!fixture.app.state.model.schematic.has_pending_operation());
        assert!(!fixture.app.state.model.schematic.can_undo());
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
            fixture.app.state.model.schematic.arm_tool(Tool::Wire);
        }
        fixture.frame(vec![], true);
        assert_eq!(
            fixture.app.state.model.schematic.document().components[0].pos,
            Point::new(100, 100)
        );
        assert!(!fixture.app.state.model.schematic.has_pending_operation());
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
    }
}

#[test]
fn gesture_stale_owner_cannot_cancel_a_new_transaction() {
    let mut fixture = Fixture::new(false);
    fixture.drag(DragType::MoveSelection);
    fixture.app.state.model.schematic.cancel_operation();
    fixture
        .app
        .state
        .model
        .schematic
        .begin_operation("new edit");
    fixture
        .app
        .state
        .model
        .schematic
        .document_mut_for_test()
        .components[0]
        .value = "2k".to_owned();
    fixture.app.state.cancel_schematic_drag();
    assert!(fixture.app.state.model.schematic.has_pending_operation());
    assert_eq!(
        fixture.app.state.model.schematic.document().components[0].value,
        "2k"
    );
    assert!(fixture.app.state.model.schematic.end_operation());
}

#[test]
fn gesture_project_and_session_snapshots_preserve_committed_geometry() {
    let mut fixture = Fixture::new(false);
    fixture.drag(DragType::MoveSelection);
    fixture.app.state.sync_active_schematic_to_workspace();
    let project =
        crate::workbench::lifecycle::project_lifecycle::snapshot(&fixture.app.state).unwrap();
    let key = fixture.app.state.model.workspace.content.active_key();
    assert_eq!(
        project.file.workspace.schematic_buffers[&key]
            .document()
            .components[0]
            .pos,
        Point::new(100, 100)
    );
    let json = serde_json::to_string(&fixture.app.state).unwrap();
    let restored: AppState = serde_json::from_str(&json).unwrap();
    assert_eq!(
        restored.model.workspace.content.schematic_buffers[&key]
            .document()
            .components[0]
            .pos,
        Point::new(100, 100)
    );
    assert!(fixture.app.state.schematic_drag_in_progress());
    assert_ne!(
        fixture.app.state.model.schematic.document().components[0].pos,
        Point::new(100, 100)
    );
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn gesture_native_save_acceptance_rebases_cancel_and_release_dirty_state() {
    use crate::workbench::lifecycle::project_lifecycle::{
        DestinationAuthority, SaveScope, has_unsaved_changes, save_native,
    };
    for dirty_before_drag in [false, true] {
        for completion in 0..3 {
            let mut fixture = Fixture::new(false);
            let path = std::env::temp_dir().join(format!(
                "rspice-gesture-save-{}.rspiceproj",
                uuid::Uuid::new_v4(),
            ));
            save_native(
                &mut fixture.app.state,
                SaveScope::AllDocuments,
                &path,
                DestinationAuthority::UserSelected,
            )
            .unwrap();
            if dirty_before_drag {
                fixture
                    .app
                    .state
                    .model
                    .schematic
                    .with_undo("change resistor", |schematic| {
                        schematic.document_mut_for_test().components[0].value = "2k".to_owned();
                    });
            }
            fixture.drag(DragType::MoveSelection);
            save_native(
                &mut fixture.app.state,
                SaveScope::AllDocuments,
                &path,
                DestinationAuthority::UserSelected,
            )
            .unwrap();
            let saved = crate::io::load_project_file(&path).unwrap();
            let key = fixture.app.state.model.workspace.content.active_key();
            assert_eq!(
                saved.file.workspace.schematic_buffers[&key]
                    .document()
                    .components[0]
                    .pos,
                Point::new(100, 100)
            );
            assert!(fixture.app.state.schematic_drag_in_progress());
            assert!(has_unsaved_changes(&fixture.app.state));
            match completion {
                0 => {
                    fixture.app.state.cancel_schematic_drag();
                }
                1 => fixture.frame(
                    vec![egui::Event::PointerMoved(
                        fixture.origin + egui::vec2(100.0, 100.0),
                    )],
                    true,
                ),
                _ => {}
            }
            fixture.frame(
                vec![fixture.button(if completion == 1 { 100.0 } else { 170.0 }, false)],
                true,
            );
            assert_eq!(has_unsaved_changes(&fixture.app.state), completion == 2);
            assert_eq!(
                fixture.app.state.model.schematic.session.is_dirty,
                completion == 2,
                "dirty before drag={dirty_before_drag}, completion={completion}"
            );
            assert!(!fixture.app.state.model.schematic.has_pending_operation());
            for suffix in ["", ".bak", ".rspice.lock"] {
                let mut artifact = path.as_os_str().to_os_string();
                artifact.push(suffix);
                let _ = std::fs::remove_file(std::path::PathBuf::from(artifact));
            }
        }
    }
}

#[test]
fn gesture_previews_remain_part_of_execution_and_replacement_guards() {
    use crate::workbench::lifecycle::project_lifecycle::{
        begin_project_replacement, generated_netlist_input_digest, validate_project_replacement,
    };
    let mut fixture = Fixture::new(false);
    let baseline = generated_netlist_input_digest(&fixture.app.state).unwrap();
    let replacement = begin_project_replacement(&mut fixture.app.state).unwrap();
    fixture.drag(DragType::MoveSelection);
    assert_ne!(
        generated_netlist_input_digest(&fixture.app.state).unwrap(),
        baseline
    );
    assert!(validate_project_replacement(&fixture.app.state, replacement).is_err());
    fixture.app.state.cancel_schematic_drag();
    assert_eq!(
        generated_netlist_input_digest(&fixture.app.state).unwrap(),
        baseline
    );
    assert!(validate_project_replacement(&fixture.app.state, replacement).is_ok());
}

#[test]
fn gesture_document_navigation_rolls_back_before_buffering_the_original() {
    let mut fixture = Fixture::new(false);
    let original = fixture
        .app
        .state
        .model
        .workspace
        .content
        .active_schematic_reference();
    let second = fixture.second_document();
    fixture.drag(DragType::MoveSelection);
    fixture.app.state.open_workspace_view(second.clone());
    assert_eq!(
        fixture.app.state.model.workspace.content.active_view,
        second
    );
    let buffer = &fixture.app.state.model.workspace.content.schematic_buffers[&original.key()];
    assert_eq!(buffer.document().components[0].pos, Point::new(100, 100));
    assert!(buffer.pending_operation_id().is_none());
    fixture.app.state.open_workspace_view(original);
    fixture.frame(vec![fixture.button(170.0, false)], true);
    assert!(!fixture.app.state.model.schematic.can_undo());
    assert_eq!(
        fixture.app.state.model.schematic.document().components[0].pos,
        Point::new(100, 100)
    );
}

#[test]
fn gesture_occurrence_and_sheet_changes_cancel_the_original_transaction() {
    use crate::state::{SheetDefinition, SheetPortPolicy, SheetTemplate, ViewType};
    for occurrence_change in [false, true] {
        let mut fixture = Fixture::new(false);
        let master = fixture.enter_reused_master();
        let id = fixture.app.state.model.schematic.document().components[0].id;
        let first = fixture
            .app
            .state
            .model
            .workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&master.key(), "Sheet 1", [id])
            .unwrap();
        let catalog = fixture
            .app
            .state
            .model
            .workspace
            .content
            .design_management
            .sheet_catalog_mut(&master.key())
            .unwrap();
        let second = catalog
            .create_sheet(
                SheetDefinition {
                    name: "Sheet 2".into(),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(2),
                },
                Some(first),
            )
            .unwrap();
        catalog.set_active(first).unwrap();
        fixture.drag(DragType::MoveSelection);
        if occurrence_change {
            fixture.app.state.model.workspace.ascend_one().unwrap();
            fixture.app.state.model.workspace.descend_into(
                "X2".into(),
                master,
                ViewType::Schematic,
            );
        } else {
            fixture
                .app
                .state
                .model
                .workspace
                .content
                .design_management
                .sheet_catalog_mut(&master.key())
                .unwrap()
                .set_active(second)
                .unwrap();
        }
        assert!(!fixture.app.state.schematic_drag_in_progress());
        fixture.frame(vec![fixture.button(170.0, false)], true);
        assert_eq!(
            fixture.app.state.model.schematic.document().components[0].pos,
            Point::new(100, 100)
        );
        assert!(!fixture.app.state.model.schematic.has_pending_operation());
        assert!(!fixture.app.state.model.schematic.can_undo());
    }
}

/// A sheet switch that interrupts a drag lets go of it before it writes the
/// drawing back: the buffer of the sheet left holds the geometry it had
/// before the press, and nothing is left to undo.
#[test]
fn switching_sheets_mid_drag_leaves_the_first_sheets_buffer_as_before_the_drag() {
    use crate::state::{SheetDefinition, SheetPortPolicy, SheetTemplate};
    let mut fixture = Fixture::new(false);
    let key = fixture
        .app
        .state
        .model
        .workspace
        .content
        .active_schematic_reference()
        .key();
    let id = fixture.app.state.model.schematic.document().components[0].id;
    let designs = &mut fixture.app.state.model.workspace.content.design_management;
    let first = designs
        .bootstrap_for_cell_view(&key, "Sheet 1", [id])
        .unwrap();
    let catalog = designs.sheet_catalog_mut(&key).unwrap();
    let second = catalog
        .create_sheet(
            SheetDefinition {
                name: "Sheet 2".into(),
                template: SheetTemplate::AnalogSchematic,
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(first),
        )
        .unwrap();
    catalog.set_active(first).unwrap();
    fixture.drag(DragType::MoveSelection);

    let state = &mut fixture.app.state;
    crate::workbench::app::sheets::activate_sheet(
        &mut state.model,
        &mut state.workbench,
        &mut *state.design,
        second,
    )
    .expect("the switch publishes");

    assert!(!state.schematic_drag_in_progress());
    assert!(!state.model.schematic.has_pending_operation());
    assert!(!state.model.schematic.can_undo());
    let buffer = state.model.workspace.schematic_editor(&key).unwrap();
    assert_eq!(buffer.document().components[0].pos, Point::new(100, 100));
}

fn key(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }
}

/// Copy the fixture's resistor and Paste it, with the pointer over the
/// canvas at screen `(x, 100)`.
fn float_a_pasted_resistor(fixture: &mut Fixture, x: f32) {
    use crate::workbench::commands::vocabulary::Command;
    let id = fixture.app.state.model.schematic.document().components[0].id;
    fixture
        .app
        .state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(id);
    Command::Copy.execute(&mut fixture.app);
    fixture.frame(
        vec![egui::Event::PointerMoved(
            fixture.origin + egui::vec2(x, 100.0),
        )],
        true,
    );
    Command::Paste.execute(&mut fixture.app);
}

/// Paste floats the copy under the pointer: nothing lands until the click,
/// R turns the ghost, history waits while it floats, and the click places
/// it as one undo step where the pointer put it (audit G-1).
#[test]
fn a_paste_follows_the_pointer_turns_with_r_and_lands_on_the_click() {
    use crate::state::Rotation;
    use crate::workbench::commands::vocabulary::Command;

    let mut fixture = Fixture::new(false);
    float_a_pasted_resistor(&mut fixture, 300.0);
    assert_eq!(
        fixture
            .app
            .state
            .model
            .schematic
            .document()
            .components
            .len(),
        1
    );
    let start = fixture
        .app
        .state
        .design
        .floating_paste
        .as_ref()
        .expect("the paste floats")
        .at();
    assert!(!Command::Undo.is_enabled(&fixture.app));

    fixture.frame(vec![key(egui::Key::R)], true);
    fixture.frame(
        vec![egui::Event::PointerMoved(
            fixture.origin + egui::vec2(400.0, 100.0),
        )],
        true,
    );
    let floating = fixture
        .app
        .state
        .design
        .floating_paste
        .as_ref()
        .expect("still floating");
    let at = floating.at();
    assert!(
        at.x > start.x && at.y == start.y,
        "the copy did not follow: {start:?} -> {at:?}"
    );
    assert_eq!(floating.objects.components[0].rotation, Rotation::R90);
    assert_eq!(
        fixture
            .app
            .state
            .model
            .schematic
            .document()
            .components
            .len(),
        1
    );

    fixture.frame(vec![fixture.button(400.0, true)], true);
    fixture.frame(vec![fixture.button(400.0, false)], true);

    assert!(fixture.app.state.design.floating_paste.is_none());
    let document = fixture.app.state.model.schematic.document();
    assert_eq!(document.components.len(), 2, "the click placed the copy");
    let pasted = &document.components[1];
    assert_eq!(pasted.rotation, Rotation::R90);
    let grid = document.grid_size;
    assert!(pasted.pos.x % grid == 0 && pasted.pos.y % grid == 0);
    assert!(Command::Undo.is_enabled(&fixture.app));
    assert!(
        fixture.app.state.model.schematic.undo(),
        "the paste is one undo step"
    );
    assert_eq!(
        fixture
            .app
            .state
            .model
            .schematic
            .document()
            .components
            .len(),
        1
    );
    assert!(!fixture.app.state.model.schematic.can_undo());
}

/// Carried to the canvas edge, a floating copy pans the view and keeps
/// following the pointer; with nothing carried, the same pointer pans
/// nothing (audit G-5).
#[test]
fn a_copy_carried_to_the_canvas_edge_pans_the_view() {
    let mut fixture = Fixture::new(false);
    let near_right_edge = fixture.origin + egui::vec2(590.0, 100.0);
    let pan_before = fixture.app.state.model.schematic.session.editor.pan;
    for _ in 0..3 {
        fixture.frame(vec![egui::Event::PointerMoved(near_right_edge)], true);
    }
    assert_eq!(
        fixture.app.state.model.schematic.session.editor.pan, pan_before,
        "an idle pointer at the edge moved the view"
    );

    float_a_pasted_resistor(&mut fixture, 300.0);
    fixture.frame(vec![egui::Event::PointerMoved(near_right_edge)], true);
    let first = fixture.app.state.model.schematic.session.editor.pan;
    let at_first = fixture
        .app
        .state
        .design
        .floating_paste
        .as_ref()
        .expect("floating")
        .at();
    for _ in 0..5 {
        fixture.frame(vec![], true);
    }
    let later = fixture.app.state.model.schematic.session.editor.pan;
    let at_later = fixture
        .app
        .state
        .design
        .floating_paste
        .as_ref()
        .expect("floating")
        .at();
    assert!(
        later.0 < first.0,
        "the view did not pan right: {first:?} -> {later:?}"
    );
    assert_eq!(later.1, first.1);
    assert!(
        at_later.x > at_first.x,
        "the copy did not ride the pan: {at_first:?} -> {at_later:?}"
    );
}

/// Esc lets a floating paste go and the drawing never changed.
#[test]
fn escape_lets_a_floating_paste_go() {
    let mut fixture = Fixture::new(false);
    let before =
        crate::state::SchematicSnapshot::capture(fixture.app.state.model.schematic.document());
    float_a_pasted_resistor(&mut fixture, 300.0);
    assert!(fixture.app.state.design.floating_paste.is_some());

    fixture.frame(vec![key(egui::Key::Escape)], true);

    assert!(fixture.app.state.design.floating_paste.is_none());
    assert!(before.is_equal_document(fixture.app.state.model.schematic.document()));
    assert!(!fixture.app.state.model.schematic.can_undo());
    fixture.frame(vec![fixture.button(400.0, true)], true);
    fixture.frame(vec![fixture.button(400.0, false)], true);
    assert_eq!(
        fixture
            .app
            .state
            .model
            .schematic
            .document()
            .components
            .len(),
        1
    );
}

/// Esc drops the tool and the canvas keeps the keyboard, so the next bare
/// key arms its tool. egui lets go of a focused widget on Escape unless the
/// widget claims the key, and the canvas's single keys act only while it
/// holds focus: after Esc, W did nothing until the canvas was clicked again.
#[test]
fn after_escape_the_canvas_still_takes_its_keys() {
    let mut fixture = Fixture::new(false);
    let empty = fixture.origin + egui::vec2(400.0, 100.0);
    fixture.frame(vec![egui::Event::PointerMoved(empty)], true);
    fixture.frame(vec![fixture.button(400.0, true)], true);
    fixture.frame(vec![fixture.button(400.0, false)], true);
    let tool = |fixture: &Fixture| fixture.app.state.model.schematic.session.editor.tool;
    let tap = |fixture: &mut Fixture, pressed: egui::Key| {
        fixture.frame(vec![key(pressed)], true);
        fixture.frame(
            vec![egui::Event::Key {
                key: pressed,
                physical_key: Some(pressed),
                pressed: false,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            true,
        );
    };

    tap(&mut fixture, egui::Key::W);
    assert_eq!(tool(&fixture), Tool::Wire);
    tap(&mut fixture, egui::Key::Escape);
    assert_eq!(tool(&fixture), Tool::Select, "Esc drops the tool");
    tap(&mut fixture, egui::Key::W);
    assert_eq!(tool(&fixture), Tool::Wire, "W after Esc arms the wire tool");
}
