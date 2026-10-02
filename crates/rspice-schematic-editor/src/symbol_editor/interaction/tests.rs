//! Local symbol draft edits and their undo intent, without application state.

use super::*;

#[test]
fn finishing_pending_polyline_creates_one_selected_shape() {
    let mut session = SymbolEditorSession::default();
    let mut edit = CanvasEdit {
        session: &mut session,
        capabilities: SymbolEditCapabilities { edit: true },
        outcome: SymbolEditOutcome::default(),
    };
    let mut document = SymbolDocument::default();
    edit.session.pending_polyline = vec![Point::new(0, 0), Point::new(10, 0)];

    assert!(finish_pending_polyline(&mut edit, &mut document));

    assert_eq!(document.body.len(), 1);
    assert_eq!(edit.session.selected_shape, Some(0));
    assert!(edit.session.selection.shapes.contains(&0));
    assert!(edit.session.pending_polyline.is_empty());
}

#[test]
fn place_pin_without_available_pin_does_not_record_undo() {
    let mut session = SymbolEditorSession::default();
    let mut edit = CanvasEdit {
        session: &mut session,
        capabilities: SymbolEditCapabilities { edit: true },
        outcome: SymbolEditOutcome::default(),
    };
    let mut document = SymbolDocument::default();

    assert!(!place_selected_pin(
        &mut edit,
        &mut document,
        Point::new(10, 0)
    ));

    assert!(edit.outcome.undo_before.is_none());
    assert!(document.pins.is_empty());
}
