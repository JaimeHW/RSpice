//! Symbol command geometry and clipboard behavior independent of the app.

use super::*;
use rspice_design::symbol::{SymbolAttributeKind, SymbolPin};
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
        activate_tool(&mut session, tool, None, &[]);
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
            rotation: Default::default(),
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
    assert_eq!(outcome.step, "Hide value label");
    let shown = |kind| metadata.attribute(kind).map(|attribute| attribute.shown);
    assert_eq!(shown(SymbolAttributeKind::Value), Some(false));
    assert_eq!(shown(SymbolAttributeKind::Reference), Some(true));
    assert!(
        document.value_hidden && !document.name_hidden,
        "placed parts stop lettering the value alone"
    );
    assert_eq!(document.body.len(), 1);
    assert!(session.effective_selection().is_empty());
}

/// Enter in the Pin tool's name entry renames the pin it placed, as one
/// edit, and closes the entry. A name another pin has is refused, and so is
/// an empty one; the pin keeps the name it was given.
#[test]
fn the_name_typed_for_a_placed_pin_renames_it() {
    let mut document = SymbolDocument {
        pins: vec![
            SymbolPin::new("IN", PortDirection::In, Some(Point::new(-10, 0))),
            SymbolPin::new("PIN1", PortDirection::InOut, Some(Point::new(10, 0))),
        ],
        ..SymbolDocument::default()
    };
    let rename = |session: &mut SymbolEditorSession, document: &mut SymbolDocument, name: &str| {
        session.pin_name_draft = Some(crate::symbol_editor::session::SymbolPinNameDraft {
            pin: "PIN1".to_owned(),
            text: name.to_owned(),
            caret_placed: true,
        });
        let outcome = apply_edit(
            session,
            document,
            None,
            SymbolEditAction::RenamePlacedPin {
                pin: "PIN1".to_owned(),
                name: name.to_owned(),
            },
            SymbolEditCapabilities { edit: true },
        );
        assert!(
            session.pin_name_draft.is_none(),
            "{name:?} closes the entry"
        );
        outcome
    };
    let mut session = SymbolEditorSession::default();
    for refused in ["in", ""] {
        let outcome = rename(&mut session, &mut document, refused);
        assert!(
            !outcome.changed && outcome.undo_before.is_none(),
            "{refused:?}"
        );
        assert!(document.pin("PIN1").is_some());
    }
    let outcome = rename(&mut session, &mut document, "CLK");
    assert!(outcome.changed && outcome.undo_before.is_some());
    assert_eq!(outcome.step, "Rename pin PIN1");
    assert!(document.pin("PIN1").is_none());
    assert_eq!(
        document.pin("CLK").and_then(|pin| pin.position),
        Some(Point::new(10, 0))
    );
    assert_eq!(session.selected_pin.as_deref(), Some("CLK"));
}

/// Turning a selection turns each selected pin's side with it, and
/// mirroring flips it, so every lead still runs from its tip into the body
/// it belongs to.
#[test]
fn turning_or_mirroring_a_selection_carries_each_pins_side() {
    use rspice_design_model::symbol_pin::SymbolPinSide::{Bottom, Left, Right, Top};
    let document = SymbolDocument {
        pins: vec![
            SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, 0))).with_side(Left),
            SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(40, 0))).with_side(Right),
        ],
        body: vec![SymbolShape::Polyline {
            points: vec![
                Point::new(-20, -20),
                Point::new(20, -20),
                Point::new(20, 20),
                Point::new(-20, 20),
            ],
            closed: true,
            style: Default::default(),
        }],
        ..SymbolDocument::default()
    };
    for (transform, sides) in [
        (SymbolTransform::Rotate, [Top, Bottom]),
        (SymbolTransform::RotateCounterClockwise, [Bottom, Top]),
        (SymbolTransform::MirrorHorizontal, [Right, Left]),
        (SymbolTransform::MirrorVertical, [Left, Right]),
    ] {
        let mut document = document.clone();
        let mut metadata = SymbolEditorMetadata::for_document(&document);
        let mut session = SymbolEditorSession::default();
        session.set_selection(SymbolSelection::all_in(&document));
        let outcome = apply_edit(
            &mut session,
            &mut document,
            Some(&mut metadata),
            SymbolEditAction::Transform(transform),
            SymbolEditCapabilities { edit: true },
        );
        assert!(outcome.changed, "{transform:?}");
        let turned = document.pins.iter().map(|pin| pin.side).collect::<Vec<_>>();
        assert_eq!(turned, sides, "{transform:?}");
        for pin in &document.pins {
            let inner = pin.lead_inner().expect("a placed pin");
            assert!(
                inner.x.abs() <= 20 && inner.y.abs() <= 20,
                "{transform:?}: {} runs its lead to {inner:?}, off the body",
                pin.name
            );
        }
    }
}

