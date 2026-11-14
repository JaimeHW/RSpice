//! Rendered symbol fields preserve undo grouping and reject stale document or selection targets.

use super::*;
use crate::state::{
    Cell, CellViewRef, Library, Point, SymbolDocument, SymbolEditorMetadata, SymbolShape, View,
    ViewType,
};
use crate::workbench::RSpiceApp;
use rspice_ui_kit::panels::panel_body;
use std::collections::HashMap;

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
