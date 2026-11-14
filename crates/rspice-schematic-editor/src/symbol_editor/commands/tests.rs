//! Symbol command geometry and clipboard behavior independent of the app.

use super::*;
use rspice_design::symbol::SymbolPin;
use rspice_design_model::port::PortDirection;

#[test]
fn symbol_clipboard_copies_selected_shapes_and_pins() {
    let document = SymbolDocument {
        pins: vec![
            SymbolPin::new("IN", PortDirection::In, Some(Point::new(-10, 0))),
            SymbolPin::new("TRIM", PortDirection::InOut, Some(Point::new(0, 10))),
        ],
        body: vec![SymbolShape::Dot {
            center: Point::origin(),
            radius: 3,
        }],
        ..SymbolDocument::default()
    };
    let mut selection = SymbolSelection::default();
    selection.pins.insert("IN".to_owned());
    selection.pins.insert("TRIM".to_owned());
    selection.shapes.insert(0);

    let clipboard = clipboard_from_selection(&document, &selection);

    assert_eq!(clipboard.shapes.len(), 1);
    assert_eq!(
        clipboard
            .pins
            .iter()
            .map(|pin| pin.name.as_str())
            .collect::<Vec<_>>(),
        vec!["IN", "TRIM"]
    );
}

fn with_every_draft(tool: SymbolTool) -> SymbolEditorSession {
    SymbolEditorSession {
        tool,
        pending_polyline: vec![Point::origin()],
        shape_start: Some(Point::origin()),
        arc_sweep: Some(super::super::session::SymbolArcSweep::new(
            Point::origin(),
            Point::new(10, 0),
        )),
        text_draft: Some(super::super::session::SymbolTextDraft {
            anchor: Point::origin(),
            text: "GAIN".to_owned(),
            caret_placed: true,
        }),
        ..SymbolEditorSession::default()
    }
}

/// The first Escape lets go of the shape being drawn and keeps its tool; the
/// second puts the tool down.
#[test]
fn escape_cancels_the_shape_then_returns_to_select() {
    let mut session = with_every_draft(SymbolTool::Arc);

    finish_cancel(&mut session);

    assert!(!session.has_draft());
    assert_eq!(session.tool, SymbolTool::Arc);

    finish_cancel(&mut session);

    assert_eq!(session.tool, SymbolTool::Select);
}

/// Arming any tool, Select included, lets go of what the last one had part
/// drawn.
#[test]
fn arming_a_tool_lets_go_of_any_shape_being_drawn() {
    for tool in [SymbolTool::Select, SymbolTool::Line, SymbolTool::Text] {
        let mut session = with_every_draft(SymbolTool::Polygon);
        activate_tool(&mut session, tool, None);
        assert!(!session.has_draft(), "{tool:?}");
        assert_eq!(session.tool, tool);
    }
}

/// Text typed into the entry is placed where the tool was clicked, as one
/// selected shape; empty text places nothing.
#[test]
fn placing_text_puts_the_typed_text_at_the_anchor() {
    let mut session = with_every_draft(SymbolTool::Text);
    let mut document = SymbolDocument::default();

    let outcome = apply_edit(
        &mut session,
        &mut document,
        None,
        SymbolEditAction::PlaceText {
            anchor: Point::new(0, -30),
            text: "GAIN".to_owned(),
        },
        SymbolEditCapabilities { edit: true },
    );

    assert!(outcome.changed);
    assert!(outcome.undo_before.is_some());
    assert_eq!(
        document.body,
        vec![SymbolShape::Text {
            anchor: Point::new(0, -30),
            text: "GAIN".to_owned(),
            size: SymbolTextSize::default(),
            align: SymbolTextAlign::default(),
        }]
    );
    assert!(session.text_draft.is_none());
    assert_eq!(session.selected_shape, Some(0));

    session.text_draft = Some(super::super::session::SymbolTextDraft {
        anchor: Point::origin(),
        text: "  ".to_owned(),
        caret_placed: true,
    });
    let outcome = apply_edit(
        &mut session,
        &mut document,
        None,
        SymbolEditAction::PlaceText {
            anchor: Point::origin(),
            text: "  ".to_owned(),
        },
        SymbolEditCapabilities { edit: true },
    );
    assert!(!outcome.changed);
    assert_eq!(document.body.len(), 1);
}

/// Every symbol carries its Reference, Value and Model labels, so Delete on
/// a selected label hides it, undoably, and leaves the rest of the symbol.
#[test]
fn deleting_a_selected_label_hides_it() {
    let mut document = SymbolDocument {
        body: vec![SymbolShape::Dot {
            center: Point::origin(),
            radius: 3,
        }],
        ..SymbolDocument::default()
    };
    let mut metadata = SymbolEditorMetadata::for_document(&document);
    let mut session = SymbolEditorSession::default();
    session.select_attribute(SymbolAttributeKind::Value);

    let outcome = apply_edit(
        &mut session,
        &mut document,
        Some(&mut metadata),
        SymbolEditAction::Delete { cut: false },
        SymbolEditCapabilities { edit: true },
    );

    assert!(outcome.changed);
    assert!(
        outcome.undo_before.is_some(),
        "the hide is a step undo takes back"
    );
    let shown = |kind| metadata.attribute(kind).map(|attribute| attribute.shown);
    assert_eq!(shown(SymbolAttributeKind::Value), Some(false));
    assert_eq!(shown(SymbolAttributeKind::Reference), Some(true));
    assert_eq!(document.body.len(), 1);
    assert!(session.effective_selection().is_empty());
}