/// A pasted or duplicated pin takes the next free name, counting up from
/// the number its name ends in, and never `<name>_copy`.
#[test]
fn a_copied_pin_takes_the_next_free_name() {
    let document = SymbolDocument {
        pins: ["D3", "D4", "CLK", "A0[7:0]", "Q09", "in1"]
            .into_iter()
            .map(|name| SymbolPin::new(name, PortDirection::In, None))
            .collect(),
        ..SymbolDocument::default()
    };
    for (copied, named) in [
        ("D3", "D5"),
        ("CLK", "CLK1"),
        ("A0[7:0]", "A1[7:0]"),
        ("Q09", "Q10"),
        ("IN", "IN2"),
    ] {
        assert_eq!(next_free_pin_name(&document, copied), named, "{copied}");
    }
}

/// A lone pin turns about its own tip: it stays where it is wired and only
/// its side turns. A group turns about the middle of what is selected, not
/// the symbol's origin, so a shape drawn off to one side turns in place.
#[test]
fn a_turn_pivots_on_the_selection_not_the_origin() {
    use rspice_design_model::symbol_pin::SymbolPinSide::{Left, Top};
    let document = SymbolDocument {
        pins: vec![
            SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, 10))).with_side(Left),
        ],
        body: vec![SymbolShape::rectangle(
            Point::new(60, 40),
            Point::new(100, 60),
        )],
        ..SymbolDocument::default()
    };
    let turn = |selection: SymbolSelection, transform: SymbolTransform| {
        let mut document = document.clone();
        let mut metadata = SymbolEditorMetadata::for_document(&document);
        let mut session = SymbolEditorSession::default();
        session.set_selection(selection);
        let outcome = apply_edit(
            &mut session,
            &mut document,
            Some(&mut metadata),
            SymbolEditAction::Transform(transform),
            SymbolEditCapabilities { edit: true },
        );
        (document, outcome)
    };

    let (turned, outcome) = turn(SymbolSelection::single_pin("IN"), SymbolTransform::Rotate);
    assert_eq!(turned.pins[0].position, Some(Point::new(-40, 10)));
    assert_eq!(turned.pins[0].side, Top);
    assert_eq!(outcome.step, "Rotate pin IN");

    // A pin on the left mirrored left to right across its own tip faces
    // right; mirrored top to bottom it is the same pin, and no edit.
    let (mirrored, outcome) = turn(
        SymbolSelection::single_pin("IN"),
        SymbolTransform::MirrorVertical,
    );
    assert_eq!(mirrored, document);
    assert!(!outcome.changed && outcome.undo_before.is_none());

    // The rectangle's middle is (80, 50); it turns there, a 40 x 20 box
    // standing 20 x 40 about the same middle.
    let (turned, outcome) = turn(SymbolSelection::single_shape(0), SymbolTransform::Rotate);
    assert_eq!(
        turned.body[0],
        SymbolShape::rectangle(Point::new(70, 30), Point::new(90, 70))
    );
    assert!(outcome.changed);
}
