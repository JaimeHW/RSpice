//! The symbol editor's inspector drawn through a whole application.

use crate::state::{Library, Point};
use crate::workbench::design::inspector::symbol::tests::Editor;
use crate::workbench::design::inspector::symbol::*;

#[test]
fn queued_symbol_inspector_edits_revalidate_source_selection_and_authority() {
    let mut editor = Editor::new();
    let before = editor
        .app
        .state
        .model
        .load_active_symbol_document()
        .unwrap();
    for changed in 0..4 {
        editor.app.state.design.symbol.editor.select_shape(0);
        let mut request = editor.frame(Vec::new());
        request.undo_before.push(before.clone());
        request.document.origin = Point::new(20, 30);
        request.changed = true;
        match changed {
            0 => editor.app.state.design.symbol.editor.select_shape(1),
            1 => editor.app.state.model.active_schematic_epoch += 1,
            2 => {
                editor
                    .app
                    .state
                    .model
                    .library_manager
                    .add_library(Library::new("other"));
            }
            _ => editor.app.state.model.schematic.session.read_only = true,
        }
        apply_request(&mut editor.app.design_cx(), request);
        assert_eq!(
            editor
                .app
                .state
                .model
                .load_active_symbol_document()
                .unwrap(),
            before
        );
        assert!(!editor.app.state.can_undo_active_symbol_document());
    }
}
