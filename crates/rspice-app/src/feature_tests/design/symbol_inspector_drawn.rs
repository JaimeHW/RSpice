//! The symbol inspector's fields drawn through a whole application: undo grouping and
//! stale document or selection targets.

use crate::state::{
    Cell, CellViewRef, Library, Point, SymbolDocument, SymbolEditorMetadata, SymbolShape, View,
    ViewType,
};
use crate::workbench::RSpiceApp;
use crate::workbench::design::inspector::symbol::*;
use rspice_schematic_editor::symbol_editor::inspector::{self, SymbolInspectorRequest};
use rspice_schematic_editor::symbol_editor::interaction::SymbolEditCapabilities;
use rspice_ui_kit::panels::panel_body;
use std::collections::HashMap;

#[test]
fn symbol_field_typing_survives_passive_window_projection() {
    use crate::workbench::state::WorkspaceDocumentId;

    let mut editor = Editor::new();
    let before = editor
        .app
        .state
        .model
        .load_active_symbol_document()
        .unwrap();
    let symbol = editor.app.state.model.workspace.content.active_view.clone();
    let schematic = CellViewRef::new("work", "amp", "schematic");
    let primary = editor.app.state.workbench.window_session.primary();
    let layout = editor.app.state.workbench.current_workspace_layout();
    let secondary = editor
        .app
        .state
        .workbench
        .window_session
        .detach_document(
            WorkspaceDocumentId::CellView(schematic.clone()),
            "Other",
            layout,
            false,
        )
        .unwrap();
    editor.pass(Vec::new());
    editor.click("Center X");
    editor.replace("1");
    editor
        .app
        .state
        .workbench
        .window_session
        .set_current(secondary)
        .unwrap();
    editor.app.state.open_workspace_view(schematic);
    editor
        .app
        .state
        .workbench
        .window_session
        .set_current(primary)
        .unwrap();
    editor.app.state.open_workspace_view(symbol.clone());
    editor.pass(vec![egui::Event::Text("2".into())]);
    let after = editor
        .app
        .state
        .model
        .load_active_symbol_document()
        .unwrap();
    assert!(matches!(&after.body[0], SymbolShape::Circle { center, .. } if center.x == 12));
    assert_eq!(
        editor
            .app
            .state
            .design
            .symbol
            .history
            .undo_depth(&symbol.key()),
        1
    );
    assert!(editor.app.state.undo_active_symbol_document().unwrap());
    assert_eq!(
        editor
            .app
            .state
            .model
            .load_active_symbol_document()
            .unwrap(),
        before
    );
}

#[test]
fn symbol_field_focus_and_unfinished_text_do_not_leak_to_another_document() {
    let mut editor = Editor::new();
    let symbol = editor.app.state.model.workspace.content.active_view.clone();
    let other = CellViewRef::new("work", "other", "symbol");
    let before = editor
        .app
        .state
        .model
        .load_active_symbol_document()
        .unwrap();
    let mut cell = Cell::new("other");
    cell.add_view(View::new("symbol", ViewType::Symbol));
    editor
        .app
        .state
        .model
        .library_manager
        .get_library_mut("work")
        .unwrap()
        .add_cell(cell);
    editor.app.state.open_workspace_view(other.clone());
    editor
        .app
        .state
        .store_active_symbol_editor_bundle(&before, &SymbolEditorMetadata::for_document(&before))
        .unwrap();
    editor.app.state.open_workspace_view(symbol);
    editor.app.state.design.symbol.editor.select_shape(0);
    editor.pass(Vec::new());
    editor.click("Center X");
    editor.replace("-");
    editor.app.state.open_workspace_view(other.clone());
    editor.app.state.design.symbol.editor.select_shape(0);
    editor.pass(vec![egui::Event::Text("5".into())]);
    assert_eq!(
        editor
            .app
            .state
            .model
            .load_active_symbol_document()
            .unwrap(),
        before
    );
    assert_eq!(
        editor
            .app
            .state
            .design
            .symbol
            .history
            .undo_depth(&other.key()),
        0
    );
    editor.click("Center X");
    editor.replace("25");
    let after = editor
        .app
        .state
        .model
        .load_active_symbol_document()
        .unwrap();
    assert!(matches!(&after.body[0], SymbolShape::Circle { center, .. } if center.x == 25));
    assert!(editor.app.state.undo_active_symbol_document().unwrap());
    assert_eq!(
        editor
            .app
            .state
            .model
            .load_active_symbol_document()
            .unwrap(),
        before
    );
}

