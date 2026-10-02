//! Rendered symbol fields preserve undo grouping and reject stale document or selection targets.

use super::*;
use crate::state::{
    Cell, CellViewRef, Library, Point, SymbolDocument, SymbolEditorMetadata, SymbolShape, View,
    ViewType,
};
use rspice_ui_kit::panels::inspector::{begin_inspector_sections, finish_inspector_sections};
use std::collections::HashMap;

struct Editor {
    ctx: egui::Context,
    state: AppState,
    controls: HashMap<String, egui::Rect>,
}

impl Editor {
    fn new() -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut state = AppState::default();
        let mut library = Library::new("work");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        cell.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(cell);
        state.library_manager.add_library(library);
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
        state.ui.symbol.editor.select_shape(0);
        Self {
            ctx,
            state,
            controls: HashMap::new(),
        }
    }

    fn frame(&mut self, events: Vec<egui::Event>) -> SymbolInspectorRequest {
        let source = self.state.symbol_editor_request_source();
        rspice_schematic_editor::symbol_editor::interaction::bind_canvas_source(
            &mut self.state.ui.symbol.editor,
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
                    ui.spacing_mut().item_spacing.y = 0.0;
                    begin_inspector_sections(ui);
                    let document = self.state.load_active_symbol_document().unwrap();
                    let metadata = self
                        .state
                        .load_active_symbol_editor_metadata(&document)
                        .unwrap();
                    request = Some(inspector::show(
                        ui,
                        self.state.symbol_editor_request_source(),
                        document,
                        metadata,
                        &[],
                        &self.state.ui.symbol.editor,
                        SymbolEditCapabilities {
                            edit: !self.state.schematic_edit_read_only(),
                        },
                    ));
                    finish_inspector_sections(ui);
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
        apply_request(&mut self.state, request);
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
    let before = editor.state.load_active_symbol_document().unwrap();
    let symbol = editor.state.workspace.content.active_view.clone();
    let schematic = CellViewRef::new("work", "amp", "schematic");
    let primary = editor.state.workbench.window_session.primary();
    let layout = editor.state.workbench.current_workspace_layout();
    let secondary = editor
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
        .state
        .workbench
        .window_session
        .set_current(secondary)
        .unwrap();
    editor.state.open_workspace_view(schematic);
    editor
        .state
        .workbench
        .window_session
        .set_current(primary)
        .unwrap();
    editor.state.open_workspace_view(symbol.clone());
    editor.pass(vec![egui::Event::Text("2".into())]);
    let after = editor.state.load_active_symbol_document().unwrap();
    assert!(matches!(&after.body[0], SymbolShape::Circle { center, .. } if center.x == 12));
    assert_eq!(editor.state.ui.symbol.history.undo_depth(&symbol.key()), 1);
    assert!(editor.state.undo_active_symbol_document().unwrap());
    assert_eq!(editor.state.load_active_symbol_document().unwrap(), before);
}

#[test]
fn symbol_field_focus_and_unfinished_text_do_not_leak_to_another_document() {
    let mut editor = Editor::new();
    let symbol = editor.state.workspace.content.active_view.clone();
    let other = CellViewRef::new("work", "other", "symbol");
    let before = editor.state.load_active_symbol_document().unwrap();
    let mut cell = Cell::new("other");
    cell.add_view(View::new("symbol", ViewType::Symbol));
    editor
        .state
        .library_manager
        .get_library_mut("work")
        .unwrap()
        .add_cell(cell);
    editor.state.open_workspace_view(other.clone());
    editor
        .state
        .store_active_symbol_editor_bundle(&before, &SymbolEditorMetadata::for_document(&before))
        .unwrap();
    editor.state.open_workspace_view(symbol);
    editor.state.ui.symbol.editor.select_shape(0);
    editor.pass(Vec::new());
    editor.click("Center X");
    editor.replace("-");
    editor.state.open_workspace_view(other.clone());
    editor.state.ui.symbol.editor.select_shape(0);
    editor.pass(vec![egui::Event::Text("5".into())]);
    assert_eq!(editor.state.load_active_symbol_document().unwrap(), before);
    assert_eq!(editor.state.ui.symbol.history.undo_depth(&other.key()), 0);
    editor.click("Center X");
    editor.replace("25");
    let after = editor.state.load_active_symbol_document().unwrap();
    assert!(matches!(&after.body[0], SymbolShape::Circle { center, .. } if center.x == 25));
    assert!(editor.state.undo_active_symbol_document().unwrap());
    assert_eq!(editor.state.load_active_symbol_document().unwrap(), before);
}

#[test]
fn symbol_coordinate_typing_is_one_undo_group_until_focus_changes() {
    let mut editor = Editor::new();
    let before = editor.state.load_active_symbol_document().unwrap();
    editor.pass(Vec::new());
    editor.click("Center X");
    editor.replace("1");
    editor.replace("12");
    let key = editor.state.workspace.content.active_key();
    assert_eq!(editor.state.ui.symbol.history.undo_depth(&key), 1);
    let after = editor.state.load_active_symbol_document().unwrap();
    assert!(matches!(&after.body[0], SymbolShape::Circle { center, .. } if center.x == 12));
    editor.click("Center Y");
    editor.replace("30");
    assert_eq!(editor.state.ui.symbol.history.undo_depth(&key), 2);
    let after_y = editor.state.load_active_symbol_document().unwrap();
    editor
        .state
        .open_workspace_view(CellViewRef::new("work", "amp", "schematic"));
    editor
        .state
        .open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    editor.pass(Vec::new());
    editor.click("Center Y");
    editor.replace("40");
    assert_eq!(editor.state.ui.symbol.history.undo_depth(&key), 3);
    assert!(editor.state.undo_active_symbol_document().unwrap());
    assert_eq!(editor.state.load_active_symbol_document().unwrap(), after_y);
    assert!(editor.state.undo_active_symbol_document().unwrap());
    assert_eq!(editor.state.load_active_symbol_document().unwrap(), after);
    assert!(editor.state.undo_active_symbol_document().unwrap());
    assert_eq!(editor.state.load_active_symbol_document().unwrap(), before);
}

#[test]
fn unfinished_symbol_field_does_not_follow_a_different_selection_or_view() {
    let mut editor = Editor::new();
    editor.pass(Vec::new());
    editor.click("Center X");
    editor.replace("-");
    assert!(!editor.state.can_undo_active_symbol_document());
    editor.state.ui.symbol.editor.select_shape(1);
    editor.pass(Vec::new());
    editor.click("Center X");
    editor.replace("140");
    let after = editor.state.load_active_symbol_document().unwrap();
    assert!(matches!(&after.body[0], SymbolShape::Circle { center, .. } if center.x == 0));
    assert!(matches!(&after.body[1], SymbolShape::Circle { center, .. } if center.x == 140));
    assert_eq!(
        editor
            .state
            .ui
            .symbol
            .history
            .undo_depth(&editor.state.workspace.content.active_key()),
        1
    );

    editor.replace("-");
    editor
        .state
        .open_workspace_view(CellViewRef::new("work", "amp", "schematic"));
    editor
        .state
        .open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    editor.pass(Vec::new());
    assert_eq!(editor.state.load_active_symbol_document().unwrap(), after);
}

#[test]
fn queued_symbol_inspector_edits_revalidate_source_selection_and_authority() {
    let mut editor = Editor::new();
    let before = editor.state.load_active_symbol_document().unwrap();
    for changed in 0..4 {
        editor.state.ui.symbol.editor.select_shape(0);
        let mut request = editor.frame(Vec::new());
        request.undo_before.push(before.clone());
        request.document.origin = Point::new(20, 30);
        request.changed = true;
        match changed {
            0 => editor.state.ui.symbol.editor.select_shape(1),
            1 => editor.state.active_schematic_epoch += 1,
            2 => {
                editor
                    .state
                    .library_manager
                    .add_library(Library::new("other"));
            }
            _ => editor.state.schematic.session.read_only = true,
        }
        apply_request(&mut editor.state, request);
        assert_eq!(editor.state.load_active_symbol_document().unwrap(), before);
        assert!(!editor.state.can_undo_active_symbol_document());
    }
}