#[test]
fn symbol_coordinate_typing_is_one_undo_group_until_focus_changes() {
    let mut editor = Editor::new();
    let before = editor
        .app
        .state
        .model
        .load_active_symbol_document()
        .unwrap();
    editor.pass(Vec::new());
    editor.click("Center X");
    editor.replace("1");
    editor.replace("12");
    let key = editor.app.state.model.workspace.content.active_key();
    assert_eq!(editor.app.state.design.symbol.history.undo_depth(&key), 1);
    let after = editor
        .app
        .state
        .model
        .load_active_symbol_document()
        .unwrap();
    assert!(matches!(&after.body[0], SymbolShape::Circle { center, .. } if center.x == 12));
    editor.click("Center Y");
    editor.replace("30");
    assert_eq!(editor.app.state.design.symbol.history.undo_depth(&key), 2);
    let after_y = editor
        .app
        .state
        .model
        .load_active_symbol_document()
        .unwrap();
    editor
        .app
        .state
        .open_workspace_view(CellViewRef::new("work", "amp", "schematic"));
    editor
        .app
        .state
        .open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    editor.pass(Vec::new());
    editor.click("Center Y");
    editor.replace("40");
    assert_eq!(editor.app.state.design.symbol.history.undo_depth(&key), 3);
    assert!(editor.app.state.undo_active_symbol_document().unwrap());
    assert_eq!(
        editor
            .app
            .state
            .model
            .load_active_symbol_document()
            .unwrap(),
        after_y
    );
    assert!(editor.app.state.undo_active_symbol_document().unwrap());
    assert_eq!(
        editor
            .app
            .state
            .model
            .load_active_symbol_document()
            .unwrap(),
        after
    );
    assert!(editor.app.state.undo_active_symbol_document().unwrap());
    assert_eq!(
        editor
            .app
            .state
            .model
            .load_active_symbol_document()
            .unwrap(),
        before
    );
}

#[test]
fn unfinished_symbol_field_does_not_follow_a_different_selection_or_view() {
    let mut editor = Editor::new();
    editor.pass(Vec::new());
    editor.click("Center X");
    editor.replace("-");
    assert!(!editor.app.state.can_undo_active_symbol_document());
    editor.app.state.design.symbol.editor.select_shape(1);
    editor.pass(Vec::new());
    editor.click("Center X");
    editor.replace("140");
    let after = editor
        .app
        .state
        .model
        .load_active_symbol_document()
        .unwrap();
    assert!(matches!(&after.body[0], SymbolShape::Circle { center, .. } if center.x == 0));
    assert!(matches!(&after.body[1], SymbolShape::Circle { center, .. } if center.x == 140));
    assert_eq!(
        editor
            .app
            .state
            .design
            .symbol
            .history
            .undo_depth(&editor.app.state.model.workspace.content.active_key()),
        1
    );

    editor.replace("-");
    editor
        .app
        .state
        .open_workspace_view(CellViewRef::new("work", "amp", "schematic"));
    editor
        .app
        .state
        .open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    editor.pass(Vec::new());
    assert_eq!(
        editor
            .app
            .state
            .model
            .load_active_symbol_document()
            .unwrap(),
        after
    );
}

/// Each row of a selected pin changes that one field and nothing else: its
/// lead length (never below zero), whether its name shows, its style and its
/// side.
#[test]
fn each_pin_row_changes_only_its_own_field() {
    use crate::state::{PortDirection, SymbolPin, SymbolPinSide};
    use rspice_design_model::symbol_pin::SymbolPinStyle;
    let mut editor = Editor::new();
    let mut document = editor
        .app
        .state
        .model
        .load_active_symbol_document()
        .unwrap();
    document.pins.push(
        SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, 0)))
            .with_side(SymbolPinSide::Left),
    );
    editor
        .app
        .state
        .store_active_symbol_editor_bundle(
            &document,
            &SymbolEditorMetadata::for_document(&document),
        )
        .unwrap();
    editor.app.state.design.symbol.editor.select_pin("IN");
    editor.pass(Vec::new());
    let pin = |editor: &Editor| {
        editor
            .app
            .state
            .model
            .load_active_symbol_document()
            .unwrap()
            .pin("IN")
            .cloned()
            .unwrap()
    };
    let mut expected = pin(&editor);

    editor.click("Lead length");
    editor.replace("0");
    expected.length = 0;
    assert_eq!(pin(&editor), expected);
    editor.replace("-5");
    assert_eq!(
        pin(&editor),
        expected,
        "a lead cannot run away from the body"
    );

    editor.click("Show name");
    expected.name_hidden = true;
    assert_eq!(pin(&editor), expected);

    // A menu lays itself out unseen the frame it opens; its choices take
    // clicks from the next.
    editor.click("Style");
    editor.pass(Vec::new());
    editor.click("inverted clock");
    expected.style = SymbolPinStyle::InvertedClock;
    assert_eq!(pin(&editor), expected);

    editor.click("Side");
    editor.pass(Vec::new());
    editor.click("top");
    expected.side = SymbolPinSide::Top;
    assert_eq!(pin(&editor), expected);
}

/// A selected shape's rows each change one measure: a rectangle's Width
/// moves its right side alone and refuses nothing, a line's point row moves
/// that point's one coordinate, Add point puts a point a terminal step past
/// the last, and Closed closes a line that has three.
#[test]
fn shape_geometry_rows_each_change_one_measure() {
    let mut editor = Editor::new();
    let mut document = editor
        .app
        .state
        .model
        .load_active_symbol_document()
        .unwrap();
    document.body = vec![
        SymbolShape::Rectangle {
            min: Point::new(-20, -10),
            max: Point::new(20, 10),
        },
        SymbolShape::Polyline {
            points: vec![Point::origin(), Point::new(30, 0)],
            closed: false,
        },
    ];
    editor
        .app
        .state
        .store_active_symbol_editor_bundle(
            &document,
            &SymbolEditorMetadata::for_document(&document),
        )
        .unwrap();
    editor.app.state.design.symbol.editor.select_shape(0);
    editor.pass(Vec::new());
    let body = |editor: &Editor| {
        editor
            .app
            .state
            .model
            .load_active_symbol_document()
            .unwrap()
            .body
    };
    let widened = SymbolShape::Rectangle {
        min: Point::new(-20, -10),
        max: Point::new(40, 10),
    };

    editor.click("Width");
    editor.replace("60");
    assert_eq!(body(&editor)[0], widened);
    editor.replace("0");
    assert_eq!(body(&editor)[0], widened, "a rectangle keeps some width");

    editor.app.state.design.symbol.editor.select_shape(1);
    editor.pass(Vec::new());
    editor.click("Point 2 Y");
    editor.replace("20");
    let mut points = vec![Point::origin(), Point::new(30, 20)];
    assert_eq!(
        body(&editor)[1],
        SymbolShape::Polyline {
            points: points.clone(),
            closed: false,
        }
    );

    editor.click("Add point");
    points.push(Point::new(40, 20));
    assert_eq!(
        body(&editor)[1],
        SymbolShape::Polyline {
            points: points.clone(),
            closed: false,
        }
    );

    // The new point's row pushes the switch down; a click goes where the
    // switch is drawn now.
    editor.pass(Vec::new());
    editor.click("Closed");
    assert_eq!(
        body(&editor),
        vec![
            widened,
            SymbolShape::Polyline {
                points,
                closed: true,
            }
        ]
    );
}

pub(crate) struct Editor {
    ctx: egui::Context,
    pub(crate) app: RSpiceApp,
    controls: HashMap<String, egui::Rect>,
}

impl Editor {
    pub(crate) fn new() -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        let state = &mut app.state;
        let mut library = Library::new("work");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        cell.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(cell);
        state.model.library_manager.add_library(library);
        state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
        let document = SymbolDocument {
            body: vec![
                SymbolShape::Circle {
                    center: Point::origin(),
                    radius: 10,
                },
                SymbolShape::Circle {
                    center: Point::new(100, 0),
                    radius: 20,
                },
            ],
            ..Default::default()
        };
        state
            .store_active_symbol_editor_bundle(
                &document,
                &SymbolEditorMetadata::for_document(&document),
            )
            .unwrap();
        state.design.symbol.editor.select_shape(0);
        Self {
            ctx,
            app,
            controls: HashMap::new(),
        }
    }

    pub(crate) fn frame(&mut self, events: Vec<egui::Event>) -> SymbolInspectorRequest {
        let source = crate::workbench::app::symbol_editor_request_source(
            &self.app.state.model,
            &self.app.state.workbench,
        );
        rspice_schematic_editor::symbol_editor::interaction::bind_canvas_source(
            &mut self.app.state.design.symbol.editor,
            source,
        );
        let mut request = None;
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(360.0, 800.0),
                )),
                events,
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    panel_body(ui, |ui| {
                        let document = self.app.state.model.load_active_symbol_document().unwrap();
                        let metadata = crate::workbench::app::load_active_symbol_editor_metadata(
                            &self.app.state.model,
                            &document,
                        )
                        .unwrap();
                        request = Some(inspector::show(
                            ui,
                            crate::workbench::app::symbol_editor_request_source(
                                &self.app.state.model,
                                &self.app.state.workbench,
                            ),
                            document,
                            metadata,
                            &[],
                            &self.app.state.design.symbol.editor,
                            SymbolEditCapabilities {
                                edit: !self.app.state.model.schematic_edit_read_only(),
                            },
                        ));
                    });
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
        request.unwrap()
    }

    fn pass(&mut self, events: Vec<egui::Event>) {
        let request = self.frame(events);
        apply_request(&mut self.app.design_cx(), request);
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
            egui::Event::Key {
                key: egui::Key::A,
                physical_key: Some(egui::Key::A),
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND,
            },
            egui::Event::Text(text.to_owned()),
        ]);
    }
}
